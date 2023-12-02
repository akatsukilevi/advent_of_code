use regex::Regex;
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

    let mut powers: Vec<i32> = Vec::new();
    let nan_regex = Regex::new(r"[^0-9]").unwrap();

    while let Some(Ok(line)) = &lines.next() {
        if line == "" {
            continue; // * Ignore empty lines
        }

        // * First, grab the ID of this game
        // * Split the entire text by the ':', remove 'Game ' then parse as a number
        let mut splitted = line.split(':');
        let Ok(game_id) = splitted.next().unwrap().replace("Game ", "").parse::<i32>() else {
            println!("Failed to parse ID for game {}", &line);
            continue;
        };

        let turns: Vec<&str> = splitted.next().unwrap().split(';').collect();
        let mut colors: HashMap<Color, Option<i32>> = HashMap::new();

        println!("Handling game {}: {:?}", &game_id, &line);
        let mut turn_count = 1;
        for turn_raw in &turns {
            let turn = turn_raw.trim();

            let steps: Vec<&str> = turn.split(',').collect();
            println!(
                "Game {} on turn {} has {} steps: {:?}",
                &game_id,
                &turn_count,
                &steps.len(),
                &turn
            );
            for step in &steps {
                // * Get only the number in here
                let Ok(num) = nan_regex.replace_all(&step, "").parse::<i32>() else {
                    println!(
                        "Failed to parse step {} for turn {} in game {}",
                        &step, &turn_count, &game_id
                    );
                    continue;
                };

                let turn_color = match step.replace(&format!("{} ", &num), "").trim() {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    x => {
                        println!(
                            "Step {} in turn {} for game {} has invalid color {}",
                            &step, &turn_count, &game_id, x
                        );
                        continue;
                    }
                };

                // * Make sure it exists
                colors.entry(turn_color).or_insert_with(|| Some(0));

                // * Update the value
                colors.entry(turn_color).and_modify(|x| {
                    let existing = x.unwrap();

                    if existing >= num {
                        *x = Some(existing);
                        return;
                    }

                    *x = Some(num)
                });
            }

            turn_count += 1;
        }

        let Some(Some(red_count)) = colors.get(&Color::Red) else {
            println!("Game {} has yielded no red!", &game_id);
            continue;
        };

        let Some(Some(green_count)) = colors.get(&Color::Green) else {
            println!("Game {} has yielded no green!", &game_id);
            continue;
        };

        let Some(Some(blue_count)) = colors.get(&Color::Blue) else {
            println!("Game {} has yielded no blue!", &game_id);
            continue;
        };

        let power = red_count * green_count * blue_count;
        println!("Game {} has power of {}", &game_id, &power);
        powers.push(power);

        println!("");
    }

    println!("Sum of powers: {}", powers.iter().sum::<i32>())
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
