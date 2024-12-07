use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;

type ParsedInput = (Vec<(usize, Vec<usize>)>);

#[aoc_generator(day7)]
fn parse_day7(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!([("%: %", usize, [usize | " "]) | "\n"]);
    parser.parse_top(input)
}

fn count_posible_solution(target: usize, current_value: usize, remaning: &[usize]) -> usize {
    if remaning.len() == 0 {
        if current_value == target {
            return 1;
        } else {
            return 0;
        }
    } else {
        let add_possible = if current_value + remaning[0] <= target {
            let add = current_value + remaning[0];
            println!("target {target}, curr: {current_value} add: {add}");
            count_posible_solution(target, add, &remaning[1..])
        } else {
            0
        };

        let mul_possible = if current_value * remaning[0] <= target {
            let mul = current_value * remaning[0];
            println!("target {target}, curr: {current_value} mul: {mul}");
            count_posible_solution(target, mul, &remaning[1..])
        } else {
            0
        };
        add_possible + mul_possible
    }
}

#[aoc(day7, part1)]
fn solve_part1(input: &ParsedInput) -> Result<usize, String> {
    let possible = input
        .iter()
        .filter(|equation| count_posible_solution(equation.0, equation.1[0], &equation.1[1..]) > 0)
        .map(|equation| equation.0)
        .sum();

    Ok((possible))
}

fn count_posible_solution_bis(
    target: usize,
    op: String,
    current_value: usize,
    remaning: &[usize],
) -> usize {
    if remaning.len() == 0 {
        if current_value == target {
            //println!("{target}: {op}");
            return 1;
        } else {
            return 0;
        }
    } else {
        let add_possible = if current_value + remaning[0] <= target {
            let add = current_value + remaning[0];
            count_posible_solution_bis(
                target,
                format!("{op} + {}", remaning[0]),
                add,
                &remaning[1..],
            )
        } else {
            0
        };

        let mul_possible = if current_value * remaning[0] <= target {
            let mul = current_value * remaning[0];
            count_posible_solution_bis(
                target,
                format!("{op} * {}", remaning[0]),
                mul,
                &remaning[1..],
            )
        } else {
            0
        };

        let find_power = 10usize.checked_pow(remaning[0].ilog10() + 1);
        let concat_possible = if let Some(find_power) = find_power {
            let concat = find_power * current_value + remaning[0];
            if concat <= target {
                count_posible_solution_bis(
                    target,
                    format!("{op} || {}", remaning[0]),
                    concat,
                    &remaning[1..],
                )
            } else {
                0
            }
        } else {
            0
        };

        add_possible + mul_possible + concat_possible
    }
}

#[aoc(day7, part2)]
fn solve_part2(input: &ParsedInput) -> Result<usize, String> {
    let possible = input
        .iter()
        .filter(|equation| {
            count_posible_solution_bis(
                equation.0,
                format!("{}", equation.1[0]),
                equation.1[0],
                &equation.1[1..],
            ) > 0
        })
        .map(|equation| equation.0)
        .sum();

    Ok((possible))
}
