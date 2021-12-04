extern crate test;
use itertools::enumerate;
use itertools::Itertools;
use ndarray::{Array2, Axis};
use std::collections::HashMap;
use std::fs;
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day4/input.txt")
}

fn read_input(input: &str) -> std::io::Result<(Vec<Array2<u32>>, Vec<u32>)> {
    let mut lines_iter = input.lines();
    let numbers = lines_iter
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse())
        .collect::<Result<_, _>>()
        .unwrap();
    lines_iter.next();
    let board_size = input
        .lines()
        .skip(2)
        .next()
        .unwrap()
        .split_whitespace()
        .count();

    let mut boards: Vec<Array2<u32>> = Vec::new();
    let mut data = Vec::new();
    for line in lines_iter {
        if line.trim() == "" {
            let data_len = data.len();
            let n_rows = data_len / board_size;
            boards.push(Array2::from_shape_vec((n_rows, board_size), data.to_vec()).unwrap());
            data.clear();
        } else {
            let mut row: Vec<u32> = line
                .split_whitespace()
                .map(|v| v.parse())
                .collect::<Result<_, _>>()
                .unwrap();
            data.append(&mut row);
        }
    }
    let data_len = data.len();
    let n_rows = data_len / board_size;
    boards.push(Array2::from_shape_vec((n_rows, board_size), data.to_vec()).unwrap());
    data.clear();

    Ok((boards, numbers))
}

fn part1(input: &str, last: bool) -> std::io::Result<u32> {
    let (boards, numbers) = read_input(input)?;
    let num_boards = boards.len();
    let mut marked_boards: Vec<Array2<u32>> = (0..boards.len())
        .map(|i| Array2::ones(boards[i].dim()))
        .collect_vec();
    let mut reverse_index: HashMap<&u32, Vec<(usize, (usize, usize))>> = HashMap::new();

    for (i, b) in enumerate(&boards) {
        for ((x, y), elem) in b.indexed_iter() {
            reverse_index
                .entry(elem)
                .or_insert(Vec::new())
                .push((i, (x, y)));
        }
    }

    let mut rem_indices: Vec<_> = (0..num_boards).collect();
    let mut winner = 0;
    let mut winner_num: u32 = 0;
    let mut last_winner = false;
    for num in numbers {
        if let Some(idx_vec) = reverse_index.get(&num) {
            for (i, (x, y)) in idx_vec {
                let current_board = marked_boards.get_mut(i.to_owned()).unwrap();
                current_board[[x.to_owned(), y.to_owned()]] = 0;
            }

            fn predicate(j: usize, marked_boards: &Vec<Array2<u32>>) -> bool {
                let wins_x = marked_boards[j]
                    .sum_axis(Axis(0))
                    .iter()
                    .map(|v| *v == 0 as u32)
                    .any(|x| x);
                let wins_y = marked_boards[j]
                    .sum_axis(Axis(1))
                    .iter()
                    .map(|v| *v == 0 as u32)
                    .any(|x| x);
                wins_x | wins_y
            }

            if last & !last_winner {
                rem_indices.retain(|j| !predicate(*j, &marked_boards));
                if rem_indices.len() == 1 {
                    winner = rem_indices[0];
                    last_winner = true;
                }
            } else {
                if let Some(win_idx) = rem_indices
                    .iter()
                    .filter(|j| predicate(**j, &marked_boards))
                    .next()
                {
                    if winner == 0 {
                        winner = win_idx.to_owned();
                    }
                    winner_num = num;
                    break;
                }
            }
        }
    }

    return Ok((&boards[winner] * &marked_boards[winner]).sum() * winner_num);
}

pub fn main() -> std::io::Result<()> {
    let input = input1()?;
    println!("{:?}", part1(&input, false)?);
    println!("{:?}", part1(&input, true)?);
    Ok(())
}

#[test]
fn example() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";
    assert_eq!(part1(input, false).unwrap(), 4512);
    assert_eq!(part1(input, true).unwrap(), 1924);
}

#[test]
fn task() {
    let input = input1().unwrap();
    assert_eq!(part1(&input, false).unwrap(), 16716);
    assert_eq!(part1(&input, true).unwrap(), 4880);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = input1().unwrap();
        part1(&input, false).unwrap();
        part1(&input, true).unwrap();
    })
}
