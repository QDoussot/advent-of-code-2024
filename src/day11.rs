use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;

type ParsedInput = Vec<usize>;

#[aoc_generator(day11)]
fn parse_day11(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!([usize | " "]);
    parser.parse_top(input)
}

//number of stone childs after n blinks
fn stones_after_blinks(
    number: usize,
    blinks: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> Option<usize> {
    let res = if blinks == 0 {
        1
    } else if let Some(already_computed) = cache.get(&(number, blinks)) {
        *already_computed
    } else {
        let new_computation = if number == 0 {
            stones_after_blinks(1, blinks - 1, cache)?
        } else if (number.checked_ilog10()? + 1) % 2 == 0 {
            let len = number.checked_ilog10()? + 1;
            let cut = 10usize.checked_pow(len / 2)?;
            stones_after_blinks(number / cut, blinks - 1, cache)?
                + stones_after_blinks(number % cut, blinks - 1, cache)?
        } else {
            stones_after_blinks(number.checked_mul(2024)?, blinks - 1, cache)?
        };
        cache.insert((number, blinks), new_computation);
        new_computation
    };
    Some(res)
}

#[aoc(day11, part1)]
fn solve_part1(input: &ParsedInput) -> Result<usize, String> {
    let mut memory = HashMap::new();
    input
        .iter()
        .map(|v| stones_after_blinks(*v, 25, &mut memory))
        .try_fold(0usize, |x, y| x.checked_add(y?))
        .ok_or("Arithmetic overflow happened".into())
}

#[aoc(day11, part2)]
fn solve_part2(input: &ParsedInput) -> Result<usize, String> {
    let mut memory = HashMap::new();
    input
        .iter()
        .map(|v| stones_after_blinks(*v, 75, &mut memory))
        .try_fold(0usize, |x, y| x.checked_add(y?))
        .ok_or("Arithmetic overflow happened".into())
}
