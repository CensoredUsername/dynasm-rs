% Language Reference

# Lexical structure definition

Instructions for the `x86` and `x64` assembling backend use the following lexical structure

## Base units

The following base syntax units are recognized by the parser.

- `prefix : "cs" | "ds" | "es" | "fs" | "gs" | "ss" | "lock" | "rep" | "repne" | "repe" | "repnz" | "repz" ;`
- `static_reg_name` matches any valid register name as seen in table 4, or any previously defined alias
- `dynamic_reg_family` matches any valid register family from table 4
- `vector_reg_name` matches `v0` up to `v31`
- `modifier : "LSL" | "LSR" | "ASR" | "ROR" | "UXTB" | "UXTH" | "UXTW" | "UXTX" | "SXTB" | "SXTH" | "SXTW" | "SXTX" | "MSL" ;`

## Instruction

`instruction : ident ("." ident)* (arg ("," arg)* )? ;`

## Arguments

`arg : register | registerlist | labelref | reference | modifier_expr | immediate ;`

`register : scalar_reg | vector_reg ;`

`scalar_reg : static_reg_name | dynamic_reg_family "(" expr ")"`

`vector_reg : ( vector_reg_name | "V" "(" expr ")" ) "." vector_width_spec element_specifier ? ;`

`register_list : "{ comma_list | dash_list | amount_list "}" element_specifier ? ;`

`comma_list : register ("," register) * ;`

`dash_list : register "-" register ;`

`amount_list : register "*" expr ;`

`element_specifier : "[" expr "]" ;`

`reference : "[" refitem ("," refitem)* "]" !"? ;`

`refitem : register | modifier_expr | immediate ;`

`modifier_expr : modifier immediate? ;`

`immediate : "#"? expr ;`

# Reference

## Instructions

The language used by dynasm-rs in aarch64 mode is close to the assembly dialect described in official ARM documentation. Several additions have been made to support dynamic registers and to ensure the rust parser can handle parsing the language.

The largest difference is in the notation of vector registers. In ARM assembly, the lane count comes before the element size as in `v1.16b`. But in dynasm-rs, this is reversed as bare identifiers cannot start with numbers. So the used notation ends up being `v1.b16`. Next to this, the register section will describe the syntax used for addressing registers.

### Operands

#### Register

There are two ways to reference registers in dynasm-rs, either via their static name, or via dynamic register references. Dynamic register references allow the exact register choice to be made at runtime. Note that this does prevent optimizations to register-specific forms. However, the expression inside a dynamic register reference may be evaluated multiple times.

The following table lists all available static registers, their dynamic family name and their encoding when they are used dynamically.

Table 1: dynasm-rs registers (aarch64)

Family            | 64-bit   | 32-bit   | 64-bit   | 32-bit   | 8-bit    | 16-bit   | 32-bit   | 64-bit   | 128-bit  | vector   |
-----------------:|:---------|:---------|:---------|:---------|:---------|:---------|:---------|:---------|:---------|:---------|
Dynamic Encoding  | `X`      | `W`      | `XSP`    | `WSP`    | `B`      | `H`      | `S`      | `D`      | `Q`      | `V`      |
              `0` | `x0`     | `w0`     | `x0`     | `w0`     | `b0`     | `h0`     | `s0`     | `d0`     | `q0`     | `v0`     |
              `1` | `x1`     | `w1`     | `x1`     | `w1`     | `b1`     | `h1`     | `s1`     | `d1`     | `q1`     | `v1`     |
              `2` | `x2`     | `w2`     | `x2`     | `w2`     | `b2`     | `h2`     | `s2`     | `d2`     | `q2`     | `v2`     |
              `3` | `x3`     | `w3`     | `x3`     | `w3`     | `b3`     | `h3`     | `s3`     | `d3`     | `q3`     | `v3`     |
              `4` | `x4`     | `w4`     | `x4`     | `w4`     | `b4`     | `h4`     | `s4`     | `d4`     | `q4`     | `v4`     |
              `5` | `x5`     | `w5`     | `x5`     | `w5`     | `b5`     | `h5`     | `s5`     | `d5`     | `q5`     | `v5`     |
              `6` | `x6`     | `w6`     | `x6`     | `w6`     | `b6`     | `h6`     | `s6`     | `d6`     | `q6`     | `v6`     |
              `7` | `x7`     | `w7`     | `x7`     | `w7`     | `b7`     | `h7`     | `s7`     | `d7`     | `q7`     | `v7`     |
              `8` | `x8`     | `w8`     | `x8`     | `w8`     | `b8`     | `h8`     | `s8`     | `d8`     | `q8`     | `v8`     |
              `9` | `x9`     | `w9`     | `x9`     | `w9`     | `b9`     | `h9`     | `s9`     | `d9`     | `q9`     | `v9`     |
             `10` | `x10`    | `w10`    | `x10`    | `w10`    | `b10`    | `h10`    | `s10`    | `d10`    | `q10`    | `v10`    |
             `11` | `x11`    | `w11`    | `x11`    | `w11`    | `b11`    | `h11`    | `s11`    | `d11`    | `q11`    | `v11`    |
             `12` | `x12`    | `w12`    | `x12`    | `w12`    | `b12`    | `h12`    | `s12`    | `d12`    | `q12`    | `v12`    |
             `13` | `x13`    | `w13`    | `x13`    | `w13`    | `b13`    | `h13`    | `s13`    | `d13`    | `q13`    | `v13`    |
             `14` | `x14`    | `w14`    | `x14`    | `w14`    | `b14`    | `h14`    | `s14`    | `d14`    | `q14`    | `v14`    |
             `15` | `x15`    | `w15`    | `x15`    | `w15`    | `b15`    | `h15`    | `s15`    | `d15`    | `q15`    | `v15`    |
             `16` | `x16`    | `w16`    | `x16`    | `w16`    | `b16`    | `h16`    | `s16`    | `d16`    | `q16`    | `v16`    |
             `17` | `x17`    | `w17`    | `x17`    | `w17`    | `b17`    | `h17`    | `s17`    | `d17`    | `q17`    | `v17`    |
             `18` | `x18`    | `w18`    | `x18`    | `w18`    | `b18`    | `h18`    | `s18`    | `d18`    | `q18`    | `v18`    |
             `19` | `x19`    | `w19`    | `x19`    | `w19`    | `b19`    | `h19`    | `s19`    | `d19`    | `q19`    | `v19`    |
             `20` | `x20`    | `w20`    | `x20`    | `w20`    | `b20`    | `h20`    | `s20`    | `d20`    | `q20`    | `v20`    |
             `21` | `x21`    | `w21`    | `x21`    | `w21`    | `b21`    | `h21`    | `s21`    | `d21`    | `q21`    | `v21`    |
             `22` | `x22`    | `w22`    | `x22`    | `w22`    | `b22`    | `h22`    | `s22`    | `d22`    | `q22`    | `v22`    |
             `23` | `x23`    | `w23`    | `x23`    | `w23`    | `b23`    | `h23`    | `s23`    | `d23`    | `q23`    | `v23`    |
             `24` | `x24`    | `w24`    | `x24`    | `w24`    | `b24`    | `h24`    | `s24`    | `d24`    | `q24`    | `v24`    |
             `25` | `x25`    | `w25`    | `x25`    | `w25`    | `b25`    | `h25`    | `s25`    | `d25`    | `q25`    | `v25`    |
             `26` | `x26`    | `w26`    | `x26`    | `w26`    | `b26`    | `h26`    | `s26`    | `d26`    | `q26`    | `v26`    |
             `27` | `x27`    | `w27`    | `x27`    | `w27`    | `b27`    | `h27`    | `s27`    | `d27`    | `q27`    | `v27`    |
             `28` | `x28`    | `w28`    | `x28`    | `w28`    | `b28`    | `h28`    | `s28`    | `d28`    | `q28`    | `v28`    |
             `29` | `x29`    | `w29`    | `x29`    | `w29`    | `b29`    | `h29`    | `s29`    | `d29`    | `q29`    | `v29`    |
             `30` | `x30`    | `w30`    | `x30`    | `w30`    | `b30`    | `h30`    | `s30`    | `d30`    | `q30`    | `v30`    |
             `31` | `xzr`    | `wzr`    | `sp`     | `wsp`    | `b31`    | `h31`    | `s31`    | `d31`    | `q31`    | `v31`    |

When used statically, the notation simply matchers the given name in the table. When used dynamically, the syntax is similar to a function call: `X(reg_number)`, where reg_number is one of the given dynamic encodings listed in the table.

As aarch64 either uses scalar register 31 as the zero register `xzr` or the stack pointer register `sp`, two separate families of registers exist to encode this possible difference (as it can influence instruction variant choice).

A special case are the vector registers. These are never used as bare registers, but need to have the element size they are being accessed with postfixed to the register. This element size can be:

- `B`: 1 byte
- `H`: 2 bytes
- `S`: 4 bytes
- `D`: 8 bytes
- `Q`: 16 bytes

This means that the base for for adressing a register statically looks like `V1.B` or `V(num).B`. Additionally, many instructions also require the lane count to be specified for the vector register access.
As discussed before, this is appended after the element size specifier like `V1.B8` or `V(num).B16`. Finally, vector registers can support a lane element specifier if an instruction aims to only use a certain lane of a vector register. In this case the lane count is always optional. This lane is defined by an index expression postfixed to the vector register like `V1.B[1]` or `V(num).B[lane]`.

#### Register lists

Several vector instructions in aarch64 address a list of registers as single operands. There are several syntaxes supported by dynasm-rs for register lists:

Table 2: dynasm-rs register list types

Type          | Example
-------------:|:---------
Comma list    | `{ Vn.B, Vn+1.B, Vn+2.B, Vn+3.B }`
Dash list     | `{ Vn.B - Vn+3.B }`
Amount list   | `{ Vn.B * 4 }`

Each of these list notations is interpreted exactly the same by dynasm-rs. The first two are also standard ARM notation, the third format is added by dynasm-rs to handle dynamic registers in register lists as otherwise the amount could only be calculated at runtime. Just like vector registers, register lists support an optional element specifier after them: `{ Vn.B * 4 }[1]`.

#### Jump targets

All flow control instructions and instructions featuring PC-relative addressing have a jump target as argument. This jump target will feature a label reference as described in the common language reference. Note that this reference must be encoded in a limited amount of bits due to the fixed-width aarch64 instruction set, so check the instruction reference to see what the maximum offset range is.

#### Memory references

As a load-store architecture, the aarch64 instruction set only has a limited amount of instructions capable of addressing memory. Further more, it supports a limited set of addressing modes. The available addressing modes for each instruction are listed directly in the instruction reference. All possible addressing modes are summarized in the table below as well.

Table 3: dynasm-rs memory reference formats

Syntax   | Explanation
:--------|:-----------
<code>[Xn&#124;SP]</code> | A `WSP` family register is used as the address to be resolved.
<code>[Xn&#124;SP {, #imm } ]</code> | A `WSP` family register is used as base with an optional integer offset as the address to be resolved.
<code>[Xn&#124;SP, #imm ]!</code> | A `WSP` family register is used as base with an integer offset as the address to be resolved. The final address is written back to the base register.
<code>[Xn&#124;SP], #imm</code> | A `WSP` family register is used as the base address to be resolved. Then the immediate is added to the base register and written back.
<code>[Xn&#124;SP, Wm&#124;Xm {, MOD { #imm } } ]</code> | A `WSP` family register is used as base with an (optionally shifted) index register to compute the final address to be resolved.
<code>[Xn&#124;SP], Xm </code> | A `WSP` family register is used as the base address to be resolved. Then the second register is added to the base register and written back.

#### Modifiers

Several instructions in aarch64, as well as the indexed register addressing mode, support a so-called modifier that change the way the core interprets another argument. The instruction reference shows the supported modifiers for each instruction, and the following table lists all of them:

Table 4: aarch64 modifiers

Modifier | immediate required | description
--------:|:-------------------|:-----------
LSL      | yes                | Logical shift left
LSR      | yes                | Logical shift right
ASR      | yes                | Arithmetic shift right
ROR      | yes                | Rotate right
UXTB     | no                 | Unsigned extend byte
UXTH     | no                 | Unsigned extend halfword
UXTW     | no                 | Unsigned extend word
UXTX     | no                 | Unsigned extend doubleword
SXTB     | no                 | Signed extend byte
SXTH     | no                 | Signed extend halfword
SXTW     | no                 | Signed extend word
SXTX     | no                 | Signed extend doubleword
MSL      | yes                | Shift left, inserting ones

Modifiers can also take an immediate as argument. For shifting modifiers the immediate is required, for extending modifiers it is optional and acts as an extra shift left if provided.

#### Immediates

Dynasm-rs supports both ARM immediate notation `#1` and bare immediate notation `1`. As a fixed width instruction set, immediates are bitfields in the respective instructions and will have a limited range.
This range can be found for any immediate in the instruction reference. Additionally. Several special immediate classes are distinguished in the aarch64 instruction set. The following table lists all of these.

Table 5: aarch64 special immediate types

Immediate type | description
:--------------|:------------
Wide immediate | A 32 or 64-bit immediate which is encoded by taking 16 bits and shifting them 0, 16, 32, or 48 bits left, with possible inversion afterwards.
Logical immediate | A 32 or 64-bit bitfield, composed out of repeated 2, 4, 8, 16, 32 or 64-bit elements with the first n bits set to 1, and then rotated afterwards. All 0 or all 1 cannot be encoded. 
Stretched immediate | A 64-bit immediate encoded in 8 bits `a:b:c:d:e:f:g:h` which encodes the binary value `0baaaaaaaabbbbbbbbccccccccddddddddeeeeeeeeffffffffgggggggghhhhhhhh`.
Floating point immediate | A short, float or double value encoded into 8 bits. It can represent any value in the format `(-1.0)^s * 2.0^e * (1.0 + m / 16.0)` where `-3 <= e <= 4, 0 <= m <= 15, s = [0, 1]`.
