use aoc_runner_derive::{aoc, aoc_generator};
use derive_more::From;
use eyre::Report;

use huparse::parse::ParserMapper;
use huparse::parse::{Parsable, Parse};
use huparse::parser;

use itertools::Itertools;
use crate::relation::*;

#[derive(From)]
pub struct PlantReport(pub Vec<usize>);

impl Parsable for PlantReport {
    fn get_parser() -> impl huparse::parse::Parser<Self> {
        ParserMapper::new(parser!([usize | " "]))
    }
}

trait ReportInterpretation {
    fn levels<'a>(&'a self) -> impl 'a + Iterator<Item = &'a usize>;
}

type Predicate = fn((&usize, &usize)) -> bool;
static INCREASING: Predicate = |(a, b)| a < b;
static DECREASING: Predicate = |(a, b)| a > b;
static NEAR: Predicate = |(a, b)| (1..4).contains(&a.abs_diff(*b));

struct Raw<'a>(&'a PlantReport);

impl<'a> ReportInterpretation for Raw<'a> {
    fn levels<'b>(&'b self) -> impl 'b + Iterator<Item = &'b usize> {
        self.0 .0.iter()
    }
}

trait ReportInterpretationEx: ReportInterpretation {
    fn check_consecutive(&self, predicate: impl Relation<usize>) -> bool;
    fn is_safe(&self) -> bool;
}

impl<T: ReportInterpretation> ReportInterpretationEx for T {
    fn check_consecutive(&self, predicate: impl Relation<usize>) -> bool {
        self.levels().tuple_windows().all(predicate.into_fn())
    }

    fn is_safe(&self) -> bool {
        (self.check_consecutive(INCREASING) || self.check_consecutive(DECREASING))
            && self.check_consecutive(NEAR)
    }
}

#[aoc_generator(day2)]
fn parse_reports(input: &str) -> Result<Vec<PlantReport>, Report> {
    parser!([{ PlantReport } | "\n"]).parse_top(input)
}

#[aoc(day2, part1)]
fn number_of_safe_reports(reports: &Vec<PlantReport>) -> usize {
    reports
        .iter()
        .filter(|report| Raw(report).is_safe())
        .count()
}

#[derive(PartialEq, Debug)]
pub enum Validation {
    Safe,
    Damping(usize),
}

struct Damped<'a>(&'a PlantReport, usize);

impl<'a> ReportInterpretation for Damped<'a> {
    fn levels<'b>(&'b self) -> impl 'b + Iterator<Item = &'b usize> {
        self.0
             .0
            .iter()
            .enumerate()
            .filter_map(|(i, val)| (i != self.1).then_some(val))
    }
}

fn try_validate(damped: Damped, predicate: impl Relation<usize>) -> Option<Validation> {
    damped
        .check_consecutive(predicate)
        .then_some(Validation::Damping(damped.1))
}

fn find_bad_and_damper(report: &PlantReport, predicate: impl Relation<usize>) -> Option<Validation> {
    match report.0.iter().tuple_windows().find_position(not(predicate).into_ref_fn()) {
        None => Some(Validation::Safe),
        Some((pos, _)) => try_validate(Damped(&report, pos), predicate)
            .or_else(|| try_validate(Damped(&report, pos + 1), predicate)),
    }
}

#[aoc(day2, part2)]
fn number_of_dampened_safe_reports(reports: &Vec<PlantReport>) -> usize {
    reports
        .iter()
        .filter(|report| {
            find_bad_and_damper(&report, and(INCREASING, NEAR)).is_some()
                || find_bad_and_damper(&report, and(DECREASING, NEAR)).is_some()
        })
        .count()
}
