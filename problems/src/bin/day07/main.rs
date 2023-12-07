use crate::HandType::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPairs,
};
use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    calculate_winnings(input, j_is_jack)
}

fn part2(input: &str) -> usize {
    calculate_winnings(input, j_is_joker)
}

fn calculate_winnings(input: &str, parse_card: impl Fn(char) -> i32 + Copy) -> usize {
    let mut hands = parse_input(input, parse_card);

    hands.sort_unstable_by(compare_hands);
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

type Hand = [i32; 5];

fn compare_hands((hand1, _): &(Hand, usize), (hand2, _): &(Hand, usize)) -> Ordering {
    hand_type(hand1)
        .cmp(&hand_type(hand2))
        .then_with(|| hand1.cmp(hand2))
}

fn j_is_jack(c: char) -> i32 {
    parse_card(c, 11)
}

fn j_is_joker(c: char) -> i32 {
    parse_card(c, 1)
}

fn parse_card(c: char, j_value: i32) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => j_value,
        'T' => 10,
        c if c.is_ascii_digit() => c.to_digit(10).unwrap() as i32,
        _ => unreachable!("invalid card"),
    }
}

fn parse_input(input: &str, parse_card: impl Fn(char) -> i32 + Copy) -> Vec<(Hand, usize)> {
    input
        .lines()
        .filter_map(|hand_bid| hand_bid.split_once(' '))
        .map(|(hand, bid)| (parse_hand(hand, parse_card), bid.parse::<usize>().unwrap()))
        .collect::<Vec<_>>()
}

fn parse_hand(hand: &str, parse_card: impl Fn(char) -> i32 + Copy) -> Hand {
    hand.chars()
        .map(parse_card)
        .collect::<Vec<_>>()
        .try_into()
        .expect("five cards")
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_type(hand: &Hand) -> HandType {
    let mut map = HashMap::<i32, i32>::new();
    for card in hand {
        *map.entry(*card).or_default() += 1;
    }
    let jokers = map.remove(&1).unwrap_or(0);
    if jokers == 5 {
        return FiveOfAKind;
    }

    let mut map = map.into_iter().collect::<Vec<_>>();
    map.sort_unstable_by_key(|(c, n)| (-*n, -c));
    map[0].1 += jokers;
    match map[..] {
        [(_, 5), ..] => FiveOfAKind,
        [(_, 4), ..] => FourOfAKind,
        [(_, 3), (_, 2), ..] => FullHouse,
        [(_, 3), ..] => ThreeOfAKind,
        [(_, 2), (_, 2), ..] => TwoPairs,
        [(_, 2), ..] => OnePair,
        [(_, 1), ..] => HighCard,
        _ => unreachable!(),
    }
}

#[test]
fn part1_example() {
    assert_eq!(6440, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(250058342, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(5905, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(250506580, part2(INPUT));
}
