extern crate test;
use std::cmp::min;
use std::fs;
use itertools::Itertools;
use std::ops::Range;

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

fn min_x_velocity(max_x: usize) -> usize {
    for n in 1..max_x {
        if triangle(n) > max_x {
            return n - 2;
        }
    }
    !unreachable!()
}

fn possible_steps(initial: i32, target: [i32; 2]) -> Option<(usize, usize)> {
    let mut v = initial;
    let mut pos = 0;
    let mut i = 0;
    let mut first = None;

    loop {
        i += 1;
        pos += v;
        v -= v.signum();
        if v == 0 {
            if first.is_some() {
                return Some((first.unwrap(), usize::MAX));
            } else {
                return None;
            }
        } else if pos > target[1] {
            if first.is_some() {
                return Some((first.unwrap(), i));
            } else {
                return None;
            }
        } else if (target[0]..target[1]+1).contains(&pos) {
            if first.is_none() {
                first = Some(i);
            }
        }
    }
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

fn part2_not_brute(input: &Target) -> usize {
    let mut count = 0;
    let minx = min_x_velocity(input[0][1] as usize);
    for x in 1..=input[0][1] {
        if x < minx as i32 {
            continue;
        }
        if let Some((min_step, max_step)) = possible_steps(x, input[0]) {
            let max_y = min((input[1][0].abs() - 1) as usize, max_step);
            for y in input[1][0]..=-input[1][0] {
                if y <= max_y as i32 {
                    if let Some(_) = run_probe([x, y], input) {
                        count += 1;
                    }
                }
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
    // assert_eq!(part2_not_brute(&input), 112);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 5671);
    assert_eq!(part2(&input), 4556);
    // assert_eq!(part2_not_brute(&input), 4556);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
