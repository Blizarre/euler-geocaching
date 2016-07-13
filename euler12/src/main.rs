#[test]
fn test_nb_divisors() {
    assert_eq!(nb_divisors(1), 1);
    assert_eq!(nb_divisors(3), 2);
    assert_eq!(nb_divisors(6), 4);
    assert_eq!(nb_divisors(10), 4);
    assert_eq!(nb_divisors(15), 4);
    assert_eq!(nb_divisors(21), 4);
    assert_eq!(nb_divisors(28), 6);
}

fn nb_divisors(nb: i64) -> i32 {
    if nb == 1 {
        return 1
    }
    let mut nb_div: i32 = 2; // 1 and self
    let mut divisor: i64 = 2;
    while divisor < nb {
        if nb % divisor == 0 {
            nb_div += 1;
        }
        divisor += 1;
    }
    return nb_div
}


#[test]
fn test_digit_at() {
    assert_eq!(digit_at(123, 1), 1u8);
    assert_eq!(digit_at(123, 2), 2u8);
    assert_eq!(digit_at(123, 3), 3u8);
    assert_eq!(digit_at(2, 1), 2u8);
    assert_eq!(digit_at(23409, 3), 4u8);
    assert_eq!(digit_at(23545, 5), 5u8);
}

fn digit_at(nb: i64, digit_index: usize) -> u8 {
    let digit = nb.to_string().chars().nth(digit_index - 1).unwrap();
    return digit.to_string().parse::<u8>().unwrap();
}

#[test]
fn test_f() {
    assert_eq!(f(2,1), 3);
    assert_eq!(f(4,1), 6);
    assert_eq!(f(4,1), 6);
    assert_eq!(f(6,1), 2);
    assert_eq!(f(6,2), 8);
}

fn f(nb_divisors_min:i32, digit_index:usize) -> u8 {
    let mut n:i64 = 2;
    let mut sum:i64 = 1;
    let mut nb_div:i32 = 1;
    let mut nb_div_max:i32 = 1;
    while nb_div < nb_divisors_min {
        sum += n;
        n += 1;
        nb_div = nb_divisors(sum);
        if nb_div > nb_div_max {
            println!("{} divisors for n={}", nb_div, sum);
            nb_div_max = nb_div;
        }
    }
    println!("\nn={}, has {} divisor", sum, nb_divisors(sum));
    return digit_at(sum, digit_index);
}

fn main() {
    println!("f(500, 1)={}", f(500, 1));
}
