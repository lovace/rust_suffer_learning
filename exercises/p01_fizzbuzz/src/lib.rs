pub fn fizzbuzz(n: u32) -> Vec<String> {
    let mut result = Vec::with_capacity(n as usize);

    for i in 1..=n {
        match i {
            d if d % 3 == 0 && d % 5 == 0 => {
                result.push("FizzBuzz".to_string());
            }
            d if d % 3 == 0 => {
                result.push("Fizz".to_string());
            }
            d if d % 5 == 0 => {
                result.push("Buzz".to_string());
            }
            _ => {
                result.push(i.to_string());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(
            fizzbuzz(20),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz"
            ]
        );
    }
}
