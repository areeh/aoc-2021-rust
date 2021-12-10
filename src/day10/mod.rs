extern crate test;
use itertools::Itertools;
use std::{collections::HashSet, fs};

#[cfg(test)]
use test::Bencher;

type Code = Vec<Vec<char>>;

const LEFT_BRACKETS: [char; 4] = ['{', '[', '(', '<'];

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day10/input.txt")
}

fn parse_input(input: &str) -> Code {
    input
        .lines()
        .map(|v| v.trim().chars().collect_vec())
        .collect_vec()
}

fn score_illegal(c: &char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unknown closing bracket"),
    }
}

fn get_matching(c: &char) -> char {
    match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => panic!("unknown opening bracket"),
    }
}

fn part1(input: &Code) -> usize {
    let left_brackets = HashSet::from(LEFT_BRACKETS);
    let mut stack = Vec::new();
    let mut illegals = Vec::new();

    for line in input {
        stack.clear();
        for c in line {
            if left_brackets.contains(c) {
                stack.push(c)
            } else {
                let left = stack.pop().unwrap();
                if &get_matching(left) != c {
                    illegals.push(c);
                    break;
                }
            }
        }
    }
    illegals.into_iter().map(score_illegal).sum()
}

fn score_remaining(c: &char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!(),
    }
}

fn part2(input: &Code) -> usize {
    let left_brackets = HashSet::from(LEFT_BRACKETS);
    let mut stack = Vec::new();
    let mut scores = Vec::new();

    for line in input {
        let mut do_score = true;
        stack.clear();
        for c in line {
            if left_brackets.contains(c) {
                stack.push(c)
            } else {
                let left = stack.pop().unwrap();
                if &get_matching(left) != c {
                    do_score = false;
                    break;
                }
            }
        }
        if do_score {
            scores.push(
                stack
                    .iter()
                    .rev()
                    .fold(0, |acc, v| acc * 5 + score_remaining(*v)),
            );
        }
    }

    scores.sort();
    scores[(scores.len() / 2)]
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn example() {
    let input = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";
    let input = parse_input(input);
    assert_eq!(part1(&input), 26397);
    assert_eq!(part2(&input), 288957);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 436497);
    assert_eq!(part2(&input), 2377613374);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
