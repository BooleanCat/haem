import typing

class DNABase:
    ADENINE: DNABase
    CYTOSINE: DNABase

    def __new__(self, code: str) -> DNABase:
        """Not implemented."""
        pass
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
