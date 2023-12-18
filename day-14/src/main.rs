use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    io::stdin,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Hash)]
enum Shape {
    Cube,
    Sphere,
}

#[derive(Clone, Copy, Hash)]
struct Rock {
    position: Point,
    shape: Shape,
}

#[derive(Hash)]
struct Platform {
    rocks: Vec<Rock>,
    length: usize,
}

fn main() {
    let mut p1 = parse_image(stdin().lines().map(|l| l.unwrap()));
    let mut p2 = Platform {
        rocks: p1.rocks.to_vec(),
        ..p1
    };

    // part 1
    p1.roll();
    let sum = calculate_load(&p1);

    // part 2
    p2.cycle_times(1_000_000_000);
    let sum_of_cycled: usize = calculate_load(&p2);

    println!("{}", sum);
    println!("{}", sum_of_cycled);
}

fn parse_image<I>(lines: I) -> Platform
where
    I: Iterator<Item = String>,
{
    let mut rocks = Vec::new();
    let mut length = 0;

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let position = Point { x, y };

            match char {
                'O' => rocks.push(Rock {
                    position,
                    shape: Shape::Sphere,
                }),
                '#' => rocks.push(Rock {
                    position,
                    shape: Shape::Cube,
                }),
                _ => (),
            }
        }

        length = length.max(y + 1)
    }

    Platform { length, rocks }
}

impl Platform {
    fn roll(&mut self) {
        for i in 0..self.rocks.len() {
            if matches!(self.rocks[i].shape, Shape::Cube) {
                continue;
            }

            let Point { x, y } = self.rocks[i].position;

            self.rocks[i].position.y = self
                .rocks
                .iter()
                .filter(|&o| o.position.x == x && o.position.y < y)
                .fold(0, |y, r| match r.shape {
                    Shape::Cube => r.position.y + 1,
                    Shape::Sphere => y + 1,
                });
        }
    }

    fn rotate(&mut self) {
        for i in 0..self.rocks.len() {
            let Point { x, y } = self.rocks[i].position;

            self.rocks[i].position.x = self.length - y - 1;
            self.rocks[i].position.y = x;
        }

        self.rocks.sort_by(|a, b| a.position.cmp(&b.position));
    }

    fn cycle(&mut self) {
        for _ in 0..4 {
            self.roll();
            self.rotate();
        }
    }

    fn cycle_times(&mut self, n: usize) {
        let mut seen = HashMap::new();
        let mut period = 0;
        let mut i = 0;

        while i < n {
            let hash = calculate_hash(&self.rocks);

            if let Some(&last) = seen.get(&hash) {
                period = i - last;
                break;
            }

            seen.insert(hash, i);
            self.cycle();
            i += 1;
        }

        for _ in 0..((n - i) % period) {
            self.cycle();
        }
    }
}

fn calculate_hash(rocks: &Vec<Rock>) -> u64 {
    rocks
        .iter()
        .fold(DefaultHasher::new(), |mut hasher, rock| {
            rock.hash(&mut hasher);
            hasher
        })
        .finish()
}

fn calculate_load(platform: &Platform) -> usize {
    platform
        .rocks
        .iter()
        .filter(|&rock| matches!(rock.shape, Shape::Sphere))
        .map(|rock| platform.length - rock.position.y)
        .sum()
}
