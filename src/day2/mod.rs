use itertools::Itertools;
use std::fs;
use std::ops::Add;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day2/input.txt")
}

fn parse_line(line: &str) -> (&str, i32) {
    if let Some((cmd, number)) = line.split_whitespace().collect_tuple() {
        (
            cmd,
            number.parse().expect("Could not parse second value as i32"),
        )
    } else {
        panic!("Expected exactly 2 elements")
    }
}

struct PositionOne {
    horizontal: i32,
    depth: i32,
}

impl Default for PositionOne {
    fn default() -> Self {
        PositionOne {
            horizontal: 0,
            depth: 0,
        }
    }
}

impl PositionOne {
    fn new() -> Self {
        Default::default()
    }
}

struct PositionTwo {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Default for PositionTwo {
    fn default() -> Self {
        PositionTwo {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl PositionTwo {
    fn new() -> Self {
        Default::default()
    }
}

trait UpdatePosition {
    fn update_position(&mut self, line: &str);
    fn update_position_all(&mut self, plan: &str) {
        for line in plan.lines() {
            self.update_position(line);
        }
    }
}

impl UpdatePosition for PositionOne {
    fn update_position(&mut self, line: &str) {
        let (cmd, number) = parse_line(line);
        match cmd {
            "forward" => self.horizontal += number,
            "down" => self.depth += number,
            "up" => self.depth -= number,
            _ => panic!("Unknown command"),
        }
    }
}

impl UpdatePosition for PositionTwo {
    fn update_position(&mut self, line: &str) {
        let (cmd, number) = parse_line(line);
        match cmd {
            "forward" => {
                self.horizontal += number;
                self.depth += number * self.aim;
            }
            "down" => self.aim += number,
            "up" => self.aim -= number,
            _ => panic!("Unknown command"),
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut pos = PositionOne::new();
    pos.update_position_all(input);
    pos.horizontal * pos.depth
}

fn part2(input: &str) -> i32 {
    let mut pos = PositionTwo::new();
    pos.update_position_all(input);
    pos.horizontal * pos.depth
}

pub fn main() -> std::io::Result<()> {
    let input = input1()?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Direction {
    fn from_line(line: &str) -> Self {
        let blah: Vec<&str> = line.split_whitespace().collect();
        let v: i32 = blah[1].parse().expect("i32");
        match blah[0] {
            "forward" => Direction::Forward(v),
            "down" => Direction::Down(v),
            "up" => Direction::Up(v),
            d => panic!("bad direction {}", d),
        }
    }
}

struct Pos {
    f: i32,
    d: i32,
}

impl Add<Direction> for Pos {
    type Output = Pos;
    fn add(self, dir: Direction) -> Pos {
        match dir {
            Direction::Forward(i) => Pos {
                d: self.d,
                f: self.f + i,
            },
            Direction::Down(i) => Pos {
                d: self.d + i,
                f: self.f,
            },
            Direction::Up(i) => Pos {
                d: self.d - i,
                f: self.f,
            },
        }
    }
}

struct Pos2 {
    f: i32,
    d: i32,
    aim: i32,
}

impl Add<Direction> for Pos2 {
    type Output = Pos2;
    fn add(self, dir: Direction) -> Pos2 {
        match dir {
            Direction::Forward(i) => Pos2 {
                d: self.d + self.aim * i,
                f: self.f + i,
                aim: self.aim,
            },
            Direction::Down(i) => Pos2 {
                d: self.d,
                f: self.f,
                aim: self.aim + i,
            },
            Direction::Up(i) => Pos2 {
                d: self.d,
                f: self.f,
                aim: self.aim - i,
            },
        }
    }
}

fn part1_jeff(input: &str) -> i32 {
    let pos = input
        .lines()
        .map(|line| Direction::from_line(line))
        .fold(Pos { d: 0, f: 0 }, |acc, x| acc + x);
    pos.d * pos.f
}

fn part2_jeff(input: &str) -> i32 {
    let pos = input
        .lines()
        .map(|line| Direction::from_line(line))
        .fold(Pos2 { d: 0, f: 0, aim: 0}, |acc, x| acc + x);
    pos.d * pos.f
}

#[test]
fn example() {
    let input = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";
    assert_eq!(part1(&input), 150);
    assert_eq!(part1_jeff(&input), 150);
    assert_eq!(part2(&input), 900);
    assert_eq!(part2_jeff(&input), 900);
}

#[test]
fn task() {
    let input = input1().unwrap();
    assert_eq!(part1(&input), 1427868);
    assert_eq!(part1_jeff(&input), 1427868);
    assert_eq!(part2(&input), 1568138742);
    assert_eq!(part2_jeff(&input), 1568138742);
}
