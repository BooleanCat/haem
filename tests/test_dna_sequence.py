import operator
import typing

import pytest

import haem


def test__new__empty() -> None:
    haem.DNASequence()


def test__new__str() -> None:
    assert haem.DNASequence("ACGT") == haem.DNASequence(
        [
            haem.DNABase.ADENINE,
            haem.DNABase.CYTOSINE,
            haem.DNABase.GUANINE,
            haem.DNABase.THYMINE,
        ]
    )


def test__new__str__invalid() -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.DNASequence("ACGTX")

    assert str(excinfo.value) == 'invalid IUPAC DNA code "X"'


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


@pytest.mark.parametrize(
    "sequence,complement",
    [
        (haem.DNASequence(""), haem.DNASequence("")),
        (haem.DNASequence("A"), haem.DNASequence("T")),
        (haem.DNASequence("ACGT"), haem.DNASequence("TGCA")),
    ],
)
def test_complement(sequence: haem.DNASequence, complement: haem.DNASequence) -> None:
    assert sequence.complement == complement
    assert ~sequence == complement


def test_transcribe_not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence().transcribe()


@pytest.mark.parametrize(
    "bases,text",
    [
        ([], "<DNASequence>"),
        ([haem.DNABase.ADENINE], "<DNASequence: A>"),
        ([haem.DNABase.ADENINE, haem.DNABase.ADENINE_CYTOSINE], "<DNASequence: AM>"),
        ([haem.DNABase.ADENINE for _ in range(100)], f"<DNASequence: {'A' * 100}>"),
    ],
)
def test__repr__(bases: typing.List[haem.DNABase], text: str) -> None:
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
def test__str__(bases: typing.List[haem.DNABase], text: str) -> None:
    assert str(haem.DNASequence(bases)) == text


@pytest.mark.parametrize("bases", [[], [haem.DNABase.ADENINE, haem.DNABase.GUANINE]])
def test__eq__(bases: typing.List[haem.DNABase]) -> None:
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
def test__bool__(bases: typing.List[haem.DNABase], result: bool) -> None:
    assert bool(haem.DNASequence(bases)) is result


@pytest.mark.parametrize(
    "left,right,result",
    [
        (haem.DNASequence("A-"), haem.DNABase.GUANINE, haem.DNASequence("A-G")),
        (haem.DNASequence("A-"), haem.DNASequence("CTT"), haem.DNASequence("A-CTT")),
        (haem.DNASequence("A-"), haem.DNASequence(""), haem.DNASequence("A-")),
        (haem.DNASequence(""), haem.DNASequence(""), haem.DNASequence("")),
        (haem.DNASequence(""), haem.DNABase.GUANINE, haem.DNASequence("G")),
    ],
)
def test__add__(
    left: haem.DNASequence,
    right: typing.Union[haem.DNABase, haem.DNASequence],
    result: haem.DNASequence,
) -> None:
    assert left + right == result


def test__contains__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNABase.ADENINE in haem.DNASequence("")


@pytest.mark.parametrize("bases,length", [([], 0), ([haem.DNABase.ADENINE], 1)])
def test__len__(bases: typing.List[haem.DNABase], length: int) -> None:
    assert len(haem.DNASequence(bases)) == length


def test__getitem__not_implemented() -> None:
    with pytest.raises(NotImplementedError):
        haem.DNASequence()[0]


@pytest.mark.parametrize(
    "bases", [[haem.DNABase("T")], [haem.DNABase("A"), haem.DNABase("C")], []]
)
def test__iter__(bases: typing.List[haem.DNABase]) -> None:
    sequence_iter = iter(haem.DNASequence(bases))

    for base in bases:
        assert next(sequence_iter) == base

    with pytest.raises(StopIteration):
        next(sequence_iter)


@pytest.mark.parametrize(
    "sequence,target,total",
    [
        (haem.DNASequence(""), haem.DNABase.ADENINE, 0),
        (haem.DNASequence("AGCG"), haem.DNABase.GUANINE, 2),
        (haem.DNASequence("AGCG"), "G", 2),
    ],
)
def test_count(
    sequence: haem.DNASequence, target: typing.Union[haem.DNABase, str], total: int
) -> None:
    assert sequence.count(target) == total
