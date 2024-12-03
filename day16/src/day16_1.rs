const FILE_DATA: &str = include_str!("input.txt");

use std::collections::HashSet;

// Taken mostly from day 10
fn char_to_dir(c: char, prev_dir: (i32, i32)) -> Vec<(i32, i32)> {
    match c {
        '-' => if prev_dir.1 != 0 {
            vec![(1, 0), (-1, 0)]
        } else {
            vec![(prev_dir.0, 0)]
        },
        '|' => if prev_dir.0 != 0 {
            vec![(0, 1), (0, -1)]
        } else {
            vec![(0, prev_dir.1)]
        },
        '/' => vec![(-prev_dir.1,-prev_dir.0)],
        '\\' => vec![(prev_dir.1, prev_dir.0)],
        _   => vec![prev_dir]
    }
}

struct Path {
    pub direction: (i32, i32),
    pub position: (i32, i32)
}

#[derive(Default)]
struct Visited {
    going_left: HashSet<(i32, i32)>,
    going_right: HashSet<(i32, i32)>,
    going_down: HashSet<(i32, i32)>,
    going_up: HashSet<(i32, i32)>,
    overall: HashSet<(i32, i32)>,
}

fn walk_path(path: Path, visited: &mut Visited, grid: &Vec<Vec<char>>) {
    let (mut x, mut y) = path.position;

    let correct_hash_set = if path.direction.0 == 1 {
        &mut visited.going_right
    } else if path.direction.0 == -1 {
        &mut visited.going_left
    } else if path.direction.1 == 1 {
        &mut visited.going_down
    } else /*if path.direction.1 == -1*/ {
        &mut visited.going_up
    };

    if correct_hash_set.contains(&path.position) {
        return;
    }

    visited.overall.insert((x, y));

    // First step
    x += path.direction.0;
    y += path.direction.1;

    if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
        return;
    }

    // Next steps
    while grid[y as usize][x as usize] == '.'  {
        visited.overall.insert((x, y));

        if correct_hash_set.contains(&(x, y)) {
            return;
        } else {
            correct_hash_set.insert((x, y));
        }
        
        y += path.direction.1;
        x += path.direction.0;

        if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
            return;
        }
    }

    let next_dirs = char_to_dir(grid[y as usize][x as usize], path.direction);

    for direction in next_dirs {
        walk_path(Path {
            position: (x, y),
            direction
        }, visited, grid);
    }
}

pub fn answer() -> usize {
    let grid: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| 
            line.chars().collect()
        ).collect();

    let mut visited = Visited::default();

    for direction in char_to_dir(grid[0][0], (1, 0)) {
        walk_path(Path {
            direction,
            position: (0, 0)
        }, &mut visited, &grid);
    }

    visited.overall.len()
}