use std::collections::HashSet;

const FILE_DATA: &str = include_str!("input.txt");

pub fn flood_fill(coord: (usize, usize), map: &mut Vec<Vec<char>>) {
    map[coord.1][coord.0] = '#';
    
    if coord.0 > 0 {
        if map[coord.1][coord.0 - 1] == '.' {
            flood_fill((coord.0 - 1, coord.1), map);
        }
    }
    if coord.0 < map[coord.1].len() - 1 {
        if map[coord.1][coord.0 + 1] == '.' {
            flood_fill((coord.0 + 1, coord.1), map);
        }
    }

    if coord.1 > 0 {
        if map[coord.1 - 1][coord.0] == '.' {
            flood_fill((coord.0, coord.1 - 1), map);
        }
    }
    if coord.1 < map.len() - 1 {
        if map[coord.1 + 1][coord.0] == '.' {
            flood_fill((coord.0, coord.1 + 1), map);
        }
    }
}

pub fn answer() -> usize {
    let steps: Vec<Vec<&str>> = FILE_DATA
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();
    
    let mut line: HashSet<(i32, i32)> = HashSet::new();

    let mut p_x = 0;
    let mut p_y = 0;

    line.insert((p_x, p_y));

    for step in steps {
        let c = step[0].chars().nth(0).unwrap();
        let length: i32 = step[1].parse().unwrap();
        
        let dir = match c {
            'U' => ( 0,-1),
            'D' => ( 0, 1),
            'R' => ( 1, 0),
            'L' => (-1, 0),
            _ => (0, 0)
        };
    
        //let start_p_x = p_x;
        for _ in 0..length {
            p_x += dir.0;
            p_y += dir.1;

            //if p_x == start_p_x || p_x == start_p_x + length {
                line.insert((p_x, p_y));
            //}
        }
    }

    let min_x = line.iter().min_by_key(|(x, _)| x).unwrap().0;
    let min_y = line.iter().min_by_key(|(_, y)| y).unwrap().1;

    let max_x = line.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = line.iter().max_by_key(|(_, y)| y).unwrap().1;

    let map_width = (max_x - min_x) as usize + 1;
    let map_height = (max_y - min_y) as usize + 1;

    let mut map = vec![vec!['.'; map_width]; map_height];
    for elem in &line {
        map[(elem.1 - min_y) as usize][(elem.0 - min_x) as usize] = '#';
    }

    std::thread::Builder::new().stack_size(map_width * map_height * 8 * 4).spawn(move || {
        flood_fill(((1 - min_x) as usize, (1 - min_y) as usize), &mut map);
        
        let mut sum = 0;

        for line in &map {
            for &c in line {
                if c == '#' {
                    sum += 1;
                }
            }
        }

        sum
    }).unwrap().join().unwrap()
}