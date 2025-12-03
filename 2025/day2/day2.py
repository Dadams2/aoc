# if odd length must be repeating digits but as it must be repeated twice we can discard this case
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
            if l % 2 == 0:
                f = num[:l//2]
                l = num[l//2:]
                if int(f) == int(l):
                    count += i 
    print(count)