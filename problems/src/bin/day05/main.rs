use std::cmp::{max, min};
use std::fmt::{Debug, Formatter};

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> i64 {
    let (seeds, tables) = parse_input1(input).expect("invalid input");

    find_min_location(seeds, tables)
}

fn part2(input: &str) -> i64 {
    let (seeds, tables) = parse_input2(input).expect("invalid input");

    find_min_location(seeds, tables)
}

fn find_min_location(mut values: Vec<Range>, tables: Vec<Table>) -> i64 {
    for table in &tables {
        values = values
            .into_iter()
            .flat_map(|v| map_range(v, table))
            .collect::<Vec<_>>();
    }

    values
        .into_iter()
        .map(|r| r.start)
        .min()
        .expect("at least one value")
}

fn map_range(v: Range, table: &Table) -> Vec<Range> {
    let Some(entries) = table.find_range(&v) else {
        return vec![v];
    };

    let mut result = Vec::new();
    let mut remaining = Some(v);
    for entry in entries {
        let (before, mapped, after) = entry.map_range(remaining.expect("remaining range"));
        result.extend(before);
        result.extend(mapped);

        remaining = after;
    }
    result.extend(remaining);

    result
}

fn parse_input1(input: &str) -> Option<(Vec<Range>, Vec<Table>)> {
    let (seeds, tables) = input.split_once("\n\n")?;
    let seeds = parse_seeds1(seeds)?;
    let tables = tables
        .split_terminator("\n\n")
        .map(parse_table)
        .collect::<Option<Vec<Table>>>()?;
    Some((seeds, tables))
}

fn parse_seeds1(seeds: &str) -> Option<Vec<Range>> {
    let seeds = seeds.strip_prefix("seeds: ")?;
    seeds
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().map(Range::value).ok())
        .collect::<Option<Vec<_>>>()
}

fn parse_input2(input: &str) -> Option<(Vec<Range>, Vec<Table>)> {
    let (seeds, tables) = input.split_once("\n\n")?;
    let seeds = parse_seeds2(seeds)?;
    let tables = tables
        .split_terminator("\n\n")
        .map(parse_table)
        .collect::<Option<Vec<_>>>()?;
    Some((seeds, tables))
}

fn parse_seeds2(seeds: &str) -> Option<Vec<Range>> {
    let seeds = seeds.strip_prefix("seeds: ")?;
    let seeds = seeds
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().ok())
        .collect::<Option<Vec<_>>>()?;
    seeds
        .chunks(2)
        .map(|chunk| match chunk {
            &[start, length] => Some(Range::new(start, start + length)),
            _ => None,
        })
        .collect()
}

fn parse_table(table: &str) -> Option<Table> {
    let (_header, table) = table.split_once('\n')?;
    let mut table = table
        .lines()
        .map(|line| {
            let (destination, line) = line.split_once(' ')?;
            let (source, length) = line.split_once(' ')?;
            let destination = destination.parse::<i64>().ok()?;
            let source = source.parse::<i64>().ok()?;
            let length = length.parse::<i64>().ok()?;
            Some(Entry {
                range: Range::new(source, source + length),
                offset: destination - source,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    table.sort_by_key(Entry::start);
    Some(Table { entries: table })
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    #[inline]
    const fn value(start: i64) -> Self {
        Self {
            start,
            end: start + 1,
        }
    }

    #[inline]
    const fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    #[inline]
    fn offset(self, offset: i64) -> Self {
        Self {
            start: self.start + offset,
            end: self.end + offset,
        }
    }

    #[inline]
    const fn intersects(&self, b: &Range) -> bool {
        self.end > b.start && b.end > self.start
    }

    #[inline]
    const fn preceeds(&self, b: &Range) -> bool {
        self.start < b.start
    }

    #[inline]
    const fn succeeds(&self, b: &Range) -> bool {
        self.end > b.end
    }

    fn intersect(&self, b: &Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        let before = self
            .preceeds(b)
            .then(|| Range::new(self.start, min(self.end, b.start)));
        let overlap = self
            .intersects(b)
            .then(|| Range::new(max(self.start, b.start), min(self.end, b.end)));
        let after = self.succeeds(b).then(|| Range::new(b.end, self.end));

        (before, overlap, after)
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Clone, Copy)]
struct Entry {
    range: Range,
    offset: i64,
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} -> {}", self.range, self.offset)
    }
}

impl Entry {
    #[inline]
    const fn start(&self) -> i64 {
        self.range.start
    }

    fn map_range(&self, v: Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        let (before, overlap, after) = v.intersect(&self.range);
        let mapped = overlap.map(|r| r.offset(self.offset));
        (before, mapped, after)
    }
}

struct Table {
    entries: Vec<Entry>,
}

impl Table {
    fn find_range(&self, v: &Range) -> Option<&[Entry]> {
        let start = self.find_start(v)?;
        let end = self.find_end(v)?;
        Some(&self.entries[start..=end])
    }

    fn find_start(&self, v: &Range) -> Option<usize> {
        match self.entries.binary_search_by_key(&v.start, Entry::start) {
            Ok(i) => Some(i),
            Err(i) if i > 0 && v.intersects(&self.entries[i - 1].range) => Some(i - 1),
            Err(i) if i < self.entries.len() && v.intersects(&self.entries[i].range) => Some(i),
            _ => None,
        }
    }
    fn find_end(&self, v: &Range) -> Option<usize> {
        match self.entries.binary_search_by_key(&v.end, Entry::start) {
            Err(i) if i < self.entries.len() && v.intersects(&self.entries[i].range) => Some(i),
            Ok(i) | Err(i) if i > 0 && v.intersects(&self.entries[i - 1].range) => Some(i - 1),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn entry(start: i64, end: i64, offset: i64) -> Entry {
        Entry {
            range: Range::new(start, end),
            offset,
        }
    }

    #[test]
    fn test_entry_map_range() {
        let e = entry(15, 25, 10);

        assert_eq!(
            (None, Some(Range { start: 25, end: 35 }), None),
            e.map_range(Range { start: 15, end: 25 })
        );

        assert_eq!(
            (
                None,
                Some(Range { start: 30, end: 35 }),
                Some(Range { start: 25, end: 30 })
            ),
            e.map_range(Range { start: 20, end: 30 })
        );

        assert_eq!(
            (
                Some(Range { start: 10, end: 15 }),
                Some(Range { start: 25, end: 30 }),
                None
            ),
            e.map_range(Range { start: 10, end: 20 })
        );

        assert_eq!(
            (
                Some(Range { start: 10, end: 15 }),
                Some(Range { start: 25, end: 35 }),
                Some(Range { start: 25, end: 30 })
            ),
            e.map_range(Range { start: 10, end: 30 })
        );
    }

    #[test]
    fn test_intersects() {
        assert!(!Range::intersects(
            &Range { start: 1, end: 2 },
            &Range { start: 3, end: 4 }
        ));
        assert!(!Range::intersects(
            &Range { start: 1, end: 3 },
            &Range { start: 3, end: 4 }
        ));
        assert!(Range::intersects(
            &Range { start: 1, end: 4 },
            &Range { start: 3, end: 4 }
        ));
        assert!(Range::intersects(
            &Range { start: 1, end: 5 },
            &Range { start: 3, end: 4 }
        ));
        assert!(Range::intersects(
            &Range { start: 2, end: 5 },
            &Range { start: 3, end: 4 }
        ));
        assert!(Range::intersects(
            &Range { start: 3, end: 5 },
            &Range { start: 3, end: 4 }
        ));
        assert!(!Range::intersects(
            &Range { start: 4, end: 5 },
            &Range { start: 3, end: 4 }
        ));
    }

    #[test]
    fn test_table_find_start() {
        let table = Table {
            entries: vec![entry(10, 20, 0), entry(30, 40, 0), entry(50, 98, 0)],
        };
        assert_eq!(None, table.find_start(&Range::new(0, 10)));
        assert_eq!(Some(0), table.find_start(&Range::new(0, 50)));

        assert_eq!(Some(0), table.find_start(&Range::new(0, 11)));
        assert_eq!(Some(0), table.find_start(&Range::new(0, 20)));
        assert_eq!(Some(0), table.find_start(&Range::new(15, 25)));
        assert_eq!(None, table.find_start(&Range::new(20, 25)));
        assert_eq!(None, table.find_start(&Range::new(20, 30)));
        assert_eq!(Some(0), table.find_start(&Range::new(10, 40)));
        assert_eq!(Some(1), table.find_start(&Range::new(30, 40)));
        assert_eq!(None, table.find_start(&Range::new(40, 50)));
        assert_eq!(Some(2), table.find_start(&Range::new(79, 93)));
    }

    #[test]
    fn test_table_find_end() {
        let table = Table {
            entries: vec![entry(10, 20, 0), entry(30, 40, 0)],
        };
        assert_eq!(None, table.find_end(&Range::new(0, 10)));
        assert_eq!(Some(1), table.find_end(&Range::new(0, 50)));

        assert_eq!(Some(0), table.find_end(&Range::new(0, 11)));
        assert_eq!(Some(0), table.find_end(&Range::new(0, 20)));
        assert_eq!(Some(0), table.find_end(&Range::new(15, 25)));
        assert_eq!(None, table.find_end(&Range::new(20, 25)));
        assert_eq!(None, table.find_end(&Range::new(20, 30)));
        assert_eq!(Some(1), table.find_end(&Range::new(20, 31)));
        assert_eq!(Some(1), table.find_end(&Range::new(10, 40)));
        assert_eq!(Some(1), table.find_end(&Range::new(30, 40)));
        assert_eq!(None, table.find_end(&Range::new(40, 50)));

        let table = Table {
            entries: vec![entry(10, 20, 0), entry(20, 30, 0)],
        };
        assert_eq!(Some(0), table.find_end(&Range::new(10, 20)));
    }

    #[test]
    fn part1_example() {
        assert_eq!(35, part1(include_str!("example.txt")));
    }

    #[ignore]
    #[test]
    fn part1_verify() {
        assert_eq!(910845529, part1(INPUT));
    }

    #[test]
    fn part2_example() {
        assert_eq!(46, part2(include_str!("example.txt")));
    }

    #[ignore]
    #[test]
    fn part2_verify() {
        assert_eq!(77435348, part2(INPUT));
    }
}
