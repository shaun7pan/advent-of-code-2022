use day10::process_part2;
use std::{fs, println};

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part2(&file));
}
