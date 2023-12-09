use std::fs;
use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");

    let instructions = contents
        .lines()
        .next()
        .unwrap()
        .chars()
        .cycle();

    let inst2 = instructions.clone();

    let mut nodes = HashMap::new();
    let contents = contents.replace(['(', ')', ',', '='], "");
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        if line.chars().count() > 100 {
            continue;
        } 
        let new_string = line.split_whitespace().collect::<Vec<_>>();

        nodes.insert(new_string[0],(new_string[1],new_string[2]));
    }

    let mut location = "AAA";
    let mut part_1_answer = 1;
    for direction in instructions {
        if direction == 'L' {
            location = nodes.get(location).unwrap().0;
        } else {
            location = nodes.get(location).unwrap().1;
        }
        if location == "ZZZ" {
            break;
        }
        part_1_answer+=1;
    }
    println!("Part 1 Answer: {}",part_1_answer);

    let starting_nodes = nodes
        .keys()
        .filter(|x| x.ends_with('A'));

    let mut steps_collection = vec![];
    for starting_node in starting_nodes {
        let mut location = *starting_node;
        let mut steps = 1;
        let instructions_2 = inst2.clone();
        for direction in instructions_2 {
            if direction == 'L' {
                location = nodes.get(location).unwrap().0;
            } else {
                location = nodes.get(location).unwrap().1;
            }
            if location.ends_with('Z') {
                break;
            }
            steps+=1;
        }
        steps_collection.push(steps);
        println!("Starting Node: {} - {}",*starting_node, steps);
    }

    let mut least_common_multiple: i64 = steps_collection[0];

    for i in 0..steps_collection.len() -1 {
        least_common_multiple = lcm(least_common_multiple,steps_collection[i+1]);
        println!("{}",least_common_multiple);
    }

    println!("Part 2 Answer: {}",least_common_multiple);

}

