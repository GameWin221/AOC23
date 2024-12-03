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

struct Line {
    start: (i32, i32),
    end: (i32, i32)
}

// Check if a horizontal line starting from x: p.0 at height y: p.1 intersects with a perpendicular or a parallel line
fn row_intersects_line_right(p: (i32, i32), l: &Line) -> bool {
    let max_x = l.start.0.max(l.end.0);

    // If the row start behind the line
    if p.0 > max_x {
        return false;
    }
    
    let min_y = l.start.1.min(l.end.1);
    let max_y = l.start.1.max(l.end.1);

    // If the row start under or above or on the line 
    if p.1 <= min_y || p.1 > max_y {
        return false;
    }

    true
}

fn is_p_inside(p: (i32, i32), lines: &Vec<Line>) -> bool {
    let mut intersections = 0;
    
    for line in lines {
        if row_intersects_line_right(p, line) {
            intersections += 1;
        }
    }

    intersections % 2 == 1
}

pub fn answer() -> usize {
    let chars: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut start_pos = (-1, -1);

    'find_s_loop: for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            if chars[y][x] == 'S' {
                start_pos = (x as i32, y as i32);
                break 'find_s_loop;
            }
        }
    }

    let mut curr_pos = (0, 0);
    let mut prev_dir = (0, 0);

    // Up
    if start_pos.1 > 0 {
        let c = chars[start_pos.1 as usize - 1][start_pos.0 as usize];
        if c == '|' || c == '7' || c == 'F'  {
            curr_pos = (start_pos.0, start_pos.1 - 1);
            prev_dir = (0, -1);
        }
    }

    // Down
    if start_pos.1 < chars.len() as i32 - 1 {
        let c = chars[start_pos.1 as usize + 1][start_pos.0 as usize];
        if c == '|' || c == 'J' || c == 'L'  {
            curr_pos = (start_pos.0, start_pos.1 + 1);
            prev_dir = (0, 1);
        }
    }

    // Right
    if start_pos.0 < chars[0].len() as i32 - 1 {
        let c = chars[start_pos.1 as usize][start_pos.0 as usize + 1];
        if c == '-' || c == 'J' || c == '7'  {
            curr_pos = (start_pos.0 + 1, start_pos.1);
            prev_dir = (1, 0);
        }
    }

    // Left
    if start_pos.0 > 0 {
        let c = chars[start_pos.1 as usize][start_pos.0 as usize - 1];
        if c == '-' || c == 'L' || c == 'F'  {
            curr_pos = (start_pos.0 - 1, start_pos.1);
            prev_dir = (-1, 0);
        }
    }
    
    let mut lines: Vec<Line> = Vec::new();
    let mut last_change = start_pos;

    let mut main_pipe_elements = vec![vec![false; chars[0].len()]; chars.len()];

    main_pipe_elements[start_pos.1 as usize][start_pos.0 as usize] = true;

    // Make a full loop
    while curr_pos != start_pos {
        let dir = char_to_dir(chars[curr_pos.1 as usize][curr_pos.0 as usize], prev_dir);

        if prev_dir != dir {
            // Skip horizontal lines
            if last_change.1 - curr_pos.1 != 0 {
                lines.push(Line{
                    start: last_change,
                    end: curr_pos
                });
            }

            last_change = curr_pos;
        }

        main_pipe_elements[curr_pos.1 as usize][curr_pos.0 as usize] = true;

        curr_pos.0 += dir.0;
        curr_pos.1 += dir.1;

        prev_dir = dir;
    }

    // Push the last line
    lines.push(Line{
        start: last_change,
        end: start_pos
    });

    let mut enclosed = 0;
    for y in 0..chars.len() as i32 {
        for x in 0..chars[0].len() as i32 {
            if !main_pipe_elements[y as usize][x as usize] {
                if is_p_inside((x, y), &lines) {
                    enclosed += 1;
                }  
            }
        }
    }

    enclosed
}
