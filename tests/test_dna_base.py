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


@pytest.mark.parametrize(
    "base,code",
    [
        (haem.DNABase.ADENINE, "A"),
        (haem.DNABase.CYTOSINE, "C"),
        (haem.DNABase.GUANINE, "G"),
        (haem.DNABase.THYMINE, "T"),
        (haem.DNABase.ADENINE_CYTOSINE, "M"),
        (haem.DNABase.ADENINE_GUANINE, "R"),
        (haem.DNABase.ADENINE_THYMINE, "W"),
        (haem.DNABase.CYTOSINE_GUANINE, "S"),
        (haem.DNABase.CYTOSINE_THYMINE, "Y"),
        (haem.DNABase.GUANINE_THYMINE, "K"),
        (haem.DNABase.ADENINE_CYTOSINE_GUANINE, "V"),
        (haem.DNABase.ADENINE_CYTOSINE_THYMINE, "H"),
        (haem.DNABase.ADENINE_GUANINE_THYMINE, "D"),
        (haem.DNABase.CYTOSINE_GUANINE_THYMINE, "B"),
        (haem.DNABase.ANY, "N"),
        (haem.DNABase.GAP, "-"),
    ],
)
def test_code(base: haem.DNABase, code: str) -> None:
    assert base.code == code


@pytest.mark.parametrize(
    "base,complement",
    [
        (haem.DNABase.ADENINE, haem.DNABase.THYMINE),
        (haem.DNABase.CYTOSINE, haem.DNABase.GUANINE),
        (haem.DNABase.GUANINE, haem.DNABase.CYTOSINE),
        (haem.DNABase.THYMINE, haem.DNABase.ADENINE),
        (haem.DNABase.ADENINE_CYTOSINE, haem.DNABase.GUANINE_THYMINE),
        (haem.DNABase.ADENINE_GUANINE, haem.DNABase.CYTOSINE_THYMINE),
        (haem.DNABase.ADENINE_THYMINE, haem.DNABase.ADENINE_THYMINE),
        (haem.DNABase.CYTOSINE_GUANINE, haem.DNABase.CYTOSINE_GUANINE),
        (haem.DNABase.CYTOSINE_THYMINE, haem.DNABase.ADENINE_GUANINE),
        (haem.DNABase.GUANINE_THYMINE, haem.DNABase.ADENINE_CYTOSINE),
        (haem.DNABase.ADENINE_CYTOSINE_GUANINE, haem.DNABase.CYTOSINE_GUANINE_THYMINE),
        (haem.DNABase.ADENINE_CYTOSINE_THYMINE, haem.DNABase.ADENINE_GUANINE_THYMINE),
        (haem.DNABase.ADENINE_GUANINE_THYMINE, haem.DNABase.ADENINE_CYTOSINE_THYMINE),
        (haem.DNABase.CYTOSINE_GUANINE_THYMINE, haem.DNABase.ADENINE_CYTOSINE_GUANINE),
        (haem.DNABase.ANY, haem.DNABase.ANY),
        (haem.DNABase.GAP, haem.DNABase.GAP),
    ],
)
def test_complement(base: haem.DNABase, complement: haem.DNABase) -> None:
    assert base.complement == complement
    assert ~base == complement


@pytest.mark.parametrize(
    "dna_base,rna_base",
    [
        (haem.DNABase.ADENINE, haem.RNABase.ADENINE),
        (haem.DNABase.CYTOSINE, haem.RNABase.CYTOSINE),
        (haem.DNABase.GUANINE, haem.RNABase.GUANINE),
        (haem.DNABase.THYMINE, haem.RNABase.URACIL),
        (haem.DNABase.ADENINE_CYTOSINE, haem.RNABase.ADENINE_CYTOSINE),
        (haem.DNABase.ADENINE_GUANINE, haem.RNABase.ADENINE_GUANINE),
        (haem.DNABase.ADENINE_THYMINE, haem.RNABase.ADENINE_URACIL),
        (haem.DNABase.CYTOSINE_GUANINE, haem.RNABase.CYTOSINE_GUANINE),
        (haem.DNABase.CYTOSINE_THYMINE, haem.RNABase.CYTOSINE_URACIL),
        (haem.DNABase.GUANINE_THYMINE, haem.RNABase.GUANINE_URACIL),
        (haem.DNABase.ADENINE_CYTOSINE_GUANINE, haem.RNABase.ADENINE_CYTOSINE_GUANINE),
        (haem.DNABase.ADENINE_CYTOSINE_THYMINE, haem.RNABase.ADENINE_CYTOSINE_URACIL),
        (haem.DNABase.ADENINE_GUANINE_THYMINE, haem.RNABase.ADENINE_GUANINE_URACIL),
        (haem.DNABase.CYTOSINE_GUANINE_THYMINE, haem.RNABase.CYTOSINE_GUANINE_URACIL),
        (haem.DNABase.ANY, haem.RNABase.ANY),
        (haem.DNABase.GAP, haem.RNABase.GAP),
    ],
)
def test_transcribe(dna_base: haem.DNABase, rna_base: haem.RNABase) -> None:
    assert dna_base.transcribe() == rna_base


def test__eq__() -> None:
    assert haem.DNABase.ADENINE == haem.DNABase.ADENINE


def test__ne__() -> None:
    assert haem.DNABase.ADENINE != haem.DNABase.CYTOSINE


@pytest.mark.parametrize("op", [operator.gt, operator.ge, operator.lt, operator.le])
def test_unsupported_comparison(
    op: typing.Callable[[haem.DNABase, haem.DNABase], bool],
) -> None:
    with pytest.raises(TypeError):
        op(haem.DNABase.ADENINE, haem.DNABase.CYTOSINE)


@pytest.mark.parametrize(
    "base,result", [(haem.DNABase.ADENINE, True), (haem.DNABase.GAP, False)]
)
def test__bool__(base: haem.DNABase, result: bool) -> None:
    assert bool(base) == result


@pytest.mark.parametrize(
    "left,right,result",
    [
        (haem.DNABase("A"), haem.DNABase("-"), haem.DNASequence("A-")),
        (haem.DNABase("A"), haem.DNASequence("CTT"), haem.DNASequence("ACTT")),
        (haem.DNABase("A"), haem.DNASequence(), haem.DNASequence("A")),
        (haem.DNABase("A"), "", haem.DNASequence("A")),
        (haem.DNABase("A"), haem.DNASequence("T"), haem.DNASequence("AT")),
        (haem.DNABase("A"), iter("ACG"), haem.DNASequence("AACG")),
        (haem.DNABase("A"), ["A", "C", "G"], haem.DNASequence("AACG")),
        (
            haem.DNABase.ADENINE,
            [haem.DNABase.ADENINE, haem.DNABase.THYMINE],
            haem.DNASequence("AAT"),
        ),
        (
            haem.DNABase.ADENINE,
            iter([haem.DNABase.ADENINE, haem.DNABase.THYMINE]),
            haem.DNASequence("AAT"),
        ),
    ],
)
def test__add__(
    left: haem.DNABase,
    right: typing.Union[
        haem.DNABase,
        haem.DNASequence,
        typing.Iterator[str],
        typing.Sequence[str],
        str,
    ],
    result: haem.DNASequence,
) -> None:
    assert left + right == result


@pytest.mark.parametrize(
    "left,right,result",
    [
        (haem.DNASequence("CTT"), haem.DNABase("A"), haem.DNASequence("CTTA")),
        (haem.DNASequence(), haem.DNABase("A"), haem.DNASequence("A")),
        ("", haem.DNABase("A"), haem.DNASequence("A")),
        ("T", haem.DNABase("A"), haem.DNASequence("TA")),
        (iter("ACG"), haem.DNABase("A"), haem.DNASequence("ACGA")),
        (["A", "C", "G"], haem.DNABase("A"), haem.DNASequence("ACGA")),
        (
            [haem.DNABase.ADENINE, haem.DNABase.THYMINE],
            haem.DNABase.ADENINE,
            haem.DNASequence("ATA"),
        ),
        (
            iter([haem.DNABase.ADENINE, haem.DNABase.THYMINE]),
            haem.DNABase.ADENINE,
            haem.DNASequence("ATA"),
        ),
    ],
)
def test__radd__(
    left: typing.Union[haem.DNASequence, str],
    right: haem.DNABase,
    result: haem.DNASequence,
) -> None:
    assert left + right == result
