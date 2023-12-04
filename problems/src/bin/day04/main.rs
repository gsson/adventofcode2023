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

    scratch_and_win(&mut Unresolved(cards.len()), &mut cards)
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

fn scratch_and_win(head: &mut Cards, tail: &mut [Cards]) -> i32 {
    match *head {
        Resolved(n) => n,
        Unresolved(won_cards) => {
            let n = (0..won_cards)
                .map(|i| match &mut tail[i..] {
                    [head, tail @ ..] => scratch_and_win(head, tail),
                    _ => 0,
                })
                .sum::<i32>()
                + won_cards as i32;
            *head = Resolved(n);
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
