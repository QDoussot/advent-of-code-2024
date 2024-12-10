use crate::space2d::{Coord, Field};
use crate::tree_reduce::{TreeElement, TreeReduce};

pub mod neighbors {
    use crate::space2d::Coord;

    pub static FOUR: [Coord; 4] = [Coord(1, 0), Coord(0, 1), Coord(-1, 0), Coord(0, -1)];
}

// Describes how to explore the 2d space
pub trait Explore {
    type T;

    fn considered_directions(&self) -> Vec<Coord>;
    fn filtered_directions(&self, value: &Self::T, parent_value: &Self::T) -> bool;
    fn reached_target(&self, value: &Self::T) -> bool;
}


// What is gathered at the end of the path, and how it is aggregated/simplified
// at each cross road when backtracking
pub trait Gather<T> {
    type Loot; // L for loot

    fn bring_back(&self, coord: &Coord, value: &T, reached_target: bool) -> Self::Loot;
    fn gather(&self, it: impl Iterator<Item = Self::Loot>) -> Self::Loot;
}

// Combine exploration and gathering to compute result
pub struct Exploration<'a, F: Field, E: Explore, G: Gather<F::Out>> {
    field: &'a F,
    explore2d: E,
    gather: G,
    //stop_on_target_reached: bool,
}

impl<'a, F: Field, E: Explore, G: Gather<F::Out>> Exploration<'a, F, E, G> {
    pub fn new(field: &'a F, explore2d: E, gather: G) -> Self {
        Self {
            field,
            explore2d,
            gather,
        }
    }
}

impl<'a, F, E, G> TreeReduce<(Coord, F::Out), G::Loot> for Exploration<'a, F, E, G>
where
    E: Explore<T = F::Out>,
    F::Out: Copy,
    F: Field,
    G: Gather<F::Out>,
{
    fn generate_child(
        &self,
        _depth: usize,
        (node, value): &(Coord, F::Out),
    ) -> Vec<TreeElement<(Coord, F::Out), G::Loot>> {
        self.explore2d
            .considered_directions()
            .iter()
            .map(|c| *c + *node)
            //
            // Discard coords out of the field definition
            .filter_map(|c| Some((c, *self.field.get(&c)?)))
            //
            // Now the explorer filter the potential directions
            .filter(|(_, child_value)| self.explore2d.filtered_directions(child_value, value))
            .map(|c| TreeElement::Node::<(Coord, F::Out), G::Loot>(c))
            .collect()
    }

    fn collapse(&self, node: &(Coord, F::Out)) -> G::Loot {
        self.gather
            .bring_back(&node.0, &node.1, self.explore2d.reached_target(&node.1))
    }

    fn reduce(&self, it: impl Iterator<Item = G::Loot>) -> G::Loot {
        self.gather.gather(it)
    }
}
