fn main() {
    let mut part1_answer = 0;
    let mut part2_answer = 0;
    if let Ok(lines) = utils::read_lines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let mut line_iter = l.split(',');
                let left = line_iter.next().unwrap();
                let right = line_iter.next().unwrap();
                let mut left_iter = left.split('-');
                let mut right_iter = right.split('-');
                let left_start = left_iter.next().unwrap().parse::<usize>().unwrap();
                let left_end = left_iter.next().unwrap().parse::<usize>().unwrap();
                let right_start = right_iter.next().unwrap().parse::<usize>().unwrap();
                let right_end = right_iter.next().unwrap().parse::<usize>().unwrap();

                if (left_start..=left_end).all(|x| (right_start..=right_end).contains(&x))
                    || (right_start..=right_end).all(|x| (left_start..=left_end).contains(&x))
                {
                    part1_answer += 1;
                }

                if (left_start..=left_end).any(|x| (right_start..=right_end).contains(&x))
                    || (right_start..=right_end).any(|x| (left_start..=left_end).contains(&x))
                {
                    part2_answer += 1;
                }
            }
        }
    }

    println!("The answer of part1 is: {}", part1_answer);
    println!("The answer of part2 is: {}", part2_answer);
}
