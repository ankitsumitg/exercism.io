ROW_FORMAT = '{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}'


def tally(rows):
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
    lst.insert(0, ["Team", "MP", "W", "D", "L", "P"])
    return [ROW_FORMAT.format(*i) for i in lst]
