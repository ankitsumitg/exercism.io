import re


def count_words(sentence):
    d = {}
    for i in re.findall(r'[a-zA-Z]+\'*[a-zA-Z]+|[0-9]+|a', sentence.lower()):
        d[i] = d.get(i, 0) + 1
    return d
