use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::pyclass::PyClass;
use pyo3::types::PySlice;
use rayon::prelude::*;

#[derive(FromPyObject)]
pub enum IntOrSlice<'py> {
    Int(isize),
    Slice(Bound<'py, PySlice>),
}

pub struct Wrapper<T>(pub T);

impl<T> Wrapper<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> TryFrom<PyBackedStr> for Wrapper<Vec<T>>
where
    T: TryFrom<char, Error = PyErr> + Send + PyClass + Clone,
{
    type Error = PyErr;

    fn try_from(value: PyBackedStr) -> PyResult<Self> {
        Ok(Wrapper(
            value
                .as_parallel_string()
                .par_chars()
                .map(T::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
