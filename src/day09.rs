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

type Block = (usize, Option<usize>);

#[aoc_generator(day9)]
fn parse_day9(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!([# usize | ""  / "\n"]);

    let value = parser.parse_top(input)?;
    let (_, _, value) = value.into_tuple();
    let res = value[0]
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if i % 2 == 0 {
                (*x, Some(i / 2))
            } else {
                (*x, None)
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
    input.iter().fold(vec![], |mut acc, (size, value)| {
        match (size, value) {
            (n, None) => acc.extend(std::iter::repeat_n(None, *n)),
            (n, Some(value)) => acc.extend(std::iter::repeat_n(Some(*value), *n)),
        }
        acc
    })
}

#[aoc(day9, part1)]
fn solve_part1(input: &ParsedInput) -> Result<usize, String> {
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
    let debug = false;

    let mut target = input.clone();
    let ids: Vec<_> = input.iter().filter_map(|b| b.1).rev().collect();

    if debug {
        print(&to_block(&target));
        println!("");
    }

    ids.iter().for_each(|to_move_id| {
        //
        // Look for a potential swapping of blocks
        let swap = target
            .iter()
            .enumerate()
            .rev() // Goes backward,
            .fold(
                None::<(Block, usize, Block, usize)>,
                |swap, (curr_pos, block)| {
                    match (swap, block) {
                        // find the block to be moved
                        (None, file @ (size, Some(id))) if id == to_move_id => {
                            //By default, swap the bloc with itself
                            Some((*file, curr_pos, (*size, None), curr_pos))
                        }
                        // and check afterwards for potential new place
                        (Some((file, file_pos, _, _)), free @ (free_space_size, None))
                            if file.0 <= *free_space_size =>
                        {
                            Some((file, file_pos, *free, curr_pos))
                        }
                        _ => swap,
                    }
                },
            )
            .filter(|(_, file_pos, _, target_pos)| file_pos != target_pos);


        // Do the swap
        if let Some((block, block_pos, free, free_pos)) = swap {
            target[block_pos] = (block.0, None);
            if free.0 > block.0 {
                target[free_pos] = (free.0 - block.0, None);
            } else {
                target.remove(free_pos);
            }
            target.insert(free_pos, block);
        }
    });

    let res: usize = to_block(&target)
        .iter()
        .enumerate()
        .map(|(i, x)| i * x.unwrap_or(0))
        .sum();

    Ok(res)
}
