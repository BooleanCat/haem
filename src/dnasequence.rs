use crate::dnabase::AddInput;
use crate::dnabase::DNABase;
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyIterator;
use std::fmt;
use std::ops;
use std::vec;

#[derive(FromPyObject)]
pub enum DNASequenceInput<'a> {
    Str(&'a str),
    Iter(&'a PyIterator),
    Seq(Vec<DNABase>),
    SeqStr(Vec<char>),
}

impl TryFrom<DNASequenceInput<'_>> for Vec<DNABase> {
    type Error = PyErr;

    fn try_from(bases: DNASequenceInput) -> PyResult<Self> {
        match bases {
            DNASequenceInput::Str(bases) => bases
                .chars()
                .map(DNABase::try_from)
                .collect::<PyResult<Vec<_>>>(),
            DNASequenceInput::Iter(_bases) => Err(
                pyo3::exceptions::PyNotImplementedError::new_err("not implemented"),
            ),
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

    fn __add__(&self, other: AddInput) -> Self {
        match other {
            AddInput::Base(base) => self + &base,
            AddInput::Seq(seq) => self + &seq,
        }
    }

    fn __len__(&self) -> usize {
        self.bases.len()
    }

    fn __getitem__(&self, _index: isize) -> PyResult<Self> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __iter__(this: PyRef<'_, Self>) -> PyResult<Py<DNASequenceIterator>> {
        Py::new(
            this.py(),
            DNASequenceIterator {
                sequence: this.bases.clone().into_iter(),
            },
        )
    }

    fn __contains__(&self, _base: DNABase) -> PyResult<bool> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
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

#[pyclass]
struct DNASequenceIterator {
    sequence: vec::IntoIter<DNABase>,
}

#[pymethods]
impl DNASequenceIterator {
    fn __iter__(this: PyRef<'_, Self>) -> PyRef<'_, Self> {
        this
    }

    fn __next__(mut this: PyRefMut<'_, Self>) -> Option<DNABase> {
        this.sequence.next()
    }
}
