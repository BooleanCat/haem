use crate::member::MemberOrMembers;
use crate::utils::{IntOrSlice, SequenceLikeInput, Wrapper};
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::pyclass::PyClass;
use pyo3::types::PyIterator;
use rayon::prelude::*;
use std::os::raw::c_long;

pub trait Sequence<T>
where
    T: PartialEq + Clone + Sync,
    for<'a> char: From<&'a T>,
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
                    .map(char::from)
                    .collect::<String>(),
            )
        }
    }

    fn str(&self) -> String {
        if self.len() < 21 {
            self.members().iter().map(char::from).collect::<String>()
        } else {
            format!(
                "{}...{}",
                self.members()[0..10]
                    .iter()
                    .map(char::from)
                    .collect::<String>(),
                self.members()[self.len() - 10..self.len()]
                    .iter()
                    .map(char::from)
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

    fn add(&self, other: SequenceLikeInput<T>, swap: bool) -> PyResult<Vec<T>>
    where
        T: TryFrom<char, Error = PyErr> + Send + Clone,
    {
        Ok(match other {
            SequenceLikeInput::Member(member) => {
                let mut members = Vec::with_capacity(self.len() + 1);

                match swap {
                    true => {
                        members.push(member);
                        members.par_extend(self.members().to_vec());
                    }
                    false => {
                        members.par_extend(self.members().to_vec());
                        members.push(member);
                    }
                }

                members
            }
            SequenceLikeInput::Members(others) => {
                let mut members = Vec::with_capacity(self.len() + others.len());

                match swap {
                    true => {
                        members.par_extend(others);
                        members.par_extend(self.members().to_vec());
                    }
                    false => {
                        members.par_extend(self.members().to_vec());
                        members.par_extend(others);
                    }
                }

                members
            }
            SequenceLikeInput::Codes(codes) => {
                let mut members = Vec::with_capacity(self.len() + codes.len());

                match swap {
                    true => {
                        members.par_extend(Wrapper::try_from(codes)?.into_inner());
                        members.par_extend(self.members().to_vec());
                    }
                    false => {
                        members.par_extend(self.members().to_vec());
                        members.par_extend(Wrapper::try_from(codes)?.into_inner());
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

    fn count(&self, bases: SequenceLikeInput<T>, overlap: bool) -> PyResult<usize>
    where
        T: TryFrom<char, Error = PyErr> + Clone + Copy,
        for<'a> Wrapper<Vec<T>>: TryFrom<SequenceLikeInput<'a, T>, Error = PyErr>,
    {
        let sequence = Wrapper::try_from(bases)?.into_inner();
        let sequence_len = sequence.len();

        if sequence_len == 0 {
            return Ok(0);
        }

        let mut count = 0;

        let mut iter = self.members().windows(sequence.len());
        while let Some(item) = iter.next() {
            if item == sequence {
                count += 1;
                if !overlap {
                    for _ in 0..sequence_len - 1 {
                        iter.next();
                    }
                }
            }
        }

        Ok(count)
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
