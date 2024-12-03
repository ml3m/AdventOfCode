// 2015 day 5 p1

// check for 3 vowels
// check double letter 
// check priority: (ab, cd, pq, xy) 
use std::fs::File;
use std::io::{self, BufRead};

fn has_non_overlapping_pairs(s: &String) -> bool {
    let s = s.as_str(); // Convert &String to &str
    for i in 0..s.len() - 1 {
        let pair = &s[i..i + 2];  // Take the two-character substring
        if let Some(rest) = s[i + 2..].find(pair) {  // Look for the pair again, without overlapping
            return true;
        }
    }
    false
}

fn has_repeating_letter_with_one_between(s: &String) -> bool {
    let s = s.as_str(); // Convert &String to &str
    for i in 0..s.len() - 2 {
        if s.chars().nth(i) == s.chars().nth(i + 2) {  // Check if the letter repeats with exactly one letter between
            return true;
        }
    }
    false
}

fn string_checked_counting_fn(string_list: Vec<String>) -> i32{
    let mut nice_string_count = 0;

    for string in string_list{
        if has_non_overlapping_pairs(&string) && has_repeating_letter_with_one_between(&string){
                nice_string_count += 1;
        }
    }
    nice_string_count
}


fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input")?;
    
    // Create a BufReader for more efficient reading
    let reader = io::BufReader::new(file);
    
    // Collect lines into a vector
    let lines: Vec<String> = reader
        .lines() // Iterator over lines in the file
        .filter_map(Result::ok) // Filter out errors
        .collect();
    
    // Print the vector
    for line in &lines {
        println!("{}", line);
    }
    println!("the count of nice strings: {}",string_checked_counting_fn(lines));

    Ok(())
}
