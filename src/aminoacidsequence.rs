use crate::aminoacid::AminoAcid;
use crate::member::MemberOrMembers;
use crate::sequence::{Sequence, SequenceInput};
use crate::utils::{IntOrSlice, SequenceLikeInput};
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;

#[pyclass(frozen)]
pub struct AminoAcidSequence {
    pub amino_acids: Vec<AminoAcid>,
}

#[pymethods]
impl AminoAcidSequence {
    #[new]
    #[pyo3(signature = (amino_acids = SequenceInput::Str("")))]
    pub fn __new__(amino_acids: SequenceInput<AminoAcid>) -> PyResult<Self> {
        Ok(Self {
            amino_acids: amino_acids.try_into()?,
        })
    }

    #[pyo3(name = "count")]
    #[pyo3(signature = (base, overlap = false))]
    fn py_count(&self, base: SequenceLikeInput<AminoAcid>, overlap: bool) -> PyResult<usize> {
        self.count(base, overlap)
    }

    #[pyo3(name = "find")]
    fn py_find(&self, base: SequenceLikeInput<AminoAcid>) -> PyResult<Option<usize>> {
        self.find(base)
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

    fn __add__(&self, other: SequenceLikeInput<AminoAcid>) -> PyResult<Self> {
        Ok(Self {
            amino_acids: self.add(other, false)?,
        })
    }

    fn __radd__(&self, other: SequenceLikeInput<AminoAcid>) -> PyResult<Self> {
        Ok(Self {
            amino_acids: self.add(other, true)?,
        })
    }

    fn __contains__(&self, amino_acid_or_seq: MemberOrMembers<AminoAcid>) -> PyResult<bool> {
        self.contains(amino_acid_or_seq)
    }

    fn __len__(&self) -> usize {
        self.len()
    }

    fn __getitem__(&self, py: Python, index_or_slice: IntOrSlice) -> PyResult<Py<PyAny>> {
        match self.getitem(index_or_slice)? {
            MemberOrMembers::Member(member) => Ok(member.into_py(py)),
            MemberOrMembers::Sequence(sequence) => Ok(Self {
                amino_acids: sequence,
            }
            .into_py(py)),
        }
    }
}

impl Sequence<AminoAcid> for AminoAcidSequence {
    #[inline]
    fn members(&self) -> &Vec<AminoAcid> {
        &self.amino_acids
    }

    #[inline]
    fn name(&self) -> &str {
        "AminoAcidSequence"
    }
}
