// day 4 puzzle
// MD5 hashes which, in hexadecimal, start with at least five zeroes.
// followed by a number in decimal 

// find lowest starting from 1, no leading zeros.  that produces the hashes
//
// brute force way.
//
// check each hash after the pattern
//
// find the number concat to the key = > hash that has 5 zeros at the beggining. => solution is
// number
//

use std::str;

fn find_concat_number(input: &str) -> i32{
    let num = 0;
    for i in 1..1000000000{
        let concat_str = format!("{}{}", input, i);

        let digest = md5::compute(concat_str);
        let digest_str = format!("{:x}", digest);

        if  digest_str.starts_with("000000"){
            return i
        }
    }
    num
}

/*
 * digest is 0000045c5e2b3911eb937d9d8c574f09
    num is 346386
 */

fn main() {
    let puzzle_input = "iwrupvqb";
    let digest = md5::compute(b"iwrupvqb346386");

    let digest_str = format!("{:x}", digest);

    println!("digest is {}", digest_str); // prints digest on screen.
                                          
    let num = find_concat_number(puzzle_input);
    println!("num is {:?}", num);
}
