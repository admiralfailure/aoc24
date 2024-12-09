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
    let result = accumulate(0, 0, numbers.as_slice(), target, &Operator::Add);
    if result == target {
        return Some(target);
    }

    return None;
}

fn accumulate(depth: usize, running_total: usize, numbers: &[usize], target: usize, operator: &Operator) -> usize {
    //println!("Current running total: {}, depth: {}", running_total, depth);

    if numbers.len() > 0 {
        let running_total_inclusive = calculate_value(running_total, numbers[0], operator);
        //println!("running_total_inclusive: {}, depth: {}, numbers.len: {}", running_total_inclusive, depth, numbers.len());
        if numbers.len() == 1 && running_total_inclusive == target {
            return target;
        }
        else if running_total_inclusive > target {
            return 0;
        }

        if numbers.len() > 1 {
            if accumulate(depth + 1, running_total_inclusive, &numbers[1..numbers.len()], target, &Operator::Add) == target {
                //println!("Current running total: {}, depth: {}", running_total_inclusive, depth);
                //if depth == 0 {
                    return target;
                //}
                //else {
                //    return 0;
                //}
            }
            if accumulate(depth + 1, running_total_inclusive, &numbers[1..numbers.len()], target, &Operator::Mul) == target {
                //println!("Current running total: {}, depth: {}", running_total_inclusive, depth);
                //if depth == numbers.len() {
                    return target;
                //}
                //else {
                //    return 0;
                //}
            }
            if accumulate(depth + 1, running_total_inclusive, &numbers[1..numbers.len()], target, &Operator::Combine) == target {
                //println!("Current running total: {}, depth: {}", running_total_inclusive, depth);
                //if depth == numbers.len() {
                    return target;
                //}
                //else {
                //    return 0;
                //}
            }
        }
    }

    return running_total;
}

fn calculate_value(a: usize, b: usize, operator: &Operator) -> usize {
    return match operator {
        Operator::Add => a + b,
        Operator::Mul => a * b,
        Operator::Combine => {
            //println!("Combining {} and {}...", a, b);

            let mut a_str = a.to_string();
            let b_str = b.to_string();

            a_str.push_str(&b_str);

            let result = a_str.parse::<usize>().unwrap();
            //println!("Result: {}", result);

            return result;
        }
    };
}

#[derive(PartialEq, Debug)]
enum Operator {
    Add,
    Mul,
    Combine
}