use crate::member::{MemberOrCode, MemberOrMembers};
use crate::utils::IntOrSlice;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::pyclass::PyClass;
use pyo3::types::PyIterator;
use rayon::prelude::*;

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
        match self.members().is_empty() {
            true => format!("<{}>", self.name()),
            false => format!(
                "<{}: {}>",
                self.name(),
                self.members()
                    .par_iter()
                    .map(char::from)
                    .collect::<String>(),
            ),
        }
    }

    fn str(&self) -> String {
        match self.len() {
            length if length < 21 => self.members().iter().map(char::from).collect::<_>(),
            _ => format!(
                "{}...{}",
                self.members()[0..10]
                    .iter()
                    .map(char::from)
                    .collect::<String>(),
                self.members()[self.len() - 10..self.len()]
                    .iter()
                    .map(char::from)
                    .collect::<String>()
            ),
        }
    }

    fn contains(&self, sequence: &Vec<T>) -> PyResult<bool> {
        Ok(match sequence.is_empty() {
            true => true,
            false => self
                .members()
                .par_windows(sequence.len())
                .any(|w| w == sequence),
        })
    }

    fn add(&self, other: &[T], swap: bool) -> Vec<T>
    where
        T: Send,
        for<'a> &'a [T]: IntoParallelIterator<Item = &'a T>,
    {
        match swap {
            true => other
                .par_iter()
                .chain(self.members().par_iter())
                .cloned()
                .collect(),
            false => self
                .members()
                .par_iter()
                .chain(other.par_iter())
                .cloned()
                .collect(),
        }
    }

    fn getitem(&self, index_or_slice: IntOrSlice) -> PyResult<MemberOrMembers<T>> {
        match index_or_slice {
            IntOrSlice::Int(index) => {
                let index = match index {
                    index if index < 0 => self.len() - index.unsigned_abs(),
                    _ => index as usize,
                };

                match index {
                    index if index < self.len() => {
                        Ok(MemberOrMembers::Member(self.members()[index].clone()))
                    }
                    _ => Err(pyo3::exceptions::PyIndexError::new_err(format!(
                        "{} index out of range",
                        self.name()
                    ))),
                }
            }
            IntOrSlice::Slice(slice) => {
                let indices = slice.indices(self.len() as isize)?;

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

    fn count(&self, sequence: &[T], overlap: bool) -> PyResult<usize> {
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

    fn find(&self, sequence: &[T]) -> PyResult<Option<usize>> {
        Ok(if self.members().is_empty() || sequence.is_empty() {
            None
        } else {
            self.members()
                .par_windows(sequence.len())
                .position_first(|w| w == sequence)
        })
    }
}

#[derive(FromPyObject)]
pub enum SequenceInput<'py, T> {
    Str(PyBackedStr),
    Iter(Bound<'py, PyIterator>),
    Seq(Vec<T>),
    SeqStr(Vec<char>),
    Member(T),
}

impl<'a, T> TryFrom<SequenceInput<'a, T>> for Vec<T>
where
    T: TryFrom<char, Error = PyErr> + Send + PyClass + Clone,
{
    type Error = PyErr;

    fn try_from(bases: SequenceInput<'a, T>) -> PyResult<Self> {
        Ok(match bases {
            SequenceInput::Str(bases) => bases
                .as_parallel_string()
                .par_chars()
                .map(T::try_from)
                .collect::<PyResult<_>>()?,
            SequenceInput::Iter(bases) => bases
                .into_iter()
                .map(
                    |member_or_code| match MemberOrCode::extract_bound(&member_or_code?)? {
                        MemberOrCode::Code(code) => code.try_into(),
                        MemberOrCode::Member(member) => Ok(member),
                    },
                )
                .collect::<PyResult<_>>()?,
            SequenceInput::Seq(bases) => bases,
            SequenceInput::SeqStr(codes) => codes
                .into_par_iter()
                .map(T::try_from)
                .collect::<PyResult<_>>()?,
            SequenceInput::Member(base) => vec![base],
        })
    }
}
