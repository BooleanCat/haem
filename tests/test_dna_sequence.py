import operator
import typing

import pytest

import haem


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


def test__new__iterable_base() -> None:
    assert haem.DNASequence(
        iter(
            [
                haem.DNABase.ADENINE,
                haem.DNABase.CYTOSINE,
                haem.DNABase.GUANINE,
                haem.DNABase.THYMINE,
            ]
        )
    ) == haem.DNASequence("ACGT")


def test__new__iterable_str() -> None:
    assert haem.DNASequence(iter(["A", "C", "G", "T"])) == haem.DNASequence("ACGT")


def test__new__iterable_invalid() -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.DNASequence(iter(["A", "C", "G", "X"]))

    assert str(excinfo.value) == 'invalid IUPAC DNA code "X"'


def test__new__sequence_bases() -> None:
    assert haem.DNASequence(
        [
            haem.DNABase.ADENINE,
            haem.DNABase.CYTOSINE,
            haem.DNABase.GUANINE,
            haem.DNABase.THYMINE,
        ]
    ) == haem.DNASequence("ACGT")


def test__new__sequence_str() -> None:
    assert haem.DNASequence(["A", "C", "G", "T"]) == haem.DNASequence("ACGT")


@pytest.mark.parametrize(
    "sequence,complement",
    [
        (haem.DNASequence(), haem.DNASequence()),
        (haem.DNASequence("A"), haem.DNASequence("T")),
        (haem.DNASequence("ACGT"), haem.DNASequence("TGCA")),
    ],
)
def test_complement(sequence: haem.DNASequence, complement: haem.DNASequence) -> None:
    assert sequence.complement == complement
    assert ~sequence == complement


@pytest.mark.parametrize(
    "dna_sequence,rna_sequence",
    [
        (haem.DNASequence(), haem.RNASequence()),
        (haem.DNASequence("ACGT"), haem.RNASequence("ACGU")),
    ],
)
def test_transcribe(
    dna_sequence: haem.DNASequence, rna_sequence: haem.RNASequence
) -> None:
    assert dna_sequence.transcribe() == rna_sequence


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
        (haem.DNASequence("A-"), haem.DNASequence(), haem.DNASequence("A-")),
        (haem.DNASequence(), haem.DNASequence(), haem.DNASequence()),
        (haem.DNASequence(), haem.DNABase.GUANINE, haem.DNASequence("G")),
        (haem.DNASequence("A-"), "", haem.DNASequence("A-")),
        (haem.DNASequence("A-"), "GT", haem.DNASequence("A-GT")),
        (haem.DNASequence(), "", haem.DNASequence()),
        (haem.DNASequence(), "TG", haem.DNASequence("TG")),
    ],
)
def test__add__(
    left: haem.DNASequence,
    right: typing.Union[haem.DNABase, haem.DNASequence, str],
    result: haem.DNASequence,
) -> None:
    assert left + right == result


@pytest.mark.parametrize(
    "left,right,result",
    [
        (haem.DNABase("G"), haem.DNASequence("A-"), haem.DNASequence("GA-")),
        (haem.DNABase("G"), haem.DNASequence(), haem.DNASequence("G")),
        ("", haem.DNASequence("A-"), haem.DNASequence("A-")),
        ("GT", haem.DNASequence("A-"), haem.DNASequence("GTA-")),
        ("", haem.DNASequence(), haem.DNASequence()),
        ("TG", haem.DNASequence(), haem.DNASequence("TG")),
    ],
)
def test__radd__(
    left: typing.Union[haem.DNABase, str],
    right: haem.DNASequence,
    result: haem.DNASequence,
) -> None:
    assert left + right == result


@pytest.mark.parametrize(
    "sequence,target,result",
    [
        (haem.DNASequence("A"), haem.DNABase.ADENINE, True),
        (haem.DNASequence(), haem.DNABase.ADENINE, False),
        (haem.DNASequence("G"), haem.DNABase.ADENINE, False),
        (haem.DNASequence("AGCG"), haem.DNABase.GUANINE, True),
        (haem.DNASequence("AGCG"), haem.DNABase.THYMINE, False),
        (haem.DNASequence(), haem.DNASequence(), True),
        (haem.DNASequence("AGT"), haem.DNASequence("GT"), True),
        (haem.DNASequence("AGT"), haem.DNASequence("TG"), False),
        (haem.DNASequence("A"), haem.DNASequence("AA"), False),
    ],
)
def test__contains__(
    sequence: haem.DNASequence,
    target: typing.Union[haem.DNABase, haem.DNASequence],
    result: bool,
) -> None:
    assert (target in sequence) is result


@pytest.mark.parametrize("bases,length", [([], 0), ([haem.DNABase.ADENINE], 1)])
def test__len__(bases: typing.List[haem.DNABase], length: int) -> None:
    assert len(haem.DNASequence(bases)) == length


@pytest.mark.parametrize(
    "sequence,index,base",
    [
        (haem.DNASequence("GAT"), 0, haem.DNABase.GUANINE),
        (haem.DNASequence("GAT"), 1, haem.DNABase.ADENINE),
        (haem.DNASequence("GAT"), 2, haem.DNABase.THYMINE),
        (haem.DNASequence("GAT"), -1, haem.DNABase.THYMINE),
        (haem.DNASequence("GAT"), -2, haem.DNABase.ADENINE),
        (haem.DNASequence("GAT"), -3, haem.DNABase.GUANINE),
    ],
)
def test__getitem__index(
    sequence: haem.DNASequence, index: int, base: haem.DNABase
) -> None:
    assert sequence[index] == base


@pytest.mark.parametrize("index", [3, -4])
def test__getitem__index_out_of_range(index: int) -> None:
    with pytest.raises(IndexError) as excinfo:
        haem.DNASequence("GAT")[index]

    assert str(excinfo.value) == "DNASequence index out of range"


@pytest.mark.parametrize(
    "sequence,slic,result",
    [
        (haem.DNASequence(), slice(0, 0), haem.DNASequence()),
        (haem.DNASequence("GAT"), slice(0, 2), haem.DNASequence("GA")),
        (haem.DNASequence("GAT"), slice(1, 3), haem.DNASequence("AT")),
        (haem.DNASequence("GAT"), slice(0, 3), haem.DNASequence("GAT")),
        (haem.DNASequence("GAT"), slice(0, 4), haem.DNASequence("GAT")),
        (haem.DNASequence("GAT"), slice(0, None), haem.DNASequence("GAT")),
        (haem.DNASequence("GAT"), slice(1, None), haem.DNASequence("AT")),
        (haem.DNASequence("GAT"), slice(-1, None), haem.DNASequence("T")),
        (haem.DNASequence("GAT"), slice(-3, None), haem.DNASequence("GAT")),
        (haem.DNASequence("GAT"), slice(-4, None), haem.DNASequence("GAT")),
        (haem.DNASequence("GAT"), slice(0, -1), haem.DNASequence("GA")),
        (haem.DNASequence("GATCCA"), slice(0, -1, 2), haem.DNASequence("GTC")),
        (haem.DNASequence("GATCCA"), slice(5, None, -1), haem.DNASequence("ACCTAG")),
        (haem.DNASequence("GATCCA"), slice(None, None, -1), haem.DNASequence("ACCTAG")),
        (haem.DNASequence("GATCCA"), slice(10, 2, -2), haem.DNASequence("AC")),
    ],
)
def test__getitem__slice(
    sequence: haem.DNASequence, slic: slice, result: haem.DNASequence
) -> None:
    assert sequence[slic.start : slic.stop : slic.step] == result


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
        (haem.DNASequence(), haem.DNABase.ADENINE, 0),
        (haem.DNASequence("AGCG"), haem.DNABase.GUANINE, 2),
        (haem.DNASequence("AGCG"), "G", 2),
    ],
)
def test_count(
    sequence: haem.DNASequence, target: typing.Union[haem.DNABase, str], total: int
) -> None:
    assert sequence.count(target) == total
