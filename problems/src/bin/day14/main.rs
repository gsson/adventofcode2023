#![feature(map_try_insert)]

use crate::Rock::{Cubic, Round};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let input = input.trim();
    let width = input.chars().position(|c| c == '\n').expect("first line");
    let height = input.len() / (width + 1) + 1;

    let mut next_free = vec![0; width];
    let mut weights = vec![0; width];
    input.lines().enumerate().for_each(|(y, line)| {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => next_free[x] = y + 1,
                'O' => {
                    weights[x] += height - next_free[x];
                    next_free[x] += 1;
                }
                _ => {}
            }
        }
    });
    weights.into_iter().sum()
}

fn part2(input: &str) -> usize {
    let mut platform = parse_input(input);

    let mut cycle_detector = HashMap::new();

    cycle_detector.insert(platform.clone(), 0);
    let mut i = 1;
    let (cycle_start, cycle_end, mut platform) = loop {
        platform = platform.tilt_cycle();
        if let Err(e) = cycle_detector.try_insert(platform.clone(), i) {
            break (*e.entry.get(), i, platform);
        }
        i += 1;
    };

    let remaining = (1_000_000_000 - cycle_start) % (cycle_end - cycle_start);
    for _ in 0..remaining {
        platform = platform.tilt_cycle();
    }

    platform.load_north()
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
enum Rock {
    Round,
    Cubic,
}

impl Debug for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Round => f.write_char('O'),
            Cubic => f.write_char('#'),
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Platform {
    width: usize,
    height: usize,
    rocks: Vec<Option<Rock>>,
}

impl Debug for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            let yo = y * self.width;
            for x in 0..self.width {
                match self.rocks[x + yo] {
                    None => f.write_char('.')?,
                    Some(r) => r.fmt(f)?,
                }
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

impl Platform {
    fn tilt_cycle(self) -> Self {
        self.tilt_north().tilt_west().tilt_south().tilt_east()
    }
    fn tilt_north(mut self) -> Self {
        let mut next_free = (0..self.width).collect::<Vec<_>>();
        for y in 0..self.height {
            let yo = y * self.width;
            for (x, next_free) in next_free.iter_mut().enumerate() {
                let o = x + yo;
                match self.rocks[o] {
                    Some(Round) => {
                        self.rocks.swap(o, *next_free);
                        *next_free += self.width;
                    }
                    Some(Cubic) => {
                        *next_free = o + self.width;
                    }
                    _ => {}
                }
            }
        }

        self
    }

    fn tilt_east(mut self) -> Self {
        let mut next_free = (0..self.height)
            .map(|y| y * self.width + self.width - 1)
            .collect::<Vec<_>>();
        for x in (0..self.width).rev() {
            for (y, next_free) in next_free.iter_mut().enumerate() {
                let o = x + y * self.width;
                match self.rocks[o] {
                    Some(Round) => {
                        self.rocks.swap(o, *next_free);
                        *next_free -= 1;
                    }
                    Some(Cubic) => {
                        *next_free = o - 1;
                    }
                    _ => {}
                }
            }
        }

        self
    }

    fn tilt_south(mut self) -> Self {
        let mut next_free = (0..self.width)
            .map(|x| self.width * self.height - self.width + x)
            .collect::<Vec<_>>();

        for y in (0..self.height).rev() {
            let yo = y * self.width;
            for (x, next_free) in next_free.iter_mut().enumerate() {
                let o = x + yo;
                match self.rocks[o] {
                    Some(Round) => {
                        self.rocks.swap(o, *next_free);
                        *next_free = next_free.saturating_sub(self.width);
                    }
                    Some(Cubic) => {
                        *next_free = o.saturating_sub(self.width);
                    }
                    _ => {}
                }
            }
        }

        self
    }

    fn tilt_west(mut self) -> Self {
        let mut next_free = (0..self.height).map(|y| y * self.width).collect::<Vec<_>>();
        for x in 0..self.width {
            for (y, next_free) in next_free.iter_mut().enumerate() {
                let o = x + y * self.width;
                match self.rocks[o] {
                    Some(Round) => {
                        self.rocks.swap(o, *next_free);
                        *next_free += 1;
                    }
                    Some(Cubic) => {
                        *next_free = o + 1;
                    }
                    _ => {}
                }
            }
        }

        self
    }

    fn load_north(&self) -> usize {
        self.rocks
            .iter()
            .enumerate()
            .filter(|(_, rock)| matches!(rock, Some(Round)))
            .map(|(i, _)| {
                let y = i / self.width;
                self.height - y
            })
            .sum()
    }
}

fn parse_input(input: &str) -> Platform {
    let input = input.trim();
    let width = input.chars().position(|c| c == '\n').expect("first line");
    let height = input.len() / (width + 1) + 1;
    let rocks = input
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '#' => Some(Cubic),
            'O' => Some(Round),
            _ => None,
        })
        .collect();
    Platform {
        width,
        height,
        rocks,
    }
}

#[test]
fn part1_example() {
    assert_eq!(136, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(105_461, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(64, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(102_829, part2(INPUT));
}
