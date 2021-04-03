def tally(rows):
    table = ["Team                           | MP |  W |  D |  L |  P"]
    d = {}
    for i in rows:
        name1, name2, play = i.split(';')
        if name1 not in d:
            d[name1] = [0] * 5
        if name2 not in d:
            d[name2] = [0] * 5
        d[name1][0] += 1
        d[name2][0] += 1
        if play == 'win':
            d[name1][1] += 1
            d[name2][3] += 1
            d[name1][-1] += 3
        elif play == 'draw':
            d[name1][2] += 1
            d[name2][2] += 1
            d[name1][-1] += 1
            d[name2][-1] += 1
        else:
            d[name1][3] += 1
            d[name2][1] += 1
            d[name2][-1] += 3
    lst = [[k] + v for k, v in d.items()]
    lst.sort(key=lambda x: (-x[-1], x[0]))
    for k in lst:
        s = k[0].ljust(31) + '|'
        for i in range(1, 5):
            s += str(k[i]).rjust(3).ljust(4) + '|'
        s += str(k[-1]).rjust(3)
        table.append(s)
    return table
