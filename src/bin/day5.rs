use aoc_2023::*;
use rayon::{iter::ParallelIterator, prelude::*};
use std::ops::Range;

struct SeedMap(Vec<(Range<u64>, Range<u64>)>);
impl SeedMap {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn get(&self, seed: u64) -> u64 {
        self.0
            .iter()
            .find_map(|(src_range, dest_range)| {
                if src_range.contains(&seed) {
                    Some(seed - src_range.start + dest_range.start)
                } else {
                    None
                }
            })
            .unwrap_or(seed)
    }
}

struct Almanac {
    seed_to_soil: SeedMap,
    soil_to_fertilizer: SeedMap,
    fertilizer_to_water: SeedMap,
    water_to_light: SeedMap,
    light_to_temperature: SeedMap,
    temperature_to_humidity: SeedMap,
    humidity_to_location: SeedMap,
}

fn parse_map<'a>(maps: &mut impl Iterator<Item = &'a str>) -> SeedMap {
    maps.next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            let mut parts = line.split_whitespace();
            let dest: u64 = parts.next().unwrap().parse().unwrap();
            let src: u64 = parts.next().unwrap().parse().unwrap();
            let len: u64 = parts.next().unwrap().parse().unwrap();

            (src, dest, len)
        })
        .fold(SeedMap::new(), |mut map, (src, dest, len)| {
            map.0.push((src..src + len, dest..dest + len));
            map
        })
}

fn parse<'a>(mut input: impl Iterator<Item = &'a str>) -> Almanac {
    let seed_to_soil = parse_map(&mut input);
    let soil_to_fertilizer = parse_map(&mut input);
    let fertilizer_to_water = parse_map(&mut input);
    let water_to_light = parse_map(&mut input);
    let light_to_temperature = parse_map(&mut input);
    let temperature_to_humidity = parse_map(&mut input);
    let humidity_to_location = parse_map(&mut input);

    Almanac {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn part1(input: &str) -> u64 {
    let mut maps = input.split("\n\n");
    let seeds = maps
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap());
    let almanac = parse(maps);

    seeds
        .map(|seed| {
            let soil = almanac.seed_to_soil.get(seed);
            let fertilizer = almanac.soil_to_fertilizer.get(soil);
            let water = almanac.fertilizer_to_water.get(fertilizer);
            let light = almanac.water_to_light.get(water);
            let temperature = almanac.light_to_temperature.get(light);
            let humidity = almanac.temperature_to_humidity.get(temperature);
            almanac.humidity_to_location.get(humidity)
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let mut maps = input.split("\n\n");
    let seeds = maps
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .chunks()
        .map(|[start, length]| (start..(start + length)));
    let almanac = parse(maps);

    seeds.fold(u64::MAX, |acc, seed_range| {
        let local_min = seed_range
            .into_par_iter()
            .map(|seed| {
                let soil = almanac.seed_to_soil.get(seed);
                let fertilizer = almanac.soil_to_fertilizer.get(soil);
                let water = almanac.fertilizer_to_water.get(fertilizer);
                let light = almanac.water_to_light.get(water);
                let temperature = almanac.light_to_temperature.get(light);
                let humidity = almanac.temperature_to_humidity.get(temperature);
                almanac.humidity_to_location.get(humidity)
            })
            .min()
            .unwrap_or(u64::MAX);
        acc.min(local_min)
    })
}

aoc!(
    5,
    r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    35,
    46
);
