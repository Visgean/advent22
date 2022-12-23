import re

SIZE_RE = re.compile('\d+ \w+')

small_sizes = []

class Node:
    def __init__(self, name, parent):
        self.files = {}
        self.folders = {}
        self.name = name
        self.parent = parent

    def size(self):
        size = 0
        size += sum(self.files.values())
        size += sum([f.size() for f in self.folders.values()])

        return size

    def add_subfolder(self, name):
        if name not in self.folders:
            self.folders[name] = Node(
                name, self
            )

    def flatten_small_dirs(self):
        size = self.size()

        if size < 100000 and self.parent:
            print(self.name, size)
            small_sizes.append(size)

        for folder in self.folders.values():
            folder.flatten_small_dirs()


    def print(self, indent=0):
        indent_str = '-' * indent
        print(f'{indent_str}{self.name}')
        for file in self.files.keys():
            print(f'{indent_str}{file}')
        for folder in self.folders.values():
            folder.print(indent + 2)


def day7():
    with open('day7') as f:
        lines = f.read().splitlines()

    root = Node(
        '/', None
    )

    current_dir = root
    for line in lines:

        if line.startswith('dir'):
            dirname = line[4:]
            current_dir.add_subfolder(dirname)

        elif SIZE_RE.match(line):
            size_str, filename = line.split(' ')
            current_dir.files[filename] = int(size_str)
        elif line == '$ cd /':
            current_dir = root

        elif line == '$ cd ..':
            current_dir = current_dir.parent

        elif line.startswith('$ cd '):
            target = line[5:]
            current_dir.add_subfolder(target)
            current_dir = current_dir.folders[target]



    root.print()
    root.flatten_small_dirs()
    print(sum(small_sizes))
    #
    print(root.size())


day7()
