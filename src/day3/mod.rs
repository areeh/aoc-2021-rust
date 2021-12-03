use std::fs;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day3/input.txt")
}

fn to_u32(slice: &[bool]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

fn parse(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|v| u16::from_str_radix(v.trim(), 2))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn nth(v: &u16, bits: usize, n: usize) -> u16 {
    (v & (1 << bits - 1) >> n) >> (bits - 1 - n)
}

fn sum_nth_bit(bits_slice: &[u16], n: usize, bits: usize) -> usize {
    bits_slice
        .iter()
        .fold(0, |acc, v| acc + nth(v, bits, n) as usize)
}

fn part1(input: &str) -> u32 {
    let bits = input.lines().next().unwrap().len();
    let v = parse(input);
    let half_n_elements = v.len() / 2;
    let most_mask: Vec<_> = (0..bits)
        .map(|i| sum_nth_bit(&v, i, bits) > half_n_elements)
        .collect();
    let least_mask: Vec<_> = most_mask.iter().map(|v| !v).collect();
    to_u32(&most_mask) * to_u32(&least_mask)
}

fn some_retain(bit_vec: &[u16], bits: usize, flip: bool) -> u32 {
    let mut tmp_bit_vec = bit_vec.to_vec();
    for i in 0..bits {
        let half_n_elements = tmp_bit_vec.len() as f32 / 2.0;
        let sum_of_nth = sum_nth_bit(&tmp_bit_vec, i, bits);
        let cmp_val = (sum_of_nth as f32 >= half_n_elements) ^ flip;
        tmp_bit_vec.retain(|&v| nth(&v, bits, i) == cmp_val as u16);
        if tmp_bit_vec.len() == 1 {
            break;
        }
    }
    tmp_bit_vec[0] as u32
}

fn part2(input: &str) -> u32 {
    let bits = input.lines().next().unwrap().len();
    let bit_vec = parse(input);

    let oxy = some_retain(&bit_vec, bits, false);
    let co2 = some_retain(&bit_vec, bits, true);

    oxy * co2
}

pub fn main() -> std::io::Result<()> {
    let input = input1()?;
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
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
    assert_eq!(part1(input), 198);
    assert_eq!(part2(input), 230);
}

#[test]
fn task() {
    let input = input1().unwrap();
    assert_eq!(part1(&input), 1458194);
    assert_eq!(part2(&input), 2829354);
}
