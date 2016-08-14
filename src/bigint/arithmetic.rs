use std::num::Wrapping;

type W = Wrapping<u64>;

pub fn add_big_ints(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
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
