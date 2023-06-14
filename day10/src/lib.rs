use itertools::Itertools;

use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::complete::{self, newline};
use nom::sequence::preceded;
use nom::Parser;
use nom::{multi::separated_list1, IResult};
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Self::Noop => 1,
            Self::Add(_) => 2,
        }
    }
}
fn instruction_set(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, vecs) = separated_list1(
        newline,
        alt((
            (tag("noop").map(|_| Instruction::Noop)),
            preceded(tag("addx "), complete::i32).map(|num| Instruction::Add(num)),
        )),
    )(input)?;

    Ok((input, vecs))
}
pub fn process_part1(input: &str) -> String {
    let known_cycles = [20, 60, 100, 140, 180, 220];
    let mut scores: BTreeMap<u32, i32> = BTreeMap::new();

    let (_, instructions) = instruction_set(input).unwrap();
    let mut x: i32 = 1;
    let mut cycles: u32 = 0;

    for instruction in instructions.iter() {
        if known_cycles.contains(&(cycles + 1)) {
            scores.insert(cycles + 1, (cycles as i32 + 1) * x);
        };

        if known_cycles.contains(&(cycles + 2)) {
            scores.insert(cycles + 2, (cycles as i32 + 2) * x);
        };

        cycles += instruction.cycles();
        match instruction {
            Instruction::Noop => {}
            Instruction::Add(num) => {
                x += num;
            }
        }
    }

    dbg!(&scores);
    scores
        .iter()
        .map(|(_key, value)| value)
        .sum::<i32>()
        .to_string()
}

struct Computer {
    x: i32,
    cycles: u32,
    display_string: String,
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.display_string
                .chars()
                .chunks(40)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .join("\n")
        )
    }
}

struct Cycle<'a> {
    cycle: u32,
    pixel: u32,
    computer: &'a mut Computer,
}

// won't work without it
impl<'a> Drop for Cycle<'a> {
    fn drop(&mut self) {
        self.computer.cycles += 1;
    }
}

impl Computer {
    fn new() -> Self {
        Computer {
            x: 1,
            cycles: 0,
            display_string: "".to_string(),
        }
    }

    fn sprite_range(&self) -> RangeInclusive<i32> {
        (self.x - 1)..=(self.x + 1)
    }

    fn start_cycle(&mut self) -> Cycle {
        Cycle {
            cycle: self.cycles,
            pixel: self.cycles % 40,
            computer: self,
        }
    }

    fn interpret(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.cycles() {
            let cycle_guard = self.start_cycle();

            if cycle_guard
                .computer
                .sprite_range()
                .contains(&(cycle_guard.pixel as i32))
            {
                cycle_guard.computer.display_string.push_str("#");
            } else {
                cycle_guard.computer.display_string.push_str(".");
            }
        }

        match instruction {
            Instruction::Noop => {}
            Instruction::Add(n) => {
                self.x += n;
            }
        }
    }
}

pub fn process_part2(input: &str) -> String {
    let (_, instructions) = instruction_set(input).unwrap();

    let computer = instructions
        .iter()
        .fold(Computer::new(), |mut abc, instrction| {
            abc.interpret(instrction);
            abc
        });

    computer.to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 13140.to_string());
    }

    #[test]

    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(
            result,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string()
        );
    }
}
