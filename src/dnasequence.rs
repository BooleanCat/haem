use crate::dnabase::DNABase;
use crate::dnabase::DNABaseOrSequence;
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyIterator, PySlice};
use std::fmt;
use std::ops;

#[derive(FromPyObject)]
pub enum IntOrSlice<'a> {
    Int(isize),
    Slice(&'a PySlice),
}

#[derive(FromPyObject)]
pub enum DNASequenceInput<'a> {
    Str(&'a str),
    Iter(&'a PyIterator),
    Seq(Vec<DNABase>),
    SeqStr(Vec<char>),
}

#[derive(FromPyObject)]
pub enum CharOrDNABase {
    Char(char),
    Base(DNABase),
}

impl TryFrom<DNASequenceInput<'_>> for Vec<DNABase> {
    type Error = PyErr;

    fn try_from(bases: DNASequenceInput) -> PyResult<Self> {
        match bases {
            DNASequenceInput::Str(bases) => bases
                .chars()
                .map(DNABase::try_from)
                .collect::<PyResult<Vec<_>>>(),
            DNASequenceInput::Iter(bases) => bases
                .iter()?
                .map(|base| match base?.extract::<CharOrDNABase>() {
                    Ok(CharOrDNABase::Char(code)) => code.try_into(),
                    Ok(CharOrDNABase::Base(base)) => Ok(base),
                    Err(e) => Err(e),
                })
                .collect(),
            DNASequenceInput::Seq(bases) => Ok(bases),
            DNASequenceInput::SeqStr(codes) => codes
                .iter()
                .map(|code| DNABase::try_from(*code))
                .collect::<PyResult<Vec<_>>>(),
        }
    }
}

#[derive(FromPyObject)]
pub enum DNABaseInput {
    BaseStr(char),
    Base(DNABase),
}

impl TryFrom<DNABaseInput> for DNABase {
    type Error = PyErr;

    fn try_from(base: DNABaseInput) -> PyResult<DNABase> {
        match base {
            DNABaseInput::BaseStr(base) => base.try_into(),
            DNABaseInput::Base(base) => Ok(base),
        }
    }
}

#[pyclass(frozen)]
#[derive(PartialEq, Clone)]
pub struct DNASequence {
    pub bases: Vec<DNABase>,
}

#[pymethods]
impl DNASequence {
    #[new]
    #[pyo3(signature = (bases = DNASequenceInput::Str("")))]
    pub fn __new__(bases: DNASequenceInput) -> PyResult<Self> {
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

    fn transcribe(&self) -> PyResult<()> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn count(&self, base: DNABaseInput) -> PyResult<usize> {
        let base = base.try_into()?;
        Ok(self.bases.iter().filter(|&b| *b == base).count())
    }

    fn __invert__(&self) -> Self {
        self.get_complement()
    }

    fn __repr__(&self) -> String {
        if self.bases.is_empty() {
            "<DNASequence>".to_string()
        } else {
            format!(
                "<DNASequence: {}>",
                self.bases.iter().map(char::from).collect::<String>()
            )
        }
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }

    fn __richcmp__(&self, _other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.bases == _other.bases).into_py(py),
            CompareOp::Ne => (self.bases != _other.bases).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __bool__(&self) -> bool {
        !self.bases.is_empty()
    }

    fn __add__(&self, other: DNABaseOrSequence) -> Self {
        match other {
            DNABaseOrSequence::Base(base) => self + &base,
            DNABaseOrSequence::Seq(seq) => self + &seq,
        }
    }

    fn __len__(&self) -> usize {
        self.bases.len()
    }

    fn __getitem__(&self, py: Python, index_or_slice: IntOrSlice) -> PyResult<Py<PyAny>> {
        match index_or_slice {
            IntOrSlice::Int(index) => {
                let index: usize = if index < 0 {
                    self.bases.len() - index.unsigned_abs()
                } else {
                    index as usize
                };

                if index >= self.bases.len() {
                    return Err(pyo3::exceptions::PyIndexError::new_err(
                        "DNASequence index out of range",
                    ));
                }

                Ok(self.bases[index].into_py(py))
            }
            IntOrSlice::Slice(slice) => {
                let indices = slice.indices(self.bases.len() as i64)?;

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

    fn __contains__(&self, base_or_seq: DNABaseOrSequence) -> PyResult<bool> {
        match base_or_seq {
            DNABaseOrSequence::Base(base) => Ok(self.bases.contains(&base)),
            DNABaseOrSequence::Seq(seq) if seq.bases.is_empty() => Ok(true),
            DNABaseOrSequence::Seq(seq) => {
                Ok(self.bases.windows(seq.bases.len()).any(|w| w == seq.bases))
            }
        }
    }
}

impl fmt::Display for DNASequence {
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

impl ops::Add<&DNABase> for &DNASequence {
    type Output = DNASequence;

    fn add(self, rhs: &DNABase) -> DNASequence {
        DNASequence {
            bases: [self.bases.as_slice(), &[*rhs]].concat(),
        }
    }
}

impl ops::Add<&DNASequence> for &DNASequence {
    type Output = DNASequence;

    fn add(self, rhs: &DNASequence) -> DNASequence {
        DNASequence {
            bases: [self.bases.as_slice(), rhs.bases.as_slice()].concat(),
        }
    }
}
