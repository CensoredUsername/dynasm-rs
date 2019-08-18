% Language Reference

# Lexical structure definition

Instructions for the `x86` and `x64` assembling backend use the following lexical structure

## Base units

The following base syntax units are recognized by the parser.

- `prefix : "cs" | "ds" | "es" | "fs" | "gs" | "ss" | "lock" | "rep" | "repne" | "repe" | "repnz" | "repz" ;`
- `static_reg` matches any valid register name as seen in table 4, or any previously defined alias
- `dynamic_reg_family` matches any valid register family from table 4
- `size : "BYTE" | "WORD" | "DWORD" | "AWORD" | "QWORD" | "OWORD" | "HWORD"`
- `nosplit : "NOSPLIT"`

## Instruction

`instruction : prefix* ident (arg ("," arg)* )? ;`

## Arguments

`arg : register | (size? ( memoryref | labelref | typemap | expr ));`

`typemap : register "=>" expr_path ("." ident | "[" size? regref "]" ("." ident)?) ;`

`memoryref : "[" nosplit? size? (regref | labelref) "]" ;`

`regref : regrefitem ("+" regrefitem)* ;`

`regrefitem : (register "*" num_lit | num_lit "*" register | register | expr) ;`

`register = static_reg | dynamic_reg ;`

`dynamic_reg = dynamic_reg_family "(" expr ")" ;`

# Reference

## Instructions

The language used by dynasm-rs is a nasm-dialect. The largest difference is that instead of prefixing memory operands with segment registers, segment register overrides are prefixed to the entire instruction. Furthermore, it is currently not possible to override the size of the displacement used in memory operands.

This results in the following syntax for instructions. First, zero or more prefixes can be listed (these prefixes can be found in the base units section). The instruction mnemnonic is then mentioned, followed by zero or more comma separated operands.

### Operands

#### Register

There are two ways to reference registers in dynasm-rs, either via their static name, or via dynamic register references. Dynamic register references allow the exact register choice to be postponed to the runtime. Note that this does prevent optimizations to register-specific forms. However, the expression inside a dynamic register reference may be evaluated multiple times by dynasm-rs.

The following table lists all available static registers, their dynamic family name and their encoding when they are used dynamically.

Table 1: dynasm-rs registers (x64/x86)

Family              | 8-bit       | 8-bit high | 16-bit     | 32-bit      | 64-bit (x64 only) | RIP       | Floating Point | MMX    | 128-bit   | 256-bit   | Segment | Control | Debug | Bound
-------------------:|:------------|:-----------|:-----------|:------------|:------------------|:----------|:---------------|:-------|:----------|:----------|:--------|:--------|:------|:-----
Dynamic Encoding    | `Rb`        | `Rh`       | `Rw`       | `Rd`        | `Rq`              |           | `Rf`           | `Rm`   | `Rx`      | `Ry`      | `Rs`    | `RC`    | `RD`  | `RB`
                `0` | `al`/`r0b`  |            | `ax`/`r0w` | `eax`/`r0d` | `rax`/`r0`        |           | `st0`          | `mmx0` | `xmm0`    | `ymm0`    | `es`    | `cr0`   | `dr0` | `bnd0`
                `1` | `cl`/`r1b`  |            | `cx`/`r1w` | `ecx`/`r1d` | `rcx`/`r1`        |           | `st1`          | `mmx1` | `xmm1`    | `ymm1`    | `cs`    | `cr1`   | `dr1` | `bnd1`
                `2` | `dl`/`r2b`  |            | `dx`/`r2w` | `edx`/`r2d` | `rdx`/`r2`        |           | `st2`          | `mmx2` | `xmm2`    | `ymm2`    | `ss`    | `cr2`   | `dr2` | `bnd2`
                `3` | `bl`/`r3b`  |            | `bx`/`r3w` | `ebx`/`r3d` | `rbx`/`r3`        |           | `st3`          | `mmx3` | `xmm3`    | `ymm3`    | `ds`    | `cr3`   | `dr3` | `bnd3`
                `4` | `spl`/`r4b` | `ah`       | `sp`/`r4w` | `esp`/`r4d` | `rsp`/`r4`        |           | `st4`          | `mmx4` | `xmm4`    | `ymm4`    | `fs`    | `cr4`   | `dr4` |
                `5` | `bpl`/`r5b` | `ch`       | `bp`/`r5w` | `ebp`/`r5d` | `rbp`/`r5`        | `eip/rip` | `st5`          | `mmx5` | `xmm5`    | `ymm5`    | `gs`    | `cr5`   | `dr5` |
                `6` | `sil`/`r6b` | `dh`       | `si`/`r6w` | `esi`/`r6d` | `rsi`/`r6`        |           | `st6`          | `mmx6` | `xmm6`    | `ymm6`    |         | `cr6`   | `dr6` |
                `7` | `dil`/`r7b` | `bh`       | `di`/`r7w` | `edi`/`r7d` | `rdi`/`r7`        |           | `st7`          | `mmx7` | `xmm7`    | `ymm7`    |         | `cr7`   | `dr7` |
    (x64 only)  `8` | `r8b`       |            | `r8w`      | `r8d`       | `r8`              |           |                |        | `xmm8`    | `ymm8`    |         | `cr8`   | `dr8` |
    (x64 only)  `9` | `r9b`       |            | `r9w`      | `r9d`       | `r9`              |           |                |        | `xmm9`    | `ymm9`    |         | `cr9`   | `dr9` |
    (x64 only) `10` | `r10b`      |            | `r10w`     | `r10d`      | `r10`             |           |                |        | `xmm10`   | `ymm10`   |         | `cr10`  | `dr10`|
    (x64 only) `11` | `r11b`      |            | `r11w`     | `r11d`      | `r11`             |           |                |        | `xmm11`   | `ymm11`   |         | `cr11`  | `dr11`|
    (x64 only) `12` | `r12b`      |            | `r12w`     | `r12d`      | `r12`             |           |                |        | `xmm12`   | `ymm12`   |         | `cr12`  | `dr12`|
    (x64 only) `13` | `r13b`      |            | `r13w`     | `r13d`      | `r13`             |           |                |        | `xmm13`   | `ymm13`   |         | `cr13`  | `dr13`|
    (x64 only) `14` | `r14b`      |            | `r14w`     | `r14d`      | `r14`             |           |                |        | `xmm14`   | `ymm14`   |         | `cr14`  | `dr14`|
    (x64 only) `15` | `r15b`      |            | `r15w`     | `r15d`      | `r15`             |           |                |        | `xmm15`   | `ymm15`   |         | `cr15`  | `dr15`|

#### Jump targets

All flow control instructions have a jump target as argument. A jump target can be either an immediate specifying a relative offset to the end of the current instruction or a label reference. For many instructions, the size of the offset to be encoded is variable, and by default dynasm-rs will pick the largest size possible. This can be overridden using a size prefix on the operand.

#### Memory references

Many x64 instructions can taken an indirect memory reference as operand. Such an operand is denoted as an expression containing registers surrounded by square brackets. Note that, unlike the original dynasm, dynasm-rs is insensitive to the order of the different operands in the expression and can perform rudimentary arithmetic to encode forms like `[rax * 5]`. However, due to the limitations of x64 assembly, there are of course limitations on what can be encoded. These limitations are detected at compile time, but several of them cannot be checked when dynamic registers are used. The size of the dereferenced value can be determined by a size prefix.

To give more control to how the operand gets encoded, dynasm-rs features both displacement size overrides and a hinting mechanism similar to NASM. By default dynasm will try to infer the wanted displacement size if the displacement is a constant, and if it fails to it will encode a four-byte displacement. However, this behaviour can be altered using a size override after the opening bracket of the memory reference. 

The hinting mechanism by default tries to select the smallest way for a memory reference to be encoded, and any freedom in this is solved with the following rules:
- The first unscaled register that only appears once can be encoded as base will be used as such.
- If no unscaled register is present, the first register with a total scale of 1 will be used as base.

One complication in the "smallest encoding" rule is `[rax * 2]`. As memory references without base require a four-byte displacement, it is shorter to encode this as `[rax + rax * 1]`. This kind of index splitting is the default behaviour for dynasm-rs, and can be disabled by using the `NOSPLIT` keyword in the memory reference. This keyword must come before the displacement size specifier if both are used as in `[NOSPLIT BYTE rax * 2 + 1]`.

As a final node, the `mib` addressing mode used by Intel's MPX extensions deserves some attention. Dynasm-rs does not implement special syntax for this addressing mode. Instead, the index and base registers in this addressing mode can simply be specified by the hinting behaviour described above.


The following are several examples of what can be encoded:

Table 2: dynasm-rs memory reference formats

Syntax   | Explanation
:--------|:-----------
`[expr]` | An arbitrary expression will be encoded as an immediate
`[rax]`  | A register can be dereferenced. This can either be a 32-bit or a 64-bit register.
`[rax * 4]` | A scaled register can be dereferenced. Possible scales are 8, 4, 2 and 1, although 3, 5 and 9 can also be encoded when it is the only used register.
`[BYTE rax + 77]` | The size of the displacement encoded can b e defined using a size override.
`[rax * 1 + rbx]` | Which register is encoded as index can be explicitly controlled by multiplying with 1.
`[NOSPLIT rax * 2]` | The nosplit keyword forces this to be encoded sub-optimally without a base register.
`[rax * 4 + rbx + expr]` | The previously mentioned forms can all be combined. Order is not important.
`[xmm * 4 + rbx + expr]` | When VSIB addressing is allowed, an xmm or ymm register can be used as index.
`[rip + expr]` | Addresses relative to the instruction pointer at the end of the instruction can also be used, but in this case no scale can be encoded.
`[->label]` | Label references can also be dereferenced. This goes for all label types.

#### Type mapped references

To ease interoperation with rust structures, dynasm-rs supports the following syntax for accessing members of pointers to structs and struct arrays. In this syntax, the scale and displacement in a normal memory reference are derived from the size of the type and the offset of the member in the type. Due to the limitations of procedural macros, invalid scales will unfortunately only panic at runtime. Note that dynasm-rs is unable to infer the size of the attribute and it should therefore be determined by a size prefix.

Just like memory references, type mapped references support displacement size overrides after the opening square bracket. However, as the first register is always encoded as index, they do not support `NOSPLIT`.

The syntax for type maps is as follows:

Table 3: dynasm-rs type map formats

Syntax | Equivalent expression
:------|:-----------
`rax => Type.attr`             | `(rax as *mut Type).attr`
`rax => Type[expr]`            | `(rax as *mut [Type])[expr]`
`rax => Type[rbx]`             | `(rax as *mut [Type])[rbx]`
`rax => Type[rbx + expr].attr` | `(rax as *mut [Type])[rbx + expr].attr `

#### Immediates

Any operand which does not match the previously discussed forms will be interpreted as an immediate argument. This operand will be evaluated as an expression at runtime and the resulting value will be encoded. The size of the encoded value can be determined by a size prefix. If such a a prefix is not given, dynasm-rs will try to infer it from the value of the immediate, but this is only possible if the immediate is a simple constant. As this might change in the future, you should use explicit size overrides if the encoded displacement size matters.
