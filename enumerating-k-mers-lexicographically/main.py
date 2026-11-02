"""Enumerating k-mers Lexicographically"""
from rosalind_utils import read_input, write_output
from itertools import combinations
a,num = read_input().split("\n")
def s(b):return "".join(b)
answer = "\n".join(sorted(set(map(s,list(combinations(((a + " ")*int(num)).strip().split(" "), int(num)))))))

write_output(answer)
