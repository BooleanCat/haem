use crate::aminoacidsequence::AminoAcidSequence;
use crate::aminoacidsequence::AminoAcidSequenceInput;
use crate::member::Member;
use crate::rnabase::RNABase;
use crate::sequence::Sequence;
use pyo3::create_exception;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use std::fmt;

create_exception!(haem, StopTranslation, pyo3::exceptions::PyException);

#[derive(FromPyObject)]
enum Codon {
    Bases(RNABase, RNABase, RNABase),
    Chars(char, char, char),
}

impl TryFrom<Codon> for AminoAcid {
    type Error = PyErr;

    fn try_from(codon: Codon) -> PyResult<AminoAcid> {
        match codon {
            Codon::Bases(first, second, third) => (&first, &second, &third).try_into(),
            Codon::Chars(first, second, third) => (first, second, third).try_into(),
        }
    }
}

impl TryFrom<(char, char, char)> for AminoAcid {
    type Error = PyErr;

    fn try_from(codon: (char, char, char)) -> PyResult<AminoAcid> {
        (
            &RNABase::try_from(codon.0)?,
            &RNABase::try_from(codon.1)?,
            &RNABase::try_from(codon.2)?,
        )
            .try_into()
    }
}

#[derive(FromPyObject)]
enum CodeOrCodon {
    Code(char),
    Codon(Codon),
    CodonStr(PyBackedStr),
}

impl TryFrom<CodeOrCodon> for AminoAcid {
    type Error = PyErr;

    fn try_from(code_or_codon: CodeOrCodon) -> PyResult<AminoAcid> {
        Ok(match code_or_codon {
            CodeOrCodon::Code(code) => code.try_into()?,
            CodeOrCodon::Codon(codon) => codon.try_into()?,
            CodeOrCodon::CodonStr(codon) if codon.len() == 3 => {
                let bases = codon
                    .chars()
                    .map(RNABase::try_from)
                    .collect::<PyResult<Vec<_>>>()?;

                (&bases[0], &bases[1], &bases[2]).try_into()?
            }
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "invalid amino acid codon",
                ))
            }
        })
    }
}

#[pyclass(eq, eq_int, frozen, rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AminoAcid {
    Alanine,
    AsparticAcidAsparagine,
    Cysteine,
    AsparticAcid,
    GlutamicAcid,
    Phenylalanine,
    Glycine,
    Histidine,
    Isoleucine,
    Lysine,
    Leucine,
    Methionine,
    Asparagine,
    Proline,
    Glutamine,
    Arginine,
    Serine,
    Threonine,
    Valine,
    Tryptophan,
    Any,
    Tyrosine,
    GlutamineGlutamicAcid,
}

#[pymethods]
impl AminoAcid {
    #[new]
    fn __new__(code_or_codon: CodeOrCodon) -> PyResult<Self> {
        code_or_codon.try_into()
    }

    #[getter]
    fn get_code(&self) -> char {
        self.into()
    }

    #[getter]
    fn get_short_name(&self) -> &'static str {
        match self {
            Self::Alanine => "ala",
            Self::AsparticAcidAsparagine => "asx",
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
            Self::Any => "xaa",
            Self::Tyrosine => "tyr",
            Self::GlutamineGlutamicAcid => "glx",
        }
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __bool__(&self) -> bool {
        true
    }

    fn __add__(&self, other: AminoAcidSequenceInput) -> PyResult<AminoAcidSequence> {
        Ok(AminoAcidSequence {
            amino_acids: self.add(AminoAcidSequence::try_from(other)?.members(), false),
        })
    }

    fn __radd__(&self, other: AminoAcidSequenceInput) -> PyResult<AminoAcidSequence> {
        Ok(AminoAcidSequence {
            amino_acids: self.add(AminoAcidSequence::try_from(other)?.members(), true),
        })
    }
}

impl From<&AminoAcid> for char {
    fn from(amino_acid: &AminoAcid) -> Self {
        match amino_acid {
            AminoAcid::Alanine => 'A',
            AminoAcid::AsparticAcidAsparagine => 'B',
            AminoAcid::Cysteine => 'C',
            AminoAcid::AsparticAcid => 'D',
            AminoAcid::GlutamicAcid => 'E',
            AminoAcid::Phenylalanine => 'F',
            AminoAcid::Glycine => 'G',
            AminoAcid::Histidine => 'H',
            AminoAcid::Isoleucine => 'I',
            AminoAcid::Lysine => 'K',
            AminoAcid::Leucine => 'L',
            AminoAcid::Methionine => 'M',
            AminoAcid::Asparagine => 'N',
            AminoAcid::Proline => 'P',
            AminoAcid::Glutamine => 'Q',
            AminoAcid::Arginine => 'R',
            AminoAcid::Serine => 'S',
            AminoAcid::Threonine => 'T',
            AminoAcid::Valine => 'V',
            AminoAcid::Tryptophan => 'W',
            AminoAcid::Any => 'X',
            AminoAcid::Tyrosine => 'Y',
            AminoAcid::GlutamineGlutamicAcid => 'Z',
        }
    }
}

impl fmt::Display for AminoAcid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alanine => "alanine",
                Self::AsparticAcidAsparagine => "aspartic acid/asparagine",
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
                Self::Any => "any",
                Self::Tyrosine => "tyrosine",
                Self::GlutamineGlutamicAcid => "glutamine/glutamic acid",
            }
        )
    }
}

impl TryFrom<char> for AminoAcid {
    type Error = PyErr;

    fn try_from(code: char) -> PyResult<AminoAcid> {
        Ok(match code {
            'A' => Self::Alanine,
            'B' => Self::AsparticAcidAsparagine,
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
            'X' => Self::Any,
            'Y' => Self::Tyrosine,
            'Z' => Self::GlutamineGlutamicAcid,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "invalid IUPAC amino acid code \"{}\"",
                    code
                )))
            }
        })
    }
}

impl TryFrom<(&RNABase, &RNABase, &RNABase)> for AminoAcid {
    type Error = PyErr;

    fn try_from(codon: (&RNABase, &RNABase, &RNABase)) -> PyResult<AminoAcid> {
        Ok(match (codon.0, codon.1, codon.2) {
            (RNABase::Gap, _, _) | (_, RNABase::Gap, _) | (_, _, RNABase::Gap) => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "codon contains gap",
                ))
            }

            // Alanine
            (RNABase::Guanine, RNABase::Cytosine, _) => Self::Alanine,

            // Aspartic acid/Asparagine
            (
                RNABase::AdenineGuanine,
                RNABase::Adenine,
                RNABase::Cytosine | RNABase::Uracil | RNABase::CytosineUracil,
            ) => Self::AsparticAcidAsparagine,

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

            (
                RNABase::CytosineGuanine,
                RNABase::Adenine,
                RNABase::Adenine | RNABase::Guanine | RNABase::AdenineGuanine,
            ) => Self::GlutamineGlutamicAcid,

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
