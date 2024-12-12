use std::collections::HashMap;
use std::iter::zip;

use eyre::Report;
use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

use huparse::parse::Parse;
use huparse::parser;

type SideBySide = (Vec<usize>, Vec<usize>);

#[aoc_generator(day1)]
fn parse_lists(input: &str) -> Result<SideBySide, Report> {
    let parser = parser!([("%   %", usize, usize) | "\n"]);
    let lists = parser
        .parse_top(input)?
        .into_iter()
        .fold((vec![], vec![]), |mut acc, x| {
            acc.0.push(x.0);
            acc.1.push(x.1);
            acc
        });
    Ok(lists)
}

#[aoc(day1, part1)]
fn lists_distance(lists: &SideBySide) -> Result<String, String> {
    let diff_sum = zip(lists.0.iter().sorted(), lists.1.iter().sorted())
        .map(|(lhs, rhs)| lhs.abs_diff(*rhs))
        .try_fold(0usize, |acc, x| acc.checked_add(x))
        .ok_or("Sum overflowned usize capability")?;

    Ok(diff_sum.to_string())
}

#[aoc(day1, part2)]
fn lists_similarity(lists: &SideBySide) -> Result<String, String> {
    let mut counts = lists
        .0
        .iter()
        .fold(HashMap::<usize, (usize, usize)>::new(), |mut acc, x| {
            if let Some(count) = acc.get_mut(x) {
                count.0 += 1;
            } else {
                acc.insert(*x, (1, 0));
            }
            acc
        });

    lists.1.iter().for_each(|x| {
        if let Some(count) = counts.get_mut(x) {
            count.1 += 1;
        }
        // It is not necessary to count numbers only appearing in the right list
    });

    let simi = counts.iter().try_fold(0usize, |acc, x| {
        if x.1 .0 != 0 && x.1 .1 != 0 {
            println!("{} -> {} * {}", x.0, x.1 .0, x.1 .1);
        }
        x.1 .0
            .checked_mul(x.1 .1)
            .and_then(|counts_mul| counts_mul.checked_mul(*x.0))
            .and_then(|points| acc.checked_add(points))
            .ok_or("Sum overflowned usize capability")
    })?;

    Ok(simi.to_string())
}
