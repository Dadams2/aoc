#!/usr/bin/env python3
from itertools import groupby
sums = []
with open("input.txt") as f:
    val = 0
    for line in f:
        if line.strip() == "":
            sums.append(val)
            val = 0
        else:
            val += int(line.strip())
print(sum(sorted(sums, reverse=True)[:3]))

print(sum(sorted((list((sum(list(g)) for k,g in groupby([int(line.strip()) if line.strip() != "" else line.strip() for line in open("input.txt")], key=lambda x: x != '') if k))), reverse=True)[:3]))
