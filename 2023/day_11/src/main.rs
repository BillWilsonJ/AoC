use std::fs;
use std::cmp;

fn main() {
    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");

    let max_rows = contents.lines().next().unwrap().len();
    let max_cols = contents.lines().count();

    let mut grid = vec![];

    for line in contents.lines() {
        let grid_line = line.chars().collect::<Vec<_>>();
        grid.push(grid_line);
    }

    // find all galaxies and empty rows
    let mut galaxy_locations = vec![];
    let mut empty_rows = vec![];
    for i in 0..max_rows {
        let mut row_empty = true;
        for j in 0..max_cols {
            if grid[i][j] == '#' {
                let point = vec![j,i];
                galaxy_locations.push(point);
                row_empty = false;
            }
        }
        if row_empty == true {
            empty_rows.push(i);
        }
    }

    let mut empty_cols = vec![];
    for i in 0..max_cols {
        let mut col_empty = true;
        for j in 0..max_rows {
            if grid[j][i] == '#' {
                col_empty = false;
            }
        }
        if col_empty == true {
            empty_cols.push(i);
        }
    }

    let num_empty_galaxies = galaxy_locations.len();
    let mut part_1_answer = 0; 
    for i in 0..num_empty_galaxies { 
        for j in i..num_empty_galaxies {
            let min_x = cmp::min(galaxy_locations[i][0],galaxy_locations[j][0]);
            let min_y = cmp::min(galaxy_locations[i][1],galaxy_locations[j][1]);
            let max_x = cmp::max(galaxy_locations[i][0],galaxy_locations[j][0]);
            let max_y = cmp::max(galaxy_locations[i][1],galaxy_locations[j][1]);

            let mut empty_add = 0;
            for ecol in &empty_cols {
                if min_x <= *ecol && *ecol <= max_x {
                    empty_add+=1;
                }
            }

            for erow in &empty_rows {
                if min_y <= *erow && *erow <= max_y {
                    empty_add+=1;
                }
            }

            let distance = (max_x - min_x) + (max_y - min_y) + empty_add;
            part_1_answer += distance;
        }
    }

    let mut part_2_answer = 0; 
    for i in 0..num_empty_galaxies { 
        for j in i..num_empty_galaxies {
            let min_x = cmp::min(galaxy_locations[i][0],galaxy_locations[j][0]);
            let min_y = cmp::min(galaxy_locations[i][1],galaxy_locations[j][1]);
            let max_x = cmp::max(galaxy_locations[i][0],galaxy_locations[j][0]);
            let max_y = cmp::max(galaxy_locations[i][1],galaxy_locations[j][1]);

            let mut empty_add = 0;
            for ecol in &empty_cols {
                if min_x <= *ecol && *ecol <= max_x {
                    empty_add+=999999;
                }
            }

            for erow in &empty_rows {
                if min_y <= *erow && *erow <= max_y {
                    empty_add+=999999;
                }
            }

            let distance = (max_x - min_x) + (max_y - min_y) + empty_add;
            part_2_answer += distance;
        }
    }

    println!("Part 1 Answer: {}",part_1_answer);
    println!("Part 1 Answer: {}",part_2_answer);

}
