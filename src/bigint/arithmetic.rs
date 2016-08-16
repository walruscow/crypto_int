use std::cmp::Ordering;
use std::u64;

// TODO: Add overflow checks? Or is overflow useful?
// TODO: Use slices instead of vectors?
// TODO: In place additions?

// TODO: I'm not sure this will overflow correctly..
pub fn add_big_ints(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
    assert_eq!(a.len(), b.len());
    let mut overflow = false;
    a.iter().zip(b.iter()).map(move |(x, y)| {
        let digit = if overflow {
            x.wrapping_add(*y).wrapping_add(1)
        } else {
            x.wrapping_add(*y)
        };
        // Digit overflowed iff result is less than either of the operands
        overflow = digit < *x;
        digit
    }).collect()
}

pub fn sub_big_ints(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
    assert_eq!(a.len(), b.len());
    let mut underflow = false;
    a.iter().zip(b.iter()).map(move |(x, y)| {
        let digit = if underflow {
            x.wrapping_sub(*y).wrapping_sub(1)
        } else {
            x.wrapping_sub(*y)
        };
        // Digit underflowed iff result is more than the original value
        underflow = digit > *x;
        digit
    }).collect()
}

// Oh god...
pub fn mul_big_ints(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
    assert_eq!(a.len(), b.len());
    if a.len() == 1 {
        let (low, high) = mul_ints(a[0], b[0]);
        return vec![low, high];
    }
    let (a0, a1) = a.split_at(a.len() / 2);
    let (a0, a1) = (a0.to_vec(), a1.to_vec());
    let (b0, b1) = b.split_at(b.len() / 2);
    let (b0, b1) = (b0.to_vec(), b1.to_vec());
    let z0 = mul_big_ints(&a0, &b0);
    let z1 = add_big_ints(&mul_big_ints(&a0, &b1), &mul_big_ints(&a1, &b0));
    let z2 = mul_big_ints(&a1, &b1);

    let (low_mid, high_mid) = z1.split_at(z1.len() / 2);
    let (mut low_mid, mut high_mid) = (low_mid.to_vec(), high_mid.to_vec());
    // Now push 0s onto the front of low_mid, and onto the back of high_mid
    let mut low_result: Vec<u64> = Vec::new();
    while low_result.len() < a0.len() {
        low_result.push(0);
    }
    low_result.append(&mut low_mid);
    while high_mid.len() < z2.len() {
        high_mid.push(0);
    }

    let mut low_result = add_big_ints(&low_result, &z0);
    let mut high_result = add_big_ints(&z2, &high_mid);
    low_result.append(&mut high_result);
    low_result
}

// Return (low bits, high bits)
fn mul_ints(a: u64, b: u64) -> (u64, u64) {
    let (a1, a0) = ((a >> 32), a & 0xffffffff);
    let (b1, b0) = ((b >> 32), b & 0xffffffff);

    let z0 = a0 * b0;
    let z1 = a0 * b1 + b0 * a1;
    let z2 = a1 * b1;

    let low_bits = z0.wrapping_add(z1 << 32);
    let high_bits = if low_bits < z0 {
        z2 + (z1 >> 32) + 1
    } else {
        z2 + (z1 >> 32)
    };
    (low_bits, high_bits)
}

pub fn cmp_big_ints(a: &Vec<u64>, b: &Vec<u64>) -> Ordering {
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

fn last_nonzero(vec: &Vec<u64>) -> (usize, u64) {
    for (i, v) in vec.iter().enumerate().rev() {
        if *v != 0 {
            return (i, *v);
        }
    }
    (0, vec[0])
}

pub fn rem_big_ints(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
    println!("{:?}, {:?}", a, b);
    // This is a % b, of course.
    // TODO: Should we handle this and try to make it take the same time?
    match cmp_big_ints(&a, &b) {
        Ordering::Equal => return vec![0, 0, 0, 0],
        Ordering::Less => return a.clone(),
        Ordering::Greater => (), // do nothing :)
    }

    let (a_idx, a_byte) = last_nonzero(&a);
    let (b_idx, b_byte) = last_nonzero(&b);

    let mut a_new = a.clone();
    if a_idx > b_idx {
        println!("Index greater");
        // A is a whole thing bigger, figure out how much to get there
        let mut mul: u64;
        if a_byte > b_byte {
            mul = u64::MAX;
            println!("Got mul: int_max {}", mul);
        } else {
            let m = u64::MAX / b_byte;
            mul = if m == 0 {
                a_byte
            } else if m == 1 {
                a_byte
            } else {
                a_byte * (m - 1)
            };
            println!("Got mul: {}", mul);
        }
        let mut b_new = mul_big_ints(&b, &vec![mul, 0, 0, 0])[..4].to_vec();
        while cmp_big_ints(&b_new, &b) == Ordering::Greater {
            println!("Subbing b down");
            b_new = sub_big_ints(&b_new ,&b);
        }
        a_new = sub_big_ints(&a, &b_new);
        if cmp_big_ints(&a_new, &b) == Ordering::Less {
            return a_new;
        }
        println!("Hey that");
    }

    // idx should be the same
    println!("Index: {} {}", a_idx, b_idx);
    while cmp_big_ints(&b, &a_new) == Ordering::Less {
        println!("Hay {} {}", a_idx, b_idx);
        println!("b: {:?} a: {:?}", b, a_new);
        let (a_idx, a_byte) = last_nonzero(&a_new);
        let mul = a_byte / b_byte;
        let b_new = if mul > 1 {
            println!("mul: {}", mul);
            mul_big_ints(&b, &vec![mul, 0, 0, 0])[..4].to_vec()
        } else {
            b.clone()
        };
        //println!("b_new: {:?}", b_new);
        a_new = sub_big_ints(&a_new, &b_new);
        //println!("a_new: {:?}", b_new);
    }

    //println!("Returning a");
    return a_new;
}
