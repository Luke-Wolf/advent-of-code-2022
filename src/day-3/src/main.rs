use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io,
};

fn decode_line(input: &str) -> String {
    let (lhs, rhs) = input.split_at(input.len() / 2);
    let lhs_set = lhs.chars().collect::<HashSet<char>>();
    let rhs_set = rhs.chars().collect::<HashSet<char>>();
    let intersection = lhs_set.intersection(&rhs_set);
    let mut result = String::new();
    intersection.for_each(|c| result += &c.to_string());
    result
}

fn decode(x: &str, y: &str, z: &str) -> String {
    let x_set = x.chars().collect::<HashSet<char>>();
    let y_set = y.chars().collect::<HashSet<char>>();
    let z_set = z.chars().collect::<HashSet<char>>();
    let intersection = x_set
        .intersection(&y_set)
        .map(|c| c.clone())
        .collect::<HashSet<char>>();
    let intersection = intersection.intersection(&z_set);

    let mut result = String::new();
    intersection.for_each(|c| result += &c.to_string());

    result
}

fn calculate_priority(input: &str) -> i32 {
    let priority: HashMap<char, i32> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);
    input
        .chars()
        .map(|c| priority[&c])
        .reduce(|accum, item| accum + item)
        .unwrap()
}

fn calculate_rearrangement(input: &str) -> i32 {
    input
        .lines()
        .map(|l| decode_line(l))
        .map(|l| calculate_priority(&l))
        .reduce(|accum, item| accum + item)
        .unwrap()
}

fn calculate_rearrangement_2(input: &str) -> i32 {
    input
        .lines()
        .batching(|it| match it.next() {
            None => None,
            Some(x) => match it.next() {
                None => None,
                Some(y) => match it.next() {
                    None => None,
                    Some(z) => Some((x, y, z)),
                },
            },
        })
        .map(|(x, y, z)| decode(x, y, z))
        .map(|l| calculate_priority(&l))
        .reduce(|accum, item| accum + item)
        .unwrap()
}

fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();
    let rearrangement_cost = calculate_rearrangement(&input);
    let group_cost = calculate_rearrangement_2(&input);
    println!("rearrangement cost: {rearrangement_cost}");
    println!("group cost: {group_cost}");
}
