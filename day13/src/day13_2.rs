const FILE_DATA: &str = include_str!("input.txt");

fn find_smudges(a: u64, b: u64) -> u64 {
    let mut max = a.max(b);
    let mut max_bit = 0;

    while max > 0 {
        max >>= 1;
        max_bit += 1;
    }

    let mut smudges = 0;
    for i in 0..max_bit {
        if (a & (1 << i)) != (b & (1 << i)) {
            smudges += 1;
        }
    }

    smudges
}

fn find_symmetry(data: &Vec<u64>) -> Option<usize> {
    for position in 0..data.len()-1 {
        let mut width = 0;

        while position != width - 1 && position + 1 + width < data.len() {
            let a = position - width;
            let b = position + 1 + width;

            let mut smudges = 0;

            for i in 0..=width {
                smudges += find_smudges(data[position-i], data[position+1+i])
            }

            if smudges == 1 && (a == 0 || b == data.len() - 1){
                return Some(position + 1);
            }

            width += 1;
        }
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

    // Find smudges
    let mut sum = 0;

    for i in 0..gears_cols.len() {
        if let Some(symmetry) = find_symmetry(&gears_rows[i]) {
            sum += symmetry * 100;
            continue;
        }
        
        if let Some(symmetry) = find_symmetry(&gears_cols[i]) {
            sum += symmetry;
        }
    }

    sum
}