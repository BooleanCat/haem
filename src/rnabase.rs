use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Clone, Copy, PartialEq)]
pub enum RNABase {
    ADENINE,
}
