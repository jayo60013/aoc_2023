use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    print_result(part1("input"), "Part 1");
    print_result(part2("input"), "Part 2");
}

fn part1(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut total: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let (winning, elf) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning: HashSet<_> = winning.split_whitespace().collect();
        let matches = elf
            .split_whitespace()
            .filter_map(|num| winning.contains(num).then_some(()))
            .count() as u32;
        total += if matches > 0 {
            2u32.pow(matches - 1)
        } else {
            0
        };
    }
    Ok(total)
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

fn print_result(result: io::Result<u32>, part_name: &str) {
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
        assert_eq!(part1("sample.txt").unwrap(), 13);
    }
    #[test]
    fn test_part1_input() {
        assert_eq!(part1("input").unwrap(), 22488);
    }
    #[test]
    fn test_part2_sample() {
        assert_eq!(part2("sample.txt").unwrap(), 30);
    }
    #[test]
    fn test_part2_input() {
        assert_eq!(part2("input").unwrap(), 7013204);
    }
}
