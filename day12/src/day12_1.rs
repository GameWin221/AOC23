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

pub fn permute_spring(original: &Vec<char>, questionmark_positions: &Vec<usize>, num: u32) -> Vec<char> {
    let mut new_spring = original.clone();

    for i in 0..questionmark_positions.len() {
        new_spring[questionmark_positions[i]] = if (num & (1 << i)) > 0 {
            '#'
        } else {
            '.'
        }
    }

    new_spring
}

pub fn spring_correct(spring: &Vec<char>, counts: &Vec<u32>) -> bool {
    let mut v = Vec::with_capacity(counts.len());

    let mut len = 0;
    for &c in spring {
        if c == '#' {
            len += 1;
        } else {
            if len > 0 {
                v.push(len);
            }

            len = 0;
        }
    }

    if len > 0 {
        v.push(len);
    }

    if v.len() != counts.len() {
        return false;
    }

    for i in 0..v.len() {
        if v[i] != counts[i] {
            return false;
        }
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
        for i in 0..=max {
            let perm = permute_spring(&spring, &q_pos, i);

            if spring_correct(&perm, counts) {
                possibilities += 1;
            }
        }

        sum += possibilities;
    }
    

    sum
}