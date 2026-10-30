"""Calculating Expected Offspring"""
from pathlib import Path



here = Path(__file__).parent
text = (here / "input.txt").read_text()

AA_AA, AA_Aa, AA_aa, Aa_Aa, Aa_aa, aa_aa = map(float, text.split(" "))

prob = 0
prob += AA_AA
prob += AA_Aa
prob += AA_aa
prob += Aa_Aa * 0.75
prob += Aa_aa * 0.5


# prob /= sum(map(float, text.split(" ")))
print(str(prob*2))
