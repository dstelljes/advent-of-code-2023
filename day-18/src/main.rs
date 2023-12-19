use std::io::stdin;

use regex_macro::regex;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

struct Instruction {
    direction: Direction,
    distance: u32,
    color: u32,
}

fn main() {
    let mut instructions =
        parse_instructions(stdin().lines().map(|l| l.unwrap())).collect::<Vec<_>>();

    // part 1
    let area_by_distance = calculate_area(&instructions);

    // part 2
    fix_instructions(&mut instructions);
    let area_by_color = calculate_area(&instructions);

    println!("{}", area_by_distance);
    println!("{}", area_by_color);
}

fn parse_instructions<I>(lines: I) -> impl Iterator<Item = Instruction>
where
    I: Iterator<Item = String>,
{
    let instruction_pattern =
        regex!(r"(?<direction>[UDLR])\s*(?<distance>\d+)\s*\(#(?<color>[0-9a-f]+)\)");

    lines.map(|line| {
        let captures = instruction_pattern
            .captures(&line)
            .expect("bad instruction");

        Instruction {
            direction: match captures.name("direction").unwrap().as_str() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            },
            distance: captures.name("distance").unwrap().as_str().parse().unwrap(),
            color: u32::from_str_radix(captures.name("color").unwrap().as_str(), 16).unwrap(),
        }
    })
}

fn to_vertices(instructions: &[Instruction]) -> Vec<Point> {
    let mut vertices = Vec::new();
    let mut position = Point { x: 0, y: 0 };

    for instruction in instructions {
        match instruction.direction {
            Direction::Up => position.y -= instruction.distance as i64,
            Direction::Down => position.y += instruction.distance as i64,
            Direction::Left => position.x -= instruction.distance as i64,
            Direction::Right => position.x += instruction.distance as i64,
        };

        vertices.push(position);
    }

    vertices
}

fn calculate_area(instructions: &[Instruction]) -> i64 {
    let tiles = to_vertices(instructions);

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let exterior_area = instructions.iter().fold(0, |s, i| s + i.distance as i64) / 2 + 1;
    let interior_area = tiles
        .iter()
        .zip(tiles[1..].iter())
        .fold(0, |s, (a, b)| s + (a.x * b.y) - (a.y * b.x))
        / 2;

    exterior_area + interior_area
}

fn fix_instructions(instructions: &mut [Instruction]) {
    for instruction in instructions {
        instruction.direction = match instruction.color % 16 {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => unreachable!(),
        };

        instruction.distance = instruction.color >> 4;
    }
}
