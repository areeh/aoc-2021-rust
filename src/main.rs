#![feature(test)]

use chrono::{Date, Datelike, Local, TimeZone};
use curl::easy::Easy;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

const TOKEN: &str = "***REMOVED***";

fn make_day(date: Date<Local>) -> std::io::Result<()> {
    let mut day_dir = PathBuf::from("./src/");
    day_dir.push(format!("day{}", date.day()));

    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        date.year(),
        date.day()
    );

    match fs::create_dir(&day_dir) {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => (),
            other_error => {
                panic!("Problem creating directory: {:?}", other_error)
            }
        },
    }

    let mut input_path = day_dir.clone();
    input_path.push("input.txt");

    if !input_path.exists() {
        let mut file = File::create(&input_path)?;

        let mut easy = Easy::new();
        easy.cookie(&format!("session={}", TOKEN)).unwrap();
        easy.url(&url).unwrap();
        easy.write_function(move |data| {
            file.write_all(data).unwrap();
            Ok(data.len())
        })
        .unwrap();
        easy.get(true).unwrap();
        easy.perform().expect(&format!(
            "Encountered error when performing request to {:?}",
            &url
        ));
        assert_eq!(easy.response_code().unwrap(), 200);
    }

    let mut rs_path = day_dir.clone();
    rs_path.push("mod.rs");

    if !rs_path.exists() {
        let _ = File::create(&rs_path)?;
    }

    Ok(())
}

fn make_some_day(year: i32, day: u32) -> std::io::Result<()> {
    let day = Local.ymd(year, 12, day);
    make_day(day)
}

fn make_until_today() -> std::io::Result<()> {
    // let today: Date<Local> = Local::today();
    (1..25 + 1)
        .map(|x| make_some_day(2021, x))
        .collect()
}

fn main() -> std::io::Result<()> {
    // make_until_today()?;
    day1::main()?;
    day2::main()?;
    day3::main()?;
    day4::main()?;
    day5::main()?;
    day6::main()?;
    day7::main()?;
    day8::main()?;
    day9::main()?;
    day10::main()?;
    day11::main()?;
    day12::main()?;
    day13::main()?;
    day14::main()?;
    day15::main()?;
    day16::main()?;
    day17::main()?;
    day18::main()?;
    day19::main()?;
    day20::main()?;
    day21::main()?;
    day22::main()?;
    day23::main()?;
    day24::main()?;
    day25::main()?;

    Ok(())
}
