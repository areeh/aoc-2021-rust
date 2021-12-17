extern crate test;
use itertools::Itertools;
use ndarray::Array2;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;

#[cfg(test)]
use test::Bencher;

const CARDINALS: &[(usize, usize); 4] = &[(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)];

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day15/input.txt")
}

fn parse_input(input: &str) -> Array2<u32> {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<_> = line
            .trim()
            .chars()
            .map(|v| v.to_digit(10))
            .collect::<Option<_>>()
            .expect("could not convert char to digit");
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;
    let floor = Array2::from_shape_vec((n_rows, board_width), data).unwrap();

    floor
}

fn manhattan_distance((x, y): (usize, usize), (dest_x, dest_y): (usize, usize)) -> u32 {
    let ret = ((dest_x as i32 - x as i32).abs() + (dest_y as i32 - y as i32)).abs() as u32;
    ret
}

fn get_neighbors_opts(
    pos: (usize, usize),
    floor: &Array2<u32>,
    maxs: (usize, usize),
    max_wrap_range: Range<usize>,
    wrap: bool,
) -> Vec<((usize, usize), u32)> {
    if wrap {
        wrapping_get_neighbors(pos, maxs, max_wrap_range, floor)
    } else {
        get_neighbors(pos, floor)
    }
}

fn get_neighbors((x, y): (usize, usize), floor: &Array2<u32>) -> Vec<((usize, usize), u32)> {
    let mut neighbors = Vec::new();
    for dir in CARDINALS {
        let next = (x.wrapping_add(dir.0), y.wrapping_add(dir.1));
        if let Some(v) = floor.get(next) {
            neighbors.push((next, *v));
        }
    }
    neighbors
}

fn get_wrap_cost(
    (x, y): (usize, usize),
    (max_x, max_y): (usize, usize),
    max_wrap_range: &Range<usize>,
    floor: &Array2<u32>,
) -> Option<u32> {
    if !max_wrap_range.contains(&x) | !max_wrap_range.contains(&y) {
        return None;
    }

    let wrap_next = (x % max_x, y % max_y);
    let add = ((x / max_x) + (y / max_y)) as u32;

    if let Some(v) = floor.get(wrap_next) {
        let mut val = v + add;
        if val > 9 {
            val -= 9;
        }
        Some(val)
    } else {
        None
    }
}

fn wrapping_get_neighbors(
    (x, y): (usize, usize),
    maxs: (usize, usize),
    max_wrap_range: Range<usize>,
    floor: &Array2<u32>,
) -> Vec<((usize, usize), u32)> {
    let mut neighbors = Vec::new();

    for dir in CARDINALS {
        let next = (x.wrapping_add(dir.0), y.wrapping_add(dir.1));
        if let Some(v) = get_wrap_cost(next, maxs, &max_wrap_range, floor) {
            neighbors.push((next, v));
        }
    }
    neighbors
}

fn retrace_path(
    current_pos: (usize, usize),
    came_from: HashMap<(usize, usize), (usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut path = vec![current_pos];
    let mut curr_pos = current_pos.clone();
    while came_from.contains_key(&curr_pos) {
        curr_pos = came_from[&curr_pos];
        if curr_pos == (0, 0) {
            break;
        }
        path.push(curr_pos);
    }
    path.into_iter().rev().collect_vec()
}

fn a_star(floor: &Array2<u32>, end: (usize, usize), wrap: bool) -> Vec<(usize, usize)> {
    let start = (0, 0);
    let mut from_start_cost = HashMap::from([(start, 0)]);
    let mut to_end_cost_estimate = HashMap::from([(start, manhattan_distance(start, end))]);

    let mut closed = HashSet::new();
    let mut open = PriorityQueue::new();
    open.push(start, -(manhattan_distance(start, end) as i32));

    let mut came_from = HashMap::new();

    while let Some((current_pos, _)) = open.pop() {
        if current_pos == end {
            return retrace_path(current_pos, came_from);
        }
        closed.insert(current_pos);

        for (neighbor_pos, neighbor_cost) in
            get_neighbors_opts(current_pos, floor, floor.dim(), 0..floor.dim().0 * 5, wrap)
        {
            if closed.contains(&neighbor_pos) {
                continue;
            }

            let neighbor_cost_estimate = from_start_cost[&current_pos] + neighbor_cost;

            if let Some(cost) = from_start_cost.get_mut(&neighbor_pos) {
                if neighbor_cost_estimate >= *cost {
                    continue;
                }
            } else {
                open.push(neighbor_pos, -(neighbor_cost_estimate as i32));
            }
            came_from.insert(neighbor_pos, current_pos);
            from_start_cost.insert(neighbor_pos, neighbor_cost_estimate);
            to_end_cost_estimate.insert(
                neighbor_pos,
                neighbor_cost_estimate + manhattan_distance(neighbor_pos, end),
            );
        }
    }
    unreachable!();
}

fn part1(floor: &Array2<u32>) -> u32 {
    let path = a_star(floor, (floor.dim().0 - 1, floor.dim().1 - 1), false);
    path.iter().fold(0, |acc, (x, y)| acc + floor[[*x, *y]])
}

fn part2(floor: &Array2<u32>) -> u32 {
    let path = a_star(floor, (5 * floor.dim().0 - 1, 5 * floor.dim().1 - 1), true);
    path.iter().fold(0, |acc, pos| {
        acc + get_wrap_cost(*pos, floor.dim(), &(0..floor.dim().0 * 5), floor).unwrap()
    })
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn example() {
    let input = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";
    let input = parse_input(input);
    assert_eq!(part1(&input), 40);
    assert_eq!(part2(&input), 315);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 388);
    assert_eq!(part2(&input), 2819);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
