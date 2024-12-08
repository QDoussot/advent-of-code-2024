use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use derive_more::derive::{Add, Sub};
use eyre::Report;

use huparse::{parse::Parse, parser};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Add, Sub, Hash, PartialEq, Eq)]
struct Coords(isize, isize);
type ParsedInput = (Coords, HashMap<char, Vec<Coords>>);

#[aoc_generator(day8)]
fn parse_day8(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!([# char | "" / "\n"]);
    let (w, h, lines) = parser.parse_top(input)?.into_tuple();

    let map = lines.into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter().enumerate().filter_map(move |(x, c)| {
                match c {
                    '.' => None,
                    c => Some(c),
                }
                .map(|item| (item, Coords(x as isize, y as isize)))
            })
        })
        .flatten()
        .into_group_map();
    Ok((Coords(w as isize, h as isize), map))
}


#[aoc(day8, part1)]
fn solve_part1(input: &ParsedInput) -> Result<usize, String> {
    let antinodes: HashSet<_> = input
        .1
        .iter()
        .map(|(antenna, coords)| {
            coords
                .clone()
                .into_iter()
                //.map(|(x, y)| Coords(*x, *y))
                .tuple_combinations()
                .map(|couple: (Coords, Coords)| {
                    vec![
                        couple.0 + (couple.0 - couple.1),
                        couple.1 + (couple.1 - couple.0),
                    ]
                    .into_iter()
                    .filter(|coord| {
                        (0isize..input.0 .0).contains(&coord.0)
                            && (0isize..input.0 .1).contains(&coord.1)
                    })
                    .inspect(|coord| {
                        println!(
                            "{}: {:?} - {:?} node at {:?}",
                            *antenna, couple.0, couple.1, *coord
                        )
                    })
                    .collect::<Vec<_>>()
                })
                .flatten()
        })
        .flatten()
        .collect();
    Ok(antinodes.len())
}

#[aoc(day8, part2)]
fn solve_part2(input: &ParsedInput) -> Result<usize, String> {
    let antinodes: HashSet<_> = input
        .1
        .iter()
        .map(|(antenna, coords)| {
            coords
                .clone()
                .into_iter()
                //.map(|(x, y)| Coords(*x, *y))
                .tuple_combinations()
                .map(|couple: (Coords, Coords)| {
                    let one_dir = std::iter::successors(Some(couple.0), |coord| {
                        Some(*coord + (couple.0 - couple.1))
                    })
                    .take_while(|coord| {
                        (0isize..input.0 .0).contains(&coord.0)
                            && (0isize..input.0 .1).contains(&coord.1)
                    });
                    let snd_dir = std::iter::successors(Some(couple.1), |coord| {
                        Some(*coord + (couple.1 - couple.0))
                    })
                    .take_while(|coord| {
                        (0isize..input.0 .0).contains(&coord.0)
                            && (0isize..input.0 .1).contains(&coord.1)
                    });
                    one_dir.chain(snd_dir)
                    .inspect(|coord| {
                        println!(
                            "{}: {:?} - {:?} node at {:?}",
                            *antenna, couple.0, couple.1, *coord
                        )
                    })
                    .collect::<Vec<_>>()
                })
                .flatten()
        })
        .flatten()
        .collect();
    Ok(antinodes.len())
}
