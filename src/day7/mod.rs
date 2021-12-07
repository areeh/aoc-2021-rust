extern crate test;
use std::fs;
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day7/input.txt")
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|v| v.trim().parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

fn cost_p1(l: i64, r: i64) -> i64 {
    (l - r).abs()
}

fn triangle(n: i64) -> i64 {
    (n * n + n) / 2
}

fn cost_p2(l: i64, r: i64) -> i64 {
    triangle((l - r).abs())
}

fn part1(input: &Vec<i64>) -> i64 {
    let dest = statistical::median(&input);
    input.iter().map(|v| cost_p1(*v, dest)).sum()
}

fn part2(input: &Vec<i64>) -> i64 {
    let mn = input.iter().min().unwrap();
    let mx = input.iter().max().unwrap();

    let mut best = cost_p2(*mn, *mx) * input.len() as i64;

    for a in *mn..*mx+1 {
        let cost = input.iter().map(|v| cost_p2(*v, a)).sum();
        if cost < best {
            best = cost;
        }      
    }
    best
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn example() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    let input = parse_input(input);
    assert_eq!(part1(&input), 37);
    assert_eq!(part2(&input), 168);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 337833);
    assert_eq!(part2(&input), 96678050);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
