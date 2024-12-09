use std::{fs::File, io::{self, BufRead}};
use std::path::Path;

fn main() {
    let input_path = "input.txt";

    let mut total = 0;
    let mut idx = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                println!("Processing line {}...", idx + 1);
                match process_line(&line_value) {
                    Some(x) => total += x,
                    None => {}
                }
            }

            idx += 1;
        }
    }

    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_line(line: &str) -> Option<usize> { 
    //println!("Processing '{}'...", line);

    // Split the line into components
    let parts: Vec<&str> = line.split(':').collect();
    let target = parts[0].parse::<usize>().unwrap();
    let numbers: Vec<usize> = parts[1].trim().split(' ').collect::<Vec<&str>>().into_iter().map(|x| x.parse::<usize>().unwrap()).collect();

    // Recurse through the list
    let result = accumulate(0, numbers.as_slice(), target, &Operator::Add);
    if result == target {
        return Some(target);
    }

    return None;
}

fn accumulate(running_total: usize, numbers: &[usize], target: usize, operator: &Operator) -> usize {
    //println!("Current running total: {}", running_total);

    if numbers.len() > 0 {
        let running_total_inclusive = calculate_value(running_total, numbers[0], operator);
        if accumulate(running_total_inclusive, &numbers[1..numbers.len()], target, &Operator::Add) == target {
            return target;
        }
        if accumulate(running_total_inclusive, &numbers[1..numbers.len()], target, &Operator::Mul) == target {
            return target;
        }
    }

    return running_total;
}

fn calculate_value(a: usize, b: usize, operator: &Operator) -> usize {
    return match operator {
        Operator::Add => a + b,
        Operator::Mul => a * b
    };
}

#[derive(PartialEq, Debug)]
enum Operator {
    Add,
    Mul
}