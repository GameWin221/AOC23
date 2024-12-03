const FILE_DATA: &str = include_str!("input.txt");

fn char_to_dir(c: char, prev_dir: (i32, i32)) -> (i32, i32) {
    match c {
        '-' => (prev_dir.0, 0),
        '|' => (0, prev_dir.1),
        'F' => (-prev_dir.1,-prev_dir.0),
        '7' => ( prev_dir.1, prev_dir.0),
        'L' => ( prev_dir.1, prev_dir.0),
        'J' => (-prev_dir.1,-prev_dir.0),
        _   => (0, 0)
    }
}

pub fn answer() -> usize {
    let chars: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (mut start_x, mut start_y) = (-1, -1);

    'find_s_loop: for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            if chars[y][x] == 'S' {
                (start_x, start_y) = (x as i32, y as i32);
                break 'find_s_loop;
            }
        }
    }

    let mut curr_pos: [(i32, i32); 2] = [(0, 0), (0, 0)];
    let mut prev_dir: [(i32, i32); 2] = [(0, 0), (0, 0)];

    let mut start_idx = 0;

    // Up
    if start_y < chars.len() as i32 - 1 {
        let c = chars[start_y as usize - 1][start_x as usize];
        if c == '|' || c == '7' || c == 'F'  {
            curr_pos[start_idx] = (start_x, start_y - 1);
            prev_dir[start_idx] = (0, -1);
            start_idx += 1;
        }
    }

    // Down
    if start_y > 0 {
        let c = chars[start_y as usize + 1][start_x as usize];
        if c == '|' || c == 'J' || c == 'L'  {
            curr_pos[start_idx] = (start_x, start_y + 1);
            prev_dir[start_idx] = (0, 1);
            start_idx += 1;
        }
    }

    // Right
    if start_x < chars[0].len() as i32 - 1 {
        let c = chars[start_y as usize][start_x as usize + 1];
        if c == '-' || c == 'J' || c == '7'  {
            curr_pos[start_idx] = (start_x + 1, start_y);
            prev_dir[start_idx] = (1, 0);
            start_idx += 1;
        }
    }

    // Left
    if start_x > 0 {
        let c = chars[start_y as usize][start_x as usize - 1];
        if c == '-' || c == 'L' || c == 'F'  {
            curr_pos[start_idx] = (start_x - 1, start_y);
            prev_dir[start_idx] = (-1, 0);
            start_idx += 1;
        }
    }
    
    let mut length = 1;
    while curr_pos[0] != curr_pos[1] {
        for i in 0..2 {
            let dir = char_to_dir(chars[curr_pos[i].1 as usize][curr_pos[i].0 as usize], prev_dir[i]);

            curr_pos[i].0 += dir.0;
            curr_pos[i].1 += dir.1;

            prev_dir[i] = dir;
        }

        length += 1;
    }

    length
}
