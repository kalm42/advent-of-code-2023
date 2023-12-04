use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut game_index_sum_part_one = 0;
    let mut game_power_part_two = 0;
    let cubes_part_one: HashMap<&str, i32> = [("red", 12), ("green", 13), ("blue", 14)].into();

    let lines = match read_lines("./games.txt") {
        Ok(lines) => lines,
        Err(e) => panic!("Error: {}", e),
    };

    for line in lines {
        if let Ok(line) = line {
            let (result, power) = analyze_game(line, &cubes_part_one);
            game_index_sum_part_one += result;
            game_power_part_two += power;
        }
    }

    let duration = start.elapsed();
    println!("Part 1 game index sum is: {}", game_index_sum_part_one);
    println!("Part 2 game power is: {}", game_power_part_two);
    println!("Time elapsed is: {:?}", duration);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// returns the index of the game if valid or 0 if not valid
fn analyze_game(game: String, base: &HashMap<&str, i32>) -> (i32, i32) {
    let mut game_index = 0;
    let mut game_power = 1;
    let mut max_cubes: HashMap<&str, i32> = [("red", 0), ("green", 0), ("blue", 0)].into();

    // I need the game index as a number, i32
    if let Ok(r) = Regex::new(r"Game (\d+)") {
        if let Some(c) = r.captures(&game) {
            game_index = match c.get(1) {
                Some(x) => match x.as_str().parse::<i32>() {
                    Ok(x) => x,
                    Err(_) => 0,
                },
                None => 0,
            };
        }
    }

    // the values of the cubes per pull
    if let Ok(r) = Regex::new(r"((\d+) (\w+))") {
        for cap in r.captures_iter(&game) {
            let color = match cap.get(3) {
                Some(x) => x.as_str(),
                None => "",
            };
            let value = match cap.get(2) {
                Some(x) => match x.as_str().parse::<i32>() {
                    Ok(x) => x,
                    Err(_) => 0,
                },
                None => 0,
            };

            // if this exceeds the base value then it is not possible, set index to 0 and return
            let base_value = match base.get(color) {
                Some(x) => *x,
                None => 0,
            };

            if value > max_cubes[color] {
                max_cubes.insert(color, value);
            }

            if value > base_value && game_index != 0 {
                game_index = 0;
            }
        }
    }

    // calculate the power of the game
    for (_color, value) in max_cubes.iter() {
        if value > &0 {
            game_power *= value;
        }
    }

    return (game_index, game_power);
}
