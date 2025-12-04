use std::{io::Read, u32};
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
    
    fn sum_of_invalid_ids<R>(&mut self, reader: &mut Reader<R>) -> u32 where R: Read {
        
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

        0
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

                Self::sum_wrong_ids_in_range(&self);

            }
            _ => {
                match self.l_b_completed {
                    true => {self.upper_b.push(c);}
                    false => {self.lower_b.push(c);}
                    
                }
            }
        }
    }

    fn sum_wrong_ids_in_range(&self) {

        // if both bounds are of the same uneven length there will be no invalid ids
        // and we would skip
        let lower_bound = self.num_l_b.to_string();
        let upper_bound = self.num_u_b.to_string();

        if  !( lower_bound.len() == self.num_u_b.to_string().len() && lower_bound.len() % 2 == 1 ) {
            print!("Calculating for range ... ");


            // TODO first we need to find if the upper and/or lower bounds are not uneven, if yes, 
            // we need to find real_lower_bound and real_upper_bound

            // splitting the number in two parts to create a real bound
            let upper_char_vec = upper_bound.chars().collect::<Vec<char>>();
            println!("{:?}", &upper_char_vec[..upper_char_vec.len()/2])

        } else {
            print!("Skipping for range ... ")
        }

        println!("{} - {}", self.num_l_b, self.num_u_b);

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
