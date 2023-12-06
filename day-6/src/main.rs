use std::{io::stdin, iter::zip};

#[derive(Default)]
struct Race {
    time: u64,
    distance: u64,
}

fn main() {
    // part 1
    let races = parse_races(stdin().lines().map(|l| l.unwrap()));
    let ways_product = races.iter().map(|r| ways_to_win(&r)).fold(1, |p, c| p * c);

    // part 2
    let megarace = combine_races(&races);
    let ways_megarace = ways_to_win(&megarace);

    println!("{}", ways_product);
    println!("{}", ways_megarace);
}

fn parse_races<I>(lines: I) -> Vec<Race>
where
    I: IntoIterator<Item = String>,
{
    let mut rows = lines.into_iter().map(|l| {
        l.split_whitespace()
            .skip(1)
            .map(|s| u64::from_str_radix(s, 10).unwrap())
            .collect::<Vec<_>>()
    });

    let times = rows.next().expect("missing times");
    let distances = rows.next().expect("missing distances");

    zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn combine_races(races: &Vec<Race>) -> Race {
    fn concat<I>(numbers: I) -> u64
    where
        I: Iterator<Item = u64>,
    {
        numbers
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse()
            .unwrap()
    }

    Race {
        time: concat(races.iter().map(|r| r.time)),
        distance: concat(races.iter().map(|r| r.distance)),
    }
}

fn ways_to_win(race: &Race) -> u64 {
    (0..race.time)
        .filter(|&t| ((race.time - t) * t) > race.distance)
        .count() as _
}
