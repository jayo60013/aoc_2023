use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Part 1: {}", part1("input"));
    match part2("input") {
        Ok(answer) => println!("Part 2: {}", answer),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}

fn part1(filename: &str) -> u32 {
    if let Ok(contents) = fs::read_to_string(filename) {
        return contents
            .lines()
            .filter_map(|line| {
                let (winning, elf) = line.split_once(':').unwrap().1.split_once('|').unwrap();
                let winning: HashSet<_> = winning.split_whitespace().collect();
                let matches = elf
                    .split_whitespace()
                    .filter_map(|num| winning.contains(num).then_some(()))
                    .count() as u32;
                (matches > 0).then(|| 2u32.pow(matches - 1))
            })
            .sum();
    } else {
        eprintln!("Error reading file: {}", filename);
        return 0;
    }
}

fn part2(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut copies: HashMap<usize, u32> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        copies.entry(i + 1).and_modify(|n| *n += 1).or_insert(1);

        let line = line?;
        let (winning, elf) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning: HashSet<_> = winning.split_whitespace().collect();
        let matches = elf
            .split_whitespace()
            .filter_map(|num| winning.contains(num).then_some(()))
            .count() as usize;

        for j in (i + 2)..matches + (i + 2) {
            if let Some(local_copies) = copies.get(&(i + 1)).cloned() {
                copies
                    .entry(j)
                    .and_modify(|n| *n += local_copies)
                    .or_insert(local_copies);
            }
        }
    }

    Ok(copies.values().cloned().sum())
}
