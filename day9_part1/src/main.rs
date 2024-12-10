use std::{fs::File, io::{self, BufRead}};
use std::path::Path;

fn main() {
    let input_path = "input.txt";

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                process_line(&line_value);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_line(line: &str) {
    let mut disk: Vec<String> = Vec::new();
    let mut file_id = 0;
    for (idx, data) in line.chars().enumerate() {
        let data_length = data.to_digit(10).unwrap();

        // Even digits are files
        if idx % 2 == 0 {
            println!("File: {} blocks, id {}", data, file_id);
            
            for _ in 0..data_length {
                disk.push(file_id.to_string());
            }

            file_id += 1;
        }
        else if idx % 2 == 1 {
            // Odd digits are space
            println!("Free space: {} blocks", data);

            for _ in 0..data_length {
                disk.push(".".to_string());
            }
        }
    }

    println!("Disk map: {:?}", disk);

    let iter_forward = disk.iter();
    let iter_back = disk.iter().rev();

    defrag_tick(&mut disk, iter_forward, iter_back);
);
}

fn defrag_tick(disk: &mut Vec<String>, forward: impl Iterator, backward: impl Iterator) {
    
}