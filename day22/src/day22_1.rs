use std::collections::HashSet;

const FILE_DATA: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Block {
    start: [usize; 3],
    end: [usize; 3]
}

// Modified AABB intersection
fn blocks_intersect(a: &Block, b: &Block, a_z_offset: i32) -> bool {
    a.start[0] <= b.end[0] && a.end[0] >= b.start[0] &&
    a.start[1] <= b.end[1] && a.end[1] >= b.start[1] &&

    (a.start[2] as i32 + a_z_offset) as usize <= b.end[2] && 
    (a.end[2] as i32 + a_z_offset) as usize >= b.start[2]
}

pub fn answer() -> usize {
    let mut blocks: Vec<Block> = FILE_DATA.lines().map(|line| {
        let line: Vec<Vec<usize>> = line
            .split('~')
            .map(|frag| 
                frag.split(',').map(|n| 
                    n.parse().unwrap()
                ).collect()
            ).collect();

        Block {
            start: line[0][0..3].try_into().unwrap(),
            end: line[1][0..3].try_into().unwrap(),
        }
    }).collect();

    let mut any_moved = true;
    while any_moved {
        any_moved = false;

        for i in 0..blocks.len() {
            if blocks[i].start[2] <= 1 {
                continue;
            }

            let mut can_be_moved = true;

            for j in 0..blocks.len() {
                if i == j {
                    continue;
                }
    
                // blocks[i] is one block above blocks[j]
                if blocks[i].start[2] == blocks[j].end[2] + 1 {
                    if blocks_intersect(&blocks[i], &blocks[j], -1) {
                        can_be_moved = false;
                        break;
                    }
                }
            }

            if can_be_moved {
                blocks[i].start[2] -= 1;
                blocks[i].end[2] -= 1;
                any_moved = true;
            }
        }
    }

    let mut supported_by = vec![Vec::new(); blocks.len()];

    // Check which blocks support block[i]
    for i in 0..blocks.len() {
        for j in 0..blocks.len() {
            if i == j {
                continue;
            }

            // blocks[i] is one block above blocks[j]
            if blocks[i].start[2] == blocks[j].end[2] + 1 {
                if blocks_intersect(&blocks[i], &blocks[j], -1) {
                    supported_by[i].push(j);
                }
            }
        }
    }
    
    //for (i, s) in supported_by.iter().enumerate() {
    //    println!("{i} is supported by: {:?}", s);
    //}

    // Keep only the indices that didn't occur at all or never occured alone 

    let mut indices: HashSet<usize> = (0..blocks.len()).collect();

    // Remove all the indices that occured alone
    for supports in &supported_by {
        if supports.len() == 1 {
            indices.remove(&supports[0]);
            //println!("Discarding {} as it is the only support for some block!", supports[0]);
        }
    }

    //println!("We can remove only: {:?}", indices);

    indices.len()
}