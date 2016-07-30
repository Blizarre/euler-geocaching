// https://projecteuler.net/problem=3

use std::fmt;

#[derive(Debug, PartialEq)]
struct Result {
    f: u64,
    g: u64,
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "f: {:4}; g: {:4}", self.f, self.g)
    }
}

#[test]
fn test_euler3_geo() {
    assert_eq!(euler3_geo(100), Result { f: 5, g: 14 });
}

fn euler3_geo(mut nb: u64) -> Result {
    let mut current_divisor: u64 = 2;
    let mut sum_prime_divisor: u64 = 0;
    let mut max_prime_divisor: u64 = 0;

    while nb != 1 {
        while nb % current_divisor == 0 {
            max_prime_divisor = current_divisor;
            sum_prime_divisor += current_divisor;
            nb /= current_divisor
        }
        current_divisor += 1;
    }
    return Result {
        f: max_prime_divisor,
        g: sum_prime_divisor,
    };
}

fn main() {
    println!("Euler project:");
    println!("{}", euler3_geo(600851475143).f);
    println!("geocaching project:");
    println!("{}", euler3_geo(67108864));
    println!("{}", euler3_geo(36));
    println!("{}", euler3_geo(475262344414189));
    println!("{}", euler3_geo(16));
    println!("{}", euler3_geo(17592567));
}
