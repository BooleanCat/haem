use pyo3::prelude::*;
use rayon::iter::{once, IntoParallelIterator};
use rayon::prelude::*;

pub trait Member<T> {
    fn add(&self, other: &[T], swap: bool) -> Vec<T>;
}

impl<T> Member<T> for T
where
    T: Sync + Send + Clone,
    for<'a> &'a [T]: IntoParallelIterator<Item = &'a T>,
{
    fn add(&self, other: &[T], swap: bool) -> Vec<T> {
        match swap {
            true => other.par_iter().chain(once(self)).cloned().collect(),
            false => once(self).chain(other.par_iter()).cloned().collect(),
        }
    }
}

#[derive(FromPyObject)]
pub enum MemberOrCode<T> {
    Member(T),
    Code(char),
}

#[derive(FromPyObject)]
pub enum MemberOrMembers<T> {
    Member(T),
    Sequence(Vec<T>),
}
