const FILE_DATA: &str = include_str!("input.txt");

#[inline]
fn distance(a_x: usize, a_y: usize, b_x: usize, b_y: usize) -> usize {
    a_x.max(b_x) - a_x.min(b_x) + a_y.max(b_y) - a_y.min(b_y)
}

pub fn answer() -> usize {
    let lines: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| {
            line.chars().collect()
        }).collect();

    let mut galaxies_x = Vec::new();
    let mut galaxies_y = Vec::new();

    let mut free_rows = Vec::new();
    let mut free_cols = Vec::new();

    free_rows.resize(lines.len(), true);
    free_cols.resize(lines[0].len(), true);

    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y][x] == '#' {
                galaxies_x.push(x);
                galaxies_y.push(y);

                free_cols[x] = false;
                free_rows[y] = false;
            }
        }
    }

    let mut new_galaxies_x = galaxies_x.clone();
    let mut new_galaxies_y = galaxies_y.clone();

    for y in 0..free_rows.len()  {
        if free_rows[y] {
            for i in 0..galaxies_y.len() {
                if galaxies_y[i] > y {
                    new_galaxies_y[i] += 1000000 - 1;
                }
            }
        }
    }

    for x in 0..free_cols.len() {
        if free_cols[x] {
            for i in 0..galaxies_x.len() {
                if galaxies_x[i] > x {
                    new_galaxies_x[i] += 1000000 - 1;
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..new_galaxies_x.len() {
        for j in 0..new_galaxies_x.len()  {
            if i == j {
                continue;
            }

            sum += distance(new_galaxies_x[i], new_galaxies_y[i], new_galaxies_x[j], new_galaxies_y[j])
        }
    }

    sum / 2
}
