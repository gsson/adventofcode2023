#![feature(iter_map_windows)]

use std::fmt::Debug;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    input.lines().map(parse_values).map(predict).sum::<i32>()
}

fn predict(history: Vec<i32>) -> i32 {
    let mut n = 0;
    let mut last = history;
    while last.iter().any(|v| *v != 0) {
        n += *last.last().unwrap();
        last = derive(&last);
    }

    n
}

fn derive(values: &[i32]) -> Vec<i32> {
    values.iter().map_windows(|[a, b]| **b - **a).collect()
}

fn parse_values<T>(line: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    line.split_ascii_whitespace()
        .map(|v| v.parse::<T>().unwrap())
        .collect()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let mut v = parse_values(l);
            v.reverse();
            v
        })
        .map(predict)
        .sum::<i32>()
}

#[test]
fn part1_example() {
    assert_eq!(114, part1(include_str!("example1.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(1842168671, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(2, part2(include_str!("example1.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(903, part2(INPUT));
}
