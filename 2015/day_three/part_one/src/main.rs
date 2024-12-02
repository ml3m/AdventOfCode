// day 3 

use std::fs::File;
use std::io::{self, BufRead};

/*
 *   use a grid.
 *   store santa visited houses cordinates in a set
 *   starts at (0, 0)
 *
 * if a house is visited twice or more it won't be added in the set cause we check each new house
 * to the existing ones in the set.
 * then count all the elements in the len(set) = total houses
 *
 */

fn find_unique_house_number(dir_list: &Vec<char>) -> i32 {
    let mut house_counter = 1;
    let mut grid: Vec<(i32, i32)> = Vec::new();

    let mut current_pos = (0, 0);
    grid.push(current_pos);

    for &arrow in dir_list.iter(){

        current_pos = match arrow {
            '>' => (current_pos.0 + 1, current_pos.1),
            '<' => (current_pos.0 - 1, current_pos.1),
            '^' => (current_pos.0, current_pos.1 + 1),
            'v' => (current_pos.0, current_pos.1 - 1),
            _ => current_pos,
        };

        if !grid.contains(&current_pos){
            grid.push(current_pos);
            house_counter += 1;
        }
    }
    house_counter
}


fn main() {
    let filename = "input"; // Replace with your file path
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    let mut reader = io::BufReader::new(file);

    // Read the first line only
    let mut first_line = String::new();
    match reader.read_line(&mut first_line) {
        Ok(_) => {
            // Remove any trailing newline characters
            first_line = first_line.trim().to_string();

            // Convert the first line into a vector of characters
            let list_of_dir: Vec<char> = first_line.chars().collect();
            let houses = find_unique_house_number(&list_of_dir);
            println!("unique houses visited are: {}", houses);
        }
        Err(e) => {
            eprintln!("Error reading the first line: {}", e);
        }
    }
}
