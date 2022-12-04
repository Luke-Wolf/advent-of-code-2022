use std::io;

fn decode_line(input: &str) -> ((i32, i32), (i32, i32)) {
    let mut intermediate = input.split(',').map(|part| {
        let mut numbers = part
            .split('-')
            .map(|s| s.parse::<i32>().unwrap())
            .into_iter();
        (numbers.next().unwrap(), numbers.next().unwrap())
    });
    (intermediate.next().unwrap(), intermediate.next().unwrap())
}

fn overlap(l_low: i32, l_high: i32, r_low: i32, r_high: i32) -> bool {
    if (l_low <= r_low && l_high >= r_high) || (r_low <= l_low && r_high >= l_high) {
        true
    } else {
        false
    }
}

fn overlaps_at_all(l_low: i32, l_high: i32, r_low: i32, r_high: i32) -> bool {
    if (l_low <= r_low && l_high >= r_low)
        || (l_low <= r_high && l_high >= r_high)
        || (r_low <= l_low && r_high >= l_low)
        || (r_low <= l_high && r_high >= l_high)
    {
        true
    } else {
        false
    }
}

fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();

    let overlaps = input
        .lines()
        .map(|line| decode_line(&line))
        .map(|((l_low, l_high), (r_low, r_high))| overlap(l_low, l_high, r_low, r_high))
        .fold(0, |accum, item| if item { accum + 1 } else { accum });

    let overlaps_at_all = input
        .lines()
        .map(|line| decode_line(&line))
        .map(|((l_low, l_high), (r_low, r_high))| overlaps_at_all(l_low, l_high, r_low, r_high))
        .fold(0, |accum, item| if item { accum + 1 } else { accum });

    println!("# of Subset Overlaps {overlaps}");
    println!("# of Overlaps at all {overlaps_at_all}");
}
