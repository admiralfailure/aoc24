use std::{fs::File, io::{self, BufRead}};
use std::path::Path;

fn main() {
    let input_path = "input.txt";
    let mut total = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                total = process_line(&line_value);
            }
        }
    }

    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_line(line: &str) -> usize {
    let mut disk: Vec<String> = Vec::new();
    let mut file_id = 0;
    for (idx, data) in line.chars().enumerate() {
        let data_length = data.to_digit(10).unwrap();

        // Even digits are files
        if idx % 2 == 0 {
            //println!("File: {} blocks, id {}", data, file_id);
            
            for _ in 0..data_length {
                disk.push(file_id.to_string());
            }

            file_id += 1;
        }
        else if idx % 2 == 1 {
            // Odd digits are space
            //println!("Free space: {} blocks", data);

            for _ in 0..data_length {
                disk.push(".".to_string());
            }
        }
    }

    //println!("Disk map: {:?}", disk);

    let mut forward_idx = 0;
    let mut backward_idx = disk.len() - 1;
    
    while !defrag_tick(&mut disk, &mut forward_idx, &mut backward_idx) {
        //println!("{:?}", disk);
    }

    // Calculate the checksum
    let mut checksum = 0;

    println!("{:?}", disk);
    for (idx, block) in disk.iter().enumerate() {
        //println!("{}, {}", idx, block);

        if block == "." {
            break;
        }

        let file_id = block.parse::<usize>().unwrap();
        checksum += idx * file_id;

        //println!("{} * {} = {} (checksum {})", idx, file_id, idx * file_id, checksum);
    }

    return checksum;
}

fn defrag_tick(disk: &mut Vec<String>, forward_idx: &mut usize, backward_idx: &mut usize) -> bool {
    // Are we currently on a '.' going forward?
    while *forward_idx <= disk.len() - 1 && disk[*forward_idx] != "." {
        *forward_idx += 1;
    }

    // Are we currently on a number going backward?
    while *backward_idx >= 0 && disk[*backward_idx] == "." {
        *backward_idx -= 1;
    }

    if *backward_idx > *forward_idx {
        // Should now be in a position to swap
        let forward = &disk[*forward_idx].to_string();
        let backward = &disk[*backward_idx].to_string();

        //println!("Replacing '{}' with '{}'...", forward,backward);

        disk[*forward_idx] = backward.to_string();
        disk[*backward_idx] = forward.to_string();
    }

    // Return true if we're finished
    if *backward_idx <= *forward_idx || *forward_idx == disk.len() - 1 || *backward_idx == 0 {
        return true;
    }

    return false;
}