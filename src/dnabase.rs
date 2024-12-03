use crate::dnasequence::{DNASequence, DNASequenceInput};
use crate::member::Member;
use crate::rnabase::RNABase;
use pyo3::prelude::*;
use std::fmt;

#[pyclass(eq, eq_int, frozen, rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Clone, Copy, PartialEq)]
pub enum DNABase {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    AdenineCytosine,
    AdenineGuanine,
    AdenineThymine,
    CytosineGuanine,
    CytosineThymine,
    GuanineThymine,
    AdenineCytosineGuanine,
    AdenineCytosineThymine,
    AdenineGuanineThymine,
    CytosineGuanineThymine,
    Any,
    Gap,
}

impl Member for DNABase {}

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

    pub fn transcribe(&self) -> RNABase {
        self.into()
    }

    fn __bool__(&self) -> bool {
        *self != Self::Gap
    }

    fn __invert__(&self) -> Self {
        self.get_complement()
    }

    fn __add__(&self, other: DNASequenceInput) -> PyResult<DNASequence> {
        Ok(self.add(&DNASequence::try_from(other)?, false).into())
    }

    fn __radd__(&self, other: DNASequenceInput) -> PyResult<DNASequence> {
        Ok(self.add(&DNASequence::try_from(other)?, true).into())
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

impl From<&RNABase> for DNABase {
    fn from(base: &RNABase) -> Self {
        match base {
            RNABase::Adenine => Self::Adenine,
            RNABase::Cytosine => Self::Cytosine,
            RNABase::Guanine => Self::Guanine,
            RNABase::Uracil => Self::Thymine,
            RNABase::AdenineCytosine => Self::AdenineCytosine,
            RNABase::AdenineGuanine => Self::AdenineGuanine,
            RNABase::AdenineUracil => Self::AdenineThymine,
            RNABase::CytosineGuanine => Self::CytosineGuanine,
            RNABase::CytosineUracil => Self::CytosineThymine,
            RNABase::GuanineUracil => Self::GuanineThymine,
            RNABase::AdenineCytosineGuanine => Self::AdenineCytosineGuanine,
            RNABase::AdenineCytosineUracil => Self::AdenineCytosineThymine,
            RNABase::AdenineGuanineUracil => Self::AdenineGuanineThymine,
            RNABase::CytosineGuanineUracil => Self::CytosineGuanineThymine,
            RNABase::Any => Self::Any,
            RNABase::Gap => Self::Gap,
        }
    }
}
