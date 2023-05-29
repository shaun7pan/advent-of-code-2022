fn main() {
    let mut alphabet: Vec<char> = Vec::new();

    for c in 'a'..='z' {
        alphabet.push(c)
    }
    for c in 'A'..='Z' {
        alphabet.push(c)
    }

    let mut part1_result: Vec<usize> = Vec::new();
    let mut part2_converted_iter: Vec<String> = Vec::new();

    if let Ok(lines) = utils::read_lines("./input.txt") {
        for line in lines {
            if let Ok(x) = line {
                part1(&x, &mut part1_result, &alphabet);
                part2_converted_iter.push(x.to_string());
            }
        }
    }

    let part2_result = part2(part2_converted_iter, &alphabet);

    println!(
        "The sum of priorities: {}",
        part1_result.iter().sum::<usize>()
    );

    println!(
        "The sum of priorities part2: {}",
        part2_result.iter().sum::<usize>()
    );
}

fn part1(x: &str, result: &mut Vec<usize>, alphabet: &Vec<char>) {
    let (left, right) = x.split_at(x.len() / 2);
    let a = left.chars().find(|x| right.contains(*x));
    if let Some(c) = a {
        for (i, v) in alphabet.iter().enumerate() {
            if c == *v {
                result.push(i + 1);
            }
        }
    }
}

fn part2(src_iter: Vec<String>, alphabet: &Vec<char>) -> Vec<usize> {
    let temp: Vec<&[String]> = src_iter.chunks(3).collect();
    let mut result: Vec<usize> = Vec::new();
    for v in temp {
        for c1 in v[0].chars() {
            if v[1].contains(c1) && v[2].contains(c1) {
                for (i, v) in alphabet.iter().enumerate() {
                    if c1 == *v {
                        result.push(i + 1);
                    }
                }
                break;
            }
        }
    }
    result
}
