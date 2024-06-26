use crate::dnabase::DNABase;
use crate::member::MemberOrMembers;
use crate::rnabase::RNABase;
use crate::rnasequence::RNASequence;
use crate::sequence::{Sequence, SequenceInput};
use crate::utils::{IntOrSlice, SequenceLikeInput};
use pyo3::prelude::*;
use rayon::prelude::*;

#[pyclass(frozen)]
pub struct DNASequence {
    pub bases: Vec<DNABase>,
}

#[pymethods]
impl DNASequence {
    #[new]
    #[pyo3(signature = (bases = SequenceInput::Seq(vec![])))]
    pub fn __new__(bases: SequenceInput<DNABase>) -> PyResult<Self> {
        Ok(Self {
            bases: bases.try_into()?,
        })
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

    #[pyo3(name = "count", signature = (bases, overlap = false))]
    fn py_count(&self, bases: SequenceLikeInput<DNABase>, overlap: bool) -> PyResult<usize> {
        self.count(bases, overlap)
    }

    #[pyo3(name = "find")]
    fn py_find(&self, bases: SequenceLikeInput<DNABase>) -> PyResult<Option<usize>> {
        self.find(bases)
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

    fn __add__(&self, other: SequenceLikeInput<DNABase>) -> PyResult<Self> {
        Ok(Self {
            bases: self.add(other, false)?,
        })
    }

    fn __radd__(&self, other: SequenceLikeInput<DNABase>) -> PyResult<Self> {
        Ok(Self {
            bases: self.add(other, true)?,
        })
    }

    fn __len__(&self) -> usize {
        self.len()
    }

    fn __getitem__(&self, py: Python, index_or_slice: IntOrSlice) -> PyResult<Py<PyAny>> {
        match self.getitem(index_or_slice)? {
            MemberOrMembers::Member(base) => Ok(base.into_py(py)),
            MemberOrMembers::Sequence(bases) => Ok(Self { bases }.into_py(py)),
        }
    }

    fn __contains__(&self, sequence: SequenceLikeInput<DNABase>) -> PyResult<bool> {
        self.contains(sequence)
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
