use std::{fs::File, io::{self, BufRead}};
use std::path::Path;

fn main() {
    let input_path = "input.txt";
    let mut total = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                if process_line(&line_value) {
                    println!("Safe - {}", &line_value);
                    total += 1;
                }
                else {
                    println!("Unsafe - {}", &line_value);
                }

                println!("");
            }
        }
    }

    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_line(line: &String) -> bool {
    let vals: Vec<&str> = line.split(" ").collect();

    let mut iter = vals.iter().peekable();
    let mut prev = iter.next().unwrap().parse::<i32>().unwrap();

    for _idx in 1..vals.len() - 1 {
        let curr = iter.next().unwrap().parse::<i32>().unwrap();
        let next = iter.peek().unwrap().parse::<i32>().unwrap();

        println!("Prev: {}, Val: {}, Next: {}", prev, curr, next);

        if prev > curr && curr > next {
            if prev - curr > 3 || curr - next > 3 {
                println!("Invalid - Descending, but gap too large");
                return false;
            }
            
        }
        else if prev < curr && curr < next {
            if curr - prev > 3 || next - curr > 3 {
                println!("Invalid - Ascending, but gap too large");
                return false;
            }
        }
        else {
            println!("Invalid - incorrect order");
            return false;
        }

        prev = curr;
    }

    println!("Valid");
    return true;
}
