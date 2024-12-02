use std::fs::File;
use std::io::{self, BufRead};
use std::usize;

/*
 Now, given the same instructions, find the position of the first character that causes him to
 enter the basement (floor -1). The first character in the instructions has position 1, the second
 character has position 2, and so on.

For example:
    ) causes him to enter the basement at character position 1.
    ()()) causes him to enter the basement at character position 5.
    What is the position of the character that causes Santa to first enter the basement?*

*/

fn find_idx_of_negative_floor(paranthesis_list: &Vec<char>) -> usize {
    let mut current_floor = 0;
    for (i, &par) in paranthesis_list.iter().enumerate(){
        if par == '('{
            current_floor += 1;
        }else {
            current_floor -= 1;
        }
        
        if current_floor == -1 {
            return i+1;
        }
    }
    0
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
            let idx = find_idx_of_negative_floor(&list_of_paranthesis);
            println!("first negative floor idx: {}", idx);
        }
        Err(e) => {
            eprintln!("Error reading the first line: {}", e);
        }
    }
}
