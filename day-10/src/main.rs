use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    vec,
};

use regex_macro::regex;

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Connection {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct Map {
    grid: HashMap<Point, Connection>,
    start: Point,
}

fn main() {
    let map = parse_map(stdin().lines().map(|l| l.unwrap()));
    let path = find_loop(&map).expect("no loop");

    // part 1
    let halfway = path.len() / 2;

    // part 2
    let area = calculate_area(&map.grid, &path);

    println!("{}", halfway);
    println!("{}", area);
}

fn parse_map<I>(lines: I) -> Map
where
    I: IntoIterator<Item = String>,
{
    let tile_pattern = regex!(r"([|\-LJ7FS])");

    let mut grid = HashMap::new();
    let mut start = None;

    for (y, line) in lines.into_iter().enumerate() {
        for m in tile_pattern.find_iter(&line) {
            let position = Point { x: m.start(), y };

            match m.as_str() {
                "S" => {
                    start = Some(position);
                }
                _ => {
                    grid.insert(
                        position,
                        match m.as_str() {
                            "|" => Connection::NorthSouth,
                            "-" => Connection::EastWest,
                            "L" => Connection::NorthEast,
                            "J" => Connection::NorthWest,
                            "F" => Connection::SouthEast,
                            "7" => Connection::SouthWest,
                            _ => unreachable!(),
                        },
                    );
                }
            };
        }
    }

    Map {
        grid,
        start: start.expect("no start"),
    }
}

fn next_heading(heading: &Direction, connection: &Connection) -> Option<Direction> {
    match (heading, connection) {
        (Direction::North, Connection::NorthSouth) => Some(Direction::North),
        (Direction::North, Connection::SouthWest) => Some(Direction::West),
        (Direction::North, Connection::SouthEast) => Some(Direction::East),
        (Direction::East, Connection::EastWest) => Some(Direction::East),
        (Direction::East, Connection::NorthWest) => Some(Direction::North),
        (Direction::East, Connection::SouthWest) => Some(Direction::South),
        (Direction::South, Connection::NorthSouth) => Some(Direction::South),
        (Direction::South, Connection::NorthEast) => Some(Direction::East),
        (Direction::South, Connection::NorthWest) => Some(Direction::West),
        (Direction::West, Connection::EastWest) => Some(Direction::West),
        (Direction::West, Connection::NorthEast) => Some(Direction::North),
        (Direction::West, Connection::SouthEast) => Some(Direction::South),
        _ => None,
    }
}

fn next_position(point: &Point, heading: &Direction) -> Point {
    match heading {
        Direction::North => Point {
            x: point.x,
            y: point.y - 1,
        },
        Direction::East => Point {
            x: point.x + 1,
            y: point.y,
        },
        Direction::South => Point {
            x: point.x,
            y: point.y + 1,
        },
        Direction::West => Point {
            x: point.x - 1,
            y: point.y,
        },
    }
}

fn walk(
    grid: &HashMap<Point, Connection>,
    mut position: Point,
    mut heading: Direction,
) -> Option<Vec<Point>> {
    let end = position;
    let mut path = vec![];

    loop {
        position = next_position(&position, &heading);
        path.push(position);

        if position == end {
            break;
        }

        if let Some(connection) = grid.get(&position) {
            if let Some(next_heading) = next_heading(&heading, connection) {
                heading = next_heading;
                continue;
            }
        }

        return None;
    }

    return Some(path);
}

fn find_loop(map: &Map) -> Option<Vec<Point>> {
    [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .filter_map(|&heading| walk(&map.grid, map.start, heading))
    .nth(0)
}

fn calculate_area(grid: &HashMap<Point, Connection>, path: &Vec<Point>) -> usize {
    let mut points = HashSet::new();
    let mut width = 0;
    let mut height = 0;

    for point in path {
        points.insert(point);
        width = width.max(point.x + 1);
        height = height.max(point.y + 1);
    }

    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if points.contains(&Point { x, y }) {
                continue;
            }

            count += (0..x)
                .filter_map(|rx| {
                    let r = Point { x: rx, y };

                    if points.contains(&r) {
                        grid.get(&r)
                    } else {
                        None
                    }
                })
                .filter(|c| {
                    matches!(
                        c,
                        Connection::NorthSouth | Connection::NorthEast | Connection::NorthWest
                    )
                })
                .count()
                & 1;
        }
    }

    count
}
