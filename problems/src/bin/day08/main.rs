use crate::Step::{Left, Right};
use adventofcode2023_common::Gcd;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let input = parse(input, |s| s == "AAA", |s| s == "ZZZ");
    solve(input)
}

fn part2(input: &str) -> usize {
    let input = parse(input, |s| s.ends_with('A'), |s| s.ends_with('Z'));
    solve(input)
}

struct Input {
    instructions: Vec<Step>,
    starts: Vec<usize>,
    nodes: Vec<([usize; 2], bool)>,
}

fn solve(input: Input) -> usize {
    input
        .starts
        .iter()
        .map(|start| count_steps(&input.instructions, *start, &input.nodes))
        .reduce(usize::lcm)
        .expect("solution")
}

fn count_steps(instructions: &[Step], start: usize, nodes: &[([usize; 2], bool)]) -> usize {
    let mut here = start;
    let mut i = 0;
    while !nodes[here].1 {
        here = match instructions[i % instructions.len()] {
            Left => nodes[here].0[0],
            Right => nodes[here].0[1],
        };

        i += 1;
    }

    i
}

#[repr(u8)]
#[derive(Debug)]
enum Step {
    Left,
    Right,
}

fn parse_instruction(instructions: &str) -> Vec<Step> {
    instructions
        .chars()
        .map(|c: char| match c {
            'L' => Left,
            'R' => Right,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
}

#[derive(Default)]
struct NameIndex<'a>(HashMap<&'a str, usize>);
impl<'a> NameIndex<'a> {
    fn index(&mut self, name: &'a str) -> usize {
        let next_id = self.0.len();
        *self.0.entry(name).or_insert(next_id)
    }
}

fn parse(
    input: &str,
    is_start: impl Fn(&str) -> bool,
    is_terminal: impl Fn(&str) -> bool,
) -> Input {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let instructions = parse_instruction(instructions);
    let mut name_index = NameIndex::default();
    let mut node_index = Vec::new();
    let mut starts = Vec::new();

    for line in nodes.lines() {
        let (from, left_right) = line.split_once(" = ").unwrap();

        let is_start = is_start(from);
        let is_terminal = is_terminal(from);
        let from = name_index.index(from);
        if is_start {
            starts.push(from);
        }

        let (left, right) = left_right
            .trim_matches(['(', ')'])
            .split_once(", ")
            .unwrap();
        let left = name_index.index(left);
        let right = name_index.index(right);

        while node_index.len() <= from {
            node_index.push(Default::default());
        }

        node_index[from] = ([left, right], is_terminal);
    }

    Input {
        instructions,
        starts,
        nodes: node_index,
    }
}

#[test]
fn part1_example1() {
    assert_eq!(2, part1(include_str!("example1.txt")));
}

#[test]
fn part1_example2() {
    assert_eq!(6, part1(include_str!("example2.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(12169, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(6, part2(include_str!("example3.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(12030780859469, part2(INPUT));
}
