def score(word):
    return sum(
        [1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10][ord(i) - ord('a')] for i in
        word.lower() if i.isalpha())