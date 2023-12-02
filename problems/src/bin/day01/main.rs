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
    input.lines().map(|line| parse_digits(line, table)).sum()
}

fn parse_digits(s: &str, table: &[(&str, i32)]) -> i32 {
    let mut it = str_tails(s).filter_map(|tail| match_digit(tail, table));

    let tens = it.next().expect("digit");
    let ones = it.last().unwrap_or(tens);
    tens * 10 + ones
}

fn str_tails(mut s: &str) -> impl Iterator<Item = &str> + '_ {
    std::iter::from_fn(move || match s {
        "" => None,
        next => {
            s = &next[1..];
            Some(next)
        }
    })
}

fn match_digit(s: &str, table: &[(&str, i32)]) -> Option<i32> {
    for (n, v) in table {
        if s.starts_with(n) {
            return Some(*v);
        }
    }

    None
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
