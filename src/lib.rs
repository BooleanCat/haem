mod dnabase;
mod rnabase;

use dnabase::DNABase;
use rnabase::RNABase;

use pyo3::prelude::*;

#[pymodule]
fn haem(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RNABase>()?;
    m.add_class::<DNABase>()?;

    Ok(())
}
