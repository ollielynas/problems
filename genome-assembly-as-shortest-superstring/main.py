"""Genome Assembly as Shortest Superstring"""
from functools import reduce
from random import shuffle

from rosalind_utils import read_input, write_output, parse_fasta

data = [d[1] for d in parse_fasta(read_input())]

n = len(data[0])

def c(a, b):
    if b in a:
        return a
    for L in range(min(len(a), len(b)), n//2, -1):
        if a.endswith(b[:L]):
            return a + b[L:]
    return b + "-" + a

while len(data) > 1:
    data = reduce(c, data).split("-")
    shuffle(data)

write_output(data[0])
