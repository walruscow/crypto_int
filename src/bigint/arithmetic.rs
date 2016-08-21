use std::cmp::Ordering;

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

pub fn mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    assert_eq!(a.len(), b.len());
    if a.len() == 1 {
        let (low, high) = mul_ints(a[0], b[0]);
        return vec![low, high];
    }
    let (a0, a1) = a.split_at(a.len() / 2);
    let (b0, b1) = b.split_at(b.len() / 2);

    let z0 = mul(&a0, &b0);
    let z1 = {
        let mut m1 = mul(&a0, &b1);
        if add(&mut m1, &mul(&a1, &b0)) {
            m1.push(1);
        }
        m1
    };
    let mut z2 = mul(&a1, &b1);

    let (low_mid, high_mid) = z1.split_at(a0.len());

    let mut low_result: Vec<u64> = Vec::with_capacity(z1.len());
    while low_result.len() < a0.len() {
        low_result.push(0);
    }
    low_result.extend_from_slice(&low_mid);

    let overflow = add(&mut low_result, &z0);
    add(&mut z2, &high_mid);
    if overflow {
        add(&mut z2, &vec![1]);
    }

    low_result.append(&mut z2);
    low_result
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
    assert_eq!(a.len(), b.len());
    let mut order = Ordering::Equal;
    for (x, y) in a.iter().zip(b.iter()).rev() {
        if x > y {
            order = match order {
                Ordering::Equal => Ordering::Greater,
                _ => order,
            };
        } else if y > x {
            order = match order {
                Ordering::Equal => Ordering::Less,
                _ => order,
            };
        }
    }
    order
}

// TODO: Make this fast AF
pub fn shl(a: &[u64], shift: usize) -> Vec<u64> {
    assert!(shift < 256);
    if shift == 0 {
        return a.to_vec();
    }

    // Create mask of shift high bits
    let lead = shift / 64;
    // lowest bits just get shifted.
    let mut new_vec: Vec<u64> = Vec::with_capacity(a.len());
    for _ in 0..lead {
        new_vec.push(0);
    }

    let shift = shift % 64;
    new_vec.push(a[0] << shift);
    for (i, bits) in a.iter().enumerate().skip(1).take(a.len() - lead - 1) {
        let last_high_bits = a[i - 1] >> (64 - shift);
        new_vec.push((*bits << shift) | last_high_bits);
    }
    new_vec
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

pub fn div_rem(a: &[u64], b: &[u64]) -> (Vec<u64>, Vec<u64>) {
    assert_eq!(a.len(), b.len());
    let mut rem = a.to_vec();
    let b_msb_idx = get_msb_idx(&b);

    let mut quotient: Vec<u64> = rem.iter().map(|_| 0).collect();
    loop {
        match cmp(&b, &rem) {
            Ordering::Equal => {
                quotient[0] |= 1;
                break;
            },
            Ordering::Greater => break,
            Ordering::Less => (),
        }

        let a_msb_idx = get_msb_idx(&rem);
        let mut shift_amount = if a_msb_idx > b_msb_idx {
            a_msb_idx - b_msb_idx - 1
        } else {
            0
        };

        let shifted_b = shl(&b, shift_amount);
        let shifted_b_more = shl(&b, shift_amount + 1);
        if cmp(&shifted_b_more, &rem) != Ordering::Greater {
            sub(&mut rem, &shifted_b_more);
            shift_amount += 1;
        } else {
            sub(&mut rem, &shifted_b);
        }
        let num_idx = shift_amount / 64;
        quotient[num_idx] |= 1 << (shift_amount % 64);
    }

    (quotient, rem)
}
