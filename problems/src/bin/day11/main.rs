use std::fmt::Debug;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", solve(INPUT, 2));
    eprintln!("{}", solve(INPUT, 1_000_000));
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| parse_line(line).map(move |x| p(x as i64, y as i64)))
        .collect()
}

fn parse_line(line: &str) -> impl Iterator<Item = usize> + '_ {
    line.chars()
        .enumerate()
        .filter_map(|(x, c)| (c == '#').then_some(x))
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point([i64; 2]);

const fn p(x: i64, y: i64) -> Point {
    Point([x, y])
}

fn expand(mut v: Vec<i64>, expansion_factor: i64) -> Vec<i64> {
    let expansion_factor = expansion_factor - 1;
    let mut offset = 0;
    let mut last_p = 0;
    v.iter_mut().for_each(|p| {
        let p_diff = *p - last_p - 1;
        if p_diff > 0 {
            offset += p_diff * expansion_factor;
        }
        last_p = *p;
        *p += offset;
    });
    v
}

fn sum_absolute_pairwise_differences(v: Vec<i64>) -> i64 {
    let n = v.len() - 1;
    v.into_iter()
        .enumerate()
        .map(|(i, p)| i as i64 * p - (n - i) as i64 * p)
        .sum()
}

fn solve(input: &str, expansion_factor: i64) -> i64 {
    let input = parse_input(input);
    let mut xs = input
        .iter()
        .copied()
        .map(|Point([x, _])| x)
        .collect::<Vec<_>>();
    xs.sort_unstable();
    let mut ys = input
        .iter()
        .copied()
        .map(|Point([_, y])| y)
        .collect::<Vec<_>>();
    ys.sort_unstable();
    let xs = expand(xs, expansion_factor);
    let ys = expand(ys, expansion_factor);

    sum_absolute_pairwise_differences(xs) + sum_absolute_pairwise_differences(ys)
}

#[test]
fn part1_example() {
    assert_eq!(374, solve(include_str!("example.txt"), 2));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(9522407, solve(INPUT, 2));
}

#[test]
fn part2_example() {
    assert_eq!(1030, solve(include_str!("example.txt"), 10));
    assert_eq!(8410, solve(include_str!("example.txt"), 100));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(544723432977, solve(INPUT, 1_000_000));
}
