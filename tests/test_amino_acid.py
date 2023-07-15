import operator
import typing

import pytest

import haem


def test__new__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.AminoAcid("A")


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
