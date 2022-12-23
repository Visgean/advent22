import re

SIZE_RE = re.compile('\d+ \w+')

flat_sizes = []

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

    def flatten(self):
        size = self.size()

        flat_sizes.append(size)
        for folder in self.folders.values():
            folder.flatten()


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

    total_space = 70000000
    used_space = root.size()
    unused_space = total_space - used_space
    update_space_required = 30000000

    root.flatten()

    targets = [
        t
        for t in flat_sizes
        if (t + unused_space) > update_space_required
    ]

    print(sorted(targets)[0])




day7()
