const FILE_DATA: &str = include_str!("input.txt");

struct MapRange {
    pub src_start: usize,
    pub src_end: usize,
    pub dst_start: usize,
}

fn get_map_from_lines(lines: &Vec<Vec<usize>>) -> Vec<MapRange> {
    lines.iter().map(|line| {
        MapRange {
            src_start: line[1],
            src_end: line[1] + line[2] - 1,
            dst_start: line[0],
        }
    }).collect()
}

fn map_src_to_dst(map: &Vec<MapRange>, src: usize) -> usize {
    for m in map {
        if src < m.src_start {
            continue;
        }

        if src > m.src_end {
            continue;
        }

        return m.dst_start + (src - m.src_start);
    }
    
    src
}

pub fn answer() -> usize {
    let map_data: Vec<Vec<Vec<usize>>> = FILE_DATA
        .split("\r\n\r\n")
        .map(|m| {
            m.split("\r\n")
            .skip(1)
            .map(|ln| 
                ln.split_ascii_whitespace()
                .map(|n| 
                    n.parse::<usize>().unwrap()
                ).collect()
            ).collect()
        })
        .skip(1)
        .collect();

    let seeds: Vec<usize> = FILE_DATA
        .split_once("\r\n").unwrap().0
        .split_at(7).1
        .split_ascii_whitespace()
        .map(|word| 
            word.parse().unwrap()
        ).collect();

    // Performance note: ordered vectors might be faster
    // loop in map_src_to_dst skips if src < m.src_start so it would skip all values that are too small

    let seed_to_soil = get_map_from_lines(&map_data[0]);
    let soil_to_fertilizer = get_map_from_lines(&map_data[1]);
    let fertiziler_to_water = get_map_from_lines(&map_data[2]);
    let water_to_light = get_map_from_lines(&map_data[3]);
    let light_to_temperature = get_map_from_lines(&map_data[4]);
    let temperature_to_humidity = get_map_from_lines(&map_data[5]);
    let humidity_to_location = get_map_from_lines(&map_data[6]);
    
    seeds.iter().map(|&seed| {
        map_src_to_dst(&humidity_to_location, 
            map_src_to_dst(&temperature_to_humidity, 
                map_src_to_dst(&light_to_temperature, 
                    map_src_to_dst(&water_to_light, 
                        map_src_to_dst(&fertiziler_to_water, 
                            map_src_to_dst(&soil_to_fertilizer, 
                                map_src_to_dst(&seed_to_soil, 
                                    seed
                                )
                            )
                        )
                    ) 
                )
            )
        )
    }).min().unwrap()
}
