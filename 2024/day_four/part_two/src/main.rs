// Part 1

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

// Part 2 
//
// define a pattern
//
/// all cases.
//
// M.M
// .A.
// S.S
//
// M.S
// .A.
// M.S
//
// S.S
// .A.
// M.M
//
// S.M
// .A.
// S.M
//
// from the matrix take all chunks of 3x3 and plug them in the
// check_X() that check for all cases.
//
// generate a vector with chunks and check each chunk in the check_X() function.
 
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn are_matrices_equal<T: PartialEq>(matrix1: &Vec<Vec<T>>, matrix2: &Vec<Vec<T>>) -> bool {
    if matrix1.len() != matrix2.len() {
        return false; // Different number of rows
    }

    for (row1, row2) in matrix1.iter().zip(matrix2.iter()) {
        if row1.len() != row2.len() {
            return false; // Different number of columns
        }

        for (elem1, elem2) in row1.iter().zip(row2.iter()) {
            if *elem1 != *elem2 {
                return false; // If elements are different and it's not a dot
            }
        }
    }
    true // Matrices are equal
}

fn create_chunks_vector(matrix: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut chunks = Vec::new();

    // Check if the matrix is large enough (at least 3x3)
    let rows = matrix.len();
    let cols = matrix[0].len();

    if rows < 3 || cols < 3 {
        return chunks; // Return an empty vector if the matrix is too small
    }

    // Iterate over all possible starting positions for 3x3 submatrices
    for i in 0..=rows - 3 {
        for j in 0..=cols - 3 {
            let mut submatrix = Vec::new();
            for x in i..i + 3 {
                let mut row = Vec::new();
                for y in j..j + 3 {
                    row.push(matrix[x][y]);
                }
                submatrix.push(row);
            }
            chunks.push(submatrix);
        }
    }
    chunks
}

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

// This function checks if the chunk matches the pattern with dots treated as any character
fn are_matrices_equal_with_any_char(
    chunk: &Vec<Vec<char>>,
    pattern: &Vec<Vec<char>>,
) -> bool {
    if chunk.len() != pattern.len() {
        return false; // Different number of rows
    }

    for (row_chunk, row_pattern) in chunk.iter().zip(pattern.iter()) {
        if row_chunk.len() != row_pattern.len() {
            return false; // Different number of columns
        }

        for (c1, c2) in row_chunk.iter().zip(row_pattern.iter()) {
            if *c2 != '.' && *c1 != *c2 {
                return false; // If the character is not a dot and does not match
            }
        }
    }

    true
}

fn check_pattern_1(chunk: &Vec<Vec<char>>) -> bool {
    // Pattern 1: M.M
    // .A.
    // S.S
    let pattern = vec![
        vec!['M', '.', 'M'],
        vec!['.', 'A', '.'],
        vec!['S', '.', 'S'],
    ];
    are_matrices_equal_with_any_char(chunk, &pattern)
}

fn check_pattern_2(chunk: &Vec<Vec<char>>) -> bool {
    // Pattern 2: M.S
    // .A.
    // M.S
    let pattern = vec![
        vec!['M', '.', 'S'],
        vec!['.', 'A', '.'],
        vec!['M', '.', 'S'],
    ];
    are_matrices_equal_with_any_char(chunk, &pattern)
}

fn check_pattern_3(chunk: &Vec<Vec<char>>) -> bool {
    // Pattern 3: S.S
    // .A.
    // M.M
    let pattern = vec![
        vec!['S', '.', 'S'],
        vec!['.', 'A', '.'],
        vec!['M', '.', 'M'],
    ];
    are_matrices_equal_with_any_char(chunk, &pattern)
}

fn check_pattern_4(chunk: &Vec<Vec<char>>) -> bool {
    // Pattern 4: S.M
    // .A.
    // S.M
    let pattern = vec![
        vec!['S', '.', 'M'],
        vec!['.', 'A', '.'],
        vec!['S', '.', 'M'],
    ];
    are_matrices_equal_with_any_char(chunk, &pattern)
}

fn check_X(chunk: &Vec<Vec<char>>) -> bool {
    // Check all patterns
    check_pattern_1(chunk) || check_pattern_2(chunk) || check_pattern_3(chunk) || check_pattern_4(chunk)
}

// Function to print a chunk (3x3 matrix)
fn print_chunk(chunk: &Vec<Vec<char>>) {
    for row in chunk {
        println!("{:?}", row.iter().collect::<String>());
    }
    println!(); // Add an extra newline for readability between chunks
}

fn main() -> io::Result<()> {
    let path = "input"; // Replace with your file path
    let matrix = read_file_to_matrix(path)?;

    let chunks = create_chunks_vector(&matrix);

    // Initialize a counter for the matches
    let mut match_count = 0;

    // Print each chunk before checking for patterns
    println!("Generated Chunks:");
    for chunk in &chunks {
        print_chunk(chunk); // Print the chunk
    }

    // Check each chunk against all patterns and count the matches
    for chunk in chunks {
        if check_X(&chunk) {
            match_count += 1; // Increment the counter for each matching chunk
        }
    }

    // Print the number of matching chunks
    println!("Total matching chunks: {}", match_count);

    Ok(())
}
