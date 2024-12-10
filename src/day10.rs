use std::collections::HashSet;

use crate::space2d::Field;
use crate::tree_reduce::TreeElement;
use crate::tree_reduce::TreeReduceCompute;
use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;

use crate::space2d::Coord;
use crate::space2d::TableField;
use crate::tree_reduce::TreeReduce;

struct HikingPath<'a> {
    field: &'a TableField<usize>,
}

impl<'a> TreeReduce<Coord, HashSet<Coord>> for HikingPath<'a> {
    fn generate_child(
        &self,
        depth: usize,
        node: &Coord,
    ) -> Vec<crate::tree_reduce::TreeElement<Coord, HashSet<Coord>>> {
        //println!("Enter {node:?} {depth}");
        if depth == 10 {
            vec![TreeElement::Collapsed(self.collapse(node))]
        } else {
            vec![Coord(1, 0), Coord(0, 1), Coord(-1, 0), Coord(0, -1)]
                .into_iter()
                .map(|c| c + *node)
                .filter(|c| self.field.definition_area().contains(&c))
                //.inspect(|c| println!("checking {c:?} == {depth} -> {:?}", self.field.get(c)))
                .filter(|c| self.field.get(c) == Some(&(depth)))
                //.inspect(|c| println!("continue {c:?}"))
                .map(TreeElement::Node)
                .collect()
        }
    }

    fn collapse(&self, node: &Coord) -> HashSet<Coord> {
        if self.field.get(node) == Some(&9) {
            vec![*node].into_iter().collect()
        } else {
            HashSet::new()
        }
    }

    fn reduce(&self, it: impl Iterator<Item = HashSet<Coord>>) -> HashSet<Coord> {
        it.flatten().collect()
    }
}

type ParsedInput = TableField<usize>;

use eyre::eyre;
use eyre::Result;

#[aoc_generator(day10)]
fn parse_day10(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!([# usize | "" / "\n"]);
    let res = parser.parse_top(input)?;

    let res = Ok(
        TableField::try_from(res).map_err(|_| eyre!("tam,ere"))?,
    );
    res
}

use itertools::Itertools;
#[aoc(day10, part1)]
fn solve_part1(input: &ParsedInput) -> Result<usize, Report> {
    let bb = input.definition_area();
    let hk = HikingPath{field:input};

    let res = (0..bb.ymax)
        .into_iter()
        .cartesian_product((0..bb.xmax).into_iter())
        .map(|(y, x)| Coord(x, y))
        .filter(|c| hk.field.get(&c) == Some(&0))
        //.inspect(|c| println!("start {c:?}"))
        .map(|c| hk.compute(&c, 1).len())
        .sum();

    Ok(res)
}


struct HikingPathNumber<'a> {
    field: &'a TableField<usize>,
}

impl<'a> TreeReduce<Coord, usize> for HikingPathNumber<'a> {
    fn generate_child(
        &self,
        depth: usize,
        node: &Coord,
    ) -> Vec<crate::tree_reduce::TreeElement<Coord, usize>> {
        //println!("Enter {node:?} {depth}");
        if depth == 10 {
            vec![TreeElement::Collapsed(self.collapse(node))]
        } else {
            vec![Coord(1, 0), Coord(0, 1), Coord(-1, 0), Coord(0, -1)]
                .into_iter()
                .map(|c| c + *node)
                .filter(|c| self.field.definition_area().contains(&c))
                //.inspect(|c| println!("checking {c:?} == {depth} -> {:?}", self.field.get(c)))
                .filter(|c| self.field.get(c) == Some(&(depth)))
                //.inspect(|c| println!("continue {c:?}"))
                .map(TreeElement::Node)
                .collect()
        }
    }

    fn collapse(&self, node: &Coord) -> usize {
        if self.field.get(node) == Some(&9) {
            1
        } else {
            0
        }
    }

    fn reduce(&self, it: impl Iterator<Item = usize>) -> usize {
        it.sum()
    }
}


#[aoc(day10, part2)]
fn solve_part2(input: &ParsedInput) -> Result<usize, Report> {
    let bb = input.definition_area();
    let hk = HikingPathNumber{field:input};

    let res = (0..bb.ymax)
        .into_iter()
        .cartesian_product((0..bb.xmax).into_iter())
        .map(|(y, x)| Coord(x, y))
        .filter(|c| hk.field.get(&c) == Some(&0))
        //.inspect(|c| println!("start {c:?}"))
        .map(|c| hk.compute(&c, 1))
        .sum();

    Ok(res)
}
