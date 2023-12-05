use std::fs;
type VecOfArrays<T, const N: usize> = Vec<[T; N]>;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");

    let mut total = 0;
    let mut symbol_locations: VecOfArrays<i32, 2> = vec![];
    let mut row_index = 0;

    for line in contents.lines()
    {
        let mut col_index = 0;
        for char in line.chars()
        {
            if !char.is_digit(10) && char != '.' {
                let row = row_index;
                let col = col_index;
                let location = [row,col];
                symbol_locations.push(location);
            }
            col_index+=1;
        }
        row_index+=1;
    }

    row_index = 0;

    // parse numbers and check if found symbol is next to it
    let mut num_first_index = 0;
    let mut num_last_index = 0;
    let mut num_found = false;
    for line in contents.lines() {
        let mut num: String = "".to_owned();
        let mut col_index = 0;
        let line_length: i32 = line.len() as i32;
        for char in line.chars() {
            if num_found == false {
                if char.is_digit(10) {
                    num_first_index = col_index;
                    num_last_index = col_index;
                    num_found = true;
                    num.push(char);
                }
            }
            else {
                if char.is_digit(10) {
                    num_last_index = col_index;
                    num.push(char);
                    if col_index == line_length - 1 {
                        let num_int: i32 = num.parse().expect(&num);
                        for symbol_location in &symbol_locations {
                            if symbol_location[0] >= (row_index - 1)  && symbol_location[0] <= (row_index + 1){
                                if  symbol_location[1] >= (num_first_index - 1) && symbol_location[1] <= (num_last_index + 1){
                                    println!{"Num found {}",num};
                                    total = total + num_int;
                                    break;
                                }
                            }
                        }
                        num_found = false;
                        num.clear();
                        num_first_index = 0;
                        num_last_index = 0;
                    }
                }
                else {
                    num_found = false;
                    let num_int: i32 = num.parse().expect(&num);
                    for symbol_location in &symbol_locations {
                        if symbol_location[0] >= (row_index - 1)  && symbol_location[0] <= (row_index + 1){
                            if  symbol_location[1] >= (num_first_index - 1) && symbol_location[1] <= (num_last_index + 1){
                                println!{"Num found {}",num};
                                total = total + num_int;
                                break;
                            }
                        }
                    }
                    num.clear();
                    num_first_index = 0;
                    num_last_index = 0;
                }
            }
            col_index+=1;
        }
        row_index+=1;
    }

    println!("Part 1 Answer: {}", total);
}

fn part_2() {
    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");

    let mut total = 0;
    let mut num_locations: VecOfArrays<i32, 3> = vec![];
    let mut numbers: Vec<i32> = Vec::new();
    let mut row_index = 0;

    // parse numbers and check if found symbol is next to it
    let mut num_first_index = 0;
    let mut num_last_index = 0;
    let mut num_found = false;
    for line in contents.lines() {
        let mut num: String = "".to_owned();
        let mut col_index = 0;
        let line_length: i32 = line.len() as i32;
        for char in line.chars() {
            if num_found == false {
                if char.is_digit(10) {
                    num_first_index = col_index;
                    num_last_index = col_index;
                    num_found = true;
                    num.push(char);
                }
            }
            else {
                if char.is_digit(10) {
                    num_last_index = col_index;
                    num.push(char);
                    if col_index == line_length - 1 {
                        let location = [row_index,num_first_index,num_last_index];
                        num_locations.push(location);
                        let num_int: i32 = num.parse().expect(&num);
                        numbers.push(num_int);
                        num_found = false;
                        num.clear();
                        num_first_index = 0;
                        num_last_index = 0;
                    }
                }
                else {
                    let location = [row_index,num_first_index,num_last_index];
                    num_locations.push(location);
                    let num_int: i32 = num.parse().expect(&num);
                    numbers.push(num_int);
                    num_found = false;
                    num.clear();
                    num_first_index = 0;
                    num_last_index = 0;
                }
            }
            col_index+=1;
        }
        row_index+=1;
    }

    row_index = 0;

    for line in contents.lines()
    {
        let mut col_index = 0;
        for char in line.chars()
        {
            if !char.is_digit(10) && char == '*' {
                let mut num_index = 0;
                let mut first_adj_found = false;
                let mut first_num = 0;
                for num_location in &num_locations {
                    if num_location[0] >= (row_index - 1)  && num_location[0] <= (row_index + 1){
                        if  (num_location[1] - 1) <= col_index && num_location[2] + 1 >= col_index {
                            if first_adj_found == false {
                                first_adj_found = true;
                                first_num = numbers[num_index];
                            }
                            else {
                                let ratio = first_num * numbers[num_index];
                                total = total + ratio;
                            }
                        }
                    }
                    num_index+=1;
                }
            }
            col_index+=1;
        }
        row_index+=1;
    }

    println!("Part 2 Answer: {}", total);
}