class Clock:
    def __init__(self, hour, minute):
        self.t = hour * 60 + minute

    def __repr__(self):
        return str((self.t // 60) % 24).zfill(2) + ':' + str(self.t % 60).zfill(2)

    def __eq__(self, other):
        return (self.t // 60) % 24 == (other.t // 60) % 24 and self.t % 60 == other.t % 60

    def __add__(self, minutes):
        return Clock(0, self.t + minutes)

    def __sub__(self, minutes):
        return Clock(0, self.t - minutes)
