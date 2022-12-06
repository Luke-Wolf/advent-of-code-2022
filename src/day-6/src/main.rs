use std::{collections::HashSet, io};

fn find_start_of_packet(input: &str) -> Option<(&str, usize)> {
    for i in 0..input.len() {
        let window = &input[i..i + 4];
        let char_set = window.chars().collect::<HashSet<char>>();
        if char_set.len() == 4 {
            return Some((window, i + 4));
        }
    }
    None
}

fn find_start_of_message(input: &str) -> Option<(&str, usize)> {
    for i in 0..input.len() {
        let window = &input[i..i + 14];
        let char_set = window.chars().collect::<HashSet<char>>();
        if char_set.len() == 14 {
            return Some((window, i + 14));
        }
    }
    None
}

fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();
    if let Some((start, index)) = find_start_of_packet(&input) {
        println!("Start of Packet: {start} at {index}");
    }
    if let Some((start, index)) = find_start_of_message(&input) {
        println!("Start of Message: {start} at {index}");
    }
}
