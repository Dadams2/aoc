

with open("in.txt") as f:
    count = 0
    for line in f:
        num = line.strip()
        l = len(num)
        prev = 0
        indicies = [0 for _ in range(12)]
        for i in range(12):
            max = 0
            for j in range(prev, l-(12 - i)+1):
                if int(num[j]) > max:
                    max = int(num[j])
                    indicies[i] = j
                    # print(f"i {i} j {j} num[j] {num[j]} max {max}")
                    prev = j+1
            print(max)
            print(indicies)
        # print(f"indicies {indicies}")
        value = int("".join([num[i] for i in indicies]))
        # print(f" {num} value {value}")
        count += value
        # break
    print(count)
        
        
            
            

                