extern crate test;
use ndarray::Array2;
use std::fs;

#[cfg(test)]
use test::Bencher;

type Segment = [[usize; 2]; 2];

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day5/input.txt")
}

fn parse_input(input: &str) -> Vec<Segment> {
    let mut out = Vec::new();
    for line in input.lines() {
        let nums = line
            .trim()
            .split(" -> ")
            .flat_map(|v| v.split(','))
            .map(|v| v.parse())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        out.push([[nums[0], nums[2]], [nums[1], nums[3]]]);
    }
    out
}

fn step(seg: [usize; 2]) -> i32 {
    if seg[0] < seg[1] {
        1
    } else if seg[0] > seg[1] {
        -1
    } else {
        0
    }
}

fn increment(segment: Segment, board: &mut Array2<u32>) {
    let xs = step(segment[0]);
    let ys = step(segment[1]);

    let mut x = segment[0][0];
    let mut y = segment[1][0];
    loop {
        board[[x, y]] += 1;
        if x == segment[0][1] && y == segment[1][1] {
            break;
        }
        x = (x as i32 + xs).try_into().unwrap();
        y = (y as i32 + ys).try_into().unwrap();
    }
}

fn parts(input: &Vec<Segment>, board_size: usize, consider_diag: bool) -> usize {
    let mut board = Array2::<u32>::zeros((board_size, board_size));
    for segment in input {
        if (segment[0][0] == segment[0][1]) != (segment[1][0] == segment[1][1]) {
            increment(*segment, &mut board);
        } else if consider_diag {
            increment(*segment, &mut board);
        } else {
            continue;
        }
    }
    board.iter().filter(|v| v >= &&2).count()
}

fn part1(input: &Vec<Segment>, board_size: usize) -> usize {
    parts(input, board_size, false)
}

fn part2(input: &Vec<Segment>, board_size: usize) -> usize {
    parts(input, board_size, true)
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input, 1000));
    println!("{:?}", part2(&input, 1000));
    Ok(())
}

#[test]
fn example() {
    let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";
    let input = parse_input(input);
    assert_eq!(part1(&input, 10), 5);
    assert_eq!(part2(&input, 10), 12);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input, 1000), 7142);
    assert_eq!(part2(&input, 1000), 20012);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input, 1000);
        part2(&input, 1000);
    })
}
