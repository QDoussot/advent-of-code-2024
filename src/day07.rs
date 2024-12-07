use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;

type ParsedInput = Vec<(usize, Vec<usize>)>;

#[aoc_generator(day7)]
fn parse_day7(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!([("%: %", usize, [usize | " "]) | "\n"]);
    parser.parse_top(input)
}

fn count_possible_solution_generical(
    target: usize,
    current_value: usize,
    remaning: &[usize],
    state_gen: &[fn(usize, usize) -> usize],
) -> usize {
    if remaning.len() == 0 {
        if current_value == target {
            return 1;
        } else {
            return 0;
        }
    } else {
        state_gen
            .iter()
            .map(|state_gen| state_gen(current_value, remaning[0]))
            .filter(|generated| generated <= &target)
            .filter(|generated| {
                count_possible_solution_generical(target, *generated, &remaning[1..], state_gen) > 0
            })
            .count()
    }
}

#[aoc(day7, part1)]
fn solve_part1(input: &ParsedInput) -> Result<usize, String> {
    let generator = vec![|a, b| a + b, |a, b| a * b];
    let possible = input
        .iter()
        .filter(|equation| {
            count_possible_solution_generical(
                equation.0,
                equation.1[0],
                &equation.1[1..],
                &generator,
            ) > 0
        })
        .map(|equation| equation.0)
        .sum();

    Ok(possible)
}

#[aoc(day7, part2)]
fn solve_part2(input: &ParsedInput) -> Result<usize, String> {
    let generator = vec![|a, b| a + b, |a, b| a * b, |a: usize, b: usize| {
        a * 10usize.pow(b.ilog10() + 1) + b
    }];
    let possible = input
        .iter()
        .filter(|equation| {
            count_possible_solution_generical(
                equation.0,
                equation.1[0],
                &equation.1[1..],
                &generator,
            ) > 0
        })
        .map(|equation| equation.0)
        .sum();

    Ok(possible)
}
