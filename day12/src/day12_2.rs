const FILE_DATA: &str = include_str!("input.txt");

fn solve_record_bottom(springs: &Vec<char>, groups: &Vec<usize>) -> usize {
    let mut dp: Vec<Vec<usize>> = vec![vec![0; groups.len()]; springs.len() + groups[groups.len() - 1] + 1];
    let mut min_j = 0;
    'i: for i in 0..springs.len() {
        // Manage memory
        if i > 0 {
            dp[i - 1].clear();
        }
        for j in 0..groups.len() {
            // If first group is at a broken spring, we skip it from now on
            // The first group decides all the valid starting positions and its placement can never
            // be past the first #.
            let cur_char = springs[i];
            if j < min_j {
                continue;
            }
            if cur_char == '#' && j == 0 {
                min_j = 1;
            }
            // Skip periods
            if cur_char == '.' {
                continue 'i;
            }
            // If group can't be placed here according to previous logic, continue
            if j > 0 && dp[i][j - 1] == 0 {
                continue;
            }
            // If remaining groups don't fit in remaining springs, continue
            if groups[j..].iter().sum::<usize>() + groups[j..].len() - 1 > springs[i..].len() {
                continue;
            }
            // if we are at last group and there are springs remaining, group isn't valid
            if (j == groups.len() - 1) && springs[i + groups[j]..].iter().any(|&c| c == '#') {
                continue;
            }
            // Check if current group is valid
            let max_idx = springs.len().min(i + groups[j]);
            let end_reached = max_idx == springs.len();
            let subsequent_character = springs.get(max_idx).cloned().unwrap_or(' ');
            let group_valid = springs[i..i + groups[j]].iter().all(|&x| x == '?' || x == '#') && (end_reached || subsequent_character != '#');
            if !group_valid {
                continue;
            }

            // If our current group is valid, we add the amount of ways we can reach the next
            // starting location, to all indices up to and including a broken spring.
            // If there are no broken springs, that means all remaining positions are valid for the
            // next group. During next iterations, we can check if the next group fits there.
            // If it does, we can do the same thing and add the amount of ways we could get to the starting index for the group after that,
            // and so forth.
            // --------------------------------------------------
            //             01234567
            // Scenario 1: ??.??.?? 1,1,1
            // --------------------------------------------------
            //
            //       dp[0]      dp[1]      dp[2]      dp[3]      dp[4]      dp[5]      dp[6]      dp[7]      dp[8]       dp[9]     ]
            //     [ [0, 0, 0], [0, 0, 0], [1, 0, 0], [2, 0, 0], [2, 0, 0], [2, 2, 0], [2, 4, 0], [2, 4, 0], [2, 4, 4],  [2, 4, 8] ]
            // --------------------------------------------------
            //             0123456
            // Scenario 2: ??.#.?? 1,1,1
            // --------------------------------------------------
            //
            //       dp[0]      dp[1]      dp[2]      dp[3]      dp[4]      dp[5]      dp[6]      dp[7]      dp[8]     ]
            //     [ [0, 0, 0], [0, 0, 0], [1, 0, 0], [2, 0, 0], [0, 0, 0], [0, 2, 0], [0, 2, 0], [0, 2, 2], [0, 2, 4] ]
            let next_start_idx = (springs.len()).min(i + groups[j] + 1);
            let next_broken_idx = match springs[next_start_idx..].iter().position(|&x| x == '#') {
                Some(n) => next_start_idx + n,
                None => dp.len() - 1,
            };
            for k in next_start_idx..=next_broken_idx {
                if j > 0 {
                    dp[k][j] += dp[i][j - 1];
                } else {
                    dp[k][j] += 1;
                }
            }
        }
    }
    dp[dp.len() - 1][dp[dp.len() - 1].len() - 1]
}

pub fn answer() -> usize {
    let lines: Vec<(Vec<char>, Vec<usize>)> = FILE_DATA
        .lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();

            let springs: Vec<char> = springs.chars().collect();
            let counts: Vec<usize> = counts
                .split_terminator(',')
                .map(|n| n.parse().unwrap())
                .collect();

            let mut ext_springs = Vec::with_capacity(springs.len() * 5);
            let mut ext_counts = Vec::with_capacity(counts.len() * 5);

            for i in 0..5 {
                for &s in &springs {
                    ext_springs.push(s);
                }

                for &c in &counts {
                    ext_counts.push(c);
                }

                if i != 4 {
                    ext_springs.push('?');
                }
            }

            println!("{:?}, {:?}", ext_springs, ext_counts);

            (ext_springs, ext_counts)
        }).collect();

    let mut sum = 0;
        
    for (spring, counts) in &lines {
        sum += solve_record_bottom(spring, counts);
    }

    sum
}