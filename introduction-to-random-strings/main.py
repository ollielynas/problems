"""Introduction to Random Strings"""
from pathlib import Path
from math import log10



text = (Path(__file__).parent / "input.txt").read_text()

dna,nums,_ = text.split("\n")

nums = [float(n) for n in nums.split(" ")]

for prob in nums:
    total = 1
    at = (1-prob)/2
    cg = (prob)/2
    for letter in dna:
        if letter in "AT":
            total *= at
        else:
            total *= cg
    print(log10(total))
