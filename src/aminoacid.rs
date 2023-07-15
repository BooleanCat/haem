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
        match code {
            'A' => Ok(Self::Alanine),
            'C' => Ok(Self::Cysteine),
            'D' => Ok(Self::AsparticAcid),
            'E' => Ok(Self::GlutamicAcid),
            'F' => Ok(Self::Phenylalanine),
            'G' => Ok(Self::Glycine),
            'H' => Ok(Self::Histidine),
            'I' => Ok(Self::Isoleucine),
            'K' => Ok(Self::Lysine),
            'L' => Ok(Self::Leucine),
            'M' => Ok(Self::Methionine),
            'N' => Ok(Self::Asparagine),
            'P' => Ok(Self::Proline),
            'Q' => Ok(Self::Glutamine),
            'R' => Ok(Self::Arginine),
            'S' => Ok(Self::Serine),
            'T' => Ok(Self::Threonine),
            'V' => Ok(Self::Valine),
            'W' => Ok(Self::Tryptophan),
            'Y' => Ok(Self::Tyrosine),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "invalid IUPAC amino acid code \"{}\"",
                code
            ))),
        }
    }

    #[getter]
    fn get_code(&self) -> PyResult<char> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    #[getter]
    fn get_short_name(&self) -> PyResult<&'static str> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
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
