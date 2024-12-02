// day 3  part 2

use std::fs::File;
use std::io::{self, BufRead};

/*
 *
 * 
 *
 *
 *
 */

fn find_unique_house_number(dir_list: &Vec<char>) -> i32 {
    let mut house_counter = 1;
    let mut grid: Vec<(i32, i32)> = Vec::new();

    let mut current_pos_santa = (0, 0);
    let mut current_pos_robosanta = (0, 0);
    grid.push(current_pos_santa);


    for (index, &arrow) in dir_list.iter().enumerate(){
        // santa 
        if index%2 == 0 {
            current_pos_santa = match arrow {
                '>' => (current_pos_santa.0 + 1, current_pos_santa.1),
                '<' => (current_pos_santa.0 - 1, current_pos_santa.1),
                '^' => (current_pos_santa.0, current_pos_santa.1 + 1),
                'v' => (current_pos_santa.0, current_pos_santa.1 - 1),
                _ => current_pos_santa,
            };

            if !grid.contains(&current_pos_santa){
                grid.push(current_pos_santa);
                house_counter += 1;
            }

        } else {
        // robo santa
            current_pos_robosanta = match arrow {
                '>' => (current_pos_robosanta.0 + 1, current_pos_robosanta.1),
                '<' => (current_pos_robosanta.0 - 1, current_pos_robosanta.1),
                '^' => (current_pos_robosanta.0, current_pos_robosanta.1 + 1),
                'v' => (current_pos_robosanta.0, current_pos_robosanta.1 - 1),
                _ => current_pos_robosanta,
            };

            if !grid.contains(&current_pos_robosanta){
                grid.push(current_pos_robosanta);
                house_counter += 1;
            }
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
