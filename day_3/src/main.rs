#![allow(warnings)]

use std::{fs::File, io::{BufRead, BufReader, Read}, u8};

fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut joltage_manager = JoltageManager::default();
    joltage_manager.count_total_output_joltage_from_file(reader);

    println!("Sum {}", joltage_manager.total_output_joltage)

}

// A struct to keep track of the current 
// Highiest joltage number for a given batch
struct JoltageNumber {
    num_of_tens: u8,
    num_of_ones: u8,
}

impl JoltageNumber {

    fn default() -> Self {
        JoltageNumber { num_of_tens: 0, num_of_ones: 0 }
    }

}

struct JoltageManager {
    joltage_number: JoltageNumber,
    total_output_joltage: u64,
}

impl JoltageManager {

    fn default() -> Self {
        JoltageManager { 
            joltage_number: JoltageNumber::default(),
            total_output_joltage: 0 
        }
    }
    
    fn count_total_output_joltage_from_file<R>(&mut self, reader: BufReader<R>)
    where R: Read {
        
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    // TODO: Take the line,
                    // calculate the highiest joltage,
                    // add to total_output_joltage
                    self.calc_high_possible_joltage(&l);

                }
                Err(e) => {eprintln!("{e}")}
            }
        }

    }

    fn calc_high_possible_joltage(&mut self, line: &str) {
        // Searching for the largest digit up until the before-last place
        // Because only 2 digit numbers are considered
        //
        // Afterwards it searches from the specified index up until the end

        // digits in the current batch
        let batch_digits = self.parse_string_slice_to_vec_u8(line);

        // after we find the highiest first digit
        // we can only start searching from it
        let mut largest_num_of_tens_index = 0;
        let mut largest_num_of_ones = 0;

        // - 1 cause we cant have the first digit be on the last position
        for tens in 0..batch_digits.len() - 1 {
            if batch_digits[tens] > self.joltage_number.num_of_tens {
                self.joltage_number.num_of_tens = batch_digits[tens];
                largest_num_of_tens_index = tens;
            }
        }

        for ones in largest_num_of_tens_index + 1..batch_digits.len() {
            if batch_digits[ones] > self.joltage_number.num_of_ones {
                self.joltage_number.num_of_ones = batch_digits[ones];
                largest_num_of_ones = ones;
            }

        }


        // add to the total
        self.total_output_joltage += 
            self.joltage_number.num_of_tens
                .try_into()
                .unwrap_or(1)
            
            * 10 + 
            
            self.joltage_number.num_of_ones
                .try_into()
                .unwrap_or(1);

        println!("{} {}", self.joltage_number.num_of_tens, self.joltage_number.num_of_ones);

        (self.joltage_number.num_of_tens, self.joltage_number.num_of_ones) = (0, 0);



    }

    // returning a vector of u8 values from a string or an empty vec
    fn parse_string_slice_to_vec_u8(&self, slice: &str) -> Vec<u8> {
        slice
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(0) as u8)
            .collect()

    }
}










