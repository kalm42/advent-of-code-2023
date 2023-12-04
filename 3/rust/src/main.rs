use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut y_cursor = 0;
    let mut part_numer_sum = 0;

    // For each number found is there a non number, non letter, within 1 character space of it above, same, and below?
    // If so, add it to the sum.
    let lines = match read_lines("./input.txt") {
        Ok(lines) => lines,
        Err(e) => panic!("Error reading file: {}", e),
    };

    let lines: Vec<_> = lines.collect();
    // println!("Number of lines: {}", lines.len());

    let x_bound = match lines.get(0) {
        Some(line) => match line {
            Ok(l) => l.len(),
            Err(e) => panic!("Error reading line: {}", e),
        },
        None => panic!("No lines found"),
    };
    let y_bound = lines.len();

    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    while y_cursor < y_bound {
        let mut x_cursor = 0;
        let line = get_line(y_cursor, &lines);
        // println!("Line {}: {}", y_cursor + 1, line);

        while x_cursor < x_bound {
            // Does the line contain a number?
            let substr = &line[x_cursor..];
            // println!("Rest of line to check: {}", substr);

            match substr.find(|c: char| c.is_numeric()) {
                Some(i) => {
                    let x1_cursor = i + x_cursor;
                    let mut x2_cursor = i + x_cursor;
                    // println!("Found number at index: {}", i);
                    // println!("Subsubstr: {}", &line[x1_cursor..]);

                    // Find the first non number in the line
                    while x2_cursor < x_bound {
                        let c = match line.chars().nth(x2_cursor) {
                            Some(c) => c,
                            None => panic!("No character found at index: {}", x2_cursor),
                        };
                        if !c.is_numeric() {
                            break;
                        }
                        x2_cursor += 1;
                    }

                    // println!(
                    //     "Captured number: {}\nChecking if it is a part number.",
                    //     &line[x1_cursor..x2_cursor]
                    // );

                    if is_part_number(x1_cursor, x2_cursor, y_cursor, &lines) {
                        let subsubsubstr = &line[x1_cursor..x2_cursor];
                        // println!("Found part number: {}", subsubsubstr);
                        let part_number = match subsubsubstr.parse::<i32>() {
                            Ok(n) => n,
                            Err(e) => panic!("Error parsing number: {}", e),
                        };

                        // We know it's a part number, so check if it could be a gear mapped by the * coordinate
                        if let Some(coord) =
                            is_possible_gear(x1_cursor, x2_cursor, y_cursor, &lines)
                        {
                            match gears.get_mut(&coord) {
                                Some(gear) => gear.push(part_number),
                                None => {
                                    gears.insert(coord, vec![part_number]);
                                }
                            }
                        }
                        part_numer_sum += part_number;
                    }
                    x_cursor = x2_cursor + 1; // because it's going to be added back in at the end of this loop
                }
                None => {
                    // println!("No number found");
                    // There is no number in the line, so set the cursor to the end of the line
                    x_cursor = x_bound;
                }
            }
        }

        y_cursor += 1;
    }

    let mut gear_product_sum = 0;
    // Look through the gears and see if any have exactly 2 numbers
    for (_coord, gear) in &gears {
        if gear.len() == 2 {
            gear_product_sum += gear[0] * gear[1];
        }
    }

    let duration = start.elapsed();
    println!("Part 1 Sum is: {}", part_numer_sum);
    println!("Part 2 Sum is: {}", gear_product_sum);
    println!("Time elapsed is: {:?}", duration);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_line(y: usize, lines: &Vec<Result<String, Error>>) -> String {
    return match lines.get(y) {
        Some(line) => match line {
            Ok(l) => l.to_string(),
            Err(e) => panic!("Error reading line: {}", e),
        },
        None => panic!("No lines found"),
    };
}

// x, y is the line and column the number starts on
fn is_part_number(
    x_start: usize,
    x_end: usize,
    y: usize,
    lines: &Vec<Result<String, Error>>,
) -> bool {
    let mut is_part_number = false;
    let mut y_cursor = if y < 1 { 0 } else { y - 1 };

    let x_boundary = get_line(y, lines).len();

    // Left bound cannot be less than 0
    let x1_boundary = if x_start < 1 { 0 } else { x_start - 1 };
    // right cannot be more than the length of the line
    let x2_boundary = if x_end == x_boundary {
        x_end
    } else {
        x_end + 1
    };
    // down boundary cannot be more than the length of the lines
    let y2_boundary = if y + 2 > lines.len() {
        lines.len()
    } else {
        y + 2
    };

    // println!(
    //     "y_cursor: {}, y2_boundary: {}, x1_boundary: {}, x2_boundary: {}",
    //     y_cursor, y2_boundary, x1_boundary, x2_boundary
    // );

    while y_cursor < y2_boundary && !is_part_number {
        let line = get_line(y_cursor, lines);
        let substr = match line.get(x1_boundary..x2_boundary) {
            Some(s) => s,
            None => "...", // default value
        };

        // println!("Substr to check: {}", substr);

        let r = match Regex::new(r"[^a-zA-Z0-9.]") {
            Ok(r) => r,
            Err(e) => panic!("Error creating regex: {}", e),
        };

        // println!("Has symbol: {}", r.is_match(substr));

        is_part_number = r.is_match(substr);

        y_cursor += 1;
    }

    return is_part_number;
}

fn is_possible_gear(
    x_start: usize,
    x_end: usize,
    y: usize,
    lines: &Vec<Result<String, Error>>,
) -> Option<(usize, usize)> {
    let mut y_cursor = if y < 1 { 0 } else { y - 1 };

    let x_boundary = get_line(y, lines).len();

    // Left bound cannot be less than 0
    let x1_boundary = if x_start < 1 { 0 } else { x_start - 1 };
    // right cannot be more than the length of the line
    let x2_boundary = if x_end == x_boundary {
        x_end
    } else {
        x_end + 1
    };
    // down boundary cannot be more than the length of the lines
    let y2_boundary = if y + 2 > lines.len() {
        lines.len()
    } else {
        y + 2
    };

    while y_cursor < y2_boundary {
        let line = get_line(y_cursor, lines);
        let substr = match line.get(x1_boundary..x2_boundary) {
            Some(s) => s,
            None => "...", // default value
        };

        if let Some(x_additive) = substr.find("*") {
            let coord = (x_additive + x1_boundary, y_cursor);
            // println!("Possible gear at: {:?}", coord);
            return Some(coord);
        }

        y_cursor += 1;
    }

    return None;
}
