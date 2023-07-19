use crate::dnabase::DNABase;
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyIterator, PySequence};

#[derive(FromPyObject)]
pub enum DNASequenceInput<'a> {
    BasesStr(&'a str),
    BasesIter(&'a PyIterator),
    BasesSeq(&'a PySequence),
}

#[pyclass(frozen)]
#[derive(PartialEq)]
pub struct DNASequence {
    bases: Vec<DNABase>,
}

#[pymethods]
impl DNASequence {
    #[new]
    #[pyo3(signature = (amino_acids = DNASequenceInput::BasesStr("")))]
    pub fn __new__(amino_acids: DNASequenceInput) -> PyResult<Self> {
        match amino_acids {
            DNASequenceInput::BasesStr(_) => Ok(Self { bases: vec![] }),
            DNASequenceInput::BasesIter(_) => Err(
                pyo3::exceptions::PyNotImplementedError::new_err("not implemented"),
            ),
            DNASequenceInput::BasesSeq(_) => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "not implemented",
            )),
        }
    }

    #[getter]
    fn get_complement(&self) -> PyResult<Self> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn transcribe(&self) -> PyResult<()> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn count(&self, _base: DNABase) -> PyResult<usize> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __invert__(&self) -> PyResult<Self> {
        self.get_complement()
    }

    fn __repr__(&self) -> PyResult<String> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __str__(&self) -> PyResult<String> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __richcmp__(&self, _other: &Self, op: CompareOp, py: Python<'_>) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq | CompareOp::Ne => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "not implemented",
            )),
            _ => Ok(py.NotImplemented()),
        }
    }

    fn __bool__(&self) -> PyResult<bool> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __add__(&self, _other: &Self) -> PyResult<Self> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __len__(&self) -> PyResult<usize> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __getitem__(&self, _index: isize) -> PyResult<Self> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __iter__(&self) -> PyResult<()> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }
}
