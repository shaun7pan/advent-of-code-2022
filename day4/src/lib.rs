use std::ops::RangeInclusive;

use nom::bytes::complete::tag;
use nom::character::complete::{self, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

// parse `2-4` to `2..=4`
fn parse_single_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    // let (input, start) = complete::u32(input)?;
    // let (input, _) = tag("-")(input)?;
    // let (input, end) = complete::u32(input)?;

    let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;

    Ok((input, start..=end))
}

// parse '2-4, 3-6' to `[(2..=4), (3..=6)]`
fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, (left_range, right_range)) =
        nom::sequence::separated_pair(parse_single_range, tag(","), parse_single_range)(input)?;

    Ok((input, (left_range, right_range)))
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let (input, ranges) = separated_list1(newline, line)(input)?;
    Ok((input, ranges))
}

pub fn process_part1(input: &str) -> String {
    let (_, ranges) = parse_ranges(input).unwrap();

    ranges
        .iter()
        .filter(|(left, right)| {
            let right_contains_left_element = left.clone().into_iter().all(|x| right.contains(&x));
            let left_contains_right_element = right.clone().into_iter().all(|x| left.contains(&x));

            right_contains_left_element || left_contains_right_element
        })
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, ranges) = parse_ranges(input).unwrap();

    ranges
        .iter()
        .filter(|(left, right)| {
            let right_contains_left_element = left.clone().into_iter().any(|x| right.contains(&x));
            let left_contains_right_element = right.clone().into_iter().any(|x| left.contains(&x));

            right_contains_left_element || left_contains_right_element
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn it_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, 2.to_string());
    }

    #[test]

    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, 4.to_string());
    }
}
