#!/usr/bin/env python
"""
Check that all the functions defined in the chemfiles-sys crate are
effectivelly used in the chemfiles binding.
"""
import os

ROOT = os.path.dirname(os.path.dirname(__file__))


def functions_list():
    functions = []
    with open(os.path.join(ROOT, "chemfiles-sys", "lib.rs")) as fd:
        extern = False
        for line in fd:
            if "extern \"C\" {" in line:
                extern = True
                continue
            if extern and "}" in line:
                extern = False
                continue
            if extern:
                name = line.split()[2].split('(')[0]
                functions.append(name)
    return functions


def read_all_source():
    source = ""
    for (dirpath, _, pathes) in os.walk(os.path.join(ROOT, "src")):
        for path in pathes:
            with open(os.path.join(ROOT, dirpath, path)) as fd:
                source += fd.read()
    return source


def check_functions(functions, source):
    for function in functions:
        if function not in source:
            print("Missing: " + function)


if __name__ == '__main__':
    functions = functions_list()
    source = read_all_source()
    check_functions(functions, source)
