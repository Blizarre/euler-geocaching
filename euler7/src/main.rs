// https://projecteuler.net/problem=12

#[test]
fn test_is_prime() {
    assert!(is_prime(7));
    assert!(is_prime(2));
    assert!(!is_prime(6));
    assert!(is_prime(13));
    assert!(!is_prime(15));
}

fn is_prime(nb: u64) -> bool {
    let mut i: u64 = 3;
    let max = (nb as f64).sqrt().ceil() as u64;
    while i <= max {
        if nb % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}

#[test]
fn test_nth_prime() {
    assert_eq!(nth_prime(7), 17);
    assert_eq!(nth_prime(6), 13);
    assert_eq!(nth_prime(5), 11);
}

fn nth_prime(wanted_index: u64) -> u64 {
    let mut current_index: u64 = 2;
    let mut nb: u64 = 3;

    while current_index < wanted_index {
        nb += 2;
        if is_prime(nb) {
            current_index += 1
        }
    }
    return nb;
}

fn main() {
    println!("Euler project:");
    println!("10001: {}", nth_prime(10001));
    println!("Geocaching project:");
    println!("362229 - 10: {}", nth_prime(362229) - nth_prime(10));
    println!("867: {}", nth_prime(867));
}
