% riscv assembly language reference

# Lexical structure definition

Instructions for the `riscv` assembling backends use the following lexical structure:

## Base units

The following base syntax units are recognized by the parser.

- `static_reg_name` matches any valid register name as seen in table 2, or any previously defined alias
- `dynamic_reg_family` matches any valid register family from table 2

## Instruction

`instruction : ident ("." ident)* (arg ("," arg)* )? ;`

## Arguments

`arg : register | registerlist | labelref | reference | expr ;`

`register : static_reg_name | dynamic_reg_family "(" expr ")" ;`

`register_list : "{ comma_list | amount_list "}";`

`comma_list : register ("," register ("-" register)? )?  ;`

`amount_list : register ";" expr ;`

`reference : "[" register ("," expr | labelref)? "]"  ;`

# Reference

## Targets

The RISC-V instruction set family comprises several different architectures. At the time of writing, dynasm-rs supports the following targets, which can be selected using the `.arch` directive:

Table 1: dynasm-rs RISC-V architecture support

Instruction set | Directive        | Integer register width | Integer register count |
---------------:|:-----------------|:-----------------------|:-----------------------|
`RV32I`         | `.arch riscv32i` | `32`                   | `32`                   |
`RV32E`         | `.arch riscv32e` | `32`                   | `16`                   |
`RV64I`         | `.arch riscv64i` | `64`                   | `32`                   |
`RV64E`         | `.arch riscv64e` | `64`                   | `16`                   |

### Instruction Set Extensions

The RISC-V instruction set family has a small base instruction set, and defines a large set of extensions. These extensions are either identified by a single letter like `A`, or a longer name starting with a `Z` like `Zifencei`. The full set of extensions for a RISC-V instruction set is identified by concatenating these instruction set identifiers, wherein underscores are added after longer names, combining into identifiers like `IMAFDZicsr_Zifencei`.

Selecting the active set of instruction set extensions in dynasm-rs is done using the `.feature` directive. It is possible to pass in a full instruction set identifier into this directive, or a comma-separated list of instruction set extension identifiers. Instruction set identifiers are case-insensitive. The following examples have identical behaviour:

- `.feature IMAFDZicsr_Zifencei`
- `.feature I, M, A, F, D, Zicsr, Zifencei`
- `.feature IMAFD, Zicsr, Zifencei`
- `.feature imafdzicsr_zifencei`

## Instructions

At the time of writing, the official RISC-V Assembly Programmer's manual is still in development state at version `0.0.1`. It currently doesn't cover a significant part of the syntax that is used in much of the RISC-V documentation. The assembly language used by dynasm-rs in riscv mode is therefore inspired by the assembly dialect used by the GNU assembler. Several additions have been made to support dynamic registers, and to ensure the Rust parser can parse it.

A significant difference exists in the syntax used for memory references. The GNU assembler uses `offset(base_register)` syntax for these. Use of this syntax in dynasm-rs would cause parsing ambiguities as it is unclear if the given expression should be parsed as an immediate that contains a function call, or a memory reference. Therefore, the dynasm-rs RISC-V assembly language uses arm-style `[base, offset]` memory references.

### Operands

#### Register

There are two ways to reference registers in dynasm-rs, either via their static name, or via dynamic register references. Dynamic register references allow the exact register choice to be made at runtime. Please note that the expression inside a dynamic register reference may be evaluated multiple times during assembly of the instruction.

The following table lists all available static registers, their dynamic family name and their encoding when they are used dynamically. Note that when the architecture is set to `riscv32e` or `riscv64e`, only the first 16 integer registers can be used.

Table 2: dynasm-rs registers (RISC-V)

Family            | integer     | floating point | vector |
-----------------:|:------------|:---------------|:-------|
Dynamic Encoding  | `X`         | `W`            | `V`    |
              `0` | `x0/zero`   | `f0/ft`        | `v0`   |
              `1` | `x1/ra`     | `f1/ft`        | `v1`   |
              `2` | `x2/sp`     | `f2/ft`        | `v2`   |
              `3` | `x3/gp`     | `f3/ft`        | `v3`   |
              `4` | `x4/tp`     | `f4/ft`        | `v4`   |
              `5` | `x5/t0`     | `f5/ft`        | `v5`   |
              `6` | `x6/t1`     | `f6/ft`        | `v6`   |
              `7` | `x7/t2`     | `f7/ft`        | `v7`   |
              `8` | `x8/s0/fp`  | `f8/fs`        | `v8`   |
              `9` | `x9/s1`     | `f9/fs`        | `v9`   |
             `10` | `x10/a0`    | `f10/fa`       | `v10`  |
             `11` | `x11/a1`    | `f11/fa`       | `v11`  |
             `12` | `x12/a2`    | `f12/fa`       | `v12`  |
             `13` | `x13/a3`    | `f13/fa`       | `v13`  |
             `14` | `x14/a4`    | `f14/fa`       | `v14`  |
             `15` | `x15/a5`    | `f15/fa`       | `v15`  |
             `16` | `x16/a6`    | `f16/fa`       | `v16`  |
             `17` | `x17/a7`    | `f17/fa`       | `v17`  |
             `18` | `x18/s2`    | `f18/fs`       | `v18`  |
             `19` | `x19/s3`    | `f19/fs`       | `v19`  |
             `20` | `x20/s4`    | `f20/fs`       | `v20`  |
             `21` | `x21/s5`    | `f21/fs`       | `v21`  |
             `22` | `x22/s6`    | `f22/fs`       | `v22`  |
             `23` | `x23/s7`    | `f23/fs`       | `v23`  |
             `24` | `x24/s8`    | `f24/fs`       | `v24`  |
             `25` | `x25/s9`    | `f25/fs`       | `v25`  |
             `26` | `x26/s10`   | `f26/fs`       | `v26`  |
             `27` | `x27/s11`   | `f27/fs`       | `v27`  |
             `28` | `x28/t3`    | `f28/ft`       | `v28`  |
             `29` | `x29/t4`    | `f29/ft`       | `v29`  |
             `30` | `x30/t5`    | `f30/ft`       | `v30`  |
             `31` | `x31/t6`    | `f31/ft`       | `v31`  |

When used statically, the notation simply matchers the given name in the table. When used dynamically, the syntax is similar to a function call: `X(reg_number)`, where `reg_number` is one of the given dynamic encodings listed in the table.

Note that not all RISC-V instructions accept all registers. In particular, many instructions in the `C` instruction set extension don't support the `zero` register, or only support registers `x8-x15`. Attempting to use those will result in an error at compile time, or a panic at runtime.

#### Register lists

Several instructions in the `Zcmp` instruction set extension take a list of registers as argument. These register lists conform to a fixed format of the `ra` register, and 0 to 12 registers from the set `s0-s11`. Alternatively, the amount of saved registers can be passed dynamically using the `{ra; expr}` syntax, where `expr` should be an expression that evaluates to the amount of saved registers in the register list. This can be any number from 0 to 10, or 12. It is impossible to encode 11 saved registers. Note that on `RV32E` and `RV64E` only 0 to 2 saved registers can be encoded.

The following instructions are examples of the allowed formats:

- `{ra}`: Only the return address
- `{ra, s0}`: The return address and s0
- `{ra, s0 - s4}`: `ra` and five saved registers
- `{ra, s0 - s11}`: The full set of `ra` and twelve saved registers
- `{ra; 0}`: Only the return address
- `{ra; 1}`: The return address and s0
- `{ra; 5}`: `ra` and five saved registers
- `{ra; 12}`: The full set of `ra` and twelve saved registers

#### Jump targets

All flow control instructions and instructions featuring PC-relative addressing have a jump target as argument. This jump target will feature a label reference as described in the common language reference. Note that this reference must be encoded in a limited amount of bits in the relevant instructions, so check the instruction reference to see what the maximum offset range is.

#### Memory references

As a load-store architecture, the RISC-V instruction sets only has a limited amount of instructions capable of addressing memory. These memory references can have several different format, which are listed in the table below. The valid formats for each instruction can be found in the instruction reference.

Table 3: dynasm-rs RISC-V memory reference formats

Syntax                           | Explanation
:--------------------------------|:-----------
<code>[xn]</code>                | An `X` family register is used as the address to be resolved.
<code>[xn {, imm } ]</code>      | An `X` family register is used as base with an optional integer offset as the address to be resolved.
<code>[sp {, imm } ]</code>      | The `sp` register is used as base with an optional integer offset as the address to be resolved.
<code>[xn {, labelref } ]</code> | The lower 12 bits of a relocation are added to an address in the `X` family register. See the section on pc-relative instructions for further details.

#### Immediates

The RISC-V instruction set features both signed and unsigned immediate operands. The size of these immediates is often not a clean amount of bytes and thus a larger than the maximum value integer type is needed to pass these arguments. Dynasm-rs expects the type of any dynamic RISC-V immediates to be passed to be `u32` for unsigned immediates and `i32` for signed immediates, with the exception of the >32bits `li` pseudo-instructions which use `i64`. These immediates are where possible validated at compile time. If an impossible immediate is provided at runtime, this will result in a panic.

Several instructions have additional requirements on any passed immediates. Consult the instruction reference for the exact requirements of each instruction.

### Compressed instructions

The `C` extension set for RISC-V defines several compressed instructions that implement a subset of functionality of base RISC-V instructions. These compressed extensions are only 2 bytes long, compared to the 4 bytes length of regular RISC-V instructions. As RISC-V assumes a minimum instruction alignment of only 2 bytes, these instructions can be freely intermixed in the instruction stream.

### Pseudo-Instructions

The RISC-V ISA specifies several pseudo-instructions next to its regular instructions. These are either aliases for another instruction with some preconfigured arguments (like `sext.w rd, rs1 = addiw rd, rs1, 0`), or they expand to sequences of several instructions. Alias instructions can be treated just like regular instructions and thus require no special handling, but those that expand to sequences of instructions are of special interest, as dynasm-rs provides guarantees that the length of a sequence of instructions doesn't change depending on the value of arguments, only the chosen instruction format. The following table lists all multi-instruction non-`li` pseudo instructions, as well as what they expand to.

Table 4: RISC-V pseudo-instructions

Instruction         |Architecture| Equivalent dynasm-rs instructions               | Function
:-------------------|:-----------|:------------------------------------------------|:----------------------------------
`la rd, label`      | RV32/64    | `auipc rd, label` <br>`addi rd, rd, label + 4`  | PC-relative load address
`lb rd, label`      | RV32/64    | `auipc rb, label` <br>`lb rd, [rd, label + 4]`  | PC-relative load signed byte
`lbu rd, label`     | RV32/64    | `auipc rbu, label`<br>`lbu rd, [rd, label + 4]` | PC-relative load unsigned byte
`lh rd, label`      | RV32/64    | `auipc rh, label` <br>`lh rd, [rd, label + 4]`  | PC-relative load signed halfword
`lhu rd, label`     | RV32/64    | `auipc rhu, label`<br>`lhu rd, [rd, label + 4]` | PC-relative load unsigned halfword
`lw rd, label`      | RV32/64    | `auipc rw, label` <br>`lw rd, [rd, label + 4]`  | PC-relative load signed word
`lwu rd, label`     | RV64       | `auipc rwu, label`<br>`lwu rd, [rd, label + 4]` | PC-relative load unsigned word
`ld rd, label`      | RV64       | `auipc rd, label` <br>`ld rd, [rd, label + 4]`  | PC-relative load doubleword
`flh rd, label, rt` | RV32/64Zfh | `auipc rt, label` <br>`flh rd, [rt, label + 4]` | PC-relative load half float
`flw rd, label, rt` | RV32/64F   | `auipc rt, label` <br>`flw rd, [rt, label + 4]` | PC-relative load float
`fld rd, label, rt` | RV32/64D   | `auipc rt, label` <br>`fld rd, [rt, label + 4]` | PC-relative load double float
`flq rd, label, rt` | RV32/64Q   | `auipc rt, label` <br>`flq rd, [rt, label + 4]` | PC-relative load quad float
`sb rd, label, rt`  | RV32/64    | `auipc rt, label` <br>`sb rd, [rt, label + 4]`  | PC-relative store byte
`sh rd, label, rt`  | RV32/64    | `auipc rt, label` <br>`sh rd, [rt, label + 4]`  | PC-relative store halfword
`sw rd, label, rt`  | RV32/64    | `auipc rt, label` <br>`sw rd, [rt, label + 4]`  | PC-relative store word
`sd rd, label, rt`  | RV32/64    | `auipc rt, label` <br>`sd rd, [rt, label + 4]`  | PC-relative store doubleword
`fsh rd, label, rt` | RV32/64Zfh | `auipc rt, label` <br>`fsh rd, [rt, label + 4]` | PC-relative store half float
`fsw rd, label, rt` | RV32/64F   | `auipc rt, label` <br>`fsw rd, [rt, label + 4]` | PC-relative store float
`fsd rd, label, rt` | RV32/64D   | `auipc rt, label` <br>`fsd rd, [rt, label + 4]` | PC-relative store double float
`fsq rd, label, rt` | RV32/64Q   | `auipc rt, label` <br>`fsq rd, [rt, label + 4]` | PC-relative store quad float
`sext.b rd, rs`     | RV32       | `slli rd, rs, 24` <br>`srai rd, rd, 24`         | Sign extend byte, when `Zbb` is unavailable
`sext.b rd, rs`     | RV64       | `slli rd, rs, 56` <br>`srai rd, rd, 56`         | Sign extend byte, when `Zbb` is unavailable
`sext.h rd, rs`     | RV32       | `slli rd, rs, 16` <br>`srai rd, rd, 16`         | Sign extend halfword, when `Zbb` is unavailable
`sext.h rd, rs`     | RV64       | `slli rd, rs, 48` <br>`srai rd, rd, 48`         | Sign extend halfword, when `Zbb` is unavailable
`zext.h rd, rs`     | RV32       | `slli rd, rs, 16` <br>`srli rd, rd, 16`         | Zero extend halfword, when `Zbb` is unavailable
`zext.h rd, rs`     | RV64       | `slli rd, rs, 48` <br>`srli rd, rd, 48`         | Zero extend halfword, when `Zbb` is unavailable
`zext.w rd, rs`     | RV64       | `slli rd, rs, 32` <br>`srli rd, rd, 32`         | Zero extend word, when `Zba` is unavailable
`jump offset, rt`   | RV32/64    | `auipc rt, label` <br>`jalr zero, rt, label`    | 32-bit relative jump
`call offset`       | RV32/64    | `auipc ra, label` <br>`jalr ra, ra, label`      | 32-bit relative call
`call rd, offset`   | RV32/64    | `auipc rd, label` <br>`jalr rd, rd, label`      | 32-bit relative call, writing the return address to `rd`
`tail offset`       | RV32/64    | `auipc t1, label` <br>`jalr zero, t1, label`    | 32-bit relative tail call. Uses `t1` as temp, or `t2` when the `Zicfilp` extension is available

Note: `rt` in these instructions is a temporary register to use during address generation. Its value is not important to the instruction.

#### Load immediate

Another important pseudo-instruction is `li` or load immediate. In the GNU assembler, this instruction expands to a variable amount of instructions, designed to load the wanted immediate in an as small amount of instructions as possible. This means that the instruction sequence generated is dependent on the value of the immediate, and thus this approach does not work for dynasm-rs.

Instead, dynasm-rs provides the user with several `li.bitsize` instructions that can load a signed immediate of at most `bitsize` bits into a register. Depending on the target architecture, the following pseudo-instructions are available:

Table 5: Load immediate formats

Instruction     |Architecture| Sequence length | Value range
:---------------|:-----------|:----------------|:----------------------
`li.12 rd, imm` | RV32/64    | 4 bytes         | `-0x800 <= imm <= 0x7FF`
`li rd, imm`    | RV32       | 8 bytes         | `-0x8000_0000 <= imm <= 0x7FFF_FFFF`
`li.32 rd, imm` | RV64       | 8 bytes         | `-0x8000_0000 <= imm <= 0x7FFF_FFFF`
`li.43 rd, imm` | RV64       | 16 bytes        | `-0x400_0000_0000 <= imm <= 0x3FF_FFFF_FFFF`
`li.54 rd, imm` | RV64       | 24 bytes        | `-0x20_0000_0000_0000 <= imm <= 0x1F_FFFF_FFFF_FFFF`
`li rd, imm`    | RV64       | 32 bytes        | `-0x8000_0000_0000_0000 <= imm <= 0x7FFF_FFFF_FFFF_FFFF`

### Upper immediate instructions

The behaviour of the load upper immediate instructions (`lui`, `c.lui`, and `auipc`) in dynasm-rs differs slightly from their behaviour in the GNU assembler. Where the GNU assembler expects the argument to be the result value shifted right 12 bits, dynasm-rs expects the argument to be the expected result value of the instruction. This is both done out of consistency (every other immediate in the instruction set is encoded this way) and to be logical with the way label references are handled. The following table shows the difference:

Table 6: Upper immediate syntax

GNU style           | Dynasm-rs style        | Result
:-------------------|:-----------------------|:----------------------
`lui rd, 0x12345`   | `lui rd, 0x12345000`   | `rd == 0x12345000`
`c.lui rd, 0x12`    | `lui rd, 0x12000`      | `rd == 0x12000`
`auipc rd, 0x12345` | `auipc rd, 0x12345000` | `rd == pc + 0x12345000`

### PC-relative instructions

Due to its use of multi-instruction sequences for many PC-relative operations, RISC-V requires extra attention regarding jumps and pc-relative loads/stores. This section lays out the different classes of instructions, and their rules.

#### Normal branch and jump instructions

The basic jump to label instructions `j`, `jal`, and their compressed variants (`c.j`, `c.jal`), work without issues with dynasm-rs's relocation system. The same applies to all conditional branches (`c.bnez`, `c.beqz`, and all `b[ge|le|eq|gt|lt|ne][uz ]` instructions). Note that many of these have very limited ranges, as shown in the table below:

Table 7: Regular jump and branch range

Instructions       | jump offset size | range
:------------------|:-----------------|:--------------------------
`j`, `jal`         | 20 bits          | `pc-0x8_0000` to `pc+0x7_FFFE`
`beq`, `beqz`,<br>`bne`, `bnez`,<br>`blt`, `bltu`, `bltz`,<br>`bgt`, `bgtu`, `bgtz`,<br>`ble`, `bleu`, `blez`,<br>`bge`, `bgeu`, `bgez` | 12 bits | `pc-0x800` to `pc+0x7FF`
`c.j`, `c.jal`     | 12 bits          | `pc-0x800` to `pc+0x7FF`
`c.beqz`, `c.bnez` | 9  bits          | `pc-0x100` to `pc+0xFF`

#### AUIPC

`auipc rd, imm` is the special instruction that allows for 32-bit PC-relative jumps and address generation in RISC-V. It functions by loading the current program counter, adding an immediate to it, and storing it to the destination register. However, this immediate only contains the upper 20 bits of a signed 32-bit value. The lower 12 bits of this address are then intended to be provided by instructions like `addi` and `addiw`, the offset in `jalr`, or the memory reference offset in load/store instructions.

This does raise a problem in that these offsets are signed. Therefore, one cannot simply mask the higher bits of an offset and pass that to `auipc`, and then pass the lower bits to any of these instructions. The immediate passed to `auipc` must be biased by 0x800 before masking it. To ensure that such a sequence works correctly, dynasm-rs performs the needed adjustment for the user, provided the full immediate (or label) is passed to `auipc`. 

This results in the following behaviour for `auipc`:

- `auipc rb, 0x12345000`: `rb = pc + 0x12345000`
- `auipc rb, 0x123457FF`: `rb = pc + 0x12345000`
- `auipc rb, 0x12345800`: `rb = pc + 0x12346000`
- `auipc rb, 0x12346000`: `rb = pc + 0x12346000`

#### Lower immediate instructions

After use of `auipc rb, offset32` to load the offset program counter value, the following instructions can be used to fill in the lowest bits of the offset.

Table 8: Lower immediate instruction formats for pc-relative operations

Instruction formats                                                            | Function
:------------------------------------------------------------------------------|:-----------------
`addi rb, rb, offset32 & 0xFFF`                                                | load `pc + offset32` into `rb`
`jalr ra, rb, offset32 & 0xFFF`                                                | Jump (possibly with link) to `pc + offset32`
`lb  rb, [rb, offset32 & 0xFFF]`<br>and `lh`/`lw`/`ld`/`lbu`/`lhu`/`lwu`       | loads a value from `[pc + offset32]` into `rb`
`sb  rd, [rb, offset32 & 0xFFF]`<br>and `sh`/`sw`/`sd`                         | stores `rd` to `[pc + offset32]`
`flh rd, [rb, offset32 & 0xFFF]`<br>and `flw`/`fld`/`flq`                      | loads a floating point value from `[pc + offset32]` into `rd`
`slh rd, [rb, offset32 & 0xFFF]`<br>and `slw`/`sld`/`slq`                      | stores a floating point value `rd` to `[pc + offset32]`

These instructions can also be used with dynamic offsets, in which case, dynasm-rs takes care of the masking automatically. It should be noted, that the program counter referenced in the description of these instructions is the address of the `auipc` instruction. In the case of static offsets, this is not a problem. But when dynasm-rs labels are used as the offset, the offset will evaluate to different values in the `auipc` instruction and the subsequent load/store/`addi`/`jalr`. To remedy this, an offset equal to the spacing between these instructions needs to be added to the relocation in the subsequent instruction:

```rust
->our_target_label:
.u32 0xAABBCCDD
<some code>
auipc x8, ->our_target_label
lw x9, [x8, ->our_target_label + 4] // loads 0xAABBCCDD
lw x9, [x8, ->our_target_label + 8] // also loads 0xAABBCCDD
nop
lw x9, [x8, ->our_target_label + 16] // also loads 0xAABBCCDD
```

Using these offsets, it is also possible to load additional values around the label without additional `auipc` instructions, provided the net difference between the address of the `auipc` instruction and the address of the loaded value stays within the same `diff_hi - 0x800` to `diff_hi + 0x7FF` range.

#### Pseudo instructions

As the above combination of `auipc` and another instruction with these extra requirements, RISC-V provides several pseudo-instructions that expand into these sequences. These instructions are listed amongst other pseudo instructions in table 4, but to summarize them:

- `la rd, offset/label` will load an address from the given label or 32-bit pc-relative offset.
- Integer load instructions have an additional format: `lb rd, offset/label`, which will perform a pc-relative load from the given label/32-bit offset 
- Integer store instructions, as well as floating point load/store instructions have an additional format: `lb rd, offset/label, rt`, which will perform a pc-relative load from the given label/32-bit offset, using `rt` as a temporary.
- `call offset/label`, `jump offset/label` and `tail offset/label` perform 32-bit calls/jumps/tail calls to the given label/32-bit offset.

#### Range limitations

Due to the mechanism used for performing 32-bit pc-relative operations on RISC-V (loading an upper immediate and then adding a signed lower immediate), the range of these 32-bit offsets is a bit odd. On RV64, They allow for creating addresses from `pc-0x8000_0800` to `pc+0x7FFF_F7FF`, or 32 bits of signed integer range biased around `-0x800`. This range would mean that provided values could be outside the range of an `i32`, and thus dynasm-rs restricts this further, limiting offsets to being between `-0x8000_0000` and `0x7FFF_F7FF`. This does reduce the available range slightly, but as the range was asymmetric to begin with, this extra range on backwards jumps was never useful.

# Supported extensions

Dynasm-rs currently supports the following ratified RISC-V instruction set extensions:

- `A`: atomic instructions
- `C`: compressed instructions
- `D`: double floating point support
- `F`: floating point support
- `I`: Base instruction set
- `M`: multiplication and division
- `Q`quad floating point support
- `Zabha`: byte and halfword atomics
- `Zacas`: atomic compare and swap
- `Zawrs`: atomic wait-on-reservation-set
- `Zba`: bit manipulation for address generation
- `Zbb`: basic bit-manipulation
- `Zbc`: carry-less multiplication
- `Zbkb`: bit-manipulation for cryptography
- `Zbkc`: carry-less bit-manipulation for cryptography
- `Zbkx`: crossbar permutations
- `Zbs`: single-bit instructions
- `Zcb`: simple code-size savings
- `Zcmop`: compressed may-be-operations
- `Zcmp`: microcoded push/pop operations
- `Zcmt`: table jumps
- `Zdinx`: double floating point integer registers.
- `Zfa`: additional floating point instructions
- `Zfbfmin`: scalar convert to/from BF16
- `Zfh`: half floating point support
- `Zfhmin`: half floating point support: conversion only
- `Zfinx`: floating point in integer registers
- `Zhinx`: half floating point in integer registers
- `Zhinxmin`: half floating point in integer registers: conversion only
- `Zicbom`: cache block operations: management
- `Zicbop`: cache block operations: prefetching
- `Zicboz`: cache block operations: zero
- `Zicfilp`: control flow integrity: landing pads
- `Zicfiss`: control flow integrity: shadow stack
- `Zicntr`: counters
- `Zicond`: integer conditional operations
- `Zicsr`: control and status registers
- `Zifencei`: data to instruction cache fence
- `Zihintntl`: non temporal locality hints
- `Zihintpause`: pause hint
- `Zimop`: may-be-operations
- `Zk`: Scalar cryptography
- `Zkn`: NIST algorithm suite
- `Zknd`: NIST suite: AES decryption
- `Zkne`: NIST suite: AES decryption
- `Zknh`: NIST suite: Hash function instruction
- `Zks`: ShangMi algorithm suite
- `Zksed`: ShangMi suite: SM4 block cipher
- `Zksh`: ShangMi suite: SM3 hash function
