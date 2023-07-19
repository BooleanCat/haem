import typing

class StopTranslation(Exception):
    """StopTranslation is raised when a stop codon is encountered during
    translation."""

    pass

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
        """Transcription of the DNA base to an RNA base."""
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
    def __add__(self, other: DNABase) -> typing.Any:
        """Not implemented."""
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
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __bool__(self) -> bool:
        """Casting to bool is False for RNABase.GAP and True otherwise."""
        ...
    def __invert__(self) -> RNABase:
        """See `RNABase.complement`."""
    def __add__(self, other: RNABase) -> typing.Any:
        """Not implemented."""
        ...

class AminoAcid:
    """An enumeration of amino acids, as defined by IUPAC.

    AminoAcids may be instantiated either directly by their variant, by their
    IUPAC code, by a tuple of three RNABases or by a string containing three
    RNABase IUPAC codes. For example:

    >>> AminoAcid.SERINE
    >>> AminoAcid('S')
    >>> AminoAcid((RNABase.ADENINE, RNABase.GUANINE, RNABase.CYTOSINE))
    >>> AminoAcid('AGC')

    AminoAcids may also be instantiated by ambiguous IUPAC RNA codes where
    appropriate. For example:

    >>> AminoAcid('AGY')  # Serine

    Invalid inputs or codons that result in ambiguous amino acids will raise a
    ValueError.

    Stop codons will cause a StopTranslation exception to be raised."""

    ALANINE: AminoAcid
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
    TYROSINE: AminoAcid

    @classmethod
    def __new__(
        cls, code_or_codon: typing.Union[str, tuple[RNABase, RNABase, RNABase]]
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
    def __add__(self, other: AminoAcid) -> typing.Any:
        """Not implemented."""
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
        """Not implemented."""
        ...
    @property
    def complement(self) -> DNASequence:
        """Not implemented."""
        ...
    def transcribe(self) -> typing.Any:
        """Not implemented."""
        ...
    def __invert__(self) -> DNASequence:
        """See `DNASequence.complement`."""
        ...
    def __repr__(self) -> str:
        """Not implemented."""
        ...
    def __str__(self) -> str:
        """Not implemented."""
        ...
    def __eq__(self, other: object) -> bool:
        """Not implemented."""
        ...
    def __ne__(self, other: object) -> bool:
        """Not implemented."""
        ...
    def __bool__(self) -> bool:
        """Not implemented."""
        ...
    def __add__(self, other: typing.Union[DNABase, DNASequence]) -> DNASequence:
        """Not implemented."""
        ...
    def __contains__(self, item: typing.Any) -> bool:
        """Not implemented."""
        ...
    def __len__(self) -> int:
        """Not implemented."""
        ...
    def __getitem__(self, key: typing.Any) -> typing.Any:
        """Not implemented."""
        ...
    def __iter__(self) -> typing.Any:
        """Not implemented."""
        ...
    def count(self, item: DNABase) -> int:
        """Not implemented."""
        ...
