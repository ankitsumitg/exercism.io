class Clock:
    def __init__(self, hour, minute):
        self.t = (hour * 60 + minute) % 1440

    def __repr__(self):
        return f'{self.t // 60:02d}:{self.t % 60 :02d}'

    def __eq__(self, other):
        return self.t == other.t

    def __add__(self, minutes):
        return Clock(0, self.t + minutes)

    def __sub__(self, minutes):
        return Clock(0, self.t - minutes)