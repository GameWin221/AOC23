const FILE_DATA: &str = include_str!("input.txt");

pub fn len_to_max(len: usize) -> u32 {
    let mut n = 0;

    for i in 0..len {
        n |= 1 << i;
    }

    n
}

pub fn questionmark_positions(spring: &Vec<char>) -> Vec<usize> {
    let mut positions = Vec::new();

    for i in 0..spring.len() {
        if spring[i] == '?' {
            positions.push(i);
        }
    }

    positions
}

pub fn permute_spring(permutation: &mut Vec<char>, questionmark_positions: &Vec<usize>, num: u32) {
    for i in 0..questionmark_positions.len() {
        permutation[questionmark_positions[i]] = if (num & (1 << i)) > 0 {
            '#'
        } else {
            '.'
        }
    }
}

pub fn spring_correct(spring: &Vec<char>, counts: &Vec<u32>) -> bool {
    let mut len = 0;
    let mut idx = 0;
   
    for &c in spring {
        if c == '#' {
            len += 1;
        } else {
            if len > 0 {
                if idx >= counts.len() {
                    break;
                }
                
                if counts[idx] != len {
                    return false;
                }

                idx += 1;
            }

            len = 0;
        }
    }

    if len > 0 {
        if idx < counts.len() {
            if counts[idx] != len {
                return false;
            }
        }

        idx += 1;
    }

    if idx != counts.len() {
        return false;
    }

    true
}

pub fn answer() -> usize {
    let lines: Vec<(Vec<char>, Vec<u32>)> = FILE_DATA
        .lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();

            let springs = springs.chars().collect();
            let counts = counts
                .split_terminator(',')
                .map(|n| n.parse().unwrap())
                .collect();

            (springs, counts)
        }).collect();

    let mut sum = 0;
     
    for (spring, counts) in &lines {
        let q_pos = questionmark_positions(spring);
        let max = len_to_max(q_pos.len());

        let mut possibilities = 0;
        let mut permutation = spring.clone();

        for i in 0..=max {
            permute_spring(&mut permutation, &q_pos, i);

            if spring_correct(&permutation, counts) {
                possibilities += 1;
            }
        }

        sum += possibilities;
    }

    sum
}