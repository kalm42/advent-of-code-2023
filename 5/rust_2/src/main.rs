use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::time::Instant;

// 114137738 too high

fn main() {
    let start = Instant::now();

    let lines = match read_lines("./input_sample.txt") {
        Ok(l) => l,
        Err(e) => panic!("Error: {}", e),
    };

    let lines: Vec<_> = lines.collect();

    // Line 1 is our seeds
    let seeds = get_seeds(&lines);
    // println!("Seeds: {:?}", seeds);

    // Ok... new strategy.We're going to check if a seed is in a map range. It it is, find the mapped location, rather rinse and repeat.
    let ranges = get_map_ranges(&lines);

    let mut location: Option<i64> = None;

    for seed in seeds.iter() {
        let new_location = match get_location_for_seed(*seed, &ranges) {
            Some(l) => l,
            None => panic!("No location found for seed: {}", seed),
        };
        match location {
            Some(l) => {
                if new_location < l {
                    location = Some(new_location);
                }
            }
            None => {
                location = Some(new_location);
            }
        }
    }

    let closest_location = match location {
        Some(l) => l,
        None => panic!("No location found for seeds: {:?}", seeds),
    };

    // Part 2
    let mut seed_ranges: Vec<Range> = Vec::new();
    for seed_chunk in seeds.chunks(2) {
        if let [seed_start, distance] = seed_chunk {
            let seed_end = seed_start + distance - 1;
            seed_ranges.push(Range::new(*seed_start, seed_end));
        }
    }
    println!("Seed ranges: {:?}", seed_ranges);

    let mut omega_closest_location: i64 = std::i64::MAX;
    for seed_range in seed_ranges.iter() {
        let new_location = match get_location_for_seed_range(seed_range.clone(), &ranges) {
            Some(l) => l,
            None => vec![Range::new(std::i64::MAX, std::i64::MAX)],
        };
        for location in new_location {
            if location.start < omega_closest_location {
                omega_closest_location = location.start;
            }
        }
    }
    // println!("Omega closest location: {}", omega_closest_location);

    let duration = start.elapsed();
    println!("Part 1 Closest location is: {}", closest_location);
    println!("Part 2 Closest location is: {}", omega_closest_location);
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

    fn contains_range(&self, range: Range) -> bool {
        return self.contains(range.start) && self.contains(range.end);
    }

    fn len(&self) -> i64 {
        return self.end - self.start + 1;
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

fn get_seeds(lines: &Vec<Result<String, Error>>) -> Vec<i64> {
    return match lines[0].as_ref() {
        Ok(s) => {
            let not_parsed = s.split(":").collect::<Vec<&str>>();
            let seeds = get_numbers_from_line(not_parsed[1].to_string());
            seeds
        }
        Err(e) => panic!("Error: {}", e),
    };
}

fn get_map_ranges(
    lines: &Vec<Result<String, Error>>,
) -> (
    Vec<(Range, Range)>,
    Vec<(Range, Range)>,
    Vec<(Range, Range)>,
    Vec<(Range, Range)>,
    Vec<(Range, Range)>,
    Vec<(Range, Range)>,
    Vec<(Range, Range)>,
) {
    enum Mapping {
        SeedToSoil,
        SoilToFertilizer,
        FertilizerToWater,
        WaterToLight,
        LightToTemperature,
        TemperatureToHumidity,
        HumidityToLocation,
    }
    let mut mapping = Mapping::SeedToSoil;
    let mut seed_to_soil: Vec<(Range, Range)> = Vec::new();
    let mut soil_to_fertilizer: Vec<(Range, Range)> = Vec::new();
    let mut fertilizer_to_water: Vec<(Range, Range)> = Vec::new();
    let mut water_to_light: Vec<(Range, Range)> = Vec::new();
    let mut light_to_temperature: Vec<(Range, Range)> = Vec::new();
    let mut temperature_to_humidity: Vec<(Range, Range)> = Vec::new();
    let mut humidity_to_location: Vec<(Range, Range)> = Vec::new();

    for line in lines.iter().skip(1) {
        match line.as_ref() {
            Ok(l) => {
                // a line is either blank, a title, or a mapping
                let first_char = l.chars().nth(0);
                match first_char {
                    Some(c) => {
                        if c.is_alphabetic() {
                            // title
                            match l.as_str() {
                                "seed-to-soil map:" => mapping = Mapping::SeedToSoil,
                                "soil-to-fertilizer map:" => mapping = Mapping::SoilToFertilizer,
                                "fertilizer-to-water map:" => mapping = Mapping::FertilizerToWater,
                                "water-to-light map:" => mapping = Mapping::WaterToLight,
                                "light-to-temperature map:" => {
                                    mapping = Mapping::LightToTemperature
                                }
                                "temperature-to-humidity map:" => {
                                    mapping = Mapping::TemperatureToHumidity
                                }
                                "humidity-to-location map:" => {
                                    mapping = Mapping::HumidityToLocation
                                }
                                _ => panic!("Unknown mapping: {}", l),
                            }
                        } else {
                            match mapping {
                                Mapping::SeedToSoil => {
                                    update_range(&mut seed_to_soil, &l.to_string());
                                }
                                Mapping::SoilToFertilizer => {
                                    update_range(&mut soil_to_fertilizer, &l.to_string());
                                }
                                Mapping::FertilizerToWater => {
                                    update_range(&mut fertilizer_to_water, &l.to_string());
                                }
                                Mapping::WaterToLight => {
                                    update_range(&mut water_to_light, &l.to_string());
                                }
                                Mapping::LightToTemperature => {
                                    update_range(&mut light_to_temperature, &l.to_string());
                                }
                                Mapping::TemperatureToHumidity => {
                                    update_range(&mut temperature_to_humidity, &l.to_string());
                                }
                                Mapping::HumidityToLocation => {
                                    update_range(&mut humidity_to_location, &l.to_string());
                                }
                            }
                        }
                    }
                    None => {
                        // blank line, fine, NEXT!
                    }
                }
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    return (
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    );
}

fn update_range(map: &mut Vec<(Range, Range)>, line: &String) {
    let values = get_numbers_from_line(line.to_string());
    if values.len() != 3 {
        panic!("Invalid line: {}, got {:?}", line, values);
    }
    let destination_start = values[0];
    let source_start = values[1];
    let distance = values[2];
    let source_end = source_start + distance - 1;
    let destination_end = destination_start + distance - 1;

    let destination = Range::new(destination_start, destination_end);
    let source = Range::new(source_start, source_end);

    map.push((source, destination));
}

fn get_next_marker(map: &Vec<(Range, Range)>, key: i64) -> Option<i64> {
    let mut marker: Option<i64> = None;
    for (source, destination) in map.iter() {
        if source.contains(key) {
            // To figure out next fig is x - source_start + destination_start
            marker = Some(key - source.start + destination.start);
            break;
        }
    }

    return marker;
}

fn get_next_marker_ranges(
    map: &Vec<(Range, Range)>,
    key_ranges: &Vec<Range>,
) -> Option<Vec<Range>> {
    let mut accepted_source_ranges: Vec<Range> = Vec::new();
    let mut accepted_destination_ranges: Vec<Range> = Vec::new();

    // Loop over the key ranges to see if they are in a map
    for key_range in key_ranges.iter() {
        let mut key = key_range.clone();
        for (source_range, destination_range) in map.iter() {
            // How do we move from the source to the destination?
            let move_distance = destination_range.start - source_range.start;

            // Check to see if any of the key_range intersects with the source_range
            if source_range.contains_range(key.clone()) {
                // println!("Source range contains key range.");

                // Get the new range
                let new_start = key.start + move_distance;
                let new_end = key.end + move_distance;
                let new_range = Range::new(new_start, new_end);

                // println!("New range: start: {}, end: {}", new_start, new_end);
                accepted_destination_ranges.push(new_range);

                // Update the accepted source ranges
                accepted_source_ranges.push(key.clone());
            } else if source_range.contains(key.start) || source_range.contains(key.end) {
                // println!("Source range intersects with key range.");
                // start is not in range, end is
                if let Ok(intersecting_range) = source_range.intersection(&key) {
                    accepted_source_ranges.push(intersecting_range.clone());

                    let new_start = intersecting_range.start + move_distance;
                    let new_end = intersecting_range.end + move_distance;
                    let new_range = Range::new(new_start, new_end);
                    // println!("New range: start: {}, end: {}", new_start, new_end);
                    accepted_destination_ranges.push(new_range);

                    // Update the key range to exclude
                    if let Ok(diff) = key.difference(&intersecting_range) {
                        // println!("Difference: {:?}", diff);
                        if diff.len() == 1 {
                            key = diff[0].clone();
                        }
                    }
                }
            }
        }
    }

    // All key_ranges have been checked to see if they map to a different
    // destination_range. We now need to see what if any accepted_source_ranges
    // are not in the key_range.
    if accepted_source_ranges.len() == 0 {
        return None;
    }

    // Now we need to see if any of the key_ranges are not in the accepted_source_ranges
    let mut ranges_to_carry_on: Vec<Range> = Vec::new();
    let mut uncovered_ranges: Vec<Range> = Vec::new();
    uncovered_ranges.append(&mut key_ranges.clone());

    while uncovered_ranges.iter().map(|r| r.len()).sum::<i64>() != 1 {
        let mut remaining_range: Vec<Range> = Vec::new();
        for accepted_range in &accepted_source_ranges {
            for r_range in uncovered_ranges.iter() {
                if r_range.contains_range(accepted_range.clone()) {
                    if let Ok(diff) = r_range.difference(accepted_range) {
                        if !diff.is_empty() {
                            remaining_range.append(&mut diff.clone());
                        } else {
                            remaining_range = vec![Range::new(0, 0)]; // empty range
                            break;
                        }
                    }
                }
            }
        }
        println!("Remaining range: {:?}", remaining_range);
        uncovered_ranges = remaining_range;
    }

    for key_range in key_ranges.iter() {
        let mut remaining_range = vec![key_range.clone()];
        for accepted_range in &accepted_source_ranges {
            for r_range in remaining_range.iter() {
                if r_range.contains_range(accepted_range.clone()) {
                    // println!("Removing: {:?}", accepted_range);
                    if let Ok(diff) = r_range.difference(accepted_range) {
                        if !diff.is_empty() {
                        } else {
                            remaining_range = vec![Range::new(0, 0)]; // empty range
                            break;
                        }
                    }
                }
            }
        }
        for r_range in remaining_range.iter() {
            if r_range.len() > 0 {
                uncovered_ranges.push(r_range.clone());
            }
        }
    }
    accepted_destination_ranges.append(&mut uncovered_ranges);

    let mut keys_length: i64 = 0;
    let mut dest_length: i64 = 0;
    for key_range in key_ranges.iter() {
        keys_length += key_range.len();
    }
    for dest_range in accepted_destination_ranges.iter() {
        dest_length += dest_range.len();
    }

    if keys_length != dest_length {
        panic!(
            "Key length: {}, dest length: {}, diff: {}",
            keys_length,
            dest_length,
            keys_length - dest_length
        );
    }

    return Some(accepted_destination_ranges);
}

fn get_location_for_seed(
    seed: i64,
    ranges: &(
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
    ),
) -> Option<i64> {
    let (
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = ranges;

    let soil = match get_next_marker(&seed_to_soil, seed) {
        Some(s) => s,
        None => seed, // if there the number is the same
    };
    let fertilizer = match get_next_marker(&soil_to_fertilizer, soil) {
        Some(f) => f,
        None => soil, // if there the number is the same
    };
    let water = match get_next_marker(&fertilizer_to_water, fertilizer) {
        Some(w) => w,
        None => fertilizer, // if there the number is the same
    };
    let light = match get_next_marker(&water_to_light, water) {
        Some(l) => l,
        None => water, // if there the number is the same
    };
    let temperature = match get_next_marker(&light_to_temperature, light) {
        Some(t) => t,
        None => light, // if there the number is the same
    };
    let humidity = match get_next_marker(&temperature_to_humidity, temperature) {
        Some(h) => h,
        None => temperature, // if there the number is the same
    };
    let location = match get_next_marker(&humidity_to_location, humidity) {
        Some(l) => l,
        None => humidity, // if there the number is the same
    };

    return Some(location);
}

fn get_location_for_seed_range(
    seed_range: Range,
    ranges: &(
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
        Vec<(Range, Range)>,
    ),
) -> Option<Vec<Range>> {
    let (
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = ranges;

    let start = vec![seed_range.clone()];
    let soil: Vec<Range> = match get_next_marker_ranges(&seed_to_soil, &start) {
        Some(s) => s,
        None => vec![seed_range], // if there the number is the same
    };
    // println!("Soil: {:?}", soil);

    let fertilizer: Vec<Range> = match get_next_marker_ranges(&soil_to_fertilizer, &soil) {
        Some(f) => f,
        None => soil, // if there the number is the same
    };
    // println!("Fertilizer: {:?}", fertilizer);

    let water: Vec<Range> = match get_next_marker_ranges(&fertilizer_to_water, &fertilizer) {
        Some(w) => w,
        None => fertilizer, // if there the number is the same
    };
    // println!("Water: {:?}", water);

    let light: Vec<Range> = match get_next_marker_ranges(&water_to_light, &water) {
        Some(l) => l,
        None => water, // if there the number is the same
    };
    // println!("Light: {:?}", light);

    let temperature: Vec<Range> = match get_next_marker_ranges(&light_to_temperature, &light) {
        Some(t) => t,
        None => light, // if there the number is the same
    };
    // println!("Temperature: {:?}", temperature);

    let humidity: Vec<Range> = match get_next_marker_ranges(&temperature_to_humidity, &temperature)
    {
        Some(h) => h,
        None => temperature, // if there the number is the same
    };
    // println!("Humidity: {:?}", humidity);

    let location: Vec<Range> = match get_next_marker_ranges(&humidity_to_location, &humidity) {
        Some(l) => l,
        None => humidity, // if there the number is the same
    };
    // println!("Location: {:?}", location);

    return Some(location);
}
