use std::borrow::Borrow;

use crate::coord_iter::coords_along;
use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = Vec<Vec<char>>;

#[aoc_generator(day4)]
fn parse_whatever(input: &str) -> ParsedInput {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[allow(dead_code)]
fn mark_string(
    array: &mut Vec<Vec<char>>,
    s: &str,
    x: isize,
    y: isize,
    delta_x: isize,
    delta_y: isize,
) {
    if (0isize..(array[0].len() as isize)).contains(&(x + delta_x * (s.len() - 1) as isize))
        && (0isize..(array.len() as isize)).contains(&(y + delta_y * (s.len() - 1) as isize))
    {
        s.chars().enumerate().for_each(|(i, c)| {
            array[(y + delta_y * (i as isize)) as usize][(x + delta_x * (i as isize)) as usize] = c;
        });
    }
}

#[allow(dead_code)]
fn print(array: &Vec<Vec<char>>) {
    array.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{c}"));
        println!("");
    })
}

type Coord = (usize, usize);
type Delta = (isize, isize);

fn find_string(array: &Vec<Vec<char>>, s: &str, coords: Coord, delta: impl Borrow<Delta>) -> bool {
    let coords = coords_along(coords, delta, s.len(), (array[0].len(), array.len()));
    coords.len() == s.len()
        && coords
            .zip(s.chars())
            .all(|(coord, value)| array[coord.1][coord.0] == value)
}

#[aoc(day4, part1)]
fn solve(input: &ParsedInput) -> Result<usize, String> {
    let h = input.len();
    let w = input[0].len();

    let dir: Vec<(isize, isize)> = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let search = "XMAS";
    let mut occurence = 0;
    for y in 0..h {
        for x in 0..w {
            occurence += dir
                .iter()
                .filter(|delta| find_string(&input, &search, (x, y), *delta))
                .count();
        }
    }
    Ok(occurence)
}

static RIGHT_UP: Delta = (1, 1);
static LEFT_UP: Delta = (-1, 1);
static LEFT_DOWN: Delta = (-1, -1);
static RIGHT_DOWN: Delta = (1, -1);

static DIAG_VALUES: [&'static str; 4] = ["AM", "AM", "AS", "AS"];
static CONSECUTIVE_DIAGS: [Delta; 4] = [RIGHT_UP, LEFT_UP, LEFT_DOWN, RIGHT_DOWN];

fn cross_mas(array: &Vec<Vec<char>>, coords: Coord, first_mas_ind: usize) -> bool {
    (0..4)
        .map(|i| CONSECUTIVE_DIAGS[(i + first_mas_ind) % 4]) // Modulo magic to cycle through diagonals
        .zip(DIAG_VALUES.iter())
        .all(|(diag, value)| find_string(array, value, coords, diag))
}

#[aoc(day4, part2)]
fn count_cross_mas(input: &ParsedInput) -> Result<usize, String> {
    let h = input.len();
    let w = input[0].len();

    let mut occurence = 0;
    for y in 0..h {
        for x in 0..w {
            occurence += (0..4)
                .filter(|starting_diag| cross_mas(&input, (x, y), *starting_diag))
                .count();
        }
    }
    Ok(occurence)
}
