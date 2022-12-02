use std::{collections::HashMap, io, ops::Add};

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl From<Play> for i32 {
    fn from(play: Play) -> Self {
        match play {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

impl Add<PlayOutcome> for Play {
    type Output = i32;

    fn add(self, rhs: PlayOutcome) -> Self::Output {
        Into::<i32>::into(self) + Into::<i32>::into(rhs)
    }
}

impl Add<Play> for i32 {
    type Output = i32;

    fn add(self, rhs: Play) -> Self::Output {
        self + Into::<i32>::into(rhs)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum PlayOutcome {
    Win,
    Draw,
    Lose,
}

impl From<PlayOutcome> for i32 {
    fn from(play_outcome: PlayOutcome) -> Self {
        match play_outcome {
            PlayOutcome::Win => 6,
            PlayOutcome::Draw => 3,
            PlayOutcome::Lose => 0,
        }
    }
}

impl Add<Play> for PlayOutcome {
    type Output = i32;

    fn add(self, rhs: Play) -> Self::Output {
        Into::<i32>::into(self) + Into::<i32>::into(rhs)
    }
}

impl Add<PlayOutcome> for i32 {
    type Output = i32;

    fn add(self, rhs: PlayOutcome) -> Self::Output {
        self + Into::<i32>::into(rhs)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Move {
    Play(Play),
    PlayOutcome(PlayOutcome),
}

impl From<Play> for Move {
    fn from(play: Play) -> Self {
        match play {
            Play::Paper => Move::Play(Play::Paper),
            Play::Rock => Move::Play(Play::Rock),
            Play::Scissors => Move::Play(Play::Scissors),
        }
    }
}

impl From<PlayOutcome> for Move {
    fn from(play_outcome: PlayOutcome) -> Self {
        match play_outcome {
            PlayOutcome::Win => Move::PlayOutcome(PlayOutcome::Win),
            PlayOutcome::Draw => Move::PlayOutcome(PlayOutcome::Draw),
            PlayOutcome::Lose => Move::PlayOutcome(PlayOutcome::Lose),
        }
    }
}

fn shoot(opponent: Play, player: Play) -> i32 {
    // Outcome:    Shape:
    // Win:  +6    Rock:     +1
    // Draw: +3    Paper:    +2
    // Lose: +0    Scissors: +3
    // (Opponent, Player)
    let plays = HashMap::from([
        //shape + outcome
        ((Play::Rock, Play::Rock), Play::Rock + PlayOutcome::Draw), //4
        ((Play::Rock, Play::Paper), Play::Paper + PlayOutcome::Win), //8
        (
            (Play::Rock, Play::Scissors),
            Play::Scissors + PlayOutcome::Lose,
        ), //3
        ((Play::Paper, Play::Rock), Play::Rock + PlayOutcome::Lose), //1
        ((Play::Paper, Play::Paper), Play::Paper + PlayOutcome::Draw), //5
        (
            (Play::Paper, Play::Scissors),
            Play::Scissors + PlayOutcome::Win,
        ), //9
        ((Play::Scissors, Play::Rock), Play::Rock + PlayOutcome::Win), //7
        (
            (Play::Scissors, Play::Paper),
            Play::Paper + PlayOutcome::Lose,
        ), //2
        (
            (Play::Scissors, Play::Scissors),
            Play::Scissors + PlayOutcome::Draw,
        ), //6
    ]);

    plays[&(opponent, player)]
}

fn shoot_2(opponent: Move, player: Move) -> i32 {
    // Outcome:    Shape:
    // Win:  +6    Rock:     +1
    // Draw: +3    Paper:    +2
    // Lose: +0    Scissors: +3
    // (Opponent, Player)
    let plays = HashMap::from([
        //shape + outcome
        (
            (Move::Play(Play::Rock), Move::PlayOutcome(PlayOutcome::Win)),
            PlayOutcome::Win + Play::Paper,
        ), //8
        (
            (Move::Play(Play::Rock), Move::PlayOutcome(PlayOutcome::Draw)),
            PlayOutcome::Draw + Play::Rock,
        ), //4
        (
            (Move::Play(Play::Rock), Move::PlayOutcome(PlayOutcome::Lose)),
            PlayOutcome::Lose + Play::Scissors,
        ), //3
        (
            (Move::Play(Play::Paper), Move::PlayOutcome(PlayOutcome::Win)),
            PlayOutcome::Win + Play::Scissors,
        ), //9
        (
            (
                Move::Play(Play::Paper),
                Move::PlayOutcome(PlayOutcome::Draw),
            ),
            PlayOutcome::Draw + Play::Paper,
        ), //5
        (
            (
                Move::Play(Play::Paper),
                Move::PlayOutcome(PlayOutcome::Lose),
            ),
            PlayOutcome::Lose + Play::Rock,
        ), //1
        (
            (
                Move::Play(Play::Scissors),
                Move::PlayOutcome(PlayOutcome::Win),
            ),
            PlayOutcome::Win + Play::Rock,
        ), //7
        (
            (
                Move::Play(Play::Scissors),
                Move::PlayOutcome(PlayOutcome::Draw),
            ),
            PlayOutcome::Draw + Play::Scissors,
        ), //6
        (
            (
                Move::Play(Play::Scissors),
                Move::PlayOutcome(PlayOutcome::Lose),
            ),
            PlayOutcome::Lose + Play::Paper,
        ), //2
    ]);

    plays[&(opponent, player)]
}

fn decode_line(input: &str) -> (Play, Play) {
    let mut result = input.split_whitespace().flat_map(|play| {
        play.chars().map(|c| match c {
            'A' | 'X' => Play::Rock,
            'B' | 'Y' => Play::Paper,
            'C' | 'Z' => Play::Scissors,
            _ => {
                panic!("Shouldn't Happen, Character is {c}")
            }
        })
    });
    (result.next().unwrap(), result.next().unwrap())
}

fn decode_line2(input: &str) -> (Move, Move) {
    let mut result = input.split_whitespace().flat_map(|play| {
        play.chars().map(|c| match c {
            'A' => Move::Play(Play::Rock),
            'B' => Move::Play(Play::Paper),
            'C' => Move::Play(Play::Scissors),
            'Z' => Move::PlayOutcome(PlayOutcome::Win),
            'Y' => Move::PlayOutcome(PlayOutcome::Draw),
            'X' => Move::PlayOutcome(PlayOutcome::Lose),
            _ => {
                panic!("Shouldn't Happen, Character is {c}")
            }
        })
    });
    (result.next().unwrap(), result.next().unwrap())
}

fn play_rps(input: &str) -> i32 {
    let mut result = 0;
    input
        .lines()
        .take_while(|line| line.len() > 0)
        .for_each(|line| {
            let play = decode_line(line);
            result += shoot(play.0, play.1)
        });
    result
}

fn play_rps2(input: &str) -> i32 {
    let mut result = 0;
    input
        .lines()
        .take_while(|line| line.len() > 0)
        .for_each(|line| {
            let play = decode_line2(line);
            result += shoot_2(play.0, play.1)
        });
    result
}

fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();
    let score1 = play_rps(&input);
    let score2 = play_rps2(&input);
    println!("score1: {score1}");
    println!("score2: {score2}");
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn rock_rock() {
        assert_eq!(play_rps("A X"), Play::Rock + PlayOutcome::Draw)
    }
    #[test]
    fn rock_paper() {
        assert_eq!(play_rps("A Y"), Play::Paper + PlayOutcome::Win)
    }
    #[test]
    fn rock_scissors() {
        assert_eq!(play_rps("A Z"), Play::Scissors + PlayOutcome::Lose)
    }
    #[test]
    fn paper_rock() {
        assert_eq!(play_rps("B X"), Play::Rock + PlayOutcome::Lose)
    }
    #[test]
    fn paper_paper() {
        assert_eq!(play_rps("B Y"), Play::Paper + PlayOutcome::Draw)
    }
    #[test]
    fn paper_scissors() {
        assert_eq!(play_rps("B Z"), Play::Scissors + PlayOutcome::Win)
    }
    #[test]
    fn scissors_rock() {
        assert_eq!(play_rps("C X"), Play::Rock + PlayOutcome::Win)
    }
    #[test]
    fn scissors_paper() {
        assert_eq!(play_rps("C Y"), Play::Paper + PlayOutcome::Lose)
    }
    #[test]
    fn scissors_scissors() {
        assert_eq!(play_rps("C Z"), Play::Scissors + PlayOutcome::Draw)
    }

    #[test]
    fn multiple_plays() {
        assert_eq!(
            play_rps("A Y\nB Z\nC X"),
            Play::Paper
                + PlayOutcome::Win
                + Play::Scissors
                + PlayOutcome::Win
                + Play::Rock
                + PlayOutcome::Win
        )
    }

    #[test]
    fn rock_lose() {
        assert_eq!(play_rps2("A X"), PlayOutcome::Lose + Play::Scissors)
    }
    #[test]
    fn rock_draw() {
        assert_eq!(play_rps2("A Y"), PlayOutcome::Draw + Play::Rock)
    }
    #[test]
    fn rock_win() {
        assert_eq!(play_rps2("A Z"), PlayOutcome::Win + Play::Paper)
    }
    #[test]
    fn paper_lose() {
        assert_eq!(play_rps2("B X"), PlayOutcome::Lose + Play::Rock)
    }
    #[test]
    fn paper_draw() {
        assert_eq!(play_rps2("B Y"), PlayOutcome::Draw + Play::Paper)
    }
    #[test]
    fn paper_win() {
        assert_eq!(play_rps2("B Z"), PlayOutcome::Win + Play::Scissors)
    }
    #[test]
    fn scissors_lose() {
        assert_eq!(play_rps2("C X"), PlayOutcome::Lose + Play::Paper)
    }
    #[test]
    fn scissors_draw() {
        assert_eq!(play_rps2("C Y"), PlayOutcome::Draw + Play::Scissors)
    }
    #[test]
    fn scissors_win() {
        assert_eq!(play_rps2("C Z"), PlayOutcome::Win + Play::Rock)
    }
    #[test]
    fn multiple_plays_shoot2() {
        assert_eq!(
            play_rps2("A Y\nB Z\nC X"),
            PlayOutcome::Draw
                + Play::Rock
                + PlayOutcome::Win
                + Play::Scissors
                + PlayOutcome::Lose
                + Play::Paper
        )
    }
}
