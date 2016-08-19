extern crate crypto_int;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crypto_int::U512;

fn hex_char_to_u8(c: u8) -> u8 {
    if 48 <= c && c <= 57 {
        return c - 48
    } else if 97 <= c && c <= 102 {
        return c - 97 + 10
    } else {
        println!("What the fuck {}", c);
        panic!();
    }
}

fn byte_str_to_bytes(s: &str) -> Vec<u8> {
    // Okay...
    let mut x = s.as_bytes();
    let mut v: Vec<u8> = Vec::new();

    if s.len() % 2 == 1 {
        v.push(hex_char_to_u8(x[0]));
        x = &x[1..];
    }

    for i in 0..(x.len() / 2) {
        let b1 = hex_char_to_u8(x[i*2]) << 4;
        let b2 = hex_char_to_u8(x[i*2 + 1]);
        v.push(b1 | b2);
    }
    v
}

fn byte_str_to_u512(s: &str) -> U512 {
    U512::from_bytes_be(byte_str_to_bytes(&s))
}

#[test]
fn basic_interface() {
    let x = U512::from_u64(10);
    let y = U512::from_bytes_be(vec![0x0a]);
    assert_eq!(U512::from_u64(20), x + y);
}

#[test]
fn addition_interface() {
    let file = match File::open("./tests/addition.data") {
        Ok(fh) => fh,
        Err(_) => panic!(),
    };

    let file = BufReader::new(file);
    for line in file.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => panic!(),
        };
        let v: Vec<&str> = line.split("\t").collect();
        let x = byte_str_to_u512(v[0].trim());
        let y = byte_str_to_u512(v[1].trim());
        let ans = byte_str_to_u512(v[2].trim());
        assert_eq!(x + y, ans);
    }
}

#[test]
fn subtraction_interface() {
    let file = match File::open("./tests/subtraction.data") {
        Ok(fh) => fh,
        Err(_) => panic!(),
    };

    let file = BufReader::new(file);
    for line in file.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => panic!(),
        };
        let v: Vec<&str> = line.split("\t").collect();
        let x = byte_str_to_u512(v[0].trim());
        let y = byte_str_to_u512(v[1].trim());
        let ans = byte_str_to_u512(v[2].trim());
        assert_eq!(x - y, ans);
    }
}

#[test]
fn multiplication_interface() {
    let file = match File::open("./tests/multiplication.data") {
        Ok(fh) => fh,
        Err(_) => panic!(),
    };

    let file = BufReader::new(file);
    for (i, line) in file.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => panic!(),
        };
        let v: Vec<&str> = line.split("\t").collect();
        let x = byte_str_to_u512(v[0].trim());
        let y = byte_str_to_u512(v[1].trim());
        let ans = byte_str_to_u512(v[2].trim());
        assert_eq!(x * y, ans);
        //assert!(x * y == ans, "i: {}", i);
    }
}

#[test]
fn division_interface() {
    let file = match File::open("./tests/division.data") {
        Ok(fh) => fh,
        Err(_) => panic!(),
    };

    let file = BufReader::new(file);
    for line in file.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => panic!(),
        };
        let v: Vec<&str> = line.split("\t").collect();
        let x = byte_str_to_u512(v[0].trim());
        let y = byte_str_to_u512(v[1].trim());
        let ans = byte_str_to_u512(v[2].trim());
        assert_eq!(x / y, ans);
    }
}

#[test]
fn remainder_interface() {
    let file = match File::open("./tests/remainder.data") {
        Ok(fh) => fh,
        Err(_) => panic!(),
    };

    let file = BufReader::new(file);
    for line in file.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => panic!(),
        };
        let v: Vec<&str> = line.split("\t").collect();
        let x = byte_str_to_u512(v[0].trim());
        let y = byte_str_to_u512(v[1].trim());
        let ans = byte_str_to_u512(v[2].trim());
        assert_eq!(x % y, ans);
    }
}
