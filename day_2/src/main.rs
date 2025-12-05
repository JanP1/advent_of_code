// #![allow(warnings)]

use std::{io::Read};
use utf8_read::Reader;

// Struct to keep track of all invalid ids in the specified ranges
struct InvalidIdManager {
    sum: u64, // sum of all invalid ids
    lower_b: String, // lower bound - value to add next chars to
    upper_b: String, // upper bound - value to add next chars to

    l_b_completed: bool, // whether the lower bound is completed

    num_l_b: u64, // a value to store the completed lower bound - casted lower_b
    num_u_b: u64, // a value to store the completed upper bound - casted upper_b
}

impl InvalidIdManager {
    
    fn sum_of_invalid_ids<R>(&mut self, reader: &mut Reader<R>) -> u64 where R: Read {
        
        self.sum = 0;
        self.lower_b = "".to_string();
        self.upper_b = "".to_string();
        self.l_b_completed = false;


        for c in reader {
            match c {
                Ok(ch) => {
                    // checking the current char
                    Self::handle_current_char(self, ch);

                }
                Err(e) => { println!("{e}") }
            }
        }


        // At the end check for the last range
        if !self.lower_b.is_empty() || !self.upper_b.is_empty() {
            println!("Lower b completed {}", self.l_b_completed);
            if self.l_b_completed {
                println!("Self upper = {}", self.upper_b);
                self.num_u_b = self.upper_b.trim().parse::<u64>().unwrap_or(0);
                self.l_b_completed = false;
                Self::sum_wrong_ids_in_range(self);
            }
        }

        self.sum
    }


    fn handle_current_char(&mut self, c: char) {
        match c {
            '-' => {
                self.l_b_completed = true;

                // TODO: Change to a match with a print if defaulting to 0
                self.num_l_b = self.lower_b.parse::<u64>().unwrap_or(0);

                // the value for storing the chars is set back to empty
                self.lower_b = "".to_string();
                
            }
            ',' => {

                self.l_b_completed = false;

                // TODO: Change to a match with a print if defaulting to 0
                self.num_u_b = self.upper_b.parse::<u64>().unwrap_or(0);

                // the value for storing the chars is set back to empty
                self.upper_b = "".to_string();

                Self::sum_wrong_ids_in_range(self);

            }
            _ => {
                match self.l_b_completed {
                    true => {self.upper_b.push(c);}
                    false => {self.lower_b.push(c);}
                    
                }
            }
        }
    }

    fn sum_wrong_ids_in_range(&mut self) {

        // if both bounds are of the same uneven length there will be no invalid ids
        // and we would skip
        let lower_bound = self.num_l_b.to_string();
        let upper_bound = self.num_u_b.to_string();

        println!("Lower b {lower_bound} Upper b {upper_bound}");

        if  !( lower_bound.len() == upper_bound.to_string().len() && lower_bound.len() % 2 == 1 ) {

            for num in self.num_l_b..=self.num_u_b {
                let mut num_str = num.to_string();
                if num_str.len() % 2 == 0 {
                    let second_half = num_str.split_off(num_str.len()/2);

                    // if the 2 parts are the same add to sum
                    if num_str == second_half {
                        println!("Adding - {num}");
                        self.sum += num;

                    }
                   

                }

            }

        }

    }
}


fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let mut reader = Reader::new(&file);
    let mut id_manager = InvalidIdManager{
        sum:0, 
        lower_b:"".to_string(),
        upper_b:"".to_string(), 
        l_b_completed:false, 
        num_l_b:0, 
        num_u_b:0
    };

    println!("Sum of invalid ids = {}", id_manager.sum_of_invalid_ids(&mut reader))

}
