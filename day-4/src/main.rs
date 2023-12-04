use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use regex_macro::regex;

struct Card {
    id: u32,
    numbers: HashSet<u8>,
    winners: HashSet<u8>,
}

fn main() {
    let mut points = 0;
    let mut copy_counts = HashMap::new();

    for line in stdin().lines().map(|l| l.unwrap()) {
        let card = parse_card(&line);
        let winning_count = card.numbers.intersection(&card.winners).count() as u32;

        // part 1
        points += calculate_score(winning_count);

        // part 2
        let copy_count = copy_counts
            .entry(card.id)
            .and_modify(|c| *c += 1)
            .or_insert(1)
            .clone();

        for id in (card.id + 1)..=(card.id + winning_count) {
            copy_counts
                .entry(id)
                .and_modify(|c| *c += copy_count)
                .or_insert(copy_count);
        }
    }

    println!("{}", points);
    println!("{}", copy_counts.values().sum::<u32>());
}

fn parse_card(str: &str) -> Card {
    let captures = regex!(r"Card\s*(?<id>\d+):\s*(?<winners>.+)\s*\|\s*(?<numbers>.+)")
        .captures(str)
        .expect(&format!("malformed card: {}", str));

    Card {
        id: u32::from_str_radix(captures.name("id").unwrap().as_str(), 10).unwrap(),
        numbers: parse_numbers(captures.name("numbers").unwrap().as_str()),
        winners: parse_numbers(captures.name("winners").unwrap().as_str()),
    }
}

fn parse_numbers(str: &str) -> HashSet<u8> {
    str.split_whitespace()
        .map(|s| u8::from_str_radix(s, 10).unwrap())
        .collect()
}

fn calculate_score(winning_count: u32) -> u32 {
    if winning_count < 1 {
        0
    } else {
        2u32.pow(winning_count - 1)
    }
}
