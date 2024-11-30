use pyo3::prelude::*;
use rayon::iter::{once, IntoParallelIterator};
use rayon::prelude::*;

pub trait Member {
    fn add(&self, other: &[Self], swap: bool) -> Vec<Self>
    where
        Self: Sync + Send + Clone,
        for<'a> &'a [Self]: IntoParallelIterator<Item = &'a Self>,
    {
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

impl<T> MemberOrCode<T>
where
    T: TryFrom<char, Error = PyErr>,
{
    pub fn into_member(self) -> PyResult<T> {
        match self {
            MemberOrCode::Member(member) => Ok(member),
            MemberOrCode::Code(code) => code.try_into(),
        }
    }
}

#[derive(FromPyObject)]
pub enum MemberOrMembers<T> {
    Member(T),
    Sequence(Vec<T>),
}
