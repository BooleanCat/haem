use crate::dnabase::DNABase;
use crate::member::MemberOrMembers;
use crate::rnabase::RNABase;
use crate::rnasequence::RNASequence;
use crate::sequence::{Sequence, SequenceInput};
use crate::utils::IntOrSlice;
use pyo3::prelude::*;
use rayon::prelude::*;

#[derive(FromPyObject)]
#[pyclass(frozen)]
pub struct DNASequence {
    pub bases: Vec<DNABase>,
}

#[pymethods]
impl DNASequence {
    #[new]
    #[pyo3(signature = (sequence = DNASequenceInput::Sequence(SequenceInput::Seq(vec![]))))]
    pub fn __new__(sequence: DNASequenceInput) -> PyResult<Self> {
        sequence.try_into()
    }

    #[getter]
    fn get_complement(&self) -> Self {
        Self {
            bases: self
                .bases
                .par_iter()
                .map(|base| base.get_complement())
                .collect(),
        }
    }

    fn transcribe(&self) -> RNASequence {
        RNASequence {
            bases: self.bases.par_iter().map(RNABase::from).collect::<Vec<_>>(),
        }
    }

    #[pyo3(name = "count", signature = (sequence, overlap = false))]
    fn py_count(&self, sequence: DNASequenceInput, overlap: bool) -> PyResult<usize> {
        self.count(DNASequence::try_from(sequence)?.members(), overlap)
    }

    #[pyo3(name = "find")]
    fn py_find(&self, sequence: DNASequenceInput) -> PyResult<Option<usize>> {
        self.find(DNASequence::try_from(sequence)?.members())
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

    fn __add__(&self, other: DNASequenceInput) -> PyResult<Self> {
        Ok(Self {
            bases: self.add(DNASequence::try_from(other)?.members().clone(), false)?,
        })
    }

    fn __radd__(&self, other: DNASequenceInput) -> PyResult<Self> {
        Ok(Self {
            bases: self.add(DNASequence::try_from(other)?.members().clone(), true)?,
        })
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
            MemberOrMembers::Member(base) => Ok(base.into_pyobject(py)?.into_any()),
            MemberOrMembers::Sequence(bases) => Ok(Self { bases }.into_pyobject(py)?.into_any()),
        }
    }

    fn __contains__(&self, sequence: DNASequenceInput) -> PyResult<bool> {
        self.contains(DNASequence::try_from(sequence)?.members())
    }
}

impl Sequence<DNABase> for DNASequence {
    #[inline]
    fn members(&self) -> &Vec<DNABase> {
        &self.bases
    }

    #[inline]
    fn name(&self) -> &str {
        "DNASequence"
    }
}

#[derive(FromPyObject)]
pub enum DNASequenceInput<'py> {
    DNASequence(DNASequence),
    #[pyo3(transparent)]
    Sequence(SequenceInput<'py, DNABase>),
}

impl<'py> TryFrom<DNASequenceInput<'py>> for DNASequence {
    type Error = PyErr;

    fn try_from(sequence: DNASequenceInput<'py>) -> PyResult<Self> {
        match sequence {
            DNASequenceInput::DNASequence(sequence) => Ok(sequence),
            DNASequenceInput::Sequence(sequence) => Ok(Self {
                bases: sequence.try_into()?,
            }),
        }
    }
}
