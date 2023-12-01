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
    let mut freqs: Vec<i32> = Vec::new();
    if let Ok(freq_jumble) = line {
        for c in freq_jumble.chars() {
            if let Ok(num) = c.to_string().parse::<i32>() {
                freqs.push(num);
            }
        }
    }

    return match freqs.len() {
        0 => 0,
        1 => format!("{0}{0}", freqs[0]).parse::<i32>().unwrap(),
        2 => format!("{}{}", freqs[0], freqs[1]).parse::<i32>().unwrap(),
        _ => format!("{}{}", freqs[0], freqs[freqs.len() - 1])
            .parse::<i32>()
            .unwrap(),
    };
}

fn part_two(line: &Result<String, Error>) -> i32 {
    let mut word_freqs = Vec::new();
    let alpha_numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut pointer = 0;

    if let Ok(freq_jumble) = line {
        while freq_jumble.len() > pointer {
            let substr = &freq_jumble[pointer..];

            // is the first character a number?
            if let Ok(num) = substr.chars().next().unwrap().to_string().parse::<i32>() {
                word_freqs.push(num);
            } else {
                // if not is this a word?
                for word in &alpha_numbers {
                    if substr.starts_with(word) {
                        match word {
                            &"one" => word_freqs.push(1),
                            &"two" => word_freqs.push(2),
                            &"three" => word_freqs.push(3),
                            &"four" => word_freqs.push(4),
                            &"five" => word_freqs.push(5),
                            &"six" => word_freqs.push(6),
                            &"seven" => word_freqs.push(7),
                            &"eight" => word_freqs.push(8),
                            &"nine" => word_freqs.push(9),
                            _ => {}
                        }
                    }
                }
            }
            pointer += 1;
        }
    }

    return match word_freqs.len() {
        0 => 0,
        1 => format!("{0}{0}", word_freqs[0]).parse::<i32>().unwrap(),
        2 => format!("{}{}", word_freqs[0], word_freqs[1])
            .parse::<i32>()
            .unwrap(),
        _ => format!("{}{}", word_freqs[0], word_freqs[word_freqs.len() - 1])
            .parse::<i32>()
            .unwrap(),
    };
}
