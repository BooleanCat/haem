use crate::member::{MemberOrCode, MemberOrMembers};
use crate::utils::{AddInput, IntOrSlice, Wrapper};
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::pyclass::PyClass;
use pyo3::types::PyIterator;
use rayon::prelude::*;
use std::os::raw::c_long;

pub trait Sequence<T>
where
    for<'a> &'a Vec<T>: PartialEq,
    T: PartialEq + Clone + Sync,
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
                self.members()
                    .par_iter()
                    .map(|m| m.into())
                    .collect::<String>(),
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

    fn contains(&self, base_or_seq: MemberOrMembers<T>) -> PyResult<bool> {
        match base_or_seq {
            MemberOrMembers::Member(member) => Ok(self.members().contains(&member)),
            MemberOrMembers::Sequence(sequence) if sequence.is_empty() => Ok(true),
            MemberOrMembers::Sequence(sequence) => Ok(self
                .members()
                .par_windows(sequence.len())
                .any(|w| w == sequence)),
        }
    }

    fn add(&self, other: AddInput<T>, swap: bool) -> PyResult<Vec<T>>
    where
        T: TryFrom<char, Error = PyErr> + Send + Clone,
    {
        Ok(match other {
            AddInput::Member(member) => {
                let mut members = Vec::with_capacity(self.len() + 1);

                match swap {
                    true => {
                        members.push(member);
                        members.extend(self.members().to_vec());
                    }
                    false => {
                        members.extend(self.members().to_vec());
                        members.push(member);
                    }
                }

                members
            }
            AddInput::Members(others) => {
                let mut members = Vec::with_capacity(self.len() + others.len());

                match swap {
                    true => {
                        members.extend(others);
                        members.extend(self.members().to_vec());
                    }
                    false => {
                        members.extend(self.members().to_vec());
                        members.extend(others);
                    }
                }

                members
            }
            AddInput::Codes(codes) => {
                let mut members = Vec::with_capacity(self.len() + codes.len());

                match swap {
                    true => {
                        members.extend(Wrapper::try_from(codes)?.into_inner());
                        members.extend(self.members().to_vec());
                    }
                    false => {
                        members.extend(self.members().to_vec());
                        members.extend(Wrapper::try_from(codes)?.into_inner());
                    }
                }

                members
            }
        })
    }

    fn getitem(&self, index_or_slice: IntOrSlice) -> PyResult<MemberOrMembers<T>> {
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

                Ok(MemberOrMembers::Member(self.members()[index].clone()))
            }
            IntOrSlice::Slice(slice) => {
                let indices = slice.indices(self.len() as c_long)?;

                Ok(MemberOrMembers::Sequence(match indices.step {
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
        let base = Wrapper::try_from(base)?.into_inner();
        Ok(self.members().par_iter().filter(|&b| *b == base).count())
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
    T: TryFrom<char, Error = PyErr> + Send + PyClass + Clone,
{
    type Error = PyErr;

    fn try_from(bases: SequenceInput<'a, T>) -> PyResult<Self> {
        Ok(match bases {
            SequenceInput::Str(bases) => Wrapper::try_from(bases)?.into_inner(),
            SequenceInput::Iter(bases) => Wrapper::try_from(bases)?.into_inner(),
            SequenceInput::Seq(bases) => bases,
            SequenceInput::SeqStr(codes) => Wrapper::try_from(codes)?.into_inner(),
        })
    }
}
