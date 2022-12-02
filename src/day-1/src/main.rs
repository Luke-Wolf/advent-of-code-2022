use std::io;

fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();
    let top_calories = top_calories(&input);
    let top_3_calories = top_3_calories(&input);
    println!("top calories: {top_calories:?}");
    println!("top 3 calories: {top_3_calories:?}");
}

/// A certain number of calories are passed in with the format
/// 1000
/// 2000
///
/// 3000
/// 4000
///
/// Where elf 1 has calories 1000, and 2000 and elf 2 has calories 3000, and 4000
///
/// This function adds up the total calories per elf and returns that as a vector
/// after sorting and then reversing it so that the highest numbers come first
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

/// Gets the highest number of calories the elves are holding
pub fn top_calories(numbers: &str) -> Option<i32> {
    if let Some(result) = get_calories(numbers) {
        Some(result[0])
    } else {
        None
    }
}

/// Gets the combined total of the number of calories the top 3 elves are holding
pub fn top_3_calories(numbers: &str) -> Option<i32> {
    if let Some(result) = get_calories(numbers) {
        Some(result.iter().take(3).fold(0, |accum, item| accum + item))
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

    #[test]
    fn add_top_2_calories() {
        assert_eq!(top_3_calories("100\n\n200"), Some(300))
    }

    #[test]
    fn add_top_calories() {
        assert_eq!(top_3_calories("100"), Some(100))
    }
}
