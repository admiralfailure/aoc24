use std::{fs::File, io::{self, BufRead}};
use std::path::Path;
use regex::Regex;

fn main() {
    let input_path = "input.txt";
    let mut total = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                total += process_line(&line_value);
            }
        }
    }

    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_line(line: &String) -> i32 {
    let reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut lineTotal = 0;
    for (_, [arg1, arg2]) in reg.captures_iter(line).map(|c| c.extract()) {
        let val1 = arg1.parse::<i32>().unwrap();
        let val2 = arg2.parse::<i32>().unwrap();

        lineTotal += val1 * val2;
    }

    return lineTotal;
}

