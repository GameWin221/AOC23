const FILE_DATA: &str = include_str!("input.txt");

#[inline]
fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.1.max(b.1) - a.1.min(b.1) + a.0.max(b.0) - a.0.min(b.0)
}

pub fn answer() -> usize {
    let lines: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| {
            line.chars().collect()
        }).collect();

    // Splitting to individual x and y components might speed it up
    let mut galaxies = Vec::new();

    let mut free_rows = Vec::new();
    let mut free_cols = Vec::new();

    free_rows.resize(lines.len(), true);
    free_cols.resize(lines[0].len(), true);

    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y][x] == '#' {
                galaxies.push((x, y));
                
                free_cols[x] = false;
                free_rows[y] = false;
            }
        }
    }

    let mut new_galaxies = galaxies.clone();

    for y in 0..free_rows.len()  {
        if free_rows[y] {
            for i in 0..galaxies.len() {
                if galaxies[i].1 > y {
                    new_galaxies[i].1 += 1;
                }
            }
        }
    }

    for x in 0..free_cols.len() {
        if free_cols[x] {
            for i in 0..galaxies.len() {
                if galaxies[i].0 > x {
                    new_galaxies[i].0 += 1;
                }
            }
        }
    }

    let mut sum = 0;
    for galaxy_a in &new_galaxies {
        for galaxy_b in &new_galaxies {
            if *galaxy_a == *galaxy_b {
                continue;
            }

            sum += distance(galaxy_a, galaxy_b);
        }
    }

    sum / 2
}
