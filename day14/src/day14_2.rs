const FILE_DATA: &str = include_str!("input.txt");

use std::collections::HashMap;

fn debug_draw_map(columns: &Vec<Vec<(u32, bool)>>) {
    let mut map: Vec<Vec<char>> = vec![vec!['.'; columns.len()]; columns.len()];

    for x in 0..columns.len() {
        for (y, is_rock) in &columns[x] {
            if *is_rock {
                map[*y as usize][x] = 'O';
            } else {
                map[*y as usize][x] = '#';
            }
        }
    }

    println!("MAP:");
    for line in &map {
        println!("{:?}", line);
    }
}

fn hash_columns(columns: &Vec<Vec<(u32, bool)>>) -> u64 {
    let mut hash = 6613;

    for col in columns {
        for &(y, is_rock) in col {
            hash ^= (y as u64 ^ ((is_rock as u64) * (2048+256+32+4))) as u64 + 0x9e3779b9 + (hash << 6) + (hash >> 2);
        }
    }

    hash
}

fn rotate_cw(old_columns: Vec<Vec<(u32, bool)>>) -> Vec<Vec<(u32, bool)>> {
    let mut columns: Vec<Vec<(u32, bool)>> = vec![Vec::new(); old_columns.len()];

    for x in 0..old_columns.len() {
        for &(y_val, is_rock) in &old_columns[x] {
            columns[old_columns.len() - y_val as usize - 1].push((x as u32, is_rock));
        }
    }

    columns
}

pub fn answer() -> usize {
    let world: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut columns = Vec::new();
    for x in 0..world[0].len() {
        let mut column = Vec::with_capacity(world.len());

        for y in 0..world.len() {
            if world[y][x] != '.' {
                column.push((y as u32, world[y][x] == 'O'));
            }
        }

        columns.push(column);
    }

    let mut hashes: HashMap<u64, (usize, usize)> = HashMap::new();
    let mut previous_hashes = Vec::new();

    // Move and roll until we find a cycle pattern
    for i in 0..1_000_000_000 {
        let hash = hash_columns(&columns);
        previous_hashes.push(hash);

        if hashes.contains_key(&hash) {
            let cycle_start = hashes.get(&hash).unwrap().0;
            let cycle_length = i - cycle_start;
            let remaining_next_cycle = (1_000_000_000 - i) % cycle_length;

            // If there is a repetition found for iteration i
            // Get cycle_start (last index of the repeated hash)
            // Calculate cycle_length (between every cycle_length the cycle repeats itself)
            // Calculate remaining_next_cycle (remaining iterations for the cycle [that starts from 0] to repeat itself)
            // Return the sum for cycle_start + remaining_next_cycle (next occurence of the cycle that starts from 0)

            return hashes.get(&previous_hashes[cycle_start + remaining_next_cycle]).unwrap().1;
        } else {
            // If doesn't exist, remember a new sum for a given hash
            let mut sum = 0;

            for col in &columns {
                for &(y, is_rock) in col {
                    if is_rock {
                        sum += columns.len() - y as usize;
                    }
                }
            }

            hashes.insert(hash, (i, sum));
        }
    
        for _ in 0..4 {
            for col in &mut columns {
                if col.len() > 0 {
                    if col[0].1 {
                        col[0].0 = 0;
                    }

                    for row in 1..col.len() {
                        let (y, is_rock) = col[row];
            
                        if is_rock && y != 0 {
                            col[row].0 = col[row-1].0 + 1;
                        }
                    }
                }
            }

            columns = rotate_cw(columns);
        }
    }

    0
}