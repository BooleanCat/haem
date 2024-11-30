use crate::dnabase::DNABase;
use crate::member::Member;
use crate::rnasequence::{RNASequence, RNASequenceInput};
use crate::sequence::Sequence;
use pyo3::prelude::*;
use std::fmt;

#[pyclass(eq, eq_int, frozen, rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Clone, Copy, PartialEq)]
pub enum RNABase {
    Adenine,
    Cytosine,
    Guanine,
    Uracil,
    AdenineCytosine,
    AdenineGuanine,
    AdenineUracil,
    CytosineGuanine,
    CytosineUracil,
    GuanineUracil,
    AdenineCytosineGuanine,
    AdenineCytosineUracil,
    AdenineGuanineUracil,
    CytosineGuanineUracil,
    Any,
    Gap,
}

impl Member for RNABase {}

#[pymethods]
impl RNABase {
    #[new]
    pub fn __new__(code: char) -> PyResult<Self> {
        Self::try_from(code)
    }

    #[getter]
    fn get_code(&self) -> char {
        self.into()
    }

    #[getter]
    pub fn get_complement(&self) -> Self {
        match self {
            Self::Adenine => Self::Uracil,
            Self::Cytosine => Self::Guanine,
            Self::Guanine => Self::Cytosine,
            Self::Uracil => Self::Adenine,
            Self::AdenineCytosine => Self::GuanineUracil,
            Self::AdenineGuanine => Self::CytosineUracil,
            Self::AdenineUracil => Self::AdenineUracil,
            Self::CytosineGuanine => Self::CytosineGuanine,
            Self::CytosineUracil => Self::AdenineGuanine,
            Self::GuanineUracil => Self::AdenineCytosine,
            Self::AdenineCytosineGuanine => Self::CytosineGuanineUracil,
            Self::AdenineCytosineUracil => Self::AdenineGuanineUracil,
            Self::AdenineGuanineUracil => Self::AdenineCytosineUracil,
            Self::CytosineGuanineUracil => Self::AdenineCytosineGuanine,
            Self::Any => Self::Any,
            Self::Gap => Self::Gap,
        }
    }

    fn retro_transcribe(&self) -> DNABase {
        self.into()
    }

    fn __bool__(&self) -> bool {
        *self != Self::Gap
    }

    fn __invert__(&self) -> Self {
        self.get_complement()
    }

    fn __add__(&self, other: RNASequenceInput) -> PyResult<RNASequence> {
        Ok(RNASequence {
            sequence: self.add(RNASequence::try_from(other)?.members(), false),
        })
    }

    fn __radd__(&self, other: RNASequenceInput) -> PyResult<RNASequence> {
        Ok(RNASequence {
            sequence: self.add(RNASequence::try_from(other)?.members(), true),
        })
    }

    fn __str__(&self) -> String {
        self.to_string()
    }
}

impl From<&RNABase> for char {
    fn from(base: &RNABase) -> Self {
        match base {
            RNABase::Adenine => 'A',
            RNABase::Cytosine => 'C',
            RNABase::Guanine => 'G',
            RNABase::Uracil => 'U',
            RNABase::AdenineCytosine => 'M',
            RNABase::AdenineGuanine => 'R',
            RNABase::AdenineUracil => 'W',
            RNABase::CytosineGuanine => 'S',
            RNABase::CytosineUracil => 'Y',
            RNABase::GuanineUracil => 'K',
            RNABase::AdenineCytosineGuanine => 'V',
            RNABase::AdenineCytosineUracil => 'H',
            RNABase::AdenineGuanineUracil => 'D',
            RNABase::CytosineGuanineUracil => 'B',
            RNABase::Any => 'N',
            RNABase::Gap => '-',
        }
    }
}

impl TryFrom<char> for RNABase {
    type Error = PyErr;

    fn try_from(code: char) -> PyResult<RNABase> {
        Ok(match code {
            'A' => RNABase::Adenine,
            'C' => RNABase::Cytosine,
            'G' => RNABase::Guanine,
            'U' => RNABase::Uracil,
            'M' => RNABase::AdenineCytosine,
            'R' => RNABase::AdenineGuanine,
            'W' => RNABase::AdenineUracil,
            'S' => RNABase::CytosineGuanine,
            'Y' => RNABase::CytosineUracil,
            'K' => RNABase::GuanineUracil,
            'V' => RNABase::AdenineCytosineGuanine,
            'H' => RNABase::AdenineCytosineUracil,
            'D' => RNABase::AdenineGuanineUracil,
            'B' => RNABase::CytosineGuanineUracil,
            'N' => RNABase::Any,
            '.' | '-' => RNABase::Gap,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "invalid IUPAC RNA code \"{}\"",
                    code
                )))
            }
        })
    }
}

impl fmt::Display for RNABase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Adenine => "adenine",
                Self::Cytosine => "cytosine",
                Self::Guanine => "guanine",
                Self::Uracil => "uracil",
                Self::AdenineCytosine => "adenine/cytosine",
                Self::AdenineGuanine => "adenine/guanine",
                Self::AdenineUracil => "adenine/uracil",
                Self::CytosineGuanine => "cytosine/guanine",
                Self::CytosineUracil => "cytosine/uracil",
                Self::GuanineUracil => "guanine/uracil",
                Self::AdenineCytosineGuanine => "adenine/cytosine/guanine",
                Self::AdenineCytosineUracil => "adenine/cytosine/uracil",
                Self::AdenineGuanineUracil => "adenine/guanine/uracil",
                Self::CytosineGuanineUracil => "cytosine/guanine/uracil",
                Self::Any => "any",
                Self::Gap => "gap",
            }
        )
    }
}

impl From<&DNABase> for RNABase {
    fn from(base: &DNABase) -> Self {
        match base {
            DNABase::Adenine => Self::Adenine,
            DNABase::Cytosine => Self::Cytosine,
            DNABase::Guanine => Self::Guanine,
            DNABase::Thymine => Self::Uracil,
            DNABase::AdenineCytosine => Self::AdenineCytosine,
            DNABase::AdenineGuanine => Self::AdenineGuanine,
            DNABase::AdenineThymine => Self::AdenineUracil,
            DNABase::CytosineGuanine => Self::CytosineGuanine,
            DNABase::CytosineThymine => Self::CytosineUracil,
            DNABase::GuanineThymine => Self::GuanineUracil,
            DNABase::AdenineCytosineGuanine => Self::AdenineCytosineGuanine,
            DNABase::AdenineCytosineThymine => Self::AdenineCytosineUracil,
            DNABase::AdenineGuanineThymine => Self::AdenineGuanineUracil,
            DNABase::CytosineGuanineThymine => Self::CytosineGuanineUracil,
            DNABase::Any => Self::Any,
            DNABase::Gap => Self::Gap,
        }
    }
}
