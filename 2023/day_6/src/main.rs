use std::fs;

fn main() {
    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");

    // part 1
    let time = contents
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>();

    let distance = contents
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>();
         
    let mut number_of_ways_to_win = vec![];
    for t in 0..time.len() {
        let record_distance = distance[t].parse::<usize>().unwrap();
        let current_time = time[t].parse::<usize>().unwrap();
        let mut ways_to_win = 0;
        for i in 1..current_time {
            let current_distance = i * (current_time - i);
            if current_distance > record_distance {
                ways_to_win+=1;
            }
        }
        number_of_ways_to_win.push(ways_to_win);
    }

    let time_pt2= contents
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>().unwrap();

    let distance_pt2= contents
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>().unwrap();

    println!("{}",time_pt2);
    println!("{}",distance_pt2);

    let mut part_2_answer = 0;
    for i in 1..time_pt2 {
        let current_distance = i * (time_pt2 - i);
        if current_distance > distance_pt2 {
            part_2_answer+=1;
        }
    }

    let part_1_answer: usize = number_of_ways_to_win.iter().product();
    println!("Part 1 Answer: {}",part_1_answer);
    println!("Part 1 Answer: {}",part_2_answer);
}

