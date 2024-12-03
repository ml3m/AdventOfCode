// day 3 part 1

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};



fn parse_with_re(re: Regex, dirty_input: &str) -> Vec<&str> {

    let mut clean_input_muls = Vec::new();

    for mul in re.find_iter(dirty_input) {
        println!("mul found: {:?}", mul.as_str());
        clean_input_muls.push(mul.as_str());
    }
    clean_input_muls
}

fn calculate_mul(clean_input: & Vec<&str>) -> i32{
    let mut sum_of_prod = 0;

    for mul in clean_input{
        let numbers: Vec<i32> = mul
            .split(|c: char| !c.is_numeric())
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();
        let prod: i32 = numbers.iter().product();
        sum_of_prod += prod;
    }
    sum_of_prod
}

fn main() {

    let pattern = r"mul\(\s*\d+\s*,\s*\d+\s*\)";

    let re = Regex::new(pattern).unwrap();

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

            println!("input(dirty muls): {:?}", first_line);
        }
        Err(e) => {
            eprintln!("Error reading the first line: {}", e);
        }
    }
    let clean_mul_vec = parse_with_re(re, &first_line);
    println!("sum of all muls: {}", calculate_mul(&clean_mul_vec));
}
