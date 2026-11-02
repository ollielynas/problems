"""Locating Restriction Sites"""
from rosalind_utils import parse_fasta, read_input, reverse_complement, write_output

text = parse_fasta(read_input())[0][1]
answer = ""

for i in range(len(text)):
    for k in range(4,13):
        if i+k > len(text):continue
        t=text[i:i+k]
        if t == reverse_complement(t):
            text2=f"{i+1} {k}\n"
            if text2 not in answer: answer+= text2
write_output(answer)
