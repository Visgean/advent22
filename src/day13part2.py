import json
from functools import cmp_to_key
from itertools import zip_longest

def get_messages(input_file='../inputs/day13'):
    with open(input_file) as f:
        data = f.read()

    lines = list(data.splitlines())

    messages = []

    for line_no in range(0, len(lines), 3):
        messages.append(json.loads(lines[line_no]))
        messages.append(json.loads(lines[line_no + 1]))

    return messages


def is_ordered(a, b, indent=0):
    indent_str = " " * indent
    print(f"{indent_str}- Compare {a} vs {b}")

    if a is None:
        return True
    if b is None:
        return False

    if isinstance(a, int) ^ isinstance(b, int):
        if isinstance(a, int):
            return is_ordered([a], b, indent + 2)
        elif isinstance(b, int):
            return is_ordered(a, [b], indent + 2)


    if isinstance(a, int) and isinstance(b, int):
        if a == b:
            return None
        return a <= b
    elif isinstance(a, list) and isinstance(b, list):
        for x, y in zip_longest(a, b, fillvalue=None):
            res = is_ordered(x, y, indent + 2)
            if res is not None:
                return res

    return None


def custom_sort(a, b):
    if is_ordered(a, b):
        return 1
    return -1


def main():
    messages = get_messages()

    div_packet1 = [[2]]
    messages.append(div_packet1)
    div_packet2 = [[6]]
    messages.append(div_packet2)
    messages.sort(reverse=True, key=cmp_to_key(custom_sort))
    for m in messages:
        print(m)


    div_packet1_loc = messages.index(div_packet1) + 1
    div_packet2_loc = messages.index(div_packet2) + 1
    div_prod = div_packet1_loc * div_packet2_loc


    print('div packet:', div_packet1_loc)
    print('div packet:', div_packet2_loc)
    print('div packet product:', div_prod)





if __name__ == '__main__':
    main()
