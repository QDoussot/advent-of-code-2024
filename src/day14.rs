use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;
use huparse::{parse::Parse, parser};

type ParsedInput = Vec<(i32, i32, i32, i32)>;

static W: i32 = 101;
static H: i32 = 103;

#[aoc_generator(day14)]
fn parse_day14(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!([("p=%,% v=%,%", i32, i32, i32, i32) | "\n"]);
    parser.parse_top(input)
}

#[aoc(day14, part1)]
fn solve_part1(input: &ParsedInput) -> i32 {
    input
        .iter()
        .map(|(px, py, vx, vy)| ((px + vx * 100).rem_euclid(W), (py + vy * 100).rem_euclid(H)))
        .filter(|(x, y)| *x != W / 2 && *y != H / 2)
        .fold([0, 0, 0, 0], |mut acc, (x, y)| {
            acc[(x / ((W + 1) / 2) + y / ((H + 1) / 2) * 2) as usize] += 1;
            acc
        })
        .iter()
        .product()
}

#[aoc(day14, part2)]
fn solve_part2(input: &ParsedInput) -> Result<i32, String> {
    let tree_pattern = std::iter::once((2, 0)) //    ..*..
        .chain((1..=3).zip(std::iter::repeat(1))) // .***.
        .chain((0..=4).zip(std::iter::repeat(2))) // *****
        .collect_vec();

    for inc in 0..(W * H) {
        let mut map = vec![[vec!['.'; W as usize], vec!['\n']].concat(); H as usize];

        input
            .iter()
            .map(|(px, py, vx, vy)| ((px + vx * inc).rem_euclid(W), (py + vy * inc).rem_euclid(H)))
            .for_each(|(x, y)| map[y as usize][x as usize] = '*');

        for (y, x) in (0..map.len() - 2).cartesian_product(0..map[0].len() - 4 - 1) {
            if tree_pattern.iter().all(|e| map[y + e.1][x + e.0] == '*') {
                for (y, x) in (0..map.len()).cartesian_product(0..map[0].len()) {
                    print!("{}", map[y][x]); // You need visual check, right ?
                }
                return Ok(inc);
            }
        }
    }
    Err("No tree found".into())
}
