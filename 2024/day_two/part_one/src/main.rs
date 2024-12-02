/*
 *
 *  each row is to be checked if it is Safe or Unsafe
 *  Safe is :
 *      - Levels (numbers) are either all increasing or all decreasing
 *      - Any two adjacent levels differ by at least 1 and at most 3. 
 *      ( difference of levels:d  st.  1<= d <= 3)
 *
 *      return how many reports are safe.  ( sum of all safe reports computed. )
 *
 * approach:
 *      1. get hold of each row from the file. 
 *      2. check if increasing or decreasing. 
 *      3. for checking the condition of 1<= d <=3 we can use the function with absolute distance
 *         (used in day_one 1dec)
 *         between the neighbours numbers. and check that output out.
 *      4. decide if is S/US
 *      5. add all decides. -> number of safe reports.
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

    // increasing 
    for i in 0..report.len() -1 {
        let diff = abs_diff(report[i], report[i+1]);

        if diff < 1 || diff > 3 {
            return false;
        }

        if report[i] < report[i + 1] {
            is_decreasing = false;
        } else if report[i] > report[i + 1] {
            is_increasing = false;
        }
    }

    is_increasing || is_decreasing
}

fn calculate_reports_safe_number(reports_list: &Vec<Vec<i32>>) -> i32{
    let mut num_safe_reports = 0;

    for report in reports_list{
        if check_safe_report_conditions(report) {
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
