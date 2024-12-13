use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Play = (i64, i64, i64, i64, i64, i64);

#[aoc_generator(day13)]
fn parse_day13(input: &str) -> Vec<Play> {
    input
        .chars()
        .chunk_by(|c| char::is_ascii_digit(c))
        .into_iter()
        .filter_map(|(_, v)| v.collect::<String>().parse().ok())
        .tuples()
        .collect_vec()
}

// Literally follow the proposed algorithm, fast to write, quite unefficient
fn find_token_nbr((ax, ay, bx, by, px, py): &Play) -> Option<i64> {
    (0..100)
        .cartesian_product(0..100)
        .filter(|(an, bn)| (ax * an + bx * bn == *px) && (ay * an + by * bn == *py))
        .map(|(an, bn)| an * 3 + bn)
        .min()
}

#[aoc(day13, part1)]
fn solve_part1(input: &[Play]) -> i64 {
    input.iter().filter_map(|p| find_token_nbr(p)).sum()
}

// Solve au + bv = p like a regular two unknows - two equations system
fn solve_equation_system((ax, ay, bx, by, px, py): &Play) -> Option<(i64, i64)> {
    let (v, rem) = {
        let num = py * ax - px * ay;
        let denum = by * ax - bx * ay;
        (num / denum, num % denum)
    };
    if rem == 0 {
        let num = px - v * bx;
        (num % ax == 0).then_some((num / ax, v))
    } else {
        None
    }
}

#[aoc(day13, part2)]
fn solve_part2(input: &[Play]) -> i64 {
    input
        .iter()
        .filter_map(|play| solve_equation_system(play))
        .map(|(u, v)| 3 * u + v)
        .sum()
}
