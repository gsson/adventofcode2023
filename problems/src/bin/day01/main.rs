const INPUT: &str = include_str!("input.txt");

const PART1_DIGITS: [(&str, i32); 9] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

const PART2_DIGITS: [(&str, i32); 18] = [
    ("1", 1),
    ("one", 1),
    ("2", 2),
    ("two", 2),
    ("3", 3),
    ("three", 3),
    ("4", 4),
    ("four", 4),
    ("5", 5),
    ("five", 5),
    ("6", 6),
    ("six", 6),
    ("7", 7),
    ("seven", 7),
    ("8", 8),
    ("eight", 8),
    ("9", 9),
    ("nine", 9),
];

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    calibration_value(input, &PART1_DIGITS)
}

fn part2(input: &str) -> i32 {
    calibration_value(input, &PART2_DIGITS)
}

fn calibration_value(input: &str, table: &[(&str, i32)]) -> i32 {
    input
        .lines()
        .map(|line| parse_digits(line, table))
        .map(|digits| make_number(&digits))
        .sum()
}

fn match_digit(s: &str, table: &[(&str, i32)]) -> Option<i32> {
    for (n, v) in table {
        if s.starts_with(n) {
            return Some(*v);
        }
    }

    None
}

fn parse_digits(mut s: &str, table: &[(&str, i32)]) -> Vec<i32> {
    let mut digits = Vec::new();
    while !s.is_empty() {
        if let Some(digit) = match_digit(s, table) {
            digits.push(digit);
        }
        s = &s[1..];
    }
    digits
}

fn make_number(digits: &[i32]) -> i32 {
    match digits[..] {
        [digit] => digit * 10 + digit,
        [tens, .., ones] => tens * 10 + ones,
        _ => unreachable!(),
    }
}

#[test]
fn part1_example() {
    assert_eq!(142, part1(include_str!("example1.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(55017, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(281, part2(include_str!("example2.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(53539, part2(INPUT));
}
