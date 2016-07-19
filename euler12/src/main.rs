// https://projecteuler.net/problem=12

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
    let mut current_divisor:u64 = 2;
    let mut nb_current_divisor:u32 = 0;
    let mut nb_divisors:u32 = 1;

    while nb != 1 {
        while nb % current_divisor == 0 {
            nb_current_divisor += 1;
            nb /= current_divisor
        }
        nb_divisors *= 1 + nb_current_divisor;
        current_divisor += 1;
        nb_current_divisor = 0
    }
    return nb_divisors;
}

#[test]
fn test_euler12() {
    assert_eq!(euler12(2), 3);
    assert_eq!(euler12(4), 6);
    assert_eq!(euler12(6), 28);
}

fn euler12(nb_divisors_min:u32) -> u64 {
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
    return sum;
}

fn main() {
    euler12(1500);
}
