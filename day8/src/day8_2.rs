const FILE_DATA: &str = include_str!("input.txt");

// The extreme optimisations come from my day8_2_bruteforce.rs
// - Use u16::MAX sized array instead of a map
// - Index by u16 created from 3 characters instead of strings
// - Convert directions to bools

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut max = a.max(b);
    let mut min = a.min(b);

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

// Only last 5 bits matter for letters from A to Z in ascii
fn name_to_u16(name: &[char]) -> u16 {
    ((name[0] as u16 & 0b00011111) << 10) |
    ((name[1] as u16 & 0b00011111) << 5) |
    ((name[2] as u16 & 0b00011111))
}

#[derive(Default, Clone, Copy)]
struct Node {
    left: u16,
    right: u16,
}

pub fn answer() -> usize {
    let mut node_map = [Node::default(); 0b0111111111111111]; // 15 bit max

    let lines: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| 
            line.chars().collect()
        ).collect();

    let directions: Vec<bool> = lines[0]
        .iter()
        .map(|&c| c == 'R')
        .collect();

    let mut node_ids = Vec::new();

    lines
        .into_iter()
        .skip(2)
        .for_each(|line| {
            let name = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            if *name.last().unwrap() == 'A' {
                node_ids.push(name_to_u16(name));
            }

            node_map[name_to_u16(name) as usize] = Node {
                left: name_to_u16(left),
                right: name_to_u16(right),
            };
        });
        
    let mut first_occurences = Vec::new();

    for mut node_id in node_ids {
        let mut idx: usize = 0;

        'outer: loop {
            for &dir in &directions {
                node_id = if dir /* is right */ {
                    node_map[node_id as usize].right
                } else /* is left */ {
                    node_map[node_id as usize].left
                };

                idx += 1;

                if node_id & 0b00011111 == 'Z' as u16 & 0b00011111 {
                    break 'outer;
                }
            }
        }

        first_occurences.push(idx);
    }

    let mut count = first_occurences[0];
    for i in 0..(first_occurences.len()-1) {
        count = lcm(count, first_occurences[i+1])
    }

    count
}
