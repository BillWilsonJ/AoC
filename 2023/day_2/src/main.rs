use std::fs;

fn main() {

    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");
    let mut part_1_total = 0;
    let mut part_2_total = 0;
    for line in contents.lines() {
        let parts = line.split(":");
        let mut current_game_num = 0;
        let mut invalid_game = false;
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        // Iterate over each game
        for part in parts {
            if part.contains("Game"){
                let game_num_str = part.replace("Game ", "");
                let game_num: u32 = game_num_str.trim().parse().expect("Could not get game number");
                current_game_num = game_num;
            }
            else {
                let pulls = part.split(";");
                for pull in pulls {
                    let colors = pull.split(",");
                    for color in colors {
                        if color.contains("red"){
                            let color_num_str = color.replace(" red", "");
                            let color_num: u32 = color_num_str.trim().parse().expect("Did not get a number)");
                            if color_num > 12 {
                                invalid_game = true;
                            }
                            if color_num > min_red {
                                min_red = color_num;
                            }
                        } else if color.contains("green"){
                            let color_num_str = color.replace(" green", "");
                            let color_num: u32 = color_num_str.trim().parse().expect("Did not get a number)");
                            if color_num > 13 {
                                invalid_game = true;
                            }
                            if color_num > min_green {
                                min_green = color_num;
                            }
                        } else if color.contains("blue"){
                            let color_num_str = color.replace(" blue", "");
                            let color_num: u32 = color_num_str.trim().parse().expect("Did not get a number)");
                            if color_num > 14 {
                                invalid_game = true;
                            }
                            if color_num > min_blue {
                                min_blue = color_num;
                            }
                        }
                    }
                }
            }
        }
        if invalid_game == false {
            part_1_total = part_1_total + current_game_num;
        }
        let game_power = min_red * min_green * min_blue;
        part_2_total = part_2_total + game_power;
    }
    println!("Part 1 Answer: {}", part_1_total);
    println!("Part 2 Answer: {}", part_2_total);
}