/// 256 bit unsigned integers.

use std::cmp;
use std::fmt;
use std::ops;

use super::arithmetic;

#[derive(Clone, Debug)]
pub struct U256 {
    // These are stored with the least significant 64 bits first.
    digits: Vec<u64>,
}

// TODO: Remainder, GT, LT, Quotient (division)
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
        U256::literal(vec![x, 0, 0, 0])
    }

    pub fn from_bytes_be(bytes: Vec<u8>) -> U256 {
        assert_eq!(bytes.len(), 32);
        let mut digits: Vec<u64> = Vec::with_capacity(4);
        for chunk in bytes.chunks(8).rev() {
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

    pub fn is_zero(&self) -> bool {
        self.digits[0] == 0 && self.digits[1] == 0 &&
            self.digits[2] == 0 && self.digits[3] == 0
    }
}

impl ops::Add for U256 {
    type Output = U256;

    fn add(self, rhs: U256) -> U256 {
        U256::literal(arithmetic::add_big_ints(&self.digits, &rhs.digits))
    }
}

impl ops::Sub for U256 {
    type Output = U256;

    fn sub(self, rhs: U256) -> U256 {
        U256::literal(arithmetic::sub_big_ints(&self.digits, &rhs.digits))
    }
}

impl ops::Mul for U256 {
    type Output = U256;

    fn mul(self, rhs: U256) -> U256 {
        let v = arithmetic::mul_big_ints(&self.digits, &rhs.digits);
        U256::literal(v[..4].to_vec())
    }
}

impl ops::Rem for U256 {
    type Output = U256;
    fn rem(self, rhs: U256) -> U256 {
        U256::literal(arithmetic::rem_big_ints(&self.digits, &rhs.digits))
    }
}

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Think of a better way to print this...
        write!(f, "{:0>#018x}{:0>16x}{:0>16x}{:0>16x}",
               self.digits[3], self.digits[2],
               self.digits[1], self.digits[0])
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

impl cmp::Eq for U256 {}

impl cmp::PartialOrd for U256 {
    fn partial_cmp(&self, other: &U256) -> Option<cmp::Ordering> {
        Some(arithmetic::cmp_big_ints(&self.digits, &other.digits))
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn addition() {
        let x = U256::from_u64(10);
        let y = U256::from_u64(12);
        assert_eq!(x + y, U256::from_u64(22));

        let x = U256::from_bytes_be(vec![
            0x01, 0xff, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00,
            0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xff, 0xfe, 0xff, 0xff, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff,
        ]);
        let y = U256::from_bytes_be(vec![
            0x00, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x00, 0xab, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x01, 0x18,
        ]);
        let expected = U256::from_bytes_be(vec![
            0x02, 0x01, 0x00, 0xff, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0xaa, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x02, 0x17,
        ]);
        let ans = x + y;
        assert_eq!(ans, expected);

        let x = U256::from_u64(187236152);
        let y = U256::from_u64(187236152);
        assert_eq!(x, y + U256::zero());
    }

    #[test]
    fn subtraction() {
        let x = U256::from_u64(10);
        let y = U256::from_u64(12);
        assert_eq!(y - x, U256::from_u64(2));

        let x = U256::from_bytes_be(vec![
            0x02, 0x01, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0xaa, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x02, 0x17,
        ]);
        let y = U256::from_bytes_be(vec![
            0x00, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x00, 0xab, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x01, 0x18,
        ]);
        let expected = U256::from_bytes_be(vec![
            0x01, 0xff, 0x01, 0xff, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xff, 0xfe, 0xff, 0xff, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff,
        ]);
        let ans = x - y;
        assert_eq!(ans, expected);

        let x = U256::from_u64(7192478999);
        let y = U256::from_u64(7192478999);
        assert_eq!(x, y - U256::zero());
    }

    #[test]
    fn multiplication() {
        let x = U256::from_u64(20);
        let y = U256::from_u64(16);
        assert_eq!(x * y, U256::from_u64(20 * 16));

        // Got these numbers from testing in Python
        let x = U256::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x13, 0x2f, 0x40, 0xb7, 0x63,
            0x50, 0xe4, 0x7c, 0xcd, 0x9a, 0x5f, 0x4e, 0xa2,
        ]);
        let y = U256::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x6f, 0x6c, 0x08, 0xeb, 0xf4,
            0x47, 0x5f, 0x5b, 0xdb, 0x28, 0xc7, 0x8d, 0x29,
        ]);
        let expected = U256::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x59,
            0x95, 0xa9, 0xfa, 0x22, 0x7f, 0x94, 0x5c, 0xf4,
            0x80, 0x65, 0xd0, 0x3f, 0x78, 0x3c, 0xe1, 0xea,
            0xfd, 0xe0, 0xf9, 0xe9, 0xa7, 0x80, 0xd1, 0xf2,
        ]);
        assert_eq!(x * y, expected);

        let x = U256::from_u64(7192478999);
        let y = U256::from_u64(7192478999);
        assert_eq!(x, y * U256::from_u64(1));
    }

    #[test]
    fn remainder() {
        //let x = U256::from_u64(13);
        //let y = U256::from_u64(7);
        //assert_eq!(y % x, U256::from_u64(7));

        //for i in 0..60 {
        //    let x = U256::from_u64(13 + 7 * i);
        //    let y = U256::from_u64(7);
        //    assert_eq!(x % y, U256::from_u64(6));
        //}

        let x = U256::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xbc, 0x86, 0x00, 0x8f, 0xff, 0x85, 0x3f, 0x8e,
            0xc6, 0x0a, 0x0b, 0xb4, 0xd0, 0x36, 0x26, 0xfc,
            0x44, 0x7c, 0xf3, 0x2a, 0x45, 0x2c, 0xd0, 0x1c,
        ]);

        let y = U256::from_bytes_be(vec![
            0x1b, 0x50, 0xdd, 0xa8, 0x70, 0x14, 0xa2, 0x7d,
            0x4b, 0xd4, 0xe8, 0xcb, 0x1d, 0xfa, 0xe7, 0xfc,
            0xbe, 0x5a, 0x68, 0x53, 0x24, 0x01, 0x92, 0xc1,
            0x55, 0x22, 0xbc, 0x55, 0x2e, 0xc5, 0xc8, 0x9a,
        ]);


        let expected = U256::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x45, 0x14, 0xe6, 0x13,
            0x71, 0x84, 0x35, 0x15, 0xa3, 0x66, 0x89, 0x8a,
            0x55, 0xe6, 0x70, 0x29, 0xb9, 0xaf, 0x7c, 0xb8,
            0x38, 0x2c, 0x43, 0xd8, 0xec, 0xf6, 0xfb, 0x6a,
        ]);
        assert_eq!(y % x, expected);
    }
}
