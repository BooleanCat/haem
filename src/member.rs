use crate::utils::MemberOrSequence;
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;

pub trait Member<T> {
    fn add(&self, other: MemberOrSequence<T>) -> Vec<T>;
    fn richcmp(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject;
}

impl<T: Clone + PartialEq> Member<T> for T {
    fn add(&self, other: MemberOrSequence<T>) -> Vec<T> {
        match other {
            MemberOrSequence::Member(member) => {
                vec![self.clone(), member]
            }
            MemberOrSequence::Sequence(sequence) => {
                let mut sequence = sequence;
                sequence.insert(0, self.clone());
                sequence
            }
        }
    }

    fn richcmp(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self == other).into_py(py),
            CompareOp::Ne => (self != other).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}
