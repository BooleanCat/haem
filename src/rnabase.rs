use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Clone, Copy, PartialEq)]
pub enum RNABase {
    #[pyo3(name = "ADENINE")]
    Adenine,

    #[pyo3(name = "CYTOSINE")]
    Cytosine,

    #[pyo3(name = "GUANINE")]
    Guanine,

    #[pyo3(name = "URACIL")]
    Uracil,

    #[pyo3(name = "ADENINE_CYTOSINE")]
    AdenineCytosine,

    #[pyo3(name = "ADENINE_GUANINE")]
    AdenineGuanine,

    #[pyo3(name = "ADENINE_URACIL")]
    AdenineUracil,

    #[pyo3(name = "CYTOSINE_GUANINE")]
    CytosineGuanine,

    #[pyo3(name = "CYTOSINE_URACIL")]
    CytosineUracil,

    #[pyo3(name = "GUANINE_URACIL")]
    GuanineUracil,

    #[pyo3(name = "ADENINE_CYTOSINE_GUANINE")]
    AdenineCytosineGuanine,

    #[pyo3(name = "ADENINE_CYTOSINE_URACIL")]
    AdenineCytosineUracil,

    #[pyo3(name = "ADENINE_GUANINE_URACIL")]
    AdenineGuanineUracil,

    #[pyo3(name = "CYTOSINE_GUANINE_URACIL")]
    CytosineGuanineUracil,

    #[pyo3(name = "ANY")]
    Any,

    #[pyo3(name = "GAP")]
    Gap,
}

#[pymethods]
impl RNABase {
    #[new]
    fn __new__(code: char) -> PyResult<Self> {
        Ok(match code {
            'A' => Self::Adenine,
            'C' => Self::Cytosine,
            'G' => Self::Guanine,
            'U' => Self::Uracil,
            'M' => Self::AdenineCytosine,
            'R' => Self::AdenineGuanine,
            'W' => Self::AdenineUracil,
            'S' => Self::CytosineGuanine,
            'Y' => Self::CytosineUracil,
            'K' => Self::GuanineUracil,
            'V' => Self::AdenineCytosineGuanine,
            'H' => Self::AdenineCytosineUracil,
            'D' => Self::AdenineGuanineUracil,
            'B' => Self::CytosineGuanineUracil,
            'N' => Self::Any,
            '.' | '-' => Self::Gap,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "invalid IUPAC RNA code \"{}\"",
                    code
                )))
            }
        })
    }

    #[getter]
    fn get_code(&self) -> char {
        match self {
            Self::Adenine => 'A',
            Self::Cytosine => 'C',
            Self::Guanine => 'G',
            Self::Uracil => 'U',
            Self::AdenineCytosine => 'M',
            Self::AdenineGuanine => 'R',
            Self::AdenineUracil => 'W',
            Self::CytosineGuanine => 'S',
            Self::CytosineUracil => 'Y',
            Self::GuanineUracil => 'K',
            Self::AdenineCytosineGuanine => 'V',
            Self::AdenineCytosineUracil => 'H',
            Self::AdenineGuanineUracil => 'D',
            Self::CytosineGuanineUracil => 'B',
            Self::Any => 'N',
            Self::Gap => '-',
        }
    }

    #[getter]
    fn get_complement(&self) -> Self {
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
    }
}
