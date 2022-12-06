use std::io;

struct Command {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl Command {
    pub fn new(count: usize, from: usize, to: usize) -> Self {
        Command { count, from, to }
    }

    pub fn from_str(input: &str) -> Self {
        let mut itr = input.split_whitespace();
        // Move
        itr.next();
        // Count
        let count = itr.next().unwrap().parse::<usize>().unwrap();
        // From
        itr.next();
        // From #
        let from = itr.next().unwrap().parse::<usize>().unwrap();
        // To
        itr.next();
        // To #
        let to = itr.next().unwrap().parse::<usize>().unwrap();

        Command::new(count, from, to)
    }
}

#[derive(Debug, Clone)]
struct Crates {
    crates: Vec<Vec<String>>,
}

impl Crates {
    pub fn new(input: &str) -> Self {
        let mut iter = input.lines().rev();
        let columns = iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut crates = vec![vec![]; columns];
        iter.for_each(|line| {
            let iter = line.chars();
            iter.skip(1)
                .step_by(4)
                .enumerate()
                .for_each(|(index, value)| {
                    if value != ' ' {
                        crates[index].push(value.to_string());
                    }
                });
        });
        Crates { crates }
    }
    pub fn move_crate(&mut self, from: usize, to: usize) {
        let crate_ = self.crates[from - 1].pop();
        self.crates[to - 1].push(crate_.unwrap());
    }

    pub fn move_multiple_crates(&mut self, count: usize, from: usize, to: usize) {
        let mut crates = vec![];
        for _ in 0..count {
            crates.push(self.crates[from - 1].pop().unwrap())
        }

        crates
            .iter()
            .rev()
            .for_each(|_crate| self.crates[to - 1].push(_crate.clone()));
    }

    pub fn top_crates(&self) -> String {
        self.crates
            .iter()
            .map(|stack| stack.last())
            .fold(String::new(), |accum, top_crate| {
                if let Some(top_crate) = top_crate {
                    accum + top_crate
                } else {
                    accum
                }
            })
    }
}

fn split_input(input: &str) -> (&str, &str) {
    input.split_once("\n\n").unwrap()
}

fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();
    let (crates, commands) = split_input(&input);
    let mut crates = Crates::new(crates);
    let commands = commands.lines().map(|line| Command::from_str(line));

    let mut crates_2 = crates.clone();

    commands.for_each(|command| {
        for _ in 0..command.count {
            crates.move_crate(command.from, command.to);
        }
        crates_2.move_multiple_crates(command.count, command.from, command.to);
    });

    let top_crates = crates.top_crates();
    let top_crates_2 = crates_2.top_crates();
    println!("Top Crates: {top_crates}");
    println!("Top Crates Move Multiple: {top_crates_2}");
}

#[cfg(test)]
mod Tests {
    use crate::*;

    #[test]
    fn decode_move() {
        let command = Command::from_str("move 1 from 1 to 2");
        assert_eq!(command.count, 1);
        assert_eq!(command.from, 1);
        assert_eq!(command.to, 2);
    }

    #[test]
    fn decode_one_stack_of_crates() {
        let crates = create_one_stack_of_crates();
        assert_eq!(crates.crates.len(), 1);
        assert_eq!(crates.crates[0].len(), 1);
    }

    #[test]
    fn decode_multiple_stacks_of_crates() {
        let crates = create_multiple_stacks_of_crates();
        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 1);
        assert_eq!(crates.crates[1].len(), 1);
    }

    #[test]
    fn decode_multiple_stacks_of_crates_with_depth() {
        let crates = create_multiple_stacks_of_crates_with_depth();
        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 2);
        assert_eq!(crates.crates[1].len(), 1);
    }

    #[test]
    fn move_crate_from_one_stack_to_another() {
        let mut crates = create_multiple_stacks_of_crates_with_depth();
        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 2);
        assert_eq!(crates.crates[1].len(), 1);
        crates.move_crate(1, 2);
        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 1);
        assert_eq!(crates.crates[1].len(), 2);
    }

    #[test]
    fn move_multiple_crates_from_one_stack_to_another() {
        let mut crates = create_multiple_stacks_of_crates_with_depth();
        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 2);
        assert_eq!(crates.crates[1].len(), 1);
        crates.move_multiple_crates(2, 1, 2);
        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 0);
        assert_eq!(crates.crates[1].len(), 3);
        assert_eq!(crates.crates[1][0], "C");
        assert_eq!(crates.crates[1][1], "B");
        assert_eq!(crates.crates[1][2], "A");
    }

    #[test]
    fn get_top_crates_one_stack() {
        let crates = create_one_stack_of_crates();
        assert_eq!(crates.top_crates(), "A");
    }

    #[test]
    fn get_top_crates_multiple() {
        let crates = create_multiple_stacks_of_crates();
        assert_eq!(crates.top_crates(), "AB");
    }

    fn get_top_crates_with_empty() {
        let crates = create_multiple_stacks_of_crates_with_missing();
        assert_eq!(crates.top_crates(), "AB");
    }

    #[test]
    fn run_command() {
        let command = create_command();
        let mut crates = create_multiple_stacks_of_crates_with_depth();
        for i in 0..command.count {
            crates.move_crate(command.from, command.to);
        }
        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 1);
        assert_eq!(crates.crates[1].len(), 2);
    }

    #[test]
    fn command_with_crates() {
        let input = "[A]    \n[B] [C]\n 1   2 \n\nmove 1 from 1 to 2";
        let (crates, command) = split_input(input);
        let mut crates = Crates::new(crates);
        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 2);
        assert_eq!(crates.crates[1].len(), 1);
        let command = Command::from_str(command);
        for i in 0..command.count {
            crates.move_crate(command.from, command.to);
        }

        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 1);
        assert_eq!(crates.crates[1].len(), 2);
    }

    fn create_one_stack_of_crates() -> Crates {
        Crates::new("[A]\n 1")
    }

    fn create_multiple_stacks_of_crates() -> Crates {
        Crates::new("[A] [B]\n 1   2 ")
    }
    fn create_multiple_stacks_of_crates_with_depth() -> Crates {
        Crates::new("[A]    \n[B] [C]\n 1   2 ")
    }
    fn create_multiple_stacks_of_crates_with_missing() -> Crates {
        Crates::new("[A]     [B]\n 1   2   3 ")
    }

    fn create_command() -> Command {
        Command::from_str("move 1 from 1 to 2")
    }
}
