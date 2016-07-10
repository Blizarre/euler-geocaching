fn nb_divisors(nb: i64) -> i32 {
    return 0
}

fn digit_at(nb: i64, digit_index: i32) -> u8 {
    return 0
}

fn f(nb_divisors_min:i32, digit_index:i32) -> u8 {
    let mut n:i64 = 0;
    while nb_divisors(n) < nb_divisors_min {
        n += 1;
    }
    return digit_at(n, digit_index);
}

fn main() {
    println!("Hello, world!");
}
