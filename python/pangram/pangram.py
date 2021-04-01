def is_pangram(sentence):
    return len(set(i for i in sentence.lower() if i.isalpha())) == 26
