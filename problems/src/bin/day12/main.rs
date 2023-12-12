use crate::Condition::{Damaged, Operational, Unknown};

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    parse_input(input).map(memoized_arrangements).sum()
}

fn part2(input: &str) -> usize {
    parse_input(input)
        .map(expand_row)
        .map(memoized_arrangements)
        .sum()
}

struct Memo {
    arrangements: Vec<Option<usize>>,
    conditions_length: usize,
}

impl Memo {
    fn new(conditions_length: usize, damaged_length: usize) -> Self {
        Self {
            arrangements: vec![None; (conditions_length + 1) * (damaged_length + 1)],
            conditions_length,
        }
    }

    #[inline]
    const fn memo_key(&self, conditions_length: usize, damaged_length: usize) -> usize {
        conditions_length + damaged_length * self.conditions_length
    }

    #[inline]
    fn set(&mut self, memo_key: usize, combinations: usize) {
        self.arrangements[memo_key] = Some(combinations)
    }
    #[inline]
    fn get(&self, memo_key: usize) -> Option<usize> {
        self.arrangements[memo_key]
    }
}

fn all_non_damaged(springs: &[Condition]) -> bool {
    springs.iter().all(|s| *s != Damaged)
}

fn non_operational(springs: &[Condition]) -> usize {
    springs
        .iter()
        .position(|c| *c == Operational)
        .unwrap_or(springs.len())
}

fn trim_operational(springs: &[Condition]) -> &[Condition] {
    let first = springs
        .iter()
        .position(|c| *c != Operational)
        .unwrap_or(springs.len());
    &springs[first..]
}

fn next_placement(springs: &[Condition], damaged_length: usize) -> Option<&[Condition]> {
    let non_operational = non_operational(springs);

    if non_operational == damaged_length {
        // An exact match. Fits like a glove.
        Some(&springs[damaged_length..])
    } else if non_operational > damaged_length && springs[damaged_length] != Damaged {
        // If not an exact match, leave space for an operational spring.
        Some(&springs[damaged_length + 1..])
    } else {
        None
    }
}

fn memoized_arrangements(row: Row) -> usize {
    let mut memo = Memo::new(row.conditions.len(), row.damaged_groups.len());

    fn arrangements(memo: &mut Memo, springs: &[Condition], damaged_groups: &[usize]) -> usize {
        let springs = trim_operational(springs);

        let memo_key = memo.memo_key(springs.len(), damaged_groups.len());
        if let Some(m) = memo.get(memo_key) {
            return m;
        }

        let result = match (springs, damaged_groups) {
            (_, []) if all_non_damaged(springs) => 1,
            ([_, ..], [group, remaining_groups @ ..]) => {
                let a = next_placement(springs, *group)
                    .map(|next| arrangements(memo, next, remaining_groups))
                    .unwrap_or(0);
                let b = (springs[0] == Unknown)
                    .then(|| arrangements(memo, &springs[1..], damaged_groups))
                    .unwrap_or(0);
                a + b
            }
            _ => 0,
        };

        memo.set(memo_key, result);
        result
    }

    arrangements(&mut memo, &row.conditions, &row.damaged_groups)
}

fn expand_row(mut row: Row) -> Row {
    row.conditions.push(Unknown);
    let mut conditions = row.conditions.repeat(5);
    conditions.pop();

    let damaged_groups = row.damaged_groups.repeat(5);

    Row {
        conditions,
        damaged_groups,
    }
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

struct Row {
    conditions: Vec<Condition>,
    damaged_groups: Vec<usize>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Row> + '_ {
    input.lines().map(parse_line)
}

fn parse_line(line: &str) -> Row {
    let (springs, groups) = line.split_once(' ').expect("springs and groups");
    let conditions = springs
        .chars()
        .map(|c| match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => unreachable!("invalid spring"),
        })
        .collect();
    let damaged_groups = groups
        .split(',')
        .map(|n| n.parse::<usize>().expect("group"))
        .collect();
    Row {
        conditions,
        damaged_groups,
    }
}

#[test]
fn part1_example() {
    assert_eq!(21, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(7_350, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(525_152, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(200_097_286_528_151, part2(INPUT));
}
