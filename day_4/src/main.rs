#![allow(warnings)]

use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::fs::File;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut printing_department = PrintingDepartment::default();

    printing_department.calculate_accessible_rolls(reader);

}


struct PrintingDepartment {
    // lines will be changed with each iteration
    // top    - ...@@...@
    // middle - ..@@...@. 
    // bottom - @...@@@@@
    top_line: String,
    middle_line: String,
    bottom_line: String,

    file_lines: usize,
    current_line_num: usize,

}


impl PrintingDepartment {
    fn default() -> Self {
        PrintingDepartment{
            top_line: "".to_string(),
            middle_line: "".to_string(),
            bottom_line: "".to_string(),

            file_lines: 0,
            current_line_num: 0,
        }

    }

    fn calculate_accessible_rolls<R>(&mut self, mut reader: BufReader<R>) -> Result<(), std::io::Error>
    where R: Read + Seek{

        self.file_lines = reader.by_ref().lines().count();
        reader.get_mut().seek(SeekFrom::Start(0))?; 

        for line in reader.lines() {
            match line {
                Ok(l) => {
                    self.current_line_num += 1;
                    self.check_rolls(&l);
                    

                }
                Err(e) => { eprintln!("{e}") }
            }
        }

        println!("Lines = {}", self.file_lines);

        Ok(())

    }

    fn check_rolls(&self, current_line: &str) {
        match self.current_line_num {
            1 => {
                // When we begin there is only 1 line read we store it as current bottom line

            }
            2 => {
                // We put the bottom line as the middle line and iterate over the middle line and
                // check

            }
            _ => {
                if self.current_line_num == self.file_lines {
                    // When we are on the last line we add it last time as bottom line 
                    // after that we examin the last added line changing it to middle and the one
                    // that was previosly middle is now top
                    // Has to happen in the current step
                }
                else{
                    // We add new lines as bottom line and calculate the possible positions
                }

            }
            
        };
    

    }

}
