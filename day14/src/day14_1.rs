const FILE_DATA: &str = include_str!("input.txt");

pub fn answer() -> usize {
    let mut world: Vec<Vec<char>> = FILE_DATA
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut prev_world = world.clone();

    loop {
        let mut world_identical = true;

        for y in 0..world.len() {
            for x in 0..world[0].len() {
                if world[y][x] == 'O' {
                    if y > 0 {
                        if world[y-1][x] == '.' {
                            world[y-1][x] = 'O';
                            world[y][x] = '.';
                        }
                    }
                }
            }

            if world[y] != prev_world[y] {
                world_identical = false;
            }
        }

        if world_identical {
            break;
        }

        prev_world = world.clone();
    }

    let mut sum = 0;

    for y in 0..world.len() {
        for &c in &world[y] {
            if c == 'O' {
                sum += world.len() - y;
            }
        }
    }

    sum
}