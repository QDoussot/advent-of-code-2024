use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;

use itertools::Itertools;
type ParsedInput = Vec<Block>;

#[allow(dead_code)]
fn print(res: &[Option<usize>]) {
    res.iter().for_each(|x| match x {
        Some(n) => print!("{n}"),
        None => print!("."),
    });
    println!("");
}

#[derive(Debug, Copy, Clone)]
enum Block {
    Free(usize),
    Taken(usize, usize),
}

impl Block {
    fn is_taken(&self, id: usize) -> bool {
        match self {
            Block::Free(_) => false,
            Block::Taken(_, actual_id) => id == *actual_id,
        }
    }

    fn is_available_free_space(&self, min_size: usize) -> bool {
        match self {
            Block::Free(n) => *n >= min_size,
            Block::Taken(_, _) => false,
        }
    }
}

#[aoc_generator(day9)]
fn parse_day9(input: &str) -> Result<ParsedInput, Report> {
    use Block::*;
    let parser = parser!([# usize | ""  / "\n"]);

    let value = parser.parse_top(input)?;
    let (_, _, value) = value.into_tuple();
    let res = value[0]
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if i % 2 == 0 {
                Taken(*x, i / 2)
            } else {
                Free(*x)
            }
        })
        .collect();

    Ok(res)
}

//let res = value[0].iter().enumerate().fold(vec![], |mut acc, (i, x)| {
//    if i % 2 == 0 {
//        acc.extend(std::iter::repeat_n(Some(i / 2), *x));
//    } else {
//        acc.extend(std::iter::repeat_n(None, *x));
//    }
//    acc
//});

fn to_block(input: &[Block]) -> Vec<Option<usize>> {
    input.iter().fold(vec![], |mut acc, x| {
        match x {
            Block::Free(n) => acc.extend(std::iter::repeat_n(None, *n)),
            Block::Taken(n, value) => acc.extend(std::iter::repeat_n(Some(*value), *n)),
        }
        acc
    })
}

#[aoc(day9, part1)]
fn solve_part1(input: &ParsedInput) -> Result<usize, String> {
    let mut input = input.clone();
    let mut input = to_block(&input);
    while input
        .iter()
        .dedup_by(|x, y| x.is_some() == y.is_some())
        .count()
        > 2
    {
        if let Some((empty_pos, _)) = input.iter().find_position(|block| block.is_none()) {
            if let Some(tomove) = input.iter().rposition(|e| e.is_some()) {
                input[empty_pos] = Some(input[tomove].unwrap());
                input[tomove] = None;
            }
        }
    }
    let res: usize = input
        .iter()
        .enumerate()
        .map(|(i, x)| i * x.unwrap_or(0))
        .sum();
    Ok(res)
}

#[aoc(day9, part2)]
fn solve_part2(input: &ParsedInput) -> Result<usize, String> {
    use Block::*;

    let debug = false;

    let mut target = input.clone();
    let ids: Vec<_> = input
        .iter()
        .enumerate()
        .filter_map(|(_, b)| match b {
            Free(_) => None,
            Taken(size, id) => Some((size, id)),
        })
        .rev()
        .collect();

    if debug {
        print(&to_block(&target));
        println!("");
    }

    ids.iter().for_each(|(taken_size, taken_id)| {
        let taken_pos = target
            .iter()
            .find_position(|b| b.is_taken(**taken_id))
            .unwrap()
            .0;
        if let Some((free_place_pos, free_space)) =
            target
                .iter()
                .enumerate()
                .find_map(|(free_place_pos, b)| match *b {
                    Block::Free(free_space)
                        if free_space >= **taken_size && free_place_pos < taken_pos =>
                    {
                        Some((free_place_pos, free_space))
                    }
                    _ => None,
                })
        {
            if debug {
                println!("free space for {taken_size}x{taken_id} (currently at {taken_pos}) at {free_place_pos}");
            }
            target[taken_pos] = Free(**taken_size);
            target[free_place_pos] = Taken(**taken_size, **taken_id);

            if debug { print(&to_block(&target)); }
            if free_space > **taken_size {
                target.insert(free_place_pos + 1, Free(free_space - **taken_size));
            }
            if debug { println!(""); }
        }
    });
    let res: usize = to_block(&target)
        .iter()
        .enumerate()
        .map(|(i, x)| i * x.unwrap_or(0))
        .sum();

    Ok(res)
}
