extern crate test;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[cfg(test)]
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day24/input.txt")
}

fn input2() -> std::io::Result<String> {
    fs::read_to_string("./src/day24/input2.txt")
}

fn input2_2() -> std::io::Result<String> {
    fs::read_to_string("./src/day24/input2_2.txt")
}

fn input3() -> std::io::Result<String> {
    fs::read_to_string("./src/day24/input3.txt")
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

fn run_alu<'a>(program: &Vec<Cmd>, number_iter: &mut impl Iterator<Item = &'a usize>) -> Alu {
    let mut alu = [0; 4];

    for cmd in program {
        match cmd {
            Cmd::Inp(c) => {
                // println!("{:?}", alu[3]);
                alu[*c] = *number_iter.next().unwrap() as i64
            }
            Cmd::Set(a, b) => alu[*a] = value_to_i64(*b, &alu),
            Cmd::Neq(a, b) => alu[*a] = (alu[*a] != value_to_i64(*b, &alu)) as i64,
            Cmd::Mul(a, b) => alu[*a] = alu[*a] * value_to_i64(*b, &alu),
            Cmd::Eql(a, b) => alu[*a] = (alu[*a] == value_to_i64(*b, &alu)) as i64,
            Cmd::Add(a, b) => alu[*a] = alu[*a] + value_to_i64(*b, &alu),
            Cmd::Div(a, b) => alu[*a] = alu[*a] / value_to_i64(*b, &alu),
            Cmd::Mod(a, b) => alu[*a] = alu[*a] % value_to_i64(*b, &alu),
        };
    }
    // println!("{:?}", alu[3]);
    alu
}

fn run_alu_memo<'a>(
    program: &Vec<Cmd>,
    number_iter: &mut impl Iterator<Item = &'a usize>,
    mut alu: Alu,
) -> Alu {
    for cmd in program {
        match cmd {
            Cmd::Inp(c) => {
                // println!("{:?}", alu[3]);
                alu[*c] = *number_iter.next().unwrap() as i64
            }
            Cmd::Set(a, b) => alu[*a] = value_to_i64(*b, &alu),
            Cmd::Neq(a, b) => alu[*a] = (alu[*a] != value_to_i64(*b, &alu)) as i64,
            Cmd::Mul(a, b) => alu[*a] = alu[*a] * value_to_i64(*b, &alu),
            Cmd::Eql(a, b) => alu[*a] = (alu[*a] == value_to_i64(*b, &alu)) as i64,
            Cmd::Add(a, b) => alu[*a] = alu[*a] + value_to_i64(*b, &alu),
            Cmd::Div(a, b) => alu[*a] = alu[*a] / value_to_i64(*b, &alu),
            Cmd::Mod(a, b) => alu[*a] = alu[*a] % value_to_i64(*b, &alu),
        };
    }
    // println!("{:?}", alu[3]);
    alu
}

fn search(input: &Vec<Cmd>) -> Vec<usize> {
    let mut ret = Vec::new();
    // for v in (111111111..999999999).rev() {
    for v in (111111111..999999999).rev() {
        let numbers = digits_vec(v);
        if numbers.iter().any(|v| *v == 0) {
            continue;
        }
        let alu = run_alu(input, &mut numbers.iter());
        // if alu[3] == 0 {
        //     return v;
        // }

        let mul1 = alu[3] / 26;
        let mul2 = mul1 * 26;
        let mul3 = mul2 / 26;
        let rem4 = mul3 % 26;
        let mul4 = mul3 / 26;
        let rem5 = mul4 % 26;
        let mul5 = mul4 / 26;

        if  (16..=24).contains(&rem4)
            & (5..=13).contains(&rem5)
            & (mul5 == 0)
        {
            ret.push((v, alu[3]))
        }
    }
    // ret.sort_by_key(|(_, z)| *z);
    ret.sort_by_key(|(v, _)| *v);
    println!("{:?}", ret.iter().take(100).collect_vec());
    println!("{:?}", ret.iter().rev().take(100).collect_vec());
    // println!("{:?}", ret.iter().filter(|(_, z)| *z < 10000).collect_vec());
    // println!("{:?}", ret.iter().rev().filter(|(_, z)| *z <= 16900).take(100).collect_vec());
    ret.iter().map(|(v, _)| *v).collect_vec()
}

fn validate(mut z: i64) -> Option<[i64; 5]>{
    let mut ret = [0; 5];
    let w_range = 1..=9;

    let mut x = (z % 26) - 6;
    z /= 26;


    if !w_range.contains(&x) {
        return None;
    }
    ret[0] = x;
    z *= 26;

    for w in w_range.clone() {
        ret[1] = w;
        z += w+2;

        x = z % 26;
        z /= 26;

        if !w_range.contains(&x) {
            continue;
        }
        ret[2] = x;

        x = (z % 26) - 15;
        z /= 26;

        if !w_range.contains(&x) {
            continue;
        }
        ret[3] = x;

        x = (z % 26) - 4; 
        z /= 26;

        if z != 0 {
            continue;
        }

        if !w_range.contains(&x) {
            continue;
        }

        ret[4] = x;
        return Some(ret);
    }
    None
}

fn parts(input: &Vec<Cmd>) -> (usize, [i64; 5]) {
    for v in (111111111..999999999).rev() {
        let numbers = digits_vec(v);
        if numbers.iter().any(|v| *v == 0) {
            continue;
        }
        let alu = run_alu(input, &mut numbers.iter());

        if let Some(r) = validate(alu[3]) {
            return (v, r)
        }
    }
    unreachable!();
}

fn part1_dp(input: &Vec<Cmd>) -> (usize, usize) {
    let mut memo = HashMap::new();

    let mut n = 0;
    let input_first = input
        .clone()
        .into_iter()
        .take_while(|v| {
            if let Cmd::Inp(_) = v {
                let ret = n < 7;
                n += 1;
                ret
            } else {
                true
            }
        })
        .collect_vec();
    let input_second = &input[input_first.len()..input.len()].to_vec();

    // println!("{:?}", input_first);
    // println!("{:?}", input_first.len());
    println!(
        "{:?}",
        input_first
            .iter()
            .filter(|v| if let Cmd::Inp(_) = v { true } else { false })
            .count()
    );
    // println!();

    // println!("{:?}", input_second);
    // println!("{:?}", input_second.len());
    println!(
        "{:?}",
        input_second
            .iter()
            .filter(|v| if let Cmd::Inp(_) = v { true } else { false })
            .count()
    );
    // println!();

    for v in (1111111..9999999) {
        let numbers = digits7(v);
        if numbers.iter().any(|v| *v == 0) {
            continue;
        }
        let alu = run_alu(&input_first, &mut numbers.iter());
        let entry = memo.entry(alu).or_insert(v);
        if *entry < v {
            *entry = v;
        }
    }

    for v in (11111..99999) {
        let numbers = digits5(v);
        for (alu, n) in memo.iter() {
            if numbers.iter().any(|v| *v == 0) {
                continue;
            }
            let alu = run_alu_memo(input_second, &mut numbers.iter(), *alu);
            if alu[3] == 0 {
                return (*n, v);
            }
        }
    }
    unreachable!()
}

fn part1(input: &Vec<Cmd>) -> usize {
    for v in (59998111111111..59998999999999).rev() {
        // for v in (11111111111111..99999999999999).rev() {
        let numbers = digits14(v);
        if numbers.iter().any(|v| *v == 0) {
            continue;
        }
        let alu = run_alu(input, &mut numbers.iter());
        if alu[3] == 0 {
            return v;
        }
    }
    unreachable!();
}

fn part2(input: &Vec<Cmd>) -> usize {
    for v in 13621111111111..13621999999999 {
        // for v in (11111111111111..99999999999999).rev() {
        let numbers = digits14(v);
        if numbers.iter().any(|v| *v == 0) {
            continue;
        }
        let alu = run_alu(input, &mut numbers.iter());
        if alu[3] == 0 {
            return v;
        }
    }
    unreachable!();
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    let candidates = search(&input);
    println!("{:?}", part2(&input));

    // run_alu(
    //     &input3,
    //     &mut digits14(131121114).iter(),
    // );
    // println!();
    // run_alu(
    //     &input2,
    //     &mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5].iter(),
    // );

    // run_alu(
    //     &input,
    //     &mut [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1].iter(),
    // );
    // println!();
    // run_alu(
    //     &input,
    //     &mut [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9].iter(),
    // );
    // println!();
    // run_alu(
    //     &input,
    //     &mut [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2].iter(),
    // );

    // println!("{:?}", part2(&input));
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
    assert_eq!(run_alu(&input.clone(), &mut [3].iter())[1], -3);
    assert_eq!(run_alu(&input.clone(), &mut [9].iter())[1], -9);
}

#[test]
fn example_mul_eql() {
    let input = "inp z
    inp x
    mul z 3
    eql z x";

    let input = parse_input(input);
    assert_eq!(run_alu(&input.clone(), &mut [2, 6].iter())[3], 1);
    assert_eq!(run_alu(&input.clone(), &mut [3, 6].iter())[3], 0);
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
    let alu = run_alu(&input, &mut [8].iter());
    assert_eq!(alu[0], 1);
    assert_eq!(alu[1], 0);
    assert_eq!(alu[2], 0);
    assert_eq!(alu[3], 0);
}

#[test]
fn task() {
    // let input = parse_input(&input1().unwrap());
    // assert_eq!(part1(&input), 375482);
    // assert_eq!(part2(&input), 1689540415957);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        // part2(&input);
    })
}
