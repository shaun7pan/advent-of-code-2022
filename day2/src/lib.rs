use std::{panic, str::FromStr};

#[derive(PartialEq, Clone, Copy)] // PartialEq will give the result that paper beats rock, Scissors beats paper.
                                  // but not rock beats Scissors because the circle
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // other is winner
        if self == &Move::Scissors && other == &Move::Rock {
            Some(std::cmp::Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8))) // cmp means self < other or self > other,
        }
    }
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}
pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(std::cmp::Ordering::Equal) => 3 + moves[1] as u32,
                Some(std::cmp::Ordering::Less) => 6 + moves[1] as u32,
                Some(std::cmp::Ordering::Greater) => 0 + moves[1] as u32,
                None => panic!("not comparable"),
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let moves: Vec<_> = line.split(" ").collect();
            let opponent_move = moves[0].parse::<Move>().unwrap();

            match moves[1] {
                "X" => {
                    let our_move = match opponent_move {
                        Move::Rock => Move::Scissors,
                        Move::Paper => Move::Rock,
                        Move::Scissors => Move::Paper,
                    };

                    0 + our_move as u32
                }

                "Y" => {
                    let our_move = match opponent_move {
                        Move::Rock => Move::Rock,
                        Move::Paper => Move::Paper,
                        Move::Scissors => Move::Scissors,
                    };

                    3 + our_move as u32
                }

                "Z" => {
                    let our_move = match opponent_move {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissors,
                        Move::Scissors => Move::Rock,
                    };
                    6 + our_move as u32
                }
                _ => {
                    panic!("Unexpected response")
                }
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn it_works_part1() {
        let result = process_part1(&INPUT);
        assert_eq!(result, 15.to_string());
    }

    #[test]
    fn it_works_part2() {
        let result = process_part2(&INPUT);
        assert_eq!(result, 12.to_string());
    }
}
