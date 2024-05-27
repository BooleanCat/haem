import operator
import typing

import pytest

import haem


def test__new__str() -> None:
    assert haem.RNASequence("ACGU") == haem.RNASequence(
        [
            haem.RNABase.ADENINE,
            haem.RNABase.CYTOSINE,
            haem.RNABase.GUANINE,
            haem.RNABase.URACIL,
        ]
    )


def test__new__str__invalid() -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.RNASequence("ACGUX")

    assert str(excinfo.value) == 'invalid IUPAC RNA code "X"'


def test__new__iterable_base() -> None:
    assert haem.RNASequence(
        iter(
            [
                haem.RNABase.ADENINE,
                haem.RNABase.CYTOSINE,
                haem.RNABase.GUANINE,
                haem.RNABase.URACIL,
            ]
        )
    ) == haem.RNASequence("ACGU")


def test__new__iterable_str() -> None:
    assert haem.RNASequence(iter(["A", "C", "G", "U"])) == haem.RNASequence("ACGU")


def test__new__iterable_invalid() -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.RNASequence(iter(["A", "C", "G", "X"]))

    assert str(excinfo.value) == 'invalid IUPAC RNA code "X"'


def test__new__sequence_bases() -> None:
    assert haem.RNASequence(
        [
            haem.RNABase.ADENINE,
            haem.RNABase.CYTOSINE,
            haem.RNABase.GUANINE,
            haem.RNABase.URACIL,
        ]
    ) == haem.RNASequence("ACGU")


def test__new__sequence_str() -> None:
    assert haem.RNASequence(["A", "C", "G", "U"]) == haem.RNASequence("ACGU")


@pytest.mark.parametrize(
    "sequence,complement",
    [
        (haem.RNASequence(), haem.RNASequence()),
        (haem.RNASequence("A"), haem.RNASequence("U")),
        (haem.RNASequence("ACGU"), haem.RNASequence("UGCA")),
    ],
)
def test_complement(sequence: haem.RNASequence, complement: haem.RNASequence) -> None:
    assert sequence.complement == complement
    assert ~sequence == complement


@pytest.mark.parametrize(
    "rna_sequence,dna_sequence",
    [
        (haem.RNASequence(), haem.DNASequence("")),
        (haem.RNASequence("ACGU"), haem.DNASequence("ACGT")),
    ],
)
def test_retro_transcribe(
    rna_sequence: haem.RNASequence, dna_sequence: haem.DNASequence
) -> None:
    assert rna_sequence.retro_transcribe() == dna_sequence


@pytest.mark.parametrize(
    "bases,text",
    [
        ([], "<RNASequence>"),
        ([haem.RNABase.ADENINE], "<RNASequence: A>"),
        ([haem.RNABase.ADENINE, haem.RNABase.ADENINE_CYTOSINE], "<RNASequence: AM>"),
        ([haem.RNABase.ADENINE for _ in range(100)], f"<RNASequence: {'A' * 100}>"),
    ],
)
def test__repr__(bases: typing.List[haem.RNABase], text: str) -> None:
    assert repr(haem.RNASequence(bases)) == text


@pytest.mark.parametrize(
    "bases,text",
    [
        ([], ""),
        ([haem.RNABase.ADENINE], "A"),
        ([haem.RNABase.ADENINE, haem.RNABase.ADENINE_CYTOSINE], "AM"),
        ([haem.RNABase.ADENINE for _ in range(20)], "A" * 20),
        (
            [haem.RNABase.ADENINE for _ in range(21)] + [haem.RNABase.GUANINE],
            f"{'A' * 10}...{'A' * 9}G",
        ),
    ],
)
def test__str__(bases: typing.List[haem.RNABase], text: str) -> None:
    assert str(haem.RNASequence(bases)) == text


@pytest.mark.parametrize("bases", [[], [haem.RNABase.ADENINE, haem.RNABase.GUANINE]])
def test__eq__(bases: typing.List[haem.RNABase]) -> None:
    assert haem.RNASequence(bases) == haem.RNASequence(bases)


def test__ne__() -> None:
    assert haem.RNASequence(
        [haem.RNABase.ADENINE, haem.RNABase.GUANINE]
    ) != haem.RNASequence([haem.RNABase.ADENINE, haem.RNABase.URACIL])


@pytest.mark.parametrize("op", [operator.gt, operator.ge, operator.lt, operator.le])
def test_unsupported_comparison(
    op: typing.Callable[[haem.RNASequence, haem.RNASequence], bool],
) -> None:
    with pytest.raises(TypeError):
        op(haem.RNASequence(), haem.RNASequence())


@pytest.mark.parametrize("bases,result", [([], False), ([haem.RNABase.ADENINE], True)])
def test__bool__(bases: typing.List[haem.RNABase], result: bool) -> None:
    assert bool(haem.RNASequence(bases)) is result


@pytest.mark.parametrize(
    "left,right,result",
    [
        (haem.RNASequence("A-"), haem.RNABase.GUANINE, haem.RNASequence("A-G")),
        (haem.RNASequence("A-"), haem.RNASequence("CUU"), haem.RNASequence("A-CUU")),
        (haem.RNASequence("A-"), haem.RNASequence(), haem.RNASequence("A-")),
        (haem.RNASequence(), haem.RNASequence(), haem.RNASequence()),
        (haem.RNASequence(), haem.RNABase.GUANINE, haem.RNASequence("G")),
        (haem.RNASequence("A-"), "", haem.RNASequence("A-")),
        (haem.RNASequence("A-"), "GU", haem.RNASequence("A-GU")),
        (haem.RNASequence(), "", haem.RNASequence()),
        (haem.RNASequence(), "UG", haem.RNASequence("UG")),
    ],
)
def test__add__(
    left: haem.RNASequence,
    right: typing.Union[haem.RNABase, haem.RNASequence, str],
    result: haem.RNASequence,
) -> None:
    assert left + right == result


@pytest.mark.parametrize(
    "left,right,result",
    [
        (haem.RNABase.GUANINE, haem.RNASequence("A-"), haem.RNASequence("GA-")),
        (haem.RNABase.GUANINE, haem.RNASequence(), haem.RNASequence("G")),
        ("", haem.RNASequence("A-"), haem.RNASequence("A-")),
        ("GU", haem.RNASequence("A-"), haem.RNASequence("GUA-")),
        ("", haem.RNASequence(), haem.RNASequence()),
        ("UG", haem.RNASequence(), haem.RNASequence("UG")),
    ],
)
def test__radd__(
    left: typing.Union[haem.RNABase, str],
    right: haem.RNASequence,
    result: haem.RNASequence,
) -> None:
    assert left + right == result


@pytest.mark.parametrize(
    "sequence,target,result",
    [
        (haem.RNASequence("A"), haem.RNABase.ADENINE, True),
        (haem.RNASequence(), haem.RNABase.ADENINE, False),
        (haem.RNASequence("G"), haem.RNABase.ADENINE, False),
        (haem.RNASequence("AGCG"), haem.RNABase.GUANINE, True),
        (haem.RNASequence("AGCG"), haem.RNABase.URACIL, False),
        (haem.RNASequence(), haem.RNASequence(), True),
        (haem.RNASequence("AGU"), haem.RNASequence("GU"), True),
        (haem.RNASequence("AGU"), haem.RNASequence("UG"), False),
        (haem.RNASequence("A"), haem.RNASequence("AA"), False),
        (haem.RNASequence(), "", True),
        (haem.RNASequence("AGU"), "GU", True),
        (haem.RNASequence("AGU"), "UG", False),
        (haem.RNASequence("A"), "AA", False),
    ],
)
def test__contains__(
    sequence: haem.RNASequence,
    target: typing.Union[haem.RNABase, haem.RNASequence],
    result: bool,
) -> None:
    assert (target in sequence) is result


@pytest.mark.parametrize("bases,length", [([], 0), ([haem.RNABase.ADENINE], 1)])
def test__len__(bases: typing.List[haem.RNABase], length: int) -> None:
    assert len(haem.RNASequence(bases)) == length


@pytest.mark.parametrize(
    "sequence,index,base",
    [
        (haem.RNASequence("GAU"), 0, haem.RNABase.GUANINE),
        (haem.RNASequence("GAU"), 1, haem.RNABase.ADENINE),
        (haem.RNASequence("GAU"), 2, haem.RNABase.URACIL),
        (haem.RNASequence("GAU"), -1, haem.RNABase.URACIL),
        (haem.RNASequence("GAU"), -2, haem.RNABase.ADENINE),
        (haem.RNASequence("GAU"), -3, haem.RNABase.GUANINE),
    ],
)
def test__getitem__index(
    sequence: haem.RNASequence, index: int, base: haem.RNABase
) -> None:
    assert sequence[index] == base


@pytest.mark.parametrize("index", [3, -4])
def test__getitem__index_out_of_range(index: int) -> None:
    with pytest.raises(IndexError) as excinfo:
        haem.RNASequence("GAU")[index]

    assert str(excinfo.value) == "RNASequence index out of range"


@pytest.mark.parametrize(
    "sequence,slic,result",
    [
        (haem.RNASequence(), slice(0, 0), haem.RNASequence()),
        (haem.RNASequence("GAU"), slice(0, 2), haem.RNASequence("GA")),
        (haem.RNASequence("GAU"), slice(1, 3), haem.RNASequence("AU")),
        (haem.RNASequence("GAU"), slice(0, 3), haem.RNASequence("GAU")),
        (haem.RNASequence("GAU"), slice(0, 4), haem.RNASequence("GAU")),
        (haem.RNASequence("GAU"), slice(0, None), haem.RNASequence("GAU")),
        (haem.RNASequence("GAU"), slice(1, None), haem.RNASequence("AU")),
        (haem.RNASequence("GAU"), slice(-1, None), haem.RNASequence("U")),
        (haem.RNASequence("GAU"), slice(-3, None), haem.RNASequence("GAU")),
        (haem.RNASequence("GAU"), slice(-4, None), haem.RNASequence("GAU")),
        (haem.RNASequence("GAU"), slice(0, -1), haem.RNASequence("GA")),
        (haem.RNASequence("GAUCCA"), slice(0, -1, 2), haem.RNASequence("GUC")),
        (haem.RNASequence("GAUCCA"), slice(5, None, -1), haem.RNASequence("ACCUAG")),
        (haem.RNASequence("GAUCCA"), slice(None, None, -1), haem.RNASequence("ACCUAG")),
        (haem.RNASequence("GAUCCA"), slice(10, 2, -2), haem.RNASequence("AC")),
    ],
)
def test__getitem__slice(
    sequence: haem.RNASequence, slic: slice, result: haem.RNASequence
) -> None:
    assert sequence[slic.start : slic.stop : slic.step] == result


@pytest.mark.parametrize(
    "bases", [[haem.RNABase("U")], [haem.RNABase("U"), haem.RNABase("C")], []]
)
def test__iter__(bases: typing.List[haem.RNABase]) -> None:
    sequence_iter = iter(haem.RNASequence(bases))

    for base in bases:
        assert next(sequence_iter) == base

    with pytest.raises(StopIteration):
        next(sequence_iter)


@pytest.mark.parametrize(
    "sequence,target,total",
    [
        (haem.RNASequence(), haem.RNABase.ADENINE, 0),
        (haem.RNASequence("AGCG"), haem.RNABase.GUANINE, 2),
        (haem.RNASequence("AGCG"), "G", 2),
    ],
)
def test_count(
    sequence: haem.RNASequence, target: typing.Union[haem.RNABase, str], total: int
) -> None:
    assert sequence.count(target) == total


@pytest.mark.parametrize(
    "sequence,target,total",
    [
        (haem.RNASequence(), haem.RNASequence("GA"), 0),
        (haem.RNASequence("GAUC"), haem.RNASequence("AU"), 1),
        (haem.RNASequence("GAGA"), haem.RNASequence("GA"), 2),
        (haem.RNASequence(), "GA", 0),
        (haem.RNASequence("GAUC"), "AU", 1),
        (haem.RNASequence("AUAU"), "AU", 2),
        (haem.RNASequence(), haem.RNASequence(), 0),
        (haem.RNASequence("GUC"), haem.RNASequence(), 0),
        (haem.RNASequence(), "", 0),
        (haem.RNASequence("GAUC"), "", 0),
    ],
)
def test_count_sequence(
    sequence: haem.RNASequence,
    target: typing.Union[haem.RNASequence, str],
    total: int,
) -> None:
    assert sequence.count(target) == total


@pytest.mark.parametrize(
    "sequence,target,total",
    [
        (haem.RNASequence("GAAA"), haem.RNASequence("AA"), 2),
        (haem.RNASequence("GAAG"), haem.RNASequence("AA"), 1),
    ],
)
def test_count_overlap(
    sequence: haem.RNASequence,
    target: typing.Union[haem.RNASequence, str],
    total: int,
) -> None:
    assert sequence.count(target, overlap=True) == total


@pytest.mark.parametrize(
    "sequence,target,result",
    [
        (haem.RNASequence(), haem.RNASequence(), None),
        (haem.RNASequence(), haem.RNABase("A"), None),
        (haem.RNASequence(), "", None),
        (haem.RNASequence(), "AU", None),
        (haem.RNASequence("AUG"), haem.RNASequence(), None),
        (haem.RNASequence("AUG"), haem.RNABase("C"), None),
        (haem.RNASequence("AUG"), "", None),
        (haem.RNASequence("AUG"), "GU", None),
        (haem.RNASequence("AUG"), haem.RNASequence("UG"), 1),
        (haem.RNASequence("AUG"), haem.RNABase("G"), 2),
        (haem.RNASequence("AUG"), "UG", 1),
    ],
)
def test_find(
    sequence: haem.RNASequence,
    target: typing.Union[haem.RNASequence, haem.RNABase, str],
    result: typing.Optional[int],
) -> None:
    assert sequence.find(target) == result


@pytest.mark.parametrize(
    "rna_sequence,amino_acid_sequence",
    [
        (haem.RNASequence("AUGUAA"), haem.AminoAcidSequence("M")),
        (haem.RNASequence("AUGUAAA"), haem.AminoAcidSequence("M")),
        (haem.RNASequence("CAUGUAA"), haem.AminoAcidSequence("M")),
        (haem.RNASequence("AUGUAAAUG"), haem.AminoAcidSequence("M")),
        (haem.RNASequence("AUGUGGUAA"), haem.AminoAcidSequence("MW")),
    ],
)
def test_translate(
    rna_sequence: haem.RNASequence, amino_acid_sequence: haem.AminoAcidSequence
) -> None:
    assert rna_sequence.translate() == amino_acid_sequence


def test_translate_no_start_codon() -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.RNASequence().translate()

    assert str(excinfo.value) == "no start codon found"


def test_translate_no_stop_codon() -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.RNASequence("AUG").translate()

    assert str(excinfo.value) == "no stop codon found"
