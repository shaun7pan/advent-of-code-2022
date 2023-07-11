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

#[derive(Debug, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::List(l), Packet::List(r)) => l == r,
            (Packet::List(l), Packet::Number(r)) => l == &vec![Packet::Number(*r)],
            (Packet::Number(l), Packet::List(r)) => &vec![Packet::Number(*l)] == r,
            (Packet::Number(l), Packet::Number(r)) => l == r,
        }
    }
}
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

//[1,1,3,1,1]
//[1,1,5,1,1]
//
//[[1],[2,3,4]]
//[[1],4]
//
//[9]
//[[8,7,6]]
//
//[[4,4],4,4]
//[[4,4],4,4,4]
//
//[7,7,7,7]
//[7,7,7]
//
//[]
//[3]
//
//[[[]]]
//[[]]
//
//[1,[2,[3,[4,[5,6,7]]]],8,9]
//[1,[2,[3,[4,[5,6,0]]]],8,9]

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]"))
            .map(|vec| Packet::List(vec)),
        complete::u32.map(|n| Packet::Number(n)),
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
    let (_, mut pairs) = pairs(input).unwrap();
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
