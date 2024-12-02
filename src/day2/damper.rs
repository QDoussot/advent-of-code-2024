use aoc_runner_derive::aoc;
use itertools::Itertools;

use crate::day2::are_safe_adjacent;
use crate::day2::Validation;
use crate::day2::{parse_reports, PlantReport};

struct Damped<'a>(&'a PlantReport, usize);

//TODO Eventually, homogeneize interface with PlantReport with a Trait
impl<'a> Damped<'a> {
    pub fn iter(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.0
             .0
            .iter()
            .enumerate()
            .filter_map(|(i, val)| (i != self.1).then_some(*val))
    }

    fn has_safe_adjacent_levels(&self) -> bool {
        self.iter()
            .tuple_windows()
            .all(|(w0, w1)| are_safe_adjacent(w0, w1))
    }

    fn is_safe_pred(&self, predicat: fn(usize, usize) -> bool) -> bool {
        self.iter().tuple_windows().all(|(w0, w1)| predicat(w0, w1))
            && self.has_safe_adjacent_levels()
    }
}

fn find_bad_and_damper(
    report: &PlantReport,
    predicat: fn(usize, usize) -> bool,
) -> Option<Validation> {
    match report
        .0
        .windows(2)
        .find_position(|w| !(predicat(w[0], w[1]) && are_safe_adjacent(w[0], w[1])))
    {
        None => Some(Validation::Safe),
        Some((pos, _)) => Damped(&report, pos)
            .is_safe_pred(predicat)
            .then_some(Validation::Damping(pos))
            .or_else(|| {
                Damped(&report, pos + 1)
                    .is_safe_pred(predicat)
                    .then_some(Validation::Damping(pos + 1))
            }),
    }
}
#[aoc(day2, part2)]
fn number_of_safe_reports_with_dampener_bis(reports: &Vec<PlantReport>) -> usize {
    reports
        .iter()
        .filter(|pr| find_bad_and_damper(&pr, |w0, w1| w0 < w1).is_some()
            || find_bad_and_damper(&pr, |w0, w1| w0 > w1).is_some()
            )
        .count()
}
