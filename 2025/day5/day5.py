

with open("in.txt") as f:
    ranges = []
    finished_ranges = False
    count = 0
    for line in f:
        if line == "\n":
            finished_ranges = True
            print("got the split")
            print(ranges)
            continue
        if not finished_ranges:
            range = line.strip().split("-")
            ranges.append((int(range[0]), int(range[1])))
        else:
            val = int(line.strip())
            print(val)
            for r in ranges:
                if r[0] <= val <= r[1]:
                    count += 1
                    break
    print(count)
