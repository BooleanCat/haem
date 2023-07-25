use crate::utils::{MemberOrSequence, Wrapper};
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::pyclass::PyClass;
use pyo3::types::PyIterator;
use rayon::prelude::*;

pub trait Member<T> {
    fn add(&self, other: MemberOrSequence<T>) -> Vec<T>;
    fn richcmp(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject;
}

impl<T: Clone + PartialEq> Member<T> for T {
    fn add(&self, other: MemberOrSequence<T>) -> Vec<T> {
        match other {
            MemberOrSequence::Member(member) => {
                vec![self.clone(), member]
            }
            MemberOrSequence::Sequence(sequence) => {
                let mut sequence = sequence;
                sequence.insert(0, self.clone());
                sequence
            }
        }
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
                .collect::<Result<Vec<_>, _>>()?,
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
                .par_iter()
                .map(|c| T::try_from(*c))
                .collect::<Result<Vec<_>, _>>()?,
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
                    let member_or_code = member_or_code?;

                    if let Ok(code) = member_or_code.extract::<char>() {
                        return T::try_from(code);
                    };

                    if let Ok(member) = member_or_code.extract::<T>() {
                        return Ok(member);
                    }

                    Err(pyo3::exceptions::PyValueError::new_err(
                        "invalid sequence member",
                    ))
                })
                .collect::<PyResult<Vec<_>>>()?,
        ))
    }
}
