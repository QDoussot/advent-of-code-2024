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

fn find_string(
    array: &Vec<Vec<char>>,
    s: &str,
    x: usize,
    y: usize,
    delta_x: isize,
    delta_y: isize,
) -> bool {
    let coords = coords_along(
        (x, y),
        (delta_x, delta_y),
        s.len(),
        (array[0].len(), array.len()),
    );
    coords.len() == s.len() &&
        coords
    .zip(s.chars())
    .all(|(coord, value)| array[coord.1][coord.0] == value)
}

#[aoc(day4, part1)]
fn solve(input: &ParsedInput) -> Result<usize, String> {
    let h = input.len();
    let w = input[0].len();

    let search = "XMAS";
    let mut occurence = 0;
    for y in 0..h {
        for x in 0..w {
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

            occurence += dir
                .iter()
                .filter(|(dx, dy)| find_string(&input, &search, x , y , *dx, *dy))
                .count();
        }
    }
    Ok(occurence)
}

fn cross_mas(array: &Vec<Vec<char>>, x: usize, y: usize, delta_x: isize, delta_y: isize) -> bool {
    if delta_y != 0 {
        let (dx1, dx2) = (1, -1);
        let (dy1, dy2) = (delta_y, delta_y);

        return find_string(array, "AM", x, y, dx1, dy1)
            && find_string(array, "AM", x, y, dx2, dy2)
            && find_string(array, "AS", x, y, dx1, -dy1)
            && find_string(array, "AS", x, y, dx2, -dy2);
    } else if delta_x != 0 {
        let (dx1, dx2) = (delta_x, delta_x);
        let (dy1, dy2) = (1, -1);

        return find_string(array, "AM", x, y, dx1, dy1)
            && find_string(array, "AM", x, y, dx2, dy2)
            && find_string(array, "AS", x, y, -dx1, dy1)
            && find_string(array, "AS", x, y, -dx2, dy2);
    } else {
        return false;
    }
}

#[aoc(day4, part2)]
fn count_cross_mas(input: &ParsedInput) -> Result<usize, String> {
    let h = input.len();
    let w = input[0].len();

    let mut occurence = 0;
    for y in 0..h {
        for x in 0..w {
            let dir: Vec<(isize, isize)> = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];

            occurence += dir
                .iter()
                .filter(|(dx, dy)| cross_mas(&input, x , y , *dx, *dy))
                .count();
        }
    }
    Ok(occurence)
}
