"""Inferring mRNA from Protein"""
from pathlib import Path




here = Path(__file__).parent
text = (here / "input.txt").read_text()

total = 3
mapping = {
"UUU":"F",
"CUU":"L",
"AUU":"I",
"GUU":"V",
"UUC":"F",
"CUC":"L",
"AUC":"I",
"GUC":"V",
"UUA":"L",
"CUA":"L",
"AUA":"I",
"GUA":"V",
"UUG":"L",
"CUG":"L",
"AUG":"M",
"GUG":"V",
"UCU":"S",
"CCU":"P",
"ACU":"T",
"GCU":"A",
"UCC":"S",
"CCC":"P",
"ACC":"T",
"GCC":"A",
"UCA":"S",
"CCA":"P",
"ACA":"T",
"GCA":"A",
"UCG":"S",
"CCG":"P",
"ACG":"T",
"GCG":"A",
"UAU":"Y",
"CAU":"H",
"AAU":"N",
"GAU":"D",
"UAC":"Y",
"CAC":"H",
"AAC":"N",
"GAC":"D",
"UAA":"Stop",
"CAA":"Q",
"AAA":"K",
"GAA":"E",
"UAG":"Stop",
"CAG":"Q",
"AAG":"K",
"GAG":"E",
"UGU":"C",
"CGU":"R",
"AGU":"S",
"GGU":"G",
"UGC":"C",
"CGC":"R",
"AGC":"S",
"GGC":"G",
"UGA":"Stop",
"CGA":"R",
"AGA":"R",
"GGA":"G",
"UGG":"W",
"CGG":"R",
"AGG":"R",
"GGG":"G"
}

option_count = {}

for v in mapping.values():
    if v not in option_count:
        option_count[v] = 1
    else:
        option_count[v] += 1

for l in text.strip():
    total *= option_count[l]
    total = total%1_000_000

answer = str(total)
print(answer)
(here / "output.txt").write_text(f"{answer}\n")
