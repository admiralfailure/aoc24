use std::{fs::File, io::{self, BufRead}};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let input_path = "input.txt";

    let mut leftValues: Vec<u32> = Vec::new();
    let mut rightValues: HashMap<u32, u32> = HashMap::new();
    let mut total: u32 = 0;

    println!("Loading file {}", input_path);

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                println!("{}", line_value);

                let (left, right) = process_line(line_value);

                leftValues.push(left);
                let count = rightValues.entry(right).or_insert(0);
                *count += 1;
            }
        }

        leftValues.sort();

        let mut idx = 0;
        for left in leftValues {
            let multiple = match rightValues.get(&left) {
                Some(mult) => mult,
                None => &0
            };
            let val: u32;
            
            val = left * multiple;

            total += val;
            idx += 1;
        }
    }

    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_line(line: String) -> (u32, u32) {
    let parts : Vec<&str> = line.split(" ").collect();

    let mut idx = 0;
    for p in &parts {
        print!("Part {}: '{}'", idx, p);
        idx += 1;
    }

    let left = parts[0];
    let right = parts[3];

    let leftInt = left.parse::<u32>().unwrap();
    let rightInt: u32 = right.parse::<u32>().unwrap();

    println!("Left: {}, Right: {}", leftInt, rightInt);

    return (leftInt, rightInt);
}
