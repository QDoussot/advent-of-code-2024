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

fn to_block(input: &[Block]) -> Vec<Option<usize>> {
    input.iter().fold(vec![], |mut acc, (size, value)| {
        match (size, value) {
            (n, None) => acc.extend(std::iter::repeat_n(None, *n)),
            (n, Some(value)) => acc.extend(std::iter::repeat_n(Some(*value), *n)),
        }
        acc
    })
}

fn compute_checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, x)| i * x.unwrap_or(0))
        .sum()
}

#[aoc(day9, part1)]
fn solve_part1(input: &ParsedInput) -> Result<usize, String> {
    let mut input = to_block(&input);
    while input
        .iter()
        .map(|x| x.is_some())
        .dedup()
        .collect::<Vec<_>>()
        != vec![true, false]
    {
        if let Some(empty_pos, ) = input.iter().position(|block| block.is_none()) {
            if let Some(tomove) = input.iter().rposition(|e| e.is_some()) {
                input[empty_pos] = Some(input[tomove].unwrap());
                input[tomove] = None;
            }
        }
    }
    Ok(compute_checksum(&input))
}

use crate::functional::do_until_stable;
#[aoc(day9, part1, use_functional)]
fn solve_part1_use_f(input: &ParsedInput) -> Result<usize, String> {
    let input = to_block(&input);
    let compressed = do_until_stable(
        input,
        |mut disk: Vec<Option<usize>>| {
            if let Some(tomove) = disk.iter().rposition(|e| e.is_some()) {
                if let Some(empty_pos) = disk.iter().take(tomove).position(|block| block.is_none()) {
                    disk.swap(empty_pos, tomove);
                }
            }
            disk
        },
        |disk| disk.clone(),
    );

    Ok(compute_checksum(&compressed))
}

type Swap = (Block, usize, Block, usize);

fn find_swap_double_find_version(iter: &[Block], id: &usize) -> Option<Swap> {
    iter.iter()
        .find_position(|block| block.1 == Some(*id))
        .and_then(|(pos, file)| {
            iter.iter()
                .take(pos)
                .find_position(|block| block.0 >= file.0 && block.1.is_none())
                .map(|(free_pos, free)| (*file, pos, *free, free_pos))
        })
}

fn find_swap_fold_version(disk: &[Block], to_move_id: &usize) -> Option<Swap> {
    // Look for a potential swapping of blocks
    disk.iter()
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
        .filter(|(_, file_pos, _, target_pos)| file_pos != target_pos)
}

fn move_file(target: &mut Vec<Block>, swap: Option<Swap>) {
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
}

#[aoc(day9, part2)]
fn solve_part2(input: &ParsedInput) -> Result<usize, String> {
    let mut target = input.clone();
    let ids: Vec<_> = input.iter().filter_map(|b| b.1).rev().collect();

    ids.iter().for_each(|to_move_id| {
        let swap = find_swap_double_find_version(&target, to_move_id);
        move_file(&mut target, swap);
    });

    let res: usize = compute_checksum(&to_block(&target));

    Ok(res)
}
