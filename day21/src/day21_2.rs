use std::collections::HashSet;

const FILE_DATA: &str = include_str!("input.txt");

fn wrap_to_range(n: i32, len: i32) -> i32 {
    if n >= 0 {
        n % len
    } else {
        len - (-(n+1) % (len)) - 1
    }
}

fn hash(set: &HashSet<(i32, i32)>) -> u64 {
    let mut hash = 6613;

    for coord in set {
        hash ^= (coord.0 as u64 ^ ((coord.1 as u64) * (2048+256+32+4))) as u64 + 0x9e3779b9 + (hash << 6) + (hash >> 2);
    }

    hash
}

pub fn answer() -> usize {
    let mut map: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut starting_pos = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                starting_pos = (x as i32, y as i32);
            }
        }
    }

    map[starting_pos.1 as usize][starting_pos.0 as usize] = '.';

    let mut positions = HashSet::new();
    //let mut positions_a = HashSet::new();
    //let mut positions_b = HashSet::new();
    //positions_a.insert(starting_pos);
    positions.insert(starting_pos);

    //let mut a = &mut positions_a;
    //let mut b = &mut positions_b;

    let mut hashes = HashSet::new();

    for i in 0..100 {
        for p in positions.clone() {
        //for p in a.iter() {
            if map[wrap_to_range(p.1, map.len() as i32) as usize][wrap_to_range(p.0 + 1, map[0].len() as i32) as usize] == '.' {
                positions.insert((p.0 + 1, p.1));
            }

            if map[wrap_to_range(p.1, map.len() as i32) as usize][wrap_to_range(p.0 - 1, map[0].len() as i32) as usize] == '.' {
                positions.insert((p.0 - 1, p.1));
            }

            if map[wrap_to_range(p.1 + 1, map.len() as i32) as usize][wrap_to_range(p.0, map[0].len() as i32) as usize] == '.' {
                positions.insert((p.0, p.1 + 1));
            }

            if map[wrap_to_range(p.1 - 1, map.len() as i32) as usize][wrap_to_range(p.0, map[0].len() as i32) as usize] == '.' {
                positions.insert((p.0, p.1 - 1));
            }
        
            positions.remove(&(p.0, p.1));
        }

        let h = hash(&positions);
        if hashes.contains(&h) {
            println!("repeat {i}");
            break;
        } else {
            hashes.insert(h);
        }
    }

    positions.len()
}
