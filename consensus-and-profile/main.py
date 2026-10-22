"""Consensus and Profile"""
from pathlib import Path

text = (Path(__file__).parent / "input.txt").read_text()

rows = [""]
for line in text.split("\n"):
    if len(line) > 2 and not line.startswith(">"):
        rows[len(rows)-1] += line
    else:
        rows += [""]
rows = rows[1:-1]
cols = []
for col in range(len(rows[0])):
    c_array = ""
    for r in range(len(rows)):
        c_array += rows[r][col]
    cols += [c_array]



def most_common(a):
    cnt = {"":0}
    for item in a:
        if item in cnt:
            cnt[item] += 1
        else:
            cnt[item] = 1
    max_item = ""
    for item in cnt.items():
        if item[1] > cnt[max_item]:
            max_item = item[0]
    return max_item

def count_letter(a, letter):
    cnt = {"":0}
    for item in a:
        if item in cnt:
            cnt[item] += 1
        else:
            cnt[item] = 1
    if letter in cnt:
        return cnt[letter]
    else:
        return 0
output = ""
output += ''.join([most_common(list(c)) for c in cols])
output += "\nA: "+' '.join([str(count_letter(list(c), "A")) for c in cols])
output += "\nC: "+' '.join([str(count_letter(list(c), "C")) for c in cols])
output += "\nG: "+' '.join([str(count_letter(list(c), "G")) for c in cols])
output += "\nT: "+' '.join([str(count_letter(list(c), "T")) for c in cols])

with open("out.txt", "w") as f:
    f.write(output)
