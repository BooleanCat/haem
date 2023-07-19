import operator
import typing

import pytest

import haem


def test__new__empty() -> None:
    haem.DNASequence()


def test__new__str() -> None:
    haem.DNASequence("ACGT")


def test__new__iterable_base_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence(
            iter(
                [
                    haem.DNABase.ADENINE,
                    haem.DNABase.CYTOSINE,
                    haem.DNABase.GUANINE,
                    haem.DNABase.THYMINE,
                ]
            )
        )


def test__new__iterable_str_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence(iter(["A", "C", "G", "T"]))


def test_new__sequence_bases() -> None:
    haem.DNASequence(
        [
            haem.DNABase.ADENINE,
            haem.DNABase.CYTOSINE,
            haem.DNABase.GUANINE,
            haem.DNABase.THYMINE,
        ]
    )


def test__new__sequence_str_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence(["A", "C", "G", "T"])


def test_complement_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence().complement


def test_transcribe_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence().transcribe()


def test__invert__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        ~haem.DNASequence()


@pytest.mark.parametrize(
    "bases,text",
    [
        ([], "<DNASequence>"),
        ([haem.DNABase.ADENINE], "<DNASequence: A>"),
        ([haem.DNABase.ADENINE, haem.DNABase.ADENINE_CYTOSINE], "<DNASequence: AM>"),
        ([haem.DNABase.ADENINE for _ in range(100)], f"<DNASequence: {'A' * 100}>"),
    ],
)
def test__repr__(bases: list[haem.DNABase], text: str) -> None:
    assert repr(haem.DNASequence(bases)) == text


@pytest.mark.parametrize(
    "bases,text",
    [
        ([], ""),
        ([haem.DNABase.ADENINE], "A"),
        ([haem.DNABase.ADENINE, haem.DNABase.ADENINE_CYTOSINE], "AM"),
        ([haem.DNABase.ADENINE for _ in range(20)], "A" * 20),
        (
            [haem.DNABase.ADENINE for _ in range(21)] + [haem.DNABase.GUANINE],
            f"{'A' * 10}...{'A' * 9}G",
        ),
    ],
)
def test__str__(bases: list[haem.DNABase], text: str) -> None:
    assert str(haem.DNASequence(bases)) == text


@pytest.mark.parametrize("bases", [[], [haem.DNABase.ADENINE, haem.DNABase.GUANINE]])
def test__eq__(bases: list[haem.DNABase]) -> None:
    assert haem.DNASequence(bases) == haem.DNASequence(bases)


def test__ne__() -> None:
    assert haem.DNASequence(
        [haem.DNABase.ADENINE, haem.DNABase.GUANINE]
    ) != haem.DNASequence([haem.DNABase.ADENINE, haem.DNABase.THYMINE])


@pytest.mark.parametrize("op", [operator.gt, operator.ge, operator.lt, operator.le])
def test_unsupported_comparison(
    op: typing.Callable[[haem.DNASequence, haem.DNASequence], bool]
) -> None:
    with pytest.raises(TypeError):
        op(haem.DNASequence(), haem.DNASequence())


@pytest.mark.parametrize("bases,result", [([], False), ([haem.DNABase.ADENINE], True)])
def test__bool__(bases: list[haem.DNABase], result: bool) -> None:
    assert bool(haem.DNASequence(bases)) is result


def test__add__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence() + haem.DNASequence()


def test__contains__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence() in haem.DNASequence()


@pytest.mark.parametrize("bases,length", [([], 0), ([haem.DNABase.ADENINE], 1)])
def test__len__(bases: list[haem.DNABase], length: int) -> None:
    assert len(haem.DNASequence(bases)) == length


def test__getitem__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence()[0]


def test__iter__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        iter(haem.DNASequence())


def test_count_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence().count(haem.DNABase.ADENINE)
