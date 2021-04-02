class School:
    def __init__(self):
        self.d = {}

    def add_student(self, name, grade):
        if grade not in self.d:
            self.d[grade] = []
        self.d[grade] += [name]

    def roster(self):
        ans = []
        for i in sorted(self.d.keys()):
            ans += sorted([j for j in self.d[i]])
        return ans

    def grade(self, grade_number):
        if grade_number not in self.d:
            return []
        return sorted(self.d[grade_number])
