/// 256 bit unsigned integers.

use std::cmp;
use std::fmt;
use std::ops;

use super::arithmetic::add_big_ints;

#[derive(Clone, Debug)]
pub struct U256 {
    digits: Vec<u64>,
}

impl U256 {
    // TODO: We can do away with this once we are sure the 4 len requirement
    // isn't being violated
    fn literal(digits: Vec<u64>) -> U256 {
        // 4 * 64 == 256
        assert_eq!(digits.len(), 4);
        U256 {
            digits: digits,
        }
    }

    pub fn from_u64(x: u64) -> U256 {
        U256::literal(vec![0, 0, 0, x])
    }

    pub fn from_bytes_be(bytes: Vec<u8>) -> U256 {
        assert_eq!(bytes.len(), 32);
        let mut digits: Vec<u64> = Vec::with_capacity(4);
        for chunk in bytes.chunks(8) {
            let mut x = 0u64;
            for (i, byte) in chunk.iter().enumerate() {
                x |= (*byte as u64) << ((7 - i) * 8);
            }
            digits.push(x);
        }
        U256::literal(digits)
    }

    pub fn zero() -> U256 {
        U256::literal(vec![0, 0, 0, 0])
    }
}

impl ops::Add for U256 {
    type Output = U256;

    fn add(self, rhs: U256) -> U256 {
        U256::literal(add_big_ints(&self.digits, &rhs.digits))
    }
}

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#0x}{:x}{:x}{:x}",
               self.digits[0], self.digits[1],
               self.digits[2], self.digits[3])
    }
}

impl cmp::PartialEq for U256 {
    fn eq(&self, other: &U256) -> bool {
        self.digits == other.digits
    }

    fn ne(&self, other: &U256) -> bool {
        self.digits != other.digits
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn addition() {
        let x = U256::from_u64(10);
        let y = U256::from_u64(12);
        assert_eq!(x + y, U256::from_u64(22));

        let x = U256::from_bytes_be(vec![
            0x01, 0xff, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xff, 0xfe, 0xff, 0xff, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x02,
        ]);
        let y = U256::from_bytes_be(vec![
            0x00, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x01,
        ]);

        let z = U256::from_bytes_be(vec![
            0x02, 0x01, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x03,
        ]);
        assert_eq!(x + y, z);
    }

    #[test]
    fn basic() {
        let zero_1 = U256::zero();
        let zero_2 = U256::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);

        let five_1 = U256::from_u64(5);
        let five_2 = U256::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
        ]);

        assert_eq!(zero_1, zero_2);
        assert_eq!(five_1, five_2);
    }
}
