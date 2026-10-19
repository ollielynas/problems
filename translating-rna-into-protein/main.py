"""Translating RNA into Protein"""
from pathlib import Path

text = (Path(__file__).parent / "input.txt").read_text()

# Source - https://stackoverflow.com/a/18854817
# Posted by rlms, modified by community. See post 'Timeline' for change history
# Retrieved 2026-10-19, License - CC BY-SA 3.0

def chunkstring(string, length):
    return (string[0+i:length+i] for i in range(0, len(string), length))

def translate(frame):
    match frame:
        case "UUU" | "UUC":
           return "F"
        case "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG":
           return "L"
        case "AUU" | "AUC" | "AUA":
           return "I"
        case "AUG":
           return "M"
        case "GUU" | "GUC" | "GUA" | "GUG":
           return "V"
        case "UCU" | "UCC" | "UCA" | "UCG":
           return "S"
        case "CCU" | "CCC" | "CCA" | "CCG":
           return "P"
        case "ACU" | "ACC" | "ACA" | "ACG":
           return "T"
        case "GCU" | "GCC" | "GCA" | "GCG":
           return "A"
        case "UAU" | "UAC":
           return "Y"
        case "UAA" | "UAG" | "UGA":
            # STOP coden
           return "Stop"
        case "CAU" | "CAC":
           return "H"
        case "CAA" | "CAG":
           return "Q"
        case "AAU" | "AAC":
           return "N"
        case "AAA" | "AAG":
           return "K"
        case "GAU" | "GAC":
           return "D"
        case "GAA" | "GAG":
           return "E"
        case "UGU" | "UGC":
           return "C"
        case "UGG":
           return "W"
        case "CGU" | "CGC" | "CGA" | "CGG":
            return "R"
        case "AGU" | "AGC":
            return "S"
        case "AGA" | "AGG":
            return "R"
        case "GGU" | "GGC" | "GGA" | "GGG":
            return "G"
        case _:
           return "-"

reading_frames = list(chunkstring(text, 3))
protein = []
for frame in reading_frames:
    protein += [translate(frame)]

print((''.join(protein).split("Stop"))[0])
