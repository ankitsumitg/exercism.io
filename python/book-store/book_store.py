def total(basket):
    groups = []
    while basket:
        uniques = set(basket)
        for x in uniques:
            basket.remove(x)
        groups.append(len(uniques))
    # group of 4 and 4 is less than 5,3
    while 3 in groups and 5 in groups:
        groups.remove(5)
        groups.remove(3)
        groups.append(4)
        groups.append(4)
    tots = {
        1: 800,
        2: (800 * 2) * 0.95,
        3: (800 * 3) * 0.9,
        4: (800 * 4) * 0.8,
        5: (800 * 5) * 0.75,
    }
    return int(sum(tots[x] for x in groups))
