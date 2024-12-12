use eyre::eyre;
use itertools::Itertools;
use std::collections::HashMap;

use crate::space2d::{BoundingBox, Field};
use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Report;

use huparse::parse::Parse;
use huparse::parser;

use crate::space2d::{Coord, TableField};

type ParsedInput = TableField<char>;

fn get_connexe(
    coord: &Coord,
    tag: usize,
    ground: &TableField<char>,
    connexe_map: &mut HashMap<Coord, usize>,
    fences_per_tag: &mut HashMap<usize, Vec<Coord>>,
) {
    // Connexes parts
    let child = [Coord(1, 0), Coord(0, 1), Coord(-1, 0), Coord(0, -1)]
        .into_iter()
        .map(|d| (*coord + d))
        .filter(|nc| ground.definition_area().contains(&nc))
        .filter(|nc| !connexe_map.contains_key(&nc))
        .filter(|nc| ground.get(&nc) == ground.get(coord))
        .collect::<Vec<_>>();

    // Here is the trick
    // The space is mapped to a 3 timers bigger space
    // - Previous cells are mapped to 3*x+2,3*y+2 coordinates
    // - Each fence has two corresponding value in the mapped space
    //   - One for each tag on its left/right side, if the fence is vertical
    //   - One for each tag on its bottom/top side, if the fence is horizontal
    // So the maps looks like
    //
    //    ┴  ┴  ┴
    //    ┬  ┬  ┬
    //  ┤├A┤├B  B┤├
    //          ┴
    //          ┬
    //  ┤├A┤├B  C┤├
    //
    // where:
    //  - ├  is right side of fence
    //
    //  - ┤  is left side of fence
    //
    //  - ┬  is down side of fence
    //
    //  - ┴  is top side of fence
    //
    let out_fences = [Coord(1, 0), Coord(0, 1), Coord(-1, 0), Coord(0, -1)]
        .into_iter()
        .filter(|d| !ground.definition_area().contains(&(*coord + *d)))
        .map(|d| *coord + *coord + *coord + d + Coord(2, 2))
        .collect();

    let fences = [Coord(1, 0), Coord(0, 1), Coord(-1, 0), Coord(0, -1)]
        .into_iter()
        .filter(|d| ground.definition_area().contains(&(*coord + *d)))
        .filter(|d| ground.get(&(*coord + *d)) != ground.get(coord))
        .map(|d| *coord + *coord + *coord + d + Coord(2, 2))
        .collect();

    // update fence coords per tag
    let new_count = [
        fences_per_tag
            .remove_entry(&tag)
            .map(|(_, v)| v)
            .unwrap_or_default(),
        fences,
        out_fences,
    ]
    .concat();
    fences_per_tag.insert(tag, new_count);

    // Update connexity map and recurse
    child.iter().for_each(|c| {
        connexe_map.insert(*c, tag);
    });
    child
        .iter()
        .for_each(|c| get_connexe(c, tag, ground, connexe_map, fences_per_tag));
}

#[aoc_generator(day12)]
fn parse_day12(input: &str) -> Result<ParsedInput, Report> {
    let parser = parser!([# char | "" / "\n"]);
    let table = parser.parse_top(input)?;
    TableField::try_from(table).map_err(|_| eyre!(""))
}

#[aoc(day12, part1)]
fn solve_part1(input: &ParsedInput) -> Result<usize, String> {
    let mut connexe = HashMap::<Coord, usize>::new();
    let mut fencing = HashMap::<usize, Vec<Coord>>::new();
    let mut grp = 0;

    // Fill connexity map starting from each cell
    let bb = input.definition_area();
    for y in bb.ymin..bb.ymax {
        for x in bb.xmin..bb.xmax {
            let c = Coord(x, y);
            if !connexe.contains_key(&c) {
                connexe.insert(c, grp);
                get_connexe(&c, grp, input, &mut connexe, &mut fencing);
                grp += 1;
            }
        }
    }

    // Gather each coords per tag
    let coords_per_tag = connexe
        .into_iter()
        .map(|(coord, tag)| (tag, coord))
        .into_group_map()
        .into_iter();

    // Compute part perimeter * part area
    let res = coords_per_tag
        .map(|(tag, coords)| fencing[&tag].len() * coords.len())
        .sum();

    Ok(res)
}

fn display_garden_with_fences(
    garden: &TableField<char>,
    fencing: &HashMap<Coord,usize>,
    bb: &BoundingBox,
) {
    for y in 0..bb.ymax * 3 + 1 {
        for x in 0..bb.xmax * 3 + 1 {
            if let Some(tag) = fencing.get(&Coord(x, y)) {
                print!("{tag}")
            } else if x % 3 == 2 && y % 3 == 2 {
                print!(
                    "{}",
                    *garden.get(&Coord((x - 2) / 3, (y - 2) / 3)).unwrap_or(&'?')
                );
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

#[aoc(day12, part2)]
fn solve_part2(input: &ParsedInput) -> Result<usize, String> {
    let mut connexe = HashMap::<Coord, usize>::new();
    let mut fencing = HashMap::<usize, Vec<Coord>>::new();
    let mut tag_letter = HashMap::<usize, char>::new();
    let mut grp = 0;
    let bb = input.definition_area();
    for y in bb.ymin..bb.ymax {
        for x in bb.xmin..bb.xmax {
            let c = Coord(x, y);
            if !connexe.contains_key(&c) {
                tag_letter.insert(grp, *input.get(&c).unwrap());
                connexe.insert(c, grp);
                get_connexe(&c, grp, input, &mut connexe, &mut fencing);
                grp += 1;
            }
            //
        }
    }
    // Generate a map of fences side tag
    let fencing: HashMap<_, _> = fencing
        .into_iter()
        .map(|(tag, coords)| coords.into_iter().map(move |c| (c, tag)))
        .flatten()
        .collect();

    display_garden_with_fences(&input, &fencing, &bb);

    // Count, per tag, number of consecutives fence part (with same tag) for each (columns,side)
    let mut region_sides = HashMap::<usize, usize>::new();
    for x in 0..(bb.xmax * 3 + 2) {
        if x % 3 != 2 {
            println!("{x}");
            let col_side_counts = (0..bb.ymax)
                .map(|y| fencing.get(&Coord(x, y * 3 + 2)))
                .dedup_by(|t1, t2| match (t1, t2) {
                    (None, None) => true,
                    (Some(t1), Some(t2)) => t1 == t2,
                    _ => false,
                })
                .flatten()
                .counts();
            println!("{col_side_counts:?}");
            col_side_counts.iter().for_each(|(tag, count)| {
                let new_count = region_sides.remove_entry(*tag).map(|x| x.1).unwrap_or(0) + count;
                region_sides.insert(**tag, new_count);
            });
        }
    }

    // Count, per tag, number of consecutives fence part (with same tag) for each (row,side)
    for y in 0..(bb.ymax * 3 + 2) {
        if y % 3 != 2 {
            println!("{y}");
            let row_side_counts = (0..bb.ymax)
                .map(|x| fencing.get(&Coord(x * 3 + 2, y)))
                .dedup_by(|t1, t2| match (t1, t2) {
                    (None, None) => true,
                    (Some(t1), Some(t2)) => t1 == t2,
                    _ => false,
                })
                .flatten()
                .counts();
            println!("{row_side_counts:?}");
            row_side_counts.iter().for_each(|(tag, count)| {
                let new_count = region_sides.remove_entry(*tag).map(|x| x.1).unwrap_or(0) + count;
                region_sides.insert(**tag, new_count);
            });
        }
    }

    // Compute the price number of regions sides
    let res: usize = connexe
        .into_iter()
        .map(|(k, v)| (v, k))
        .into_group_map()
        .into_iter()
        .map(|(tag, vec)| region_sides[&tag] * vec.len())
        .sum();

    Ok(res)
}
