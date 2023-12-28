use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let lines = match read_lines("./input.txt") {
        Ok(l) => l,
        Err(e) => panic!("Error: {}", e),
    };

    let lines: Vec<_> = lines.collect();

    let race_data = extract_race_data(&lines);
    let race_data_2 = extract_the_race_data(&lines);

    let win_path_count_product = get_product_of_win_path_count(race_data);
    let win_path_count = get_win_path_count_binary(race_data_2);
    println!("Win path count: {}", win_path_count);

    let duration = start.elapsed();
    println!("Part 1: {}", win_path_count_product);
    println!("Part 2: {}", win_path_count);
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
fn get_numbers(line: String, delimiters: &str) -> Vec<i32> {
    // Has a title, need to remove it then split on the number delimiter
    // ex: Time:      7  15   30
    return line
        .split(|c| delimiters.contains(c))
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();
}

/**
 * Binary search for lower bound of curve where 't' is a constant and 'w' is the
 * left/right bound.
 */
fn get_lower_bound(t: i64, w: i64) -> i64 {
    let mut low = 0;
    let mut high = t;

    while low < high {
        let mid = (low + high) / 2;

        let d = mid * (t - mid);

        if d < w {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    if low * (t - low) > w {
        return low;
    } else {
        return low - 1;
    }
}
/**
 * Binary search for upper bound of curve where 't' is a constant and 'w' is the
 * left/right bound.
 */
fn get_upper_bound(t: i64, w: i64) -> i64 {
    let mut low = 0;
    let mut high = t;

    while low < high {
        let mid = low + (high - low) / 2;

        let d = mid * (t - mid);

        if d < w {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    if low * (t - low) > w {
        return low;
    } else {
        return low - 1;
    }
}

/*******************************************************************************
 * Day specific functions: Part 1
 */
fn extract_race_data(lines: &Vec<Result<String, Error>>) -> Vec<(i32, i32)> {
    let mut race_data: Vec<(i32, i32)> = Vec::new();
    let mut time_collection: Vec<i32> = Vec::new();
    let mut distance_collection: Vec<i32> = Vec::new();

    if let Ok(line) = &lines[0] {
        let time = get_numbers(line.to_string(), ": ");
        time_collection.extend(time);
    }

    if let Ok(line) = &lines[1] {
        let distance = get_numbers(line.to_string(), ": ");
        distance_collection.extend(distance);
    }

    if time_collection.len() == distance_collection.len() {
        let mut i = 0;
        for time in time_collection {
            let distance = distance_collection[i];
            race_data.push((time, distance));
            i += 1;
        }
    }

    return race_data;
}

fn get_product_of_win_path_count(race_data: Vec<(i32, i32)>) -> i32 {
    let mut product = 1;

    for race in race_data {
        let win_count = get_win_path_count(race);
        product *= win_count;
    }

    return product;
}

fn get_win_path_count(race: (i32, i32)) -> i32 {
    let mut win_path_count = 0;

    // time is in ms, distance is in mm
    let (time, distance) = race;

    // For each ms of time spent charging the velocity increases by 1 mm/ms
    let mut i = 0;
    while i < time {
        if i * (time - i) > distance {
            win_path_count += 1;
        }

        i += 1;
    }

    return win_path_count;
}

/*******************************************************************************
 * Day specific functions: Part 2
 */
fn extract_the_race_data(lines: &Vec<Result<String, Error>>) -> (i64, i64) {
    let mut time: i64 = 0;
    let mut distance: i64 = 0;

    if let Ok(line) = &lines[0] {
        let time_split = get_numbers(line.to_string(), ": ");
        if let Some(t) = time_split
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i64>()
            .ok()
        {
            time = t;
        }
    }

    if let Ok(line) = &lines[1] {
        let distance_split = get_numbers(line.to_string(), ": ");
        if let Some(d) = distance_split
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i64>()
            .ok()
        {
            distance = d;
        }
    }

    return (time, distance);
}

fn get_win_path_count_binary(race_data: (i64, i64)) -> i32 {
    // let mut win_path_count = 0;

    let (time, distance) = race_data;

    // Going to do two binary searches, one for the lower bound and one for the
    // upper bound. Then subtract the two to get the number of win paths.
    let lower_bound = get_lower_bound(time, distance);
    let upper_bound = get_upper_bound(time, distance);

    println!("Lower bound: {}", lower_bound);
    println!("Upper bound: {}", upper_bound);

    let count: i32 = (upper_bound - lower_bound + 1) as i32; // because we want to include the bounds

    return count;
}
