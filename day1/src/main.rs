use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result = get_most_calories();
    result.sort();

    let outcome1: usize = *result.iter().rev().take(1).next().unwrap();
    let outcome2: usize = result.iter().rev().take(3).sum();

    println!("The most Calories: {}", outcome1);
    println!("The total is: {}", outcome2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_most_calories() -> Vec<usize> {
    let mut temp_value = 0;
    let mut result: Vec<usize> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if let Ok(value) = l.parse::<usize>() {
                    temp_value += value;
                } else {
                    result.push(temp_value);
                    temp_value = 0;
                }
            }
        }
        result.push(temp_value);
    }
    result
}
