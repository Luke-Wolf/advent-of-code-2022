use std::io;
fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();
    let top_calories = top_calories(&input);
    let top_3_calories = top_3_calories(&input);
    println!("top calories: {top_calories:?}");
    println!("top 3 calories: {top_3_calories:?}");
}

pub fn get_calories(numbers: &str) -> Option<Vec<i32>> {
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
        Some(results)
    }
}

pub fn top_calories(numbers: &str) -> Option<i32> {
    if let Some(result) = get_calories(numbers) {
        Some(result[0])
    } else {
        None
    }
}

pub fn top_3_calories(numbers: &str) -> Option<i32> {
    if let Some(result) = get_calories(numbers) {
        Some(result[0..3].iter().fold(0, |accum, item| accum + item))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn add_numbers_empty() {
        assert_eq!(top_calories(""), None)
    }

    #[test]
    fn add_numbers_single() {
        assert_eq!(top_calories("100"), Some(100))
    }

    #[test]
    fn add_numbers_multiple() {
        assert_eq!(top_calories("100\n200"), Some(300))
    }

    #[test]
    fn add_numbers_single_multiple_groups() {
        assert_eq!(top_calories("100\n\n200"), Some(200))
    }

    #[test]
    fn add_numbers_multiple_multiple_groups() {
        assert_eq!(top_calories("100\n200\n\n300\n400"), Some(700))
    }

    #[test]
    fn add_top_3_calories() {
        assert_eq!(top_3_calories("100\n\n200\n\n300"), Some(600))
    }
}
