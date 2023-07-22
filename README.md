# haem

## Quick start

Create a DNA sequence, complement and transcribe it:

```python
>>> import haem
>>> dna = haem.DNASequence("ACGT")
>>> dna.complement
<DNASequence: TGCA>
>>> dna.transcribe()
<RNASequence: ACGU>
```

Create an amino acid from a codon and from an ambiguous codon:

```python
>>> haem.AminoAcid("UCA")
AminoAcid.SERINE
>>> haem.AminoAcid("UCN")
AminoAcid.SERINE
```
