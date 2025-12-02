use std::{char, fs::File, io::{BufRead, BufReader, Read}};

fn main() {

    let starting_number = 50;
    match read_file("input.txt"){
        Ok(reader) => {
            file_input_rotation(reader, starting_number);
        }
        Err(e) => {println!("{e}")}
        
    };

}

fn file_input_rotation<R>(reader: BufReader<R>, mut current_number: i32) where R: Read {
    let mut zero_count = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => { 
                let l = split_line(&l);
                current_number = rotate(l.0, l.1, current_number);
                if current_number  == 0 {
                    zero_count += 1;
                }
                

            } 
            Err(e) => eprintln!("Error reading line: {}", e), 
        } 
    } 
    println!("Password: {zero_count}")
} 


fn split_line(line: &str) -> (char, i32) { 
    let mut chars = line.chars();
    let direction = chars.next().unwrap_or('R');
    let number = chars.as_str().parse().unwrap_or(0);

    (direction, number)

}

fn read_file(path: &str) -> Result<BufReader<File>, std::io::Error> {
    Ok(BufReader::new(File::open(path)?))
}


fn rotate(direction: char, num: i32, current_number: i32) -> i32 {

    match direction {
        'L' => { wrap_number(current_number - num) }
        'R' => { wrap_number(current_number + num) }
        _ => {println!("Didnt Rotate"); current_number}
        
    }

}


fn wrap_number(num: i32) -> i32 {
    let range = 100;
        let mut value = num;

        if value < 0 {
            value += ((-value / range) + 1) * range;
        }

        value % range

}

