const FILE_DATA: &str = include_str!("input.txt");

fn read_card(card_index: usize, winning: &Vec<usize>) -> u32 {
    (0..winning[card_index]).into_iter().map(|i| {
        read_card(card_index + i + 1, winning)
    }).sum::<u32>() + 1u32
}

pub fn answer() -> u32 {
    let winning: Vec<usize> = FILE_DATA.lines().map(|line| {
        let line: Vec<&str> = line.split(":").collect();
        let numbers_text: Vec<&str> = line[1].split(" | ").collect();
        
        let winning_numbers: Vec<u32> = numbers_text[0].split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
        let picked_numbers: Vec<u32> = numbers_text[1].split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
        
        picked_numbers.into_iter().filter(|n| winning_numbers.contains(n)).count()
    }).collect();
    
    (0..winning.len()).into_iter().map(|i| {
        read_card(i, &winning)
    }).sum()
}