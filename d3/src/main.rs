use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn collect_number(line: &str) -> Option<(i32, i32, i32)> {
    let mut has_found_number = false;
    let mut start_set = false;
    let mut end_set = false;

    let mut num_start: i32 = 0;
    let mut num_end: i32 = 0;

    let mut num_str = String::from("");

    // * Iterate through the entire line and find a number
    for (i, curr_char) in line.char_indices() {
        if curr_char == '.' || !curr_char.is_digit(10) {
            // * Is at a dot or symbol of sorts
            if has_found_number {
                break; // * Completed a number, stop here
            } else {
                continue; // * No numbers, keep searching
            }
        } else if let Some(num) = curr_char.to_digit(10) {
            // * Is a digit! update the indexes
            match start_set {
                false => {
                    num_start = i as i32;
                    start_set = true;
                }
                true => {
                    num_end = i as i32;
                    end_set = true;
                }
            }

            // * Register we found a number
            has_found_number = true;

            // * Update the found number
            num_str += &num.to_string();
        }
    }

    if !end_set {
        num_end = num_start;
    }

    if has_found_number {
        let Ok(num) = num_str.parse::<i32>() else {
            println!("Failed to parse found number {}!", num_str);
            return None;
        };

        return Some((num_start, num_end, num));
    }

    None
}

fn collect_all_numbers(line: &str) -> HashMap<(i32, i32), i32> {
    let mut numbers: HashMap<(i32, i32), i32> = HashMap::new();

    let mut current_line = line.clone();
    let mut offset: i32 = 0;

    while let Some((start, end, number)) = collect_number(current_line) {
        // * Register the number
        let index = (start + offset, end + offset);
        numbers.insert(index, number);

        offset += end + 1;
        if offset >= (current_line.len() as i32) {
            break; // * Stop if we already hit the end
        }

        // * Update the line
        current_line = &current_line[(offset as usize)..(current_line.len() - 1)];
    }

    println!("Found numbers: {:?}", &numbers);
    numbers
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Specify a input file");
        return;
    }

    let Some(filename) = args.pop() else {
        panic!("Failed to get filename!");
    };

    println!("Processing file {:?}", &filename);

    let Ok(lines) = read_lines(&filename) else {
        panic!("Failed to read file {}", &filename);
    };

    let lines_iter: Vec<Result<String, std::io::Error>> = lines.collect();
    let mut numbers: Vec<i32> = Vec::new();
    let symbol_finder = Regex::new(r"[^0-9\.\n]").unwrap();

    let mut index = 0;
    for line_res in &lines_iter {
        let Ok(line) = line_res else {
            panic!("Failed to read line!");
        };

        if line == "" {
            continue; // * Ignore empty lines
        }

        let line_numbers = collect_all_numbers(&line);
        let line_max = (line.len() - 1) as i32;

        let mut previous_line: Option<&str> = None;
        let mut next_line: Option<&str> = None;

        if index != 0 {
            // * Get the previous line
            if let Some(Ok(ln)) = &lines_iter.get(index - 1) {
                previous_line = Some(ln);
            }
        }

        // * Get the next line
        if let Some(Ok(ln)) = &lines_iter.get(index + 1) {
            next_line = Some(ln);
        }

        // * For every number found, check if there is a adjacent symbol
        // * Symbol(x) = !x.is_digit(10) && x != '.'

        for (coordinates, number) in &line_numbers {
            let (start, end) = coordinates;
            println!("Checking number {} at {:?}", &number, &coordinates);

            let mut block = String::from("");

            // * Should check if any of the characters matches
            let fetch_start = match start {
                x if x != &0 => x - 1,
                _ => start.clone(),
            };

            let fetch_end = match end {
                x if x != &line_max => x + 2,
                _ => line_max,
            };

            if let Some(prev_line) = &previous_line {
                // * Should fetch this slice
                let slice = &prev_line[(fetch_start as usize)..(fetch_end as usize)];
                println!(
                    "Previous line slice({}..{}): {:?}",
                    &fetch_start, &fetch_end, &slice
                );
                block += slice;
                block += "\n";
            } else {
                println!("Has no previous line, ignoring");
            }

            let curr_slice = &line[(fetch_start as usize)..(fetch_end as usize)];
            println!(
                "Current line slice({}..{}): {:?}",
                &fetch_start, &fetch_end, &curr_slice
            );
            block += curr_slice;
            block += "\n";

            if let Some(next_line) = &next_line {
                // * Should fetch this slice
                let slice = &next_line[(fetch_start as usize)..(fetch_end as usize)];
                println!(
                    "Next line slice({}..{}): {:?}",
                    &fetch_start, &fetch_end, &slice
                );
                block += slice;
                block += "\n";
            } else {
                println!("Has no next line, ignoring")
            }

            println!("Final block:\n{}", block);

            if symbol_finder.is_match(&block) {
                // * Is a symbol! Can add it
                println!(
                    "Number {} at {:?} is a valid part number (Previous Line)",
                    &number, &coordinates
                );
                println!("");
                numbers.push(number.clone());
                continue;
            }
        }

        index += 1;
        println!("");
    }

    println!("Final sum: {}", numbers.iter().sum::<i32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
