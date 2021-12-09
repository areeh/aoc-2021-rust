extern crate test;
use itertools::zip;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[cfg(test)]
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day8/input.txt")
}

type Disp = ([String; 10], [String; 4]);

fn parse_input(input: &str) -> Vec<Disp> {
    input
        .lines()
        .map(|v| {
            let (unique_patterns, output) =
                v.trim().split_once('|').expect("Could not split on '|'");
            (
                unique_patterns
                    .split_whitespace()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Expected output to fit into size 10 array"),
                output
                    .split_whitespace()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Expected output to fit into size 4 array"),
            )
        })
        .collect_vec()
}

fn classify_unique(pattern: &str) -> Result<char, &str> {
    match pattern.len() {
        2 => Ok('1'),
        3 => Ok('7'),
        4 => Ok('4'),
        7 => Ok('8'),
        _ => Err(pattern),
    }
}

fn part1(input: &Vec<Disp>) -> usize {
    input
        .iter()
        .flat_map(|(_, output)| output)
        .map(|v| classify_unique(v))
        .filter(|v| v.is_ok())
        .count()
}

fn to_bitmask(signal: &str) -> u8 {
    //  "abcdefg"  -> 1111111
    // "abde" -> 0011011
    signal
        .bytes()
        .map(|v| 1 << v - b'a')
        .fold(0, |acc, v| acc + v)
}

fn nth(v: &u8, bits: usize, n: usize) -> u8 {
    (v & (1 << bits - 1) >> n) >> (bits - 1 - n)
}

fn sum_nth_bit(bits_slice: &[u8], n: usize, bits: usize) -> usize {
    bits_slice
        .iter()
        .fold(0, |acc, v| acc + nth(v, bits, n) as usize)
}

fn bit_n(n: usize) -> u8 {
    1 << (7 - n)
}

fn neg_7(n: u8) -> u8 {
    !n & u8::MAX >> 1
}

fn bit_union<'a>(vals: impl Iterator<Item = &'a u8>) -> u8 {
    vals.fold(0 as u8, |acc, v| acc | v)
}

fn build_classify_map(segment_map: &HashMap<char, u8>) -> HashMap<u8, char> {
    let mut classify_map = HashMap::new();
    for (seg, n) in zip(
        [
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        ],
        "0123456789".chars(),
    ) {
        classify_map.insert(
            bit_union(seg.chars().map(|v| segment_map.get(&v).unwrap())),
            n,
        );
    }
    classify_map
}

fn part2(input: &Vec<Disp>) -> i32 {
    let mut sum = 0;

    for (signal, output) in input {
        let bits: Vec<_> = signal.iter().map(|v| to_bitmask(v)).collect();
        let output_bits: Vec<_> = output.iter().map(|v| to_bitmask(v)).collect();

        let mut number_map = HashMap::new();
        let mut segment_map = HashMap::new();

        for b in &bits {
            match b.count_ones() {
                2 => number_map.insert(1, *b),
                3 => number_map.insert(7, *b),
                4 => number_map.insert(4, *b),
                7 => number_map.insert(8, *b),
                _ => None,
            };
        }

        segment_map.insert(
            'a',
            number_map.get(&7).unwrap() ^ number_map.get(&1).unwrap(),
        );

        for pos in 1..8 {
            match sum_nth_bit(&bits, pos, 8) {
                4 => segment_map.insert('e', bit_n(pos)),
                6 => segment_map.insert('b', bit_n(pos)),
                9 => segment_map.insert('f', bit_n(pos)),
                _ => None,
            };
        }

        segment_map.insert(
            'd',
            number_map.get(&4).unwrap()
                ^ number_map.get(&1).unwrap()
                ^ segment_map.get(&'b').unwrap(),
        );
        segment_map.insert(
            'c',
            segment_map.get(&'f').unwrap() ^ number_map.get(&1).unwrap(),
        );
        segment_map.insert('g', neg_7(bit_union(segment_map.values())));

        let classify_map = build_classify_map(&segment_map);

        sum += output_bits
            .iter()
            .map(|v| *classify_map.get(&v).unwrap())
            .collect::<String>()
            .parse::<i32>()
            .expect("could not parse result as number");
    }
    sum
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn example() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc 
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    let input = parse_input(input);
    assert_eq!(part1(&input), 26);
    assert_eq!(part2(&input), 61229);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 449);
    assert_eq!(part2(&input), 968175);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
