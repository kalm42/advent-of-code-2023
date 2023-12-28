use std::cmp::{min, Ordering};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::time::Instant;

// 251637622 is too high
// 250757288

fn main() {
    let start = Instant::now();

    let lines = match read_lines("./input.txt") {
        Ok(l) => l,
        Err(e) => panic!("Error: {}", e),
    };

    let lines: Vec<_> = lines.collect();

    let winnings = calculate_winnings(&lines);
    let winnings_2 = calculate_winnings_part_2(&lines);

    let duration = start.elapsed();
    println!("Part 1: {}", winnings);
    println!("Part 2: {}", winnings_2);
    println!("Time elapsed is: {:?}", duration);
}

/*******************************************************************************
 * Not day specific functions
 */
/**
 * Read a file line by line.
 */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/**
 * Given a string, return a vector of numbers
 * line: String - the line to parse
 * delimiters: &str - the delimiters to split on, ex: " :," (space, colon, and comma)
 */
// fn get_numbers(line: String, delimiters: &str) -> Vec<i32> {
//     // Has a title, need to remove it then split on the number delimiter
//     // ex: Time:      7  15   30
//     return line
//         .split(|c| delimiters.contains(c))
//         .filter_map(|s| s.parse::<i32>().ok())
//         .collect::<Vec<i32>>();
// }

fn get_whatever(line: &String, delimiters: &str) -> Vec<String> {
    return line
        .split(|c| delimiters.contains(c))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
}

/**
 * Binary search for lower bound of curve where 't' is a constant and 'w' is the
 * left/right bound.
 */
// fn get_lower_bound(t: i64, w: i64) -> i64 {
//     let mut low = 0;
//     let mut high = t;

//     while low < high {
//         let mid = (low + high) / 2;

//         let d = mid * (t - mid);

//         if d < w {
//             low = mid + 1;
//         } else {
//             high = mid;
//         }
//     }

//     if low * (t - low) > w {
//         return low;
//     } else {
//         return low - 1;
//     }
// }
/**
 * Binary search for upper bound of curve where 't' is a constant and 'w' is the
 * left/right bound.
 */
// fn get_upper_bound(t: i64, w: i64) -> i64 {
//     let mut low = 0;
//     let mut high = t;

//     while low < high {
//         let mid = low + (high - low) / 2;

//         let d = mid * (t - mid);

//         if d < w {
//             high = mid;
//         } else {
//             low = mid + 1;
//         }
//     }

//     if low * (t - low) > w {
//         return low;
//     } else {
//         return low - 1;
//     }
// }

/*******************************************************************************
 * Day specific functions: Part 1
 */
fn calculate_winnings(lines: &Vec<Result<String, Error>>) -> i32 {
    let hands: Vec<(String, i32)> = get_hands_with_bid(lines);

    let sorted_hands = sort_hands(&hands, None, None);

    let mut winnings = 0;
    let mut i = 0;
    while i < sorted_hands.len() {
        let hand_value = sorted_hands[i].1 * (i as i32 + 1);
        winnings += hand_value;
        i += 1;
    }

    return winnings;
}

fn get_card_strength() -> HashMap<char, i32> {
    return [
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]
    .iter()
    .cloned()
    .collect();
}

fn get_hands_with_bid(lines: &Vec<Result<String, Error>>) -> Vec<(String, i32)> {
    let mut hands: Vec<(String, i32)> = Vec::new();

    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(e) => panic!("Error: {}", e),
        };

        let deets = get_whatever(line, " :,");

        let bet = match deets[1].parse::<i32>() {
            Ok(b) => b,
            Err(e) => panic!("Error: {}", e),
        };

        hands.push((deets[0].to_string(), bet));
    }

    return hands;
}

fn sort_hands(
    hands: &Vec<(String, i32)>,
    determine_hand_type: Option<fn(a: &str) -> i32>,
    compare_cards_2: Option<fn(a: &String, b: &String) -> Ordering>,
) -> Vec<(String, i32)> {
    let mut sorted_hands = hands.clone();

    sorted_hands.sort_by(|a, b| sort_hand(a, b, determine_hand_type, compare_cards_2));

    return sorted_hands;
}

fn sort_hand(
    a: &(String, i32),
    b: &(String, i32),
    determine_hand_type: Option<fn(a: &str) -> i32>,
    compare_cards_2: Option<fn(a: &String, b: &String) -> Ordering>,
) -> Ordering {
    let a = &a.0;
    let b = &b.0;

    let a_type = match determine_hand_type {
        Some(f) => f(a),
        None => get_hand_type(a),
    };
    let b_type = match determine_hand_type {
        Some(f) => f(b),
        None => get_hand_type(b),
    };

    if a_type == b_type {
        return match compare_cards_2 {
            Some(f) => f(a, b),
            None => compare_cards(a, b),
        };
    }

    if a_type > b_type {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

fn compare_cards(a: &String, b: &String) -> Ordering {
    let mut i = 0;
    let mut response = Ordering::Equal;
    while i < 5 {
        let a_card = match a.chars().nth(i) {
            Some(c) => c,
            None => panic!("Invalid card"),
        };
        let b_card = match b.chars().nth(i) {
            Some(c) => c,
            None => panic!("Invalid card"),
        };
        match sort_card(a_card, b_card) {
            Ordering::Equal => {
                i += 1;
            }
            Ordering::Less => {
                response = Ordering::Less;
                break;
            }
            Ordering::Greater => {
                response = Ordering::Greater;
                break;
            }
        }
    }
    return response;
}

fn get_hand_type(hand: &str) -> i32 {
    let mut hand_value: HashMap<char, i32> = HashMap::new();

    for c in hand.chars() {
        let count = hand_value.entry(c).or_insert(0);
        *count += 1;
    }

    return match hand_value.len() {
        1 => 7, // 5 of a kind
        2 => {
            if hand_value.values().any(|&x| x == 4) {
                6 // 4 of a kind
            } else {
                5 // Full house
            }
        }
        3 => {
            if hand_value.values().any(|&x| x == 3) {
                4 // 3 of a kind
            } else {
                3 // Two pair
            }
        }
        4 => 2, // One pair
        5 => 1, // High card
        _ => panic!("Invalid hand"),
    };
}

fn sort_card(a: char, b: char) -> Ordering {
    let card_strength = get_card_strength();

    let a_strength = match card_strength.get(&a) {
        Some(s) => s,
        None => panic!("Invalid card"),
    };

    let b_strength = match card_strength.get(&b) {
        Some(s) => s,
        None => panic!("Invalid card"),
    };

    return a_strength.cmp(b_strength);
}

/*******************************************************************************
 * Day specific functions: Part 2
 */
fn calculate_winnings_part_2(lines: &Vec<Result<String, Error>>) -> i32 {
    let hands: Vec<(String, i32)> = get_hands_with_bid(lines);

    let sorted_hands = sort_hands(&hands, Some(get_hand_type_2), Some(compare_cards_2));

    let mut winnings = 0;
    let mut i = 0;
    while i < sorted_hands.len() {
        let hand_value = sorted_hands[i].1 * (i as i32 + 1);
        winnings += hand_value;
        i += 1;
    }

    return winnings;
}

fn get_hand_type_2(hand: &str) -> i32 {
    let mut hand_value: HashMap<char, i32> = HashMap::new();

    for c in hand.chars() {
        let count = hand_value.entry(c).or_insert(0);
        *count += 1;
    }

    if hand.contains("J") {
        // We have a joker, and should increase the count of the heighest card
        // Find the highest card count.
        let mut highest_count = 0;
        let mut card_face = 'A';
        for (&face, &count) in &hand_value {
            if count > highest_count && face != 'J' {
                highest_count = count;
                card_face = face;
            }
        }
        if highest_count > 0 {
            let joker_count = match hand_value.get(&'J') {
                Some(c) => *c,
                None => panic!("Invalid hand"),
            };
            let chosen_card = match hand_value.get_mut(&card_face) {
                Some(c) => c,
                None => panic!("Invalid hand"),
            };
            *chosen_card = min(5, *chosen_card + joker_count);

            // Remove the joker from the hand
            hand_value.remove(&'J');
        }
    }

    return match hand_value.len() {
        1 => 7, // 5 of a kind
        2 => {
            if hand_value.values().any(|&x| x == 4) {
                6 // 4 of a kind
            } else {
                5 // Full house
            }
        }
        3 => {
            if hand_value.values().any(|&x| x == 3) {
                4 // 3 of a kind
            } else {
                3 // Two pair
            }
        }
        4 => 2, // One pair
        5 => 1, // High card
        _ => panic!("Invalid hand"),
    };
}

fn compare_cards_2(a: &String, b: &String) -> Ordering {
    let mut i = 0;
    let mut response = Ordering::Equal;
    while i < 5 {
        let a_card = match a.chars().nth(i) {
            Some(c) => c,
            None => panic!("Invalid card"),
        };
        let b_card = match b.chars().nth(i) {
            Some(c) => c,
            None => panic!("Invalid card"),
        };
        match sort_card_2(a_card, b_card) {
            Ordering::Equal => {
                i += 1;
            }
            Ordering::Less => {
                response = Ordering::Less;
                break;
            }
            Ordering::Greater => {
                response = Ordering::Greater;
                break;
            }
        }
    }
    return response;
}

fn sort_card_2(a: char, b: char) -> Ordering {
    let card_strength = get_card_strength_2();

    let a_strength = match card_strength.get(&a) {
        Some(s) => s,
        None => panic!("Invalid card"),
    };

    let b_strength = match card_strength.get(&b) {
        Some(s) => s,
        None => panic!("Invalid card"),
    };

    return a_strength.cmp(b_strength);
}

fn get_card_strength_2() -> HashMap<char, i32> {
    return [
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]
    .iter()
    .cloned()
    .collect();
}
