use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{BinaryHeap, HashMap};

type Pattern = Vec<u8>;
type ParsedInput = (Vec<Pattern>, Vec<Pattern>);

#[aoc_generator(day19)]
fn parse_day19(input: &str) -> Option<ParsedInput> {
    let (av, wtd) = input.split_once("\n\n")?;
    Some((
        av.split(", ").map(|pat| pat.bytes().collect()).collect(),
        wtd.lines().map(|pat| pat.bytes().collect()).collect(),
    ))
}

#[derive(Default)]
struct PTrie(bool, HashMap<u8, PTrie>);

fn add_to_tree(pattern: &[u8], PTrie(leaf, child): &mut PTrie) {
    match pattern {
        [] => *leaf = true,
        [h, t @ ..] => {
            if let Some(sub_tree) = child.get_mut(h) {
                add_to_tree(t, sub_tree);
            } else {
                let mut newp = PTrie::default();
                add_to_tree(t, &mut newp);
                child.insert(*h, newp);
            }
        }
    }
}

fn build_pattern_tree(availables: &[Pattern]) -> PTrie {
    let mut tree = PTrie::default();
    for pattern in availables.iter() {
        add_to_tree(pattern, &mut tree);
    }
    tree
}

fn possible_next_end(pattern: &[u8], PTrie(leaf, child): &PTrie, depth: usize) -> Vec<usize> {
    let mut sub = match pattern {
        [] => vec![],
        [h, t @ ..] => child
            .get(h)
            .map(|child| possible_next_end(t, child, depth + 1))
            .unwrap_or_default(),
    };
    if *leaf {
        sub.push(depth);
    }
    sub
}

fn possible(pattern: &[u8], availables: &PTrie) -> bool {
    let mut stack = BinaryHeap::<usize>::from([0]);
    let mut found = false;
    while let Some(read_lenght) = stack.pop() {
        if read_lenght == pattern.len() {
            found = true;
            break;
        }
        stack.extend(possible_next_end(&pattern[read_lenght..], availables, read_lenght));
    }
    found
}

fn possibles_count(pat: &[u8], availables: &PTrie) -> usize {
    let mut counts = [vec![1], vec![0; pat.len()]].concat();
    for ind in 0..counts.len() {
        for next_end in possible_next_end(&pat[ind..], availables, ind) {
            counts[next_end] += counts[ind];
        }
    }
    *counts.last().unwrap()
}

#[aoc(day19, part1)]
fn solve_part1((availables, wanted): &ParsedInput) -> Result<usize, String> {
    let tree = build_pattern_tree(availables);
    let possibles_count = wanted
        .iter()
        .map(|wanted| if possible(wanted, &tree) { 1 } else { 0 });
    Ok(possibles_count.sum())
}

#[aoc(day19, part2)]
fn solve_part2((availables, wanteds): &ParsedInput) -> Result<usize, String> {
    let tree = build_pattern_tree(availables);
    let possible_nbr = wanteds
        .iter()
        .map(|wanted| possibles_count(wanted.as_slice(), &tree))
        .sum();
    Ok(possible_nbr)
}
