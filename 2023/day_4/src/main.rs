use std::fs;

fn main() {
    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");

    let mut part_1_total = 0;
    let mut part_2_total = 0;

    let num_lines = contents.lines().count();
    let mut copies: Vec<i32> = vec![0;num_lines];
    let mut index = 0;

    for line in contents.lines() {

        // split out card number
        let parts = line.split(":").collect::<Vec<&str>>();
        let number_list = parts[1];

        let split_numbers_list = number_list.split("|").collect::<Vec<&str>>();
        let winning_numbers = split_numbers_list[0].trim().split_whitespace();
        let mut num_matches_found: i32 = 0;
        copies[index] = copies[index] + 1;

        for winning_num in winning_numbers {
            let card_numbers = split_numbers_list[1].trim().split_whitespace();
            for card_num in card_numbers {
                if winning_num == card_num {
                    num_matches_found+=1;
                    break;
                }
            }
        }
        
        if num_matches_found > 0{
            let base: i32 = 2;
            let pow: u32 = (num_matches_found - 1).try_into().unwrap();
            let add = base.pow(pow);
            part_1_total = part_1_total + add;

            for j in 1..=num_matches_found {
                let intermediate_index = index + j.wrapping_abs() as u32 as usize;
                copies[intermediate_index] += copies[index];
            }
        }

        println!("{}",num_matches_found);
        part_2_total = part_2_total + copies[index];
        index+=1;
    }

    println!("Part 1 Answer: {}", part_1_total);
    println!("Part 2 Answer: {}", part_2_total);
}
