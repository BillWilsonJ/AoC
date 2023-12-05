use std::fs;

fn main() {

    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");

    let mut part2_contents = String::new();
    let mut index = 0;

    let digit_array: [&str; 9] = ["one","two","three","four","five","six","seven","eight","nine"];

    while index < contents.len() {
        for j in 0..=digit_array.len() - 1 {
            if index + digit_array.len() < contents.len() && contents[index..index + digit_array[j].len()] == *digit_array[j] {
                part2_contents.push(char::from_digit(j as u32,10).unwrap());
                index += 1;
            }
        }

        part2_contents.push(contents.as_bytes()[index] as char);
        index += 1;
    }

    println!("Part 1: {}",process_input(&contents));
    println!("Part 2: {}",process_input(&part2_contents));
}

fn process_input(input: &str) -> u32 {

    let mut total = 0;
    for line in input.lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let mut first_digit_found = false;
        for char in line.chars() {
            if char.is_numeric() {
                let new_char = char.to_digit(10);
                let mut int = 0;
                match new_char {
                    Some(x) => int = x,
                    _none => ()
                }
                if first_digit_found == false{first_digit = int;first_digit_found = true;}
                last_digit = int;
            }
        }
        total = total + first_digit * 10 + last_digit;
    }

    return total;
}