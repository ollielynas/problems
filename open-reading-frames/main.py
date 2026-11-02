"""Open Reading Frames"""
from pathlib import Path
from rosalind_utils import *

here = Path(__file__).parent
dna = parse_fasta((here / "input.txt").read_text())[0][1]
answer = []

rc = reverse_complement(dna)

frames = [rc, dna, rc[1:], rc[2:], dna[1:], dna[2:]]
frames = map(dna_to_protin, frames)

for f in frames:
    chains = []
    for char in f:
        if char == "*":
            answer += chains
            chains = []
        if char == "M":
            chains += [""]
        for i in range(len(chains)):
            chains[i] += char

answer = "\n".join(set(answer))
print(answer)
(here / "output.txt").write_text(f"{answer}\n")
