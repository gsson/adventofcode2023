use crate::Cards::{Resolved, Unresolved};
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(parse_card)
        .map(|c| (1i32 << c) >> 1)
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    let mut cards = input
        .lines()
        .filter_map(parse_card)
        .map(|won_cards| {
            if won_cards == 0 {
                Resolved(0)
            } else {
                Unresolved(won_cards)
            }
        })
        .collect::<Vec<_>>();

    (0..cards.len())
        .map(|i| scratch_and_win(&mut cards[i..]))
        .sum::<i32>()
        + cards.len() as i32
}

fn parse_card(line: &str) -> Option<usize> {
    let (_card, numbers) = line.split_once(": ")?;
    let (winning_numbers, numbers) = numbers.split_once(" | ")?;
    let winning_numbers = winning_numbers.split_whitespace().collect::<HashSet<_>>();
    let won_cards = numbers
        .split_whitespace()
        .filter(|h| winning_numbers.contains(h))
        .count();
    Some(won_cards)
}

#[derive(Copy, Clone)]
enum Cards {
    Resolved(i32),
    Unresolved(usize),
}

fn scratch_and_win(cards: &mut [Cards]) -> i32 {
    match cards[0] {
        Resolved(n) => n,
        Unresolved(won_cards) => {
            let n = (1..won_cards + 1)
                .map(|i| scratch_and_win(&mut cards[i..]))
                .sum::<i32>()
                + won_cards as i32;
            cards[0] = Resolved(n);
            n
        }
    }
}

#[test]
fn part1_example() {
    assert_eq!(13, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(21919, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(30, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(9881048, part2(INPUT));
}
