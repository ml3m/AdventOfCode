/*
 *
 * now, besides the initial conditions from part_one, 
 * there is another condition added:
 *
 *
 * if by removing one number from a report makes the report safe. 
 * then it is considered Safe.
 *
 */


use std::i32;
use std::fs::File;
use std::io::{self, BufRead};

fn abs_diff(a: i32, b: i32) -> i32 {
    (a-b).abs()
}

fn check_safe_report_conditions(report: &Vec<i32>) -> bool{
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 0..report.len() -1 {

        // diff checking 
        let diff = abs_diff(report[i], report[i+1]);
        if diff < 1 || diff > 3 {
            return false;
        }

        // increasing/decreasing checking
        if report[i] < report[i + 1] {
            is_decreasing = false;
        } else if report[i] > report[i + 1] {
            is_increasing = false;
        }
    }

    is_increasing || is_decreasing
}

fn check_by_removing_numbers_if_safe(report: &Vec<i32>) -> bool{
    // it should remove each number from the list 
    // check if is safe. 
    // once safe return.
    // if not safe add the number back and remove the next. and so on. 
    // the function for checking if is safe is check_safe_report_conditions(report)
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);

        if check_safe_report_conditions(&new_report) {
           return true;
        }
    }
    false
}

fn calculate_reports_safe_number(reports_list: &Vec<Vec<i32>>) -> i32{
    let mut num_safe_reports = 0;

    for report in reports_list{
        if check_safe_report_conditions(report) {
            num_safe_reports += 1;
        } else if check_by_removing_numbers_if_safe(report) {
            num_safe_reports += 1;
        }
    }

    num_safe_reports
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

    let mut list_of_reports = Vec::new(); // This will hold the list of Vec<i32>s

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let nums: Vec<i32> = line
                    .split_whitespace()  // Split by whitespace
                    .filter_map(|s| s.parse::<i32>().ok())  // Parse to i32
                    .collect();  // Collect the parsed i32s into a Vec<i32>
                list_of_reports.push(nums);  // Push the reports into the outer vector
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return;
            }
        }
    }

    let total_safe_reports = calculate_reports_safe_number(&list_of_reports);
    println!("The total number of  Safe Reports are: {}", total_safe_reports);
}
