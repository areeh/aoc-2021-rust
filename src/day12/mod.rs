extern crate test;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

#[cfg(test)]
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day12/input.txt")
}

fn is_all_upper(s: &str) -> bool {
    s.chars().all(char::is_uppercase)
}

type CaveMap<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input(input: &str) -> CaveMap {
    let mut map = HashMap::new();
    for (left, right) in input.lines().map(|v| v.trim().split_once('-').unwrap()) {
        map.entry(left).or_insert(Vec::new()).push(right);
        map.entry(right).or_insert(Vec::new()).push(left);
    }
    map
}

fn parts(input: &CaveMap<'_>, skip_dupe: bool) -> usize {
    let mut q = VecDeque::from([(vec!["end"], false)]);
    let mut count = 0;

    while let Some((current, has_duplicate)) = q.pop_front() {
        for &neighbor in &input[current.last().unwrap()] {
            match neighbor {
                "end" => continue,
                "start" => count += 1,
                neighbor => {
                    let mut has_duplicate = has_duplicate;
                    if !is_all_upper(neighbor) && current.contains(&neighbor) {
                        if skip_dupe || has_duplicate {
                            continue;
                        }
                        has_duplicate = true;
                    }
                    let mut next = current.clone();
                    next.push(neighbor);
                    q.push_back((next, has_duplicate));
                }
            }
        }
    }
    count
}

fn part1(input: &CaveMap) -> usize {
    parts(input, true)
}

fn part2(input: &CaveMap) -> usize {
    parts(input, false)
}

pub fn main() -> std::io::Result<()> {
    let input_str = input1()?;
    let input = parse_input(&input_str);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn example1() {
    let input = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";
    let input = parse_input(input);
    assert_eq!(part1(&input), 10);
    assert_eq!(part2(&input), 36);
}

#[test]
fn example2() {
    let input = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc";
    let input = parse_input(input);
    assert_eq!(part1(&input), 19);
    assert_eq!(part2(&input), 103);
}

#[test]
fn example3() {
    let input = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";
    let input = parse_input(input);
    assert_eq!(part1(&input), 226);
    assert_eq!(part2(&input), 3509);
}

#[test]
fn task() {
    let input_str = input1().unwrap();
    let input = parse_input(&input_str);
    assert_eq!(part1(&input), 4970);
    assert_eq!(part2(&input), 137948);
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
