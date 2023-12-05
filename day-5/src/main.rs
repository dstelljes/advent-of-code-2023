use std::io::stdin;

use regex_macro::regex;

#[derive(Default)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

struct Range {
    destination: u64,
    start: u64,
    length: u64,
}

fn main() {
    let almanac = parse_almanac(stdin().lines().map(|l| l.unwrap()));

    // part 1
    let min_by_numbers = (0..u64::MAX)
        .find(|&n| almanac.seeds.contains(&location_to_seed(n, &almanac)))
        .unwrap_or_default();

    // part 2
    let ranges = almanac.seeds.chunks_exact(2)
        .map(|c| c[0]..(c[0] + c[1]))
        .collect::<Vec<_>>();

    let min_by_ranges = (0..u64::MAX)
        .find(|&n| {
            let seed = location_to_seed(n, &almanac);
            ranges.iter().any(|r| r.contains(&seed))
        })
        .unwrap_or_default();

    println!("{}", min_by_numbers);
    println!("{}", min_by_ranges);
}

fn parse_almanac<I>(lines: I) -> Almanac
where
    I: IntoIterator<Item = String>,
{
    let seeds_pattern = regex!(r"seeds:\s*(?<numbers>.+)");
    let map_pattern = regex!(r"(?<source>\w+)-to-(?<destination>\w+) map:");

    let mut almanac = Almanac::default();
    let mut current_map: Option<&mut Vec<Range>> = None;

    for line in lines {
        if line.is_empty() {
            current_map = None;
            continue;
        }

        match current_map {
            Some(ref mut map) => {
                let mut numbers = line
                    .split_whitespace()
                    .map(|n| u64::from_str_radix(n, 10).unwrap());

                map.push(Range {
                    destination: numbers.next().unwrap(),
                    start: numbers.next().unwrap(),
                    length: numbers.next().unwrap(),
                });
            }
            None => {
                if let Some(map_captures) = map_pattern.captures(&line) {
                    let source = map_captures.name("source").unwrap().as_str();
                    let destination = map_captures.name("destination").unwrap().as_str();

                    current_map = Some(match (source, destination) {
                        ("seed", "soil") => &mut almanac.seed_to_soil,
                        ("soil", "fertilizer") => &mut almanac.soil_to_fertilizer,
                        ("fertilizer", "water") => &mut almanac.fertilizer_to_water,
                        ("water", "light") => &mut almanac.water_to_light,
                        ("light", "temperature") => &mut almanac.light_to_temperature,
                        ("temperature", "humidity") => &mut almanac.temperature_to_humidity,
                        ("humidity", "location") => &mut almanac.humidity_to_location,
                        _ => unreachable!(),
                    });
                }

                if let Some(seeds_captures) = seeds_pattern.captures(&line) {
                    almanac.seeds = seeds_captures
                        .name("numbers")
                        .unwrap()
                        .as_str()
                        .split_whitespace()
                        .map(|d| u64::from_str_radix(d, 10).unwrap())
                        .collect();
                }
            }
        }
    }

    almanac
}

fn location_to_seed(location: u64, almanac: &Almanac) -> u64 {
    let mut seed = location;
    seed = destination_to_source(seed, &almanac.humidity_to_location);
    seed = destination_to_source(seed, &almanac.temperature_to_humidity);
    seed = destination_to_source(seed, &almanac.light_to_temperature);
    seed = destination_to_source(seed, &almanac.water_to_light);
    seed = destination_to_source(seed, &almanac.fertilizer_to_water);
    seed = destination_to_source(seed, &almanac.soil_to_fertilizer);
    seed = destination_to_source(seed, &almanac.seed_to_soil);
    seed
}

fn destination_to_source(n: u64, destination_map: &Vec<Range>) -> u64 {
    destination_map
        .iter()
        .find(|&r| (r.destination..(r.destination + r.length)).contains(&n))
        .map_or(n, |r| (n - r.destination) + r.start)
}
