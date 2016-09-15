// https://projecteuler.net/problem=13

use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::cmp;
use std::ops::Add;

#[test]
fn test_to_string() {
    let test_data = vec!["0", "1", "25663", "10000000000000"];
    for x in test_data {
        assert_eq!(*x,
                   fmt::format(
                       format_args!("{}", BigNumber::from_string(x).unwrap())
                       )
                  )
    }
}


fn test_add_equal(sa: &str, sb: &str, sapb: &str) {
    let a = BigNumber::from_string(sa).unwrap();
    let b = BigNumber::from_string(sb).unwrap();
    let apb = BigNumber::from_string(sapb).unwrap();

    let ab = a.clone() + b.clone();
    let ba = b.clone() + a.clone();
    assert_eq!(ab, apb);
    assert_eq!(ba, apb);
    assert!(ab != a);
}

#[test]
fn test_zero() {
    let a = BigNumber::from_string("12345").unwrap();
    let zero = BigNumber::zero();

    assert_eq!(a.clone() + zero.clone(), a);
    assert_eq!(zero.clone() + a.clone(), a);

    test_add_equal("0", "123", "123");
}

#[test]
fn test_add_same_length() {
    test_add_equal("100", "100", "200");
    test_add_equal("11111", "22222", "33333");
    test_add_equal("8", "1", "9");
    test_add_equal("82", "11", "93");
}



#[test]
fn test_carry() {
    test_add_equal("1", "999", "1000");
    test_add_equal("1", "9", "10");
    test_add_equal("9", "9", "18");
    test_add_equal("123", "98", "221");
}

#[test]
fn test_error() {
    let a = BigNumber::from_string("1%2");
    let b = BigNumber::from_string("12");
    assert!(a.is_err());
    assert!(b.is_ok());
}

#[test]
fn test_add_different_length() {
    test_add_equal("123", "12", "135");
    test_add_equal("123", "3", "126");
    test_add_equal("123333333", "3", "123333336");
    test_add_equal("76543", "1234", "77777");
}

#[derive(Debug, Clone)]
struct BigNumber {
    // Numbers are stored in reverse number in the Vector
    data: Vec<u8>,
}

impl BigNumber {
    fn zero() -> BigNumber {
        let zero_vec = vec![0u8];
        BigNumber { data: zero_vec }
    }

    fn char_to_u8(c: char) -> Result<u8, &'static str> {
        match c {
            '0' => Ok(0),
            '1' => Ok(1),
            '2' => Ok(2),
            '3' => Ok(3),
            '4' => Ok(4),
            '5' => Ok(5),
            '6' => Ok(6),
            '7' => Ok(7),
            '8' => Ok(8),
            '9' => Ok(9),
            _   => Err("Character is not a number")
        }
    }

    fn from_string(s: &str) -> Result<BigNumber, &'static str> {
        let array: Result<Vec<_>, &str> = s.chars().map( |x| BigNumber::char_to_u8(x)).rev().collect();

        match array {
            Ok(x)  => Ok(BigNumber::from_vec(x)),
            Err(e) => Err(e)
        }
    }

    fn from_vec(v: Vec<u8>) -> BigNumber {
        BigNumber { data:v }
    }

    fn iter(&self) -> std::slice::Iter<u8> {
        self.data[..self.len()].iter()
    }

    fn len(&self) -> usize {
        // ignore leading zeros in the BigNumber (stored in reverse, so ignore trailing zeros in the array)
        for (i, elt) in self.data.iter().enumerate().rev() {
            if *elt != 0 {
                return i + 1
            }
        }
        return 0
    }
}

impl PartialEq for BigNumber {
    fn eq(&self, other:&BigNumber) -> bool {
       return self.data[..self.len()] == other.data[..other.len()]
    }
}

impl fmt::Display for BigNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.len() == 0 {
            return write!(f, "0");
        } else {
            let result: Vec<String> = self.iter().rev()
                .map(|c| format!("{}", c))
                .collect();

            write!(f, "{}", result.join(""))
        }
    }
}

impl Add for BigNumber {
	type Output = BigNumber;
    fn add(self, other: BigNumber) -> BigNumber {
		let max_length = cmp::max(self.data.len(), other.data.len());
        let mut result = BigNumber{ data: Vec::with_capacity(max_length + 1)};
        let mut carry = 0u8;
        for i in 0..max_length {
            if i < self.data.len() {
                carry += self.data[i];
            }
            if i < other.data.len() {
                carry += other.data[i];
            }
            result.data.push(carry % 10);
            carry = carry / 10;
        }
        if carry > 0 {
            result.data.push(carry);
        }
        result
    }
}

fn main() {
    let mut f = File::open("euler.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let result = s.lines().map(|x| BigNumber::from_string(x).unwrap()).fold(BigNumber::zero(), |acc, x| acc + x);
    println!("{}", result);
}
