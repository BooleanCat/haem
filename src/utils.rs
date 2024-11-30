use pyo3::prelude::*;
use pyo3::types::PySlice;

#[derive(FromPyObject)]
pub enum IntOrSlice<'py> {
    Int(isize),
    Slice(Bound<'py, PySlice>),
}
