import operator
import typing

import pytest

import haem


def test_instantiate() -> None:
    haem.RNABase.ADENINE


def test__new__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.RNABase("A")


def test__repr__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        repr(haem.RNABase.ADENINE)


def test__str__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        str(haem.RNABase.ADENINE)


def test_code_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.RNABase.ADENINE.code


def test_code_complement_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.RNABase.ADENINE.complement


def test__eq__() -> None:
    assert haem.RNABase.ADENINE == haem.RNABase.ADENINE


def test__ne__() -> None:
    assert haem.RNABase.ADENINE != haem.RNABase.CYTOSINE


@pytest.mark.parametrize("op", [operator.gt, operator.ge, operator.lt, operator.le])
def test_unsupported_comparison(
    op: typing.Callable[[haem.RNABase, haem.RNABase], bool]
) -> None:
    with pytest.raises(TypeError):
        op(haem.RNABase.ADENINE, haem.RNABase.CYTOSINE)


def test__bool__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        bool(haem.RNABase.ADENINE)


def test__invert__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        ~haem.RNABase.ADENINE


def test__add__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.RNABase.ADENINE + haem.RNABase.CYTOSINE
