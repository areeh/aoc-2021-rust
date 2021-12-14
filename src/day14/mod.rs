extern crate test;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[cfg(test)]
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day14/input.txt")
}

fn parse_input(input: &str) -> (&str, HashMap<(char, char), char>) {
    let (template, rule_str) = input
        .split_once("\n\n")
        .expect("Did not find double newline");

    let rules = rule_str.lines().fold(HashMap::new(), |mut acc, v| {
        let (pair, insert) = v
            .trim()
            .split_once(" -> ")
            .expect("Did not find arrow to split on");
        acc.entry(pair.chars().tuple_windows::<(_, _)>().nth(0).unwrap())
            .or_insert(insert.chars().nth(0).unwrap());
        acc
    });

    (template, rules)
}

fn count_pairs(input: &str) -> HashMap<(char, char), usize> {
    input
        .chars()
        .tuple_windows::<(_, _)>()
        .fold(HashMap::new(), |mut acc, pair| {
            *acc.entry(pair).or_insert(0) += 1;
            acc
        })
}

fn rule_step(counts: &mut HashMap<(char, char), usize>, rules: &HashMap<(char, char), char>) {
    let current_counts = counts.clone();
    for ((left, right), insert) in rules.iter() {
        if let Some(count) = current_counts.get(&(*left, *right)) {
            if *count > 0 {
                *counts.get_mut(&(*left, *right)).unwrap() -= count;
                *counts.entry((*left, *insert)).or_insert(0) += count;
                *counts.entry((*insert, *right)).or_insert(0) += count;
            }
        }
    }
}

fn parts(input: &(&str, HashMap<(char, char), char>), steps: usize) -> usize {
    let template = input.0.to_string();
    let rules = &input.1;
    let mut pair_counts = count_pairs(template.as_str());

    for _ in 0..steps {
        rule_step(&mut pair_counts, rules);
    }
    let mut char_counts = pair_counts
        .iter()
        .fold(HashMap::new(), |mut acc, ((l, _), count)| {
            *acc.entry(l).or_insert(0) += count;
            acc
        });

    // Account for last character being on the right of a pair
    *char_counts
        .get_mut(&template.chars().last().unwrap())
        .unwrap() += 1;

    char_counts.values().max().unwrap() - char_counts.values().filter(|v| **v > 0).min().unwrap()
}

fn part1(input: &(&str, HashMap<(char, char), char>)) -> usize {
    parts(input, 10)
}
fn part2(input: &(&str, HashMap<(char, char), char>)) -> usize {
    parts(input, 40)
}

pub fn main() -> std::io::Result<()> {
    let input_str = &input1()?;
    let input = parse_input(input_str);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn example() {
    let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";
    let input = parse_input(input);
    assert_eq!(part1(&input), 1588);
    assert_eq!(part2(&input), 2188189693529);
}

#[test]
fn task() {
    let input_str = input1().unwrap();
    let input = parse_input(&input_str);
    assert_eq!(part1(&input), 2027);
    assert_eq!(part2(&input), 2265039461737);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input_str = input1().unwrap();
        let input = parse_input(&input_str);
        part1(&input);
        part2(&input);
    })
}
