#![feature(isqrt)]

use std::iter::zip;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

// distance = (race time - press time) * speed
// speed = press time
// d = (tr - tp) * tp
// d = tr * tp - tp^2
// tp = (tr +- sqrt(tr * tr - 4 * d)) / 2

fn part1(input: &str) -> u64 {
    let races = parse_races1(input).expect("races");
    races.into_iter().map(|(t, d)| solve(t, d)).product::<u64>()
}

fn part2(input: &str) -> u64 {
    let (time, distance) = parse_races2(input).expect("races");
    solve(time, distance)
}

fn solve(race_time: i64, distance_to_beat: i64) -> u64 {
    let d = distance_to_beat + 1;
    let a = (race_time - (race_time * race_time - 4 * d).isqrt() + 1) / 2;
    let b = (race_time + (race_time * race_time - 4 * d).isqrt()) / 2;
    a.abs_diff(b) + 1
}

fn parse_races1(input: &str) -> Option<Vec<(i64, i64)>> {
    let (time, distance) = input.split_once('\n')?;
    let time = time.strip_prefix("Time:")?.trim();
    let distance = distance.strip_prefix("Distance:")?.trim();
    let time = time
        .split_whitespace()
        .map(|t| t.parse::<i64>().ok())
        .collect::<Option<Vec<_>>>()?;
    let distance = distance
        .split_whitespace()
        .map(|t| t.parse::<i64>().ok())
        .collect::<Option<Vec<_>>>()?;
    let td = zip(time, distance).collect::<Vec<_>>();
    Some(td)
}

fn parse_races2(input: &str) -> Option<(i64, i64)> {
    let (time, distance) = input.split_once('\n')?;
    let time = time
        .strip_prefix("Time:")?
        .replace(|c: char| c.is_ascii_whitespace(), "");
    let distance = distance
        .strip_prefix("Distance:")?
        .replace(|c: char| c.is_ascii_whitespace(), "");
    let time = time.parse::<i64>().ok()?;
    let distance = distance.parse::<i64>().ok()?;
    Some((time, distance))
}

#[test]
fn part1_example() {
    assert_eq!(288, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(588588, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(71503, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(34655848, part2(INPUT));
}
