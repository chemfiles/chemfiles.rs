#!/usr/bin/env python
"""
Check that all the functions defined in the chemfiles C API are
declared in the chemfiles `lib.rs` file. Utilizes `bindgen`.
"""
import os
import subprocess
import sys
import re

ERROR = False
ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")


def error(message):
    print(message)
    global ERROR
    ERROR = True


def functions_list():
    function_headers = set()
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
                func_header = re.findall(r"(chfl_[a-z A-Z 0-9 _]*)\(", line)
                function_headers.update(func_header)
    return function_headers


def read_current_function_list():
    function_headers = set()
    result = subprocess.run(["bindgen",
            os.path.join(ROOT, "chemfiles-sys", "chemfiles", "include", "chemfiles.h"),
            "--",
            "-I", os.path.join(ROOT, "chemfiles-sys", "chemfiles", "include"),
            "-I", os.path.join(ROOT, "chemfiles-sys-tests")
        ], stdout=subprocess.PIPE)
    dump_str = result.stdout.decode("utf-8")
    extern = False
    for line in dump_str.splitlines():
        if line == "extern \"C\" {":
            extern = True
            continue
        if extern and line == "}":
            extern = False
            continue
        if extern:
            func_header = re.findall(r"(chfl_[a-z A-Z 0-9 _]*)\(", line)
            function_headers.update(func_header)
    return function_headers


def check_functions(functions, current_functions):
    missing_functions = current_functions.difference(functions)
    if len(missing_functions) > 0:
        for function in missing_functions:
            error("Missing: " + function)
    deprecated_functions = functions.difference(current_functions)
    if len(deprecated_functions) > 0:
        for function in deprecated_functions:
            error("Deprecated: " + function)


if __name__ == '__main__':
    functions = functions_list()
    current_functions = read_current_function_list()
    check_functions(functions, current_functions)

    if ERROR:
        sys.exit(1)
