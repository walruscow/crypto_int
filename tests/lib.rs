extern crate crypto_int;

use crypto_int::U512;

#[test]
fn basic() {
    let x = U512::from_u64(10);
    let y = U512::from_bytes_be(vec![0x0a]);
    assert_eq!(U512::from_u64(20), x + y);
}
