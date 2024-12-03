const FILE_DATA: &str = include_str!("input.txt");

// The extreme optimisations come from my day8_2_bruteforce.rs
// - Use u16::MAX sized array instead of a map
// - Index by u16 created from 3 characters instead of strings
// - Convert directions to bools

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
    
    lines
        .into_iter()
        .skip(2)
        .for_each(|line| {
            let name = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            node_map[name_to_u16(name) as usize] = Node {
                left: name_to_u16(left),
                right: name_to_u16(right),
            };
        });
        

    let start_value = name_to_u16(&['A', 'A', 'A']);
    let end_value = name_to_u16(&['Z', 'Z', 'Z']);

    let mut node = node_map[start_value as usize];
    let mut count = 1;

    loop {
        for &dir in &directions {
            let node_id = if dir /* is right */ {
                node.right
            } else /* is left */  {
                node.left
            };
    
            if node_id == end_value {
                return count;
            }
    
            node = node_map[node_id as usize];
    
            count += 1;
        }
    }
}
