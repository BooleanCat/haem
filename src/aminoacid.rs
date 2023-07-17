use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Clone, Copy, PartialEq)]
pub enum AminoAcid {
    #[pyo3(name = "ALANINE")]
    Alanine,

    #[pyo3(name = "CYSTEINE")]
    Cysteine,

    #[pyo3(name = "ASPARTIC_ACID")]
    AsparticAcid,

    #[pyo3(name = "GLUTAMIC_ACID")]
    GlutamicAcid,

    #[pyo3(name = "PHENYLALANINE")]
    Phenylalanine,

    #[pyo3(name = "GLYCINE")]
    Glycine,

    #[pyo3(name = "HISTIDINE")]
    Histidine,

    #[pyo3(name = "ISOLEUCINE")]
    Isoleucine,

    #[pyo3(name = "LYSINE")]
    Lysine,

    #[pyo3(name = "LEUCINE")]
    Leucine,

    #[pyo3(name = "METHIONINE")]
    Methionine,

    #[pyo3(name = "ASPARAGINE")]
    Asparagine,

    #[pyo3(name = "PROLINE")]
    Proline,

    #[pyo3(name = "GLUTAMINE")]
    Glutamine,

    #[pyo3(name = "ARGININE")]
    Arginine,

    #[pyo3(name = "SERINE")]
    Serine,

    #[pyo3(name = "THREONINE")]
    Threonine,

    #[pyo3(name = "VALINE")]
    Valine,

    #[pyo3(name = "TRYPTOPHAN")]
    Tryptophan,

    #[pyo3(name = "TYROSINE")]
    Tyrosine,
}

#[pymethods]
impl AminoAcid {
    #[new]
    fn __new__(code: char) -> PyResult<Self> {
        Ok(match code {
            'A' => Self::Alanine,
            'C' => Self::Cysteine,
            'D' => Self::AsparticAcid,
            'E' => Self::GlutamicAcid,
            'F' => Self::Phenylalanine,
            'G' => Self::Glycine,
            'H' => Self::Histidine,
            'I' => Self::Isoleucine,
            'K' => Self::Lysine,
            'L' => Self::Leucine,
            'M' => Self::Methionine,
            'N' => Self::Asparagine,
            'P' => Self::Proline,
            'Q' => Self::Glutamine,
            'R' => Self::Arginine,
            'S' => Self::Serine,
            'T' => Self::Threonine,
            'V' => Self::Valine,
            'W' => Self::Tryptophan,
            'Y' => Self::Tyrosine,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "invalid IUPAC amino acid code \"{}\"",
                    code
                )))
            }
        })
    }

    #[getter]
    fn get_code(&self) -> char {
        match self {
            Self::Alanine => 'A',
            Self::Cysteine => 'C',
            Self::AsparticAcid => 'D',
            Self::GlutamicAcid => 'E',
            Self::Phenylalanine => 'F',
            Self::Glycine => 'G',
            Self::Histidine => 'H',
            Self::Isoleucine => 'I',
            Self::Lysine => 'K',
            Self::Leucine => 'L',
            Self::Methionine => 'M',
            Self::Asparagine => 'N',
            Self::Proline => 'P',
            Self::Glutamine => 'Q',
            Self::Arginine => 'R',
            Self::Serine => 'S',
            Self::Threonine => 'T',
            Self::Valine => 'V',
            Self::Tryptophan => 'W',
            Self::Tyrosine => 'Y',
        }
    }

    #[getter]
    fn get_short_name(&self) -> &'static str {
        match self {
            Self::Alanine => "ala",
            Self::Cysteine => "cys",
            Self::AsparticAcid => "asp",
            Self::GlutamicAcid => "glu",
            Self::Phenylalanine => "phe",
            Self::Glycine => "gly",
            Self::Histidine => "his",
            Self::Isoleucine => "ile",
            Self::Lysine => "lys",
            Self::Leucine => "leu",
            Self::Methionine => "met",
            Self::Asparagine => "asn",
            Self::Proline => "pro",
            Self::Glutamine => "gln",
            Self::Arginine => "arg",
            Self::Serine => "ser",
            Self::Threonine => "thr",
            Self::Valine => "val",
            Self::Tryptophan => "trp",
            Self::Tyrosine => "tyr",
        }
    }

    fn __str__(&self) -> &'static str {
        match self {
            Self::Alanine => "alanine",
            Self::Cysteine => "cysteine",
            Self::AsparticAcid => "aspartic acid",
            Self::GlutamicAcid => "glutamic acid",
            Self::Phenylalanine => "phenylalanine",
            Self::Glycine => "glycine",
            Self::Histidine => "histidine",
            Self::Isoleucine => "isoleucine",
            Self::Lysine => "lysine",
            Self::Leucine => "leucine",
            Self::Methionine => "methionine",
            Self::Asparagine => "asparagine",
            Self::Proline => "proline",
            Self::Glutamine => "glutamine",
            Self::Arginine => "arginine",
            Self::Serine => "serine",
            Self::Threonine => "threonine",
            Self::Valine => "valine",
            Self::Tryptophan => "tryptophan",
            Self::Tyrosine => "tyrosine",
        }
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self == other).into_py(py),
            CompareOp::Ne => (self != other).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __bool__(&self) -> bool {
        true
    }

    fn __add__(&self, _other: &Self) -> PyResult<()> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }
}
