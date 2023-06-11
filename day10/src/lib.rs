use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
enum Action {
    Noop,
    Addx(i32),
}

fn addx(input: &str) -> IResult<&str, Action> {
    let (input, (_, count)) = separated_pair(tag("addx"), tag(" "), i32)(input)?;
    Ok((input, Action::Addx(count)))
}

fn noop(input: &str) -> IResult<&str, Action> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Action::Noop))
}

fn parse_actions(input: &str) -> IResult<&str, Vec<Action>> {
    let (input, actions) = separated_list1(newline, alt((addx, noop)))(input)?;
    Ok((input, actions))
}
pub fn process_part1(input: &str) -> String {
    let (_, actions) = parse_actions(input).unwrap();
    let mut result = BTreeMap::new();
    let mut cycle = 1;
    let mut value = 1;

    actions.iter().for_each(|action| match action {
        Action::Addx(n) => {
            // addx cycle 1 insert last value, cycle 2 insert new value
            cycle += 1;
            result.entry(cycle).or_insert(value);
            cycle += 1;
            value += n;
            result.entry(cycle).or_insert(value);
        }
        Action::Noop => {
            // noop always insert last value
            cycle += 1;
            result.entry(cycle).or_insert(value);
        }
    });

    let source = vec![20, 60, 100, 140, 180, 220];
    source
        .iter()
        .fold(0, |acc, &x| acc + (result.get(&x.clone()).unwrap() * x))
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, actions) = parse_actions(input).unwrap();
    let mut result = BTreeMap::new();
    let mut cycle = 1;
    let mut value = 1; // value of x
    let mut cur = 0; // current position
    let mut sprite_range = 0..=0;

    // println!("Sprite position: {}..{}", value - 1, value + 1);
    actions.iter().for_each(|action| {
        // println!("The value: {}", value);

        if cur == 40 {
            cur = 0
        }

        sprite_range = (value - 1)..=(value + 1);

        match action {
            Action::Addx(n) => {
                // addx cycle 1 insert last value, cycle 2 insert new value
                // println!("Start cycle  {cycle}: begin executing addx: {}", n);
                // println!("During cycle {cycle}: CRT draws pixel in position: {}", cur);

                if sprite_range.contains(&cur) {
                    result.entry(cycle).or_insert("#");
                    // println!("#");
                } else {
                    result.entry(cycle).or_insert(".");
                    // println!(".");
                }

                cycle += 1;
                cur += 1;
                if cur == 40 {
                    cur = 0
                }

                // println!("During cycle {cycle}: CRT draws pixel in position: {}", cur);
                value += n;
                // println!(
                //     "End of cycle {cycle}: finish executing addx {n} (Register X is now {})",
                //     value
                // );
                if sprite_range.contains(&cur) {
                    result.entry(cycle).or_insert("#");
                    // println!("#");
                } else {
                    result.entry(cycle).or_insert(".");
                    // println!(".");
                }
                // println!("Sprite position: {:?}", sprite_range);
                cycle += 1;
            }
            Action::Noop => {
                // noop always insert last value
                // println!("Start cycle  {cycle}: begin executing noop");
                // println!("CRT draws pixel in position: {}", cur);
                if sprite_range.contains(&cur) {
                    result.entry(cycle).or_insert("#");
                    // println!("#");
                } else {
                    result.entry(cycle).or_insert(".");
                    // println!(".");
                }
                // println!("End of cycle {cycle}: finish executing noop");
                cycle += 1;
            }
        }
        cur += 1;
    });

    let mut result_str = String::new();
    let mut result_str_array: Vec<_> = vec![];

    result.values().enumerate().for_each(|(ind, value)| {
        result_str.push_str(value as &str);
        if (ind + 1) % 40 == 0 {
            result_str_array.push(result_str.clone());
            result_str.clear();
        }
    });

    // result_str_array.iter().for_each(|x| println!("{}", x));

    let mut final_answer = String::new();
    for s in result_str_array.iter() {
        final_answer.push_str(s);
        final_answer.push_str("\n");
    }

    final_answer.trim_end().to_string()
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
