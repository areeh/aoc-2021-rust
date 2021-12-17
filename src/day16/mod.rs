extern crate test;
use bit_vec::BitVec;
use itertools::Itertools;
use std::fs;
use std::iter;

#[cfg(test)]
use test::Bencher;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day16/input.txt")
}

fn parse_input(input: &str) -> BitVec {
    BitVec::from_bytes(
        (0..input.trim().len())
            .step_by(2)
            .into_iter()
            .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
            .collect_vec()
            .as_slice(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: u8,
    body: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Literal(u64),
    Packets(Vec<Packet>),
}

fn parse_literal(bits: &mut bit_vec::IntoIter) -> Option<u64> {
    let mut value = 0u64;
    while let Some(prefix) = bits.next() {
        let part = bits.by_ref().take(4).fold(0, |acc, v| acc * 2 + v as u64);
        value = value << 4 | part;
        if !prefix {
            break;
        }
    }
    Some(value)
}

fn parse_packet(bits: &mut bit_vec::IntoIter) -> Option<Packet> {
    let version = bits.take(3).fold(0, |acc, v| acc * 2 + v as u8);
    let type_id = bits.take(3).fold(0, |acc, v| acc * 2 + v as u8);
    let body = match type_id {
        4 => Value::Literal(parse_literal(bits)?),
        _ => Value::Packets(parse_packets(bits)?),
    };
    Some(Packet {
        version,
        type_id,
        body,
    })
}

fn sum_versions(packet: &Packet) -> usize {
    let value = match &packet.body {
        Value::Literal(_) => packet.version as usize,
        Value::Packets(packets) => {
            packets.iter().map(|p| sum_versions(p)).sum::<usize>() + packet.version as usize
        }
    };
    value as usize
}

fn parse_packets(bits: &mut bit_vec::IntoIter) -> Option<Vec<Packet>> {
    let length_type_id = bits.next()?;
    let packets = if length_type_id {
        let n_packets = bits.take(11).fold(0, |acc, v| acc * 2 + v as usize);
        iter::repeat_with(|| parse_packet(bits).unwrap())
            .take(n_packets)
            .collect()
    } else {
        let n_bits = bits.take(15).fold(0, |acc, v| acc * 2 + v as usize);
        let mut bits = bits.take(n_bits).collect::<BitVec>().into_iter();
        iter::from_fn(|| parse_packet(&mut bits)).collect()
    };
    Some(packets)
}

fn run_packets(packet: &Packet) -> usize {
    match &packet.body {
        Value::Literal(v) => *v as usize,
        Value::Packets(packets) => match packet.type_id {
            0 => packets.iter().map(run_packets).sum(),
            1 => packets.iter().map(run_packets).product(),
            2 => packets.iter().map(run_packets).min().unwrap(),
            3 => packets.iter().map(run_packets).max().unwrap(),
            5 => (run_packets(&packets[0]) > run_packets(&packets[1])) as usize,
            6 => (run_packets(&packets[0]) < run_packets(&packets[1])) as usize,
            7 => (run_packets(&packets[0]) == run_packets(&packets[1])) as usize,
            _ => unreachable!(),
        }
    }
}

fn part1(input: &BitVec) -> usize {
    sum_versions(&parse_packet(&mut input.clone().into_iter()).unwrap())
}

fn part2(input: &BitVec) -> usize {
    run_packets(&parse_packet(&mut input.clone().into_iter()).unwrap())
}

pub fn main() -> std::io::Result<()> {
    let input = parse_input(&input1()?);
    // println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

#[cfg(test)]
fn literal(version: u8, value: u64) -> Packet {
    Packet {
        version,
        type_id: 4,
        body: Value::Literal(value),
    }
}

#[test]
fn example1() {
    let input = parse_input("D2FE28");
    assert_eq!(
        parse_packet(&mut input.into_iter()).unwrap(),
        literal(6, 2021),
    );
}

#[test]
fn example2() {
    let input = parse_input("38006F45291200");
    assert_eq!(
        parse_packet(&mut input.into_iter()).unwrap(),
        Packet {
            version: 1,
            type_id: 6,
            body: Value::Packets(vec![literal(6, 10), literal(2, 20)]),
        }
    );
}

#[test]
fn example3() {
    let input = parse_input("EE00D40C823060");
    assert_eq!(
        parse_packet(&mut input.into_iter()).unwrap(),
        Packet {
            version: 7,
            type_id: 3,
            body: Value::Packets(vec![literal(2, 1), literal(4, 2), literal(1, 3)]),
        }
    );
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse_input("8A004A801A8002F478")), 16);
    assert_eq!(part1(&parse_input("620080001611562C8802118E34")), 12);
    assert_eq!(part1(&parse_input("C0015000016115A2E0802F182340")), 23);
    assert_eq!(part1(&parse_input("A0016C880162017C3686B18A3D4780")), 31);
}


#[test]
fn test_p2() {
    assert_eq!(part2(&parse_input("C200B40A82")), 3);
    assert_eq!(part2(&parse_input("04005AC33890")), 54);
    assert_eq!(part2(&parse_input("880086C3E88112")), 7);
    assert_eq!(part2(&parse_input("CE00C43D881120")), 9);
    assert_eq!(part2(&parse_input("D8005AC2A8F0")), 1);
    assert_eq!(part2(&parse_input("F600BC2D8F")), 0);
    assert_eq!(part2(&parse_input("F600BC7D8E")), 1);
    assert_eq!(part2(&parse_input("9C005AC2F8F0")), 0);
    assert_eq!(part2(&parse_input("9C005AC7F8F0")), 1);
    assert_eq!(part2(&parse_input("9C0141080250320F1802104A08")), 1);
}

#[test]
fn task() {
    let input = parse_input(&input1().unwrap());
    assert_eq!(part1(&input), 895);
    assert_eq!(part2(&input), 1148595959144);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = parse_input(&input1().unwrap());
        part1(&input);
        part2(&input);
    })
}
