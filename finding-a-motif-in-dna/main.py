"""Finding a Motif in DNA"""
from pathlib import Path

text = (Path(__file__).parent / "input.txt").read_text()
big,small,_ = text.split("\n")

pos = []

for i in range(len(big)-len(small)):
    if big[i:len(small)+i] == small:
        pos += [str(i+1)]
print(" ".join(pos))
