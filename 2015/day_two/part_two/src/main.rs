use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn calculate_ribbon_for_present(one_present_dimensions: &Vec<i32>) -> i32 {
    let l = one_present_dimensions[0];
    let w = one_present_dimensions[1];
    let h = one_present_dimensions[2];

    // Calculate the smallest perimeter
    let smallest_perimeter = 2 * (cmp::min(l + w, cmp::min(w + h, h + l)));
    
    // Calculate the bow length (volume of the present)
    let bow_length = l * w * h;

    smallest_perimeter + bow_length
}

fn total_ribbon_for_all_presents(all_presents: &Vec<Vec<i32>>) -> i32 {
    let mut total_ribbon = 0;
    for present in all_presents {
        total_ribbon += calculate_ribbon_for_present(&present);
    }
    total_ribbon
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
    
    let reader = io::BufReader::new(file);

    let mut list_of_gifts = Vec::new(); // This will hold the list of Vec<i32>s

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let nums: Vec<i32> = line
                    .split('x')
                    .filter_map(|s| s.parse::<i32>().ok())  // Parse to i32
                    .collect();  // Collect the parsed i32s into a Vec<i32>
                list_of_gifts.push(nums);  // Push the dimensions into the outer vector
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return;
            }
        }
    }

    let total_ribbon_needed = total_ribbon_for_all_presents(&list_of_gifts);
    println!("The total feet of ribbon required is: {}", total_ribbon_needed);
}
