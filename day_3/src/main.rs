#![allow(warnings)]

use std::{fs::File, io::{BufRead, BufReader, Read}};

fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut joltage_manager = JoltageManager::default();
    joltage_manager.count_total_output_joltage_from_file(reader);

}

// A struct to keep track of the current 
// Highiest joltage number for a given batch
struct JoltageNumber {
    num_of_tens: u8,
    num_of_ones: u8,
}

impl JoltageNumber {

    fn default() -> Self {
        JoltageNumber { num_of_tens: 1, num_of_ones: 1 }
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
        println!("{line}");

    }
}
