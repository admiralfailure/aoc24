use std::{fs::File, io::{self, BufRead}};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let input_path = "input.txt";
    let mut stones: HashMap<usize, usize> = HashMap::new();
    const runs: usize = 75;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                stones = process_line(&line_value);
                //println!("Initial arrangement: {:?}", stones);
            }
        }
    }

    for _ in 0..runs {
        stones = tick(&stones);
        //println!("Arrangement: {:?}", stones);
    }

    println!("Stone count: {}", stones.into_values().sum::<usize>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_line(line: &str) -> HashMap<usize, usize> {
    let mut result: HashMap<usize, usize> = HashMap::new();
    let stones: Vec<&str> = line.split(" ").collect();

    for stone in stones {
        println!("Parsing '{}'...", stone);
        let stone_val: usize = stone.parse::<usize>().unwrap();
        if result.contains_key(&stone_val) {
            let stone_count: &mut usize = result.get_mut(&stone_val).unwrap();
            *stone_count = *stone_count + 1;
        }
        else {
            result.insert(stone_val, 1);
        }
    }

    return result;
}

fn tick(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut result: HashMap<usize, usize> = HashMap::new();

    for stone in stones {
        if *stone.0 == 0 {
            // Replaced by 1
            if result.contains_key(&1) {
                let stone_count: &mut usize = result.get_mut(&1).unwrap();
                *stone_count = *stone_count + *stone.1;
            }
            else {
                result.insert(1, *stone.1);
            }
        }
        else if (*stone.0).to_string().len() % 2 == 0 {
            // Split into two stones
            let str_stone = (*stone.0).to_string();
            let len = str_stone.len();
            let mid = len / 2;

            let mut left: Vec<char> = Vec::new();
            let mut right: Vec<char> = Vec::new();

            for (idx, digit) in str_stone.chars().enumerate() {
                if idx < mid {
                    left.push(digit);
                }
                else {
                    right.push(digit);
                }
            }

            let str_left = String::from_iter(left.iter());
            let str_right = String::from_iter(right.iter());
            let val_left = str_left.parse::<usize>().unwrap();
            let val_right = str_right.parse::<usize>().unwrap();

            if result.contains_key(&val_left) {
                let stone_count: &mut usize = result.get_mut(&val_left).unwrap();
                *stone_count = *stone_count + *stone.1;
            }
            else {
                result.insert(val_left, *stone.1);
            }

            if result.contains_key(&val_right) {
                let stone_count: &mut usize = result.get_mut(&val_right).unwrap();
                *stone_count = *stone_count + *stone.1;
            }
            else {
                result.insert(val_right, *stone.1);
            }
        }
        else {
            // Multiply by 2024
            let val = *stone.0 * 2024;
            if result.contains_key(&val) {
                let stone_count: &mut usize = result.get_mut(&val).unwrap();
                *stone_count = *stone_count + *stone.1;
            }
            else {
                result.insert(val, *stone.1);
            }
        }
    }

    return result;
}