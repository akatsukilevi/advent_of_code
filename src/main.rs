use regex::Regex;
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

    let re = Regex::new(r"[^0-9]").unwrap();
    let mut count = 0;

    while let Some(Ok(line)) = &lines.next() {
        // * Remove all non number characters
        let clean = re.replace_all(line, "");
        if clean == "" {
            continue; // * Ignore empty lines
        }

        let first = clean.chars().next().unwrap();
        let last = clean.chars().nth_back(0).unwrap();
        let number_str = format!("{}{}", first, last);

        let Ok(number) = number_str.parse::<i32>() else {
            println!(
                "Failed to parse line {}: Invalid number {}",
                &clean, &number_str
            );
            continue;
        };

        count = count + number;
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

