extern crate test;
use itertools::{zip, Itertools};
use ndarray::{s, Array2, Dim};
use std::{collections::VecDeque, fs};

#[cfg(test)]
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day9/input.txt")
}

type Floor = Array2<u32>;

fn parse_input(input: &str) -> Floor {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<_> = line
            .trim()
            .chars()
            .map(|v| v.to_digit(10))
            .collect::<Option<_>>()
            .expect("Could not convert a char to digit");
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;
    let floor = Array2::from_shape_vec((n_rows, board_width), data).unwrap();

    floor
}

fn pad(arr: &Floor, value: u32) -> Floor {
    // janky pad implementation
    let mut floor = Array2::zeros(arr.raw_dim() + Dim([2, 2]));
    floor.fill(value);
    floor
        .slice_mut(s![1..floor.shape()[0] - 1, 1..floor.shape()[1] - 1])
        .assign(&arr);
    floor
}

fn local_min_mask(floor: Floor) -> Vec<bool> {
    floor
        .windows([3, 3])
        .into_iter()
        .map(|v| {
            let mid = v[[1, 1]];
            (mid < v[[0, 1]]) & (mid < v[[1, 0]]) & (mid < v[[2, 1]]) & (mid < v[[1, 2]])
        })
        .collect_vec()
}

fn part1(input: &Floor) -> u32 {
    let floor = pad(input, 9);

    let mask_vec = local_min_mask(floor);

    zip(input.iter(), mask_vec.iter())
        .filter(|(_, m)| **m)
        .map(|(v, _)| v + 1)
        .sum()
}

fn part2(input: &Floor) -> u32 {
    let floor = pad(input, 9);

    let mask_vec = local_min_mask(floor);

    let indices: Vec<_> = zip(input.indexed_iter(), mask_vec.iter())
        .filter(|(_, m)| **m)
        .map(|((idx, _), _)| idx)
        .collect();

    let mut basin_id = 0;
    let mut visited = Array2::<usize>::zeros(input.raw_dim());
    for i in indices {
        basin_id += 1;
        let mut to_visit = VecDeque::from([i]);
        while let Some(front) = to_visit.pop_front() {
            const CARDINALS: &[(usize, usize); 4] =
                &[(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)];
            for dir in CARDINALS {
                let next = (front.0.wrapping_add(dir.0), front.1.wrapping_add(dir.1));
                if let Some(0..=8) = input.get(next) {
                    if visited[next] == 0 {
                        to_visit.push_back(next);
                        visited[next] = basin_id;
                    }
                }
            }
        }
    }

    let mut counts = vec![0; basin_id + 1];
    for v in visited.iter() {
        counts[*v] += 1
    }

    counts.sort_unstable();
    counts.iter().rev().skip(1).take(3).product()
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn example() {
    let input = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";
    let input = parse_input(input);
    assert_eq!(part1(&input), 15);
    assert_eq!(part2(&input), 1134);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 535);
    assert_eq!(part2(&input), 1122700);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
