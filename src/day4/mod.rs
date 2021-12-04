extern crate test;
use itertools::Itertools;
use ndarray::{Array2, Axis};
use std::collections::HashMap;
use std::fs;
use test::Bencher;

type Board = Array2<u32>;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day4/input.txt")
}

fn read_input(input: &str) -> (Vec<u32>, Vec<Array2<u32>>) {
    let (numbers, boards_str) = input.split_once("\n\n").unwrap();
    let numbers = numbers
        .split(",")
        .map(|v| v.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    let board_size = input
        .lines()
        .skip(2)
        .next()
        .unwrap()
        .split_whitespace()
        .count();

    let mut boards: Vec<Array2<u32>> = Vec::new();
    let mut data = Vec::new();
    for line in boards_str.lines() {
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

    (numbers, boards)
}

fn build_reverse_index(boards: &Vec<Board>) -> Vec<HashMap<u32, Vec<(usize, usize)>>> {
    let mut reverse_index = Vec::new();

    for b in boards {
        reverse_index.push(HashMap::new());
        for ((x, y), elem) in b.indexed_iter() {
            reverse_index
                .last_mut()
                .unwrap()
                .entry(elem.clone())
                .or_insert(Vec::new())
                .push((x, y));
        }
    }
    reverse_index
}

fn calc_score(
    winner_num: u32,
    winner_idx: usize,
    boards: &Vec<Board>,
    marked_boards: Vec<Board>,
) -> u32 {
    (&boards[winner_idx] * &marked_boards[winner_idx]).sum() * winner_num
}

fn mark(
    i: usize,
    num: &u32,
    marked_boards: &mut Vec<Board>,
    reverse_index: &Vec<HashMap<u32, Vec<(usize, usize)>>>,
) {
    if let Some(idx_vec) = reverse_index[i].get(num) {
        for (x, y) in idx_vec {
            let board = marked_boards.get_mut(i).unwrap();
            board[[*x, *y]] = 0;
        }
    }
}

fn is_win(marked_board: &Board) -> bool {
    let wins_x = marked_board
        .sum_axis(Axis(0))
        .iter()
        .map(|v| *v == 0 as u32)
        .any(|x| x);
    let wins_y = marked_board
        .sum_axis(Axis(1))
        .iter()
        .map(|v| *v == 0 as u32)
        .any(|x| x);
    wins_x | wins_y
}

fn part1(numbers: &Vec<u32>, boards: &Vec<Board>) -> u32 {
    let num_boards = boards.len();
    let mut marked_boards: Vec<Board> = (0..boards.len())
        .map(|i| Array2::ones(boards[i].dim()))
        .collect_vec();
    let reverse_index = build_reverse_index(&boards);

    for num in numbers {
        for i in 0..num_boards {
            mark(i, &num, &mut marked_boards, &reverse_index);
            if is_win(&marked_boards[i]) {
                return calc_score(*num, i, boards, marked_boards);
            }
        }
    }
    unreachable!()
}

fn part2(numbers: &Vec<u32>, boards: &Vec<Board>) -> u32 {
    let num_boards = boards.len();
    let mut marked_boards: Vec<Board> = (0..boards.len())
        .map(|i| Array2::ones(boards[i].dim()))
        .collect_vec();

    let mut rem_indices: Vec<usize> = (0..num_boards).collect();
    let reverse_index = build_reverse_index(&boards);

    let mut winner_idx = 0;

    for num in numbers {
        if let [last_idx] = *rem_indices {
            winner_idx = last_idx;
        }
        for i in &rem_indices {
            mark(*i, &num, &mut marked_boards, &reverse_index);
        }

        rem_indices = rem_indices
            .into_iter()
            .filter(|i| !is_win(&marked_boards[*i]))
            .collect();

        if rem_indices.len() == 0 {
            return calc_score(*num, winner_idx, boards, marked_boards);
        }
    }
    unreachable!()
}

pub fn main() -> std::io::Result<()> {
    let (numbers, boards) = read_input(&input1()?);
    println!("{:?}", part1(&numbers, &boards));
    println!("{:?}", part2(&numbers, &boards));
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
    let (numbers, boards) = read_input(input);
    assert_eq!(part1(&numbers, &boards), 4512);
    assert_eq!(part2(&numbers, &boards), 1924);
}

#[test]
fn task() {
    let (numbers, boards) = read_input(&input1().unwrap());
    assert_eq!(part1(&numbers, &boards), 16716);
    assert_eq!(part2(&numbers, &boards), 4880);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let (numbers, boards) = read_input(&input1().unwrap());
        part1(&numbers, &boards);
        part2(&numbers, &boards);
    })
}
