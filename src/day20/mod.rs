extern crate test;
use itertools::Itertools;
use ndarray::{s, Array2, Dim, Zip};
use std::fs;

#[cfg(test)]
use test::Bencher;

type Image = Array2<u32>;
type Algo = [u32; 512];

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day20/input.txt")
}

use std::convert::TryInto;

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn parse_image(input: &str) -> Image {
    let image_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<_> = line
            .trim()
            .chars()
            .map(|v| match v {
                '#' => 1,
                '.' => 0,
                v => panic!("unknown character {:?}", v),
            })
            .collect();
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / image_width;
    let image = Array2::from_shape_vec((image_width, n_rows), data).unwrap();

    image
}

fn parse_input(input: &str) -> (Algo, Image) {
    let (algo, image) = input.trim().split_once("\n\n").expect(&format!(
        "Did not find a double newline in {:?}",
        input.trim()
    ));
    let algo = vec_to_array(
        algo.chars()
            .map(|v| match v {
                '#' => 1,
                '.' => 0,
                v => panic!("unknown character {:?}", v),
            })
            .collect(),
    );
    let image = parse_image(image);

    (algo, image)
}

fn pad(arr: &Image, value: u32, width: usize) -> Image {
    // janky pad implementation
    let mut image = Array2::zeros(arr.raw_dim() + Dim([width * 2, width * 2]));
    image.fill(value);
    image
        .slice_mut(s![
            width..image.shape()[0] - width,
            width..image.shape()[1] - width
        ])
        .assign(&arr);
    image
}

fn enhance(image: &Image, algo: Algo) -> Image {
    ndarray::Zip::from(image.windows([3, 3])).map_collect(|v| {
        let val = v.iter().fold(0, |acc, v| acc * 2 + v) as usize;
        algo[val]
    })
}

fn parts((algo, image): (Algo, Image), n_iter: usize) -> u32 {
    let mut ret = image.clone();
    let flicker = algo[0] == 1;
    for i in 0..n_iter {
        let fill = (((i % 2) == 1) && flicker) as u32;
        ret = pad(&ret, fill, 4);
        ret = enhance(&ret, algo);
    }
    ret.iter().sum()
}

fn part1(input: (Algo, Image)) -> u32 {
    parts(input, 2)
}

fn part2(input: (Algo, Image)) -> u32 {
    parts(input, 50)
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(input.clone()));
    println!("{:?}", part2(input.clone()));
    Ok(())
}

#[test]
fn example() {
    let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
    let input = parse_input(input);
    assert_eq!(part1(input.clone()), 35);
    assert_eq!(part2(input.clone()), 3351);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(input.clone()), 5786);
    assert_eq!(part2(&input), 16757);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(input.clone());
        part2(input.clone());
    })
}
