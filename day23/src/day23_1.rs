const FILE_DATA: &str = include_str!("input.txt");

#[derive(Debug)]
struct Connection{
    targets: Vec<Connection>,
    steps: usize
}

fn is_crossroad(coord: (i32, i32), map: &Vec<Vec<char>>) -> bool {
    let coord = (coord.0 as usize, coord.1 as usize);

    ((map[coord.1 - 1][coord.0] != '#') as u32 +
    (map[coord.1 + 1][coord.0] != '#') as u32 +
    (map[coord.1][coord.0 - 1] != '#') as u32 +
    (map[coord.1][coord.0 + 1] != '#') as u32) > 2
}
fn get_next_dirs(coord: (i32, i32), last_coord: (i32, i32), map: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut dirs = Vec::new();
    let dir_from_last_coord = (last_coord.0 - coord.0, last_coord.1 - coord.1);
    let coord = (coord.0 as usize, coord.1 as usize);

    // N
    let c = map[coord.1 - 1][coord.0];
    if (c == '.' || c == '^') && dir_from_last_coord.1 != -1 {
        dirs.push((0, -1));
    }
    // S
    let c = map[coord.1 + 1][coord.0];
    if (c == '.' || c == 'v') && dir_from_last_coord.1 != 1 {
        dirs.push((0, 1));
    }

    // W
    let c = map[coord.1][coord.0 - 1];
    if (c == '.' || c == '<') && dir_from_last_coord.0 != -1 {
        dirs.push((-1, 0));
    }
    // E
    let c = map[coord.1][coord.0 + 1];
    if (c == '.' || c == '>') && dir_from_last_coord.0 != 1 {
        dirs.push((1, 0));
    }

    dirs
}
fn get_next_crossroad(start: (i32, i32), direction: (i32, i32), map: &Vec<Vec<char>>) -> Connection {
    let mut last_coord = start;
    let mut coord = start;
    coord.0 += direction.0;
    coord.1 += direction.1;
    
    let mut steps = 1; // steps = 1 maybe??
    // This order is necessary, first check the coord then if it is a crossroad
    while coord.1 != map.len() as i32 - 1 && !is_crossroad(coord, map) {
        let dir = get_next_dirs(coord, last_coord, map).first().expect("Failed to find next directions while looking for the next crossroad").clone();
        
        last_coord = coord;

        coord.0 += dir.0;
        coord.1 += dir.1;
        steps += 1;
    }
    
    let targets = if (coord.1 as usize) < map.len() - 1 {
        get_next_dirs(coord, last_coord, map).into_iter().map(|dir| 
            get_next_crossroad(coord, dir, map)
        ).collect()
    } else {
        Vec::new()
    };

    Connection {
        targets,
        steps
    }
}

fn get_path_steps(root: &Connection, prev_steps: usize, distances: &mut Vec<usize>) {
    if root.targets.is_empty() {
        distances.push(prev_steps + root.steps);
        return;
    }

    for tgt in &root.targets {
        get_path_steps(tgt, prev_steps + root.steps, distances);
    }
}

pub fn answer() -> usize {
    let map: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| 
            line.chars().collect()
        ).collect();

    // - Find crossroads and roads between them
    // - Split to individual connections and take arrows into account (you cant get back)
    // - Find the longest path

    // 1. Find crossroads and roads between them
    let root = get_next_crossroad((1, 0), (0, 1), &map);

    // 2. Recursively go through every path and get its stepcount
    let mut distances = Vec::new();
    get_path_steps(&root, 0, &mut distances);

    distances.iter().max().unwrap().clone()
}