// take referrence from https://github.com/timvisee/advent-of-code-2022/blob/master/day07a/src/main.rs
// and https://github.com/timvisee/advent-of-code-2022/blob/master/day07b/src/main.rs
use std::iter::Peekable;

struct Dir(Vec<Dir>, u64);

fn main() {
    let outputs = include_bytes!("../input.txt");
    let mut sum = 0;

    part1_size(&mut outputs.split(|b| b == &b'\n').peekable(), &mut sum);
    println!("{}", sum);

    let root = part2_dir(&mut outputs.split(|b| b == &b'\n').peekable());
    println!("{}", search(&root, root.1 - 40_000_000).unwrap());
}

fn part1_size(lines: &mut Peekable<impl Iterator<Item = &'static [u8]>>, sum: &mut u64) -> u64 {
    let mut size = 0;
    while let Some(l) = lines.next() {
        match l {
            b"$ cd .." => break,
            _ if l.len() > 0 && &l[0..3] == b"$ l" => {
                size = std::iter::from_fn(|| {
                    lines.next_if(|i| {
                        if i.len() > 0 {
                            i[0] != b'$'
                        } else {
                            return false;
                        }
                    })
                })
                .filter(|i| i[0] != b'd')
                .filter_map(|i| atoi::atoi::<u64>(i.split(|b| b == &b' ').next().unwrap()))
                .sum()
            }
            _ => size += part1_size(lines, sum),
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
}

fn part2_dir(lines: &mut Peekable<impl Iterator<Item = &'static [u8]>>) -> Dir {
    let (mut dirs, mut size) = (vec![], 0);
    while let Some(l) = lines.next() {
        match l {
            b"$ cd .." => break,
            _ if l.len() > 0 && &l[0..3] == b"$ l" => {
                size = std::iter::from_fn(|| {
                    lines.next_if(|i| {
                        if i.len() > 0 {
                            i[0] != b'$'
                        } else {
                            return false;
                        }
                    })
                })
                .filter(|i| i[0] != b'd')
                .filter_map(|i| atoi::atoi::<u64>(i.split(|b| b == &b' ').next().unwrap()))
                .sum()
            }
            _ => dirs.push(part2_dir(lines)),
        }
    }

    size += dirs.iter().map(|d| d.1).sum::<u64>();
    Dir(dirs, size)
}

fn search(d: &Dir, min: u64) -> Option<u64> {
    d.0.iter()
        .filter(|d| d.1 >= min)
        .flat_map(|d| [Some(d.1), search(d, min)])
        .flatten()
        .min()
95437
