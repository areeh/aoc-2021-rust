use std::fs;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x| x.parse().expect("Could not parse line of file as i32"))
        .collect()
}

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day1/input.txt")
}

fn part1(input: &[i32]) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2(input: &[i32]) -> usize {
    input
        .windows(3)
        .collect::<Vec<_>>()
        .windows(2)
        // [1..4].sum() > [0..3].sum() == [3] > [0]
        .filter(|w| w[1][2] > w[0][0])
        .count()
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}

#[test]
fn example() {
    let input = parse_input("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
    assert_eq!(part1(&input), 7);
    assert_eq!(part2(&input), 5);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 1121);
    assert_eq!(part2(&input), 1065);
}

#[test]
fn note() {
    // Do not produce values that can sum to overflow
    let arr: [i32; 32] = rand::random::<[i16; 32]>().map(|x| i32::from(x));

    arr.windows(3)
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| assert_eq!(
            w[1].iter().sum::<i32>() > w[0].iter().sum(),
            w[1][2] > w[0][0]
        )).collect()
}
