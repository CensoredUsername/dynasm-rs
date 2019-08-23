This folder contains the tools used to generate various data / test files used for dynasm.

Aarch64
#######

- `aarch64_gen_opmap.py`: Parses the Machine-Readable Architecture specifications for ARMv8 as produced by ARM, and combined with several translation files in the `aarch64_data` folder produces the `opmap.rs` file for the aarch64 assembler.
- `aarch64_gen_tests.py`: Parses an export of this `opmap.rs` file as produced by dynasm with the `dynasm_extract` feature used and based on this file, generates a file of dynasm-dialect assembly vs gnu as-dialect assembly.
- `aarch64_compile_tests.py`: Reads the previous file, feeds all the gnu as-dialect assembly lines through `as` and records the binary representation of the assembled data next to the assembly strings.
- `aarch64_emit_tests.py`: Takes the output of the previous step and uses it to generate the testcases in `testing/tests/gen_aarch64` that can then be used to validate dynasm.
