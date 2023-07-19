use crate::dnabase::DNABase;
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyIterator;

#[derive(FromPyObject)]
pub enum DNASequenceInput<'a> {
    BasesStr(&'a str),
    BasesIter(&'a PyIterator),
    BasesSeq(Vec<DNABase>),
    BasesSeqStr(Vec<char>),
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
            DNASequenceInput::BasesSeq(bases) => Ok(Self { bases }),
            DNASequenceInput::BasesSeqStr(_) => Err(
                pyo3::exceptions::PyNotImplementedError::new_err("not implemented"),
            ),
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

    fn __repr__(&self) -> String {
        if self.bases.len() == 0 {
            "<DNASequence>".to_string()
        } else {
            format!(
                "<DNASequence: {}>",
                self.bases.iter().map(|b| b.get_code()).collect::<String>()
            )
        }
    }

    fn __str__(&self) -> String {
        if self.bases.len() < 21 {
            self.bases.iter().map(|b| b.get_code()).collect()
        } else {
            format!(
                "{}...{}",
                self.bases[0..10]
                    .iter()
                    .map(|b| b.get_code())
                    .collect::<String>(),
                self.bases[self.bases.len() - 10..self.bases.len()]
                    .iter()
                    .map(|b| b.get_code())
                    .collect::<String>()
            )
        }
    }

    fn __richcmp__(&self, _other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.bases == _other.bases).into_py(py),
            CompareOp::Ne => (self.bases != _other.bases).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __bool__(&self) -> bool {
        self.bases.len() > 0
    }

    fn __add__(&self, _other: &Self) -> PyResult<Self> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __len__(&self) -> usize {
        self.bases.len()
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
