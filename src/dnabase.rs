use crate::rnabase::RNABase;
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use std::fmt;
use std::ops;

use crate::dnasequence::DNASequence;

#[derive(FromPyObject)]
pub enum DNABaseOrSequence {
    Base(DNABase),
    Seq(DNASequence),
}

#[pyclass(frozen)]
#[derive(Clone, Copy, PartialEq)]
pub enum DNABase {
    #[pyo3(name = "ADENINE")]
    Adenine,

    #[pyo3(name = "CYTOSINE")]
    Cytosine,

    #[pyo3(name = "GUANINE")]
    Guanine,

    #[pyo3(name = "THYMINE")]
    Thymine,

    #[pyo3(name = "ADENINE_CYTOSINE")]
    AdenineCytosine,

    #[pyo3(name = "ADENINE_GUANINE")]
    AdenineGuanine,

    #[pyo3(name = "ADENINE_THYMINE")]
    AdenineThymine,

    #[pyo3(name = "CYTOSINE_GUANINE")]
    CytosineGuanine,

    #[pyo3(name = "CYTOSINE_THYMINE")]
    CytosineThymine,

    #[pyo3(name = "GUANINE_THYMINE")]
    GuanineThymine,

    #[pyo3(name = "ADENINE_CYTOSINE_GUANINE")]
    AdenineCytosineGuanine,

    #[pyo3(name = "ADENINE_CYTOSINE_THYMINE")]
    AdenineCytosineThymine,

    #[pyo3(name = "ADENINE_GUANINE_THYMINE")]
    AdenineGuanineThymine,

    #[pyo3(name = "CYTOSINE_GUANINE_THYMINE")]
    CytosineGuanineThymine,

    #[pyo3(name = "ANY")]
    Any,

    #[pyo3(name = "GAP")]
    Gap,
}

#[pymethods]
impl DNABase {
    #[new]
    pub fn __new__(code: char) -> PyResult<Self> {
        Self::try_from(code)
    }

    #[getter]
    pub fn get_code(&self) -> char {
        self.into()
    }

    #[getter]
    pub fn get_complement(&self) -> Self {
        match self {
            Self::Adenine => Self::Thymine,
            Self::Cytosine => Self::Guanine,
            Self::Guanine => Self::Cytosine,
            Self::Thymine => Self::Adenine,
            Self::AdenineCytosine => Self::GuanineThymine,
            Self::AdenineGuanine => Self::CytosineThymine,
            Self::AdenineThymine => Self::AdenineThymine,
            Self::CytosineGuanine => Self::CytosineGuanine,
            Self::CytosineThymine => Self::AdenineGuanine,
            Self::GuanineThymine => Self::AdenineCytosine,
            Self::AdenineCytosineGuanine => Self::CytosineGuanineThymine,
            Self::AdenineCytosineThymine => Self::AdenineGuanineThymine,
            Self::AdenineGuanineThymine => Self::AdenineCytosineThymine,
            Self::CytosineGuanineThymine => Self::AdenineCytosineGuanine,
            Self::Any => Self::Any,
            Self::Gap => Self::Gap,
        }
    }

    fn transcribe(&self) -> RNABase {
        self.into()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self == other).into_py(py),
            CompareOp::Ne => (self != other).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __bool__(&self) -> bool {
        *self != Self::Gap
    }

    fn __invert__(&self) -> Self {
        self.get_complement()
    }

    fn __add__(&self, other: DNABaseOrSequence) -> DNASequence {
        match other {
            DNABaseOrSequence::Base(base) => self + &base,
            DNABaseOrSequence::Seq(seq) => self + &seq,
        }
    }

    fn __str__(&self) -> String {
        self.to_string()
    }
}

impl From<&DNABase> for char {
    fn from(base: &DNABase) -> Self {
        match base {
            DNABase::Adenine => 'A',
            DNABase::Cytosine => 'C',
            DNABase::Guanine => 'G',
            DNABase::Thymine => 'T',
            DNABase::AdenineCytosine => 'M',
            DNABase::AdenineGuanine => 'R',
            DNABase::AdenineThymine => 'W',
            DNABase::CytosineGuanine => 'S',
            DNABase::CytosineThymine => 'Y',
            DNABase::GuanineThymine => 'K',
            DNABase::AdenineCytosineGuanine => 'V',
            DNABase::AdenineCytosineThymine => 'H',
            DNABase::AdenineGuanineThymine => 'D',
            DNABase::CytosineGuanineThymine => 'B',
            DNABase::Any => 'N',
            DNABase::Gap => '-',
        }
    }
}

impl TryFrom<char> for DNABase {
    type Error = PyErr;

    fn try_from(code: char) -> PyResult<DNABase> {
        Ok(match code {
            'A' => Self::Adenine,
            'C' => Self::Cytosine,
            'G' => Self::Guanine,
            'T' => Self::Thymine,
            'M' => Self::AdenineCytosine,
            'R' => Self::AdenineGuanine,
            'W' => Self::AdenineThymine,
            'S' => Self::CytosineGuanine,
            'Y' => Self::CytosineThymine,
            'K' => Self::GuanineThymine,
            'V' => Self::AdenineCytosineGuanine,
            'H' => Self::AdenineCytosineThymine,
            'D' => Self::AdenineGuanineThymine,
            'B' => Self::CytosineGuanineThymine,
            'N' => Self::Any,
            '.' | '-' => Self::Gap,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "invalid IUPAC DNA code \"{}\"",
                    code
                )))
            }
        })
    }
}

impl fmt::Display for DNABase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Adenine => "adenine",
                Self::Cytosine => "cytosine",
                Self::Guanine => "guanine",
                Self::Thymine => "thymine",
                Self::AdenineCytosine => "adenine/cytosine",
                Self::AdenineGuanine => "adenine/guanine",
                Self::AdenineThymine => "adenine/thymine",
                Self::CytosineGuanine => "cytosine/guanine",
                Self::CytosineThymine => "cytosine/thymine",
                Self::GuanineThymine => "guanine/thymine",
                Self::AdenineCytosineGuanine => "adenine/cytosine/guanine",
                Self::AdenineCytosineThymine => "adenine/cytosine/thymine",
                Self::AdenineGuanineThymine => "adenine/guanine/thymine",
                Self::CytosineGuanineThymine => "cytosine/guanine/thymine",
                Self::Any => "any",
                Self::Gap => "gap",
            }
        )
    }
}

impl ops::Add<&DNABase> for &DNABase {
    type Output = DNASequence;

    fn add(self, rhs: &DNABase) -> DNASequence {
        DNASequence {
            bases: vec![*self, *rhs],
        }
    }
}

impl ops::Add<&DNASequence> for &DNABase {
    type Output = DNASequence;

    fn add(self, rhs: &DNASequence) -> DNASequence {
        let mut bases = Vec::with_capacity(rhs.bases.len() + 1);
        bases.push(*self);
        bases.extend_from_slice(&rhs.bases);
        DNASequence { bases }
    }
}
