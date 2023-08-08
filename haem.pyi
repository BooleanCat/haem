import typing

class StopTranslation(Exception):
    """StopTranslation is raised when a stop codon is encountered during
    translation."""

    ...

class DNABase:
    """An enumeration of DNA bases, as defined by IUPAC.

    DNABases may be instantiated either directly by their variant or by their
    IUPAC code. For example:

    >>> DNABase.THYMINE
    >>> DNABase('T')

    A ValueError is raised if the code is not valid."""

    ADENINE: DNABase
    CYTOSINE: DNABase
    GUANINE: DNABase
    THYMINE: DNABase
    ADENINE_CYTOSINE: DNABase
    ADENINE_GUANINE: DNABase
    ADENINE_THYMINE: DNABase
    CYTOSINE_GUANINE: DNABase
    CYTOSINE_THYMINE: DNABase
    GUANINE_THYMINE: DNABase
    ADENINE_CYTOSINE_GUANINE: DNABase
    ADENINE_CYTOSINE_THYMINE: DNABase
    ADENINE_GUANINE_THYMINE: DNABase
    CYTOSINE_GUANINE_THYMINE: DNABase
    ANY: DNABase
    GAP: DNABase

    @classmethod
    def __new__(cls, code: str) -> DNABase: ...
    @property
    def code(self) -> str:
        """One-letter IUPAC code of the DNA base."""
        ...
    @property
    def complement(self) -> DNABase:
        """The complementary DNA base."""
        ...
    def transcribe(self) -> RNABase:
        """Transcription of the DNA base to a RNA base."""
        ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __bool__(self) -> bool:
        """Casting to bool is False for DNABase.GAP and True otherwise."""
        ...
    def __invert__(self) -> DNABase:
        """See `DNABase.complement`."""
    def __add__(self, other: typing.Union[DNABase, DNASequence, str]) -> DNASequence:
        """Create a new sequence consisting of this base followed by the given
        sequence member(s)."""
        ...
    def __radd__(self, other: typing.Union[DNABase, DNASequence, str]) -> DNASequence:
        """Create a new sequence consisting of the given sequence member(s)
        followed by this base."""
        ...

class RNABase:
    """An enumeration of RNA bases, as defined by IUPAC.

    RNABases may be instantiated either directly by their variant or by their
    IUPAC code. For example:

    >>> RNABase.URACIL
    >>> RNABase('U')

    A ValueError is raised if the code is not valid."""

    ADENINE: RNABase
    CYTOSINE: RNABase
    GUANINE: RNABase
    URACIL: RNABase
    ADENINE_CYTOSINE: RNABase
    ADENINE_GUANINE: RNABase
    ADENINE_URACIL: RNABase
    CYTOSINE_GUANINE: RNABase
    CYTOSINE_URACIL: RNABase
    GUANINE_URACIL: RNABase
    ADENINE_CYTOSINE_GUANINE: RNABase
    ADENINE_CYTOSINE_URACIL: RNABase
    ADENINE_GUANINE_URACIL: RNABase
    CYTOSINE_GUANINE_URACIL: RNABase
    ANY: RNABase
    GAP: RNABase

    @classmethod
    def __new__(cls, code: str) -> RNABase: ...
    @property
    def code(self) -> str:
        """One-letter IUPAC code of the RNA base."""
        ...
    @property
    def complement(self) -> RNABase:
        """The complementary RNA base."""
        ...
    def retro_transcribe(self) -> DNABase:
        """Reverse transcription of the RNA base to a DNA base."""
        ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __bool__(self) -> bool:
        """Casting to bool is False for RNABase.GAP and True otherwise."""
        ...
    def __invert__(self) -> RNABase:
        """See `RNABase.complement`."""
    def __add__(self, other: typing.Union[RNABase, RNASequence, str]) -> RNASequence:
        """Create a new sequence consisting of this base followed by the given
        sequence member(s)."""
        ...
    def __radd__(self, other: typing.Union[RNABase, RNASequence, str]) -> RNASequence:
        """Create a new sequence consisting of the given sequence member(s)
        followed by this base."""
        ...

class AminoAcid:
    """An enumeration of amino acids, as defined by IUPAC.

    AminoAcids may be instantiated either directly by their variant, by their
    IUPAC code, by a tuple of three RNABases or by a string containing three
    RNABase IUPAC codes. For example:

    >>> AminoAcid.SERINE
    >>> AminoAcid('S')
    >>> AminoAcid((RNABase.ADENINE, RNABase.GUANINE, RNABase.CYTOSINE))
    >>> AminoAcid(('A', 'G', 'C'))
    >>> AminoAcid('AGC')

    AminoAcids may also be instantiated by ambiguous IUPAC RNA codes where
    appropriate. For example:

    >>> AminoAcid('AGY')  # Serine

    Invalid inputs or codons that result in ambiguous amino acids will raise a
    ValueError.

    Stop codons will cause a StopTranslation exception to be raised."""

    ALANINE: AminoAcid
    ASPARTIC_ACID_ASPARAGINE: AminoAcid
    CYSTEINE: AminoAcid
    ASPARTIC_ACID: AminoAcid
    GLUTAMIC_ACID: AminoAcid
    PHENYLALANINE: AminoAcid
    GLYCINE: AminoAcid
    HISTIDINE: AminoAcid
    ISOLEUCINE: AminoAcid
    LYSINE: AminoAcid
    LEUCINE: AminoAcid
    METHIONINE: AminoAcid
    ASPARAGINE: AminoAcid
    PROLINE: AminoAcid
    GLUTAMINE: AminoAcid
    ARGININE: AminoAcid
    SERINE: AminoAcid
    THREONINE: AminoAcid
    VALINE: AminoAcid
    TRYPTOPHAN: AminoAcid
    ANY: AminoAcid
    TYROSINE: AminoAcid
    GLUTAMINE_GLUTAMIC_ACID: AminoAcid

    @classmethod
    def __new__(
        cls,
        code_or_codon: typing.Union[
            str, typing.Tuple[RNABase, RNABase, RNABase], typing.Tuple[str, str, str]
        ],
    ) -> AminoAcid: ...
    @property
    def code(self) -> str:
        """One-letter IUPAC code of the amino acid."""
        ...
    @property
    def short_name(self) -> str:
        """Three-letter IUPAC code of the amino acid."""
        ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __bool__(self) -> bool:
        """Always true."""
        ...
    def __add__(
        self, other: typing.Union[AminoAcid, AminoAcidSequence, str]
    ) -> AminoAcidSequence:
        """Create a new sequence consisting of this amino acid followed by the
        given sequence member(s)."""
        ...
    def __radd__(
        self, other: typing.Union[AminoAcid, AminoAcidSequence, str]
    ) -> AminoAcidSequence:
        """Create a new sequence consisting of the given sequence member(s)
        followed by this amino acid."""
        ...

class DNASequence:
    @classmethod
    def __new__(
        cls,
        bases: typing.Union[
            str,
            typing.Iterable[typing.Union[str, DNABase]],
            typing.Sequence[typing.Union[str, DNABase]],
        ] = "",
    ) -> DNASequence:
        """A sequence of `DNABase`s.

        `DNASequence` may be instantiated by a string of IUPAC DNA codes, or a
        sequence or iterable of `DNABase`s or IUPAC DNA codes. For example:

        >>> DNASequence("ACGT")
        >>> DNASequence(["A", "C", "G", "T"])
        >>> DNASequence(iter(["A", "C", "G", "T"]))
        >>> DNASequence([DNABase.ADENINE, DNABase.CYTOSINE])
        >>> DNASequence(iter([DNABase.ADENINE, DNABase.CYTOSINE]))

        A ValueError is raised if any DNA code is not valid."""

        ...
    @property
    def complement(self) -> DNASequence:
        """The complementary DNA sequence."""
    def transcribe(self) -> RNASequence:
        """Transcription of the DNA sequence to a RNA sequence."""
        ...
    def __invert__(self) -> DNASequence:
        """See `DNASequence.complement`."""
        ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __bool__(self) -> bool:
        """Casting to bool is False for empty sequences and True otherwise."""
        ...
    def __add__(self, other: typing.Union[DNABase, DNASequence, str]) -> DNASequence:
        """Create a new sequence consisting of this sequence followed by the
        given sequence member(s)."""
    def __radd__(self, other: typing.Union[DNABase, DNASequence, str]) -> DNASequence:
        """Create a new sequence consisting of the given sequence member(s)
        followed by this sequence."""
        ...
    def __contains__(self, item: typing.Union[DNABase, DNASequence]) -> bool:
        """Return true if the given DNABase or DNASequence is contained within
        this sequence."""
        ...
    def __len__(self) -> int: ...
    def __getitem__(
        self, key: typing.Union[int, slice]
    ) -> typing.Union[DNABase, DNASequence]: ...
    def __iter__(self) -> typing.Iterator[DNABase]: ...
    def count(
        self, item: typing.Union[DNABase, DNASequence, str], overlap: bool = True
    ) -> int:
        """Count the occurances of a DNABase in the sequence."""
        ...
    def find(
        self, target: typing.Union[DNASequence, DNABase, str]
    ) -> typing.Optional[int]:
        """Find the index of the first occurance of the given DNABase or
        DNASequence."""
        ...

class RNASequence:
    @classmethod
    def __new__(
        cls,
        bases: typing.Union[
            str,
            typing.Iterable[typing.Union[str, RNABase]],
            typing.Sequence[typing.Union[str, RNABase]],
        ] = "",
    ) -> RNASequence:
        """A sequence of `RNABase`s.

        `RNASequence` may be instantiated by a string of IUPAC RNA codes, or a
        sequence or iterable of `RNABase`s or IUPAC RNA codes. For example:

        >>> RNASequence("ACGU")
        >>> RNASequence(["A", "C", "G", "U"])
        >>> RNASequence(iter(["A", "C", "G", "U"]))
        >>> RNASequence([RNABase.ADENINE, RNABase.CYTOSINE])
        >>> RNASequence(iter([RNABase.ADENINE, RNABase.CYTOSINE]))

        A ValueError is raised if any RNA code is not valid."""

        ...
    @property
    def complement(self) -> RNASequence:
        """The complementary RNA sequence."""
    def retro_transcribe(self) -> DNASequence:
        """Reverse transcription of the RNA sequence to a DNA sequence."""
        ...
    def __invert__(self) -> RNASequence:
        """See `RNASequence.complement`."""
        ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __bool__(self) -> bool:
        """Casting to bool is False for empty sequences and True otherwise."""
        ...
    def __add__(self, other: typing.Union[RNABase, RNASequence, str]) -> RNASequence:
        """Create a new sequence consisting of this sequence followed by the
        given sequence member(s)."""
    def __radd__(self, other: typing.Union[RNABase, RNASequence, str]) -> RNASequence:
        """Create a new sequence consisting of the given sequence member(s)
        followed by this sequence."""
        ...
    def __contains__(self, item: typing.Union[RNABase, RNASequence]) -> bool:
        """Return true if the given RNABase or RNASequence is contained within
        this sequence."""
        ...
    def __len__(self) -> int: ...
    def __getitem__(
        self, key: typing.Union[int, slice]
    ) -> typing.Union[RNABase, RNASequence]: ...
    def __iter__(self) -> typing.Iterator[RNABase]: ...
    def count(
        self, item: typing.Union[RNABase, RNASequence, str], overlap: bool = True
    ) -> int:
        """Count the occurances of a RNABase in the sequence."""
        ...
    def find(
        self, target: typing.Union[RNASequence, RNABase, str]
    ) -> typing.Optional[int]:
        """Find the index of the first occurance of the given RNABase or
        RNASequence."""
        ...

class AminoAcidSequence:
    @classmethod
    def __new__(
        cls,
        bases: typing.Union[
            str,
            typing.Iterable[typing.Union[str, AminoAcid]],
            typing.Sequence[typing.Union[str, AminoAcid]],
        ] = "",
    ) -> AminoAcidSequence:
        """A sequence of `AminoAcid`s.

        `AminoAcidSequence` may be instantiated by a string of IUPAC amino acid
        codes, or a sequence or iterable of `AminoAcid`s or IUPAC amino acid
        codes. For example:

        >>> AminoAcidSequence("MVVR")
        >>> AminoAcidSequence(["M", "V", "V", "R"])
        >>> AminoAcidSequence(iter(["M", "V", "V", "R"]))
        >>> AminoAcidSequence([AminoAcid.METHIONINE, AminoAcid.VALINE])
        >>> AminoAcidSequence(iter([AminoAcid.METHIONINE, AminoAcid.VALINE]))

        A ValueError is raised if any amino acid code is not valid."""

        ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __bool__(self) -> bool:
        """Casting to bool is False for empty sequences and True otherwise."""
        ...
    def __add__(
        self, other: typing.Union[AminoAcid, AminoAcidSequence, str]
    ) -> AminoAcidSequence:
        """Create a new sequence consisting of this sequence followed by the
        given sequence member(s)."""
    def __radd__(
        self, other: typing.Union[AminoAcidSequence, AminoAcid, str]
    ) -> AminoAcidSequence:
        """Create a new sequence consisting of the given sequence member(s)
        followed by this sequence."""
        ...
    def __contains__(self, item: typing.Union[AminoAcid, AminoAcidSequence]) -> bool:
        """Return true if the given AminoAcid or AminoAcidSequence is contained
        within this sequence."""
        ...
    def __len__(self) -> int: ...
    def __getitem__(
        self, key: typing.Union[int, slice]
    ) -> typing.Union[AminoAcid, AminoAcidSequence]: ...
    def __iter__(self) -> typing.Iterator[AminoAcid]: ...
    def count(
        self,
        item: typing.Union[AminoAcid, str, AminoAcidSequence],
        overlap: bool = False,
    ) -> int:
        """Count the occurances of an AminoAcid in the sequence."""
        ...
    def find(
        self, target: typing.Union[AminoAcidSequence, AminoAcid, str]
    ) -> typing.Optional[int]:
        """Find the index of the first occurance of the given AminoAcid or
        AminoAcidSequence."""
        ...
