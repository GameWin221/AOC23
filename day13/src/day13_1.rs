const FILE_DATA: &str = include_str!("input.txt");

fn find_symmetry(data: &Vec<u64>) -> Option<usize> {
    // Find positions of values repeated twice one after another
    let mut positions = Vec::new();

    for i in 0..data.len()-1 {
        if data[i] == data[i+1] {
            positions.push(i);
        }
    }

    'posloop: for &position in &positions {
        let mut width = 1;

        // Go left/right or up/down until we hit index 0 or index data.len()
        // If the symmetry is broken along the way, the symmetry is incorrect and try the next one

        while position != width - 1 && position + 1 + width < data.len() {
            let a = position - width;
            let b = position + 1 + width;

            if data[a] != data[b] {
                continue 'posloop;
            }

            width += 1;
        }

        return Some(position + 1);
    }

    None
}

pub fn answer() -> usize {
    let gears: Vec<Vec<Vec<bool>>> = FILE_DATA
        .split("\r\n\r\n")
        .map(|gear| {
            gear
                .lines()
                .map(|line| line.chars().map(|c| c == '#')
                .collect()
            ).collect()
        }).collect();

    let mut gears_rows: Vec<Vec<u64>> = Vec::new();
    let mut gears_cols: Vec<Vec<u64>> = Vec::new();

    for gear in &gears {
        let mut row_data = Vec::with_capacity(gear.len());
        let mut col_data = Vec::with_capacity(gear[0].len());

        // Split rows
        for row in gear {
            let mut data = 0;

            for &c in row {
                if c {
                    data |= 1;
                }

                data <<= 1;
            }
            
            row_data.push(data);
        }

        // Split columns
        for x in 0..gear[0].len() {
            let mut data = 0;

            for y in 0..gear.len() {
                if gear[y][x] {
                    data |= 1;
                }

                data <<= 1;
            }

            col_data.push(data);
        }

        gears_rows.push(row_data);
        gears_cols.push(col_data);
    }

    let mut sum = 0;

    for gear_rows in &gears_rows {
        if let Some(symmetry) = find_symmetry(gear_rows) {
            sum += symmetry * 100;
        }
    }

    for gear_cols in &gears_cols {
        if let Some(symmetry) = find_symmetry(gear_cols) {
            sum += symmetry;
        }
    }

    sum
}