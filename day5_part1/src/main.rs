use std::{fs::File, io::{self, BufRead}};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

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
    if line_is_valid(line, rules) {
        return Some(get_middle_value(line));
    }

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

fn get_middle_value(line: &String) -> i32 {
    let values: Vec<&str> = line.split(',').collect();
    let idx = (values.len() - 1) / 2;
    return values[idx].parse::<i32>().unwrap();
}