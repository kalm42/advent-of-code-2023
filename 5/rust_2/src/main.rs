use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::time::Instant;

// 114137738 too high
// 84206669
// 1245951 too low
// 0 is too low

fn main() {
    let start = Instant::now();

    let lines = match read_lines("./input.txt") {
        Ok(l) => l,
        Err(e) => panic!("Error: {}", e),
    };

    let lines: Vec<_> = lines.collect();

    // Part 2
    let (seed_ranges, maps) = parse_almanac(&lines);

    // bucket
    let mut closest_location: i64 = std::i64::MAX;

    for seed_range in seed_ranges {
        closest_location = min(closest_location, find_lowest_location(seed_range, &maps));
    }

    let duration = start.elapsed();
    println!("Part 2 Closest location is: {}", closest_location);
    println!("Time elapsed is: {:?}", duration);
}

/*******************************************************************************
 * Generic functions
 */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_numbers_from_line(line: String) -> Vec<i64> {
    // White space is our delimiter
    return line
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
}
/*******************************************************************************
 * Not generic functions
 */
#[derive(Clone, Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Range {
        if start > end {
            panic!("Invalid range: {} - {}", start, end);
        }
        Range { start, end }
    }

    fn contains(&self, value: i64) -> bool {
        return value >= self.start && value <= self.end;
    }

    // fn contains_range(&self, range: Range) -> bool {
    //     return self.contains(range.start) && self.contains(range.end);
    // }

    fn len(&self) -> i64 {
        return self.end - self.start + 1;
    }

    fn intersects(&self, range: &Range) -> bool {
        let r = self.contains(range.start)
            || self.contains(range.end)
            || (range.start <= self.start && range.end >= self.end);
        return r;
    }

    // fn union(&self, range: &Range) -> Result<Range, String> {
    //     println!("Union: {:?} {:?}", self, range);
    //     if self.end < range.start || self.start > range.end {
    //         return Err(format!("Ranges do not overlap: {:?} {:?}", self, range));
    //     }
    //     let start = min(self.start, range.start);
    //     let end = max(self.end, range.end);
    //     return Ok(Range::new(start, end));
    // }

    fn intersection(&self, range: &Range) -> Result<Range, String> {
        if self.end < range.start || self.start > range.end {
            return Err(format!("Ranges do not overlap: {:?} {:?}", self, range));
        }
        let start = max(self.start, range.start);
        let end = min(self.end, range.end);
        return Ok(Range::new(start, end));
    }

    fn difference(&self, range: &Range) -> Result<Vec<Range>, String> {
        if self.end < range.start || self.start > range.end {
            return Err(format!("Ranges do not overlap: {:?} {:?}", self, range));
        }
        let mut ranges: Vec<Range> = Vec::new();
        if self.start < range.start {
            ranges.push(Range::new(self.start, range.start - 1));
        }
        if self.end > range.end {
            ranges.push(Range::new(range.end + 1, self.end));
        }
        return Ok(ranges);
    }
}
impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

fn parse_almanac(lines: &Vec<Result<String, Error>>) -> (Vec<Range>, Vec<Vec<(Range, Range)>>) {
    let seed_line: String = match lines[0].as_ref() {
        Ok(s) => s.to_string(),
        Err(e) => panic!("Error: {}", e),
    };

    let seed_ranges: Vec<Range> = extract_seed_ranges(&seed_line);
    let maps: Vec<Vec<(Range, Range)>> = extract_maps(lines);

    return (seed_ranges, maps);
}

fn extract_seed_ranges(line: &String) -> Vec<Range> {
    let mut seed_ranges: Vec<Range> = Vec::new();

    // Line is in the format "seeds: 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15"
    let not_parsed = line.split(":").collect::<Vec<&str>>();
    let seeds = get_numbers_from_line(not_parsed[1].to_string());

    // Seeds are in pairs, so we need to chunk them
    for seed_chunk in seeds.chunks(2) {
        if let [seed_start, distance] = seed_chunk {
            let seed_end = seed_start + distance - 1;
            seed_ranges.push(Range::new(*seed_start, seed_end));
        }
    }

    return seed_ranges;
}

fn extract_maps(lines: &Vec<Result<String, Error>>) -> Vec<Vec<(Range, Range)>> {
    let mut maps: Vec<Vec<(Range, Range)>> = Vec::new();
    let mut current_map: Vec<(Range, Range)> = Vec::new();

    for line in lines.iter().skip(2) {
        let line: String = match line.as_ref() {
            Ok(s) => s.to_string(),
            Err(e) => panic!("Error: {}", e),
        };
        if line.is_empty() && current_map.len() > 1 {
            maps.push(current_map);
            current_map = Vec::new();
        } else if line.starts_with(char::is_alphabetic) {
            // This is a map name
            continue;
        } else {
            // This is a map
            let numbers: Vec<_> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if let [destination_start, source_start, distance] = numbers.as_slice() {
                let destination_end = destination_start + distance - 1;
                let source_end = source_start + distance - 1;
                current_map.push((
                    Range::new(*source_start, source_end),
                    Range::new(*destination_start, destination_end),
                ));
            }
        }
    }

    if current_map.len() > 1 {
        maps.push(current_map);
    }

    maps
}

fn transform_range(a: &Range, map: &Vec<(Range, Range)>) -> Vec<Range> {
    let mut transformed_ranges: Vec<Range> = Vec::new();
    let mut maps = map.iter();

    while let Some((source, destination)) = maps.next() {
        let source = source.clone();
        let destination = destination.clone();

        if source.intersects(a) {
            let intersection = match source.intersection(a) {
                Ok(r) => r,
                Err(e) => panic!("Error: {}", e),
            };
            let difference = match a.difference(&source) {
                Ok(r) => r,
                Err(e) => panic!("Error: {}", e),
            };

            for new_a in difference {
                let transformed = transform_range(&new_a, map);
                transformed_ranges.extend(transformed);
            }

            let new_location = intersection.start + (destination.start - source.start);
            let new_range = Range::new(new_location, new_location + intersection.len() - 1);
            transformed_ranges.push(new_range);
            return transformed_ranges;
        }
    }

    if transformed_ranges.len() == 0 {
        transformed_ranges.push(a.clone());
    }

    return transformed_ranges;
}

fn find_lowest_location(seed_range: Range, maps: &Vec<Vec<(Range, Range)>>) -> i64 {
    let mut lowest_location: i64 = std::i64::MAX;
    let mut current_ranges = vec![seed_range.clone()];

    for map in maps {
        let mut transformed_ranges: Vec<Range> = Vec::new();
        for current_range in current_ranges {
            let r = transform_range(&current_range, map);
            transformed_ranges.extend(r);
        }
        current_ranges = transformed_ranges;
    }

    for current_range in current_ranges {
        if current_range.start < lowest_location {
            lowest_location = current_range.start;
        }
        if current_range.end < lowest_location {
            lowest_location = current_range.end;
        }
    }

    return lowest_location;
}
