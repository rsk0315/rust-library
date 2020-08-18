import json
import os
import re
import sys

import make_index


FILE_RE = re.compile(r'([^/]+)/([^/]+)\.rs')
USE_SINGLE_RE = re.compile(r'^\s*(?:pub )?use my::(\w+)::(\w+|\*)')
USE_MULTIPLE_RE = re.compile(r'^\s*(?:pub )?use my::(\w+)::\{([\w, ]+)\};')
USE_MULTILINE_RE = re.compile(r'^\s*(?:pub )?use my::(\w+)::\{$')

DOC_COMMENT_RE = re.compile(r'\s*//!')
TEST_ATTR_RE = re.compile(r'#\[test\]|#\[cfg\(test\)\]')

SEPARATOR = '''
// -------- Following codes are bundled automatically. -------- //

'''

LIB_ROOT = f'{os.path.expanduser("~")}/git/rsk0315/rust-library'


def get_uses(fin):
    output = ''
    uses = []
    use_lines = []
    for line in fin:
        if (m := USE_SINGLE_RE.match(line)) is not None:
            output += f'// {line}'
            use_lines.append(line)
            uses.append(f'{m.group(1)}::{m.group(2)}')
        elif (m := USE_MULTIPLE_RE.match(line)) is not None:
            output += f'// {line}'
            use_lines.append(line)
            major = m.group(1)
            minors = m.group(2).split(', ')
            for minor in minors:
                uses.append(f'{major}::{minor.rstrip(",")}')
        elif (m := USE_MULTILINE_RE.match(line)) is not None:
            output += f'// {line}'
            use_lines.append(line)
            major = m.group(1)
            minors = []
            for line in fin:
                output += f'// {line}'
                use_lines.append(line)
                if line.startswith('}'): break
                minors.extend(line.strip().split(', '))
            for minor in minors:
                uses.append(f'{major}::{minor.rstrip(",")}')
        else:
            output += line

    return output, uses, use_lines


def get_index():
    return make_index.analysis_deps()


def resolve_deps(uses):
    uses = set(uses)
    used = set()
    deps = set()
    index = get_index()
    while uses:
        use = uses.pop()
        if use in used: continue
        used.add(use)
        deps.add(index[use]['defined'])
        for use_next in index[use]['uses']:
            if use_next.endswith('::*'):
                uses.update(index[use_next[:-3]]['defines'])
            else:
                uses.add(use_next)

    return deps


def bundle(output, bundled, use_lines):
    rs = {}
    for inname in bundled:
        dirname, basename = FILE_RE.fullmatch(inname).group(1, 2)
        if dirname not in rs:
            rs[dirname] = []
        rs[dirname].append(basename)

    output += SEPARATOR

    for dirname in rs:
        output += f'pub mod {dirname} {{\n'
        first = True
        for basename in rs[dirname]:
            if not first: output += '\n'
            output += f'    pub mod {basename} {{\n'
            blank = False
            with open(f'{LIB_ROOT}/src/{dirname}/{basename}.rs') as fin:
                for line in fin:
                    if DOC_COMMENT_RE.match(line): continue
                    if TEST_ATTR_RE.match(line):
                        make_index.skip_test(fin)
                        blank = False
                        continue
                    if line == '\n':
                        blank = True
                        continue

                    if blank: line += '\n'
                    blank = False
                    output += '        '
                    output += line
            output += f'    }}\n'
            output += f'    pub use {basename}::*;\n'
            first = False

        output += f'}}\n\n'

    for use in use_lines:
        output += use.replace('my::', 'crate::', 1)

    return output


def main():
    if len(sys.argv) <= 1:
        print(f'{sys.argv[0]} infile', file=sys.stderr)
        return 1

    infile = sys.argv[1]
    with open(infile) as fin:
        output, uses, use_lines = get_uses(fin)

    bundled = resolve_deps(uses)
    output = bundle(output, bundled, use_lines)
    print(output.rstrip())


if __name__ == '__main__':
    exit(main())
