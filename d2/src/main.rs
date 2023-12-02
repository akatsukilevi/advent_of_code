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

    let mut possible_games: Vec<i32> = Vec::new();
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
        let mut is_game_possible = true;

        println!("Handling game {}: {:?}", &game_id, &line);
        let mut turn_count = 1;
        for turn_raw in &turns {
            let turn = turn_raw.trim();

            let mut colors: HashMap<Color, Option<i32>> = HashMap::new();
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
                colors
                    .entry(turn_color)
                    .and_modify(|x| *x = Some(x.unwrap() + num));
            }

            // * Now, compare against the available colors
            let mut is_possible = true;
            if let Some(red_color) = colors.entry(Color::Red).or_default() {
                if red_color > &mut 12 {
                    println!(
                        "Turn {} on game {} is not possible: There are {} red on turn out of 12",
                        &turn_count, &game_id, red_color
                    );

                    is_possible = false;
                }
            };

            if let Some(green_color) = colors.entry(Color::Green).or_default() {
                if green_color > &mut 13 {
                    println!(
                        "Turn {} on game {} is not possible: There are {} green on turn out of 13",
                        &turn_count, &game_id, green_color
                    );

                    is_possible = false;
                }
            };

            if let Some(blue_color) = colors.entry(Color::Blue).or_default() {
                if blue_color > &mut 14 {
                    println!(
                        "Turn {} on game {} is not possible: There are {} blue out of 14",
                        &turn_count, &game_id, blue_color
                    );

                    is_possible = false;
                }
            };

            if !is_possible {
                println!("Turn {} on game {} is impossible", &turn_count, &game_id);
                is_game_possible = false;
                break;
            }

            turn_count += 1;
        }

        if is_game_possible {
            println!("Game {} is possible", &game_id);
            possible_games.push(game_id);
        }

        println!("");
    }

    println!(
        "{} games are possible, sum: {}",
        &possible_games.len(),
        &possible_games.iter().sum::<i32>()
    );
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
