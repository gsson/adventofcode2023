use std::cmp::max;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(parse_game)
        .filter(is_possible)
        .map(|(id, _)| id)
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .filter_map(parse_game)
        .map(min_required_cubes)
        .map(|c| c.into_iter().product::<i32>())
        .sum::<i32>()
}

fn parse_game(line: &str) -> Option<(i32, Vec<[i32; 3]>)> {
    let (game, subsets) = line.split_once(": ")?;
    let game = game.strip_prefix("Game ")?.parse::<i32>().ok()?;
    let subsets = subsets
        .split("; ")
        .filter_map(parse_subset)
        .collect::<Vec<_>>();
    Some((game, subsets))
}

fn parse_subset(subset: &str) -> Option<[i32; 3]> {
    subset
        .split(", ")
        .filter_map(|colour| colour.split_once(' '))
        .try_fold([0, 0, 0], |[r, g, b], (n, colour)| {
            let n = n.parse::<i32>().ok()?;
            match colour {
                "red" => Some([n, g, b]),
                "green" => Some([r, n, b]),
                "blue" => Some([r, g, n]),
                _ => unreachable!(),
            }
        })
}

fn is_possible((_, rounds): &(i32, Vec<[i32; 3]>)) -> bool {
    rounds
        .iter()
        .copied()
        .all(|[r, g, b]| r <= 12 && g <= 13 && b <= 14)
}

fn min_required_cubes((_, rounds): (i32, Vec<[i32; 3]>)) -> [i32; 3] {
    rounds
        .iter()
        .copied()
        .fold([0, 0, 0], |[max_r, max_g, max_b], [r, g, b]| {
            [max(max_r, r), max(max_g, g), max(max_b, b)]
        })
}

#[test]
fn part1_example() {
    assert_eq!(8, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(2348, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(2286, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(76008, part2(INPUT));
}
