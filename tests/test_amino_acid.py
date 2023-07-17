import operator
import typing

import pytest

import haem


@pytest.fixture
def rna_bases() -> typing.Iterator[haem.RNABase]:
    return map(haem.RNABase, "ACGUMRWSYKVHDBN")


@pytest.mark.parametrize(
    "code,amino_acid",
    [
        ("F", haem.AminoAcid.PHENYLALANINE),
        ("G", haem.AminoAcid.GLYCINE),
        ("H", haem.AminoAcid.HISTIDINE),
        ("I", haem.AminoAcid.ISOLEUCINE),
        ("K", haem.AminoAcid.LYSINE),
        ("L", haem.AminoAcid.LEUCINE),
        ("M", haem.AminoAcid.METHIONINE),
        ("N", haem.AminoAcid.ASPARAGINE),
        ("P", haem.AminoAcid.PROLINE),
        ("Q", haem.AminoAcid.GLUTAMINE),
        ("R", haem.AminoAcid.ARGININE),
        ("S", haem.AminoAcid.SERINE),
        ("T", haem.AminoAcid.THREONINE),
        ("V", haem.AminoAcid.VALINE),
        ("W", haem.AminoAcid.TRYPTOPHAN),
        ("Y", haem.AminoAcid.TYROSINE),
    ],
)
def test__new__(code: str, amino_acid: haem.AminoAcid) -> None:
    assert haem.AminoAcid(code) == amino_acid


def test__new__alanine(rna_bases: typing.Iterator[haem.RNABase]) -> None:
    assert haem.AminoAcid("A") == haem.AminoAcid.ALANINE

    for base in rna_bases:
        assert (
            haem.AminoAcid((haem.RNABase.GUANINE, haem.RNABase.CYTOSINE, base))
            == haem.AminoAcid.ALANINE
        )

        assert haem.AminoAcid("GC" + base.code) == haem.AminoAcid.ALANINE


def test__new__cysteine() -> None:
    assert haem.AminoAcid("C") == haem.AminoAcid.CYSTEINE

    codons = [
        (haem.RNABase.URACIL, haem.RNABase.GUANINE, haem.RNABase.CYTOSINE),
        (haem.RNABase.URACIL, haem.RNABase.GUANINE, haem.RNABase.URACIL),
        (haem.RNABase.URACIL, haem.RNABase.GUANINE, haem.RNABase.CYTOSINE_URACIL),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.CYSTEINE
        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.CYSTEINE
        )


def test__new__glutamic_acid() -> None:
    assert haem.AminoAcid("E") == haem.AminoAcid.GLUTAMIC_ACID

    codons = [
        (haem.RNABase.GUANINE, haem.RNABase.ADENINE, haem.RNABase.ADENINE),
        (haem.RNABase.GUANINE, haem.RNABase.ADENINE, haem.RNABase.GUANINE),
        (haem.RNABase.GUANINE, haem.RNABase.ADENINE, haem.RNABase.ADENINE_GUANINE),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.GLUTAMIC_ACID
        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.GLUTAMIC_ACID
        )


def test__new__aspartic_acid() -> None:
    assert haem.AminoAcid("D") == haem.AminoAcid.ASPARTIC_ACID

    codons = [
        (haem.RNABase.GUANINE, haem.RNABase.ADENINE, haem.RNABase.CYTOSINE),
        (haem.RNABase.GUANINE, haem.RNABase.ADENINE, haem.RNABase.URACIL),
        (haem.RNABase.GUANINE, haem.RNABase.ADENINE, haem.RNABase.CYTOSINE_URACIL),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.ASPARTIC_ACID
        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.ASPARTIC_ACID
        )


@pytest.mark.parametrize(
    "code,message",
    [
        ("X", 'invalid IUPAC amino acid code "X"'),
        ("XX", "invalid amino acid codon"),
        ("NNN", "ambiguous codon"),
    ],
)
def test__new__invalid_code(code: str, message: str) -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.AminoAcid(code)

    assert str(excinfo.value) == message


def test__repr__() -> None:
    assert repr(haem.AminoAcid.ALANINE) == "AminoAcid.ALANINE"


@pytest.mark.parametrize(
    "amino_acid,text",
    [
        (haem.AminoAcid.ALANINE, "alanine"),
        (haem.AminoAcid.CYSTEINE, "cysteine"),
        (haem.AminoAcid.ASPARTIC_ACID, "aspartic acid"),
        (haem.AminoAcid.GLUTAMIC_ACID, "glutamic acid"),
        (haem.AminoAcid.PHENYLALANINE, "phenylalanine"),
        (haem.AminoAcid.GLYCINE, "glycine"),
        (haem.AminoAcid.HISTIDINE, "histidine"),
        (haem.AminoAcid.ISOLEUCINE, "isoleucine"),
        (haem.AminoAcid.LYSINE, "lysine"),
        (haem.AminoAcid.LEUCINE, "leucine"),
        (haem.AminoAcid.METHIONINE, "methionine"),
        (haem.AminoAcid.ASPARAGINE, "asparagine"),
        (haem.AminoAcid.PROLINE, "proline"),
        (haem.AminoAcid.GLUTAMINE, "glutamine"),
        (haem.AminoAcid.ARGININE, "arginine"),
        (haem.AminoAcid.SERINE, "serine"),
        (haem.AminoAcid.THREONINE, "threonine"),
        (haem.AminoAcid.VALINE, "valine"),
        (haem.AminoAcid.TRYPTOPHAN, "tryptophan"),
        (haem.AminoAcid.TYROSINE, "tyrosine"),
    ],
)
def test__str__(amino_acid: haem.AminoAcid, text: str) -> None:
    assert str(amino_acid) == text


@pytest.mark.parametrize(
    "amino_acid,code",
    [
        (haem.AminoAcid.ALANINE, "A"),
        (haem.AminoAcid.CYSTEINE, "C"),
        (haem.AminoAcid.ASPARTIC_ACID, "D"),
        (haem.AminoAcid.GLUTAMIC_ACID, "E"),
        (haem.AminoAcid.PHENYLALANINE, "F"),
        (haem.AminoAcid.GLYCINE, "G"),
        (haem.AminoAcid.HISTIDINE, "H"),
        (haem.AminoAcid.ISOLEUCINE, "I"),
        (haem.AminoAcid.LYSINE, "K"),
        (haem.AminoAcid.LEUCINE, "L"),
        (haem.AminoAcid.METHIONINE, "M"),
        (haem.AminoAcid.ASPARAGINE, "N"),
        (haem.AminoAcid.PROLINE, "P"),
        (haem.AminoAcid.GLUTAMINE, "Q"),
        (haem.AminoAcid.ARGININE, "R"),
        (haem.AminoAcid.SERINE, "S"),
        (haem.AminoAcid.THREONINE, "T"),
        (haem.AminoAcid.VALINE, "V"),
        (haem.AminoAcid.TRYPTOPHAN, "W"),
        (haem.AminoAcid.TYROSINE, "Y"),
    ],
)
def test_code(amino_acid: haem.AminoAcid, code: str) -> None:
    assert amino_acid.code == code


@pytest.mark.parametrize(
    "amino_acid,short_name",
    [
        (haem.AminoAcid.ALANINE, "ala"),
        (haem.AminoAcid.CYSTEINE, "cys"),
        (haem.AminoAcid.ASPARTIC_ACID, "asp"),
        (haem.AminoAcid.GLUTAMIC_ACID, "glu"),
        (haem.AminoAcid.PHENYLALANINE, "phe"),
        (haem.AminoAcid.GLYCINE, "gly"),
        (haem.AminoAcid.HISTIDINE, "his"),
        (haem.AminoAcid.ISOLEUCINE, "ile"),
        (haem.AminoAcid.LYSINE, "lys"),
        (haem.AminoAcid.LEUCINE, "leu"),
        (haem.AminoAcid.METHIONINE, "met"),
        (haem.AminoAcid.ASPARAGINE, "asn"),
        (haem.AminoAcid.PROLINE, "pro"),
        (haem.AminoAcid.GLUTAMINE, "gln"),
        (haem.AminoAcid.ARGININE, "arg"),
        (haem.AminoAcid.SERINE, "ser"),
        (haem.AminoAcid.THREONINE, "thr"),
        (haem.AminoAcid.VALINE, "val"),
        (haem.AminoAcid.TRYPTOPHAN, "trp"),
        (haem.AminoAcid.TYROSINE, "tyr"),
    ],
)
def test_short_name(amino_acid: haem.AminoAcid, short_name: str) -> None:
    assert amino_acid.short_name == short_name


def test__eq__() -> None:
    assert haem.AminoAcid.ALANINE == haem.AminoAcid.ALANINE


def test__ne__() -> None:
    assert haem.AminoAcid.ALANINE != haem.AminoAcid.ARGININE


@pytest.mark.parametrize("op", [operator.gt, operator.ge, operator.lt, operator.le])
def test_unsupported_comparison(
    op: typing.Callable[[haem.AminoAcid, haem.AminoAcid], bool]
) -> None:
    with pytest.raises(TypeError):
        op(haem.AminoAcid.ALANINE, haem.AminoAcid.ARGININE)


def test__bool__() -> None:
    assert bool(haem.AminoAcid.ALANINE) is True


def test__add__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.AminoAcid.ALANINE + haem.AminoAcid.ARGININE
