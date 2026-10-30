"""Calculating Expected Offspring"""
from pathlib import Path


def fasta(text):
    """Parse FASTA into {id: sequence}, in file order."""
    records = {}
    for block in text.split(">")[1:]:
        name, *lines = block.splitlines()
        records[name.strip()] = "".join(line.strip() for line in lines)
    return records


here = Path(__file__).parent
text = (here / "input.txt").read_text()

answer = text
print(answer)
(here / "output.txt").write_text(f"{answer}\n")
