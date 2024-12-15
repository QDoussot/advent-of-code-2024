use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

macro_rules! twod {
    ($map:ident[$coord:expr]) => {
        $map[$coord.1][$coord.0]
    };
}

type Coord = (usize, usize);
static LEFT: fn(Coord) -> (usize, usize) = |(x, y)| (x - 1, y);
static RIGHT: fn(Coord) -> (usize, usize) = |(x, y)| (x + 1, y);

fn as_move(d: char) -> fn(Coord) -> Coord {
    match d {
        '<' => LEFT,
        '>' => RIGHT,
        'v' => |(x, y)| (x, y + 1),
        '^' => |(x, y)| (x, y - 1),
        _ => unreachable!(),
    }
}

fn do_move(c: Coord, dc: fn(Coord) -> Coord, map: &mut Vec<Vec<char>>, act: bool) -> bool {
    let can_move = match (twod!(map[dc(c)]), dc((0, 0))) {
        ('.', _) => true,
        ('#', _) => false,
        ('[', (_, 0)) | (']', (_, 0)) | ('@', _) | ('O', _) => do_move(dc(c), dc, map, act),
        ('[', (0, _)) => do_move(dc(c), dc, map, act) && do_move(dc(RIGHT(c)), dc, map, act),
        (']', (0, _)) => do_move(dc(c), dc, map, act) && do_move(dc(LEFT(c)), dc, map, act),
        _ => unreachable!(),
    };

    if can_move && act {
        twod!(map[dc(c)]) = twod!(map[c]);
        twod!(map[c]) = '.';
    }
    can_move
}

fn solve(mut map: Vec<Vec<char>>, moves: &str, acting_level: &[bool]) -> usize {
    let mut robot = (0..map[0].len())
        .cartesian_product(0..map.len())
        .find(|(x, y)| map[*y][*x] == '@')
        .unwrap();
    moves.chars().for_each(|dir| {
        if acting_level
            .iter()
            .all(|&act| do_move(robot, as_move(dir), &mut map, act))
        {
            robot = as_move(dir)(robot);
        }
    });
    (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter_map(|(y, x)| (map[y][x] == 'O' || map[y][x] == '[').then_some(y * 100 + x))
        .sum()
}

#[aoc_generator(day15)]
fn parse_day15(input: &str) -> Option<(Vec<Vec<char>>, String)> {
    let (map, moves) = input.split_once("\n\n")?;
    let map = map.split("\n").map(|line| line.chars().collect()).collect();
    Some((map, moves.replace("\n", "")))
}

#[aoc(day15, part1)]
fn solve_part1((map, moves): &(Vec<Vec<char>>, String)) -> usize {
    solve(map.clone(), moves, &[true])
}

#[aoc(day15, part2)]
fn solve_part2((map, moves): &(Vec<Vec<char>>, String)) -> usize {
    let double = map
        .iter()
        .map(|line| {
            line.iter()
                .flat_map(|&c| match c {
                    '@' => vec![c, '.'],
                    'O' => vec!['[', ']'],
                    c => vec![c, c],
                })
                .collect()
        })
        .collect();

    solve(double, moves, &[false, true])
}
