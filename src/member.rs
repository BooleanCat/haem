use crate::utils::Wrapper;
use pyo3::prelude::*;
use pyo3::pyclass::PyClass;
use pyo3::types::PyIterator;
use rayon::prelude::*;

pub trait Member<T> {
    fn add(&self, other: &[T], swap: bool) -> PyResult<Vec<T>>;
}

impl<T> Member<T> for T
where
    T: TryFrom<char, Error = PyErr> + Send + Clone + PartialEq,
{
    fn add(&self, other: &[T], swap: bool) -> PyResult<Vec<T>> {
        let mut members = Vec::with_capacity(1 + other.len());

        match swap {
            true => {
                members.extend(other.iter().cloned());
                members.push(self.clone());
            }
            false => {
                members.push(self.clone());
                members.extend(other.iter().cloned());
            }
        }

        Ok(members.to_vec())
    }
}

impl<T> TryFrom<&str> for Wrapper<Vec<T>>
where
    T: TryFrom<char, Error = PyErr> + Send,
{
    type Error = PyErr;

    fn try_from(codes: &str) -> PyResult<Self> {
        Ok(Wrapper(
            codes
                .as_parallel_string()
                .par_chars()
                .map(T::try_from)
                .collect::<PyResult<_>>()?,
        ))
    }
}

impl<T> TryFrom<Vec<char>> for Wrapper<Vec<T>>
where
    T: TryFrom<char, Error = PyErr> + Send,
{
    type Error = PyErr;

    fn try_from(codes: Vec<char>) -> PyResult<Self> {
        Ok(Wrapper(
            codes
                .into_par_iter()
                .map(T::try_from)
                .collect::<PyResult<_>>()?,
        ))
    }
}

impl<'py, T> TryFrom<Bound<'py, PyIterator>> for Wrapper<Vec<T>>
where
    T: TryFrom<char, Error = PyErr> + PyClass + Clone,
{
    type Error = PyErr;

    fn try_from(iterator: Bound<'py, PyIterator>) -> PyResult<Self> {
        Ok(Wrapper(
            iterator
                .map(|member_or_code| {
                    Ok(
                        Wrapper::try_from(MemberOrCode::extract_bound(&member_or_code?)?)?
                            .into_inner(),
                    )
                })
                .collect::<PyResult<_>>()?,
        ))
    }
}

#[derive(FromPyObject)]
pub enum MemberOrCode<T> {
    Member(T),
    Code(char),
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

#[derive(FromPyObject)]
pub enum MemberOrMembers<T> {
    Member(T),
    Sequence(Vec<T>),
}
