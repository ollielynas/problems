"""RNA Splicing"""
from rosalind_utils import read_input, write_output, parse_fasta, dna_to_protin

inp = parse_fasta(read_input())

p, s = [inp[0][1], inp[1:]]

for i in s:
    print(i)
    p=p.replace(i[1], "")

answer = dna_to_protin(p).replace("*", "")
write_output(answer)
