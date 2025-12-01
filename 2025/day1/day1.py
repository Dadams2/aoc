
with open('in.txt') as f:
    s = 50
    count = 0
    for line in f:
        line = line.strip()
        d = line[0]
        n = int(line[1:])
        if d == 'L':
            for i in range(n):
                s -= 1
                if s == 0:
                    count += 1
                if s < 0:
                    s += 100
        if d == 'R':
            for i in range(n):
                s += 1
                if s == 100:
                    s -= 100
                    count += 1
        # print(f"direction {d} number {n}")
        # if d == 'L':
        #     count += abs((s-n)//100)
        #     s = (s-n)%100
        # if d == 'R':
        #     count += (s+n)//100
        #     s = (s+n)%100
        # print(f"final {s}")
        # if s == 0:
        #     count += 1
    print(s)
    print(count)