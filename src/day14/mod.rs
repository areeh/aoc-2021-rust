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
            .split_once(" -> ")
            .expect("Did not find arrow to split on");
        acc.entry(pair.chars().tuple_windows::<(_, _)>().nth(0).unwrap())
            .or_insert(insert.chars().nth(0).unwrap());
        acc
    });

    (template, rules)
}

fn rule_step(template: String, rules: HashMap<(char, char), char>) -> String {
    template
        .chars()
        .tuple_windows::<(_, _)>()
        .flat_map(|v| {
            if let Some(new) = rules.get(&v) {
                [v.0, *new]
            } else {
                [v.0]
            }
        })
        .collect()
}

fn part1(input: (&str, HashMap<(char, char), char>)) -> usize {
    let mut template = input.0.to_string();
    let rules = &input.1;

    for _ in 0..10 {
        template = rule_step(template, rules);
    }

    let counts = template.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    counts.values().max().unwrap() * counts.values().min().unwrap()
}

// fn part2(input: &Vec<Disp>) -> i32 {
// }

pub fn main() -> std::io::Result<()> {
    let input_str = &input1()?;
    let input = parse_input(input_str);
    println!("{:?}", part1(input));
    // println!("{:?}", part2(&input));
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
    assert_eq!(part1(input), 1588);
    // assert_eq!(part2(&input), 61229);
}

#[test]
fn task() {
    // let input = parse_input(&input1().unwrap());
    // assert_eq!(part1(&input), 449);
    // assert_eq!(part2(&input), 968175);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        // let input = parse_input(&input1().unwrap());
        // part1(&input);
        // part2(&input);
    })
}
