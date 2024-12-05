use std::cmp::Ordering;
use std::collections::HashMap;

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

fn verify_order_constraint(pages: &[usize], order_constraint: &HashMap<usize, Vec<usize>>) -> bool {
    pages.iter().enumerate().all(|(page_nbr, page)| {
        let before = &pages[..page_nbr];
        // Check that every page found before isn't expected "to be found after if found"
        before.iter().all(|page_found_before| {
            !order_constraint
                .get(page)
                .unwrap_or(&vec![])
                .contains(page_found_before)
        })
    })
}

#[aoc(day5, part1)]
fn solve(input: &ParsedInput) -> Result<usize, String> {
    let (constraint, prints) = input;
    let expected_order = constraint.into_iter().cloned().into_group_map();
    let mut sum = 0;
    prints.iter().for_each(|pages| {
        if verify_order_constraint(pages, &expected_order) {
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
        if !verify_order_constraint(pages, &expected_order) {
            let mut sorted = pages.iter().sorted_by(|lhs, rhs| {
                if expected_order.get(lhs).unwrap_or(&vec![]).contains(rhs) {
                    Ordering::Less
                } else if expected_order.get(rhs).unwrap_or(&vec![]).contains(lhs) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            sum += sorted.nth(pages.len() / 2).unwrap();
        }
    });
    Ok(sum)
}
