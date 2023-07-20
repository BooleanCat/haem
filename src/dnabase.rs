use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;

use crate::rnabase::RNABase;

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
    fn __new__(code: char) -> PyResult<Self> {
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

    #[getter]
    pub fn get_code(&self) -> char {
        match self {
            Self::Adenine => 'A',
            Self::Cytosine => 'C',
            Self::Guanine => 'G',
            Self::Thymine => 'T',
            Self::AdenineCytosine => 'M',
            Self::AdenineGuanine => 'R',
            Self::AdenineThymine => 'W',
            Self::CytosineGuanine => 'S',
            Self::CytosineThymine => 'Y',
            Self::GuanineThymine => 'K',
            Self::AdenineCytosineGuanine => 'V',
            Self::AdenineCytosineThymine => 'H',
            Self::AdenineGuanineThymine => 'D',
            Self::CytosineGuanineThymine => 'B',
            Self::Any => 'N',
            Self::Gap => '-',
        }
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
        match self {
            Self::Adenine => RNABase::Adenine,
            Self::Cytosine => RNABase::Cytosine,
            Self::Guanine => RNABase::Guanine,
            Self::Thymine => RNABase::Uracil,
            Self::AdenineCytosine => RNABase::AdenineCytosine,
            Self::AdenineGuanine => RNABase::AdenineGuanine,
            Self::AdenineThymine => RNABase::AdenineUracil,
            Self::CytosineGuanine => RNABase::CytosineGuanine,
            Self::CytosineThymine => RNABase::CytosineUracil,
            Self::GuanineThymine => RNABase::GuanineUracil,
            Self::AdenineCytosineGuanine => RNABase::AdenineCytosineGuanine,
            Self::AdenineCytosineThymine => RNABase::AdenineCytosineUracil,
            Self::AdenineGuanineThymine => RNABase::AdenineGuanineUracil,
            Self::CytosineGuanineThymine => RNABase::CytosineGuanineUracil,
            Self::Any => RNABase::Any,
            Self::Gap => RNABase::Gap,
        }
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

    fn __add__(&self, _other: &Self) -> PyResult<()> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __str__(&self) -> &'static str {
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
    }
}
