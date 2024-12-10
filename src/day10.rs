use itertools::Itertools;
use std::collections::HashSet;

use crate::explore_2d::{neighbors, Exploration, Explore, Gather};

use crate::space2d::Field;
use crate::tree_reduce::TreeReduceCompute;
use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;

use crate::space2d::Coord;
use crate::space2d::TableField;

struct Hiker {}

impl Explore for Hiker {
    type T = usize;

    fn considered_directions(&self) -> Vec<Coord> {
        neighbors::FOUR.into()
    }

    fn filtered_directions(&self, value: &usize, parent_value: &usize) -> bool {
        *value == *parent_value + 1
    }

    fn reached_target(&self, value: &Self::T) -> bool {
        *value == 9
    }
}

struct CountSummit {}

impl<T> Gather<T> for CountSummit {
    type Loot = HashSet<Coord>;

    fn bring_back(&self, coord: &Coord, _value: &T, reached_target: bool) -> Self::Loot {
        if reached_target {
            [*coord].into()
        } else {
            [].into()
        }
    }
    fn gather(&self, it: impl Iterator<Item = Self::Loot>) -> Self::Loot {
        it.map(|collection| collection.into_iter())
            .flatten()
            .collect()
    }
}

type ParsedInput = TableField<usize>;

use eyre::eyre;
use eyre::Result;

#[aoc_generator(day10)]
pub fn parse_day10(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!([# usize | "" / "\n"]);
    let res = parser.parse_top(input)?;

    let res = Ok(TableField::try_from(res).map_err(|_| eyre!("tam,ere"))?);
    res
}

#[aoc(day10, part1, explore_2d)]
fn solve_part1_explore_2d(input: &ParsedInput) -> Result<usize, Report> {
    let bb = input.definition_area();

    let exploration = Exploration::new(input, Hiker {}, CountSummit {});

    let res = (0..bb.ymax)
        .into_iter()
        .cartesian_product((0..bb.xmax).into_iter())
        .map(|(y, x)| Coord(x, y))
        .filter(|c| input.get(&c) == Some(&0))
        .map(|c| exploration.compute(&(c, 0), 1).len())
        .sum();

    Ok(res)
}

struct CountPath {}

impl<T> Gather<T> for CountPath {
    type Loot = usize;

    fn bring_back(&self, _coord: &Coord, _value: &T, reached_target: bool) -> Self::Loot {
        if reached_target {1} else {0}
    }

    fn gather(&self, it: impl Iterator<Item = Self::Loot>) -> Self::Loot {
        it.sum()
    }
}

#[aoc(day10, part2, explore_2d)]
fn solve_part2_explore(input: &ParsedInput) -> Result<usize, Report> {
    let bb = input.definition_area();

    let exploration = Exploration::new(input, Hiker {}, CountPath {});

    let res = (0..bb.ymax)
        .into_iter()
        .cartesian_product((0..bb.xmax).into_iter())
        .map(|(y, x)| Coord(x, y))
        .filter(|c| input.get(&c) == Some(&0))
        .map(|c| exploration.compute(&(c, 0), 1))
        .sum();

    Ok(res)
}
