import operator
import typing

import pytest

import haem


@pytest.fixture
def rna_bases() -> typing.Iterator[haem.RNABase]:
    return map(haem.RNABase, "ACGUMRWSYKVHDBN")


def test__new__alanine(rna_bases: typing.Iterator[haem.RNABase]) -> None:
    assert haem.AminoAcid("A") == haem.AminoAcid.ALANINE

    for base in rna_bases:
        assert (
            haem.AminoAcid((haem.RNABase.GUANINE, haem.RNABase.CYTOSINE, base))
            == haem.AminoAcid.ALANINE
        )

        assert haem.AminoAcid("GC" + base.code) == haem.AminoAcid.ALANINE


def test__new__aspartic_acid_asparagine() -> None:
    assert haem.AminoAcid("B") == haem.AminoAcid.ASPARTIC_ACID_ASPARAGINE

    codons = [
        (haem.RNABase.ADENINE_GUANINE, haem.RNABase.ADENINE, haem.RNABase.CYTOSINE),
        (haem.RNABase.ADENINE_GUANINE, haem.RNABase.ADENINE, haem.RNABase.URACIL),
        (
            haem.RNABase.ADENINE_GUANINE,
            haem.RNABase.ADENINE,
            haem.RNABase.CYTOSINE_URACIL,
        ),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.ASPARTIC_ACID_ASPARAGINE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.ASPARTIC_ACID_ASPARAGINE
        )


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


def test__new__phenylalanine() -> None:
    assert haem.AminoAcid("F") == haem.AminoAcid.PHENYLALANINE

    codons = [
        (haem.RNABase.URACIL, haem.RNABase.URACIL, haem.RNABase.CYTOSINE),
        (haem.RNABase.URACIL, haem.RNABase.URACIL, haem.RNABase.URACIL),
        (haem.RNABase.URACIL, haem.RNABase.URACIL, haem.RNABase.CYTOSINE_URACIL),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.PHENYLALANINE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.PHENYLALANINE
        )


def test__new__glycine(rna_bases: typing.Iterator[haem.RNABase]) -> None:
    assert haem.AminoAcid("G") == haem.AminoAcid.GLYCINE

    for base in rna_bases:
        assert (
            haem.AminoAcid((haem.RNABase.GUANINE, haem.RNABase.GUANINE, base))
            == haem.AminoAcid.GLYCINE
        )

        assert haem.AminoAcid("GG" + base.code) == haem.AminoAcid.GLYCINE


def test__new__histidine() -> None:
    assert haem.AminoAcid("H") == haem.AminoAcid.HISTIDINE

    codons = [
        (haem.RNABase.CYTOSINE, haem.RNABase.ADENINE, haem.RNABase.CYTOSINE),
        (haem.RNABase.CYTOSINE, haem.RNABase.ADENINE, haem.RNABase.URACIL),
        (haem.RNABase.CYTOSINE, haem.RNABase.ADENINE, haem.RNABase.CYTOSINE_URACIL),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.HISTIDINE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.HISTIDINE
        )


def test__new__iso_leucine() -> None:
    assert haem.AminoAcid("I") == haem.AminoAcid.ISOLEUCINE

    codons = [
        (haem.RNABase.ADENINE, haem.RNABase.URACIL, haem.RNABase.ADENINE),
        (haem.RNABase.ADENINE, haem.RNABase.URACIL, haem.RNABase.CYTOSINE),
        (haem.RNABase.ADENINE, haem.RNABase.URACIL, haem.RNABase.URACIL),
        (haem.RNABase.ADENINE, haem.RNABase.URACIL, haem.RNABase.ADENINE_CYTOSINE),
        (haem.RNABase.ADENINE, haem.RNABase.URACIL, haem.RNABase.ADENINE_URACIL),
        (haem.RNABase.ADENINE, haem.RNABase.URACIL, haem.RNABase.CYTOSINE_URACIL),
        (
            haem.RNABase.ADENINE,
            haem.RNABase.URACIL,
            haem.RNABase.ADENINE_CYTOSINE_URACIL,
        ),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.ISOLEUCINE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.ISOLEUCINE
        )


def test__new__lysine() -> None:
    assert haem.AminoAcid("K") == haem.AminoAcid.LYSINE

    codons = [
        (haem.RNABase.ADENINE, haem.RNABase.ADENINE, haem.RNABase.ADENINE),
        (haem.RNABase.ADENINE, haem.RNABase.ADENINE, haem.RNABase.GUANINE),
        (haem.RNABase.ADENINE, haem.RNABase.ADENINE, haem.RNABase.ADENINE_GUANINE),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.LYSINE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.LYSINE
        )


def test__new__leucine(rna_bases: typing.Iterator[haem.RNABase]) -> None:
    assert haem.AminoAcid("L") == haem.AminoAcid.LEUCINE

    for base in rna_bases:
        assert (
            haem.AminoAcid((haem.RNABase.CYTOSINE, haem.RNABase.URACIL, base))
            == haem.AminoAcid.LEUCINE
        )

        assert haem.AminoAcid("CU" + base.code) == haem.AminoAcid.LEUCINE

    codons = [
        (haem.RNABase.URACIL, haem.RNABase.URACIL, haem.RNABase.ADENINE),
        (haem.RNABase.URACIL, haem.RNABase.URACIL, haem.RNABase.GUANINE),
        (haem.RNABase.URACIL, haem.RNABase.URACIL, haem.RNABase.ADENINE_GUANINE),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.LEUCINE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.LEUCINE
        )


def test__new__methionine() -> None:
    assert haem.AminoAcid("M") == haem.AminoAcid.METHIONINE
    assert (
        haem.AminoAcid(
            (haem.RNABase.ADENINE, haem.RNABase.URACIL, haem.RNABase.GUANINE)
        )
        == haem.AminoAcid.METHIONINE
    )
    assert haem.AminoAcid("AUG") == haem.AminoAcid.METHIONINE


def test__new__asparagine() -> None:
    assert haem.AminoAcid("N") == haem.AminoAcid.ASPARAGINE

    codons = [
        (haem.RNABase.ADENINE, haem.RNABase.ADENINE, haem.RNABase.CYTOSINE),
        (haem.RNABase.ADENINE, haem.RNABase.ADENINE, haem.RNABase.URACIL),
        (haem.RNABase.ADENINE, haem.RNABase.ADENINE, haem.RNABase.CYTOSINE_URACIL),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.ASPARAGINE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.ASPARAGINE
        )


def test__new__proline(rna_bases: typing.Iterator[haem.RNABase]) -> None:
    assert haem.AminoAcid("P") == haem.AminoAcid.PROLINE

    for base in rna_bases:
        assert (
            haem.AminoAcid((haem.RNABase.CYTOSINE, haem.RNABase.CYTOSINE, base))
            == haem.AminoAcid.PROLINE
        )

        assert haem.AminoAcid("CC" + base.code) == haem.AminoAcid.PROLINE


def test__new__glutamine() -> None:
    assert haem.AminoAcid("Q") == haem.AminoAcid.GLUTAMINE

    codons = [
        (haem.RNABase.CYTOSINE, haem.RNABase.ADENINE, haem.RNABase.ADENINE),
        (haem.RNABase.CYTOSINE, haem.RNABase.ADENINE, haem.RNABase.GUANINE),
        (haem.RNABase.CYTOSINE, haem.RNABase.ADENINE, haem.RNABase.ADENINE_GUANINE),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.GLUTAMINE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.GLUTAMINE
        )


def test__new__arginine(rna_bases: typing.Iterator[haem.RNABase]) -> None:
    assert haem.AminoAcid("R") == haem.AminoAcid.ARGININE

    for base in rna_bases:
        assert (
            haem.AminoAcid((haem.RNABase.CYTOSINE, haem.RNABase.GUANINE, base))
            == haem.AminoAcid.ARGININE
        )

        assert haem.AminoAcid("CG" + base.code) == haem.AminoAcid.ARGININE

    codons = [
        (haem.RNABase.ADENINE, haem.RNABase.GUANINE, haem.RNABase.ADENINE),
        (haem.RNABase.ADENINE, haem.RNABase.GUANINE, haem.RNABase.GUANINE),
        (haem.RNABase.ADENINE, haem.RNABase.GUANINE, haem.RNABase.ADENINE_GUANINE),
        (haem.RNABase.ADENINE_CYTOSINE, haem.RNABase.GUANINE, haem.RNABase.ADENINE),
        (haem.RNABase.ADENINE_CYTOSINE, haem.RNABase.GUANINE, haem.RNABase.GUANINE),
        (
            haem.RNABase.ADENINE_CYTOSINE,
            haem.RNABase.GUANINE,
            haem.RNABase.ADENINE_GUANINE,
        ),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.ARGININE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.ARGININE
        )


def test__new__serine(rna_bases: typing.Iterator[haem.RNABase]) -> None:
    assert haem.AminoAcid("S") == haem.AminoAcid.SERINE

    for base in rna_bases:
        assert (
            haem.AminoAcid((haem.RNABase.URACIL, haem.RNABase.CYTOSINE, base))
            == haem.AminoAcid.SERINE
        )

        assert haem.AminoAcid("UC" + base.code) == haem.AminoAcid.SERINE

    codons = [
        (haem.RNABase.ADENINE, haem.RNABase.GUANINE, haem.RNABase.CYTOSINE),
        (haem.RNABase.ADENINE, haem.RNABase.GUANINE, haem.RNABase.URACIL),
        (haem.RNABase.ADENINE, haem.RNABase.GUANINE, haem.RNABase.CYTOSINE_URACIL),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.SERINE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.SERINE
        )


def test__new__threonine(rna_bases: typing.Iterator[haem.RNABase]) -> None:
    assert haem.AminoAcid("T") == haem.AminoAcid.THREONINE

    for base in rna_bases:
        assert (
            haem.AminoAcid((haem.RNABase.ADENINE, haem.RNABase.CYTOSINE, base))
            == haem.AminoAcid.THREONINE
        )

        assert haem.AminoAcid("AC" + base.code) == haem.AminoAcid.THREONINE


def test__new__valine(rna_bases: typing.Iterator[haem.RNABase]) -> None:
    assert haem.AminoAcid("V") == haem.AminoAcid.VALINE

    for base in rna_bases:
        assert (
            haem.AminoAcid((haem.RNABase.GUANINE, haem.RNABase.URACIL, base))
            == haem.AminoAcid.VALINE
        )

        assert haem.AminoAcid("GU" + base.code) == haem.AminoAcid.VALINE


def test__new__tryptophan(rna_bases: typing.Iterator[haem.RNABase]) -> None:
    assert haem.AminoAcid("W") == haem.AminoAcid.TRYPTOPHAN
    assert (
        haem.AminoAcid(
            (haem.RNABase.URACIL, haem.RNABase.GUANINE, haem.RNABase.GUANINE)
        )
        == haem.AminoAcid.TRYPTOPHAN
    )
    assert haem.AminoAcid("UGG") == haem.AminoAcid.TRYPTOPHAN


def test__new__any() -> None:
    assert haem.AminoAcid("X") == haem.AminoAcid.ANY


def test__new__tyrosine() -> None:
    assert haem.AminoAcid("Y") == haem.AminoAcid.TYROSINE

    codons = [
        (haem.RNABase.URACIL, haem.RNABase.ADENINE, haem.RNABase.CYTOSINE),
        (haem.RNABase.URACIL, haem.RNABase.ADENINE, haem.RNABase.URACIL),
        (haem.RNABase.URACIL, haem.RNABase.ADENINE, haem.RNABase.CYTOSINE_URACIL),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.TYROSINE

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.TYROSINE
        )


def test__new__glutamine_glutamic_acid() -> None:
    assert haem.AminoAcid("Z") == haem.AminoAcid.GLUTAMINE_GLUTAMIC_ACID

    codons = [
        (haem.RNABase.CYTOSINE_GUANINE, haem.RNABase.ADENINE, haem.RNABase.ADENINE),
        (haem.RNABase.CYTOSINE_GUANINE, haem.RNABase.ADENINE, haem.RNABase.GUANINE),
        (
            haem.RNABase.CYTOSINE_GUANINE,
            haem.RNABase.ADENINE,
            haem.RNABase.ADENINE_GUANINE,
        ),
    ]

    for codon in codons:
        assert haem.AminoAcid(codon) == haem.AminoAcid.GLUTAMINE_GLUTAMIC_ACID

        assert (
            haem.AminoAcid("".join(base.code for base in codon))
            == haem.AminoAcid.GLUTAMINE_GLUTAMIC_ACID
        )


def test__new__sequence_str() -> None:
    assert haem.AminoAcid(("U", "A", "C")) == haem.AminoAcid.TYROSINE


def test__new__stop() -> None:
    codons = [
        (haem.RNABase.URACIL, haem.RNABase.ADENINE, haem.RNABase.ADENINE),
        (haem.RNABase.URACIL, haem.RNABase.ADENINE, haem.RNABase.GUANINE),
        (haem.RNABase.URACIL, haem.RNABase.ADENINE, haem.RNABase.ADENINE_GUANINE),
        (haem.RNABase.URACIL, haem.RNABase.GUANINE, haem.RNABase.ADENINE),
        (haem.RNABase.URACIL, haem.RNABase.ADENINE_GUANINE, haem.RNABase.ADENINE),
    ]

    for codon in codons:
        with pytest.raises(haem.StopTranslation) as excinfo:
            haem.AminoAcid(codon)

        assert str(excinfo.value) == "stop translation"


@pytest.mark.parametrize(
    "code,message",
    [
        ("J", 'invalid IUPAC amino acid code "J"'),
        ("JJ", "invalid amino acid codon"),
        ("NNN", "ambiguous codon"),
        ("---", "codon contains gap"),
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
        (haem.AminoAcid.ASPARTIC_ACID_ASPARAGINE, "aspartic acid/asparagine"),
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
        (haem.AminoAcid.ANY, "any"),
        (haem.AminoAcid.TYROSINE, "tyrosine"),
        (haem.AminoAcid.GLUTAMINE_GLUTAMIC_ACID, "glutamine/glutamic acid"),
    ],
)
def test__str__(amino_acid: haem.AminoAcid, text: str) -> None:
    assert str(amino_acid) == text


@pytest.mark.parametrize(
    "amino_acid,code",
    [
        (haem.AminoAcid.ALANINE, "A"),
        (haem.AminoAcid.ASPARTIC_ACID_ASPARAGINE, "B"),
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
        (haem.AminoAcid.ANY, "X"),
        (haem.AminoAcid.TYROSINE, "Y"),
        (haem.AminoAcid.GLUTAMINE_GLUTAMIC_ACID, "Z"),
    ],
)
def test_code(amino_acid: haem.AminoAcid, code: str) -> None:
    assert amino_acid.code == code


@pytest.mark.parametrize(
    "amino_acid,short_name",
    [
        (haem.AminoAcid.ALANINE, "ala"),
        (haem.AminoAcid.ASPARTIC_ACID_ASPARAGINE, "asx"),
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
        (haem.AminoAcid.ANY, "xaa"),
        (haem.AminoAcid.TYROSINE, "tyr"),
        (haem.AminoAcid.GLUTAMINE_GLUTAMIC_ACID, "glx"),
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


@pytest.mark.parametrize(
    "left,right,result",
    [
        (haem.AminoAcid("M"), haem.AminoAcid("V"), haem.AminoAcidSequence("MV")),
        (
            haem.AminoAcid("M"),
            haem.AminoAcidSequence("VVR"),
            haem.AminoAcidSequence("MVVR"),
        ),
        (haem.AminoAcid("M"), haem.AminoAcidSequence(), haem.AminoAcidSequence("M")),
        (haem.AminoAcid("V"), "M", haem.AminoAcidSequence("VM")),
        (haem.AminoAcid("V"), "", haem.AminoAcidSequence("V")),
    ],
)
def test__add__(
    left: haem.AminoAcid,
    right: typing.Union[haem.AminoAcid, haem.AminoAcidSequence, str],
    result: haem.AminoAcidSequence,
) -> None:
    assert left + right == result


@pytest.mark.parametrize(
    "left,right,result",
    [
        (
            haem.AminoAcidSequence("VVR"),
            haem.AminoAcid("M"),
            haem.AminoAcidSequence("VVRM"),
        ),
        (haem.AminoAcidSequence(), haem.AminoAcid("M"), haem.AminoAcidSequence("M")),
        ("V", haem.AminoAcid("M"), haem.AminoAcidSequence("VM")),
        ("", haem.AminoAcid("M"), haem.AminoAcidSequence("M")),
    ],
)
def test__radd__(
    left: typing.Union[haem.AminoAcidSequence, str],
    right: haem.AminoAcid,
    result: haem.AminoAcidSequence,
) -> None:
    assert left + right == result
