//block of text is given as input
//find number of all 'XMAX' substrings 
//on all directions,horizontal, vertical, diagonal, 
//written backwards, or even overlapping other
//words.
//
//
// my approach:
//
// have the matrix.
//
// form a vector for each:
//  - rows
//  - columns
//  - main diagonal
//  - secondary diagonal
//
//then each item from each vector should be checked for substring of 
// 'XMAX' and substring of reverse of 'XMAX' and count them.


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_file_to_matrix(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    Ok(matrix)
}

// Generate a vector of all directional vectors as strings
fn vector_of_all_dir_vectors_str(grid: Vec<Vec<char>>) -> Vec<Vec<String>> {
    let mut result = Vec::new();

    // Convert rows to strings
    let row_strings: Vec<String> = grid.iter().map(|row| row.iter().collect()).collect();
    result.push(row_strings);

    // Convert columns to strings
    let col_strings: Vec<String> = (0..grid[0].len())
        .map(|col_idx| grid.iter().map(|row| row[col_idx]).collect())
        .collect();
    result.push(col_strings);

    // Convert main diagonals to strings
    let mut main_diagonals = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    // Top-left to bottom-right main diagonals
    for start in 0..(rows + cols - 1) {
        let mut diagonal = String::new();
        for i in 0..=start {
            let j = start - i;
            if i < rows && j < cols {
                diagonal.push(grid[i][j]);
            }
        }
        if !diagonal.is_empty() {
            main_diagonals.push(diagonal);
        }
    }
    result.push(main_diagonals);

    // Top-right to bottom-left secondary diagonals
    let mut secondary_diagonals = Vec::new();
    for start in 0..(rows + cols - 1) {
        let mut diagonal = String::new();
        for i in 0..=start {
            let j = start - i;
            if i < rows && j < cols {
                diagonal.push(grid[i][cols - 1 - j]);
            }
        }
        if !diagonal.is_empty() {
            secondary_diagonals.push(diagonal);
        }
    }
    result.push(secondary_diagonals);

    result
}

// Count occurrences of the target substring in the grid
fn count_substrings(grid: &Vec<Vec<char>>, target: &str) -> i32 {
    let directional_vectors = vector_of_all_dir_vectors_str(grid.clone());
    let mut count = 0;
    let reversed_target: String = target.chars().rev().collect();
    let reversed_slice: &str = &reversed_target;

    for direction in directional_vectors {
        for line in direction {
            count += line.matches(target).count() as i32;
            count += line.matches(reversed_slice).count() as i32;
        }
    }

    count
}

fn main() -> io::Result<()> {
    let path = "input"; // Replace with your file path
    let matrix = read_file_to_matrix(path)?;

    let target = "XMAS";
    let count = count_substrings(&matrix, target);
    println!("Number of '{}' substrings: {}", target, count);

    Ok(())
}


/*
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_file_to_matrix(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    Ok(matrix)
}

fn count_substrings(grid: &Vec<Vec<char>>, target: &str) -> usize {
    let directions = [
        (0, 1),  // Horizontal (right)
        (0, -1), // Horizontal (left)
        (1, 0),  // Vertical (down)
        (-1, 0), // Vertical (up)
        (1, 1),  // Diagonal (bottom-right)
        (-1, -1), // Diagonal (top-left)
        (1, -1), // Anti-diagonal (bottom-left)
        (-1, 1), // Anti-diagonal (top-right)
    ];
    let target_chars: Vec<char> = target.chars().collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                count += count_in_direction(grid, &target_chars, i, j, dx, dy);
            }
        }
    }
    count
}

fn count_in_direction(
    grid: &Vec<Vec<char>>,
    target: &Vec<char>,
    start_x: usize,
    start_y: usize,
    dx: isize,
    dy: isize,
) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut count = 0;

    let mut x = start_x as isize;
    let mut y = start_y as isize;

    for _ in 0..target.len() {
        if x < 0 || y < 0 || x >= rows || y >= cols || grid[x as usize][y as usize] != target[count] {
            return 0;
        }
        x += dx;
        y += dy;
        count += 1;
    }

    if count == target.len() {
        1
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let path = "input"; // Replace with your file path
    let matrix = read_file_to_matrix(path)?;

    let target = "XMAS";
    let count = count_substrings(&matrix, target);
    println!("Number of '{}' substrings: {}", target, count);

    Ok(())
}
*/
