use aoc_runner_derive::{aoc, aoc_generator};
use derive_more::From;
use eyre::Report;

use huparse::parse::ParserMapper;
use huparse::parse::{Parsable, Parse};
use huparse::parser;

#[derive(From)]
pub struct PlantReport(pub Vec<usize>);

pub fn are_safe_adjacent(l0: usize, l1: usize) -> bool {
    (1..4).contains(&l0.abs_diff(l1))
}

impl Parsable for PlantReport {
    fn get_parser() -> impl huparse::parse::Parser<Self> {
        ParserMapper::new(parser!([usize | " "]))
    }
}

impl PlantReport {
    fn is_safe(&self) -> bool {
        (self.0.windows(2).all(|w| w[0] < w[1]) || self.0.windows(2).all(|w| w[0] > w[1]))
            && self.0.windows(2).all(|w| are_safe_adjacent(w[0], w[1]))
    }
}

#[derive(PartialEq, Debug)]
pub enum Validation {
    Safe,
    Damping(usize),
}

#[aoc_generator(day2)]
fn parse_reports(input: &str) -> Result<Vec<PlantReport>, Report> {
    parser!([{ PlantReport } | "\n"]).parse_top(input)
}

#[aoc(day2, part1)]
fn number_of_safe_reports(reports: &Vec<PlantReport>) -> usize {
    reports
        .iter()
        .filter(|report| report.is_safe())
        .count()
}


mod damper;
