use crate::utils::{IntOrSlice, MemberOrCode, MemberOrSequence, Wrapper};
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyIterator;
use std::os::raw::c_long;

pub trait Sequence<T>
where
    for<'a> &'a Vec<T>: PartialEq,
    T: PartialEq + Clone,
    for<'a> &'a T: Into<char>,
{
    fn members(&self) -> &Vec<T>;
    fn name(&self) -> &str;

    fn richcmp(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.members() == other.members()).into_py(py),
            CompareOp::Ne => (self.members() != other.members()).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn bool(&self) -> bool {
        !self.members().is_empty()
    }

    fn repr(&self) -> String {
        if self.members().is_empty() {
            format!("<{}>", self.name())
        } else {
            format!(
                "<{}: {}>",
                self.name(),
                self.members().iter().map(|m| m.into()).collect::<String>(),
            )
        }
    }

    fn str(&self) -> String {
        if self.len() < 21 {
            self.members().iter().map(|m| m.into()).collect::<String>()
        } else {
            format!(
                "{}...{}",
                self.members()[0..10]
                    .iter()
                    .map(|m| m.into())
                    .collect::<String>(),
                self.members()[self.len() - 10..self.len()]
                    .iter()
                    .map(|m| m.into())
                    .collect::<String>()
            )
        }
    }

    fn contains(&self, base_or_seq: MemberOrSequence<T>) -> PyResult<bool> {
        match base_or_seq {
            MemberOrSequence::Member(member) => Ok(self.members().contains(&member)),
            MemberOrSequence::Sequence(sequence) if sequence.is_empty() => Ok(true),
            MemberOrSequence::Sequence(sequence) => Ok(self
                .members()
                .windows(sequence.len())
                .any(|w| w == sequence)),
        }
    }

    fn add(&self, other: MemberOrSequence<T>) -> Vec<T> {
        match other {
            MemberOrSequence::Member(member) => {
                let mut members = self.members().clone().to_vec();
                members.push(member);
                members
            }
            MemberOrSequence::Sequence(sequence) => {
                let mut members = self.members().clone().to_vec();
                members.extend(sequence);
                members
            }
        }
    }

    fn getitem(&self, index_or_slice: IntOrSlice) -> PyResult<MemberOrSequence<T>> {
        match index_or_slice {
            IntOrSlice::Int(index) => {
                let index = if index < 0 {
                    self.len() - index.unsigned_abs()
                } else {
                    index as usize
                };

                if index >= self.len() {
                    return Err(pyo3::exceptions::PyIndexError::new_err(format!(
                        "{} index out of range",
                        self.name()
                    )));
                }

                Ok(MemberOrSequence::Member(self.members()[index].clone()))
            }
            IntOrSlice::Slice(slice) => {
                let indices = slice.indices(self.len() as c_long)?;

                Ok(MemberOrSequence::Sequence(match indices.step {
                    s if s < 0 => (indices.stop + 1..indices.start + 1)
                        .rev()
                        .step_by(indices.step.unsigned_abs())
                        .map(|i| self.members()[i as usize].clone())
                        .collect(),
                    _ => (indices.start..indices.stop)
                        .step_by(indices.step as usize)
                        .map(|i| self.members()[i as usize].clone())
                        .collect(),
                }))
            }
        }
    }

    fn len(&self) -> usize {
        self.members().len()
    }

    fn count(&self, base: MemberOrCode<T>) -> PyResult<usize>
    where
        T: TryFrom<char, Error = PyErr>,
    {
        let base = Wrapper::<T>::try_from(base)?.peel();
        Ok(self.members().iter().filter(|&b| *b == base).count())
    }
}

#[derive(FromPyObject)]
pub enum SequenceInput<'a, T> {
    Str(&'a str),
    Iter(&'a PyIterator),
    Seq(Vec<T>),
    SeqStr(Vec<char>),
}

impl<'a, T> TryFrom<SequenceInput<'a, T>> for Vec<T>
where
    T: TryFrom<char, Error = PyErr> + FromPyObject<'a>,
{
    type Error = PyErr;

    fn try_from(bases: SequenceInput<'a, T>) -> PyResult<Self> {
        match bases {
            SequenceInput::Str(bases) => {
                bases.chars().map(T::try_from).collect::<PyResult<Vec<_>>>()
            }
            SequenceInput::Iter(bases) => bases
                .iter()?
                .map(|base| Ok(Wrapper::<_>::try_from(base?.extract::<MemberOrCode<_>>()?)?.peel()))
                .collect(),
            SequenceInput::Seq(bases) => Ok(bases),
            SequenceInput::SeqStr(codes) => codes
                .iter()
                .map(|code| T::try_from(*code))
                .collect::<PyResult<Vec<_>>>(),
        }
    }
}
