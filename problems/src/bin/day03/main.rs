use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let parts = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| find_parts(y, line))
        .collect::<HashSet<_>>();

    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| find_numbers(y, line))
        .collect::<Vec<_>>();

    numbers
        .into_iter()
        .filter(|((x1, x2), y, _)| adjacent(*x1, *x2, *y).any(|xy| parts.contains(&xy)))
        .map(|(.., n)| n)
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    let gears = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| find_gears(y, line))
        .collect::<HashSet<_>>();

    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| find_numbers(y, line))
        .collect::<Vec<_>>();

    let ratios = numbers
        .into_iter()
        .flat_map(|((x1, x2), y, n)| adjacent(x1, x2, y).map(move |xy| (xy, n)))
        .filter(|(xy, ..)| gears.contains(xy))
        .fold(
            HashMap::<(i32, i32), Vec<i32>>::new(),
            |mut ratios, (xy, n)| {
                ratios.entry(xy).or_default().push(n);
                ratios
            },
        );

    ratios
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<i32>())
        .sum()
}

fn adjacent(x1: i32, x2: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    let left = std::iter::once((x1 - 1, y));
    let right = std::iter::once((x2, y));
    let above = (x1 - 1..x2 + 1).map(move |x| (x, y - 1));
    let below = (x1 - 1..x2 + 1).map(move |x| (x, y + 1));
    above.chain(below).chain(left).chain(right)
}

fn find_numbers(y: usize, line: &str) -> impl Iterator<Item = ((i32, i32), i32, i32)> + '_ {
    fn next_number(p: usize, line: &str) -> Option<(usize, usize)> {
        let start = line[p..].find(|c: char| c.is_ascii_digit())? + p;
        let end = line[start..]
            .find(|c: char| !c.is_ascii_digit())
            .map(|e| e + start)
            .unwrap_or(line.len());
        Some((start, end))
    }
    let mut pos = 0;
    std::iter::from_fn(move || {
        let (start, end) = next_number(pos, line)?;
        pos = end;
        Some((
            (start as i32, end as i32),
            y as i32,
            line[start..end].parse().unwrap(),
        ))
    })
}

fn find_parts(y: usize, line: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    fn next(p: usize, line: &str) -> Option<usize> {
        line[p..]
            .find(|c: char| c != '.' && !c.is_ascii_digit())
            .map(|s| s + p)
    }
    let mut pos = 0;
    std::iter::from_fn(move || {
        let x = next(pos, line)?;
        pos = x + 1;
        Some((x as i32, y as i32))
    })
}

fn find_gears(y: usize, line: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    fn next(p: usize, line: &str) -> Option<usize> {
        line[p..].find(|c: char| c == '*').map(|s| s + p)
    }
    let mut pos = 0;
    std::iter::from_fn(move || {
        let x = next(pos, line)?;
        pos = x + 1;
        Some((x as i32, y as i32))
    })
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
