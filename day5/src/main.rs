use itertools::Itertools;
fn main() {
    let content = include_bytes!("../input.txt");
    let (up, bottom) = content.split_at(content.windows(2).position(|b| b == b"\n\n").unwrap() + 2);
    part1(up, bottom);
    part2(up, bottom);
}

fn part1(up: &[u8], bottom: &[u8]) {
    let mut result: [Vec<u8>; 9] = Default::default();
    // println!("{}---", String::from_utf8(up.to_vec()).unwrap());
    // println!("{}", String::from_utf8(bottom.to_vec()).unwrap());
    up.split(|b| b == &b'\n').rev().skip(1).for_each(|l| {
        l.iter()
            .skip(1) // remove the first char '[' in each line
            .step_by(4) // step into 4 each time
            .enumerate()
            .filter(|(_, b)| b != &&b' ')
            .for_each(|(i, v)| result[i].push(*v));
    });

    bottom.split(|b| b == &b'\n').for_each(|m| {
        if m.len() != 0 {
            let (n, a, b): (usize, _, _) = m
                .split(|b| b == &b' ')
                .skip(1) // skip 'move'
                .step_by(2) // only pick number
                .map(|n| atoi::atoi(n).unwrap())
                .collect_tuple()
                .unwrap();

            for _ in 0..n {
                let temp = result[a - 1].pop().unwrap();
                result[b - 1].push(temp);
            }
        }
    });

    result.iter().for_each(|s| {
        if s.len() > 0 {
            print!("{}", *s.last().unwrap() as char);
        }
    });
    println!();
}

fn part2(up: &[u8], bottom: &[u8]) {
    let mut result: [Vec<u8>; 9] = Default::default();
    // println!("{}---", String::from_utf8(up.to_vec()).unwrap());
    // println!("{}", String::from_utf8(bottom.to_vec()).unwrap());
    up.split(|b| b == &b'\n').rev().skip(1).for_each(|l| {
        l.iter()
            .skip(1) // remove the first char '[' in each line
            .step_by(4) // step into 4 each time
            .enumerate()
            .filter(|(_, b)| b != &&b' ')
            .for_each(|(i, v)| result[i].push(*v));
    });

    bottom.split(|b| b == &b'\n').for_each(|m| {
        if m.len() != 0 {
            let (n, a, b): (usize, _, _) = m
                .split(|b| b == &b' ')
                .skip(1) // skip 'move'
                .step_by(2) // only pick number
                .map(|n| atoi::atoi(n).unwrap())
                .collect_tuple()
                .unwrap();

            let mut temp_vec: Vec<u8> = Vec::new();
            for _ in 0..n {
                let temp = result[a - 1].pop().unwrap();
                temp_vec.push(temp);
            }

            for _ in 0..temp_vec.len() {
                let t = temp_vec.pop().unwrap();
                result[b - 1].push(t);
            }
        }
    });

    result.iter().for_each(|s| {
        if s.len() > 0 {
            print!("{}", *s.last().unwrap() as char);
        }
    });
    println!();
}
