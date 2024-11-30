use crate::aminoacid::AminoAcid;
use crate::impl_sequence;
use crate::member::MemberOrMembers;
use crate::sequence::{Sequence, SequenceInput};
use crate::utils::IntOrSlice;
use pyo3::prelude::*;

#[pyclass]
#[derive(FromPyObject)]
pub struct AminoAcidSequence {
    pub sequence: Vec<AminoAcid>,
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
        self.count(&AminoAcidSequence::try_from(sequence)?, overlap)
    }

    #[pyo3(name = "find")]
    fn py_find(&self, sequence: AminoAcidSequenceInput) -> PyResult<Option<usize>> {
        self.find(&AminoAcidSequence::try_from(sequence)?)
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
        Ok(self.add(&AminoAcidSequence::try_from(other)?, false).into())
    }

    fn __radd__(&self, other: AminoAcidSequenceInput) -> PyResult<Self> {
        Ok(self.add(&AminoAcidSequence::try_from(other)?, true).into())
    }

    fn __contains__(&self, sequence: AminoAcidSequenceInput) -> PyResult<bool> {
        self.contains(&AminoAcidSequence::try_from(sequence)?)
    }

    fn __len__(&self) -> usize {
        self.len()
    }

    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        index_or_slice: IntOrSlice,
    ) -> PyResult<Bound<'py, PyAny>> {
        Ok(match self.getitem(index_or_slice)? {
            MemberOrMembers::Member(member) => member.into_pyobject(py)?.into_any(),
            MemberOrMembers::Sequence(sequence) => {
                Self::from(sequence).into_pyobject(py)?.into_any()
            }
        })
    }
}

impl_sequence!(AminoAcidSequence, AminoAcid, "AminoAcidSequence");

#[derive(FromPyObject)]
pub enum AminoAcidSequenceInput<'py> {
    AminoAcidSequence(AminoAcidSequence),
    Sequence(SequenceInput<'py, AminoAcid>),
}

impl<'py> TryFrom<AminoAcidSequenceInput<'py>> for AminoAcidSequence {
    type Error = PyErr;

    fn try_from(sequence: AminoAcidSequenceInput<'py>) -> PyResult<Self> {
        match sequence {
            AminoAcidSequenceInput::AminoAcidSequence(sequence) => Ok(sequence),
            AminoAcidSequenceInput::Sequence(sequence) => Ok(Vec::try_from(sequence)?.into()),
        }
    }
}
