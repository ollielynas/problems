"""Counting DNA Nucleotides"""
from pathlib import Path

text = (Path(__file__).parent / "input.txt").read_text()
text = text.lower()

print(' '.join([str(text.count(l)) for l in ['a', 'c', 'g', 't']]))
