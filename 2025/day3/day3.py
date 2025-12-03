

with open("in.txt") as f:
    count = 0
    for line in f:
        num = line.strip()
        i = 0
        start = 0
        end = 1
        l = len(num)
        while(i<l):
            if num[i] > num[start] and i < l-1:
                start = i
                end = i+1
            elif num[i] > num[end]:
                end = i
            i += 1
        count += 10*int(num[start]) + int(num[end])
        print(f"{num} first high {num[start]} second {num[end]}")
    print(count)
            
            

                