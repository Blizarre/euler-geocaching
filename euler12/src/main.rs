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

fn nb_divisors(mut nb: u64) -> u32 {
    if nb == 1 {
        return 1;
    }
    let mut current_divisor:u64 = 2;
    let mut previous_divisor:u64 = 2;
    let mut nb_current_divisor:u32 = 0;
    let mut nb_divisors:u32 = 1;

    while current_divisor <= nb {
        if nb % current_divisor == 0 {
            if previous_divisor == current_divisor {
                nb_current_divisor += 1;
            } else {
                previous_divisor = current_divisor;
                nb_divisors *= 1 + nb_current_divisor;
                nb_current_divisor = 1;
            }
            nb /= current_divisor;
            current_divisor = 2;
        }
        else {
            current_divisor += 1;
        }
    }
    nb_divisors *= 1 + nb_current_divisor;
    return nb_divisors;
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

fn digit_at(nb: u64, digit_index: usize) -> u8 {
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

fn f(nb_divisors_min:u32, digit_index:usize) -> u8 {
    let mut n:u64 = 2;
    let mut sum:u64 = 1;
    let mut nb_div:u32 = 1;
    let mut nb_div_max:u32 = 1;
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
