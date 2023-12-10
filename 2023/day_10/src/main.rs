use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

    let mut tile_loop = vec![]; 
    for i in 0..max_cols {
        for j in 0..max_rows {
            if grid[i][j] == 'S' {
                let loop_step = LoopStep {
                    position: Position{x:j,y:i},
                    index: 0,
                    tile: grid[i][j],
                    origin: Direction::Unknown
                };

                tile_loop.push(loop_step);
                break;
            }
        }
    }

    let mut next_tile = '?';
    let mut index: usize = 0;
    while next_tile != 'S' {
        // Find next tile
        let base_piece_x = tile_loop[index].position.x;
        let base_piece_y = tile_loop[index].position.y;
        for direction in Direction::iter() {
            // If the direction is the origin, dont look for it
            if tile_loop[index].origin == direction {
                continue
            }
            if allowable_direction(direction.clone(),next_tile.clone()) == false {
                continue
            }
            let (next_tile_x,next_tile_y) = find_next_tile_index(direction.clone(),base_piece_x,base_piece_y);
            if new_direction_found(direction.clone(),grid[next_tile_y][next_tile_x]) {
                index+=1;
                let loop_step = LoopStep {
                    position: Position{x:next_tile_x,y:next_tile_y},
                    index: index,
                    tile: grid[next_tile_y][next_tile_x],
                    origin: origin_direction(direction.clone())
                };

                tile_loop.push(loop_step);
                next_tile = grid[next_tile_y][next_tile_x];
                break;
            }
        }
    }

    let part_1_answer = index / 2; 
    println!("Part 1 Answer: {}",part_1_answer);

    // part two
    let mut part_2_answer = 0; 

    let mut loop_positions = vec![];
    for tile in tile_loop {
        loop_positions.push(tile.position);
    }

    for i in 0..max_cols {
        let mut winding_count = 0;
        for j in 0..max_rows {
            // Is this a tile on the loop
            let current_position = Position{x:j,y:i};
            let position_below = Position{x:j,y:i+1};
            if loop_positions.contains(&current_position) {
                let current_position_index = loop_positions.iter().position(|r| *r == current_position).unwrap() as i32;
                if loop_positions.contains(&position_below) {
                    let position_below_index = loop_positions.iter().position(|r| *r == position_below).unwrap() as i32;
                    if current_position_index == (position_below_index + 1) {
                        winding_count+=1;
                    }
                    else if current_position_index == (position_below_index - 1){
                        winding_count-=1;
                    }
                }
                continue;
            }
            else if winding_count != 0 {
                //found a tile inside
                part_2_answer+=1;
            }
        }
    }

    println!("Part 2 Answer: {}",part_2_answer);
}

#[derive(Debug, Default)]
struct LoopStep {
    position: Position,
    index: usize,
    tile: char,
    origin: Direction
}

#[derive(Debug, Default, PartialEq)]
struct Position {
    x: usize,
    y: usize
}

#[derive(Debug, Default, EnumIter, PartialEq, Clone)]
enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 4,
    #[default] Unknown = 5
}

fn new_direction_found(dir: Direction,y:char) -> bool {
    let mut found = false;
    if dir == Direction::North {
        match y {
            '|' | '7' | 'F' | 'S' => {found=true;},
            _ => {found=false;},
        }
    }
    else if dir == Direction::South {
        match y {
            '|' | 'L' | 'J' | 'S' => {found=true;},
            _ => {found=false;},
        }
    }
    else if dir == Direction::East {
        match y {
            '-' | 'J' | '7' | 'S'=> {found=true;},
            _ => {found=false;},
        }
    }
    else if dir == Direction::West {
        match y {
            '-' | 'L' | 'F'| 'S' => {found=true;},
            _ => {found=false;},
        }
    }
    found
}

fn allowable_direction(dir: Direction,y:char) -> bool {
    let mut found = false;
    if dir == Direction::North {
        match y {
            '|' | 'L' | 'J' | '?' => {found=true;},
            _ => {found=false;},
        }
    }
    else if dir == Direction::South {
        match y {
            '|' | '7' | 'F' | '?' => {found=true;},
            _ => {found=false;},
        }
    }
    else if dir == Direction::East {
        match y {
            '-' | 'L' | 'F' | '?' => {found=true;},
            _ => {found=false;},
        }
    }
    else if dir == Direction::West {
        match y {
            '-' | 'J' | '7' | '?' => {found=true;},
            _ => {found=false;},
        }
    }
    found
}

fn find_next_tile_index(dir: Direction,x: usize, y: usize) -> (usize,usize) {
    match dir {
        Direction::North => {(x,y-1)},
        Direction::South => {(x,y+1)},
        Direction::East => {(x+1,y)},
        Direction::West => {(x-1,y)},
        _ => {(x,y)}
    }
}

fn origin_direction(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
        _ => Direction::Unknown
    }
}