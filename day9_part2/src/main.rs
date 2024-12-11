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

    println!("Disk map: {:?}", disk);
    //println!("Highest file ID: {}", file_id - 1);

    while file_id - 1 >= 0 {
        println!("Processing file {}...", file_id - 1);

        let file = locate_file(&mut disk, (file_id - 1).to_string());
        let space = locate_free_space(&mut disk, (file.1 - file.0) + 1);

        match space {
            Some(free) => {
                if free.0 < file.0 {
                    for (idx, block) in (file.0..file.1 + 1).enumerate() {
                        disk.swap(block, free.0 + idx);
                    }
                }
            }
            None => {}
        }

        //println!("{:?}", disk);

        file_id -= 1;
    }

    // Calculate the checksum
    let mut checksum = 0;

    println!("{:?}", disk);
    for (idx, block) in disk.iter().enumerate() {
        //println!("{}, {}", idx, block);

        if block == "." {
            continue;
        }

        let file_id = block.parse::<usize>().unwrap();
        checksum += idx * file_id;

        //println!("{} * {} = {} (checksum {})", idx, file_id, idx * file_id, checksum);
    }

    return checksum;
}

fn locate_file(disk: &mut Vec<String>, file_id: String) -> (usize, usize) {
    let mut start_idx = 0;
    let mut end_idx = 0;
    for (idx, block) in disk.iter().enumerate() {
        if block != &file_id {
            continue;
        }

        // Found the start
        if start_idx == 0 {
            start_idx = idx;
        }

        end_idx = idx;
    }

    //println!("start: {}, end: {}", start_idx, end_idx);
    return (start_idx, end_idx);
}

fn locate_free_space(disk: &mut Vec<String>, min_size: usize) -> Option<(usize, usize)> {
    let mut start_idx: Option<usize> = None;
    let mut end_idx: Option<usize> = None;

    for (idx, block) in disk.iter().enumerate() {
        if block != "." {
            // Have we got free space meeting our criteria?
            if end_idx.is_some_and(|e| start_idx.is_some_and(|s| (e - s) + 1 >= min_size)) {
                //println!("start: {}, end: {}", start_idx, end_idx);
                return Some((start_idx?, end_idx?));
            }

            // Otherwise move on
            start_idx = None;
            end_idx = None;
            continue;
        }

        // Found the start
        if start_idx.is_none() {
            start_idx = Some(idx);
        }

        end_idx = Some(idx);
    }

    // Have we got free space meeting our criteria?
    if end_idx.is_some_and(|e| start_idx.is_some_and(|s| (e - s) + 1 >= min_size)) {
        //println!("start: {}, end: {}", start_idx, end_idx);
        return Some((start_idx?, end_idx?));
    }

    return None;
}