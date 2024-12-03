use std::collections::HashSet;

const FILE_DATA: &str = include_str!("input.txt");

pub fn answer() -> usize {
    let mut map: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut starting_pos = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                starting_pos = (x, y);
            }
        }
    }

    let mut positions = HashSet::new();
    positions.insert(starting_pos);

    for i in 0..64 {
        for p in positions.clone() {
            if p.0 > 0 {
                if map[p.1][p.0 - 1] == '.' {
                    positions.insert((p.0 - 1, p.1));
                    map[p.1][p.0 - 1] = 'O';
                }
            }
            if p.0 < map[0].len() - 1 {
                if map[p.1][p.0 + 1] == '.' {
                    positions.insert((p.0 + 1, p.1));
                    map[p.1][p.0 + 1] = 'O';
                }
            }
            if p.1 > 0 {
                if map[p.1 - 1][p.0] == '.' {
                    positions.insert((p.0, p.1 - 1));
                    map[p.1 - 1][p.0] = 'O';
                }
            }
            if p.1 < map.len() - 1 {
                if map[p.1 + 1][p.0] == '.' {
                    positions.insert((p.0, p.1+1));
                    map[p.1 + 1][p.0] = 'O';
                }
            }

            map[p.1][p.0] = '.';

            positions.remove(&(p.0, p.1));
        }
    }

    positions.len()
}
