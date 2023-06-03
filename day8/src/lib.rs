pub fn process_part1(input: &str) -> String {
    let tree_map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let outermost = tree_map.len() * 2 + tree_map[0].len() * 2 - 4;

    let mut inner_count = 0;
    for x in 1..tree_map.len() - 1 {
        for y in 1..tree_map[0].len() - 1 {
            let current_tree = tree_map[x][y];

            let current_up = tree_map[0..x].iter().all(|x| x[y] < current_tree);

            let current_down = tree_map[x + 1..tree_map.len()]
                .iter()
                .all(|x| x[y] < current_tree);

            let current_left = tree_map[x][0..y].iter().all(|n| n < &current_tree);

            let current_right = tree_map[x][y + 1..tree_map[0].len()]
                .iter()
                .all(|n| n < &current_tree);

            if current_left || current_right || current_up || current_down {
                inner_count += 1;
            }
        }
    }

    (outermost + inner_count).to_string()
}

pub fn process_part2(input: &str) -> String {
    let tree_map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut result: Vec<usize> = vec![];

    for x in 1..tree_map.len() - 1 {
        for y in 1..tree_map[0].len() - 1 {
            let current_tree = tree_map[x][y];

            // Up
            let mut current_up_vec: Vec<u32> = vec![];
            tree_map[0..x]
                .iter()
                .for_each(|x| current_up_vec.push(x[y]));

            let mut up_score = 0;
            if current_up_vec.iter().all(|&n| n < current_tree) {
                up_score = current_up_vec.len();
            } else {
                up_score = current_up_vec
                    .iter()
                    .rev()
                    .position(|&x| x >= current_tree)
                    .unwrap()
                    + 1;
            }

            // Down
            let mut current_down_vec: Vec<u32> = vec![];
            tree_map[x + 1..tree_map.len()]
                .iter()
                .for_each(|x| current_down_vec.push(x[y]));
            let mut bottom_score = 0;
            if current_down_vec.iter().all(|&n| n < current_tree) {
                bottom_score = current_down_vec.len();
            } else {
                bottom_score = current_down_vec
                    .iter()
                    .position(|&x| x >= current_tree)
                    .unwrap()
                    + 1;
            }

            // Left
            let mut left_score = 0;
            if tree_map[x][0..y].iter().all(|&n| n < current_tree) {
                left_score = tree_map[x][0..y].len();
            } else {
                left_score = tree_map[x][0..y]
                    .iter()
                    .rev()
                    .position(|&x| x >= current_tree)
                    .unwrap()
                    + 1;
            }

            //Right
            let mut right_score = 0;
            if tree_map[x][y + 1..tree_map[0].len()]
                .iter()
                .all(|&n| n < current_tree)
            {
                right_score = tree_map[x][y + 1..tree_map[0].len()].len();
            } else {
                right_score = tree_map[x][y + 1..tree_map[0].len()]
                    .iter()
                    .position(|&x| x >= current_tree)
                    .unwrap()
                    + 1;
            }

            result.push(up_score * bottom_score * left_score * right_score);
        }
    }

    result.iter().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 21.to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 8.to_string());
    }
}
