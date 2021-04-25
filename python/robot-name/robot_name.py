import random


class Robot:
    def __init__(self):
        self.name = new()

    def reset(self):
        self.__init__()


def new():
    random.seed()
    arr = [chr(i) for i in range(ord('A'), ord('Z') + 1)]
    arr2 = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']
    return random.choice(arr) + random.choice(arr) + random.choice(arr2) + random.choice(arr2) + random.choice(arr2)
