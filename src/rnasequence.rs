use crate::dnabase::DNABase;
use crate::dnasequence::DNASequence;
use crate::member::MemberOrMembers;
use crate::rnabase::RNABase;
use crate::sequence::{Sequence, SequenceInput};
use crate::utils::{IntOrSlice, SequenceLikeInput};
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;
use rayon::prelude::*;

#[pyclass(frozen)]
pub struct RNASequence {
    pub bases: Vec<RNABase>,
}

#[pymethods]
impl RNASequence {
    #[new]
    #[pyo3(signature = (bases = SequenceInput::Str("")))]
    pub fn __new__(bases: SequenceInput<RNABase>) -> PyResult<Self> {
        Ok(Self {
            bases: bases.try_into()?,
        })
    }

    #[getter]
    fn get_complement(&self) -> Self {
        Self {
            bases: self.bases.par_iter().map(|b| b.get_complement()).collect(),
        }
    }

    fn retro_transcribe(&self) -> DNASequence {
        DNASequence {
            bases: self.bases.par_iter().map(DNABase::from).collect::<Vec<_>>(),
        }
    }

    #[pyo3(name = "count")]
    #[pyo3(signature = (base, overlap = false))]
    fn py_count(&self, base: SequenceLikeInput<RNABase>, overlap: bool) -> PyResult<usize> {
        if overlap {}
        self.count(base)
    }

    fn __invert__(&self) -> Self {
        self.get_complement()
    }

    fn __repr__(&self) -> String {
        self.repr()
    }

    fn __str__(&self) -> String {
        self.str()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        self.richcmp(other, op, py)
    }

    fn __bool__(&self) -> bool {
        self.bool()
    }

    fn __add__(&self, other: SequenceLikeInput<RNABase>) -> PyResult<Self> {
        Ok(Self {
            bases: self.add(other, false)?,
        })
    }

    fn __radd__(&self, other: SequenceLikeInput<RNABase>) -> PyResult<Self> {
        Ok(Self {
            bases: self.add(other, true)?,
        })
    }

    fn __len__(&self) -> usize {
        self.len()
    }

    fn __getitem__(&self, py: Python, index_or_slice: IntOrSlice) -> PyResult<Py<PyAny>> {
        match self.getitem(index_or_slice)? {
            MemberOrMembers::Member(member) => Ok(member.into_py(py)),
            MemberOrMembers::Sequence(sequence) => Ok(Self { bases: sequence }.into_py(py)),
        }
    }

    fn __contains__(&self, base_or_seq: MemberOrMembers<RNABase>) -> PyResult<bool> {
        self.contains(base_or_seq)
    }
}

impl Sequence<RNABase> for RNASequence {
    #[inline]
    fn members(&self) -> &Vec<RNABase> {
        &self.bases
    }

    #[inline]
    fn name(&self) -> &str {
        "RNASequence"
    }
}
