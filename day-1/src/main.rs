use std::io::stdin;

fn main() {
    let mut sum_of_digits = 0;
    let mut sum_of_all = 0;

    for line in stdin().lines().map(|l| l.unwrap()) {
        // part 1
        sum_of_digits += recover_calibration_value(line.chars().filter_map(|c| c.to_digit(10)));

        // part 2
        sum_of_all += recover_calibration_value(find_digits(line.as_str()));
    }

    println!("{}", sum_of_digits);
    println!("{}", sum_of_all);
}

/// Computes the value for a parsed line.
fn recover_calibration_value<I>(digits: I) -> u32
where
    I: IntoIterator<Item = u32>,
{
    let mut first = 0;
    let mut last = 0;

    for digit in digits {
        // exploit that 0 doesn't show up in the input 🙃
        if last == 0 {
            first = digit * 10;
        }

        last = digit;
    }

    first + last
}

/// Parses all digits out of a line. Digits may overlap; for instance,
/// `"oneight"` parses to `[1, 8]`.
fn find_digits(str: &str) -> impl Iterator<Item = u32> + '_ {
    str.char_indices().filter_map(|(index, char)| {
        char.to_digit(10).or_else(|| match &str[index..] {
            s if s.starts_with("zero") => Some(0),
            s if s.starts_with("one") => Some(1),
            s if s.starts_with("two") => Some(2),
            s if s.starts_with("three") => Some(3),
            s if s.starts_with("four") => Some(4),
            s if s.starts_with("five") => Some(5),
            s if s.starts_with("six") => Some(6),
            s if s.starts_with("seven") => Some(7),
            s if s.starts_with("eight") => Some(8),
            s if s.starts_with("nine") => Some(9),
            _ => None,
        })
    })
}
