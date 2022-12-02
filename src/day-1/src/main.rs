use std::io;
fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();
    let result = add_numbers(&input);
    println!("{result:?}")
}

pub fn add_numbers(numbers: &str) -> Option<i32> {
    if numbers.len() == 0 {
        None
    } else {
        let mut results = vec![];

        numbers
            .split('\n')
            .map(|s| {
                if let Ok(val) = s.parse::<i32>() {
                    Some(val)
                } else {
                    None
                }
            })
            .chain([None])
            .fold(0, |accum, item| {
                if let Some(val) = item {
                    accum + val
                } else {
                    results.push(accum);
                    0
                }
            });
        results.sort();
        results.reverse();
        Some(results[0])
    }
}

#[cfg(test)]
mod test {
    use crate::add_numbers;

    #[test]
    fn add_numbers_empty() {
        assert_eq!(add_numbers(""), None)
    }

    #[test]
    fn add_numbers_single() {
        assert_eq!(add_numbers("100"), Some(100))
    }

    #[test]
    fn add_numbers_multiple() {
        assert_eq!(add_numbers("100\n200"), Some(300))
    }

    #[test]
    fn add_numbers_single_multiple_groups() {
        assert_eq!(add_numbers("100\n\n200"), Some(200))
    }

    #[test]
    fn add_numbers_multiple_multiple_groups() {
        assert_eq!(add_numbers("100\n200\n\n300\n400"), Some(700))
    }
}
