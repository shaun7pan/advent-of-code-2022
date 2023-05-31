use std::{collections::BTreeSet, dbg, println};

pub fn process_part1(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect::<Vec<char>>();
    let chunk = chars
        .windows(4)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();

    (chunk.0 + 4).to_string()
}

pub fn process_part2(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect::<Vec<char>>();
    let chunk = chars
        .windows(14)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();

    (chunk.0 + 14).to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn it_works() {
        let result = process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, 5.to_string());
        let result = process_part1("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, 6.to_string());
        let result = process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, 10.to_string());
        let result = process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, 11.to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(result, 19.to_string());
        let result = process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, 23.to_string());
        let result = process_part2("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, 23.to_string());
        let result = process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, 29.to_string());
        let result = process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, 26.to_string());
    }
}
