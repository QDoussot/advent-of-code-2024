use std::marker::PhantomData;
pub trait Relation<T>: Copy {
    fn relates(&self, operandes: (&T, &T)) -> bool;
    fn into_fn(&self) -> impl FnMut((&T, &T)) -> bool {
        |(a, b)| self.relates((a, b))
    }
    fn into_ref_fn(&self) -> impl FnMut(&(&T, &T)) -> bool {
        |(a, b)| self.relates((a, b))
    }
}

impl<T, F> Relation<T> for F
where
    F: Fn((&T, &T)) -> bool + Copy,
{
    fn relates(&self, operandes: (&T, &T)) -> bool {
        self(operandes)
    }
}

pub struct AndRelation<T, R1: Relation<T>, R2: Relation<T>>(R1, R2, PhantomData<T>);

impl<T, R1: Relation<T>, R2: Relation<T>> Relation<T> for AndRelation<T, R1, R2> {
    fn relates(&self, operandes: (&T, &T)) -> bool {
        self.0.relates(operandes) && self.1.relates(operandes)
    }
}

impl<T, R1: Relation<T>, R2: Relation<T>> Copy for AndRelation<T, R1, R2> {}
impl<T, R1: Relation<T>, R2: Relation<T>> Clone for AndRelation<T, R1, R2> {
    fn clone(&self) -> Self {
        *self
    }
}

pub fn and<T, R1: Relation<T>, R2: Relation<T>>(r1: R1, r2: R2) -> AndRelation<T, R1, R2> {
    AndRelation(r1, r2, PhantomData::default())
}

pub struct OrRelation<T, R1: Relation<T>, R2: Relation<T>>(R1, R2, PhantomData<T>);

impl<T, R1: Relation<T>, R2: Relation<T>> Relation<T> for OrRelation<T, R1, R2> {
    fn relates(&self, operandes: (&T, &T)) -> bool {
        self.0.relates(operandes) || self.1.relates(operandes)
    }
}

#[allow(dead_code)]
pub fn or<T, R1: Relation<T>, R2: Relation<T>>(r1: R1, r2: R2) -> OrRelation<T, R1, R2> {
    OrRelation(r1, r2, PhantomData::default())
}

impl<T, R1: Relation<T>, R2: Relation<T>> Copy for OrRelation<T, R1, R2> {}
impl<T, R1: Relation<T>, R2: Relation<T>> Clone for OrRelation<T, R1, R2> {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct NotRelation<T, R1: Relation<T>>(R1, PhantomData<T>);

impl<T, R1: Relation<T>> Copy for NotRelation<T, R1> {}
impl<T, R1: Relation<T>> Clone for NotRelation<T, R1> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, R1: Relation<T>> Relation<T> for NotRelation<T, R1> {
    fn relates(&self, operandes: (&T, &T)) -> bool {
        !self.0.relates(operandes)
    }
}

pub fn not<T, R1: Relation<T>>(r1: R1) -> NotRelation<T, R1> {
    NotRelation(r1, PhantomData::default())
}

#[cfg(test)]
mod tests {

    use super::and;

    #[test]
    fn lolila() {
        let increasing = |(a, b): (&usize, &usize)| a < b;
        let decreasing = |(a, b): (&usize, &usize)| a > b;
        let impossible = and(increasing, |(a, b): (&usize, &usize)| a.abs_diff(*b) < 4);
        let impossible = and(increasing, decreasing);
    }
}
