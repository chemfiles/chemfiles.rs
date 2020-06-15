#!/usr/bin/env python
"""
Check that all the functions defined in the chemfiles-sys crate are
effectively used in the chemfiles binding.
"""
import os
import sys
import re

ERROR = False
ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")


def error(message):
    print(message)
    global ERROR
    ERROR = True


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


def read_all_binding_functions():
    binding_functions = set()
    for (dirpath, _, paths) in os.walk(os.path.join(ROOT, "src")):
        for path in paths:
            with open(os.path.join(ROOT, dirpath, path)) as fd:
                # https://doc.rust-lang.org/nightly/reference/identifiers.html
                file_functions = re.findall(r"(chfl_[a-z A-Z 0-9 _]*)\(", fd.read())
                binding_functions.update(file_functions)
    return binding_functions


def check_functions(functions, binding_functions):
    for function in functions:
        if function not in binding_functions:
            error("Missing: " + function)


if __name__ == '__main__':
    functions = functions_list()
    binding_functions = read_all_binding_functions()
    check_functions(functions, binding_functions)

    if ERROR:
        sys.exit(1)
