use std::{collections::BTreeSet, io::stdin, ops::Mul};

use itertools::Itertools;

struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let image = parse_image(stdin().lines().map(|l| l.unwrap()));

    // part 1
    let sum: usize = expand_image(&image, 2)
        .iter()
        .tuple_combinations()
        .map(|(a, b)| get_shortest_distance(a, b))
        .sum();

    // part 2
    let sum_older: usize = expand_image(&image, 1_000_000)
        .iter()
        .tuple_combinations()
        .map(|(a, b)| get_shortest_distance(a, b))
        .sum();

    println!("{}", sum);
    println!("{}", sum_older);
}

fn parse_image<I>(lines: I) -> Vec<Point>
where
    I: Iterator<Item = String>,
{
    lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| Point { x, y })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn expand_image(galaxies: &Vec<Point>, factor: usize) -> Vec<Point> {
    let (cols, rows): (BTreeSet<usize>, BTreeSet<usize>) =
        galaxies.iter().map(|p| (p.x, p.y)).unzip();

    let empty_cols = (0..*cols.last().unwrap_or(&0))
        .filter(|x| !cols.contains(x))
        .collect::<Vec<_>>();

    let empty_rows = (0..*rows.last().unwrap_or(&0))
        .filter(|y| !rows.contains(y))
        .collect::<Vec<_>>();

    galaxies
        .iter()
        .map(|point| Point {
            x: point.x
                + empty_cols
                    .iter()
                    .take_while(|&&x| x < point.x)
                    .count()
                    .mul(factor - 1),
            y: point.y
                + empty_rows
                    .iter()
                    .take_while(|&&y| y < point.y)
                    .count()
                    .mul(factor - 1),
        })
        .collect()
}

fn get_shortest_distance(a: &Point, b: &Point) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}
