#![allow(warnings)]

use std::io::Read;
use utf8_read::Reader;



fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let mut reader = Reader::new(&file);

    println!("Sum of invalid ids = {}", sum_of_invalid_ids(&mut reader))

}


fn sum_of_invalid_ids<R>(reader: &mut Reader<R>) -> u32 where R: Read {
    
    let mut sum: u32 = 0;
    let mut lower_bound: String = "".to_string();
    let mut upper_bound: String = "".to_string();
    let mut lower_b_comleted: bool = false;


    for c in reader {
        match c {
            Ok(ch) => {
                // checking the current char
                handle_current_char(ch, sum, &mut lower_bound, &mut lower_b_comleted);
                println!("{lower_bound} ... "); 
                


            }
            Err(e) => { println!("{e}") }
        }
    }

    0
}


fn handle_current_char(c: char, sum: u32, number: &mut String, l_b_completed: &mut bool) {
    match  c {
        '-' => {
            *l_b_completed = true;
        }
        ',' => {
            *l_b_completed = false;
        }
        _ => {
            if 
            number.push(c);
        }
    }
}


fn sum_wrong_ids_in_range(lower_b: u32, upper_b: u32) {

}

