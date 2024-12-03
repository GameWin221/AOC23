const FILE_DATA: &str = include_str!("input.txt");

pub fn answer() -> usize {
    let steps: Vec<Vec<&str>> = FILE_DATA
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    0
}