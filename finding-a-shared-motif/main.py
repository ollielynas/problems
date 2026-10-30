from pathlib import Path

def fasta(text):
    records = {}
    for block in text.split(">")[1:]:
        name, *lines = block.splitlines()
        records[name.strip()] = "".join(l.strip() for l in lines)
    return records

seqs = list(fasta((Path(__file__).parent / "input.txt").read_text()).values())
shortest = min(seqs, key=len)
others = [s for s in seqs if s is not shortest]

def shared_motif():
    n = len(shortest)
    for length in range(n, 0, -1):
        for start in range(n - length + 1):
            cand = shortest[start:start + length]
            if all(cand in s for s in others):
                return cand
    return ""

print(shared_motif())
