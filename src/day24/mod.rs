extern crate test;
use itertools::{Either, Itertools};
use std::{fs, ops::Range};

#[cfg(test)]
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day24/input.txt")
}

type Alu = [i64; 4];

#[derive(Debug, Clone, Copy)]
enum Value {
    Var(usize),
    Number(i64),
}

fn parse_alu_var(c: char) -> usize {
    match c {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => panic!(),
    }
}

fn parse_value(s: &str) -> Value {
    match s.parse::<i64>() {
        Ok(n) => Value::Number(n),
        Err(_) => Value::Var(parse_alu_var(s.chars().next().unwrap())),
    }
}

#[derive(Debug, Clone)]
enum Cmd {
    Inp(usize),
    Set(usize, Value),
    Mul(usize, Value),
    Add(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
    Neq(usize, Value),
}

fn parse_input(input: &str) -> Vec<Cmd> {
    input
        .trim()
        .lines()
        .map(|v| {
            let v = v.trim().split_whitespace().collect_vec();
            let a = parse_alu_var(v[1].chars().next().unwrap());
            if v.len() == 2 {
                match v[0] {
                    "inp" => Cmd::Inp(a),
                    c => panic!("Unknown command with 1 argument {:?}", c),
                }
            } else {
                let b = parse_value(v[2]);
                match v[0] {
                    "inp" => Cmd::Set(a, b),
                    "neq" => Cmd::Neq(a, b),
                    "mul" => Cmd::Mul(a, b),
                    "eql" => Cmd::Eql(a, b),
                    "add" => Cmd::Add(a, b),
                    "div" => Cmd::Div(a, b),
                    "mod" => Cmd::Mod(a, b),
                    _ => panic!("parsing Cmd {:?} not implemented", v[0]),
                }
            }
        })
        .collect_vec()
}

fn value_to_i64(v: Value, alu: &Alu) -> i64 {
    match v {
        Value::Number(n) => n,
        Value::Var(v) => alu[v],
    }
}

fn digits14(n: usize) -> [usize; 14] {
    let mut current = n;
    let mut ret = [0; 14];
    for i in (0..14).rev() {
        ret[i] = current % 10;
        current /= 10;
    }
    ret
}

fn digits9(n: usize) -> [usize; 9] {
    let mut current = n;
    let mut ret = [0; 9];
    for i in (0..9).rev() {
        ret[i] = current % 10;
        current /= 10;
    }
    ret
}

fn digits7(n: usize) -> [usize; 7] {
    let mut current = n;
    let mut ret = [0; 7];
    for i in (0..7).rev() {
        ret[i] = current % 10;
        current /= 10;
    }
    ret
}

fn digits5(n: usize) -> [usize; 5] {
    let mut current = n;
    let mut ret = [0; 5];
    for i in (0..5).rev() {
        ret[i] = current % 10;
        current /= 10;
    }
    ret
}

fn digits_vec(n: usize) -> Vec<usize> {
    let mut current = n;
    let mut ret = Vec::new();
    loop {
        ret.push(current % 10);
        current /= 10;

        if current == 0 {
            break;
        }
    }
    ret.reverse();
    ret
}

fn count_inp1(cmds: &Vec<Cmd>) -> usize {
    cmds.iter()
        .filter(|v| if let Cmd::Inp(_) = v { true } else { false })
        .count()
}

fn run_alu<'a>(
    program: &Vec<Cmd>,
    number_iter: &mut impl Iterator<Item = &'a usize>,
    mut alu: Alu,
) -> Alu {
    for cmd in program {
        match cmd {
            Cmd::Inp(c) => alu[*c] = *number_iter.next().unwrap() as i64,
            Cmd::Set(a, b) => alu[*a] = value_to_i64(*b, &alu),
            Cmd::Neq(a, b) => alu[*a] = (alu[*a] != value_to_i64(*b, &alu)) as i64,
            Cmd::Mul(a, b) => alu[*a] = alu[*a] * value_to_i64(*b, &alu),
            Cmd::Eql(a, b) => alu[*a] = (alu[*a] == value_to_i64(*b, &alu)) as i64,
            Cmd::Add(a, b) => alu[*a] = alu[*a] + value_to_i64(*b, &alu),
            Cmd::Div(a, b) => alu[*a] = alu[*a] / value_to_i64(*b, &alu),
            Cmd::Mod(a, b) => alu[*a] = alu[*a] % value_to_i64(*b, &alu),
        };
    }
    alu
}

fn is_valid(z: i64) -> bool {
    let mul1 = z / 26;
    let mul2 = mul1 * 26;
    let mul3 = mul2 / 26;
    let rem4 = mul3 % 26;
    let mul4 = mul3 / 26;
    let rem5 = mul4 % 26;
    let mul5 = mul4 / 26;

    (16..=24).contains(&rem4) & (5..=13).contains(&rem5) & (mul5 == 0)
}

fn maybe_reverse(
    r: Range<usize>,
    rev: bool,
) -> itertools::Either<impl Iterator<Item = usize>, impl Iterator<Item = usize>> {
    if rev {
        itertools::Either::Left(r.rev())
    } else {
        itertools::Either::Right(r)
    }
}

fn parts(input: &Vec<Cmd>, rev: bool) -> impl Iterator<Item = usize> + '_ {
    let mut n = 0;
    let input_first = input
        .clone()
        .into_iter()
        .take_while(|cmd| {
            if let Cmd::Inp(_) = cmd {
                n += 1;
            }
            n < 10
        })
        .collect_vec();
    let input_second = &input[input_first.len()..input.len()];

    maybe_reverse(111111111..999999999, rev)
        .filter_map(move |v| {
            let digits_first_9 = digits9(v);
            if digits_first_9.iter().any(|v| *v == 0) {
                None
            } else {
                let alu = [0; 4];
                let alu = run_alu(&input_first, &mut digits_first_9.iter(), alu);
                if is_valid(alu[3]) {
                    Some((v, alu))
                } else {
                    None
                }
            }
        })
        .flat_map(move |(v_prev, alu)| {
            maybe_reverse(11111..99999, rev).filter_map(move |v| {
                let digits_final_5 = digits5(v);
                if digits_final_5.iter().any(|v| *v == 0) {
                    None
                } else {
                    let alu = run_alu(&input_second.to_vec(), &mut digits_final_5.iter(), alu);
                    if alu[3] == 0 {
                        Some(v_prev * 100000 + v)
                    } else {
                        None
                    }
                }
            })
        })
}

fn part1(input: &Vec<Cmd>) -> usize {
    parts(input, true).next().unwrap()
}

fn part2(input: &Vec<Cmd>) -> usize {
    parts(input, false).next().unwrap()
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[test]
fn digits_1() {
    assert_eq!(
        digits14(12345678912345),
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]
    )
}

#[test]
fn test_digits_vec() {
    assert_eq!(
        digits_vec(12345678912345),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]
    )
}

#[test]
fn example_negate() {
    let input = "inp x
    mul x -1";

    let input = parse_input(input);
    assert_eq!(run_alu(&input.clone(), &mut [3].iter(), [0; 4])[1], -3);
    assert_eq!(run_alu(&input.clone(), &mut [9].iter(), [0; 4])[1], -9);
}

#[test]
fn example_mul_eql() {
    let input = "inp z
    inp x
    mul z 3
    eql z x";

    let input = parse_input(input);
    assert_eq!(run_alu(&input.clone(), &mut [2, 6].iter(), [0; 4])[3], 1);
    assert_eq!(run_alu(&input.clone(), &mut [3, 6].iter(), [0; 4])[3], 0);
}

#[test]
fn example_add_div_mod() {
    let input = "inp w
    add z w
    mod z 2
    div w 2
    add y w
    mod y 2
    div w 2
    add x w
    mod x 2
    div w 2
    mod w 2";

    let input = parse_input(input);
    let alu = run_alu(&input, &mut [8].iter(), [0; 4]);
    assert_eq!(alu[0], 1);
    assert_eq!(alu[1], 0);
    assert_eq!(alu[2], 0);
    assert_eq!(alu[3], 0);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 59998426997979);
    assert_eq!(part2(&input), 13621111481315);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
