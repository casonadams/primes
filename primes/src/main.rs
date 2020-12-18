fn main() {
    println!("{:?}", find_prime_numbers_to(1_000));
}

fn find_prime_numbers_to(n: i16) -> Vec<i16> {
    let mut v = vec![];
    for i in 2..=n {
        if i == 2 || i == 3 || i == 5 {
            v.push(i);
            continue;
        } else if i % 2 == 0 || i % 3 == 0 || i % 5 == 0 {
            continue;
        }
        v.push(i);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::find_prime_numbers_to;

    #[test]
    fn test_find_prine_numbers_to() {
        let actual = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let expected = find_prime_numbers_to(30);

        assert_eq!(expected, actual);
    }
}
