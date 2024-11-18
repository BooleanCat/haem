mod aminoacid;
mod aminoacidsequence;
mod dnabase;
mod dnasequence;
mod member;
mod rnabase;
mod rnasequence;
mod sequence;
mod utils;

use pyo3::prelude::*;

#[pymodule]
mod haem {
    #[pymodule_export]
    use crate::aminoacid::StopTranslation;

    #[pymodule_export]
    use crate::rnabase::RNABase;

    #[pymodule_export]
    use crate::rnasequence::RNASequence;

    #[pymodule_export]
    use crate::dnabase::DNABase;

    #[pymodule_export]
    use crate::dnasequence::DNASequence;

    #[pymodule_export]
    use crate::aminoacid::AminoAcid;

    #[pymodule_export]
    use crate::aminoacidsequence::AminoAcidSequence;
}
