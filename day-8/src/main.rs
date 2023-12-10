use std::{collections::HashMap, io::stdin};

use num::integer::lcm;
use regex_macro::regex;

fn main() {
    let mut lines = stdin()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty());

    let instructions = parse_instructions(&lines.next().unwrap());
    let network = parse_network(lines);

    // part 1
    let person_steps = count_steps(
        |n| n == "AAA",
        |n| n == "ZZZ",
        &instructions,
        &network,
    );

    // part 2
    let ghost_steps = count_steps(
        |n| n.ends_with("A"),
        |n| n.ends_with("Z"),
        &instructions,
        &network,
    );

    println!("{}", person_steps);
    println!("{}", ghost_steps);
}

fn parse_instructions(str: &str) -> Vec<u8> {
    str.chars()
        .into_iter()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("bad instruction"),
        })
        .collect()
}

fn parse_network<I>(lines: I) -> HashMap<String, [String; 2]>
where
    I: IntoIterator<Item = String>,
{
    let pattern = regex!(r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)");
    let mut map = HashMap::new();

    for line in lines {
        let captures = pattern.captures(&line).unwrap();
        map.insert(
            captures.get(1).unwrap().as_str().to_string(),
            [
                captures.get(2).unwrap().as_str().to_string(),
                captures.get(3).unwrap().as_str().to_string(),
            ],
        );
    }

    map
}

fn count_steps<F, G>(
    starts: F,
    ends: G,
    instructions: &Vec<u8>,
    network: &HashMap<String, [String; 2]>,
) -> u64
where
    F: Fn(&str) -> bool,
    G: Fn(&str) -> bool,
{
    network
        .keys()
        .filter(|&n| starts(n))
        .map(|n| {
            let mut steps = 0;
            let mut at = n;

            for instruction in instructions.iter().cycle() {
                if ends(at) {
                    break;
                }

                at = &network.get(at).expect("invalid node")[*instruction as usize];
                steps += 1;
            }

            steps
        })
        .reduce(lcm)
        .unwrap()
}
