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

    // Simple case - does adding all the numbers get us the target?
    let total_sum: usize = numbers.iter().sum();
    if total_sum == target {
        return Some(target);
    }

    // // Is the sum too big? Can't possibly succeed if so
    // if total_sum > target {
    //     return None;
    // }

    // // Does multiplying all the numbers get us the target?
    // let mut total_mut = 0;
    // for number in &numbers {
    //     if total_mut == 0 {
    //         total_mut += number;
    //     }
    //     else {
    //         total_mut *= number;
    //     }
    // }
    
    // if total_mut == target {
    //     return Some(target);
    // }
    
    // // Is multiplying all the numbers too small? Can't possibly succeed if so
    // if total_mut < target {
    //     return None;
    // }
    
    // Simple cases done; now to the actual logic
    // let operator_positions = numbers.len() - 1;
    // let mut fixed_operators: Vec<&Operator> = Vec::new();
    //println!("Initial operators: {:?}", shiftable_operators);
    
    // Set all positions to add
    // Check
    // Set the n position to mult
    // Check
    // Rotate left through each position, checking
    // Fix the n position to mult
    // Set the n-1 position to mult
    // Check
    // Rotate left through the first n-1 positions, checking
    // Set the n-2 position to mult
    // etc

    let mut operators = vec![&Operator::Add; numbers.len() - 1];
    let line_total = calculate_line(target, &numbers, &operators);
    if line_total == target {
        return Some(target);
    }

    for f in (0..operators.len() - 1).rev() {
        let mut shiftable_operators = &operators[0..f + 1];

        for n in (0..operators.len() - 1).rev() {
            shiftable_operators[n] = &Operator::Mul;

            for _ in (0..operators.len()) {
                // let combined_operators: Vec<&Operator> = Vec::new();
                // for shiftable in shiftable_operators {
                //     combined_operators.push(shiftable);
                // }
                // for fixed in fixed_operators {
                //     combined_operators.push(fixed);
                // }

                let running_total = calculate_line(target, &numbers, &operators);
                if running_total == target {    
                    return Some(target);
                }

                shiftable_operators.rotate_left(1);
            }
        }


    }

    // for fixed_position in (0..operator_positions + 1).rev() {
    //     println!("fixed position: {}", fixed_position);

    //     let mut shiftable_operators = vec![&Operator::Add; fixed_position];
    //     for position in (0..shiftable_operators.len() - 1).rev() {
    //         println!("position: {}", position);

    //         // Shift until we reach the start
    //         for _ in 0..shiftable_operators.len() {
    //             //println!("Shiftable operators: {:?}", shiftable_operators);
    //             //println!("Fixed operators: {:?}", fixed_operators);
                
    //             let combined_operators = shiftable_operators[0..fixed_position].iter().cloned().chain(fixed_operators.iter().cloned().rev()).collect();
    //             println!("Combined operators: {:?}", combined_operators);
                
    //             let shifted_line_total = calculate_line(target, &numbers, &combined_operators);
    //             if shifted_line_total == target {
    //                 return Some(target);
    //             }
                
    //             if shiftable_operators.len() > 1 {
    //                 shiftable_operators.rotate_left(1);
    //             }
    //         }
            
    //         shiftable_operators[position] = &Operator::Mul;
    //     }

    //     // Set position to multiply
    //     fixed_operators.push(&Operator::Mul);
    // }

    return None;
}

fn calculate_line(target: usize, numbers: &Vec<usize>, operators: &Vec<&Operator>) -> usize {
    let mut running_total = 0;
    for (idx, operator) in operators.iter().enumerate() {
        if idx == 0 {
            running_total =  calculate_value(numbers[idx], numbers[idx + 1], operator);
        } 
        else {
            running_total = calculate_value(running_total, numbers[idx + 1], operator);
        }

        if running_total > target {
            return running_total;
        }

        //println!("Running total: {}", running_total);
    }

    return running_total;
}

fn calculate_value(a: usize, b: usize, operator: &Operator) -> usize {
    //println!("Calculating {} {} {:?}", a, b, operator);

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