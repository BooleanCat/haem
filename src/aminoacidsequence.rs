use crate::aminoacid::AminoAcid;
use crate::member::MemberOrMembers;
use crate::sequence::{Sequence, SequenceInput};
use crate::utils::IntOrSlice;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(FromPyObject)]
pub struct AminoAcidSequence {
    pub amino_acids: Vec<AminoAcid>,
}

#[pymethods]
impl AminoAcidSequence {
    #[new]
    #[pyo3(signature = (sequence = AminoAcidSequenceInput::Sequence(SequenceInput::Seq(vec![]))))]
    pub fn __new__(sequence: AminoAcidSequenceInput) -> PyResult<Self> {
        sequence.try_into()
    }

    #[pyo3(name = "count", signature = (sequence, overlap = false))]
    fn py_count(&self, sequence: AminoAcidSequenceInput, overlap: bool) -> PyResult<usize> {
        self.count(AminoAcidSequence::try_from(sequence)?.members(), overlap)
    }

    #[pyo3(name = "find")]
    fn py_find(&self, sequence: AminoAcidSequenceInput) -> PyResult<Option<usize>> {
        self.find(AminoAcidSequence::try_from(sequence)?.members())
    }

    fn __repr__(&self) -> String {
        self.repr()
    }

    fn __str__(&self) -> String {
        self.str()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.eq(other)
    }

    fn __bool__(&self) -> bool {
        self.bool()
    }

    fn __add__(&self, other: AminoAcidSequenceInput) -> PyResult<Self> {
        Ok(Self {
            amino_acids: self.add(AminoAcidSequence::try_from(other)?.members().clone(), false)?,
        })
    }

    fn __radd__(&self, other: AminoAcidSequenceInput) -> PyResult<Self> {
        Ok(Self {
            amino_acids: self.add(AminoAcidSequence::try_from(other)?.members().clone(), true)?,
        })
    }

    fn __contains__(&self, sequence: AminoAcidSequenceInput) -> PyResult<bool> {
        self.contains(AminoAcidSequence::try_from(sequence)?.members())
    }

    fn __len__(&self) -> usize {
        self.len()
    }

    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        index_or_slice: IntOrSlice,
    ) -> PyResult<Bound<'py, PyAny>> {
        match self.getitem(index_or_slice)? {
            MemberOrMembers::Member(member) => Ok(member.into_pyobject(py)?.into_any()),
            MemberOrMembers::Sequence(sequence) => Ok(Self {
                amino_acids: sequence,
            }
            .into_pyobject(py)?
            .into_any()),
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

#[derive(FromPyObject)]
pub enum AminoAcidSequenceInput<'py> {
    AminoAcidSequence(AminoAcidSequence),
    #[pyo3(transparent)]
    Sequence(SequenceInput<'py, AminoAcid>),
}

impl<'py> TryFrom<AminoAcidSequenceInput<'py>> for AminoAcidSequence {
    type Error = PyErr;

    fn try_from(sequence: AminoAcidSequenceInput<'py>) -> PyResult<Self> {
        match sequence {
            AminoAcidSequenceInput::AminoAcidSequence(sequence) => Ok(sequence),
            AminoAcidSequenceInput::Sequence(sequence) => Ok(Self {
                amino_acids: sequence.try_into()?,
            }),
        }
    }
}
