use std::{fs::File, io::{self, BufRead}};
use std::path::Path;
use std::cmp;

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

    if process_splits(&vals) {
        return true;
    }
    else {
        for (idx, val) in vals.iter().enumerate() {
            let before = &vals[0..idx];
            let after = &vals[cmp::min(idx+1, vals.len())..vals.len()];

            if process_splits(&[before, after].concat()) {
                return true;
            }
        }
    }

    return false;
}

fn process_splits(vals: &Vec<&str>) -> bool {
    let mut has_retried = false;

    for (idx, val) in vals[..vals.len() - 2].iter().enumerate() {
        let n0 = val.parse::<i32>().unwrap();
        let n1 = vals[idx + 1].parse::<i32>().unwrap();
        let n2 = vals[idx + 2].parse::<i32>().unwrap();

        println!("Prev: {}, Val: {}, Next: {}", val, n1, n2);

        if !process_trio(n0, n1, n2) {
            return false;
        }
    }

    //println!("Valid");
    return true;
}

fn process_trio(prev: i32, curr: i32, next: i32) -> bool {
    if prev > curr && curr > next {
        if prev - curr > 3 || curr - next > 3 {
            //println!("Invalid - Descending, but gap too large");
            return false;
        }
        
    }
    else if prev < curr && curr < next {
        if curr - prev > 3 || next - curr > 3 {
            //println!("Invalid - Ascending, but gap too large");
            return false;
        }
    }
    else {
        //println!("Invalid - incorrect order");
        return false;
    }

    return true;
}