"""Introduction to Random Strings"""
from pathlib import Path

text = (Path(__file__).parent / "input.txt").read_text()
print(text)
