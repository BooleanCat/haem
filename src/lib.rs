mod aminoacid;
mod dnabase;
mod dnasequence;
mod rnabase;
mod rnasequence;
mod utils;

use aminoacid::{AminoAcid, StopTranslation};
use dnabase::DNABase;
use dnasequence::DNASequence;
use rnabase::RNABase;
use rnasequence::RNASequence;

use pyo3::prelude::*;

#[pymodule]
fn haem(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RNABase>()?;
    m.add_class::<DNABase>()?;
    m.add_class::<AminoAcid>()?;
    m.add("StopTranslation", py.get_type::<StopTranslation>())?;
    m.add_class::<DNASequence>()?;
    m.add_class::<RNASequence>()?;

    Ok(())
}
