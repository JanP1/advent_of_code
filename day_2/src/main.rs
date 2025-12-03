use std::io::Read;
use utf8_read::Reader;


/*
 * Action enum
 * Represents either the ending of the first number - lower bound
 * or the second number - upper bound
 * or just a number char
 * or an error
*/
enum Action {
    AddLowerBounds(i32),
    AddUpperBounds(i32),
    ReadNumChar(i32),
    Error,

}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let mut reader = Reader::new(&file);

    println!("Sum of invalid ids = {}", sum_of_invalid_ids(&mut reader))

}


fn sum_of_invalid_ids<R>(reader: &mut Reader<R>) -> i32 where R: Read {
    
    let mut sum = 0;


    for c in reader {
        match c {
            Ok(x) => {
                // checking the current char
                print!("{x} ... "); 


            }
            Err(e) => { println!("{e}") }
        }
    }

    0
}

fn handle_current_char(c: char) -> Action {
    match  c {
        ' ' => {}
        _ => {}
        
    }

    Action::ReadNumChar(5)
}
