import operator
import typing

import pytest

import haem


@pytest.mark.parametrize(
    "code,base",
    [
        ("A", haem.DNABase.ADENINE),
        ("C", haem.DNABase.CYTOSINE),
        ("G", haem.DNABase.GUANINE),
        ("T", haem.DNABase.THYMINE),
        ("M", haem.DNABase.ADENINE_CYTOSINE),
        ("R", haem.DNABase.ADENINE_GUANINE),
        ("W", haem.DNABase.ADENINE_THYMINE),
        ("S", haem.DNABase.CYTOSINE_GUANINE),
        ("Y", haem.DNABase.CYTOSINE_THYMINE),
        ("K", haem.DNABase.GUANINE_THYMINE),
        ("V", haem.DNABase.ADENINE_CYTOSINE_GUANINE),
        ("H", haem.DNABase.ADENINE_CYTOSINE_THYMINE),
        ("D", haem.DNABase.ADENINE_GUANINE_THYMINE),
        ("B", haem.DNABase.CYTOSINE_GUANINE_THYMINE),
        ("N", haem.DNABase.ANY),
        (".", haem.DNABase.GAP),
        ("-", haem.DNABase.GAP),
    ],
)
def test__new__(code: str, base: haem.DNABase) -> None:
    assert haem.DNABase(code) == base


@pytest.mark.parametrize(
    "code,message",
    [("X", 'invalid IUPAC DNA code "X"'), ("XX", "expected a string of length 1")],
)
def test__new__invalid_code(code: str, message: str) -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.DNABase(code)

    assert str(excinfo.value) == message


def test__repr__() -> None:
    assert repr(haem.DNABase.ADENINE) == "DNABase.ADENINE"


@pytest.mark.parametrize(
    "base,text",
    [
        (haem.DNABase.ADENINE, "adenine"),
        (haem.DNABase.CYTOSINE, "cytosine"),
        (haem.DNABase.GUANINE, "guanine"),
        (haem.DNABase.THYMINE, "thymine"),
        (haem.DNABase.ADENINE_CYTOSINE, "adenine/cytosine"),
        (haem.DNABase.ADENINE_GUANINE, "adenine/guanine"),
        (haem.DNABase.ADENINE_THYMINE, "adenine/thymine"),
        (haem.DNABase.CYTOSINE_GUANINE, "cytosine/guanine"),
        (haem.DNABase.CYTOSINE_THYMINE, "cytosine/thymine"),
        (haem.DNABase.GUANINE_THYMINE, "guanine/thymine"),
        (haem.DNABase.ADENINE_CYTOSINE_GUANINE, "adenine/cytosine/guanine"),
        (haem.DNABase.ADENINE_CYTOSINE_THYMINE, "adenine/cytosine/thymine"),
        (haem.DNABase.ADENINE_GUANINE_THYMINE, "adenine/guanine/thymine"),
        (haem.DNABase.CYTOSINE_GUANINE_THYMINE, "cytosine/guanine/thymine"),
        (haem.DNABase.ANY, "any"),
        (haem.DNABase.GAP, "gap"),
    ],
)
def test__str__(base: haem.DNABase, text: str) -> None:
    assert str(base) == text


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


@pytest.mark.parametrize("op", [operator.gt, operator.ge, operator.lt, operator.le])
def test_unsupported_comparison(
    op: typing.Callable[[haem.DNABase, haem.DNABase], bool]
) -> None:
    with pytest.raises(TypeError):
        op(haem.DNABase.ADENINE, haem.DNABase.CYTOSINE)


@pytest.mark.parametrize(
    "base,result", [(haem.DNABase.ADENINE, True), (haem.DNABase.GAP, False)]
)
def test__bool__(base: haem.DNABase, result: bool) -> None:
    assert bool(base) == result


def test__invert__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        ~haem.DNABase.ADENINE


def test__add__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNABase.ADENINE + haem.DNABase.CYTOSINE
