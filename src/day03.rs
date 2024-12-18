use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use thiserror::Error;

#[aoc_generator(day3)]
fn read_memory(input: &str) -> String {
    input.to_string()
}

#[derive(Error, Debug, PartialEq, Eq)]
enum MulSumError {
    #[error("{0}")]
    ParsingInteger(#[from] ParseIntError),
    #[error("Addition overflow when computing {0} + {1}")]
    OverflownAddition(usize, usize),
    #[error("Multiplication overflow when computing {0} * {1}")]
    OverflownMul(usize, usize),
}

fn add_mul(acc: usize, lhs: &str, rhs: &str) -> Result<usize, MulSumError> {
    let lhs = lhs.parse::<usize>()?;
    let rhs = rhs.parse::<usize>()?;
    let mul = lhs
        .checked_mul(rhs)
        .ok_or(MulSumError::OverflownMul(lhs, rhs))?;
    acc.checked_add(mul)
        .ok_or(MulSumError::OverflownAddition(acc, mul))
}

#[aoc(day3, part1)]
fn seek_and_sum_mul(input: &str) -> Result<usize, MulSumError> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let sum = re
        .captures_iter(input)
        .try_fold(0usize, |acc, captured_mul| {
            let (_, [lhs, rhs]) = captured_mul.extract();
            add_mul(acc, lhs, rhs)
        });
    sum
}

#[aoc(day3, part2)]
fn seek_and_sum_enabled_mul(input: &str) -> Result<usize, MulSumError> {
    let re = Regex::new(r"(mul|don't|do)\((?:([0-9]{1,3}),([0-9]{1,3}))?\)").unwrap();
    let execution =
        re.captures_iter(input)
            .try_fold((0usize, true), |(sum, mul_enabled), captures| {
                match (
                    mul_enabled,
                    captures.get(1).map(|m| m.as_str()),
                    captures.get(2),
                    captures.get(3),
                ) {
                    (_, Some("do"), None, None) => Ok((sum, true)),
                    (_, Some("don't"), None, None) => Ok((sum, false)),
                    (true, Some("mul"), Some(lhs), Some(rhs)) => {
                        add_mul(sum, lhs.as_str(), rhs.as_str()).map(|sum| (sum, mul_enabled))
                    }
                    _ => Ok((sum, mul_enabled)),
                }
            })?;
    Ok(execution.0)
}

#[cfg(test)]
mod tests {
    use crate::day03::{seek_and_sum_enabled_mul, seek_and_sum_mul};

    #[test]
    fn it_discard_number_with_more_than_3_digits() {
        let haystack = "blabla()mul(1234,1)coucou";
        assert_eq!(seek_and_sum_mul(haystack), Ok(0));
    }

    #[test]
    fn it_discard_dont_with_digits() {
        let haystack = "blabla()don't(12) & mul(124,1)coucou";
        assert_eq!(seek_and_sum_enabled_mul(haystack), Ok(124));
    }

    #[test]
    fn it_disables_mul_when_dont_encountered() {
        let haystack = "blabla()don't() & mul(124,1)coucou";
        assert_eq!(seek_and_sum_enabled_mul(haystack), Ok(0));
    }
}
