use std::io::repeat;

use itertools::Itertools;
use nom::{IResult, Parser};

use nom::character::streaming::{alpha1, newline};
use nom::multi::separated_list1;
use petgraph::algo::dijkstra;
use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;

fn grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(newline, alpha1.map(|letter: &str| letter.chars().collect()))(input)
}
pub fn process_part1(input: &str) -> String {
    let (_, grid) = grid(input).unwrap();
    // grid = [
    //     [ 'S', 'a', 'b', 'q', 'p', 'o', 'n', 'm', ],
    //     [ 'a', 'b', 'c', 'r', 'y', 'x', 'x', 'l', ],
    //     [ 'a', 'c', 'c', 's', 'z', 'E', 'x', 'k', ],
    //     [ 'a', 'c', 'c', 't', 'u', 'v', 'w', 'j', ],
    //     [ 'a', 'b', 'd', 'e', 'f', 'g', 'h', 'i', ],
    // ]
    //

    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        //flat_map ----------------
        //[src/lib.rs:21] start.next() = Some(
        //    (
        //        (
        //            0,
        //            'S',
        //        ),
        //        0,
        //    ),
        //)
        //[src/lib.rs:22] start.next() = Some(
        //    (
        //        (
        //            1,
        //            'a',
        //        ),
        //        0,
        //    ),
        //)
        //[src/lib.rs:23] start.next() = Some(
        //    (
        //        (
        //            2,
        //            'b',
        //        ),
        //        0,
        //    ),
        //)
        //[src/lib.rs:24] start.next() = Some(
        //    (
        //        (
        //            3,
        //            'q',
        //        ),
        //        0,
        //    ),
        //)
        //[src/lib.rs:25] start.next() = Some(
        //    (
        //        (
        //            4,
        //            'p',
        //        ),
        //        0,
        //    ),
        //)
        //
        //map -------------------------
        //start.next() = Some(
        //    Zip {
        //        a: Enumerate {
        //            iter: Iter(
        //                [
        //                    'S',
        //                    'a',
        //                    'b',
        //                    'q',
        //                    'p',
        //                    'o',
        //                    'n',
        //                    'm',
        //                ],
        //            ),
        //            count: 0,
        //        },
        //        b: Repeat {
        //            element: 0,
        //        },
        //    },
        //)
        .find_map(|((y, &c), x)| {
            if c == 'S' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();
    // dbg!(start.clone());
    let end = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((y, &c), x)| {
            if c == 'E' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    // dbg!(end.clone());

    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    other => *other,
                })
                .collect()
        })
        .collect();

    let edges = (0i32..(grid.len()) as i32)
        .cartesian_product(0i32..(grid[0].len()) as i32)
        // flat_map can be thought as map where the closure returns iterator not value
        .flat_map(|(x, y)| {
            let neighbors = vec![(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)];
            let current_node_id = (x, y);
            //filter_map: The returned iterator yields only the values for which the supplied closure returns Some(value).
            neighbors
                .iter()
                .filter_map(|cell| {
                    grid.get(cell.0 as usize)
                        .and_then(|vec| vec.get(cell.1 as usize))
                        .and_then(|existing_cell| {
                            //if reachable
                            //current node is dynamic, existing_cell is its valid neighbor
                            let current_node_height = grid[x as usize][y as usize];
                            // > means current_node_height == existing_cell
                            // = means current_node_height 1 less than existing_cell
                            if current_node_height as u8 + 1 >= *existing_cell as u8 {
                                Some((
                                    (current_node_id.0, current_node_id.1, current_node_height), // this
                                    // is the start of the edge
                                    (cell.0, cell.1, *existing_cell), //this is the end of the
                                                                      //edge
                                ))
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<((i32, i32, char), (i32, i32, char))>>();
    let graph = DiGraphMap::<(i32, i32, char), ()>::from_edges(&edges);

    // let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let res = dijkstra(
        &graph,
        (start.0, start.1, 'a'),
        Some((end.0, end.1, 'z')),
        |_| 1,
    );
    res[&(end.0, end.1, 'z')].to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, grid) = grid(input).unwrap();
    // grid = [
    //     [ 'S', 'a', 'b', 'q', 'p', 'o', 'n', 'm', ],
    //     [ 'a', 'b', 'c', 'r', 'y', 'x', 'x', 'l', ],
    //     [ 'a', 'c', 'c', 's', 'z', 'E', 'x', 'k', ],
    //     [ 'a', 'c', 'c', 't', 'u', 'v', 'w', 'j', ],
    //     [ 'a', 'b', 'd', 'e', 'f', 'g', 'h', 'i', ],
    // ]
    //

    let mut start = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        //flat_map ----------------
        //[src/lib.rs:21] start.next() = Some(
        //    (
        //        (
        //            0,
        //            'S',
        //        ),
        //        0,
        //    ),
        //)
        //[src/lib.rs:22] start.next() = Some(
        //    (
        //        (
        //            1,
        //            'a',
        //        ),
        //        0,
        //    ),
        //)
        //[src/lib.rs:23] start.next() = Some(
        //    (
        //        (
        //            2,
        //            'b',
        //        ),
        //        0,
        //    ),
        //)
        //[src/lib.rs:24] start.next() = Some(
        //    (
        //        (
        //            3,
        //            'q',
        //        ),
        //        0,
        //    ),
        //)
        //[src/lib.rs:25] start.next() = Some(
        //    (
        //        (
        //            4,
        //            'p',
        //        ),
        //        0,
        //    ),
        //)
        //
        //map -------------------------
        //start.next() = Some(
        //    Zip {
        //        a: Enumerate {
        //            iter: Iter(
        //                [
        //                    'S',
        //                    'a',
        //                    'b',
        //                    'q',
        //                    'p',
        //                    'o',
        //                    'n',
        //                    'm',
        //                ],
        //            ),
        //            count: 0,
        //        },
        //        b: Repeat {
        //            element: 0,
        //        },
        //    },
        //)
        .find_map(|((y, &c), x)| {
            if c == 'S' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();
    // dbg!(start.clone());
    let end = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((y, &c), x)| {
            if c == 'E' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    // dbg!(end.clone());

    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    other => *other,
                })
                .collect()
        })
        .collect();

    let edges = (0i32..(grid.len()) as i32)
        .cartesian_product(0i32..(grid[0].len()) as i32)
        // flat_map can be thought as map where the closure returns iterator not value
        .flat_map(|(x, y)| {
            let neighbors = vec![(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)];
            let current_node_id = (x, y);
            //filter_map: The returned iterator yields only the values for which the supplied closure returns Some(value).
            neighbors
                .iter()
                .filter_map(|cell| {
                    grid.get(cell.0 as usize)
                        .and_then(|vec| vec.get(cell.1 as usize))
                        .and_then(|existing_cell| {
                            //if reachable
                            //current node is dynamic, existing_cell is its valid neighbor
                            let current_node_height = grid[x as usize][y as usize];
                            // > means current_node_height == existing_cell
                            // = means current_node_height 1 less than existing_cell
                            if current_node_height as u8 + 1 >= *existing_cell as u8 {
                                Some((
                                    (current_node_id.0, current_node_id.1, current_node_height), // this
                                    // is the start of the edge
                                    (cell.0, cell.1, *existing_cell), //this is the end of the
                                                                      //edge
                                ))
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<((i32, i32, char), (i32, i32, char))>>();
    let graph = DiGraphMap::<(i32, i32, char), ()>::from_edges(edges.iter().map(|(a, b)| (*b, *a)));

    // let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let res = dijkstra(&graph, (end.0, end.1, 'z'), None, |_| 1);
    // res[&(end.0, end.1, 'z')].to_string()
    let mut result: Vec<i32> = res
        .iter()
        .filter_map(
            |(node, cost)| {
                if node.2 == 'a' {
                    Some(*cost)
                } else {
                    None
                }
            },
        )
        .collect();

    result.sort();
    result.iter().next().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const INPUT: &str = include_str!("../test.txt");
    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 31.to_string());
    }

    #[test]

    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 29.to_string());
    }
}
