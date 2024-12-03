const FILE_DATA: &str = include_str!("input.txt");

fn generate_next_row(row: &Vec<i32>) -> Vec<i32> {
    row.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn generate_history(row: Vec<i32>) -> Vec<Vec<i32>> {
    let mut history = vec![row];

    while history.last().unwrap().iter().sum::<i32>() != 0 {
        history.push(generate_next_row(history.last().unwrap()))
    }

    history
}

fn generate_new_value(history: Vec<Vec<i32>>) -> i32 {
    let mut new_values = vec![0];

    for i in (0..history.len()-1).rev() {
        let bottom_val = new_values.last().unwrap().clone();
        let left_val = history[i].last().unwrap().clone();
        
        new_values.push(left_val + bottom_val);
    }

    new_values.last().unwrap().clone()
}

pub fn answer() -> usize {
    let values: Vec<Vec<i32>> = FILE_DATA
        .lines()
        .into_iter()
        .map(|line| {
            line
                .split_ascii_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        }).collect();

    values.iter().map(|line| {
        generate_new_value(generate_history(line.clone()))
    }).sum::<i32>() as usize
}
