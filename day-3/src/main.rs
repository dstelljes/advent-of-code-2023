use std::{collections::HashSet, io::stdin};

use regex_macro::regex;

enum Component {
    Number(u32),
    Symbol(char),
}

struct Point {
    x: usize,
    y: usize,
}

struct Element {
    component: Component,
    position: Point,
    length: usize,
}

struct Schematic<'a> {
    grid: Vec<Option<&'a Element>>,
    width: usize,
    height: usize,
}

fn main() {
    let mut lines = stdin().lines().map(|l| l.unwrap()).peekable();

    let width = lines.peek().map_or(0, |l| l.len());
    let elements = find_elements(lines).collect::<Vec<_>>();
    let schematic = Schematic::from_elements(&elements, width);

    // part 1
    let sum_of_part_numbers: u32 = elements
        .iter()
        .filter_map(|e| schematic.get_part_number(e))
        .sum();

    // part 2
    let sum_of_gear_ratios: u32 = elements
        .iter()
        .filter_map(|e| schematic.get_gear_ratio(e))
        .sum();

    println!("{}", sum_of_part_numbers);
    println!("{}", sum_of_gear_ratios);
}

fn find_elements<I>(lines: I) -> impl Iterator<Item = Element>
where
    I: IntoIterator<Item = String>,
{
    let element_pattern = regex!(r"(\d+|[^.])");

    lines.into_iter().enumerate().flat_map(|(y, line)| {
        element_pattern
            .find_iter(&line)
            .map(|m| Element {
                component: match u32::from_str_radix(m.as_str(), 10) {
                    Ok(number) => Component::Number(number),
                    Err(_) => Component::Symbol(m.as_str().chars().nth(0).unwrap()),
                },
                position: Point { x: m.start(), y },
                length: m.len(),
            })
            .collect::<Vec<_>>()
    })
}

impl<'a> Schematic<'a> {
    /// Builds a schematic containing the provided elements.
    fn from_elements(elements: &'a [Element], width: usize) -> Self {
        let height = elements.iter().map(|e| e.position.y + 1).max().unwrap_or(0);
        let mut grid = vec![None; width * height];

        for element in elements {
            let start = ((width * element.position.y) + element.position.x) as usize;

            for i in start..(start + element.length) {
                grid[i] = Some(element);
            }
        }

        Self {
            grid,
            width,
            height,
        }
    }

    /// Returns all elements in the schematic that are adjacent (including
    /// diagonally) to the provided element.
    fn find_adjacent_elements(&self, element: &'a Element) -> impl Iterator<Item = &Element> {
        let min = Point {
            x: element.position.x.max(1) - 1,
            y: element.position.y.max(1) - 1,
        };

        let max = Point {
            x: (element.position.x + element.length).min(self.width - 2) + 1,
            y: element.position.y.min(self.height - 2) + 1,
        };

        (min.y..=max.y).flat_map(move |y| {
            let row_start = y * self.width;
            let row = &self.grid[row_start..(row_start + self.width)];

            // avoid returning dupes:
            let mut seen = HashSet::new();

            // prevent element from appearing in its own results:
            if y == element.position.y {
                seen.insert(element.position.x);
            }

            row[min.x..max.x]
                .iter()
                .filter_map(|&pointer| pointer)
                .filter(move |&pointer| seen.insert(pointer.position.x))
        })
    }

    /// If `element` is a part (adjacent to a symbol), return its value.
    fn get_part_number(&self, element: &Element) -> Option<u32> {
        match element.component {
            Component::Number(value)
                if self
                    .find_adjacent_elements(element)
                    .any(|e| matches!(e.component, Component::Symbol(_))) =>
            {
                Some(value)
            }
            _ => None,
        }
    }

    /// If `element` is a gear (adjacent to exactly two parts), return the
    /// product of the parts.
    fn get_gear_ratio(&self, element: &Element) -> Option<u32> {
        match element.component {
            Component::Symbol('*') => {
                let adjacent_part_numbers = self
                    .find_adjacent_elements(element)
                    .filter_map(|e| self.get_part_number(e))
                    .collect::<Vec<_>>();

                if adjacent_part_numbers.len() == 2 {
                    Some(adjacent_part_numbers.iter().product())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
