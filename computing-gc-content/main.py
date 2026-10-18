"""Computing GC Content"""
from pathlib import Path

text = (Path(__file__).parent / "input.txt").read_text()
id = ""
bid = ""
gc = 0
at = 0
r = 0
for line in text.split("\n") + [">test"]:
    if line.startswith(">"):
        if at+gc > 0 and  gc / (at+gc) > r:
            r = gc / (at+gc)
            bid = id
        id=line[1:]
        at = 0
        gc = 0
    else:
        at += line.count("A") + line.count("T")
        gc += line.count("C") + line.count("G")
print(bid)
print(r*100)
