use adventofcode2023_common::StrExt;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let parts = parts_iter(input, next_part).collect::<HashSet<_>>();

    numbers_iter(input)
        .filter(|((x1, x2), y, _)| adjacent(*x1, *x2, *y).any(|xy| parts.contains(&xy)))
        .map(|(.., n)| n.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    let gears = parts_iter(input, next_gear).collect::<HashSet<_>>();

    let ratios = numbers_iter(input)
        .flat_map(|((x1, x2), y, n)| adjacent(x1, x2, y).map(move |xy| (xy, n)))
        .filter(|(xy, ..)| gears.contains(xy))
        .fold(HashMap::<_, Vec<_>>::new(), |mut ratios, (xy, n)| {
            ratios.entry(xy).or_default().push(n);
            ratios
        });

    ratios
        .values()
        .filter_map(|v| match v[..] {
            [a, b] => Some(a.to_i32() * b.to_i32()),
            _ => None,
        })
        .sum()
}

fn parts_iter(
    input: &str,
    searcher: fn(&str) -> Option<(usize, usize)>,
) -> impl Iterator<Item = (i32, i32)> + '_ {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| find(line, searcher).map(move |((x, _), _)| (x, y as i32)))
}

fn numbers_iter(input: &str) -> impl Iterator<Item = ((i32, i32), i32, &str)> + '_ {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| find(line, next_number).map(move |(xs, s)| (xs, y as i32, s)))
}

fn adjacent(x1: i32, x2: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    let above = (x1 - 1..x2 + 1).map(move |x| (x, y - 1));
    let below = (x1 - 1..x2 + 1).map(move |x| (x, y + 1));
    above.chain(below).chain([(x1 - 1, y), (x2, y)])
}

fn find(
    line: &str,
    searcher: fn(&str) -> Option<(usize, usize)>,
) -> impl Iterator<Item = ((i32, i32), &str)> + '_ {
    let mut pos = 0;
    std::iter::from_fn(move || {
        let (start, end) = searcher(&line[pos..]).map(|(s, e)| (s + pos, e + pos))?;
        pos = end;
        Some(((start as i32, end as i32), &line[start..end]))
    })
}

fn next_number(line: &str) -> Option<(usize, usize)> {
    let start = line.find(|c: char| c.is_ascii_digit())?;
    let end = line[start..]
        .find(|c: char| !c.is_ascii_digit())
        .map(|pos| pos + start)
        .unwrap_or(line.len());
    Some((start, end))
}

fn next_part(line: &str) -> Option<(usize, usize)> {
    let start = line.find(|c: char| c != '.' && !c.is_ascii_digit())?;
    Some((start, start + 1))
}

fn next_gear(line: &str) -> Option<(usize, usize)> {
    let start = line.find(|c: char| c == '*')?;
    Some((start, start + 1))
}

#[test]
fn part1_example() {
    assert_eq!(4361, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(514969, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(467835, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(78915902, part2(INPUT));
}
