#![allow(warnings)]

use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::fs::File;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut printing_department = PrintingDepartment::default();

    printing_department.calculate_accessible_rolls(reader);
    println!("{}", printing_department.sum_acc_rolls);

}


struct PrintingDepartment {
    // lines will be changed with each iteration
    // top    - ...@@...@ ...
    // middle - ..@@...@. ...
    // bottom - @...@@@@@ ...
    top_line: String,
    middle_line: String,
    bottom_line: String,

    // we need this to know when to stop
    file_lines: usize,
    
    current_line_num: usize,
    sum_acc_rolls: usize,

}


impl PrintingDepartment {
    fn default() -> Self {
        PrintingDepartment{
            top_line: "".to_string(),
            middle_line: "".to_string(),
            bottom_line: "".to_string(),
            file_lines: 0,
            current_line_num: 0,
            sum_acc_rolls: 0,
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

    fn check_rolls(&mut self, current_line: &str) {
        match self.current_line_num {
            1 => {
                // When we begin there is only 1 line read we store it as current bottom line
                self.bottom_line = current_line.to_string();

            }
            2 => {

                // We put the bottom line as the middle line and iterate over the middle line and
                // check
                self.middle_line = self.bottom_line.clone();
                self.bottom_line = current_line.to_string();

                // check how many rolls in the current line can be accessed
                self.examine_curent_row();

            }
            _ => {
                
                self.top_line = self.middle_line.clone();
                self.middle_line = self.bottom_line.clone();
                self.bottom_line = current_line.to_string();

                self.examine_curent_row();

                if self.current_line_num == self.file_lines {
                    // When we are on the last line we add it last time as bottom line 
                    // after that we examin the last added line changing it to middle and the one
                    // that was previosly middle is now top
                    // Has to happen in the current step

                    self.top_line = self.middle_line.clone();
                    self.middle_line = self.bottom_line.clone();
                    self.bottom_line = "".to_string();

                    self.examine_curent_row();

                    

                }

            }
            
        };
    

    }


    fn examine_curent_row(&mut self) {

        let mut indexes: Vec<u8> = vec![];

        let l_len = self.middle_line.len();
        let t_len = self.top_line.len();
        let b_len = self.bottom_line.len();

        let roll_char = Some(&'@');

        let mut top_vec: Vec<char> = [].to_vec();
        let mut bot_vec: Vec<char> = [].to_vec();

        let mut mid_vec = self.middle_line
            .trim()
            .chars()
            .collect::<Vec<char>>();

        if !(self.top_line == "".to_string()) {
            top_vec = self.top_line
                .trim()
                .chars()
                .collect::<Vec<char>>()
        }
        if !(self.bottom_line == "".to_string()) {
            bot_vec = self.bottom_line
                .trim()
                .chars()
                .collect::<Vec<char>>();
        }

        // check for every element of the current middle line line
        // if it is accessible
        for pos in 0..l_len {

            if mid_vec.get(pos) != roll_char {
                print!(".");
                continue;
            }

            // Count how many rolls are adjacent to the current
            let mut curr_pos_adj = 0;

            match pos {
                0 => {

                    if !(self.bottom_line == "".to_string()) {

                        if roll_char == bot_vec.get(pos) {
                            curr_pos_adj += 1;
                        }
                        if roll_char == bot_vec.get(pos + 1) {
                            curr_pos_adj += 1;
                        }

                    }
                    if !(self.top_line == "".to_string()) {

                        if roll_char == top_vec.get(pos) {
                            curr_pos_adj += 1;
                        }
                        if roll_char == top_vec.get(pos + 1) {
                            curr_pos_adj += 1;
                        }
                    }

                    if roll_char == mid_vec.get(pos + 1) {
                        curr_pos_adj += 1;
                    }
                }
                _ => {

                    if !(self.bottom_line == "".to_string()) {

                        if roll_char == bot_vec.get(pos) {
                            curr_pos_adj += 1;
                        }
                        if roll_char == bot_vec.get(pos - 1) {
                            curr_pos_adj += 1;
                        }
                        if roll_char == bot_vec.get(pos + 1) {
                            curr_pos_adj += 1;
                        }

                    }
                    if !(self.top_line == "".to_string()) {

                        if roll_char == top_vec.get(pos) {
                            curr_pos_adj += 1;
                        }
                        if roll_char == top_vec.get(pos - 1) {
                            curr_pos_adj += 1;
                        }
                        if roll_char == top_vec.get(pos + 1) {
                            curr_pos_adj += 1;
                        }
                    }

                    if roll_char == mid_vec.get(pos - 1) {
                        curr_pos_adj += 1;
                    }
                    if roll_char == mid_vec.get(pos + 1) {
                        curr_pos_adj += 1;
                    }
                }
            }

            if curr_pos_adj < 4 {
                self.sum_acc_rolls += 1;
                print!("X");
            }
            else {
                print!("@");
            }


        }
        println!()



    }

}
