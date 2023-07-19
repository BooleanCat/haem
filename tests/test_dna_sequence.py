import operator
import typing

import pytest

import haem


def test__new__empty() -> None:
    haem.DNASequence()


def test__new__str() -> None:
    haem.DNASequence("ACGT")


def test__new__iterable_base() -> None:
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


def test__new__iterable_str() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence(iter(["A", "C", "G", "T"]))


def test_new__sequence_bases() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence(
            [
                haem.DNABase.ADENINE,
                haem.DNABase.CYTOSINE,
                haem.DNABase.GUANINE,
                haem.DNABase.THYMINE,
            ]
        )


def test__new__sequence_str() -> None:
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


def test__repr__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        repr(haem.DNASequence())


def test__str__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        str(haem.DNASequence())


def test__eq__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence() == haem.DNASequence()


def test__ne__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence() != haem.DNASequence()


@pytest.mark.parametrize("op", [operator.gt, operator.ge, operator.lt, operator.le])
def test_unsupported_comparison(
    op: typing.Callable[[haem.DNASequence, haem.DNASequence], bool]
) -> None:
    with pytest.raises(TypeError):
        op(haem.DNASequence(), haem.DNASequence())


def test__bool__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        bool(haem.DNASequence())


def test__add__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence() + haem.DNASequence()


def test__contains__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence() in haem.DNASequence()


def test__len__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        len(haem.DNASequence())


def test__getitem__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence()[0]


def test__iter__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        iter(haem.DNASequence())


def test_count_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence().count(haem.DNABase.ADENINE)
