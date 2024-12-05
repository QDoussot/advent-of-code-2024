use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;
use itertools::Itertools;

type ParsedInput = (Vec<(usize, usize)>, Vec<Vec<usize>>);

#[aoc_generator(day5)]
fn parse_whatever(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!((
        "%\n\n%",
        [("%|%", usize, usize) | "\n"],
        [[usize | ","] | "\n"]
    ));
    parser.parse_top(input)
}

#[aoc(day5, part1)]
fn solve(input: &ParsedInput) -> Result<usize, String> {
    let (constraint, prints) = input;
    let expected_order = constraint.into_iter().cloned().into_group_map();
    let mut sum = 0;
    prints.iter().for_each(|pages| {
        if (0..pages.len()).all(|page_nbr| {
            let mut before = pages.iter().take(page_nbr);
            before.all(|b| !expected_order.get(&pages[page_nbr]).unwrap_or(&vec![]).contains(b))
        }) {
            sum += pages[pages.len() / 2];
        }
    });
    Ok(sum)
}

#[aoc(day5, part2)]
fn solve_incorrect(input: &ParsedInput) -> Result<usize, String> {
    let (constraint, prints) = input;
    let expected_order = constraint.into_iter().cloned().into_group_map();
    let mut sum = 0;
    prints.iter().for_each(|pages| {
        if (0..pages.len()).find(|page_nbr| {
            let mut before = pages.iter().take(*page_nbr);
            before.any(|b| expected_order.get(&pages[*page_nbr]).unwrap_or(&vec![]).contains(b))
        }).is_some() {
            let mut sorted = pages.iter().sorted_by(|lhs,rhs|
                if expected_order.get(lhs).unwrap_or(&vec![]).contains(rhs) {
                    Ordering::Less
                }
                else if expected_order.get(rhs).unwrap_or(&vec![]).contains(lhs) {
                    Ordering::Greater
                }
                else {
                    Ordering::Equal
                });
            sum += sorted.nth(pages.len() / 2).unwrap();
        }
    });
    Ok(sum)
}
