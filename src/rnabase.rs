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
        match code {
            'A' => Ok(Self::Adenine),
            'C' => Ok(Self::Cytosine),
            'G' => Ok(Self::Guanine),
            'U' => Ok(Self::Uracil),
            'M' => Ok(Self::AdenineCytosine),
            'R' => Ok(Self::AdenineGuanine),
            'W' => Ok(Self::AdenineUracil),
            'S' => Ok(Self::CytosineGuanine),
            'Y' => Ok(Self::CytosineUracil),
            'K' => Ok(Self::GuanineUracil),
            'V' => Ok(Self::AdenineCytosineGuanine),
            'H' => Ok(Self::AdenineCytosineUracil),
            'D' => Ok(Self::AdenineGuanineUracil),
            'B' => Ok(Self::CytosineGuanineUracil),
            'N' => Ok(Self::Any),
            '.' | '-' => Ok(Self::Gap),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "invalid IUPAC RNA code \"{}\"",
                code
            ))),
        }
    }

    #[getter]
    fn get_code(&self) -> PyResult<char> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    #[getter]
    fn get_complement(&self) -> PyResult<Self> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self == other).into_py(py),
            CompareOp::Ne => (self != other).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __bool__(&self) -> PyResult<bool> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __invert__(&self) -> PyResult<Self> {
        self.get_complement()
    }

    fn __add__(&self, _other: &Self) -> PyResult<()> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __str__(&self) -> String {
        match self {
            Self::Adenine => "adenine".to_string(),
            Self::Cytosine => "cytosine".to_string(),
            Self::Guanine => "guanine".to_string(),
            Self::Uracil => "uracil".to_string(),
            Self::AdenineCytosine => "adenine/cytosine".to_string(),
            Self::AdenineGuanine => "adenine/guanine".to_string(),
            Self::AdenineUracil => "adenine/uracil".to_string(),
            Self::CytosineGuanine => "cytosine/guanine".to_string(),
            Self::CytosineUracil => "cytosine/uracil".to_string(),
            Self::GuanineUracil => "guanine/uracil".to_string(),
            Self::AdenineCytosineGuanine => "adenine/cytosine/guanine".to_string(),
            Self::AdenineCytosineUracil => "adenine/cytosine/uracil".to_string(),
            Self::AdenineGuanineUracil => "adenine/guanine/uracil".to_string(),
            Self::CytosineGuanineUracil => "cytosine/guanine/uracil".to_string(),
            Self::Any => "any".to_string(),
            Self::Gap => "gap".to_string(),
        }
    }
}
