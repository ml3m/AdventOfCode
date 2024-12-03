// day 3 part 2
/*
 *
 *
There are two new instructions you'll need to handle:

The do() instruction enables future mul instructions.
The don't() instruction disables future mul instructions.
Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.

For example:

xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.
 *
 *
 */

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};


fn parse_with_re(re_mul_dodont: Regex, dirty_input: &str) -> Vec<&str> {
    // add all both clean mull and do/don'ts to the clean_input_muls vec.
    //
    // start as do = true
    // and check for do's and don's that alternate the boolean do.
    // if true then calculate prod and add to sum.
    // if false ignore all muls until you find do again. -> do = true.

    let mut clean_input_mul_or_dodont_list = Vec::new();

    for mul_or_dodont in re_mul_dodont.find_iter(dirty_input) {
        println!("mul found: {:?}", mul_or_dodont.as_str());
        clean_input_mul_or_dodont_list.push(mul_or_dodont.as_str());
    }
    clean_input_mul_or_dodont_list
}

fn calculate_mul(clean_input: &Vec<&str>) -> i32 {
    let mut sum_of_prod = 0;
    let mut do_calc = true;

    for mul_or_dodont in clean_input {
        match *mul_or_dodont {
            "do()" => do_calc = true,
            "don't()" => do_calc = false,
            _ if do_calc => {
                let numbers: Vec<i32> = mul_or_dodont
                    .split(|c: char| !c.is_numeric())
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect();
                let prod: i32 = numbers.iter().product();
                sum_of_prod += prod;
            }
            _ => {}
        }
    }
    sum_of_prod
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

            println!("input(dirty muls): {:?}", first_line);
        }
        Err(e) => {
            eprintln!("Error reading the first line: {}", e);
        }
    }

    let pattern = r"mul\(\s*\d+\s*,\s*\d+\s*\)|do\(\)|don't\(\)";
    let re_mul_dodont = Regex::new(pattern).unwrap();

    let clean_input_mul_or_dodont_list = parse_with_re(re_mul_dodont, &first_line);
    println!("sum of all muls: {}", calculate_mul(&clean_input_mul_or_dodont_list));
}

