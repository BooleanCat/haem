import operator
import typing

import pytest

import haem


@pytest.mark.parametrize(
    "code,base",
    [
        ("A", haem.RNABase.ADENINE),
        ("C", haem.RNABase.CYTOSINE),
        ("G", haem.RNABase.GUANINE),
        ("U", haem.RNABase.URACIL),
        ("M", haem.RNABase.ADENINE_CYTOSINE),
        ("R", haem.RNABase.ADENINE_GUANINE),
        ("W", haem.RNABase.ADENINE_URACIL),
        ("S", haem.RNABase.CYTOSINE_GUANINE),
        ("Y", haem.RNABase.CYTOSINE_URACIL),
        ("K", haem.RNABase.GUANINE_URACIL),
        ("V", haem.RNABase.ADENINE_CYTOSINE_GUANINE),
        ("H", haem.RNABase.ADENINE_CYTOSINE_URACIL),
        ("D", haem.RNABase.ADENINE_GUANINE_URACIL),
        ("B", haem.RNABase.CYTOSINE_GUANINE_URACIL),
        ("N", haem.RNABase.ANY),
        (".", haem.RNABase.GAP),
        ("-", haem.RNABase.GAP),
    ],
)
def test__new__(code: str, base: haem.RNABase) -> None:
    assert haem.RNABase(code) == base


@pytest.mark.parametrize(
    "code,message",
    [("X", 'invalid IUPAC RNA code "X"'), ("XX", "expected a string of length 1")],
)
def test__new__invalid_code(code: str, message: str) -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.RNABase(code)

    assert str(excinfo.value) == message


def test__repr__() -> None:
    assert repr(haem.RNABase.ADENINE) == "RNABase.ADENINE"


@pytest.mark.parametrize(
    "base,text",
    [
        (haem.RNABase.ADENINE, "adenine"),
        (haem.RNABase.CYTOSINE, "cytosine"),
        (haem.RNABase.GUANINE, "guanine"),
        (haem.RNABase.URACIL, "uracil"),
        (haem.RNABase.ADENINE_CYTOSINE, "adenine/cytosine"),
        (haem.RNABase.ADENINE_GUANINE, "adenine/guanine"),
        (haem.RNABase.ADENINE_URACIL, "adenine/uracil"),
        (haem.RNABase.CYTOSINE_GUANINE, "cytosine/guanine"),
        (haem.RNABase.CYTOSINE_URACIL, "cytosine/uracil"),
        (haem.RNABase.GUANINE_URACIL, "guanine/uracil"),
        (haem.RNABase.ADENINE_CYTOSINE_GUANINE, "adenine/cytosine/guanine"),
        (haem.RNABase.ADENINE_CYTOSINE_URACIL, "adenine/cytosine/uracil"),
        (haem.RNABase.ADENINE_GUANINE_URACIL, "adenine/guanine/uracil"),
        (haem.RNABase.CYTOSINE_GUANINE_URACIL, "cytosine/guanine/uracil"),
        (haem.RNABase.ANY, "any"),
        (haem.RNABase.GAP, "gap"),
    ],
)
def test__str__(base: haem.RNABase, text: str) -> None:
    assert str(base) == text


@pytest.mark.parametrize(
    "base,code",
    [
        (haem.RNABase.ADENINE, "A"),
        (haem.RNABase.CYTOSINE, "C"),
        (haem.RNABase.GUANINE, "G"),
        (haem.RNABase.URACIL, "U"),
        (haem.RNABase.ADENINE_CYTOSINE, "M"),
        (haem.RNABase.ADENINE_GUANINE, "R"),
        (haem.RNABase.ADENINE_URACIL, "W"),
        (haem.RNABase.CYTOSINE_GUANINE, "S"),
        (haem.RNABase.CYTOSINE_URACIL, "Y"),
        (haem.RNABase.GUANINE_URACIL, "K"),
        (haem.RNABase.ADENINE_CYTOSINE_GUANINE, "V"),
        (haem.RNABase.ADENINE_CYTOSINE_URACIL, "H"),
        (haem.RNABase.ADENINE_GUANINE_URACIL, "D"),
        (haem.RNABase.CYTOSINE_GUANINE_URACIL, "B"),
        (haem.RNABase.ANY, "N"),
        (haem.RNABase.GAP, "-"),
    ],
)
def test_code(base: haem.RNABase, code: str) -> None:
    assert base.code == code


@pytest.mark.parametrize(
    "base,complement",
    [
        (haem.RNABase.ADENINE, haem.RNABase.URACIL),
        (haem.RNABase.CYTOSINE, haem.RNABase.GUANINE),
        (haem.RNABase.GUANINE, haem.RNABase.CYTOSINE),
        (haem.RNABase.URACIL, haem.RNABase.ADENINE),
        (haem.RNABase.ADENINE_CYTOSINE, haem.RNABase.GUANINE_URACIL),
        (haem.RNABase.ADENINE_GUANINE, haem.RNABase.CYTOSINE_URACIL),
        (haem.RNABase.ADENINE_URACIL, haem.RNABase.ADENINE_URACIL),
        (haem.RNABase.CYTOSINE_GUANINE, haem.RNABase.CYTOSINE_GUANINE),
        (haem.RNABase.CYTOSINE_URACIL, haem.RNABase.ADENINE_GUANINE),
        (haem.RNABase.GUANINE_URACIL, haem.RNABase.ADENINE_CYTOSINE),
        (haem.RNABase.ADENINE_CYTOSINE_GUANINE, haem.RNABase.CYTOSINE_GUANINE_URACIL),
        (haem.RNABase.ADENINE_CYTOSINE_URACIL, haem.RNABase.ADENINE_GUANINE_URACIL),
        (haem.RNABase.ADENINE_GUANINE_URACIL, haem.RNABase.ADENINE_CYTOSINE_URACIL),
        (haem.RNABase.CYTOSINE_GUANINE_URACIL, haem.RNABase.ADENINE_CYTOSINE_GUANINE),
        (haem.RNABase.ANY, haem.RNABase.ANY),
        (haem.RNABase.GAP, haem.RNABase.GAP),
    ],
)
def test_complement(base: haem.RNABase, complement: haem.RNABase) -> None:
    assert base.complement == complement
    assert ~base == complement


@pytest.mark.parametrize(
    "rna_base,dna_base",
    [
        (haem.RNABase.ADENINE, haem.DNABase.ADENINE),
        (haem.RNABase.CYTOSINE, haem.DNABase.CYTOSINE),
        (haem.RNABase.GUANINE, haem.DNABase.GUANINE),
        (haem.RNABase.URACIL, haem.DNABase.THYMINE),
        (haem.RNABase.ADENINE_CYTOSINE, haem.DNABase.ADENINE_CYTOSINE),
        (haem.RNABase.ADENINE_GUANINE, haem.DNABase.ADENINE_GUANINE),
        (haem.RNABase.ADENINE_URACIL, haem.DNABase.ADENINE_THYMINE),
        (haem.RNABase.CYTOSINE_GUANINE, haem.DNABase.CYTOSINE_GUANINE),
        (haem.RNABase.CYTOSINE_URACIL, haem.DNABase.CYTOSINE_THYMINE),
        (haem.RNABase.GUANINE_URACIL, haem.DNABase.GUANINE_THYMINE),
        (haem.RNABase.ADENINE_CYTOSINE_GUANINE, haem.DNABase.ADENINE_CYTOSINE_GUANINE),
        (haem.RNABase.ADENINE_CYTOSINE_URACIL, haem.DNABase.ADENINE_CYTOSINE_THYMINE),
        (haem.RNABase.ADENINE_GUANINE_URACIL, haem.DNABase.ADENINE_GUANINE_THYMINE),
        (haem.RNABase.CYTOSINE_GUANINE_URACIL, haem.DNABase.CYTOSINE_GUANINE_THYMINE),
        (haem.RNABase.ANY, haem.DNABase.ANY),
        (haem.RNABase.GAP, haem.DNABase.GAP),
    ],
)
def test_retro_transcribe(rna_base: haem.RNABase, dna_base: haem.DNABase) -> None:
    assert rna_base.retro_transcribe() == dna_base


def test__eq__() -> None:
    assert haem.RNABase.ADENINE == haem.RNABase.ADENINE


def test__ne__() -> None:
    assert haem.RNABase.ADENINE != haem.RNABase.CYTOSINE


@pytest.mark.parametrize("op", [operator.gt, operator.ge, operator.lt, operator.le])
def test_unsupported_comparison(
    op: typing.Callable[[haem.RNABase, haem.RNABase], bool],
) -> None:
    with pytest.raises(TypeError):
        op(haem.RNABase.ADENINE, haem.RNABase.CYTOSINE)


@pytest.mark.parametrize(
    "base,result", [(haem.RNABase.ADENINE, True), (haem.RNABase.GAP, False)]
)
def test__bool__(base: haem.RNABase, result: bool) -> None:
    assert bool(base) == result


@pytest.mark.parametrize(
    "left,right,result",
    [
        (haem.RNABase.ADENINE, haem.RNABase("-"), haem.RNASequence("A-")),
        (haem.RNABase.ADENINE, haem.RNASequence("CUU"), haem.RNASequence("ACUU")),
        (haem.RNABase.ADENINE, haem.RNASequence(), haem.RNASequence("A")),
        (haem.RNABase.ADENINE, "-", haem.RNASequence("A-")),
        (haem.RNABase.ADENINE, "", haem.RNASequence("A")),
    ],
)
def test__add__(
    left: haem.RNABase,
    right: typing.Union[haem.RNABase, haem.RNASequence, str],
    result: haem.RNASequence,
) -> None:
    assert left + right == result


@pytest.mark.parametrize(
    "left,right,result",
    [
        (haem.RNASequence("CUU"), haem.RNABase("A"), haem.RNASequence("CUUA")),
        (haem.RNASequence(), haem.RNABase("A"), haem.RNASequence("A")),
        ("C", haem.RNABase("A"), haem.RNASequence("CA")),
        ("", haem.RNABase("A"), haem.RNASequence("A")),
    ],
)
def test__radd__(
    left: typing.Union[haem.RNASequence, str],
    right: haem.RNABase,
    result: haem.RNASequence,
) -> None:
    assert left + right == result
