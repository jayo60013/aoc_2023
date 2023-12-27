use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let grid = parse_input("input").unwrap();
    assert!(grid.len() == grid[0].len(), "Grid must be square");
    let start: (i32, i32) = ((grid.len() / 2) as i32, (grid.len() / 2) as i32);

    println!("Part 1: {}", reachable_plots(grid.clone(), start, 64));
    println!("Part 2: {}", part2(grid, 26501365));
}

fn part2(grid: Vec<String>, steps: i32) -> i64 {
    let size: i32 = grid.len() as i32;
    let mid = size / 2;

    let grid_width: i64 = (steps / size - 1) as i64;

    let odd_grids: i64 = ((grid_width - grid_width % 2 + 1) as i64).pow(2);
    let even_grids: i64 = ((grid_width + 1) as i64).pow(2);

    let odd_points: i64 = reachable_plots(grid.clone(), (mid, mid), size * 2 + 1);
    let even_points: i64 = reachable_plots(grid.clone(), (mid, mid), size * 2);

    let corner_t = reachable_plots(grid.clone(), (size - 1, mid), size - 1);
    let corner_r = reachable_plots(grid.clone(), (mid, 0), size - 1);
    let corner_b = reachable_plots(grid.clone(), (0, mid), size - 1);
    let corner_l = reachable_plots(grid.clone(), (mid, size - 1), size - 1);

    let small_tr: i64 = reachable_plots(grid.clone(), (size - 1, 0), size / 2 - 1);
    let small_tl: i64 = reachable_plots(grid.clone(), (size - 1, size - 1), size / 2 - 1);
    let small_br: i64 = reachable_plots(grid.clone(), (0, 0), size / 2 - 1);
    let small_bl: i64 = reachable_plots(grid.clone(), (0, size - 1), size / 2 - 1);

    let large_tr: i64 = reachable_plots(grid.clone(), (size - 1, 0), (size * 3) / 2 - 1);
    let large_tl: i64 = reachable_plots(grid.clone(), (size - 1, size - 1), (size * 3) / 2 - 1);
    let large_br: i64 = reachable_plots(grid.clone(), (0, 0), (size * 3) / 2 - 1);
    let large_bl: i64 = reachable_plots(grid.clone(), (0, size - 1), (size * 3) / 2 - 1);

    return odd_grids * odd_points
        + even_grids * even_points
        + corner_t
        + corner_r
        + corner_b
        + corner_l
        + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
        + grid_width * (large_tr + large_tl + large_br + large_bl);
}

fn reachable_plots(grid: Vec<String>, starting: (i32, i32), steps: i32) -> i64 {
    let size: i32 = grid.len() as i32;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut ans: HashSet<(i32, i32)> = HashSet::new();
    let mut to_visit: VecDeque<(i32, i32, i32)> =
        VecDeque::from(vec![(starting.0, starting.1, steps)]);

    while let Some((r, c, s)) = to_visit.pop_front() {
        // Because the elf HAS to take n steps
        // we need to make sure he can get back to this spot
        if s % 2 == 0 {
            ans.insert((r, c));
        }
        if s == 0 {
            continue;
        }

        for (nr, nc) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
            if (0..size).contains(&nr)
                && (0..size).contains(&nc)
                && !visited.contains(&(nr, nc))
                && grid
                    .get(nr as usize)
                    .unwrap()
                    .chars()
                    .nth(nc as usize)
                    .unwrap()
                    == '.'
            {
                visited.insert((nr, nc));
                to_visit.push_back((nr, nc, s - 1));
            }
        }
    }

    return ans.len() as i64;
}

fn parse_input(filename: &str) -> io::Result<Vec<String>> {
    let mut content = String::new();
    File::open(filename)?.read_to_string(&mut content)?;

    return Ok(content.lines().map(String::from).collect());
}
