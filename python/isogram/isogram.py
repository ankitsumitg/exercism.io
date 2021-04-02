def is_isogram(string):
    s = string.replace('-', '').replace(' ', '').lower()
    return len(s) == len(set(s))
