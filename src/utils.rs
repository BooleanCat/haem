use pyo3::prelude::*;
use pyo3::types::PySlice;

#[derive(FromPyObject)]
pub enum IntOrSlice<'a> {
    Int(isize),
    Slice(&'a PySlice),
}
