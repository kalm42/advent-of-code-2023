use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut sum_part_one = 0;
    let mut sum_part_two = 0;
    if let Ok(lines) = read_lines("./calibrate.txt") {
        for line in lines {
            let freq_one = part_one(&line);
            let freq_two = part_two(&line);
            sum_part_one += freq_one;
            sum_part_two += freq_two;
        }
    }

    let duration = start.elapsed();
    println!("Part 1 Sum is: {}", sum_part_one);
    println!("Part 2 Sum is: {}", sum_part_two);
    println!("Time elapsed is: {:?}", duration);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one(line: &Result<String, Error>) -> i32 {
    let freqs: Vec<i32> = line
        .as_ref()
        .ok()
        .map(|freq_jumble| {
            freq_jumble
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as i32))
                .collect()
        })
        .unwrap_or_else(Vec::new);

    return match freqs.len() {
        0 => 0,
        1 => format!("{0}{0}", freqs[0]).parse::<i32>().unwrap_or(0),
        _ => format!("{}{}", freqs[0], freqs[freqs.len() - 1])
            .parse::<i32>()
            .unwrap_or(0),
    };
}

use std::collections::HashMap;

fn part_two(line: &Result<String, Error>) -> i32 {
    let mut word_freqs = Vec::new();
    let alpha_numbers: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into();
    let mut pointer = 0;

    if let Ok(freq_jumble) = line {
        while freq_jumble.len() > pointer {
            let substr = &freq_jumble[pointer..];

            // is the first character a number?
            if let Some(num) = substr.chars().next().unwrap().to_digit(10) {
                word_freqs.push(num as i32);
            } else {
                // if not is this a word?
                for (word, &num) in alpha_numbers.iter() {
                    if substr.starts_with(word) {
                        word_freqs.push(num);
                        break;
                    }
                }
            }
            pointer += 1;
        }
    }

    match word_freqs.len() {
        0 => 0,
        1 => format!("{0}{0}", word_freqs[0]).parse::<i32>().unwrap_or(0),
        2 => format!("{}{}", word_freqs[0], word_freqs[1])
            .parse::<i32>()
            .unwrap_or(0),
        _ => format!("{}{}", word_freqs[0], word_freqs[word_freqs.len() - 1])
            .parse::<i32>()
            .unwrap_or(0),
    }
}
