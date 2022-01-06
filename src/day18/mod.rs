extern crate test;
use std::fmt;
use std::fs;
use std::ops::Add;
use std::str::Chars;

use itertools::Itertools;
#[cfg(test)]
use test::Bencher;

#[derive(Debug, Clone)]
enum Node {
    Pair(Box<Node>, Box<Node>),
    Leaf(u32),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pair(a, b) => write!(f, "[{},{}]", a, b),
            Self::Leaf(v) => write!(f, "{}", v),
        }
    }
}

impl Add for Node {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut ret = Self::Pair(Box::new(self), Box::new(other));
        calc(&mut ret);
        ret
    }
}

impl Node {
    fn try_into_tuple(&mut self) -> Option<(&mut Node, &mut Node)> {
        match self {
            Node::Pair(left, right) => Some((left, right)),
            Node::Leaf(_) => None,
        }
    }
}

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day18/input.txt")
}

fn dig_node(char_iter: &mut Chars) -> Node {
    match char_iter.next() {
        Some('[') => parse_node(char_iter),
        Some(c @ '0'..='9') => Node::Leaf(c.to_digit(10).unwrap()),
        c => panic!("Expected opening bracket or number, got {:?}", c),
    }
}

fn parse_node(char_iter: &mut Chars) -> Node {
    let left = dig_node(char_iter);

    let mid = char_iter.next();
    if mid.is_none() {
        return left;
    } else {
        assert_eq!(mid.unwrap(), ',');
    }

    let right = dig_node(char_iter);
    let end = char_iter.next().unwrap();
    assert_eq!(end, ']');
    Node::Pair(Box::new(left), Box::new(right))
}

fn parse_input(input: &str) -> Vec<Node> {
    let mut nodes = Vec::new();
    for line in input.lines() {
        let mut char_iter = line.trim().chars();
        nodes.push(parse_node(&mut char_iter));
    }
    nodes
}

fn add_left(node: &mut Node, value: u32) {
    match node {
        Node::Pair(left, _) => add_left(left, value),
        Node::Leaf(n) => *n += value,
    }
}

fn add_right(node: &mut Node, value: u32) {
    match node {
        Node::Pair(_, right) => add_right(right, value),
        Node::Leaf(n) => *n += value,
    }
}

fn explode(node: &mut Node, depth: usize) -> Option<(Option<u32>, Option<u32>)> {
    /*
    Explosions have 3 relevant situations:
    1: A pair with a leaf node on each side at depth 4
    -> Explode, propagate values up
    2: Try to explode the left node
    -> add right explode value to immediate right node, propagate left explode value up
    3: Try to explode the right node
    -> add left explode value to immediate left node, propagate right explode value up
    */

    if let Some((left, right)) = node.try_into_tuple() {
        match (left, right) {
            (Node::Leaf(l), Node::Leaf(r)) if depth == 4 => {
                let l = l.clone();
                let r = r.clone();
                *node = Node::Leaf(0);
                return Some((Some(l), Some(r)));
            }
            (left, right) => {
                if let Some((l, r)) = explode(left, depth + 1) {
                    r.map(|v| add_left(right, v));
                    return Some((l, None));
                }
                if let Some((l, r)) = explode(right, depth + 1) {
                    l.map(|v| add_right(left, v));
                    return Some((None, r));
                }
            }
        }
    }
    None
}

fn ceil_div(x: u32, d: u32) -> u32 {
    (x + d - 1) / d
}

fn split(node: &mut Node) -> Result<(), ()> {
    match node {
        Node::Pair(left, right) => {
            split(left)?;
            split(right)?;
        }
        Node::Leaf(v) => {
            if *v > 9 {
                *node = Node::Pair(
                    Box::new(Node::Leaf(*v / 2)),
                    Box::new(Node::Leaf(ceil_div(*v, 2))),
                );
                return Err(());
            }
        }
    }
    return Ok(());
}

fn calc(node: &mut Node) {
    loop {
        if let Some(_) = explode(node, 0) {
            continue;
        };
        if split(node).is_err() {
            continue;
        };
        // No change
        break;
    }
}

fn magnitude(node: &Node) -> u32 {
    match node {
        Node::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
        Node::Leaf(v) => *v,
    }
}

fn part1(input: Vec<Node>) -> u32 {
    input
        .into_iter()
        .reduce(|a, b| a + b)
        .map(|v| magnitude(&v))
        .unwrap()
}

fn part2(input: Vec<Node>) -> u32 {
    input
        .into_iter()
        .permutations(2)
        .map(|v| magnitude(&(v[0].clone() + v[1].clone())))
        .max()
        .unwrap()
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(input.clone()));
    println!("{:?}", part2(input.clone()));
    Ok(())
}

#[cfg(test)]
fn test_roundtrip(input: &str) {
    assert_eq!(format!("{}", parse_input(input)[0]), input);
}

#[cfg(test)]
fn test_explode(input: &str, expected: &str) {
    let mut node = parse_input(input);
    explode(&mut node[0], 0);
    assert_eq!(format!("{}", node[0]), expected);
}

#[test]
fn parse() {
    test_roundtrip("[[1,2],[[3,4],5]]");
    test_roundtrip("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    test_roundtrip("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]");
    test_roundtrip("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]");
}

#[test]
fn explosions() {
    test_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
    test_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
    test_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    test_explode(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
    );
    test_explode(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
    );
}

#[test]
fn test_add_calc() {
    let left = parse_input("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let right = parse_input("[1,1]");
    let mut res = left[0].clone() + right[0].clone();
    calc(&mut res);

    assert_eq!(format!("{}", res), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn example() {
    assert_eq!(part1(parse_input("[[1,2],[[3,4],5]]")), 143);
    assert_eq!(
        part1(parse_input("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
        1384
    );
    assert_eq!(
        part1(parse_input(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
        )),
        4140
    );
    assert_eq!(
        part2(parse_input(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
        )),
        3993
    );
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(input.clone()), 3359);
    assert_eq!(part2(input.clone()), 4616);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(input.clone());
        part2(input.clone());
    })
}
