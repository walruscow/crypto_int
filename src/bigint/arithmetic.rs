use std::num::Wrapping;

type W = Wrapping<u64>;

// TODO: Add overflow checks? Or is overflow useful?

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
    (z0 + (z1 << 32), z2 + (z1 >> 32))
}
