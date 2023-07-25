use pyo3::prelude::*;
use pyo3::types::PySlice;

#[derive(FromPyObject)]
pub enum IntOrSlice<'a> {
    Int(isize),
    Slice(&'a PySlice),
}

pub struct Wrapper<T>(pub T);

impl<T> Wrapper<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
