#![feature(iter_array_chunks)]
use std::collections::{hash_map, HashMap};

pub fn process_part1(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .map(|line| {
            let middle = &line.len() / 2;
            let first_compartment = &line[..middle];
            let second_compartment = &line[middle..];

            let the_char = first_compartment
                .chars()
                .find(|c| second_compartment.contains(*c))
                .unwrap();

            letter_scores.get(&the_char).unwrap()
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let the_char = a
                .chars()
                .find(|c_char| b.contains(*c_char) && c.contains(*c_char))
                .unwrap();

            letter_scores.get(&the_char).unwrap()
        })
        .sum::<usize>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, 157.to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, 70.to_string());
    }
}
