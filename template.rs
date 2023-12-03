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

    while let Some(Ok(line)) = &lines.next() {
        if line == "" {
            continue; // * Ignore empty lines
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
