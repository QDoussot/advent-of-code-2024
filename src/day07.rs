use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;

use crate::tree_reduce::{
    tree_reduce, TreeElement, TreeReduce, TreeReduceDebug,
};

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

//-------------------- SOLUTION WITH LAMBDAS --------------------
fn can_be_satisfied_lambda_version(
    target: usize,
    number_list: &[usize],
    state_gen: &[fn(usize, usize) -> usize],
) -> bool {
    tree_reduce(
        number_list[0],
        1,
        &|depth, node| {
            if let Some(number) = number_list.get(depth) {
                state_gen
                    .iter()
                    .map(|gen| match gen(*node, *number) {
                        result if result <= target => TreeElement::Node(result),
                        _ => TreeElement::Collapsed(false),
                    })
                    .collect()
            } else {
                vec![]
            }
        },
        &|node| *node == target,
        &|mut it| it.any(|verified| verified),
    )
}

#[aoc(day7, part2, lambda_version)]
fn solve_part2_lambda_version(input: &ParsedInput) -> Result<usize, String> {
    let generator = vec![|a, b| a + b, |a, b| a * b, |a: usize, b: usize| {
        a * 10usize.pow(b.ilog10() + 1) + b
    }];
    let calibration_result = input
        .iter()
        .filter(|equation| can_be_satisfied_lambda_version(equation.0, &equation.1, &generator))
        .map(|equation| equation.0)
        .sum::<usize>();
    Ok(calibration_result)
}

//-------------------- SOLUTION WITH TRAIT --------------------
struct EquationSolver<'a> {
    target: usize,
    numbers: &'a [usize],
    generators: &'a [fn(usize, usize) -> usize],
}

impl<'a> EquationSolver<'a> {
    fn new(
        target: usize,
        numbers: &'a [usize],
        generators: &'a [fn(usize, usize) -> usize],
    ) -> Self {
        Self {
            target,
            numbers,
            generators,
        }
    }
}

impl<'a> TreeReduce<usize, bool> for EquationSolver<'a> {
    fn generate_child(&self, depth: usize, node: &usize) -> Vec<TreeElement<usize, bool>> {
        if let Some(number) = self.numbers.get(depth) {
            self.generators
                .iter()
                .map(|gen| match gen(*node, *number) {
                    result if result <= self.target => TreeElement::Node(result),
                    _ => TreeElement::Collapsed(false),
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn collapse(&self, node: &usize) -> bool {
        self.target == *node
    }

    fn reduce(&self, mut it: impl Iterator<Item = bool>) -> bool {
        it.any(|v| v)
    }
}

impl<'a> TreeReduceDebug<usize, bool> for EquationSolver<'a> {
    fn generate_child_debug(
        &self,
        depth: usize,
        node: &usize,
    ) -> Vec<(String, TreeElement<usize, bool>)> {
        let ad_hoc = vec!["+", "*", "||"];
        if let Some(number) = self.numbers.get(depth) {
            self.generators
                .iter()
                .zip(ad_hoc)
                .map(|(gen, deb)| match gen(*node, *number) {
                    result if result <= self.target => {
                        (format!("{}{}", deb, number), TreeElement::Node(result))
                    }
                    _ => (
                        format!(" {}* = false", number),
                        TreeElement::Collapsed(false),
                    ),
                })
                .collect()
        } else {
            vec![]
        }
    }
}

#[aoc(day7, part2, tree_reducer)]
fn solve_part2_tree_reducer(input: &ParsedInput) -> Result<usize, String> {
    let generator = vec![|a, b| a + b, |a, b| a * b, |a: usize, b: usize| {
        a * 10usize.pow(b.ilog10() + 1) + b
    }];
    let calibration_result = input
        .iter()
        .filter(|equation| {
            EquationSolver::new(equation.0, &equation.1, &generator).compute_debug(
                format!("{}", equation.1[0]),
                &equation.1[0],
                1,
            )
        })
        .map(|equation| equation.0)
        .sum::<usize>();
    Ok(calibration_result)
}
