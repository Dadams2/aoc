
with open("in.txt") as f:
    rows = []
    count = 0
    for line in f:
        rows.append(line.strip())
    removed = 1
    while(removed > 0):
        removed = 0
        newrows = []
        for i in range(len(rows)):
            newrow = ""
            for j in range(len(rows[0])):
                surround = 0
                curr = rows[i][j]
                if curr != "@":
                    newrow += curr
                    continue
                # up
                if i < len(rows)-1 and rows[i+1][j] == "@":
                    surround += 1 
                # down
                if i > 0 and rows[i-1][j] == "@":
                    surround += 1
                # right
                if j < len(rows[0])-1 and rows[i][j+1] == "@":
                    surround += 1 
                # left
                if j > 0 and rows[i][j-1] == "@":
                    surround += 1
                # lower left
                if i > 0 and j > 0 and rows[i-1][j-1] == "@":
                    surround += 1
                # lower right
                if i > 0 and j < len(rows[0])-1 and rows[i-1][j+1] == "@":
                    surround += 1
                # upper right 
                if i < len(rows)-1 and j < len(rows[0])-1 and rows[i+1][j+1] == "@":
                    surround += 1
                # upper left
                if i < len(rows)-1 and j > 0 and rows[i+1][j-1] == "@":
                    surround += 1
                # we can actually remove this one
                if surround < 4:
                    newrow += "."
                    removed += 1
                    count += 1
                else:
                    newrow += curr
            newrows.append(newrow)
        rows = newrows
    print(count)


    