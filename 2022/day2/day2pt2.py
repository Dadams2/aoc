
points = {
    "X": 1,
    "Y": 2,
    "Z": 3
}

win_map = {
    "A" : {
        "X": "Z",
        "Y": "X",
        "Z": "Y" 
    } ,
    "B" : {
        "X": "X", 
        "Y": "Y",
        "Z": "Z" 
    },
    "C": {
        "X": "Y",
        "Y": "Z",
        "Z": "X"
    }
}

def win(o, me):
    if o == "A" and me == "X":
        return 3, points["X"]
    if o == "B" and me == "X":
        return 0, points["X"]
    if o == "C" and me == "X":
        return 6, points["X"]
    

    if o == "A" and me == "Y":
        return 6, points["Y"]
    if o == "B" and me == "Y":
        return 3, points["Y"]
    if o == "C" and me == "Y":
        return 0, points["Y"]


    if o == "A" and me == "Z":
        return 0, points["Z"]
    if o == "B" and me == "Z":
        return 6, points["Z"]
    if o == "C" and me == "Z":
        return 3, points["Z"]

    return 0
    

with open("day2.in") as f:
    score = 0
    for line in f:
        vals = line.strip().split(" ")
        o = vals[0]
        me = vals[1]
        w, p = win(o, win_map[o][me])
        score += p + w
    print(score)