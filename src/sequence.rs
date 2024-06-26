use crate::member::MemberOrMembers;
use crate::utils::{IntOrSlice, SequenceLikeInput, Wrapper};
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
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

    fn bool(&self) -> bool {
        !self.members().is_empty()
    }

    fn eq(&self, other: &Self) -> bool {
        self.members() == other.members()
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

    fn contains(&self, sequence: SequenceLikeInput<T>) -> PyResult<bool>
    where
        T: TryFrom<char, Error = PyErr> + Clone + Copy,
        for<'a> Wrapper<Vec<T>>: TryFrom<SequenceLikeInput<T>, Error = PyErr>,
    {
        let sequence = Wrapper::try_from(sequence)?.into_inner();

        if sequence.is_empty() {
            return Ok(true);
        }

        Ok(self
            .members()
            .par_windows(sequence.len())
            .any(|w| w == sequence))
    }

    fn add(&self, other: SequenceLikeInput<T>, swap: bool) -> PyResult<Vec<T>>
    where
        T: TryFrom<char, Error = PyErr> + Send + Clone,
    {
        let sequence = Wrapper::try_from(other)?.into_inner();
        let mut members = Vec::with_capacity(self.len() + sequence.len());

        match swap {
            true => {
                members.par_extend(sequence);
                members.par_extend(self.members().to_vec());
            }
            false => {
                members.par_extend(self.members().to_vec());
                members.par_extend(sequence);
            }
        }

        Ok(members)
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
        for<'a> Wrapper<Vec<T>>: TryFrom<SequenceLikeInput<T>, Error = PyErr>,
    {
        let sequence = Wrapper::try_from(bases)?.into_inner();

        Ok(match (sequence.len(), overlap) {
            // Special case, empty sequences always return 0.
            (0, _) => 0,
            // With a sequence lenth of 1 or when overlap is allowed, optimisation is possible.
            (len @ 1, _) | (len, true) => self
                .members()
                .par_windows(len)
                .filter(|w| *w == sequence)
                .count(),
            (len, _) => {
                let mut count = 0;

                let mut iter = self.members().windows(len);
                while let Some(item) = iter.next() {
                    if item == sequence {
                        count += 1;
                        iter.nth(sequence.len() - 2);
                    }
                }

                count
            }
        })
    }

    fn find(&self, bases: SequenceLikeInput<T>) -> PyResult<Option<usize>>
    where
        T: TryFrom<char, Error = PyErr> + Clone + Copy,
        for<'a> Wrapper<Vec<T>>: TryFrom<SequenceLikeInput<T>, Error = PyErr>,
    {
        let sequence = Wrapper::try_from(bases)?.into_inner();

        if self.members().is_empty() || sequence.is_empty() {
            return Ok(None);
        }

        Ok(self
            .members()
            .par_windows(sequence.len())
            .position_first(|w| w == sequence))
    }
}

#[derive(FromPyObject)]
pub enum SequenceInput<'a, T> {
    Str(PyBackedStr),
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
