extern crate test;
use itertools::Itertools;
use ndarray::{s, Array2, Zip};
use std::fs;

#[cfg(test)]
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day13/input.txt")
}

type Paper = Array2<bool>;
type Instructions = Vec<(char, usize)>;

fn parse_input(input: &str) -> (Paper, Instructions) {
    let (paper_str, instructions_str) = input
        .trim()
        .split_once("\n\n")
        .expect("Did not find a double newline");
    let instructions = instructions_str
        .lines()
        .map(|v| {
            let (ax, val) = v.split_once('=').unwrap();
            (ax.chars().last().unwrap(), val.parse().unwrap())
        })
        .collect_vec();

    let paper_idxs = paper_str
        .lines()
        .map(|v| {
            let (x, y) = v.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect_vec();

    let x_max: usize = *paper_idxs.iter().map(|(x, _)| x).max().unwrap();
    let y_max: usize = *paper_idxs.iter().map(|(_, y)| y).max().unwrap();

    let mut paper = Array2::from_elem((x_max + 1, y_max + 1), false);

    for (x, y) in paper_idxs {
        paper[[x, y]] = true;
    }

    (paper, instructions)
}

fn fold(paper: &mut Paper, axis: char, pos: usize) -> Paper {
    let fold_len = match axis {
        'x' => paper.dim().0 - pos - 1,
        'y' => paper.dim().1 - pos - 1,
        _ => panic!("unknown axis"),
    };

    let (left_slice, right_slice, right_inner) = match axis {
        'x' => (
            s![0..pos, ..],
            s![(pos+1)..;-1, ..],
            s![-(fold_len as isize).., ..],
        ),
        'y' => (
            s![.., 0..pos],
            s![.., (pos+1)..;-1],
            s![.., -(fold_len as isize)..],
        ),
        _ => panic!("unknown axis"),
    };

    let left = paper.slice(left_slice);
    let mut right = Array2::from_elem(left.raw_dim(), false);
    right
        .slice_mut(right_inner)
        .assign(&paper.slice(right_slice));

    &left | &right
}

fn part1(input: &(Paper, Instructions)) -> usize {
    let (mut paper, instructions) = {
        let (paper, instructions) = input;
        (paper.clone(), instructions)
    };

    for (ax, pos) in instructions.iter().take(1) {
        paper = fold(&mut paper, *ax, *pos);
    }

    paper.iter().filter(|v| **v).count()
}

fn part2(input: &(Paper, Instructions)) -> String {
    let (mut paper, instructions) = {
        let (paper, instructions) = input;
        (paper.clone(), instructions)
    };

    for (ax, pos) in instructions.iter() {
        paper = fold(&mut paper, *ax, *pos);
    }

    let mut sesame = Array2::from_elem(paper.raw_dim(), '.');

    Zip::from(&mut sesame)
        .and(&paper)
        .for_each(|sesame, paper| {
            if *paper {
                *sesame = '#'
            }
        });

    let mut disp = String::with_capacity(sesame.len());
    for col in sesame.columns() {
        disp.push_str(col.iter().cloned().collect::<String>().as_str());
        disp.push_str("\n");
    }

    disp
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}

#[test]
fn example() {
    let input = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let input = parse_input(input);
    assert_eq!(part1(&input), 17);
    part2(&input);
    assert_eq!(
        part2(&input).trim(),
        "
#####
#...#
#...#
#...#
#####
.....
.....
"
        .trim()
    );
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 655);
    assert_eq!(
        part2(&input).trim(),
        "
..##.###..####..##..#..#..##..#..#.###..
...#.#..#....#.#..#.#..#.#..#.#..#.#..#.
...#.#..#...#..#....#..#.#..#.#..#.#..#.
...#.###...#...#....#..#.####.#..#.###..
#..#.#....#....#..#.#..#.#..#.#..#.#.#..
.##..#....####..##...##..#..#..##..#..#.
"
        .trim()
    );
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
