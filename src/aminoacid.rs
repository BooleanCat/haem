use crate::rnabase::RNABase;
use pyo3::class::basic::CompareOp;
use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(haem, StopTranslation, pyo3::exceptions::PyException);

#[derive(FromPyObject)]
enum CodeOrCodon {
    Code(char),
    Codon(RNABase, RNABase, RNABase),
    CodonStr(String),
}

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
    fn __new__(code_or_codon: CodeOrCodon) -> PyResult<Self> {
        Ok(match code_or_codon {
            CodeOrCodon::Code(code) => match code {
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
            },
            CodeOrCodon::Codon(first, second, third) => Self::from_codon(first, second, third)?,
            CodeOrCodon::CodonStr(codon) if codon.len() == 3 => {
                let bases = codon
                    .chars()
                    .map(RNABase::__new__)
                    .collect::<PyResult<Vec<_>>>()?;

                Self::from_codon(bases[0], bases[1], bases[2])?
            }
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "invalid amino acid codon",
                ))
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

impl AminoAcid {
    #[inline]
    fn from_codon(first: RNABase, second: RNABase, third: RNABase) -> PyResult<Self> {
        Ok(match (first, second, third) {
            (RNABase::Gap, _, _) | (_, RNABase::Gap, _) | (_, _, RNABase::Gap) => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "codon contains gap",
                ))
            }

            // Alanine
            (RNABase::Guanine, RNABase::Cytosine, _) => Self::Alanine,

            // Cysteine
            (
                RNABase::Uracil,
                RNABase::Guanine,
                RNABase::Cytosine | RNABase::Uracil | RNABase::CytosineUracil,
            ) => Self::Cysteine,

            // Aspartic acid
            (
                RNABase::Guanine,
                RNABase::Adenine,
                RNABase::Cytosine | RNABase::Uracil | RNABase::CytosineUracil,
            ) => Self::AsparticAcid,

            // Glutamic acid
            (
                RNABase::Guanine,
                RNABase::Adenine,
                RNABase::Adenine | RNABase::Guanine | RNABase::AdenineGuanine,
            ) => Self::GlutamicAcid,

            // Phenylalanine
            (
                RNABase::Uracil,
                RNABase::Uracil,
                RNABase::Cytosine | RNABase::Uracil | RNABase::CytosineUracil,
            ) => Self::Phenylalanine,

            // Glycine
            (RNABase::Guanine, RNABase::Guanine, _) => Self::Glycine,

            // Histidine
            (
                RNABase::Cytosine,
                RNABase::Adenine,
                RNABase::Cytosine | RNABase::Uracil | RNABase::CytosineUracil,
            ) => Self::Histidine,

            // Isoleucine
            (
                RNABase::Adenine,
                RNABase::Uracil,
                RNABase::Adenine
                | RNABase::Cytosine
                | RNABase::Uracil
                | RNABase::AdenineCytosine
                | RNABase::AdenineUracil
                | RNABase::CytosineUracil
                | RNABase::AdenineCytosineUracil,
            ) => Self::Isoleucine,

            // Lysine
            (
                RNABase::Adenine,
                RNABase::Adenine,
                RNABase::Adenine | RNABase::Guanine | RNABase::AdenineGuanine,
            ) => Self::Lysine,

            // Leucine
            (RNABase::Cytosine, RNABase::Uracil, _)
            | (
                RNABase::Uracil,
                RNABase::Uracil,
                RNABase::Adenine | RNABase::Guanine | RNABase::AdenineGuanine,
            ) => Self::Leucine,

            // Methionine
            (RNABase::Adenine, RNABase::Uracil, RNABase::Guanine) => Self::Methionine,

            // Asparagine
            (
                RNABase::Adenine,
                RNABase::Adenine,
                RNABase::Cytosine | RNABase::Uracil | RNABase::CytosineUracil,
            ) => Self::Asparagine,

            // Proline
            (RNABase::Cytosine, RNABase::Cytosine, _) => Self::Proline,

            // Glutamine
            (
                RNABase::Cytosine,
                RNABase::Adenine,
                RNABase::Adenine | RNABase::Guanine | RNABase::AdenineGuanine,
            ) => Self::Glutamine,

            // Arginine
            (RNABase::Cytosine, RNABase::Guanine, _)
            | (
                RNABase::Adenine | RNABase::AdenineCytosine,
                RNABase::Guanine,
                RNABase::Adenine | RNABase::Guanine | RNABase::AdenineGuanine,
            ) => Self::Arginine,

            // Serine
            (RNABase::Uracil, RNABase::Cytosine, _)
            | (
                RNABase::Adenine,
                RNABase::Guanine,
                RNABase::Cytosine | RNABase::Uracil | RNABase::CytosineUracil,
            ) => Self::Serine,

            // Threonine
            (RNABase::Adenine, RNABase::Cytosine, _) => Self::Threonine,

            // Valine
            (RNABase::Guanine, RNABase::Uracil, _) => Self::Valine,

            // Tryptophan
            (RNABase::Uracil, RNABase::Guanine, RNABase::Guanine) => Self::Tryptophan,

            // Tyrosine
            (
                RNABase::Uracil,
                RNABase::Adenine,
                RNABase::Cytosine | RNABase::Uracil | RNABase::CytosineUracil,
            ) => Self::Tyrosine,

            // Stop
            (
                RNABase::Uracil,
                RNABase::Adenine,
                RNABase::Adenine | RNABase::Guanine | RNABase::AdenineGuanine,
            )
            | (RNABase::Uracil, RNABase::Guanine | RNABase::AdenineGuanine, RNABase::Adenine) => {
                return Err(StopTranslation::new_err("stop translation"))
            }

            _ => return Err(pyo3::exceptions::PyValueError::new_err("ambiguous codon")),
        })
    }
}
