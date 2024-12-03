use std::{fs::File, io::{self, BufRead}};
use std::path::Path;
use regex::Regex;

fn main() {
    let input_path = "input.txt";
    let mut total = 0;
    let mut is_enabled = true;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                let sum = process_line(&line_value, &mut is_enabled);
                total += sum;
            }
        }
    }

    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_line(line: &String, is_enabled: &mut bool) -> i32 {
    let reg = Regex::new(r"do\(()()\)|don't\(()()\)|mul\((\d+),(\d+)\)").unwrap();

    let mut line_total = 0;
    for (command, [arg1, arg2]) in reg.captures_iter(line).map(|c| c.extract()) {
        match command {
            "do()" => *is_enabled = true,
            "don't()" => *is_enabled = false,
            _ => {
                if *is_enabled {
                    let val1 = arg1.parse::<i32>().unwrap();
                    let val2 = arg2.parse::<i32>().unwrap();

                    line_total += val1 * val2;
                }
            }
        }
    }

    return line_total;
}

