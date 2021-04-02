class Garden:
    def __init__(self, diagram, students=None):
        if students is None:
            students = ['Alice', 'Bob', 'Charlie', 'David', 'Eve', 'Fred', 'Ginny', 'Harriet', 'Ileana', 'Joseph',
                        'Kincaid', 'Larry']
        self.d = {
            'G': 'Grass',
            'C': 'Clover',
            'R': 'Radishes',
            'V': 'Violets'
        }
        self.diagram = diagram.split('\n')
        self.students = sorted(students)

    def plants(self, student):
        index = self.students.index(student)
        return [self.d[j] for i in self.diagram for j in i[index * 2:index * 2 + 2]]
