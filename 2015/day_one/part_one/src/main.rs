use std::fs::File;
use std::io::{self, BufRead};

/*
 *An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means
 he should go down one floor.
 *
 */

fn find_destination_floor(paranthesis_list: &Vec<char>) -> i32 {
    let mut current_floor = 0;
    for &par in paranthesis_list{
        if par == '('{
            current_floor += 1;
        }else {
            current_floor -= 1;
        }
    }
    current_floor
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
            let list_of_paranthesis: Vec<char> = first_line.chars().collect();
            let floor = find_destination_floor(&list_of_paranthesis);
            println!("floor is {}", floor);
        }
        Err(e) => {
            eprintln!("Error reading the first line: {}", e);
        }
    }
}
