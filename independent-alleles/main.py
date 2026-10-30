from pathlib import Path
from math import comb

here = Path(__file__).parent
k, N = map(int, (here / "input.txt").read_text().split())

n = 2**k
p = 0.25

answer = 1 - sum(comb(n, i) * p**i * (1 - p)**(n - i) for i in range(N))

print(round(answer, 3))
(here / "output.txt").write_text(f"{answer:.3f}\n")
