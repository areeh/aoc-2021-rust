extern crate test;
use itertools::{enumerate, Itertools};
use ndarray::{s, Array2};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[cfg(test)]
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day23/input.txt")
}

type Board = Array2<char>;
type BoardMask = Array2<u8>;
type Pos = [usize; 2];
type Move = (usize, usize, Pos);
type AmphipodPositions = Vec<(Amphipod, Pos)>;
type VisitedStates = HashMap<Board, (bool, usize, Vec<(usize, Pos)>)>;

const ROOM_ENTRANCES: [Pos; 4] = [[3, 1], [5, 1], [7, 1], [9, 1]];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn cost(self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }
    fn home_col(self) -> usize {
        match self {
            Amphipod::A => 3,
            Amphipod::B => 5,
            Amphipod::C => 7,
            Amphipod::D => 9,
        }
    }
}

fn char_to_amphipod(c: char) -> Amphipod {
    match c {
        'A' => Amphipod::A,
        'B' => Amphipod::B,
        'C' => Amphipod::C,
        'D' => Amphipod::D,
        c => panic!("Got unexpected amphipod: {:?}", c),
    }
}

fn parse_input(input: &str, p2: bool) -> Board {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for (i, line) in enumerate(input.lines()) {
        if p2 & (i == 3) {
            data.append(&mut "  #D#C#B#A#  ".chars().collect_vec());
            data.append(&mut "  #D#B#A#C#  ".chars().collect_vec());
        }
        let mut row: Vec<_> = line.chars().collect();

        let mut postfix = None;

        if row.len() < board_width {
            postfix = Some(
                vec![' ']
                    .into_iter()
                    .cycle()
                    .take(board_width - row.len())
                    .collect_vec(),
            );
        }
        data.append(&mut row);

        if postfix.is_some() {
            data.append(&mut postfix.unwrap());
        }
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;
    Array2::from_shape_vec((n_rows, board_width), data)
        .unwrap()
        .reversed_axes()
}

fn amphipod_position(current_c: char, board: &Board) -> Vec<Pos> {
    board
        .indexed_iter()
        .filter_map(|(pos, c)| {
            if *c == current_c {
                Some([pos.0, pos.1])
            } else {
                None
            }
        })
        .collect_vec()
}

fn get_amphipod_positions(board: &Board) -> AmphipodPositions {
    let mut amphipods = Vec::new();
    for c in "ABCD".chars() {
        let char_pos = amphipod_position(c, board);
        for pos in char_pos {
            amphipods.push((char_to_amphipod(c), pos));
        }
    }
    amphipods
}

fn in_corridor(pos: Pos) -> bool {
    (pos[1] == 1) & (0..15).contains(&pos[0])
}

fn gen_movement_mask(idx: usize, amphipods: &AmphipodPositions, board: &Board) -> BoardMask {
    /*
    Generate movement mask for the map for an amphipod.
    0 => no passage
    1 => no stopping
    2 => free passage
    */
    let mut mask: BoardMask = Array2::zeros(board.raw_dim());
    let (self_amphipod, self_pos) = amphipods[idx];
    let home_col = self_amphipod.home_col();

    let mut corridor = mask.slice_mut(s![1..mask.shape()[0] - 1, 1]);
    if in_corridor(self_pos) {
        corridor.fill(1);
    } else {
        corridor.fill(2);
    }
    for pos in ROOM_ENTRANCES {
        let mut col_slice = mask.slice_mut(s![pos[0], 1..mask.shape()[1] - 1]);
        col_slice.fill(1);
    }

    for (_, pos) in amphipods {
        mask[*pos] = 0;
    }

    let foreign_in_home_col = amphipods
        .iter()
        .any(|(a_type, pos)| (*a_type != self_amphipod) & (pos[1] > 1) & (pos[0] == home_col));
    if !foreign_in_home_col {
        // Only open the backmost slot for stopping
        let mut prev = [home_col, 2];
        if mask[prev] != 0 {
            for row in 3..board.shape()[1] {
                let curr_pos = [home_col, row];
                if (mask[curr_pos] == 0) & (curr_pos != self_pos) {
                    mask[prev] = 2;
                    break;
                }
                prev = curr_pos;
            }
        }
    }

    mask
}

fn gen_legal_moves(amphipods: &AmphipodPositions, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    for idx in 0..amphipods.len() {
        let cost = amphipods[idx].0.cost();
        let start_pos = amphipods[idx].1;
        let movement_mask = gen_movement_mask(idx, amphipods, board);
        let mut to_visit = VecDeque::from([(start_pos, 0)]);
        let mut visited = HashSet::new();

        while let Some((current, n_moves)) = to_visit.pop_front() {
            const CARDINALS: &[(usize, usize); 4] =
                &[(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)];
            for dir in CARDINALS {
                let neighbor = [
                    current[0].wrapping_add(dir.0),
                    current[1].wrapping_add(dir.1),
                ];
                if visited.contains(&neighbor) {
                    continue;
                }
                if let Some(v) = movement_mask.get(neighbor) {
                    if *v > 0 {
                        to_visit.push_back((neighbor, n_moves + 1));
                        if movement_mask[neighbor] == 2 {
                            moves.push((idx, (n_moves + 1) * cost, neighbor));
                        }
                    }
                }
                visited.insert(current);
            }
        }
    }
    moves
}

fn amphipods_to_board(amphipods: &AmphipodPositions, board: &Board) -> Board {
    let mut board = Array2::from_elem(board.raw_dim(), ',');
    for (a_type, pos) in amphipods {
        board[*pos] = match a_type {
            Amphipod::A => 'A',
            Amphipod::B => 'B',
            Amphipod::C => 'C',
            Amphipod::D => 'D',
        }
    }
    board
}

fn is_win(board: &Board) -> bool {
    for (pos, c) in board.indexed_iter() {
        if (2..board.shape()[1] - 1).contains(&pos.1) {
            if (pos.0 == 3) & (*c != 'A') {
                return false;
            } else if (pos.0 == 5) & (*c != 'B') {
                return false;
            } else if (pos.0 == 7) & (*c != 'C') {
                return false;
            } else if (pos.0 == 9) & (*c != 'D') {
                return false;
            }
        }
    }
    true
}

fn logic(start_amphipods: AmphipodPositions, visited_states: &mut VisitedStates, board: &Board) {
    let mut move_queue = VecDeque::new();
    for m in gen_legal_moves(&start_amphipods, board) {
        let (a_idx, next_cost, to) = m;
        let mut amphipods = start_amphipods.clone();
        let mut moves = Vec::new();
        amphipods[a_idx] = (amphipods[a_idx].0, to);
        moves.push((a_idx, to));
        move_queue.push_back((next_cost, moves.clone(), amphipods));
    }

    while let Some(m) = move_queue.pop_front() {
        let (current_cost, moves, current_amphipods) = m;
        // println!("current_amphipods {:?}", current_amphipods);

        let c_board = amphipods_to_board(&current_amphipods, board);

        let win = is_win(&c_board);
        if let Some(entry) = visited_states.get(&c_board) {
            if current_cost < entry.1 {
                visited_states.insert(c_board, (win, current_cost, moves.clone()));
            } else {
                continue; // not the best way to reach this board state
            }
        } else {
            visited_states.insert(c_board, (win, current_cost, moves.clone()));
        }

        if win {
            continue;
        }

        let legal_moves = gen_legal_moves(&current_amphipods, board);
        for m in legal_moves.into_iter().rev() {
            let (a_idx, next_cost, to) = m;
            let mut next_amphipods = current_amphipods.clone();
            let mut moves = moves.clone();
            next_amphipods[a_idx] = (next_amphipods[a_idx].0, to);
            moves.push((a_idx, to));
            move_queue.push_back((current_cost + next_cost, moves.clone(), next_amphipods));
        }
    }
}

fn get_best_win_cost(visited_states: &VisitedStates) -> usize {
    visited_states
        .values()
        .filter_map(|(win, cost, _)| if *win { Some(*cost) } else { None })
        .min()
        .unwrap()
}

fn parts(input: &Board) -> usize {
    let mut visited_states = HashMap::new();
    let amphipods = get_amphipod_positions(&input);
    logic(amphipods, &mut visited_states, input);

    get_best_win_cost(&visited_states)
}

fn part1(input: &str) -> usize {
    let input = parse_input(input, false);
    parts(&input)
}

fn part2(input: &str) -> usize {
    let input = parse_input(input, true);
    parts(&input)
}

pub fn main() -> std::io::Result<()> {
    let input = input1()?;
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn not_winner() {
    let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
    let input = parse_input(input, false);
    assert!(!is_win(&input));
}

#[test]
fn not_winnerp2() {
    let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
    let input = parse_input(input, true);
    assert!(!is_win(&input));
}

#[test]
fn winner() {
    let input = "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";
    let input = parse_input(input, false);
    assert!(is_win(&input));
}

#[test]
fn winnerp2() {
    let input = "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########";
    let input = parse_input(input, false);
    assert!(is_win(&input));
}

#[test]
fn example() {
    let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
    assert_eq!(part1(&input), 12521);
    assert_eq!(part2(&input), 44169);
}

#[test]
fn task() {
    let input = input1().unwrap();
    assert_eq!(part1(&input), 15365);
    assert_eq!(part2(&input), 52055);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = input1().unwrap();
        part1(&input);
        part2(&input);
    })
}
