import glob
import json
import re


FILE_RE = re.compile(r'src/([^/]+)/([^/]+)\.rs')
DEFINE_RE = re.compile(r'^\s*pub (?:struct|trait) (\w+)')
USE_SINGLE_RE = re.compile(r'^\s*(?:pub )?use crate::(\w+)::(\w+|\*)')
USE_MULTIPLE_RE = re.compile(r'^\s*(?:pub )?use crate::(\w+)::\{([\w, ]+)\};')
USE_MULTILINE_RE = re.compile(r'^\s*(?:pub )?use crate::(\w+)::\{$')


def parse(fin, deps):
    dirname, basename = FILE_RE.fullmatch(fin.name).group(1, 2)
    defines = [f'{dirname}::{basename}']
    uses = []
    if dirname not in deps:
        deps[dirname] = {'defines': []}

    for line in fin:
        line = line.rstrip()
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
    rs = glob.glob('src/*/*.rs')
    deps = {}
    for f in rs:
        with open(f) as fin:
            parse(fin, deps)

    return deps


def main():
    print(json.dumps(analysis_deps(), indent=4))


if __name__ == '__main__':
    main()
