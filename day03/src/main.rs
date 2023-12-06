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
    print_result(part1("sample.txt"), "Part 1");
    print_result(part2("sample.txt"), "Part 2");
}

fn part1(filename: &str) -> io::Result<i32> {
    let (part_numbers, symbols) = match parse_schematics(filename, r"\d+", r"[^\d\.\n]") {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Error: {}", err);
            (vec![], vec![])
        }
    };

    let mut sum: i32 = 0;
    for part_number in part_numbers.iter() {
        if check_if_part_number(part_number, symbols.clone()) {
            sum += part_number.number;
        }
    }
    Ok(sum)
}

fn part2(filename: &str) -> io::Result<i32> {
    let (mut part_numbers, gears) = match parse_schematics(filename, r"\d+", r"\*") {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Error: {}", err);
            (vec![], vec![])
        }
    };

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

fn parse_schematics(
    filename: &str,
    pnr: &str,
    sr: &str,
) -> io::Result<(Vec<PartNumber>, Vec<Point>)> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let part_number_regex = Regex::new(pnr).unwrap();
    let symbol_regex = Regex::new(sr).unwrap();

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

    Ok((part_numbers, symbols))
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

fn print_result(result: io::Result<i32>, part_name: &str) {
    match result {
        Ok(answer) => println!("{}: {}", part_name, answer),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1("sample.txt").unwrap(), 4361);
    }
    #[test]
    fn test_part1_input() {
        assert_eq!(part1("input").unwrap(), 536576);
    }
    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("sample.txt").unwrap(), 467835);
    }
    #[test]
    fn test_part2_input() {
        assert_eq!(part2("input").unwrap(), 75741499);
    }
}
