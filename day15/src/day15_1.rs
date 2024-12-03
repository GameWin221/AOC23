const FILE_DATA: &str = include_str!("input.txt");

pub fn hash(data: &str) -> usize {
    let mut h = 0;

    for c in data.chars() {
        h += c as usize;
        h *= 17;
        h %= 256;
    }

    h
}

pub fn answer() -> usize {
    FILE_DATA.split(',').map(|code| hash(code)).sum()
}