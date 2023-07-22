use crate::rnabase::{RNABase, RNABaseOrSequence};
use crate::utils::IntOrSlice;
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyIterator;
use std::os::raw::c_long;
use std::{fmt, ops};

#[derive(FromPyObject)]
pub enum RNASequenceInput<'a> {
    Str(&'a str),
    Iter(&'a PyIterator),
    Seq(Vec<RNABase>),
    SeqStr(Vec<char>),
}

#[derive(FromPyObject)]
pub enum CharOrRNABase {
    Char(char),
    Base(RNABase),
}

#[derive(FromPyObject)]
pub enum RNABaseInput {
    BaseStr(char),
    Base(RNABase),
}

impl TryFrom<RNABaseInput> for RNABase {
    type Error = PyErr;

    fn try_from(base: RNABaseInput) -> PyResult<RNABase> {
        match base {
            RNABaseInput::BaseStr(base) => base.try_into(),
            RNABaseInput::Base(base) => Ok(base),
        }
    }
}

impl TryFrom<RNASequenceInput<'_>> for Vec<RNABase> {
    type Error = PyErr;

    fn try_from(bases: RNASequenceInput) -> PyResult<Self> {
        match bases {
            RNASequenceInput::Str(bases) => bases
                .chars()
                .map(RNABase::try_from)
                .collect::<PyResult<Vec<_>>>(),
            RNASequenceInput::Iter(bases) => bases
                .iter()?
                .map(|base| match base?.extract::<CharOrRNABase>() {
                    Ok(CharOrRNABase::Char(code)) => code.try_into(),
                    Ok(CharOrRNABase::Base(base)) => Ok(base),
                    Err(e) => Err(e),
                })
                .collect(),
            RNASequenceInput::Seq(bases) => Ok(bases),
            RNASequenceInput::SeqStr(codes) => codes
                .iter()
                .map(|code| RNABase::try_from(*code))
                .collect::<PyResult<Vec<_>>>(),
        }
    }
}

#[pyclass(frozen)]
#[derive(PartialEq, Clone)]
pub struct RNASequence {
    pub bases: Vec<RNABase>,
}

#[pymethods]
impl RNASequence {
    #[new]
    #[pyo3(signature = (bases = RNASequenceInput::Str("")))]
    pub fn __new__(bases: RNASequenceInput) -> PyResult<Self> {
        Ok(Self {
            bases: bases.try_into()?,
        })
    }

    #[getter]
    fn get_complement(&self) -> Self {
        Self {
            bases: self.bases.iter().map(|b| b.get_complement()).collect(),
        }
    }

    fn count(&self, base: RNABaseInput) -> PyResult<usize> {
        let base = base.try_into()?;
        Ok(self.bases.iter().filter(|&b| *b == base).count())
    }

    fn __invert__(&self) -> Self {
        self.get_complement()
    }

    fn __repr__(&self) -> String {
        if self.bases.is_empty() {
            "<RNASequence>".to_string()
        } else {
            format!(
                "<RNASequence: {}>",
                self.bases.iter().map(char::from).collect::<String>()
            )
        }
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.bases == other.bases).into_py(py),
            CompareOp::Ne => (self.bases != other.bases).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __bool__(&self) -> bool {
        !self.bases.is_empty()
    }

    fn __add__(&self, other: RNABaseOrSequence) -> Self {
        match other {
            RNABaseOrSequence::Base(base) => self + &base,
            RNABaseOrSequence::Seq(seq) => self + &seq,
        }
    }

    fn __len__(&self) -> usize {
        self.bases.len()
    }

    fn __getitem__(&self, py: Python, index_or_slice: IntOrSlice) -> PyResult<Py<PyAny>> {
        match index_or_slice {
            IntOrSlice::Int(index) => {
                let index = if index < 0 {
                    self.bases.len() - index.unsigned_abs()
                } else {
                    index as usize
                };

                if index >= self.bases.len() {
                    return Err(pyo3::exceptions::PyIndexError::new_err(
                        "RNASequence index out of range",
                    ));
                }

                Ok(self.bases[index].into_py(py))
            }
            IntOrSlice::Slice(slice) => {
                let indices = slice.indices(self.bases.len() as c_long)?;

                Ok(Self {
                    bases: match indices.step {
                        s if s < 0 => (indices.stop + 1..indices.start + 1)
                            .rev()
                            .step_by(indices.step.unsigned_abs())
                            .map(|i| self.bases[i as usize])
                            .collect(),
                        _ => (indices.start..indices.stop)
                            .step_by(indices.step as usize)
                            .map(|i| self.bases[i as usize])
                            .collect(),
                    },
                }
                .into_py(py))
            }
        }
    }

    fn __contains__(&self, base_or_seq: RNABaseOrSequence) -> PyResult<bool> {
        match base_or_seq {
            RNABaseOrSequence::Base(base) => Ok(self.bases.contains(&base)),
            RNABaseOrSequence::Seq(seq) if seq.bases.is_empty() => Ok(true),
            RNABaseOrSequence::Seq(seq) => {
                Ok(self.bases.windows(seq.bases.len()).any(|w| w == seq.bases))
            }
        }
    }
}

impl fmt::Display for RNASequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.bases.len() < 21 {
            write!(
                f,
                "{}",
                self.bases.iter().map(char::from).collect::<String>()
            )
        } else {
            write!(
                f,
                "{}...{}",
                self.bases[0..10].iter().map(char::from).collect::<String>(),
                self.bases[self.bases.len() - 10..self.bases.len()]
                    .iter()
                    .map(char::from)
                    .collect::<String>()
            )
        }
    }
}

impl ops::Add<&RNABase> for &RNASequence {
    type Output = RNASequence;

    fn add(self, rhs: &RNABase) -> RNASequence {
        RNASequence {
            bases: [self.bases.as_slice(), &[*rhs]].concat(),
        }
    }
}

impl ops::Add<&RNASequence> for &RNASequence {
    type Output = RNASequence;

    fn add(self, rhs: &RNASequence) -> RNASequence {
        RNASequence {
            bases: [self.bases.as_slice(), rhs.bases.as_slice()].concat(),
        }
    }
}
