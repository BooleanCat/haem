import operator
import typing

import pytest

import haem


def test_instantiate() -> None:
    haem.DNABase.ADENINE


def test__new__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNABase("A")


def test__repr__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        repr(haem.DNABase.ADENINE)


def test__str__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        str(haem.DNABase.ADENINE)


def test_code_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNABase.ADENINE.code


def test_complement_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNABase.ADENINE.complement


def test_transcribe_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNABase.ADENINE.transcribe()


def test__eq__() -> None:
    assert haem.DNABase.ADENINE == haem.DNABase.ADENINE


def test__ne__() -> None:
    assert haem.DNABase.ADENINE != haem.DNABase.CYTOSINE


def test__gt__not_supported() -> None:
    with pytest.raises(TypeError):
        haem.DNABase.ADENINE > haem.DNABase.CYTOSINE  # type: ignore


@pytest.mark.parametrize("op", [operator.gt, operator.ge, operator.lt, operator.le])
def test_unsupported_comparison(
    op: typing.Callable[[haem.DNABase, haem.DNABase], bool]
) -> None:
    with pytest.raises(TypeError):
        op(haem.DNABase.ADENINE, haem.DNABase.CYTOSINE)


def test__bool__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        bool(haem.DNABase.ADENINE)


def test__invert__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        ~haem.DNABase.ADENINE


def test__add__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNABase.ADENINE + haem.DNABase.CYTOSINE
