extern crate test;
use itertools::Itertools;
use ndarray::Array2;
use std::fs;

#[cfg(test)]
use test::Bencher;

type Floor = Array2<u32>;

const DOWN: (usize, usize) = (0, 1);
const RIGHT: (usize, usize) = (1, 0);

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day25/input.txt")
}

fn parse_input(input: &str) -> Floor {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<_> = line
            .trim()
            .chars()
            .map(|c| match c {
                '.' => 0,
                '>' => 1,
                'v' => 2,
                c => panic!("Unknown char {:?}", c),
            })
            .collect_vec();
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;
    let floor = Array2::from_shape_vec((n_rows, board_width), data)
        .unwrap()
        .reversed_axes();

    floor
}

fn check_move(dir: u32, map: &Floor) -> Floor {
    let x_max = map.shape()[0];
    let y_max = map.shape()[1];

    let (dir_x, dir_y) = match dir {
        1 => RIGHT,
        2 => DOWN,
        _ => panic!(),
    };

    let mut new_map = map.clone();
    for ((x, y), v) in map.indexed_iter().filter(|(_, v)| **v == dir) {
        let next = (x.wrapping_add(dir_x) % x_max, y.wrapping_add(dir_y) % y_max);
        if map[next] == 0 {
            new_map[next] = *v;
            new_map[(x, y)] = 0;
        }
    }
    new_map
}

fn step(map: &Floor) -> Floor {
    let map = check_move(1, map);
    let map = check_move(2, &map);
    map
}

fn part1(input: &Floor) -> usize {
    let mut n = 0;

    let mut prev = input.clone();
    let mut curr;

    loop {
        n += 1;
        curr = step(&prev);
        if curr == prev {
            return n;
        }
        prev = curr;
    }
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    Ok(())
}

#[test]
fn example_step() {
    let input = "...>...
    .......
    ......>
    v.....>
    ......>
    .......
    ..vvv..";
    let input = parse_input(input);
    let expected = parse_input(
        ">......
    ..v....
    ..>.v..
    .>.v...
    ...>...
    .......
    v......",
    );

    let mut output = input;
    for _ in 0..4 {
        output = step(&output);
    }

    assert_eq!(output, expected);
}

#[test]
fn example() {
    let input = "v...>>.vv>
    .vv>>.vv..
    >>.>v>...v
    >>v>>.>.v.
    v>v.vv.v..
    >.>>..v...
    .vv..>.>v.
    v.v..>>v.v
    ....v..v.>
    ";
    let input = parse_input(input);
    assert_eq!(part1(&input), 58);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 386);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
    })
}
