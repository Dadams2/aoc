# if odd length must be repeating digits
# if even lenght start must be equal to end


with open("in.txt") as f:
    count = 0 
    ranges = f.readlines()[0].strip().split(",")
    for r in ranges:
        vals = r.split("-")
        start = int(vals[0])
        end = int(vals[1])
        for i in range(start, end+1):
            num = str(i)
            l = len(num)
            for j in range(1, l//2+1):
                f = num[:j]
                pattern = f * (l // j)
                if pattern == num:
                    print(pattern, num)
                    count += i
                    break
    print(count)