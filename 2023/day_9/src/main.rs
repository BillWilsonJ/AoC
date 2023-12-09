use std::fs;

fn main() {
    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");

    let mut starting_sequence = vec![vec![]];
    let mut new_starting_sequence = vec![];
    for line in contents.lines() {
        new_starting_sequence = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        starting_sequence.push(new_starting_sequence.clone());
        new_starting_sequence.clear();
    }

    let mut part_1_answer = 0;
    let mut part_2_answer = 0;
    for sequence in starting_sequence {
        let mut full_history = vec![vec![]];
        let mut last_row = vec![];
        let mut last_row_not_found = true;

        last_row = sequence;
        if last_row.len() != 0 {
            while last_row_not_found {
                let mut new_row = vec![];
                let mut non_zero_found = false;
                for i in 0..last_row.len()-1 {
                    let new_value: i32 = last_row[i+1] - last_row[i];
                    if new_value != 0 {
                        non_zero_found = true;
                    }
                    new_row.push(new_value);
                }
                full_history.push(last_row.clone());
                last_row = new_row;

                if non_zero_found == false {
                    last_row_not_found = false;
                    full_history.push(last_row.clone());
                }
            }
        }

        full_history.reverse();
        let full_history_pt2 = full_history.clone();
        let mut previous_end = 0;
        for row in full_history {
            if row.len() != 0 {
                previous_end = row[row.len()-1] + previous_end;
            }
        }
        part_1_answer+=previous_end;

        let mut previous_start = 0;
        for row in full_history_pt2 {
            if row.len() != 0 {
                previous_start = row[0] - previous_start;
            }
        }
        part_2_answer+=previous_start;
    }

    println!("Part 1 Answer: {}",part_1_answer);
    println!("Part 2 Answer: {}",part_2_answer);
}
