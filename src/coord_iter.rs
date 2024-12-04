use std::borrow::Borrow;
use std::cmp::{min, Ordering};
use std::iter;

struct CoordIter {
    x: usize,
    y: usize,
    x_max: usize,
    y_max: usize,

    delta_x: isize,
    delta_y: isize,
}

type Coord = [usize; 2];
type Delta = [isize; 2];

fn checked_delta(value: impl Borrow<usize>, delta: isize) -> Option<usize> {
    match delta.cmp(&0isize) {
        Ordering::Less => value.borrow().checked_sub(delta.abs_diff(0)),
        Ordering::Equal => Some(*value.borrow()),
        Ordering::Greater => value.borrow().checked_add(delta as usize),
    }
}

fn safe_steps(coord: usize, delta: isize, steps: usize, max: usize) -> usize {
    if coord >= max {
        0
    } else {
        match delta.cmp(&0) {
            std::cmp::Ordering::Less => min(coord + 1, steps),
            std::cmp::Ordering::Equal => steps,
            std::cmp::Ordering::Greater => min(max.saturating_sub(coord), steps),
        }
    }
}

pub fn coords_along(
    coord: Coord,
    delta: Delta,
    steps: usize,
    max: Coord,
) -> impl ExactSizeIterator<Item = Coord> {
    let capped_steps = std::cmp::min(
        safe_steps(coord[0], delta[0], steps, max[0]),
        safe_steps(coord[1], delta[1], steps, max[1]),
    );

    (0..capped_steps)
        .map(move |step| {
            let step_coord: [usize; 2] = coord
                .iter()
                .zip(delta.into_iter())
                .map(move |(c, d)| checked_delta(c, d * step as isize).unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            step_coord
        })
        .into_iter()
}

impl Iterator for CoordIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
