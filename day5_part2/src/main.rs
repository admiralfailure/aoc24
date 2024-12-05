use std::{fs::File, io::{self, BufRead}};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let input_path = "input.txt";
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut has_parsed_all_rules = false;
    let mut total = 0;


    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                if line_value.len() == 0 {
                    has_parsed_all_rules = true;
                    continue;
                }

                if !has_parsed_all_rules {
                    let rule = process_rule(&line_value);
                    if !rules.contains_key(&rule.0) {
                        rules.insert(rule.0, vec![rule.1]);
                    }
                    else {
                        rules.get_mut(&rule.0).unwrap().push(rule.1);
                    }
                }
                else {
                    match process_line(&line_value, &rules) {
                        Some(x) => total += x,
                        None => {}
                    }
                }
            }
        }
    }

    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_rule(line: &String) -> (i32, i32){
    let rule_parts: Vec<&str> = line.split('|').collect();

    let part_1 = rule_parts[0].parse::<i32>().unwrap();
    let part_2 = rule_parts[1].parse::<i32>().unwrap();

    return (part_1, part_2);
}

fn process_line(line: &String, rules: &HashMap<i32, Vec<i32>>) -> Option<i32> {
    println!("Processing {}", line);

    if !line_is_valid(line, rules) {
        println!("Invalid! Fixing...");
        let fixed = fix_line(line, rules);
        return Some(get_middle_value(&fixed));
    }

    println!("Valid.");
    return None;
}

fn line_is_valid(line: &String, rules: &HashMap<i32, Vec<i32>>) -> bool {
    let values: Vec<&str> = line.split(',').collect();
    let mut processed: Vec<i32> = Vec::new();

    for value in values {
        let i_value = value.parse::<i32>().unwrap();
        if rules.contains_key(&i_value) {
            let rule_values = rules.get(&i_value).unwrap();
            for rule_value in rule_values {
                if processed.contains(rule_value) {
                    return false;
                }
            }
        }

        processed.push(i_value);
    }

    return true;
}

fn fix_line(line: &String, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let values: Vec<&str> = line.split(',').collect();
    for value in values {
        let i_value = value.parse::<i32>().unwrap();
        result.push(i_value);
    }
    println!("{:?}", result);

    result.sort_by(
        |a, b| {
            // Do we have a rule for this value?
            if rules.contains_key(a) {
                let rule = rules.get(a).unwrap();

                // Does the rule reference the other item?
                if rule.contains(b) {
                    // If so, need to be BEFORE it
                    return Ordering::Less;
                }
            }

            return Ordering::Greater;
        }
    );

    println!("{:?}", result);
        
    return result;
}


fn get_middle_value(line: &Vec<i32>) -> i32 {
    let idx = (line.len() - 1) / 2;
    return line[idx];
}