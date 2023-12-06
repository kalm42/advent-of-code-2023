use std::collections::HashMap;
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

    // Line 1 is our seeds
    let seeds = get_seeds(&lines);
    println!("Seeds: {:?}", seeds);

    // Make our maps
    let maps = parse_and_return_maps(&lines);

    // Get the location for each seed
    let mut location: Option<i32> = None;
    for seed in seeds.iter() {
        let new_location = get_location_for_seed(&maps, *seed);
        println!("Seed: {}, Location: {}", seed, new_location);
        match location {
            Some(l) => {
                if l > new_location {
                    location = Some(new_location);
                }
            }
            None => location = Some(new_location),
        }
    }

    let closest_location = match location {
        Some(l) => l,
        None => panic!("No location found"),
    };

    let duration = start.elapsed();
    println!("Part 1 Location is: {}", closest_location);
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

fn get_numbers_from_line(line: String) -> Vec<i32> {
    // White space is our delimiter
    return line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
}

/*******************************************************************************
 * Not generic functions
 */
fn get_seeds(lines: &Vec<Result<String, Error>>) -> Vec<i32> {
    return match lines[0].as_ref() {
        Ok(s) => {
            let not_parsed = s.split(":").collect::<Vec<&str>>();
            let seeds = get_numbers_from_line(not_parsed[1].to_string());
            seeds
        }
        Err(e) => panic!("Error: {}", e),
    };
}

fn parse_and_return_maps(
    lines: &Vec<Result<String, Error>>,
) -> (
    HashMap<i32, i32>,
    HashMap<i32, i32>,
    HashMap<i32, i32>,
    HashMap<i32, i32>,
    HashMap<i32, i32>,
    HashMap<i32, i32>,
    HashMap<i32, i32>,
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
    let mut seed_to_soil: HashMap<i32, i32> = HashMap::new();
    let mut soil_to_fertilizer: HashMap<i32, i32> = HashMap::new();
    let mut fertilizer_to_water: HashMap<i32, i32> = HashMap::new();
    let mut water_to_light: HashMap<i32, i32> = HashMap::new();
    let mut light_to_temperature: HashMap<i32, i32> = HashMap::new();
    let mut temperature_to_humidity: HashMap<i32, i32> = HashMap::new();
    let mut humidity_to_location: HashMap<i32, i32> = HashMap::new();

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
                                    update_map(&mut seed_to_soil, &l.to_string());
                                }
                                Mapping::SoilToFertilizer => {
                                    update_map(&mut soil_to_fertilizer, &l.to_string());
                                }
                                Mapping::FertilizerToWater => {
                                    update_map(&mut fertilizer_to_water, &l.to_string());
                                }
                                Mapping::WaterToLight => {
                                    update_map(&mut water_to_light, &l.to_string());
                                }
                                Mapping::LightToTemperature => {
                                    update_map(&mut light_to_temperature, &l.to_string());
                                }
                                Mapping::TemperatureToHumidity => {
                                    update_map(&mut temperature_to_humidity, &l.to_string());
                                }
                                Mapping::HumidityToLocation => {
                                    update_map(&mut humidity_to_location, &l.to_string());
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

    // All the custom maps are filled now to add the default values
    fill_map(&mut seed_to_soil);
    fill_map(&mut soil_to_fertilizer);
    fill_map(&mut fertilizer_to_water);
    fill_map(&mut water_to_light);
    fill_map(&mut light_to_temperature);
    fill_map(&mut temperature_to_humidity);
    fill_map(&mut humidity_to_location);

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

fn update_map(map: &mut HashMap<i32, i32>, line: &String) {
    let values = get_numbers_from_line(line.to_string());
    let iterations = values[2];
    for i in 0..iterations {
        let key = values[1] + i;
        let value = values[0] + i;
        map.insert(key, value);
    }
}

fn fill_map(map: &mut HashMap<i32, i32>) {
    for i in 0..1000000 {
        map.entry(i).or_insert(i);
    }
}

fn get_location_for_seed(
    maps: &(
        HashMap<i32, i32>,
        HashMap<i32, i32>,
        HashMap<i32, i32>,
        HashMap<i32, i32>,
        HashMap<i32, i32>,
        HashMap<i32, i32>,
        HashMap<i32, i32>,
    ),
    seed: i32,
) -> i32 {
    let (
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = maps;

    let soil = match seed_to_soil.get(&seed) {
        Some(s) => *s,
        None => panic!("No soil for seed: {}", seed),
    };

    let fertilizer = match soil_to_fertilizer.get(&soil) {
        Some(f) => *f,
        None => panic!("No fertilizer for soil: {}", soil),
    };

    let water = match fertilizer_to_water.get(&fertilizer) {
        Some(w) => *w,
        None => panic!("No water for fertilizer: {}", fertilizer),
    };

    let light = match water_to_light.get(&water) {
        Some(l) => *l,
        None => panic!("No light for water: {}", water),
    };

    let temperature = match light_to_temperature.get(&light) {
        Some(t) => *t,
        None => panic!("No temperature for light: {}", light),
    };

    let humidity = match temperature_to_humidity.get(&temperature) {
        Some(h) => *h,
        None => panic!("No humidity for temperature: {}", temperature),
    };

    let location = match humidity_to_location.get(&humidity) {
        Some(l) => *l,
        None => panic!("No location for humidity: {}", humidity),
    };

    // println!(
    //     "Seed: {}, Soil: {}, Fertilizer: {}, Water: {}, Light: {}, Temperature: {}, Humidity: {}, Location: {}",
    //     seed, soil, fertilizer, water, light, temperature, humidity, location);

    return location;
}
