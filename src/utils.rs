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

pub struct Wrapper<T>(pub T);

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
