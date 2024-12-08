use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use itertools::{Either, Itertools};

use crate::space2d::{moved, Coord, Direction};

type ParsedInput = (HashSet<Coord>, (isize, isize), Option<(Coord, Direction)>);

// type Direction = (isize, isize);

#[derive(Copy, Clone)]
enum Item {
    Obstacle,
    Guard(Direction),
}

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

#[derive(Debug, Clone)]
struct GuardPatrol {
    // Set of positions visited with each direction of guard while visiting each of them
    visited: HashSet<(Coord, Direction)>,
    is_looping: bool,
}

impl GuardPatrol {
    fn into_visited_position(self) -> impl Iterator<Item = Coord> {
        self.visited
            .into_iter()
            .unique_by(|(coord, _)| *coord)
            .map(|(coord, _)| coord)
    }
}

trait Field {
    fn defined_on(&self, coord: &Coord) -> bool;
}

impl Field for Coord {
    fn defined_on(&self, coord: &Coord) -> bool {
        (0..self.0).contains(&coord.0) && (1..self.1).contains(&coord.1)
    }
}

fn move_until_loop<F: Field, S: PartialEq + Eq + std::hash::Hash>(
    field: F,
    init: (Coord, S),
    move_fct: impl Fn(&(Coord, S)) -> (Coord, S),
) -> HashSet<(Coord, S)> {
    let mut visited = HashSet::<(Coord, S)>::new();
    let mut looping = false;
    let mut state = init;
    while !looping && field.defined_on(&state.0) {
        let new_state = move_fct(&state);
        looping = !visited.insert(state);
        state = new_state;
    }
    visited
}

fn run_guard(
    obstacles: &HashSet<Coord>,
    size: &Coord,
    (mut guard_pos, mut guard_dir): (Coord, Direction),
) -> GuardPatrol {
    move_until_loop(
        *size,
        (guard_pos, guard_dir),
        |(mut coord, mut delta): &(Coord, Direction)| {
            let toward = moved(&coord, &delta);
            if obstacles.contains(&toward) {
                delta = delta.rotate();
            } else {
                coord = toward;
            }
            (coord, delta)
        },
    );

    let mut visited = HashSet::<(Coord, Direction)>::new();
    let mut is_looping = false;
    while (0isize..(size.0) as isize).contains(&guard_pos.0)
        && (0isize..size.1 as isize).contains(&guard_pos.1)
        && !is_looping
    {
        is_looping = !visited.insert((guard_pos, guard_dir));
        let new_guard_pos = moved(&guard_pos, &guard_dir);
        if obstacles.contains(&new_guard_pos) {
            guard_dir = guard_dir.rotate()
        } else {
            guard_pos = new_guard_pos;
        }
    }

    GuardPatrol {
        visited,
        is_looping,
    }
}

#[aoc(day6, part1)]
fn solve_part1(map: &ParsedInput) -> Result<usize, String> {
    if let (map, size, Some(guard)) = map.clone() {
        Ok(run_guard(&map, &size, guard)
            .into_visited_position()
            .count())
    } else {
        Ok(0)
    }
}

#[aoc(day6, part2)]
fn solve_part2(map: &ParsedInput) -> Result<usize, String> {
    if let (mut obstacles, size, Some(guard)) = map.clone() {
        let visited = run_guard(&obstacles, &size, guard)
            .into_visited_position()
            .skip(1); // We can't put the obstacle at the guard first position
        Ok(visited
            .filter(|coord| {
                if !obstacles.contains(coord) {
                    obstacles.insert(*coord);
                    let GuardPatrol { is_looping, .. } = run_guard(&obstacles, &size, guard);
                    obstacles.remove(coord);
                    is_looping
                } else {
                    run_guard(&obstacles, &size, guard).is_looping
                }
            })
            .count())
    } else {
        Ok(0)
    }
}
