import operator
import typing

import pytest

import haem


def test__new__str() -> None:
    assert haem.AminoAcidSequence("MVVR") == haem.AminoAcidSequence(
        [
            haem.AminoAcid.METHIONINE,
            haem.AminoAcid.VALINE,
            haem.AminoAcid.VALINE,
            haem.AminoAcid.ARGININE,
        ]
    )


def test__new__str__invalid() -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.AminoAcidSequence("JJ")

    assert str(excinfo.value) == 'invalid IUPAC amino acid code "J"'


def test__new__iterable_amino_acid() -> None:
    assert haem.AminoAcidSequence(
        iter(
            [
                haem.AminoAcid.METHIONINE,
                haem.AminoAcid.VALINE,
                haem.AminoAcid.VALINE,
                haem.AminoAcid.ARGININE,
            ]
        )
    ) == haem.AminoAcidSequence("MVVR")


def test__new__iterable_str() -> None:
    assert haem.AminoAcidSequence(iter(["M", "V", "V", "R"])) == haem.AminoAcidSequence(
        "MVVR"
    )


def test__new__iterable_invalid() -> None:
    with pytest.raises(ValueError) as excinfo:
        haem.AminoAcidSequence(iter(["J"]))

    assert str(excinfo.value) == 'invalid IUPAC amino acid code "J"'


def test__new__sequence_amino_acids() -> None:
    assert haem.AminoAcidSequence(
        [
            haem.AminoAcid.METHIONINE,
            haem.AminoAcid.VALINE,
            haem.AminoAcid.VALINE,
            haem.AminoAcid.ARGININE,
        ]
    ) == haem.AminoAcidSequence("MVVR")


def test__new__sequence_str() -> None:
    assert haem.AminoAcidSequence(["M", "V", "V", "R"]) == haem.AminoAcidSequence(
        "MVVR"
    )


@pytest.mark.parametrize(
    "amino_acids,text",
    [
        ([], "<AminoAcidSequence>"),
        ([haem.AminoAcid.METHIONINE], "<AminoAcidSequence: M>"),
        (
            [haem.AminoAcid.METHIONINE, haem.AminoAcid.ARGININE],
            "<AminoAcidSequence: MR>",
        ),
        (
            [haem.AminoAcid.METHIONINE for _ in range(100)],
            f"<AminoAcidSequence: {'M' * 100}>",
        ),
    ],
)
def test__repr__(amino_acids: typing.List[haem.AminoAcid], text: str) -> None:
    assert repr(haem.AminoAcidSequence(amino_acids)) == text


@pytest.mark.parametrize(
    "amino_acids,text",
    [
        ([], ""),
        ([haem.AminoAcid.METHIONINE], "M"),
        ([haem.AminoAcid.METHIONINE, haem.AminoAcid.ARGININE], "MR"),
        ([haem.AminoAcid.VALINE for _ in range(20)], "V" * 20),
        (
            [haem.AminoAcid.VALINE for _ in range(21)] + [haem.AminoAcid.METHIONINE],
            f"{'V' * 10}...{'V' * 9}M",
        ),
    ],
)
def test__str__(amino_acids: typing.List[haem.AminoAcid], text: str) -> None:
    assert str(haem.AminoAcidSequence(amino_acids)) == text


@pytest.mark.parametrize(
    "amino_acids", [[], [haem.AminoAcid.METHIONINE, haem.AminoAcid.VALINE]]
)
def test__eq__(amino_acids: typing.List[haem.AminoAcid]) -> None:
    assert haem.AminoAcidSequence(amino_acids) == haem.AminoAcidSequence(amino_acids)


def test__ne__() -> None:
    assert haem.AminoAcidSequence(
        [haem.AminoAcid.METHIONINE, haem.AminoAcid.VALINE]
    ) != haem.AminoAcidSequence([haem.AminoAcid.METHIONINE, haem.AminoAcid.ARGININE])


@pytest.mark.parametrize("op", [operator.gt, operator.ge, operator.lt, operator.le])
def test_unsupported_comparison(
    op: typing.Callable[[haem.AminoAcidSequence, haem.AminoAcidSequence], bool]
) -> None:
    with pytest.raises(TypeError):
        op(haem.AminoAcidSequence(), haem.AminoAcidSequence())


@pytest.mark.parametrize(
    "amino_acids,result", [([], False), ([haem.AminoAcid.METHIONINE], True)]
)
def test__bool__(amino_acids: typing.List[haem.AminoAcid], result: bool) -> None:
    assert bool(haem.AminoAcidSequence(amino_acids)) is result


@pytest.mark.parametrize(
    "left,right,result",
    [
        (
            haem.AminoAcidSequence("MV"),
            haem.AminoAcid.ARGININE,
            haem.AminoAcidSequence("MVR"),
        ),
        (
            haem.AminoAcidSequence("MV"),
            haem.AminoAcidSequence("RVR"),
            haem.AminoAcidSequence("MVRVR"),
        ),
        (
            haem.AminoAcidSequence("MV"),
            haem.AminoAcidSequence(""),
            haem.AminoAcidSequence("MV"),
        ),
        (
            haem.AminoAcidSequence(""),
            haem.AminoAcidSequence(""),
            haem.AminoAcidSequence(""),
        ),
        (
            haem.AminoAcidSequence(""),
            haem.AminoAcid.METHIONINE,
            haem.AminoAcidSequence("M"),
        ),
    ],
)
def test__add__(
    left: haem.AminoAcidSequence,
    right: typing.Union[haem.AminoAcid, haem.AminoAcidSequence],
    result: haem.AminoAcidSequence,
) -> None:
    assert left + right == result


@pytest.mark.parametrize(
    "sequence,target,result",
    [
        (haem.AminoAcidSequence("M"), haem.AminoAcid.METHIONINE, True),
        (haem.AminoAcidSequence(""), haem.AminoAcid.METHIONINE, False),
        (haem.AminoAcidSequence("V"), haem.AminoAcid.METHIONINE, False),
        (haem.AminoAcidSequence("MVV"), haem.AminoAcid.VALINE, True),
        (haem.AminoAcidSequence("MVV"), haem.AminoAcid.ARGININE, False),
        (haem.AminoAcidSequence(""), haem.AminoAcidSequence(""), True),
        (haem.AminoAcidSequence("MVR"), haem.AminoAcidSequence("VR"), True),
        (haem.AminoAcidSequence("MVR"), haem.AminoAcidSequence("RV"), False),
        (haem.AminoAcidSequence("M"), haem.AminoAcidSequence("MM"), False),
    ],
)
def test__contains__(
    sequence: haem.AminoAcidSequence,
    target: typing.Union[haem.AminoAcid, haem.AminoAcidSequence],
    result: bool,
) -> None:
    assert (target in sequence) is result


@pytest.mark.parametrize("amino_acids,length", [([], 0), ([haem.AminoAcid.VALINE], 1)])
def test__len__(amino_acids: typing.List[haem.AminoAcid], length: int) -> None:
    assert len(haem.AminoAcidSequence(amino_acids)) == length


@pytest.mark.parametrize(
    "sequence,index,amino_acid",
    [
        (haem.AminoAcidSequence("MVR"), 0, haem.AminoAcid.METHIONINE),
        (haem.AminoAcidSequence("MVR"), 1, haem.AminoAcid.VALINE),
        (haem.AminoAcidSequence("MVR"), 2, haem.AminoAcid.ARGININE),
        (haem.AminoAcidSequence("MVR"), -1, haem.AminoAcid.ARGININE),
        (haem.AminoAcidSequence("MVR"), -2, haem.AminoAcid.VALINE),
        (haem.AminoAcidSequence("MVR"), -3, haem.AminoAcid.METHIONINE),
    ],
)
def test__getitem__index(
    sequence: haem.AminoAcidSequence, index: int, amino_acid: haem.AminoAcid
) -> None:
    assert sequence[index] == amino_acid


@pytest.mark.parametrize("index", [3, -4])
def test__getitem__index_out_of_range(index: int) -> None:
    with pytest.raises(IndexError) as excinfo:
        haem.AminoAcidSequence("MVR")[index]

    assert str(excinfo.value) == "AminoAcidSequence index out of range"


@pytest.mark.parametrize(
    "sequence,slic,result",
    [
        (haem.AminoAcidSequence(""), slice(0, 0), haem.AminoAcidSequence("")),
        (haem.AminoAcidSequence("MVR"), slice(0, 2), haem.AminoAcidSequence("MV")),
        (haem.AminoAcidSequence("MVR"), slice(1, 3), haem.AminoAcidSequence("VR")),
        (haem.AminoAcidSequence("MVR"), slice(0, 3), haem.AminoAcidSequence("MVR")),
        (haem.AminoAcidSequence("MVR"), slice(0, 4), haem.AminoAcidSequence("MVR")),
        (haem.AminoAcidSequence("MVR"), slice(0, None), haem.AminoAcidSequence("MVR")),
        (haem.AminoAcidSequence("MVR"), slice(1, None), haem.AminoAcidSequence("VR")),
        (haem.AminoAcidSequence("MVR"), slice(-1, None), haem.AminoAcidSequence("R")),
        (haem.AminoAcidSequence("MVR"), slice(-3, None), haem.AminoAcidSequence("MVR")),
        (haem.AminoAcidSequence("MVR"), slice(-4, None), haem.AminoAcidSequence("MVR")),
        (haem.AminoAcidSequence("MVR"), slice(0, -1), haem.AminoAcidSequence("MV")),
        (
            haem.AminoAcidSequence("MVRVRV"),
            slice(0, -1, 2),
            haem.AminoAcidSequence("MRR"),
        ),
        (
            haem.AminoAcidSequence("MVRVRV"),
            slice(5, None, -1),
            haem.AminoAcidSequence("VRVRVM"),
        ),
        (
            haem.AminoAcidSequence("MVRVRV"),
            slice(None, None, -1),
            haem.AminoAcidSequence("VRVRVM"),
        ),
        (
            haem.AminoAcidSequence("MVRVRV"),
            slice(10, 2, -2),
            haem.AminoAcidSequence("VV"),
        ),
    ],
)
def test__getitem__slice(
    sequence: haem.AminoAcidSequence, slic: slice, result: haem.AminoAcidSequence
) -> None:
    assert sequence[slic.start : slic.stop : slic.step] == result


@pytest.mark.parametrize(
    "amino_acids",
    [[haem.AminoAcid("M")], [haem.AminoAcid("V"), haem.AminoAcid("R")], []],
)
def test__iter__(amino_acids: typing.List[haem.AminoAcid]) -> None:
    sequence_iter = iter(haem.AminoAcidSequence(amino_acids))

    for amino_acid in amino_acids:
        assert next(sequence_iter) == amino_acid

    with pytest.raises(StopIteration):
        next(sequence_iter)


@pytest.mark.parametrize(
    "sequence,target,total",
    [
        (haem.AminoAcidSequence(""), haem.AminoAcid.VALINE, 0),
        (haem.AminoAcidSequence("MVRV"), haem.AminoAcid.VALINE, 2),
        (haem.AminoAcidSequence("MVRV"), "V", 2),
    ],
)
def test_count(
    sequence: haem.AminoAcidSequence,
    target: typing.Union[haem.AminoAcid, str],
    total: int,
) -> None:
    assert sequence.count(target) == total
