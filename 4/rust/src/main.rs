use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

// 21228 is too high
fn main() {
    let start = Instant::now();

    let mut card_number = 1;
    let mut card_numbers: HashMap<i32, i32> = HashMap::new();

    let mut point_sum_part_one = 0;
    let mut card_count_part_two = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for l in lines {
            // println!(
            //     "Card number: {}, copy count: {:?}, part 2: {}",
            //     card_number, card_numbers, card_count_part_two
            // );
            let line = match l {
                Ok(line) => line,
                Err(e) => panic!("Error: {}", e),
            };
            // println!("Line: {}", line);

            let (winning_numbers, my_numbers) = get_numbers(&line);

            let count = count_matches(winning_numbers, my_numbers);
            // println!("Win count: {}", count);
            // This card has won, so we make copys of that many cards

            let copy_card_count = match card_numbers.get(&card_number) {
                Some(count) => *count,
                None => 0,
            };

            for i in 1..=count {
                let copy_count = card_numbers.entry(card_number + i).or_insert(0);
                *copy_count += 1;
            }

            for _ in 0..copy_card_count {
                for i in 1..=count {
                    let copy_count = card_numbers.entry(card_number + i).or_insert(0);
                    *copy_count += 1;
                }
            }

            let score = if count == 0 {
                0
            } else {
                2_i32.pow(count as u32 - 1)
            };

            // println!("Score: {}", score);
            point_sum_part_one += score;

            // Process the original card, plus any copies
            let copy_count = match card_numbers.get(&card_number) {
                Some(count) => *count,
                None => 0,
            };
            card_count_part_two += 1 + copy_count;
            // Finished processing card
            card_number += 1;
        }
    }

    let duration = start.elapsed();
    println!("Part 1 Sum is: {}", point_sum_part_one);
    println!("Part 2 Count is: {}", card_count_part_two);
    println!("Time elapsed is: {:?}", duration);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_numbers(line: &String) -> (Vec<i32>, Vec<i32>) {
    let split: Vec<&str> = line.split(":").collect();
    let all_numbers = split[1..].join("");
    let sub_split: Vec<&str> = all_numbers.split("|").collect();
    let winning_numbers_string = sub_split[0];
    let my_numbers_string = sub_split[1];

    let winning_collection = get_numbers_from_string(winning_numbers_string);
    let my_collection = get_numbers_from_string(my_numbers_string);

    return (winning_collection, my_collection);
}

fn get_numbers_from_string(line: &str) -> Vec<i32> {
    return line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
}

fn count_matches(winning_numbers: Vec<i32>, my_number: Vec<i32>) -> i32 {
    let mut count = 0;
    for winning_number in winning_numbers {
        for my_number in &my_number {
            if winning_number == *my_number {
                count += 1;
            }
        }
    }
    return count;
}
