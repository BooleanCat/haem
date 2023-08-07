use crate::aminoacid::{AminoAcid, StopTranslation};
use crate::aminoacidsequence::AminoAcidSequence;
use crate::dnabase::DNABase;
use crate::dnasequence::DNASequence;
use crate::member::MemberOrMembers;
use crate::rnabase::RNABase;
use crate::sequence::{Sequence, SequenceInput};
use crate::utils::{IntOrSlice, SequenceLikeInput};
use pyo3::class::basic::CompareOp;
use pyo3::create_exception;
use pyo3::prelude::*;
use rayon::prelude::*;

create_exception!(haem, NoStopCodon, pyo3::exceptions::PyException);
create_exception!(haem, NoStartCodon, pyo3::exceptions::PyException);

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
        self.count(base, overlap)
    }

    #[pyo3(name = "find")]
    fn py_find(&self, base: SequenceLikeInput<RNABase>) -> PyResult<Option<usize>> {
        self.find(base)
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

    fn __contains__(&self, sequence: SequenceLikeInput<RNABase>) -> PyResult<bool> {
        self.contains(sequence)
    }

    #[pyo3(signature = (reverse = false))]
    fn translate(&self, py: Python<'_>, reverse: bool) -> PyResult<AminoAcidSequence> {
        let mut sequence = match reverse {
            true => self
                .members()
                .iter()
                .rev()
                .map(|base| base.get_complement())
                .collect::<Vec<_>>()
                .chunks_exact(3)
                .map(|chunk| AminoAcid::try_from((&chunk[0], &chunk[1], &chunk[2])))
                .collect::<Vec<_>>(),

            false => self
                .members()
                .chunks_exact(3)
                .map(|chunk| AminoAcid::try_from((&chunk[0], &chunk[1], &chunk[2])))
                .collect::<Vec<_>>(),
        }
        .into_iter();

        if !sequence.any(|amino_acid| {
            amino_acid.is_ok() && *amino_acid.as_ref().unwrap() == AminoAcid::Methionine
        }) {
            // No start codon found
            return Err(NoStartCodon::new_err("no start codon"));
        }

        let mut amino_acids = vec![AminoAcid::Methionine];

        for amino_acid in sequence {
            match amino_acid {
                Ok(amino_acid) => amino_acids.push(amino_acid),
                Err(err) if err.is_instance_of::<StopTranslation>(py) => {
                    return Ok(AminoAcidSequence { amino_acids })
                }
                Err(err) => return Err(err),
            }
        }

        Err(NoStopCodon::new_err("no stop codon"))
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
