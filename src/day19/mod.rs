extern crate test;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::ops::{Add, Sub};

#[cfg(test)]
use test::Bencher;

type Report = Vec<Position>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Position {
    fn manhattan(self, other: Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day19/input.txt")
}

fn parse_input(input: &str) -> Vec<Report> {
    let mut ret = Vec::new();
    let mut inner = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line == "" {
            ret.push(inner.clone());
            inner.clear();
        } else if line.starts_with("--") {
            continue;
        } else {
            let values: Vec<_> = line
                .split(',')
                .map(|v| v.parse())
                .collect::<Result<_, _>>()
                .unwrap();
            inner.push(Position {
                x: values[0],
                y: values[1],
                z: values[2],
            });
        }
    }
    if inner.len() > 0 {
        ret.push(inner.clone());
    }
    ret
}

fn turn(pos: &Position) -> Position {
    /* left 90 degree turn */
    Position {
        x: pos.y,
        y: -pos.x,
        z: pos.z,
    }
}

fn turn_map(report: Report) -> Report {
    report.iter().map(|c| turn(c)).collect_vec()
}

fn roll(pos: &Position) -> Position {
    /* left 90 degrees of a full barrel roll */
    Position {
        x: -pos.z,
        y: pos.y,
        z: pos.x,
    }
}

fn roll_map(report: Report) -> Report {
    report.iter().map(|c| roll(c)).collect_vec()
}

fn is_overlap(beacons: &HashSet<Position>, report: &Report) -> Option<(Position, Vec<Position>)> {
    beacons
        .iter()
        .cartesian_product(report)
        .map(|(bl, br)| *bl - *br)
        .find_map(|offset| {
            let candidate = report.iter().map(|v| (*v + offset));
            if candidate.clone().filter(|v| beacons.contains(v)).count() >= 12 {
                Some((offset, candidate.collect()))
            } else {
                None
            }
        })
}

fn find_overlapping(beacons: &mut HashSet<Position>, foo: &Report) -> Option<Position> {
    // A cube has 6 faces, scanner can be in 4 orientations per face
    let mut report = foo.clone();
    for _ in 0..2 {
        for _ in 0..3 {
            // 3 faces
            report = roll_map(report); // orientation 1
            if let Some((offset, match_report)) = is_overlap(beacons, &report) {
                // If we match, we have the correct offset, so any non-matches are new beacons
                beacons.extend(match_report);
                return Some(offset);
            };
            for _ in 0..3 {
                report = turn_map(report); // orientation 2, 3, 4
                if let Some((offset, match_report)) = is_overlap(beacons, &report) {
                    beacons.extend(match_report);
                    return Some(offset);
                };
            }
        }
        // Get over to the other 3 faces
        report = roll_map(turn_map(roll_map(report)));
    }
    None
}

fn parts(input: Vec<Report>) -> (HashSet<Position>, Vec<Position>) {
    let mut open = VecDeque::from(input);
    let mut scanners = vec![Position { x: 0, y: 0, z: 0 }];
    let mut beacons = HashSet::from_iter(open.pop_front().unwrap().into_iter());
    while let Some(report) = open.pop_front() {
        if let Some(new_scanner) = find_overlapping(&mut beacons, &report) {
            scanners.push(new_scanner);
        } else {
            open.push_back(report);
        }
    }
    (beacons, scanners)
}

fn part1(input: Vec<Report>) -> usize {
    let (beacons, _) = parts(input);
    beacons.len()
}

fn part2(input: Vec<Report>) -> usize {
    let (_, scanners) = parts(input);
    scanners
        .iter()
        .permutations(2)
        .map(|v| v[0].manhattan(*v[1]))
        .max()
        .unwrap()
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    println!("{:?}", part1(input.clone()));
    println!("{:?}", part2(input.clone()));
    Ok(())
}

#[test]
fn example() {
    let input = "--- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401
    
    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390
    
    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562
    
    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596
    
    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14";
    let input = parse_input(input);
    assert_eq!(part1(input.clone()), 79);
    assert_eq!(part2(input.clone()), 3621);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(input.clone()), 447);
    assert_eq!(part2(input.clone()), 15672);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(input.clone());
        part2(input.clone());
    })
}
