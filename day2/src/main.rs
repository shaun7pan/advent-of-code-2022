use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
pub enum HandResult {
    Win,
    Lose,
    Draw,
}

impl Hand {
    fn beats(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
    fn lose(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

fn main() {
    let mut score_board_part1: Vec<i32> = Vec::new();
    let mut score_board_part2: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let score_per_round_part1 = get_score_per_round(&l, "hand");
                score_board_part1.push(score_per_round_part1);

                let score_per_round_part2 = get_score_per_round(&l, "round_end");
                score_board_part2.push(score_per_round_part2);
            }
        }
    }
    println!("Part1: {:?}", score_board_part1.iter().sum::<i32>());
    println!("Part2: {:?}", score_board_part2.iter().sum::<i32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_score_per_round(s: &str, t: &str) -> i32 {
    match t {
        "hand" => second_column_as_hand(s),
        "round_end" => second_column_as_the_end(s),
        _ => todo!(),
    }
}

fn second_column_as_hand(s: &str) -> i32 {
    let (opponent_hand, own_hand) = convert_string_to_tuple_of_hands(s);
    let round_end = match (&opponent_hand, &own_hand) {
        _ if opponent_hand.beats() == own_hand => HandResult::Lose,
        _ if opponent_hand == own_hand.beats() => HandResult::Win,
        _ => HandResult::Draw,
    };

    get_score(own_hand, round_end)
}

fn second_column_as_the_end(s: &str) -> i32 {
    let (opponent_hand, round_end) = convert_string_to_tuple_of_hand_and_handresult(s);
    let own_hand = match &round_end {
        HandResult::Lose => opponent_hand.beats(),
        HandResult::Draw => opponent_hand,
        HandResult::Win => opponent_hand.lose(),
    };

    get_score(own_hand, round_end)
}

fn get_score(own_hand: Hand, round_end: HandResult) -> i32 {
    let own_hand_score = match own_hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };

    match round_end {
        HandResult::Win => 6 + own_hand_score,
        HandResult::Draw => 3 + own_hand_score,
        HandResult::Lose => own_hand_score,
    }
}

fn convert_string_to_tuple_of_hands(s: &str) -> (Hand, Hand) {
    let mut iter = s.split(" ");
    let opponent_hand = convert_hand_from_string(iter.next().unwrap());
    let own_hand = convert_hand_from_string(iter.next().unwrap());
    (opponent_hand, own_hand)
}

fn convert_string_to_tuple_of_hand_and_handresult(s: &str) -> (Hand, HandResult) {
    let mut iter = s.split(" ");
    let opponent_hand = convert_hand_from_string(iter.next().unwrap());
    let round_end = convert_end_from_string(iter.next().unwrap());
    (opponent_hand, round_end)
}

fn convert_hand_from_string(s: &str) -> Hand {
    match s {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => todo!(),
    }
}

fn convert_end_from_string(s: &str) -> HandResult {
    match s {
        "X" => HandResult::Lose,
        "Y" => HandResult::Draw,
        "Z" => HandResult::Win,
        _ => todo!(),
    }
}
