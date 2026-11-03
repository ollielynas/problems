"""Longest Increasing Subsequence"""
from bisect import bisect_left
from rosalind_utils import read_input, write_output


def lis(nums):
    tail_vals = []
    tail_idx = []
    prev = [-1] * len(nums)

    for i, n in enumerate(nums):
        k = bisect_left(tail_vals, n)
        if k > 0:
            prev[i] = tail_idx[k - 1]
        if k == len(tail_vals):
            tail_vals.append(n)
            tail_idx.append(i)
        else:
            tail_vals[k] = n
            tail_idx[k] = i

    run = []
    i = tail_idx[-1] if tail_idx else -1
    while i != -1:
        run.append(nums[i])
        i = prev[i]
    return run[::-1]


text = read_input()
nums = list(map(int, text.split("\n")[1].split(" ")))

asc_run = lis(nums)
dec_run = lis(nums[::-1])[::-1]

answer = " ".join(map(str, asc_run))
answer += "\n" + " ".join(map(str, dec_run))
write_output(answer)
