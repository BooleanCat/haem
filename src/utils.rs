use pyo3::prelude::*;
use pyo3::types::PySlice;
use rayon::prelude::*;

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

#[derive(FromPyObject)]
pub enum SequenceLikeInput<'a, T> {
    Member(T),
    Members(Vec<T>),
    Codes(&'a str),
}

impl<T> TryFrom<SequenceLikeInput<'_, T>> for Wrapper<Vec<T>>
where
    T: TryFrom<char, Error = PyErr> + Send,
{
    type Error = PyErr;

    fn try_from(value: SequenceLikeInput<T>) -> PyResult<Self> {
        Ok(match value {
            SequenceLikeInput::Member(member) => Wrapper(vec![member]),
            SequenceLikeInput::Members(members) => Wrapper(members),
            SequenceLikeInput::Codes(codes) => Wrapper(
                codes
                    .as_parallel_string()
                    .par_chars()
                    .map(T::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        })
    }
}
