use crate::utils::{AddInput, Wrapper};
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::pyclass::PyClass;
use pyo3::types::PyIterator;
use rayon::prelude::*;

pub trait Member<T> {
    fn add(&self, other: AddInput<T>, swap: bool) -> PyResult<Vec<T>>;
    fn richcmp(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject;
}

impl<T: Clone + PartialEq> Member<T> for T
where
    T: TryFrom<char, Error = PyErr> + Send + Clone,
{
    fn add(&self, other: AddInput<T>, swap: bool) -> PyResult<Vec<T>> {
        Ok(match other {
            AddInput::Member(member) => match swap {
                true => vec![member, self.clone()],
                false => vec![self.clone(), member],
            },
            AddInput::Members(members) => {
                let mut sequence = Vec::with_capacity(members.len() + 1);
                match swap {
                    false => {
                        sequence.push(self.clone());
                        sequence.extend(members);
                    }
                    true => {
                        sequence.extend(members);
                        sequence.push(self.clone());
                    }
                }
                sequence
            }
            AddInput::Codes(codes) => {
                let mut sequence = Vec::with_capacity(codes.len() + 1);
                match swap {
                    false => {
                        sequence.push(self.clone());
                        sequence.extend(Wrapper::try_from(codes)?.into_inner());
                    }
                    true => {
                        sequence.extend(Wrapper::try_from(codes)?.into_inner());
                        sequence.push(self.clone());
                    }
                }
                sequence
            }
        })
    }

    fn richcmp(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self == other).into_py(py),
            CompareOp::Ne => (self != other).into_py(py),
            _ => py.NotImplemented(),
        }
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

impl<T> TryFrom<&PyIterator> for Wrapper<Vec<T>>
where
    T: TryFrom<char, Error = PyErr> + PyClass + Clone,
{
    type Error = PyErr;

    fn try_from(iterator: &PyIterator) -> PyResult<Self> {
        Ok(Wrapper(
            iterator
                .map(|member_or_code| {
                    Ok(Wrapper::try_from(MemberOrCode::extract(member_or_code?)?)?.into_inner())
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
