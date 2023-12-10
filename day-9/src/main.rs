#![feature(iter_map_windows)]
use std::io::stdin;

fn main() {
    let mut sequences = stdin()
        .lines()
        .map(|l| parse_sequence(&l.unwrap()))
        .collect::<Vec<_>>();

    // part 1
    let sum_of_nexts: i32 = sequences.iter().map(predict_next).sum();

    // part 2
    sequences.iter_mut().for_each(|s| s.reverse());
    let sum_of_previouses: i32 = sequences.iter().map(predict_next).sum();

    println!("{}", sum_of_nexts);
    println!("{}", sum_of_previouses);
}

fn parse_sequence(str: &str) -> Vec<i32> {
    str.split_whitespace()
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .collect()
}

fn predict_next(sequence: &Vec<i32>) -> i32 {
    sequence
        .last()
        .map(|&n| n + predict_next(&sequence.iter().map_windows(|[&a, &b]| b - a).collect()))
        .unwrap_or(0)
}
