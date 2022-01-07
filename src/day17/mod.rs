extern crate test;
use std::fs;

#[cfg(test)]
use test::Bencher;

type Target = [[i32; 2]; 2];

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day17/input.txt")
}

fn parse_input(input: &str) -> Target {
    let (x, y) = input
        .trim()
        .strip_prefix("target area: ")
        .unwrap()
        .split_once(", ")
        .unwrap();
    let (lx, rx) = x.strip_prefix("x=").unwrap().split_once("..").unwrap();
    let (ly, ry) = y.strip_prefix("y=").unwrap().split_once("..").unwrap();
    [
        [lx.parse().unwrap(), rx.parse().unwrap()],
        [ly.parse().unwrap(), ry.parse().unwrap()],
    ]
}

fn triangle(n: usize) -> usize {
    (n * n + n) / 2
}

fn on_target(pos: [i32; 2], target: &Target) -> bool {
    pos[0] >= target[0][0]
        && pos[0] <= target[0][1]
        && pos[1] >= target[1][0]
        && pos[1] <= target[1][1]
}

fn run_probe(initial: [i32; 2], target: &Target) -> Option<usize> {
    let mut pos = [0; 2];
    let mut v = initial;
    let mut steps = 0;

    while pos[1] > target[1][0] {
        steps += 1;

        pos[0] += v[0];
        pos[1] += v[1];

        v[0] -= v[0].signum();
        v[1] -= 1;
        if on_target(pos, target) {
            return Some(steps);
        }
    }
    None
}

fn part1(input: &Target) -> usize {
    triangle((input[1][0].abs() - 1) as usize)
}

fn part2(input: &Target) -> usize {
    let mut count = 0;
    for x in 1..=input[0][1] {
        for y in input[1][0]..=-input[1][0] {
            if let Some(_) = run_probe([x, y], input) {
                count += 1;
            }
        }
    }
    count
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn example() {
    let input = "target area: x=20..30, y=-10..-5";
    let input = parse_input(input);
    assert_eq!(part1(&input), 45);
    assert_eq!(part2(&input), 112);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 5671);
    assert_eq!(part2(&input), 4556);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
