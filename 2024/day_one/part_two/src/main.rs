
// find the min in each list, remove it and then add it to a total_distance var.


use std::i32;
use std::fs::File;
use std::io::{self, BufRead};

fn find_min_in_list(num_list: &[i32]) -> (i32, usize){
    let mut min = i32::MAX; // get max value of i32
    let mut pos = 0;

    for (index, &number) in num_list.iter().enumerate(){
        if number < min{
            min = number;
            pos = index;
        }
    }
    (min, pos)
}

fn find_distance(a: i32, b: i32) -> i32 {
    if a == b {
        return 0
    }
    if a < b {
        b-a
    }else {
        a-b
    }
}

fn compute_sum(l1: &mut Vec<i32>, l2: &mut Vec<i32>) -> i32 {
    // find min in each list/ delete them from list/ find distance between 
    // / add to the sum/ the distance
    let mut solve_sum = 0;

    while !l1.is_empty() && !l2.is_empty(){
        let (l1_min, l1_min_pos) = find_min_in_list(l1);
        let (l2_min, l2_min_pos) = find_min_in_list(l2);
        l1.remove(l1_min_pos);
        l2.remove(l2_min_pos);

        solve_sum += find_distance(l1_min, l2_min);
    }

    solve_sum
}

fn number_occurences_number_list(number: i32, list: &[i32]) -> i32{
    let mut occ = 0;
    for &num in list{
        if num == number {
            occ += 1;
        }
    }

    occ
}

fn similarity_score(l1: &[i32], l2: &[i32]) -> i32 {
    let mut sc = 0;

    for &number in l1 {
        let occ = number_occurences_number_list(number, l2);
        sc += number * occ;
    }
    
    sc
}

fn main() -> io::Result<()> {
    let path = "input";

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            if let Ok(num1) = parts[0].parse::<i32>(){
                col1.push(num1);
            }
            if let Ok(num2) = parts[1].parse::<i32>(){
                col2.push(num2);
            }
        }
    }

    //let totalsum = compute_sum(&mut col1, &mut col2);
    //println!("total sum is: {:?}", totalsum);
    let similarity_score = similarity_score(&mut col1,&mut col2);
    println!("similarity_score: {:?}", similarity_score);

    /*
    println!("Column 1: {:?}", col1);
    println!("Column 2: {:?}", col2);
    */

    Ok(())

}

