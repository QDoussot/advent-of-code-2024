pub enum TreeElement<N, T> {
    Node(N),
    Collapsed(T),
}

pub trait TreeReduce<N, T> {
    fn generate_child(&self, depth: usize, node: &N) -> Vec<TreeElement<N, T>>;
    fn collapse(&self, node: &N) -> T;
    fn reduce(&self, it: impl Iterator<Item = T>) -> T;
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

pub fn tree_reduce<'a, N, G, V, C, R>(
    node: N,
    depth: usize,
    child_generator: &'a G,
    collapse: &'a C,
    reduce: &'a R,
) -> V
where
    N: 'static,
    G: Fn(usize, &N) -> Vec<TreeElement<N, V>>, // Generate
    C: Fn(&N) -> V,                             // Collapse
    R: Fn(Box<dyn Iterator<Item = V> + 'a>) -> V, // Reduce
    V: 'static,                                 // V
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
