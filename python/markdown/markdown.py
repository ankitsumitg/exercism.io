import re


def parse(markdown):
    lines = markdown.split('\n')
    res = ''
    in_list = False
    for i in lines:
        if i.startswith('###### '):
            i = '<h6>' + i[7:] + '</h6>'
        elif i.startswith('## '):
            i = '<h2>' + i[3:] + '</h2>'
        elif i.startswith('# '):
            i = '<h1>' + i[2:] + '</h1>'
        elif i.startswith('* '):
            i = '<li>' + i[2:] + '</li>'
            if not in_list:
                i = '<ul>' + i
                in_list = True
        else:
            i = '<p>' + i + '</p>'
            if in_list:
                i = '</ul>' + i
                in_list = False
        res += trans_strong_em(i)
    if in_list:
        res += '</ul>'
    return res


def trans_strong_em(s):
    output = s
    strong_match = re.match('(.*)__(.*)__(.*)', output)
    if strong_match:
        output = strong_match.group(1) + '<strong>' + strong_match.group(2) + '</strong>' + strong_match.group(3)
    em_match = re.match('(.*)_(.*)_(.*)', output)
    if em_match:
        output = em_match.group(1) + '<em>' + em_match.group(2) + '</em>' + em_match.group(3)
    return output


print(parse("__This will be bold__"))
