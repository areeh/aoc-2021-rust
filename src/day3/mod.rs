use ndarray::{s, Array2, ArrayView1, ArrayView2, Axis};
use std::{
    fs,
    io::{BufRead, BufReader},
};

fn read_array(path: &str) -> std::io::Result<Array2<u16>> {
    let reader = BufReader::new(fs::File::open(path)?);
    let line_len = reader.lines().next().unwrap().unwrap().len();

    let br = BufReader::new(fs::File::open(path)?);
    let mut data = Vec::new();

    for line in br.lines() {
        let mut row: Vec<u16> = line
            .unwrap()
            .trim()
            .chars()
            .map(|v| v.to_digit(2).unwrap() as u16)
            .collect();
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / line_len;
    Ok(Array2::from_shape_vec((n_rows, line_len), data).unwrap())
}

#[allow(dead_code)]
fn read_array_str(input: &str) -> std::io::Result<Array2<u16>> {
    let line_len = input.lines().next().unwrap().len();
    let mut data = Vec::new();

    for line in input.lines() {
        let mut row: Vec<u16> = line
            .trim()
            .chars()
            .map(|v| v.to_digit(2).unwrap() as u16)
            .collect();
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / line_len;
    Ok(Array2::from_shape_vec((n_rows, line_len), data).unwrap())
}

fn to_u32(slice: ArrayView1<bool>) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

fn to_u32_bin_u16(slice: ArrayView1<u16>) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

fn part1(arr: ArrayView2<u16>) -> u32 {
    let half_n_elements = arr.dim().0 / 2;
    let most_mask = &arr
        .sum_axis(Axis(0))
        .mapv(|a| a > half_n_elements.try_into().unwrap());
    let least_mask = !most_mask;
    let gamma = to_u32(most_mask.view());
    let epsilon = to_u32(least_mask.view());

    gamma * epsilon
}

fn part2(arr: ArrayView2<u16>) -> u32 {
    let mut valid_indices_most: Vec<usize> = (0..arr.dim().0).collect();
    let mut valid_indices_least = valid_indices_most.to_vec();

    for i in 0..arr.dim().0 {
        let mut cnt = 0;

        for j in &valid_indices_most {
            if arr[[*j, i]] == 1 {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }

        let most_common = cnt >= 0;

        valid_indices_most.retain(|&j| arr[[j, i]] == most_common as u16);
        if valid_indices_most.len() == 1 {
            break;
        }
    }
    for i in 0..arr.dim().0 {
        let mut cnt = 0;

        for j in &valid_indices_least {
            if arr[[*j, i]] == 1 {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }

        let most_common = cnt >= 0;

        valid_indices_least.retain(|&j| arr[[j, i]] == !most_common as u16);
        if valid_indices_least.len() == 1 {
            break;
        }
    }
    let oxygen_idx = valid_indices_most[0];
    let co2_idx = valid_indices_least[0];

    let oxygen_arr = arr.slice(s![oxygen_idx, ..]);
    let co2_arr = arr.slice(s![co2_idx, ..]);

    let oxygen = to_u32_bin_u16(oxygen_arr.view());
    let co2 = to_u32_bin_u16(co2_arr.view());

    oxygen * co2
}

pub fn main() -> std::io::Result<()> {
    let arr = read_array("./src/day3/input.txt")?;
    println!("{:?}", part1(arr.view()));
    println!("{:?}", part2(arr.view()));
    Ok(())
}

#[test]
fn example() {
    let input = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";
    assert_eq!(part1(read_array_str(input).unwrap().view()), 198);
    assert_eq!(part2(read_array_str(input).unwrap().view()), 230);
}

#[test]
fn task() {
    assert_eq!(
        part1(read_array("./src/day3/input.txt").unwrap().view()),
        1458194
    );
    assert_eq!(
        part2(read_array("./src/day3/input.txt").unwrap().view()),
        2829354
    );
}
