// 2015 day 5 p1

// check for 3 vowels
// check double letter 
// check priority: (ab, cd, pq, xy) 
use std::fs::File;
use std::io::{self, BufRead};

fn check_vowels(string: &String) -> bool{
    let vowel_count = string.chars()
        .filter(|c| "aeiouAEIOU".contains(*c))
        .count();
        
    if vowel_count >=3 {
        return true
    }
    false
}

fn check_double_letter(string: &String) -> bool{
    string.chars()
        .zip(string.chars().skip(1))
        .any(|(a, b)| a==b)
}

fn check_priority_groups(string: &String) -> bool{
    let groups = ["ab", "cd", "pq", "xy"];
    groups.iter().any(|&group| string.contains(group))

}

fn string_checked_counting_fn(string_list: Vec<String>) -> i32{
    let mut nice_string_count = 0;

    for string in string_list{
        if !check_priority_groups(&string) 
            && check_vowels(&string)
            && check_double_letter(&string){
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
