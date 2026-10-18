"""Counting Point Mutations"""
from pathlib import Path

text = (Path(__file__).parent / "input.txt").read_text()
a,b,_= text.split("\n")
d = 0
for i in range(min(len(a), len(b))):
    if a[i] != b[i]: d += 1
print(d)
