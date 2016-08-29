use std::cmp::Ordering;
use std::u64;

use rand::Rng;

// Stores the result in a. a must be larger than b.
pub fn add(a: &mut [u64], b: &[u64]) -> bool {
    assert!(a.len() >= b.len());

    let mut overflow = false;
    for (x, y) in a.iter_mut().zip(b.iter()) {
        let digit = if overflow {
            x.wrapping_add(*y).wrapping_add(1)
        } else {
            x.wrapping_add(*y)
        };
        overflow = digit < *x;
        *x = digit;
    }

    for x in &mut a[b.len()..] {
        if !overflow {
            break;
        }

        let digit = if overflow {
            x.wrapping_add(1)
        } else {
            *x
        };
        overflow = digit < *x;
        *x = digit;
    }

    overflow
}

// a -= b
pub fn sub(a: &mut [u64], b: &[u64]) -> bool {
    assert_eq!(a.len(), b.len());

    let mut underflow = false;
    for (x, y) in a.iter_mut().zip(b.iter()) {
        let digit = if underflow {
            x.wrapping_sub(*y).wrapping_sub(1)
        } else {
            x.wrapping_sub(*y)
        };
        // Digit underflowed iff result is more than the original value
        underflow = digit > *x;
        *x = digit;
    }
    underflow
}

/// Multiplication that ignores the high bits.
pub fn mul(a: &[u64], b: &[u64], c: &mut[u64]) {
    assert_eq!(a.len(), b.len());

    for val in c.iter_mut() {
        *val = 0;
    }

    let len = a.len();
    for (a_idx, a_val) in a.iter().enumerate() {
        for (b_idx, b_val) in b.iter().enumerate() {
            let mut ans_idx = a_idx + b_idx;
            if ans_idx >= a.len() {
                break;
            }
            let (low, high) = mul_ints(*a_val, *b_val);

            // Add the low bits
            let (val, mut overflow) = c[ans_idx].overflowing_add(low);
            c[ans_idx] = val;

            // Add the high bits
            ans_idx += 1;
            if ans_idx < len {
                let old = c[ans_idx];
                c[ans_idx] = c[ans_idx].wrapping_add(high);
                if overflow {
                    c[ans_idx] = c[ans_idx].wrapping_add(1);
                }
                overflow = c[ans_idx] < old;
            }

            // Propagate overflow down
            ans_idx += 1;
            for idx in ans_idx..len {
                if !overflow {
                    break;
                }
                let old = c[idx];
                c[idx] = c[idx].wrapping_add(1);
                overflow = c[idx] < old;
            }
        }
    }
}

// Return (low bits, high bits)
fn mul_ints(a: u64, b: u64) -> (u64, u64) {
    let (a1, a0) = ((a >> 32), a & 0xffffffff);
    let (b1, b0) = ((b >> 32), b & 0xffffffff);

    let z0 = a0 * b0;
    // z1 is the middle bits. The low bits in z1 are added to the
    // high bits of z0, and the high bits in z1 are added to the low
    // bits of z2
    let (z1, overflow) = (a0 * b1).overflowing_add(b0 * a1);
    let z2 = if overflow {
        a1 * b1 + (1 << 32)
    } else {
        a1 * b1
    };

    let (low_bits, overflow) = z0.overflowing_add(z1 << 32);
    let high_bits = if overflow {
        z2 + (z1 >> 32) + 1
    } else {
        z2 + (z1 >> 32)
    };
    (low_bits, high_bits)
}

pub fn cmp(a: &[u64], b: &[u64]) -> Ordering {
    for (x, y) in a.iter().zip(b.iter()).rev() {
        if x > y {
            return Ordering::Greater;
        } else if y > x {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

// Shorthand to save a heap allocation where we can
fn shl_to(a: &[u64], out: &mut[u64], shift: usize) {
    out.clone_from_slice(a);
    shl(out, shift);
}

pub fn shl(a: &mut[u64], shift: usize) {
    assert!(shift < 64 * a.len());
    if shift == 0 {
        return;
    }

    // How many digits will be zeroed out completely
    let digits_shifted = shift / 64;
    let shift = shift % 64;

    for idx in (digits_shifted + 1..a.len()).rev() {
        let high_bits = a[idx - digits_shifted] << shift;
        let low_bits = if shift == 0 {
            0
        } else {
            a[idx - digits_shifted - 1] >> (64 - shift)
        };
        a[idx] = high_bits | low_bits;
    }

    // The digit on the edge is shifted normally
    a[digits_shifted] = a[0] << shift;
    for idx in 0..digits_shifted {
        a[idx] = 0;
    }
}

pub fn shr(a: &mut[u64], shift: usize) {
    assert!(shift < 64 * a.len());
    if shift == 0 {
        return;
    }

    let len = a.len();
    // This is how many high digits will be zeroed out
    let digits_zeroed = shift / 64;
    // All digits past this are 0 due to shifting them out
    let last_nonzero_digit = len - 1 - digits_zeroed;
    let shift = shift % 64;

    for idx in 0..(last_nonzero_digit) {
        let low_bits = a[idx + digits_zeroed] >> shift;
        let high_bits = if shift == 0 {
            0
        } else {
            a[idx  + digits_zeroed + 1] << (64 - shift)
        };
        a[idx] = high_bits | low_bits;
    }
    a[last_nonzero_digit] = a[len - 1] >> shift;
    for val in &mut a[last_nonzero_digit+1..] {
        *val = 0;
    }
}

fn get_msb_idx(a: &[u64]) -> usize {
    let mut idx = 0;
    for (i, val) in a.iter().enumerate() {
        let x = (64 - val.leading_zeros()) as usize;
        if x != 0 {
            idx = i * 64 + x;
        }
    }
    idx
}

pub fn div_rem(a: &[u64], b: &[u64], quot: &mut[u64], rem: &mut[u64]) {
    assert_eq!(a.len(), b.len());
    rem.clone_from_slice(a);
    let b_msb_idx = get_msb_idx(b);

    for x in quot.iter_mut() {
        *x = 0;
    }

    let mut shifted_b = b.to_vec();
    loop {
        match cmp(&b, &rem) {
            Ordering::Equal => {
                quot[0] |= 1;
                for x in rem.iter_mut() {
                    *x = 0;
                }
                break;
            },
            Ordering::Greater => break,
            Ordering::Less => (),
        }

        let a_msb_idx = get_msb_idx(rem);
        let mut shift_amount = if a_msb_idx > b_msb_idx {
            a_msb_idx - b_msb_idx - 1
        } else {
            0
        };

        shl_to(b, &mut shifted_b, shift_amount + 1);
        if cmp(&shifted_b, rem) != Ordering::Greater {
            sub(rem, &shifted_b);
            shift_amount += 1;
        } else {
            // We can undo like this because we know we didn't push the msb off.
            shr(&mut shifted_b, 1);
            sub(rem, &shifted_b);
        }
        let num_idx = shift_amount / 64;
        quot[num_idx] |= 1 << (shift_amount % 64);
    }
}

pub fn bitor(a: &mut[u64], b: &[u64]) {
    for (x, y) in a.iter_mut().zip(b.iter()) {
        *x |= *y;
    }
}

pub fn bitand(a: &mut[u64], b: &[u64]) {
    for (x, y) in a.iter_mut().zip(b.iter()) {
        *x &= *y;
    }
}

pub fn bitxor(a: &mut[u64], b: &[u64]) {
    for (x, y) in a.iter_mut().zip(b.iter()) {
        *x ^= *y;
    }
}

pub fn bitnot(a: &mut[u64]) {
    for x in a.iter_mut() {
        *x = !*x;
    }
}

pub fn rand_int_lt<R: Rng>(a: &[u64], out: &mut[u64], rng: &mut R) {
    assert_eq!(a.len(), out.len());

    let msb = get_msb_idx(a);
    // How many digits to generate
    let digits = if msb % 64 == 0 {
        msb / 64
    } else {
        msb / 64 + 1
    };

    for idx in digits..out.len() {
        out[idx] = 0;
    }

    loop {
        // Everything is equal so far.
        let mut eq = true;

        // Generate digits one at a time.
        for idx in (0..digits).rev() {
            let mask: u64 = if idx == digits - 1 {
                // First digit, we only compare the relevant bits.
                // 0 <= shift <= 63
                !(u64::MAX << (msb % 64))
            } else {
                // Other digits we compare all bits.
                !0
            };

            // so we need to generate the first 64 bits.
            let rand_digit = rng.next_u64() & mask;
            if eq && rand_digit > a[idx] {
                // Previous digits equal and ours is too large. Try again.
                break;
            } else if eq && rand_digit < a[idx] {
                // This digit is smaller, so our number is smaller.
                eq = false;
                out[idx] = rand_digit;
            } else {
                out[idx] = rand_digit;
            }
        }

        if !eq {
            // We got a number smaller than a
            break;
        }
    }
}
