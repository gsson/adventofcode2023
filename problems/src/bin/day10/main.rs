use std::collections::HashMap;
use std::fmt::Debug;
use std::mem::replace;
use std::ops::Add;

const INPUT: &str = include_str!("input.txt");

fn main() {
    eprintln!("{}", part1(INPUT));
    eprintln!("{}", part2(INPUT));
}

struct Input {
    start: Point,
    nodes: HashMap<Point, Node<Point>>,
    exits_at_start: Vec<Point>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point([i32; 2]);

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Direction([i32; 2]);

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, Direction([dx, dy]): Direction) -> Self::Output {
        let Point([x, y]) = self;
        Point([x + dx, y + dy])
    }
}

const fn p(x: i32, y: i32) -> Point {
    Point([x, y])
}

const fn d(x: i32, y: i32) -> Direction {
    Direction([x, y])
}

#[derive(Copy, Clone, Debug)]
enum Node<V> {
    Start,
    Outputs([V; 2]),
}

impl<V> Node<V> {
    fn is_start(&self) -> bool {
        matches!(self, Node::Start)
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Verticality {
    None,
    Vertical,
    Partial(i32),
}

impl Node<Point> {
    fn exits_at(&self, p: &Point) -> bool {
        match self {
            Node::Start => false,
            Node::Outputs([a, b]) => a == p || b == p,
        }
    }

    fn verticality(&self) -> Verticality {
        match self {
            Node::Outputs([Point([x1, y1]), Point([x2, y2])]) if x1 == x2 && y1 != y2 => {
                Verticality::Vertical
            }
            Node::Outputs([Point([x1, y1]), Point([x2, y2])]) if x1 < x2 && y1 != y2 => {
                Verticality::Partial(y2 - y1)
            }
            Node::Outputs([Point([x1, y1]), Point([x2, y2])]) if x2 < x1 && y1 != y2 => {
                Verticality::Partial(y1 - y2)
            }
            _ => Verticality::None,
        }
    }

    fn next(&self, entry: &Point) -> Option<Point> {
        match self {
            Node::Outputs([a, b]) if a == entry => Some(*b),
            Node::Outputs([a, b]) if b == entry => Some(*a),
            _ => None,
        }
    }
}

impl Node<Direction> {
    fn resolve(self, p: Point) -> Node<Point> {
        match self {
            Node::Start => Node::Start,
            Node::Outputs([da, db]) => Node::Outputs([p + da, p + db]),
        }
    }
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let l = find_loop(&input);
    l.len() / 2
}

fn part2(input: &str) -> i32 {
    let input = parse_input(input);
    let path = find_loop(&input);

    let mut nodes = path
        .iter()
        .map(|p| (*p, input.nodes[p]))
        .collect::<HashMap<_, _>>();

    // Replace start node with its "proper" node
    nodes.insert(input.start, Node::Outputs([path[0], path[path.len() - 2]]));
    let ((min_x, max_x), (min_y, max_y)) = bounds(&path);

    let mut inner = 0;
    for y in min_y..=max_y {
        inner += inside_space_in_row(y, min_x, max_x, &nodes);
    }
    inner
}

fn inside_space_in_row(y: i32, min_x: i32, max_x: i32, nodes: &HashMap<Point, Node<Point>>) -> i32 {
    let mut inner = 0;
    let mut verticality = Verticality::None;
    let mut inside = false;
    for x in min_x..=max_x {
        let pt = p(x, y);
        if let Some(node) = nodes.get(&pt) {
            match (node.verticality(), verticality) {
                // vertical lines change "insideness"
                (Verticality::Vertical, _) => {
                    inside = !inside;
                }
                // J or 7 of F--J or L--7 (crosses the current row, so changes "insideness")
                (Verticality::Partial(i), Verticality::Partial(j)) if i == j => {
                    inside = !inside;
                    verticality = Verticality::None
                }
                // 7 or J of F--7 or L--J (u-turns don't change "insideness")
                (Verticality::Partial(_), Verticality::Partial(_)) => {
                    verticality = Verticality::None
                }
                (v, _) if v != Verticality::None => verticality = v,
                _ => {}
            }
        } else if inside {
            inner += 1;
        }
    }
    inner
}

fn find_loop(input: &Input) -> Vec<Point> {
    for node in &input.exits_at_start {
        let path = traverse(input.start, *node, &input.nodes).collect::<Vec<_>>();
        if path.last().is_some_and(|p| *p == input.start) {
            return path;
        }
    }
    unreachable!("no loop found")
}

fn bounds(path: &[Point]) -> ((i32, i32), (i32, i32)) {
    path.iter().fold(
        ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
        |((min_x, max_x), (min_y, max_y)), Point([x, y])| {
            (
                (min_x.min(*x), max_x.max(*x)),
                (min_y.min(*y), max_y.max(*x)),
            )
        },
    )
}

fn traverse(
    start: Point,
    mut here: Point,
    nodes: &HashMap<Point, Node<Point>>,
) -> impl Iterator<Item = Point> + '_ {
    let mut prev = start;
    std::iter::once(here).chain(std::iter::from_fn(move || {
        let next = nodes[&here].next(&prev)?;
        prev = replace(&mut here, next);
        Some(here)
    }))
}

const UP: Direction = d(0, -1);
const RIGHT: Direction = d(1, 0);
const DOWN: Direction = d(0, 1);
const LEFT: Direction = d(-1, 0);

fn parse_line(s: &str) -> impl Iterator<Item = (i32, Node<Direction>)> + '_ {
    s.char_indices().filter(|(_, c)| *c != '.').map(|(x, c)| {
        let d = match c {
            '|' => Node::Outputs([UP, DOWN]),
            '-' => Node::Outputs([LEFT, RIGHT]),
            'L' => Node::Outputs([UP, RIGHT]),
            'J' => Node::Outputs([UP, LEFT]),
            '7' => Node::Outputs([LEFT, DOWN]),
            'F' => Node::Outputs([RIGHT, DOWN]),
            'S' => Node::Start,
            _ => unreachable!(),
        };
        (x as i32, d)
    })
}

fn parse_input(input: &str) -> Input {
    let nodes = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| parse_line(line).map(move |(x, n)| (p(x, y as i32), n)))
        .map(|(p, n)| (p, n.resolve(p)))
        .collect::<HashMap<_, _>>();

    let start = nodes
        .iter()
        .find_map(|(xy, n)| n.is_start().then_some(*xy))
        .expect("start");

    let exits_at_start = nodes
        .iter()
        .filter_map(|(xy, n)| n.exits_at(&start).then_some(*xy))
        .collect::<Vec<_>>();

    Input {
        start,
        nodes,
        exits_at_start,
    }
}

#[test]
fn part1_example1() {
    assert_eq!(4, part1(include_str!("example1a.txt")));
    assert_eq!(4, part1(include_str!("example1b.txt")));
}

#[test]
fn part1_example2() {
    assert_eq!(8, part1(include_str!("example2a.txt")));
    assert_eq!(8, part1(include_str!("example2b.txt")));
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(6890, part1(INPUT));
}

#[test]
fn part2_example1() {
    assert_eq!(1, part2(include_str!("example1a.txt")));
    assert_eq!(1, part2(include_str!("example1b.txt")));
}

#[test]
fn part2_example2() {
    assert_eq!(1, part2(include_str!("example2a.txt")));
    assert_eq!(1, part2(include_str!("example2b.txt")));
}

#[test]
fn part2_example3() {
    assert_eq!(4, part2(include_str!("example3.txt")));
}

#[test]
fn part2_example4() {
    assert_eq!(8, part2(include_str!("example4.txt")));
}

#[test]
fn part2_example5() {
    assert_eq!(10, part2(include_str!("example5.txt")));
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(453, part2(INPUT));
}
