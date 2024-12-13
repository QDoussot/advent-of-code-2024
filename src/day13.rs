use derive_more::Mul;

use aoc_runner_derive::{aoc, aoc_generator};
use derive_more::Add;
use derive_more::Sub;
use eyre::Report;
use itertools::Itertools;

use huparse::parse::Parse;
use huparse::parser;

#[derive(Debug, Copy, Clone, Add, Sub, Hash, PartialEq, Eq, Mul)]
struct Position(i64, i64);

#[derive(Debug, Copy, Clone, Add, Sub, Hash, PartialEq, Eq, Mul)]
struct Play {
    a: Position,
    b: Position,
    p: Position,
}

type ParsedInput = Vec<Play>;

#[aoc_generator(day13)]
fn parse_day13(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!([(
        "\
Button A: X+%, Y+%
Button B: X%, Y+%
Prize: X=%, Y=%",
        i64,
        i64,
        i64,
        i64,
        i64,
        i64
    ) | "\n\n"]);

    let res = parser.parse_top(&input)?;
    let res = res
        .into_iter()
        .map(|(ax, ay, bx, by, px, py)| Play {
            a: Position(ax, ay),
            b: Position(bx, by),
            p: Position(px, py),
        })
        .collect();
    Ok(res)
}

fn find_token_nbr(play: &Play) -> Option<i64> {
    // Literally follow the proposed algorithm
    // Fast to write, quite unefficient
    (0..100)
        .cartesian_product(0..100)
        .filter(|(an, bn)| play.a * an + play.b * bn == play.p)
        .map(|(an, bn)| an * 3 + bn)
        .min()
}

#[aoc(day13, part1)]
fn solve_part1(input: &ParsedInput) -> Result<i64, String> {
    let res: i64 = input
        .iter()
        .map(|p| find_token_nbr(p))
        .filter_map(|t| t)
        .sum();
    Ok(res)
}

// Solve v in au + bv = p like a regular two unknows - two equations system
// and then check that the result is an integer
// with
//   a,b being integer 
//   b,v,p being 2 dimensions integer 'vectors'
fn compute_v(play: &Play) -> Option<i64> {
    let Play { a, b, p } = play;
    let (v, rem) = {
        let num = p.1 * a.0 - p.0 * a.1;
        let denum = b.1 * a.0 - b.0 * a.1;
        (num / denum, num % denum)
    };
    (rem == 0).then_some(v)
}

// Compute u once v is found
fn compute_u(play: &Play, v: i64) -> Option<i64> {
    let Play { a, b, p } = play;
    let num = p.0 - v * b.0;
    (num % a.0 == 0).then_some(num / a.0)
}

#[aoc(day13, part2)]
fn solve_part2(input: &ParsedInput) -> Result<i64, String> {
    let res: i64 = input
        .iter()
        .map(|p| Play {
            a: p.a,
            b: p.b,
            p: p.p + Position(10000000000000, 10000000000000),
        })
        .filter_map(|p| Some((p, compute_v(&p)?)))
        .filter_map(|(p, v)| Some((compute_u(&p, v)?, v)))
        .map(|(u, v)| 3 * u + v)
        .sum();

    Ok(res)
}
