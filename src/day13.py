import json
from itertools import zip_longest

def get_pairs(input_file='../inputs/day13'):
    with open(input_file) as f:
        data = f.read()

    lines = list(data.splitlines())

    pairs = []

    for line_no in range(0, len(lines), 3):
        a = json.loads(lines[line_no])
        b = json.loads(lines[line_no + 1])

        pairs.append((a, b))

    return pairs


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



def main():
    pairs = get_pairs()

    total_ordered = []
    for i, (a, b), in enumerate(pairs):
        index = i + 1
        print(f"== Pair {index} ==")
        if is_ordered(a, b):
            print('ordered', index)
            total_ordered.append(index)
        else:
            print(" -> unordered")
        print()

    print(total_ordered)
    print(sum(total_ordered))




if __name__ == '__main__':
    main()
