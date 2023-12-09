#![feature(let_chains)]
use std::{cmp::Ordering, collections::HashMap, io::stdin, iter::zip};

struct Hand {
    cards: [u8; 5],
    bid: u32,
}

fn main() {
    let mut hands = stdin()
        .lines()
        .map(|l| parse_hand(&l.unwrap()))
        .collect::<Vec<_>>();

    // part 1
    hands.sort_by(|a, b| order_hand(&a.cards, &b.cards));
    let total_winnings_with_jacks = sum_winnings(&hands);

    // part 2
    hands.iter_mut().for_each(|h| replace_jacks(&mut h.cards));
    hands.sort_by(|a, b| order_hand(&a.cards, &b.cards));
    let total_winnings_with_jokers = sum_winnings(&hands);

    println!("{}", total_winnings_with_jacks);
    println!("{}", total_winnings_with_jokers);
}

fn parse_hand(line: &str) -> Hand {
    let mut cols = line.split_whitespace();

    Hand {
        cards: cols
            .next()
            .expect("missing cards")
            .chars()
            .map(|c| char_to_card(c))
            .collect::<Vec<_>>()
            .try_into()
            .expect("more than 5 cards"),
        bid: cols.next().expect("missing bid").parse().unwrap(),
    }
}

fn char_to_card(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("bad card"),
    }
}

fn cards_to_rank(cards: &[u8; 5]) -> u8 {
    let mut counts = HashMap::with_capacity(cards.len());

    for card in cards {
        *counts.entry(card).or_default() += 1u8;
    }

    let jokers = counts.remove(&1).unwrap_or(0);
    let max = counts.values().map(|&v| v).max().unwrap_or(0) + jokers;

    match (counts.len(), max) {
        (_, 5) => 6, // five of a kind
        (_, 4) => 5, // four of a kind
        (2, 3) => 4, // full house
        (_, 3) => 3, // three of a kind
        (3, 2) => 2, // two pair
        (_, 2) => 1, // one pair
        _ => 0,      // high card
    }
}

fn order_hand(a: &[u8; 5], b: &[u8; 5]) -> Ordering {
    let mut ordering = cards_to_rank(a).cmp(&cards_to_rank(b));
    let mut iter = zip(a, b).map(|(a, b)| a.cmp(b));

    while ordering == Ordering::Equal
        && let Some(next) = iter.next()
    {
        ordering = next;
    }

    ordering
}

fn replace_jacks(cards: &mut [u8; 5]) {
    cards.iter_mut().filter(|c| **c == 11).for_each(|c| *c = 1);
}

fn sum_winnings(hands: &[Hand]) -> u64 {
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u64 * hand.bid as u64)
        .sum()
}
