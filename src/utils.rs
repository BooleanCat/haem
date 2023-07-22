use pyo3::prelude::*;
use pyo3::types::PySlice;

#[derive(FromPyObject)]
pub enum IntOrSlice<'a> {
    Int(isize),
    Slice(&'a PySlice),
}

#[derive(FromPyObject)]
pub enum MemberOrSequence<T> {
    Member(T),
    Sequence(Vec<T>),
}

#[derive(FromPyObject)]
pub enum MemberOrCode<T> {
    Member(T),
    Code(char),
}

pub struct Wrapper<T>(T)
where
    T: TryFrom<char, Error = PyErr>;

impl<T> Wrapper<T>
where
    T: TryFrom<char, Error = PyErr>,
{
    pub fn peel(self) -> T {
        self.0
    }
}

impl<T> TryFrom<MemberOrCode<T>> for Wrapper<T>
where
    T: TryFrom<char, Error = PyErr>,
{
    type Error = PyErr;

    fn try_from(code: MemberOrCode<T>) -> PyResult<Wrapper<T>> {
        Ok(match code {
            MemberOrCode::Member(member) => Wrapper(member),
            MemberOrCode::Code(code) => Wrapper(code.try_into()?),
        })
    }
}
