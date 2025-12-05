

with open("in.txt") as f:
    ranges = []
    finished_ranges = False
    count = 0
    for line in f:
        if line == "\n":
            finished_ranges = True
            print("got the split")
            print(ranges)
            break
        if not finished_ranges:
            range = line.strip().split("-")
            ranges.append((int(range[0]), int(range[1])))

    new_ranges = []
    ranges.sort(key=lambda x: x[0]) 
    for r in ranges:
        if not new_ranges:
            new_ranges.append(r)
        else:
            last_range = new_ranges[-1]
            if r[0] <= last_range[1]:
                new_ranges[-1] = (last_range[0], max(last_range[1], r[1]))
            else:
                new_ranges.append(r)
    for r in new_ranges:
        start, end = r
        count += end - start + 1
    print(count)
