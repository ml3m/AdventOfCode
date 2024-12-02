use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

/*
 *   l * w * h
 *
 *
 *   2x3x4 = 2*2*3 + 2*3*4 + 2*4*2 = 12 + 24 + 16 = 36 + 16 = 52
 *
 */


fn calculate_total_sq_ft_wrapping_paper(one_present_dimensions: &Vec<i32>) -> i32 {
    let l = one_present_dimensions[0];
    let w = one_present_dimensions[1];
    let h = one_present_dimensions[2];

    println!("the dimensions are of the first: l:{} w:{} h:{}", l,w,h);

    let sqft_wrapping_paper = (2*l) + (2*w);

    sqft_wrapping_paper + smallest_side
}

fn sum_of_all_dimensions_all(all_presents: &Vec<Vec<i32>>) -> i32{
    let mut total_sqft_all_presents = 0;
    for present in all_presents{
        total_sqft_all_presents += calculate_total_sq_ft_wrapping_paper(&present);
    }

    total_sqft_all_presents
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
                list_of_gifts.push(nums);  // Push the reports into the outer vector
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return;
            }
        }
    }
    let total_size_presents = sum_of_all_dimensions_all(&list_of_gifts);
    println!("The total number of square feet of wrapping paper is: {}", total_size_presents);
}
