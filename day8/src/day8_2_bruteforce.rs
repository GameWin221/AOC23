const FILE_DATA: &str = include_str!("input.txt");

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
    let mut node_map = [Node::default(); u16::MAX as usize];

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

    let mut count = 1;

    loop {
        'main_loop: for &dir in &directions {
            for node_id in &mut node_ids {
                *node_id = if dir /* is right */ {
                    node_map[*node_id as usize].right
                } else /* is left */ {
                    node_map[*node_id as usize].left
                };

                if *node_id & 0b00011111 != 'Z' as u16 & 0b00011111 {
                    count += 1;
                    continue 'main_loop;
                }
            }

            return count;
        }

        if count % 100000 == 0 {
            println!("{count}")
        }
    }
}