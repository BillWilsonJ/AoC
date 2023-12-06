use std::fs;
use std::time::SystemTime;

fn main() {
    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");

    let start_time = SystemTime::now();

    let seeds_part1 = contents
                    .lines()
                    .next()
                    .unwrap()
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split_whitespace();

    let seeds_values = seeds_part1.clone();

    let mut mappings = vec![];
    let mut new_map = vec![];

    // create mappings
    for line in contents.lines() {
        if line.is_empty() || line.contains("seeds") {
            // do nothing
        } else if line.contains("map") {
            // map is complete
            mappings.push(new_map.clone());
            new_map.clear();
        }
        else {
            // add map line to map
            let map_line_string = line.split_whitespace();
            let mut map_line = vec![];
            for num in map_line_string {
                map_line.push(num.parse::<i64>().unwrap());
            }
            new_map.push(map_line);
        }
    }
    mappings.push(new_map.clone());

    let mut part_1_answer = 0;
    for seed in seeds_part1 {
        let mut intermediate = seed.parse::<i64>().unwrap();
        for map in &mappings {
            for map_line in map {
                if map_line[1] <= intermediate && intermediate <= map_line[1] + map_line[2] {
                    intermediate = map_line[0] + intermediate - map_line[1];
                    break;
                }
            }
        }
        if intermediate < part_1_answer || part_1_answer == 0 {
            part_1_answer = intermediate;
        }
    }

    let mut part_2_answer = 0;
    let mut range_bottom = 0;

    for seed in seeds_values {
        if range_bottom == 0 {
            range_bottom = seed.parse::<i64>().unwrap();
        }
        else {
            for n in range_bottom..range_bottom + seed.parse::<i64>().unwrap() {
                let mut intermediate = n;
                for map in &mappings {
                    for map_line in map {
                        if map_line[1] <= intermediate && intermediate <= map_line[1] + map_line[2] {
                            intermediate = map_line[0] + intermediate - map_line[1];
                            break;
                        }
                    }
                }
                if intermediate < part_2_answer || part_2_answer == 0 {
                    part_2_answer = intermediate;
                }
            }
            range_bottom = 0;
        }
    }

    match start_time.elapsed() {
        Ok(elapsed) => {
            println!("Time Elapsed: {}", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error");
        }
    }

    println!("Part 1 Answer: {}",part_1_answer);
    println!("Part 2 Answer: {}",part_2_answer);
}
