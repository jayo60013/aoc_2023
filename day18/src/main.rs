use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    print_result(part1("input"), "Part 1");
    print_result(part2("input"), "Part 2");
    let elapsed_time = Instant::now() - start_time;

    println!("Time: {:?}", elapsed_time);
}

fn part1(filename: &str) -> io::Result<i64> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut points = Vec::new();
    let mut perimeter = 0;
    let mut pos = (0, 0);

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();

        if let (Some(dir), Some(steps)) = (parts.next(), parts.next()) {
            let steps: i64 = steps.parse::<i64>().unwrap();
            match dir {
                "L" => pos.0 -= steps,
                "R" => pos.0 += steps,
                "U" => pos.1 -= steps,
                "D" => pos.1 += steps,
                _ => eprintln!("Unknown direction {dir}"),
            }
            points.push(pos);
            perimeter += steps;
        }
    }
    Ok(get_area(points, perimeter))
}

fn part2(filename: &str) -> io::Result<i64> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut points = Vec::new();
    let mut perimeter = 0;
    let mut pos = (0, 0);

    for line in reader.lines() {
        let line = line?;

        let color: Vec<char> = line
            .split_once("#")
            .map(|(_, hex)| hex[..hex.len() - 1].to_string())
            .unwrap()
            .chars()
            .collect();

        let (steps, d) = color.split_at(color.len() - 1);
        let steps = i64::from_str_radix(steps.iter().collect::<String>().as_str(), 16).unwrap();
        let d = d.iter().collect::<String>().parse::<i64>().unwrap();

        match d {
            0 => pos.0 += steps,
            1 => pos.1 += steps,
            2 => pos.0 -= steps,
            3 => pos.1 -= steps,
            _ => eprintln!("Unknown direction {d}"),
        }

        points.push(pos);
        perimeter += steps;
    }

    Ok(get_area(points, perimeter))
}

fn shoelace(vertices: Vec<(i64, i64)>) -> i64 {
    vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum::<i64>()
        .abs()
        / 2
}

fn get_area(vertices: Vec<(i64, i64)>, perimeter: i64) -> i64 {
    shoelace(vertices) + (perimeter / 2) + 1
}

fn print_result(result: io::Result<i64>, part_name: &str) {
    match result {
        Ok(answer) => println!("{}: {}", part_name, answer),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
