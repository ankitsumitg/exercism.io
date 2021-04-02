class Luhn:
    def __init__(self, card_num):
        self.ss = ''.join(card_num.split())
        try:
            self.ss = list(map(int, list(self.ss)))
            self.ss = [0] + self.ss if len(self.ss) & 1 else self.ss
        except:
            self.ss = [0]
        finally:
            if self.ss == [0, 0]:
                self.ss = [0]

    def valid(self):
        if self.ss == [0]:
            return False
        return sum(j + (i * 2 if i * 2 < 9 else i * 2 - 9) for i, j in zip(self.ss[0::2], self.ss[1::2])) % 10 == 0
