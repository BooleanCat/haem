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
        match code {
            'A' => Ok(Self::Adenine),
            'C' => Ok(Self::Cytosine),
            'G' => Ok(Self::Guanine),
            'T' => Ok(Self::Thymine),
            'M' => Ok(Self::AdenineCytosine),
            'R' => Ok(Self::AdenineGuanine),
            'W' => Ok(Self::AdenineThymine),
            'S' => Ok(Self::CytosineGuanine),
            'Y' => Ok(Self::CytosineThymine),
            'K' => Ok(Self::GuanineThymine),
            'V' => Ok(Self::AdenineCytosineGuanine),
            'H' => Ok(Self::AdenineCytosineThymine),
            'D' => Ok(Self::AdenineGuanineThymine),
            'B' => Ok(Self::CytosineGuanineThymine),
            'N' => Ok(Self::Any),
            '.' | '-' => Ok(Self::Gap),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "invalid IUPAC DNA code \"{}\"",
                code
            ))),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
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

    fn transcribe(&self) -> PyResult<RNABase> {
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
}
