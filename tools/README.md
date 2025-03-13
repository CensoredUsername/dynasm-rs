This folder contains the tools used to generate various data / test files used for dynasm.

Aarch64
#######

- `aarch64_gen_opmap.py`: Parses the Machine-Readable Architecture specifications for ARMv8 as produced by ARM, and combined with several translation files in the `aarch64_data` folder produces the `opmap.rs` file for the aarch64 assembler.
- `aarch64_gen_tests.py`: Parses an export of this `opmap.rs` file as produced by dynasm with the `dynasm_extract` feature used and based on this file, generates a file of dynasm-dialect assembly vs gnu as-dialect assembly.
- `aarch64_compile_tests.py`: Reads the previous file, feeds all the gnu as-dialect assembly lines through `as` and records the binary representation of the assembled data next to the assembly strings.
- `aarch64_emit_tests.py`: Takes the output of the previous step and uses it to generate the testcases in `testing/tests/gen_aarch64` that can then be used to validate dynasm.


RISC-V
######

### Basic opmap generation

- `riscv_load_data.py`: Parses the instruction encoding data from the `riscv_data/riscv-opcodes` as a library
- `riscv_gen_opmap.py`: Builds a rough version of the risc-v opmap from the data provided by `riscv_data/risc-v opcodes`.

### Test suite generation

- First go to `../doc/insref` and generate an opcode data dump using `cargo run --bin=export -- riscv > opmap_export.txt`
- Navigate back to this folder and then use `python3 riscv_gen_tests.py riscv_opmap_export.txt 2 testcases.txt` to generate `rv32_testcases.txt` and `rv64_testcases.txt`
- Use `python3 riscv_compile_tests.py --rv32 rv32_testcases.txt rv32_compiled.txt` and `python3 riscv_compile_tests.py --rv64 rv64_testcases.txt rv64_compiled.txt` 
- Finally, use `python3 riscv_emit_tests.py --rv32 rv32_compiled.txt ../testing/tests/gen_riscv32/` and `python3 riscv_emit_tests.py --rv64 rv64_compiled.txt ../testing/tests/gen_riscv64/` to generate a new test suite.
