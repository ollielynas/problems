"""Enumerating Gene Orders"""
from rosalind_utils import read_input, write_output
from itertools import permutations

text = read_input()

num = int(text)

def s(a):return ' '.join(map(str, a))

array = list(permutations(list(range(1,num+1))))

answer = str(len(array)) + "\n" + "\n".join(map(s,array))
write_output(answer)
