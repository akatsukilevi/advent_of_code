use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

    let Ok(mut lines) = read_lines(&filename) else {
        panic!("Failed to read file {}", &filename);
    };

    let mut matcher: HashMap<&str, i32> = HashMap::new();
    let mut count = 0;

    matcher.insert("one", 1);
    matcher.insert("two", 2);
    matcher.insert("three", 3);
    matcher.insert("four", 4);
    matcher.insert("five", 5);
    matcher.insert("six", 6);
    matcher.insert("seven", 7);
    matcher.insert("eight", 8);
    matcher.insert("nine", 9);

    while let Some(Ok(line)) = &lines.next() {
        if line == "" {
            continue; // * Ignore empty lines
        }

        // * Iterate through the entire line from left to right
        let mut first: Option<i32> = None;
        let mut last: Option<i32> = None;

        let mut acc = String::from("");
        let mut first_chars = line.chars();
        while let Some(first_char) = first_chars.next() {
            acc = acc + &first_char.to_string();

            if let Some(first_num) = first_char.to_digit(10) {
                first = Some(first_num.try_into().unwrap());
                break;
            }

            for (k, v) in &matcher {
                if acc.contains(k) {
                    first = Some(v.clone());
                    break;
                }
            }

            if first.is_some() {
                break;
            }
        }

        acc = String::from("");
        let mut last_chars = line.chars();
        while let Some(last_char) = last_chars.next_back() {
            acc = last_char.to_string() + &acc;

            if let Some(last_num) = last_char.to_digit(10) {
                last = Some(last_num.try_into().unwrap());
                break;
            }

            for (k, v) in &matcher {
                if acc.contains(k) {
                    last = Some(v.clone());
                    break;
                }
            }

            if last.is_some() {
                break;
            }
        }

        let Some(first_num) = first else {
            continue;
        };

        let Some(last_num) = last else {
            continue;
        };

        let number_str = format!("{}{}", first_num, last_num);
        let Ok(number) = number_str.parse::<i32>() else {
            println!(
                "Failed to parse line {}: Invalid number {}",
                &line, &number_str
            );
            continue;
        };

        count = count + number;
        println!(
            "Line {} - Number: {} - Counter: {}",
            &line, &number_str, &count
        );
    }

    println!("Result: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

