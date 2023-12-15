use std::cmp::min;
use std::collections::HashSet;
use Reflection::{Horizontal, Vertical};

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .flat_map(Pattern::mirrored_positions)
        .map(Reflection::to_summary)
        .sum()
}

fn part2(input: &str) -> usize {
    parse_input(input)
        .map(Pattern::find_smudged)
        .map(Reflection::to_summary)
        .sum()
}

#[derive(Clone)]
struct Pattern {
    by_row: Vec<u32>,
    by_column: Vec<u32>,
}

impl Pattern {
    fn each_position(&self) -> impl Iterator<Item = (usize, usize)> + 'static {
        let width = self.by_column.len();
        let height = self.by_row.len();
        (0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }

    #[inline]
    fn smudge_position(mut self, x: usize, y: usize) -> Self {
        self.by_row[y] ^= 1 << x;
        self.by_column[x] ^= 1 << y;
        self
    }

    fn smudge_each(self) -> impl Iterator<Item = Self> + 'static {
        self.each_position()
            .map(move |(x, y)| self.clone().smudge_position(x, y))
    }

    fn mirrored_positions(self) -> impl Iterator<Item = Reflection> + 'static {
        let h = (1..self.by_column.len())
            .filter(move |i| is_mirrored_at(*i, &self.by_column))
            .map(Horizontal);
        let v = (1..self.by_row.len())
            .filter(move |i| is_mirrored_at(*i, &self.by_row))
            .map(Vertical);

        h.chain(v)
    }

    fn find_smudged(self) -> Reflection {
        let original_positions = self.clone().mirrored_positions().collect::<HashSet<_>>();

        self.smudge_each()
            .flat_map(Pattern::mirrored_positions)
            .find(|r| !original_positions.contains(r))
            .unwrap()
    }
}

#[inline]
fn is_mirrored_at(position: usize, tiles: &[u32]) -> bool {
    let len = min(position, tiles.len() - position);

    tiles[position - len..position]
        .iter()
        .eq(tiles[position..position + len].iter().rev())
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Reflection {
    fn to_summary(self) -> usize {
        match self {
            Horizontal(i) => i,
            Vertical(i) => i * 100,
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Pattern> + '_ {
    input.split("\n\n").map(parse_pattern)
}

fn parse_pattern(input: &str) -> Pattern {
    let input = input.trim();
    let width = input.chars().position(|c| c == '\n').expect("first line");
    let height = input.len() / (width + 1) + 1;

    let mut by_row = vec![0; height];
    let mut by_column = vec![0; width];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let c = match c {
                '#' => 1,
                '.' => 0,
                _ => unreachable!("invalid input"),
            };

            by_row[y] |= c << (x as u32);
            by_column[x] |= c << (y as u32);
        }
    }
    Pattern { by_column, by_row }
}

#[test]
fn part1_example() {
    assert_eq!(405, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(34_889, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(400, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(34_224, part2(INPUT));
}
