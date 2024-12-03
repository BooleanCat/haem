use crate::aminoacid::{AminoAcid, StopTranslation};
use crate::aminoacidsequence::AminoAcidSequence;
use crate::dnabase::DNABase;
use crate::dnasequence::DNASequence;
use crate::impl_sequence;
use crate::member::MemberOrMembers;
use crate::rnabase::RNABase;
use crate::sequence::{Sequence, SequenceInput};
use crate::utils::IntOrSlice;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rayon::prelude::*;

#[pyclass]
#[derive(FromPyObject)]
pub struct RNASequence {
    pub sequence: Vec<RNABase>,
}

#[pymethods]
impl RNASequence {
    #[new]
    #[pyo3(signature = (sequence = RNASequenceInput::Sequence(SequenceInput::Seq(vec![]))))]
    pub fn __new__(sequence: RNASequenceInput) -> PyResult<Self> {
        sequence.try_into()
    }

    #[getter]
    fn get_complement(&self) -> Self {
        self.sequence
            .par_iter()
            .map(|b| b.get_complement())
            .collect::<Vec<_>>()
            .into()
    }

    fn retro_transcribe(&self) -> DNASequence {
        self.sequence
            .par_iter()
            .map(DNABase::from)
            .collect::<Vec<_>>()
            .into()
    }

    #[pyo3(name = "count", signature = (sequence, overlap = false))]
    fn py_count(&self, sequence: RNASequenceInput, overlap: bool) -> PyResult<usize> {
        self.count(&RNASequence::try_from(sequence)?, overlap)
    }

    #[pyo3(name = "find")]
    fn py_find(&self, sequence: RNASequenceInput) -> PyResult<Option<usize>> {
        self.find(&RNASequence::try_from(sequence)?)
    }

    fn translate(&self, py: Python<'_>) -> PyResult<AminoAcidSequence> {
        // Find start codon
        let start = self
            .members()
            .par_windows(3)
            .map(|codon| AminoAcid::try_from((&codon[0], &codon[1], &codon[2])))
            .position_first(|member| {
                member.is_ok() && member.as_ref().unwrap() == &AminoAcid::Methionine
            });

        if start.is_none() {
            return Err(PyValueError::new_err("no start codon found"));
        }

        // Find stop codon
        let stop = self.members()[start.unwrap()..self.members().len()]
            .chunks_exact(3)
            .map(|codon| AminoAcid::try_from((&codon[0], &codon[1], &codon[2])))
            .position(|member| match member {
                Ok(_) => false,
                Err(err) => err.is_instance_of::<StopTranslation>(py),
            });

        match stop.is_none() {
            false => Ok(
                self.members()[start.unwrap()..(start.unwrap() + stop.unwrap() * 3)]
                    .par_chunks_exact(3)
                    .map(|codon| AminoAcid::try_from((&codon[0], &codon[1], &codon[2])))
                    .collect::<Result<Vec<_>, _>>()?
                    .into(),
            ),
            true => Err(PyValueError::new_err("no stop codon found")),
        }
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

    fn __eq__(&self, other: &Self) -> bool {
        self.eq(other)
    }

    fn __bool__(&self) -> bool {
        self.bool()
    }

    fn __add__(&self, other: RNASequenceInput) -> PyResult<Self> {
        Ok(self.add(&RNASequence::try_from(other)?, false).into())
    }

    fn __radd__(&self, other: RNASequenceInput) -> PyResult<Self> {
        Ok(self.add(&RNASequence::try_from(other)?, true).into())
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

    fn __contains__(&self, sequence: RNASequenceInput) -> PyResult<bool> {
        self.contains(&RNASequence::try_from(sequence)?)
    }
}

impl_sequence!(RNASequence, RNABase, "RNASequence");

#[derive(FromPyObject)]
pub enum RNASequenceInput<'py> {
    RNASequence(RNASequence),
    Sequence(SequenceInput<'py, RNABase>),
}

impl<'py> TryFrom<RNASequenceInput<'py>> for RNASequence {
    type Error = PyErr;

    fn try_from(sequence: RNASequenceInput<'py>) -> PyResult<Self> {
        Ok(match sequence {
            RNASequenceInput::RNASequence(sequence) => sequence,
            RNASequenceInput::Sequence(sequence) => Vec::try_from(sequence)?.into(),
        })
    }
}
