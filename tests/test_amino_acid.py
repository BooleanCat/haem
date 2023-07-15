import operator
import typing

import pytest

import haem


@pytest.mark.parametrize(
    "code,amino_acid",
    [
        ("A", haem.AminoAcid.ALANINE),
        ("C", haem.AminoAcid.CYSTEINE),
        ("D", haem.AminoAcid.ASPARTIC_ACID),
        ("E", haem.AminoAcid.GLUTAMIC_ACID),
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


@pytest.mark.parametrize(
    "code,message",
    [
        ("X", 'invalid IUPAC amino acid code "X"'),
        ("XX", "expected a string of length 1"),
    ],
)
def test__new__invalid_code(code: str, message: str) -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.AminoAcid(code)

    assert str(excinfo.value) == message


def test__repr__() -> None:
    assert repr(haem.AminoAcid.ALANINE) == "AminoAcid.ALANINE"


def test__str__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        str(haem.AminoAcid.ALANINE)


def test_code_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.AminoAcid.ALANINE.code


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
