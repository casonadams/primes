fn main() {
    find_prime_numbers_to(1_000);
}

fn find_prime_numbers_to(n: i16) {
    for i in 2..=n {
        if i == 2 || i == 3 || i == 5 {
            println!("{}", i);
            continue;
        } else if i % 2 == 0 || i % 3 == 0 || i % 5 == 0 {
            continue;
        }
        println!("{}", i);
    }
}
