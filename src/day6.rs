use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;
use itertools::{Either, Itertools};

type Coord = (isize, isize);

type ParsedInput = (HashSet<Coord>, (isize, isize), Option<(Coord, Direction)>);

// type Direction = (isize, isize);

#[derive(Copy, Clone)]
enum Item {
    Obstacle,
    Guard(Direction),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Ri,
    Do,
    Le,
}

fn moved(pos: &Coord, dir: &Direction) -> Coord {
    match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Ri => (pos.0 + 1, pos.1),
        Direction::Do => (pos.0, pos.1 + 1),
        Direction::Le => (pos.0 - 1, pos.1),
    }
}

impl Direction {
    fn rotate(self) -> Direction {
        match self {
            Direction::Up => Direction::Ri,
            Direction::Ri => Direction::Do,
            Direction::Do => Direction::Le,
            Direction::Le => Direction::Up,
        }
    }
}

static CLOCKWISE: [Direction; 4] = [Direction::Up, Direction::Ri, Direction::Do, Direction::Le];

#[aoc_generator(day6)]
fn parse_day6(input: &str) -> Result<ParsedInput, Report> {
    let lines = input.lines().collect::<Vec<_>>();
    let w = lines[0].len();
    let h = lines.len();
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                match c {
                    '#' => Some(Item::Obstacle),
                    '^' => Some(Item::Guard(Direction::Up)),
                    '>' => Some(Item::Guard(Direction::Ri)),
                    'v' => Some(Item::Guard(Direction::Do)),
                    '<' => Some(Item::Guard(Direction::Le)),
                    _ => None,
                }
                .map(|item| ((x as isize, y as isize), item))
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let (ground, mut guard): (Vec<_>, Vec<_>) =
        map.into_iter().partition_map(|(coord, item)| match item {
            Item::Obstacle => Either::Left(coord),
            Item::Guard(d) => Either::Right((coord, d)),
        });

    Ok((
        ground.into_iter().collect(),
        (w as isize, h as isize),
        guard.pop(),
    ))
}

#[aoc(day6, part1)]
fn solve_part1(map: &ParsedInput) -> Result<usize, String> {
    let mut visited = HashSet::<Coord>::new();
    if let (map, size, Some((mut guard_pos, mut guard_dir))) = map {
        while (0isize..(size.0) as isize).contains(&guard_pos.0)
            && (0isize..size.1 as isize).contains(&guard_pos.1)
        {
            visited.insert(guard_pos);
            let new_guard_pos = moved(&guard_pos, &guard_dir);
            if map.contains(&new_guard_pos) {
                guard_dir = guard_dir.rotate()
            } else {
                guard_pos = new_guard_pos;
            }
        }
    }
    Ok(visited.len())
}

fn looping(
    obstacles: &HashSet<Coord>,
    size: &Coord,
    (mut guard_pos, mut guard_dir): &(Coord, Direction),
) -> bool {
    let mut visited = HashSet::<(Coord, Direction)>::new();
    let mut looped = false;
    while (0isize..(size.0) as isize).contains(&guard_pos.0)
        && (0isize..size.1 as isize).contains(&guard_pos.1)
        && !looped
    {
        looped = !visited.insert((guard_pos, guard_dir));
        let new_guard_pos = moved(&guard_pos, &guard_dir);
        if obstacles.contains(&new_guard_pos) {
            guard_dir = guard_dir.rotate()
        } else {
            guard_pos = new_guard_pos;
        }
    }

    looped
}

#[aoc(day6, part2)]
fn solve_part2(map: &ParsedInput) -> Result<usize, String> {
    if let (mut obstacles, size, Some(guard)) = map.clone() {
            Ok((0..size.0 as isize)
            .cartesian_product(0..size.1 as isize)
            .filter(|coord| {
                if !obstacles.contains(coord) {
                    obstacles.insert(*coord);
                    let does_loop = looping(&obstacles, &size, &guard);
                    obstacles.remove(coord);
                    does_loop
                } else {
                    looping(&obstacles, &size, &guard)
                }
            })
            .count())
    } else {
        Ok(0)
    }
}

