use pyo3::{prelude::*, PyClass};
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

impl<'py, T> TryFrom<PyResult<Bound<'py, PyAny>>> for MemberOrCode<T>
where
    T: PyClass + Clone,
{
    type Error = PyErr;

    fn try_from(result: PyResult<Bound<'py, PyAny>>) -> PyResult<MemberOrCode<T>> {
        MemberOrCode::extract_bound(&result?)
    }
}

#[derive(FromPyObject)]
pub enum MemberOrMembers<T> {
    Member(T),
    Sequence(Vec<T>),
}
