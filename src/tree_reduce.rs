use std::{
    fmt::{Debug, Display},
    iter,
};

pub enum TreeElement<N, T> {
    Node(N),
    Collapsed(T),
}

pub trait TreeReduce<N, T> {
    fn generate_child(&self, depth: usize, node: &N) -> Vec<TreeElement<N, T>>;
    fn collapse(&self, node: &N) -> T;
    fn reduce(&self, it: impl Iterator<Item = T>) -> T;
}

pub trait TreeReduceDebug<N: Display, T: Debug>: TreeReduce<N, T> {
    fn generate_child_debug(&self, depth: usize, node: &N) -> Vec<(String, TreeElement<N, T>)>;
    fn compute_debug(&self, mut debug: String, node: &N, depth: usize) -> T {
        let childs = self.generate_child_debug(depth, &node);
        if childs.is_empty() {
            let collapsed = self.collapse(&node);
            println!("{debug}* = {collapsed:?}");
            collapsed
        } else {
            self.reduce(childs.into_iter().map(move |child| match child.1 {
                TreeElement::Node(node) => self.compute_debug(
                    format!("{debug} -({})-> {}", child.0, node),
                    &node,
                    depth + 1,
                ),
                TreeElement::Collapsed(leaf) => {
                    println!("{debug} [{}]* = {leaf:?}",child.0);
                    leaf
                }
            }))
        }
    }
}

pub trait TreeReduceCompute<N, T>: TreeReduce<N, T> {
    fn compute(&self, node: &N, depth: usize) -> T {
        let childs = self.generate_child(depth, &node);
        if childs.is_empty() {
            self.collapse(&node)
        } else {
            self.reduce(childs.into_iter().map(move |child| match child {
                TreeElement::Node(node) => self.compute(&node, depth + 1),
                TreeElement::Collapsed(leaf) => leaf,
            }))
        }
    }
}

impl<N, T, TR> TreeReduceCompute<N, T> for TR where TR: TreeReduce<N, T> {}

/// * `child_generator` Lambda taking two arguments: current depth and a reference on current node, and should generate childs
/// * `collapse` Lambda transforming a node without child into its collapsed value
/// * `reduce` Lambda taking an boxed iterator on values of collapsed childs, and generate a
/// reduced value of it
pub fn tree_reduce<'a, N, G, V, C, R>(
    node: N,
    depth: usize,
    child_generator: &'a G,
    collapse: &'a C,
    reduce: &'a R,
) -> V
where
    N: 'a,
    V: 'a,
    G: Fn(usize, &N) -> Vec<TreeElement<N, V>>, // Generate
    C: Fn(&N) -> V,                             // Collapse
    R: Fn(Box<dyn Iterator<Item = V> + 'a>) -> V, // Reduce
{
    let childs = child_generator(depth, &node);
    if childs.is_empty() {
        collapse(&node)
    } else {
        reduce(Box::new(childs.into_iter().map(move |child| match child {
            TreeElement::Node(node) => {
                tree_reduce(node, depth + 1, child_generator, collapse, reduce)
            }
            TreeElement::Collapsed(value) => value,
        })))
    }
}

#[cfg(test)]
mod tests {

    use super::{tree_reduce, TreeElement};
    #[test]
    fn it_follow_tree_and_reduce() {
        //   0
        //   1      2
        //   3  4   5   6
        //      7   8 9
        let tree: Vec<Vec<usize>> = vec![
            /* 0 */ vec![1, 2],
            /* 1 */ vec![3, 4],
            /* 2 */ vec![5, 6],
            /* 3 */ vec![],
            /* 4 */ vec![7],
            /* 5 */ vec![8, 9],
            /* 6 */ vec![],
            /* 7 */ vec![],
            /* 8 */ vec![],
            /* 9 */ vec![],
        ];

        // return a vector of the leafs
        let leafs: Vec<_> = tree_reduce(
            &0,
            1,
            &|_, node| {
                tree.get(**node)
                    .unwrap()
                    .iter()
                    .map(|child| TreeElement::Node(child))
                    .collect()
            },
            &|node| vec![*node],
            &|it| it.flatten().collect::<Vec<_>>(),
        );
        assert_eq!(leafs, vec![&3, &7, &8, &9, &6]);

        // Just generate the path through the leafs
        let paths: Vec<Vec<_>> = tree_reduce(
            vec![0],
            1,
            &|_, node| {
                tree.get(*node.last().unwrap())
                    .unwrap()
                    .iter()
                    .map(|child| {
                        let mut node = node.clone();
                        node.push(*child);
                        TreeElement::Node(node)
                    })
                    .collect()
            },
            &|node| vec![node.clone()],
            &|it| it.flatten().collect::<Vec<_>>(),
        );
        println!("{paths:?}");
    }
}
