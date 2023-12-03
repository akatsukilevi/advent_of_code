use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn collect_number(line: &str) -> Option<(i32, i32, i32)> {
    let mut num_start: Option<usize> = None;
    let mut num_str = String::new();

    for (i, curr_char) in line.char_indices() {
        if curr_char.is_digit(10) {
            // If the number has not started yet, set the start index
            if num_start.is_none() {
                num_start = Some(i);
            }

            // Add the digit to the number string
            num_str.push(curr_char);
        } else if let Some(start) = num_start {
            // If the number had started and a non-digit is encountered, set end index and break
            return match num_str.parse::<i32>() {
                Ok(num) => Some((start as i32, i as i32, num)),
                Err(_) => {
                    println!("Failed to parse found number: {}", num_str);
                    None
                }
            };
        }
    }

    if let Some(start) = num_start {
        if let Ok(num) = num_str.parse::<i32>() {
            return Some((start as i32, (line.len()) as i32, num));
        } else {
            println!("Failed to parse found number: {}", num_str);
        }
    }

    None
}

fn collect_all_numbers(line: &str) -> HashMap<(i32, i32), i32> {
    let mut numbers: HashMap<(i32, i32), i32> = HashMap::new();

    let mut current_line = line.clone();
    println!("Line: {:?} - {}", &line, &line.len());

    let mut offset: usize = 0;
    while let Some((start, end, number)) = collect_number(current_line) {
        let index = (start + (offset as i32), end + (offset as i32));
        numbers.insert(index, number);

        current_line = &current_line[(end as usize)..]; // Update the line for the next iteration
        offset += end as usize;

        println!("Collected number {} at {:?}", &number, &index);
        println!("Line: {:?} - {}", &current_line, &current_line.len());

        if (end as usize) >= line.len() {
            break; // Stop if we're at the end
        }
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
                if ln != "" {
                previous_line = Some(ln);
                }
            }
        }

        // * Get the next line
        if let Some(Ok(ln)) = &lines_iter.get(index + 1) {
            if ln != "" {
                next_line = Some(ln);
            }
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

            let mut fetch_end = match end {
                x if x != &line_max => x + 2,
                _ => line_max,
            };

            if fetch_end >= line_max {
                fetch_end = line_max;
            }

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
