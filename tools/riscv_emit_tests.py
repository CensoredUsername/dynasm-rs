#!/usr/bin/python3

# this file takes the compiled testcase listing emitted by `riscv_compile_tests.py` and
# compiles this to a set of testcase files for dynasm-rs

import os.path
import argparse
from pathlib import Path

BLACKLIST = set()

def read_input_file(f):
    buf = []
    for line in f:
        dynasm, gas, extensions, bytes = line.split("\t")
        buf.append((dynasm.strip(), gas.strip(), extensions.strip(), bytes.strip()))
    return buf

def chunks(l, n):
    for i in range(0, len(l), n):
        yield l[i:i+n]

def main():
    parser = argparse.ArgumentParser("riscv_emit_tests",  description="compile dynasm-rs riscv testcase files.")
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--rv32", help="emit tests for riscv-32", action="store_true")
    group.add_argument("--rv64", help="emit tests for riscv-64", action="store_true")

    parser.add_argument("input_file", help="The output of riscv_compile_tests.py", type=Path)
    parser.add_argument("output_folder", help="The folder in which to deposit the test files", type=Path)

    args = parser.parse_args()

    with args.input_file.open("r", encoding="utf-8") as f:
        data = read_input_file(f)

    bits = "32" if args.rv32 else "64"

    tests = [emit_test_case(i, dynasm, gas, extensions, bytes, bits) for i, (dynasm, gas, extensions, bytes) in enumerate(data)]

    if not args.output_folder.exists():
        args.output_folder.mkdir()

    for i, chunk in enumerate(chunks(tests, 800)):
        with (args.output_folder / f"riscv{bits}_tests_{i}.rs.gen").open("w", encoding="utf-8") as f:
            for test in chunk:
                f.write(test)

def emit_test_case(i, dynasm, gas, extensions, bytes, bits):
    name = dynasm.split(' ', 1)[0].replace(".", "_")
    if name in BLACKLIST:
        return ""
    bytes = ", ".join(chunks(bytes, 2)).upper()
    error = dynasm.replace("{", "{{").replace("}", "}}")
    return f"""
#[test]
fn {name.lower()}_{i}() {{
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv{bits}
        ; .feature {extensions}
        ; {dynasm}
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{{:02X}}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "{bytes}", "{error}");
}}
"""



if __name__ == "__main__":
    main()
