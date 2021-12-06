extern crate test;
use std::fs;
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day6/input.txt")
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|v| v.trim().parse()).collect::<Result<_, _>>().unwrap()
}

fn parts(input: &Vec<usize>, days: u32) -> usize {
    let mut counts = [0; 9];
    for fish in input {
        counts[*fish] += 1
    }
    for _ in 0..days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }
    counts.iter().sum()
}


fn part1(input: &Vec<usize>) -> usize {
    parts(input, 80)
}

fn part2(input: &Vec<usize>) -> usize {
    parts(input, 256)
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn example() {
    let input = "3,4,3,1,2";
    let input = parse_input(input);
    assert_eq!(part1(&input), 5934);
    assert_eq!(part2(&input), 26984457539);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 375482);
    assert_eq!(part2(&input), 1689540415957);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
