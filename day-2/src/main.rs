use std::io::stdin;

use regex_macro::regex;

struct Handful {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    handfuls: Vec<Handful>,
}

fn main() {
    let mut sum_of_possible_ids = 0;
    let mut sum_of_powers = 0;

    for line in stdin().lines().map(|l| l.unwrap()) {
        let game = parse_game(&line);

        // part 1
        if is_possible_with(&game, 12, 13, 14) {
            sum_of_possible_ids += game.id;
        }

        // part 2
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for handful in game.handfuls.into_iter() {
            min_red = min_red.max(handful.red);
            min_green = min_green.max(handful.green);
            min_blue = min_blue.max(handful.blue);
        }

        sum_of_powers += min_red * min_green * min_blue;
    }

    println!("{}", sum_of_possible_ids);
    println!("{}", sum_of_powers);
}

fn parse_game(str: &str) -> Game {
    let captures = regex!(r"Game (?<id>\d+):\s*(?<handfuls>.+)")
        .captures(str)
        .expect(&format!("malformed game: {}", str));

    let id = u32::from_str_radix(captures.name("id").unwrap().as_str(), 10).unwrap();
    let handfuls = regex!(r";\s*")
        .split(captures.name("handfuls").unwrap().as_str())
        .map(|str| {
            let mut handful = Handful {
                red: 0,
                green: 0,
                blue: 0,
            };

            for set in regex!(r",\s*").split(str) {
                let captures = regex!(r"(?<count>\d+)\s*(?<color>red|green|blue)")
                    .captures(set)
                    .expect(&format!("malformed set: {}", set));

                let count = u32::from_str_radix(captures.name("count").unwrap().as_str(), 10).unwrap();
                let color = captures.name("color").unwrap().as_str();

                match color {
                    "red" => handful.red += count,
                    "green" => handful.green += count,
                    "blue" => handful.blue += count,
                    _ => unreachable!(),
                };
            }

            handful
        })
        .collect();

    Game { id, handfuls }
}

fn is_possible_with(game: &Game, red: u32, green: u32, blue: u32) -> bool {
    game.handfuls
        .iter()
        .all(|h| h.red <= red && h.green <= green && h.blue <= blue)
}
