extern crate test;
use ndarray::Array2;
use std::fs;

#[cfg(test)]
use test::Bencher;

type Floor = Array2<u32>;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day11/input.txt")
}

fn parse_input(input: &str) -> Floor {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<_> = line
            .trim()
            .chars()
            .map(|v| v.to_digit(10))
            .collect::<Option<_>>()
            .expect("could not convert char to digit");
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;
    let floor = Array2::from_shape_vec((n_rows, board_width), data).unwrap();

    floor
}

fn increment_neighbors(floor: &mut Floor, count: &mut usize) {
    let floor_clone = floor.clone();
    let idxs = floor_clone
        .indexed_iter()
        .filter(|(_, v)| **v == 9)
        .map(|(idx, _)| idx);

    const ADJACENTS: &[(usize, usize); 8] = &[
        (1, 0),
        (usize::MAX, 0),
        (0, 1),
        (0, usize::MAX),
        (1, 1),
        (usize::MAX, usize::MAX),
        (1, usize::MAX),
        (usize::MAX, 1),
    ];
    for i in idxs {
        for dir in ADJACENTS {
            let adj_idx = (i.0.wrapping_add(dir.0), i.1.wrapping_add(dir.1));
            if let Some(v) = floor.get_mut(adj_idx) {
                if *v < 9 {
                    *v += 1;
                }
            }
        }
        floor[i] = 10;
        *count += 1;
    }
}

fn _part1(input: &Floor, steps: usize, find_all: bool) -> usize {
    let mut oct = input.clone();

    let mut count = 0;
    let mut prev_count = 0;

    for i in 0..steps {
        increment_neighbors(&mut oct, &mut count);
        while prev_count < count {
            prev_count = count.clone();
            increment_neighbors(&mut oct, &mut count);
        }
        oct.mapv_inplace(|v| if v == 10 { 0 } else { v + 1 });
        if find_all & oct.iter().all(|v| *v == 0) {
            return i + 1;
        }
    }

    count
}

fn part1(input: &Floor) -> usize {
    _part1(input, 100, false)
}

fn part2(input: &Floor) -> usize {
    _part1(input, 999, true)
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn example_mini() {
    let input = "11111
    19991
    19191
    19991
    11111";
    let input = parse_input(input);
    assert_eq!(_part1(&input, 2, false), 9);
}

#[test]
fn example() {
    let input = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";
    let input = parse_input(input);
    assert_eq!(part1(&input), 1656);
    assert_eq!(part2(&input), 195);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 1615);
    assert_eq!(part2(&input), 249);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
