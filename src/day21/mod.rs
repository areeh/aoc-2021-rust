extern crate test;
use std::cmp::max;

#[cfg(test)]
use test::Bencher;

fn wrap(v: usize, wrap: usize) -> usize {
    1 + (v - 1) % (wrap - 1)
}

fn run_wins(
    score1: usize,
    score2: usize,
    pos1: usize,
    pos2: usize,
    wins: &mut [usize; 2],
    count: usize,
    i: usize,
) {
    if score1 >= 21 {
        wins[0] += count;
        return;
    }
    if score2 >= 21 {
        wins[1] += count;
        return;
    }

    const TRANSITIONS: [(usize, usize); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    
    for (v, n) in TRANSITIONS {
        let count = count * n;
        if (i % 2) == 0 {
            let pos1 = wrap(pos1 + v, 11);
            let score1 = score1 + pos1;
            run_wins(score1, score2, pos1, pos2, wins, count, i + 1)
        } else {
            let pos2 = wrap(pos2 + v, 11);
            let score2 = score2 + pos2;
            run_wins(score1, score2, pos1, pos2, wins, count, i + 1)
        }
    }
}

fn part2((mut pos1, mut pos2): (usize, usize)) -> usize {
    let mut wins = [0, 0];

    run_wins(0, 0, pos1, pos2, &mut wins, 1, 0);
    return max(wins[0], wins[1]);
}

pub fn main() -> std::io::Result<()> {
    let input = (1, 3);
    // println!("{:?}", part1(input));
    println!("{:?}", part2(input));
    Ok(())
}

#[test]
fn example() {
    let input = (4, 8);
    // assert_eq!(part1(input.clone()), 739785);
    assert_eq!(part2(input.clone()), 444356092776315);
}

#[test]
fn task() {
    let input = (1, 3);
    // assert_eq!(part1(), 897798);
    assert_eq!(part2(input.clone()), 48868319769358);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = (1, 3);
        // part1(&input);
        part2(input.clone());
    })
}
