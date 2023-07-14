import typing

class DNABase:
    """An enumeration of DNA bases, as defined by IUPAC.

    DNABases may be instantiated either directly by their variant, or by
    their IUPAC code. For example:

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

    def __new__(self, code: str) -> DNABase: ...
    @property
    def code(self) -> str:
        """Not implemented."""
        pass
    @property
    def complement(self) -> DNABase:
        """Not implemented."""
        pass
    def transcribe(self) -> RNABase:
        """Not implemented."""
        pass
    def __repr__(self) -> str:
        """Not implemented."""
        pass
    def __str__(self) -> str:
        """Not implemented."""
        pass
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __bool__(self) -> bool:
        """Not implemented."""
        pass
    def __invert__(self) -> DNABase:
        """See `DNABase.complement`."""
    def __add__(self, other: DNABase) -> typing.Any:
        """Not implemented."""
        pass

class RNABase:
    ADENINE: RNABase
    CYTOSINE: RNABase

    def __new__(self, code: str) -> RNABase:
        """Not implemented."""
        pass
    @property
    def code(self) -> str:
        """Not implemented."""
        pass
    @property
    def complement(self) -> RNABase:
        """Not implemented."""
        pass
    def __repr__(self) -> str:
        """Not implemented."""
        pass
    def __str__(self) -> str:
        """Not implemented."""
        pass
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __bool__(self) -> bool:
        """Not implemented."""
        pass
    def __invert__(self) -> RNABase:
        """See `RNABase.complement`."""
    def __add__(self, other: RNABase) -> typing.Any:
        """Not implemented."""
        pass
