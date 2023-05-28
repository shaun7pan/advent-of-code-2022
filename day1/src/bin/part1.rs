use day1::process_part1;
use std::{fs, println};

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file));
}
