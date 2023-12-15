use std::array::from_fn;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

#[inline]
fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |h, c| ((h + *c as usize) * 17) & 0xff)
}

fn part1(input: &str) -> usize {
    input.trim().split(',').map(hash).sum()
}

fn parse_op(v: &str) -> (&str, Option<usize>) {
    let p = v.find(|c: char| !c.is_ascii_alphabetic()).unwrap();
    let label = &v[..p];
    match &v[p..p + 1] {
        "=" => (label, Some(v[p + 1..].parse::<usize>().unwrap())),
        "-" => (label, None),
        _ => unreachable!("invalid operation"),
    }
}

struct LensBox<'a>(Vec<(&'a str, usize)>);
impl<'a> LensBox<'a> {
    fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    fn find_lens(&self, label: &str) -> Option<usize> {
        self.0.iter().position(|(l, _)| *l == label)
    }

    fn insert(&mut self, label: &'a str, focal_length: usize) {
        if let Some(i) = self.find_lens(label) {
            self.0[i] = (label, focal_length);
        } else {
            self.0.push((label, focal_length));
        }
    }

    fn remove(&mut self, label: &'a str) {
        if let Some(i) = self.find_lens(label) {
            self.0.remove(i);
        }
    }
}

struct LensMap<'a> {
    boxes: [LensBox<'a>; 256],
}

impl<'a> LensMap<'a> {
    fn new() -> Self {
        Self {
            boxes: from_fn(|_| LensBox::new()),
        }
    }

    #[inline]
    fn insert(&mut self, label: &'a str, focal_length: usize) {
        self.boxes[hash(label)].insert(label, focal_length)
    }

    #[inline]
    fn remove(&mut self, label: &'a str) {
        self.boxes[hash(label)].remove(label)
    }
}

impl<'a> IntoIterator for LensMap<'a> {
    type Item = (usize, usize, usize);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.boxes
                .into_iter()
                .enumerate()
                .flat_map(|(box_index, lens_box)| {
                    lens_box.0.into_iter().enumerate().map(
                        move |(lens_index, (_, focal_length))| {
                            (box_index + 1, lens_index + 1, focal_length)
                        },
                    )
                }),
        )
    }
}

fn part2(input: &str) -> usize {
    let mut lenses = LensMap::new();
    for (label, op) in input.trim().split(',').map(parse_op) {
        match op {
            None => lenses.remove(label),
            Some(focal_length) => lenses.insert(label, focal_length),
        }
    }

    lenses
        .into_iter()
        .map(|(box_index, lens_index, focal_length)| box_index * lens_index * focal_length)
        .sum()
}

#[test]
fn part1_example() {
    assert_eq!(1320, part1(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(513_158, part1(INPUT));
}

#[test]
fn part2_example() {
    assert_eq!(145, part2(include_str!("example.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(200_277, part2(INPUT));
}
