/// 512 bit unsigned integers.

use std::cmp;
use std::fmt;
use std::ops;

use super::arithmetic;

#[derive(Copy, Clone, Debug)]
pub struct U512 {
    // These are stored with the least significant 64 bits first.
    digits: [u64; 8],
}

impl U512 {
    #[inline(always)]
    fn literal(digits: [u64; 8]) -> U512 {
        U512 {
            digits: digits,
        }
    }

    pub fn from_u64(x: u64) -> U512 {
        U512::literal([x, 0, 0, 0, 0, 0, 0, 0])
    }

    pub fn from_bytes_be(bytes: Vec<u8>) -> U512 {
        assert!(bytes.len() <= 64);
        let mut bytes = bytes;
        bytes.reverse();

        let mut digits: [u64; 8] = [0; 8];
        for (digit, chunk) in bytes.chunks(8).enumerate() {
            for (i, byte) in chunk.iter().enumerate() {
                digits[digit] |= (*byte as u64) << i * 8;
            }
        }

        U512::literal(digits)
    }

    pub fn zero() -> U512 {
        U512::literal([0, 0, 0, 0, 0, 0, 0, 0])
    }

    pub fn is_zero(&self) -> bool {
        let mut all_zero = true;
        for x in &self.digits {
            if *x != 0 {
                all_zero = false;
            }
        }
        all_zero
    }
}

impl ops::Add for U512 {
    type Output = U512;
    fn add(mut self, rhs: U512) -> U512 {
        arithmetic::add(&mut self.digits, &rhs.digits);
        self
    }
}

impl ops::AddAssign for U512 {
    fn add_assign(&mut self, rhs: U512) {
        arithmetic::add(&mut self.digits, &rhs.digits);
    }
}

impl ops::Sub for U512 {
    type Output = U512;
    fn sub(mut self, rhs: U512) -> U512 {
        arithmetic::sub(&mut self.digits, &rhs.digits);
        self
    }
}

impl ops::SubAssign for U512 {
    fn sub_assign(&mut self, rhs: U512) {
        arithmetic::sub(&mut self.digits, &rhs.digits);
    }
}

impl ops::Mul for U512 {
    type Output = U512;
    fn mul(mut self, rhs: U512) -> U512 {
        let mut new_digits = arithmetic::mul(&self.digits, &rhs.digits);
        new_digits.truncate(8);
        self.digits.clone_from_slice(&new_digits);
        self
    }
}

impl ops::MulAssign for U512 {
    fn mul_assign(&mut self, rhs: U512) {
        let mut new_digits = arithmetic::mul(&self.digits, &rhs.digits);
        new_digits.truncate(8);
        self.digits.clone_from_slice(&new_digits);
    }
}

impl ops::Rem for U512 {
    type Output = U512;
    fn rem(mut self, rhs: U512) -> U512 {
        let mut rem = [0u64; 8];
        let mut quot = [0u64; 8];
        arithmetic::div_rem(&self.digits, &rhs.digits, &mut quot, &mut rem);
        self.digits.clone_from_slice(&rem);
        self
    }
}

impl ops::RemAssign for U512 {
    fn rem_assign(&mut self, rhs: U512) {
        let mut rem = [0u64; 8];
        let mut quot = [0u64; 8];
        arithmetic::div_rem(&self.digits, &rhs.digits, &mut quot, &mut rem);
        self.digits.clone_from_slice(&rem);
    }
}

impl ops::Div for U512 {
    type Output = U512;
    fn div(mut self, rhs: U512) -> U512 {
        let mut rem = [0u64; 8];
        let mut quot = [0u64; 8];
        arithmetic::div_rem(&self.digits, &rhs.digits, &mut quot, &mut rem);
        self.digits.clone_from_slice(&quot);
        self
    }
}

impl ops::DivAssign for U512 {
    fn div_assign(&mut self, rhs: U512) {
        let mut rem = [0u64; 8];
        let mut quot = [0u64; 8];
        arithmetic::div_rem(&self.digits, &rhs.digits, &mut quot, &mut rem);
        self.digits.clone_from_slice(&quot);
    }
}

impl ops::Shl<usize> for U512 {
    type Output = U512;
    fn shl(mut self, rhs: usize) -> U512 {
        arithmetic::shl(&mut self.digits, rhs);
        self
    }
}

impl ops::ShlAssign<usize> for U512 {
    fn shl_assign(&mut self, rhs: usize) {
        arithmetic::shl(&mut self.digits, rhs);
    }
}

impl ops::Shr<usize> for U512 {
    type Output = U512;
    fn shr(mut self, rhs: usize) -> U512 {
        arithmetic::shr(&mut self.digits, rhs);
        self
    }
}

impl ops::ShrAssign<usize> for U512 {
    fn shr_assign(&mut self, rhs: usize) {
        arithmetic::shr(&mut self.digits, rhs);
    }
}

impl ops::BitOr for U512 {
    type Output = U512;
    fn bitor(mut self, rhs: U512) -> U512 {
        arithmetic::bitor(&mut self.digits, &rhs.digits);
        self
    }
}

impl ops::BitOrAssign for U512 {
    fn bitor_assign(&mut self, rhs: U512) {
        arithmetic::bitor(&mut self.digits, &rhs.digits);
    }
}

impl ops::BitAnd for U512 {
    type Output = U512;
    fn bitand(mut self, rhs: U512) -> U512 {
        arithmetic::bitand(&mut self.digits, &rhs.digits);
        self
    }
}

impl ops::BitAndAssign for U512 {
    fn bitand_assign(&mut self, rhs: U512) {
        arithmetic::bitand(&mut self.digits, &rhs.digits);
    }
}

impl ops::BitXor for U512 {
    type Output = U512;
    fn bitxor(mut self, rhs: U512) -> U512 {
        arithmetic::bitxor(&mut self.digits, &rhs.digits);
        self
    }
}

impl ops::BitXorAssign for U512 {
    fn bitxor_assign(&mut self, rhs: U512) {
        arithmetic::bitxor(&mut self.digits, &rhs.digits);
    }
}

impl ops::Not for U512 {
    type Output = U512;
    fn not(mut self) -> U512 {
        arithmetic::bitnot(&mut self.digits);
        self
    }
}

impl fmt::Display for U512 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Think of a better way to print this...
        write!(f, "{:0>#018x}{:0>16x}{:0>16x}{:0>16x}",
               self.digits[3], self.digits[2],
               self.digits[1], self.digits[0])
    }
}

impl cmp::PartialEq for U512 {
    fn eq(&self, other: &U512) -> bool {
        self.digits == other.digits
    }

    fn ne(&self, other: &U512) -> bool {
        self.digits != other.digits
    }
}

impl cmp::Eq for U512 {}

impl cmp::Ord for U512 {
    fn cmp(&self, other: &U512) -> cmp::Ordering {
        arithmetic::cmp(&self.digits, &other.digits)
    }
}

impl cmp::PartialOrd for U512 {
    fn partial_cmp(&self, other: &U512) -> Option<cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let zero_1 = U512::zero();
        let zero_2 = U512::from_bytes_be(vec![0]);

        let five_1 = U512::from_u64(5);
        let five_2 = U512::from_bytes_be(vec![5]);

        assert_eq!(zero_1, zero_2);
        assert_eq!(five_1, five_2);
    }

    #[test]
    fn addition() {
        let x = U512::from_u64(10);
        let y = U512::from_u64(12);
        assert_eq!(x + y, U512::from_u64(22));

        let x = U512::from_u64(187236152);
        let y = U512::from_u64(187236152);
        assert_eq!(x, y + U512::zero());
    }

    #[test]
    fn subtraction() {
        let x = U512::from_u64(10);
        let y = U512::from_u64(12);
        assert_eq!(y - x, U512::from_u64(2));

        let x = U512::from_u64(7192478999);
        let y = U512::from_u64(7192478999);
        assert_eq!(x, y - U512::zero());
    }

    #[test]
    fn multiplication() {
        let x = U512::from_u64(20);
        let y = U512::from_u64(16);
        assert_eq!(x * y, U512::from_u64(20 * 16));

        let x = U512::from_u64(7192478999);
        let y = U512::from_u64(7192478999);
        assert_eq!(x, y * U512::from_u64(1));
    }

    #[test]
    fn remainder() {
        let x = U512::from_u64(13);
        let y = U512::from_u64(7);
        assert_eq!(y % x, U512::from_u64(7));

        for i in 0..60 {
            let x = U512::from_u64(13 + 7 * i);
            let y = U512::from_u64(7);
            assert_eq!(x % y, U512::from_u64(6));
        }
    }

    #[test]
    fn division() {
        for i in 0..45 {
            for j in 0..15 {
                let x = U512::from_u64(i);
                let y = U512::from_u64(j + 1);
                assert_eq!(x / y, U512::from_u64(i / (j + 1)));
            }
        }
    }

    #[test]
    fn shifts() {
        let x = U512::from_u64(1);
        let y = U512::from_u64(64);
        assert_eq!(x << 6, y);
        assert_eq!(y >> 6, x);

        let x = U512::from_bytes_be(vec![
            0x1b, 0xcc, 0x2c, 0x7b, 0x2c, 0x29, 0x41, 0x9d,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x21, 0x0a, 0x23, 0x28, 0xac, 0x0e, 0x53, 0x04,
        ]);

        let y = U512::from_bytes_be(vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b,
            0xcc, 0x2c, 0x7b, 0x2c, 0x29, 0x41, 0x9d, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21,
            0x0a, 0x23, 0x28, 0xac, 0x0e, 0x53, 0x04, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);

        assert_eq!(x << 72, y);
        assert_eq!(y >> 72, x);
    }
}
