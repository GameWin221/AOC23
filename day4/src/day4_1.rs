const FILE_DATA: &str = include_str!("input.txt");

pub fn answer() -> u32 {
    FILE_DATA.lines().into_iter().map(|line| {
        let line: Vec<&str> = line.split(":").collect();
        let numbers_text: Vec<&str> = line[1].split(" | ").collect();
        
        let winning_numbers: Vec<u32> = numbers_text[0].split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
        let picked_numbers: Vec<u32> = numbers_text[1].split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
        
        let common_numbers: Vec<u32> = picked_numbers.into_iter().filter(|n| winning_numbers.contains(n)).collect();

        2u32.pow(common_numbers.len() as u32) / 2
    }).sum()
}