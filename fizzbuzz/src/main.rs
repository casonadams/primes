fn main() {
    println!("{:?}", fizzbuzz(100));
}

fn fizzbuzz(n: i16) -> Vec<String> {
    let mut v = vec![];
    for i in 1..=n {
        if i % 15 == 0 {
            v.push("fizzbuzz".to_string());
        } else if i % 3 == 0 {
            v.push("fizz".to_string());
        } else if i % 5 == 0 {
            v.push("buzz".to_string());
        } else {
            v.push(format!("{}", i));
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::fizzbuzz;

    #[test]
    fn test_fizzbuzz() {
        let actual = fizzbuzz(15);
        let expected = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz",
        ];

        assert_eq!(expected, actual);
    }
}
