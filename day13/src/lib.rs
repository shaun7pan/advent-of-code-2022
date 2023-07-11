use std::vec;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

#[derive(Debug, Eq, PartialEq)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

// The reason why we have to implement PartialOrd explicitly rather than derive the trait is
// because the derive one trying to use `lt` (lessthan) which is incosistence with what we had done
// in `Ord` implementation below.
//
// It’s easy to accidentally make cmp and partial_cmp disagree by deriving some of the traits and manually implementing others.
// https://doc.rust-lang.org/std/cmp/trait.Ord.html
//
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(b),
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
        }
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]")).map(Packet::List),
        complete::u32.map(Packet::Number),
    ))(input)
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
pub struct Pair {
    left: Packet,
    right: Packet,
}
fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, tag("\n"), packet).map(|(left, right)| Pair { left, right }),
    )(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, pairs) = pairs(input).unwrap();
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p.left.cmp(&p.right) {
            std::cmp::Ordering::Less => Some(i),
            std::cmp::Ordering::Equal => panic!("asfsdf"),
            std::cmp::Ordering::Greater => None,
        })
        .map(|i| i + 1)
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, pairs) = pairs(input).unwrap();
    let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);

    let mut packets: Vec<&Packet> = pairs
        .iter()
        .flat_map(|Pair { left, right }| vec![left, right])
        .chain(vec![&packet_2, &packet_6])
        .collect();

    packets.sort();
    let index_2 = packets
        .iter()
        .position(|&packet| packet == &packet_2)
        .unwrap();
    let index_6 = packets
        .iter()
        .position(|&packet| packet == &packet_6)
        .unwrap();
    dbg!(index_2);
    dbg!(index_6);
    ((index_2 + 1) * (index_6 + 1)).to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = include_str!("../test.txt");
    #[test]

    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 13.to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 140.to_string());
    }
}
