use std::collections::HashSet;

const FILE_DATA: &str = include_str!("input.txt");

pub fn answer() -> usize {
    // Two-way hashmap
    let mut connections: HashSet<String> = HashSet::new();
    
    FILE_DATA.lines().for_each(|line| {
        let name = &line[0..3];
        let data: Vec<&str> = line[5..].split(' ').collect();
        //println!("{name}: {:?}", data);

        for d in data {
            connections.insert(format!("{name}{d}"));
            connections.insert(format!("{d}{name}"));
        }
    });

    connections.remove(&String::from("hfxpzl"));
    connections.remove(&String::from("pzlhfx"));

    connections.remove(&String::from("vbvcmg"));
    connections.remove(&String::from("cmgvbv"));
    
    connections.remove(&String::from("nvdjqt"));
    connections.remove(&String::from("jqtnvd"));



    0
}