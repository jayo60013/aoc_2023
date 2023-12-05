use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct PartNumber {
    number: i32,
    len: i32,
    pos: Point,
    found: bool,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    col: i32,
    row: i32,
}

fn main() {
    let filename = "input";

    match part1(filename) {
        Ok(answer) => println!("Part 1: {}", answer),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
    match part2(filename) {
        Ok(answer) => println!("Part 2: {}", answer),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}

fn part2(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let part_number_regex = Regex::new(r"\d+").unwrap();
    let gear_regex = Regex::new(r"\*").unwrap();

    let mut part_numbers = vec![];
    let mut gears = vec![];

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        for part_match in part_number_regex.find_iter(&line) {
            part_numbers.push(PartNumber {
                number: part_match.as_str().parse::<i32>().unwrap(),
                len: part_match.as_str().len() as i32,
                pos: Point {
                    col: part_match.start() as i32,
                    row: i as i32,
                },
                found: false,
            });
        }

        for gear_match in gear_regex.find_iter(&line) {
            gears.push(Point {
                col: gear_match.start() as i32,
                row: i as i32,
            });
        }
    }

    let mut sum: i32 = 0;
    let mut gear_ratio: [i32; 2] = [-1, -1];

    for gear in gears.iter() {
        for part_number in part_numbers.iter_mut() {
            for i in 0..part_number.len {
                if check_adjacent(
                    Point {
                        col: part_number.pos.col + i,
                        row: part_number.pos.row,
                    },
                    gear.clone(),
                ) && !part_number.found
                {
                    if gear_ratio[0] == -1 {
                        gear_ratio[0] = part_number.number;
                    } else if gear_ratio[1] == -1 {
                        gear_ratio[1] = part_number.number;
                    } else {
                        gear_ratio[0] = -1;
                        gear_ratio[1] = -1;
                    }
                    part_number.found = true;
                }
            }
        }
        if gear_ratio[0] != -1 && gear_ratio[1] != -1 {
            sum += gear_ratio[0] * gear_ratio[1];
        }
        gear_ratio[0] = -1;
        gear_ratio[1] = -1;
    }

    Ok(sum)
}

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let part_number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"[^\d\.\n]").unwrap();

    let mut part_numbers = vec![];
    let mut symbols = vec![];

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        for part_match in part_number_regex.find_iter(&line) {
            part_numbers.push(PartNumber {
                number: part_match.as_str().parse::<i32>().unwrap(),
                len: part_match.as_str().len() as i32,
                pos: Point {
                    col: part_match.start() as i32,
                    row: i as i32,
                },
                found: false,
            });
        }

        for symbol_match in symbol_regex.find_iter(&line) {
            symbols.push(Point {
                col: symbol_match.start() as i32,
                row: i as i32,
            });
        }
    }

    let mut sum: i32 = 0;
    for part_number in part_numbers.iter() {
        if check_if_part_number(part_number, symbols.clone()) {
            sum += part_number.number;
        }
    }
    Ok(sum)
}

fn check_if_part_number(part_number: &PartNumber, symbols: Vec<Point>) -> bool {
    for symbol in symbols.iter() {
        for i in 0..part_number.len {
            if check_adjacent(
                Point {
                    col: part_number.pos.col + i,
                    row: part_number.pos.row,
                },
                symbol.clone(),
            ) {
                return true;
            }
        }
    }
    return false;
}

fn check_adjacent(a: Point, b: Point) -> bool {
    (a.col - b.col).abs() <= 1 && (a.row - b.row).abs() <= 1
}
