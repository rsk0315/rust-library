import glob
import json
import os
import re


FILE_RE = re.compile(r'src/([^/]+)/([^/]+)\.rs$')
DEFINE_RE = re.compile(r'^\s*pub (?:struct|trait) (\w+)')
USE_SINGLE_RE = re.compile(r'^\s*(?:pub )?use crate::(\w+)::(\w+|\*)')
USE_MULTIPLE_RE = re.compile(r'^\s*(?:pub )?use crate::(\w+)::\{([\w, ]+)\};')
USE_MULTILINE_RE = re.compile(r'^\s*(?:pub )?use crate::(\w+)::\{$')

DOC_COMMENT_RE = re.compile(r'\s*//!')
TEST_ATTR_RE = re.compile(r'#\[test\]|#\[cfg\(test\)\]')

LIB_ROOT = f'{os.path.expanduser("~")}/git/rsk0315/rust-library'


def skip_test(fin):
    # 複数行文字列・コメントとかで最後に { や } があるとこわれる気がするよ
    opening = 0
    for line in fin:
        line = line.strip()
        if line.endswith('{'):
            opening += 1
        elif line.endswith('}'):
            opening -= 1

        if opening == 0: return


def parse(fin, deps):
    dirname, basename = FILE_RE.search(fin.name).group(1, 2)
    defines = [f'{dirname}::{basename}']
    uses = []
    if dirname not in deps:
        deps[dirname] = {'defines': []}

    for line in fin:
        line = line.rstrip()
        if DOC_COMMENT_RE.match(line): continue
        if TEST_ATTR_RE.match(line):
            skip_test(fin)
            continue

        if (m := DEFINE_RE.match(line)) is not None:
            defines.append(f'{dirname}::{m.group(1)}')
        elif (m := USE_SINGLE_RE.match(line)) is not None:
            uses.append(f'{m.group(1)}::{m.group(2)}')
        elif (m := USE_MULTIPLE_RE.match(line)) is not None:
            major = m.group(1)
            minors = m.group(2).split(', ')
            for minor in minors:
                uses.append(f'{major}::{minor.rstrip(",")}')
        elif (m := USE_MULTILINE_RE.match(line)) is not None:
            major = m.group(1)
            minors = []
            for line in fin:
                line = line.strip()
                if line.startswith('}'): break
                minors.extend(line.split(', '))
            for minor in minors:
                uses.append(f'{major}::{minor.rstrip(",")}')

    deps[dirname]['defines'].extend(defines)
    for define in defines:
        deps[define] = {
            'defined': f'{dirname}/{basename}.rs',
            'uses': uses
        }


def analysis_deps():
    rs = glob.glob(f'{LIB_ROOT}/src/*/*.rs')
    deps = {}
    for f in rs:
        with open(f) as fin:
            parse(fin, deps)

    return deps


def main():
    print(json.dumps(analysis_deps(), indent=4))


if __name__ == '__main__':
    main()
