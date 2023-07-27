use itertools::Itertools;
use lending_iterator::prelude::*;
use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::newline;
use nom::character::complete::u32;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn line(input: &str) -> IResult<&str, (&str, u32)> {
    let (input, (direction, count)) = separated_pair(alpha1, tag(" "), u32)(input)?;

    Ok((input, (direction, count)))
}

fn parse_step(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, lines) = separated_list1(newline, line)(input)?;
    let directions: Vec<_> = lines
        .iter()
        .flat_map(|(direction, count)| {
            vec![
                match *direction {
                    "U" => Direction::Up,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    &_ => todo!(),
                };
                *count as usize
            ]
        })
        .collect();

    Ok((input, directions))
}

pub fn process_part1(input: &str) -> String {
    //println!("{:?}", vec!["aaa"; 5]);
    let (_, vecs) = parse_step(input).unwrap();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_position_set = HashSet::from([tail]);

    for direction in vecs.iter() {
        match direction {
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
        }

        let x_range = (head.0 - 1)..=(head.0 + 1);
        let y_range = (head.1 - 1)..=(head.1 + 1);

        let connected = x_range
            .cartesian_product(y_range)
            .any(|position| position == tail);

        if !connected {
            //move
            let mut new_tail_position = head.clone();
            match direction {
                Direction::Left => new_tail_position.0 += 1,
                Direction::Right => new_tail_position.0 -= 1,
                Direction::Up => new_tail_position.1 -= 1,
                Direction::Down => new_tail_position.1 += 1,
            }
            tail = new_tail_position;
            tail_position_set.insert(new_tail_position);
        }
    }

    tail_position_set.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, vecs) = parse_step(input).unwrap();
    let mut rope = [(0, 0); 10];
    let mut tail_position_set = HashSet::from([*rope.last().unwrap()]);

    for direction in vecs.iter() {
        match direction {
            Direction::Left => rope[0].0 -= 1,
            Direction::Right => rope[0].0 += 1,
            Direction::Up => rope[0].1 += 1,
            Direction::Down => rope[0].1 -= 1,
        }

        let mut rope_windows = rope.windows_mut::<2>();
        while let Some([ref mut head, tail]) = rope_windows.next() {
            let x_range = (head.0 - 1)..=(head.0 + 1);
            let y_range = (head.1 - 1)..=(head.1 + 1);

            let connected = x_range
                .cartesian_product(y_range)
                .any(|position| position == *tail);

            if !connected {
                //move tail
                if head.0 == tail.0 {
                    // x is same: in vertical line
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                } else if head.1 == tail.1 {
                    // y is same, in the line
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                } else {
                    // diagonal
                    let x_range = (head.0 - 1)..=(head.0 + 1);
                    let y_range = (head.1 - 1)..=(head.1 + 1);

                    let head_3x3 = x_range.cartesian_product(y_range).collect::<Vec<_>>();

                    let x_range = (tail.0 - 1)..=(tail.0 + 1);
                    let y_range = (tail.1 - 1)..=(tail.1 + 1);

                    let maybe_new_tail: Vec<(i32, i32)> = x_range
                        .cartesian_product(y_range)
                        .filter(|tuple| head_3x3.contains(tuple))
                        .collect();

                    match maybe_new_tail.len() {
                        // match two cases:
                        // overlap is 2. This is the case for normal head and tail move.
                        2 => {
                            // the up/down/left/right of head
                            let new_head_cross_possitions = [
                                (head.0 - 1, head.1),
                                (head.0 + 1, head.1),
                                (head.0, head.1 - 1),
                                (head.0, head.1 + 1),
                            ];

                            let next = maybe_new_tail
                                .iter()
                                .find(|tuple| new_head_cross_possitions.contains(tuple))
                                .unwrap();
                            *tail = *next;
                        }
                        // overlap is 1, this is the case for node 5 moving from
                        // ......
                        // ......
                        // ....H.
                        // .4321.
                        // 5.....  (5 covers 6, 7, 8, 9, s)
                        //
                        // via
                        // ......
                        // ....H.
                        // ....1.
                        // .432..
                        // 5.....  (5 covers 6, 7, 8, 9, s)
                        //
                        //
                        // to
                        //
                        // ....H.
                        // ....1.
                        // ..432.
                        // .5....
                        // 6.....  (6 covers 7, 8, 9, s)
                        1 => {
                            *tail = maybe_new_tail[0];
                        }
                        _ => {
                            panic!("unknown tail length");
                        }
                    }
                }
            }
        }
        tail_position_set.insert(*rope.last().unwrap());
    }

    tail_position_set.len().to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 13.to_string());
    }

    #[test]

    fn part2_works() {
        let result = process_part2(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );
        assert_eq!(result, 36.to_string());
    }
}
