 const FILE_DATA: &str = include_str!("input.txt");

pub fn answer() -> usize {
    let world: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let world_height = world.len();

    let mut columns = Vec::new();
    for x in 0..world[0].len() {
        let mut column = Vec::with_capacity(world.len());

        for y in 0..world.len() {
            if world[y][x] != '.' {
                column.push((y, world[y][x] == 'O'));
            }
        }

        columns.push(column);
    }

    for col in &mut columns {
        for row in 0..col.len() {
            let (mut y, is_rock) = col[row].clone();

            if is_rock && y > 0 {
                if row > 0 {
                    y = col[row-1].0 + 1;
                } else {
                    y = 0;
                }

                col[row].0 = y;
            }
        }
    }

    let mut sum = 0;

    for col in &columns {
        for (y, is_rock) in col {
            if *is_rock {
                sum += world_height - *y;
            }
        }
    }

    sum
}