use std::cmp::Ordering;
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;
use itertools::Itertools;

type ParsedInput = (OrderConstraint, Vec<Vec<usize>>);

struct OrderConstraint(HashMap<usize, Vec<usize>>);

impl OrderConstraint {
    fn verified_in(&self, print: &[usize]) -> bool {
        verify_order_constraint(print, &self.0)
    }

    fn order(&self, page_a: &usize, page_b: &usize) -> Ordering {
        if self.0.get(page_a).unwrap_or(&vec![]).contains(page_b) {
            Ordering::Less
        } else if self.0.get(page_b).unwrap_or(&vec![]).contains(page_a) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
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

#[aoc_generator(day5)]
fn parse_whatever(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!((
        "%\n\n%",
        [("%|%", usize, usize) | "\n"],
        [[usize | ","] | "\n"]
    ));
    parser.parse_top(input).map(|(constraints, prints)| {
        (
            OrderConstraint(constraints.into_iter().into_group_map()),
            prints,
        )
    })
}

#[aoc(day5, part1)]
fn solve(input: &ParsedInput) -> Result<usize, String> {
    let (constraint, prints) = input;
    let sum = prints
        .iter()
        .filter_map(|pages| {
            constraint
                .verified_in(pages)
                .then_some(pages.get(pages.len() / 2))
                .flatten()
        })
        .sum();
    Ok(sum)
}

#[aoc(day5, part2)]
fn solve_incorrect(input: &ParsedInput) -> Result<usize, String> {
    let (constraint, prints) = input;
    let sum = prints
        .iter()
        .filter_map(|pages| {
            (!constraint.verified_in(pages))
                .then_some(
                    pages
                        .iter()
                        .sorted_by(|lhs, rhs| constraint.order(lhs, rhs))
                        .nth(pages.len() / 2),
                )
                .flatten()
        })
        .sum();
    Ok(sum)
}
