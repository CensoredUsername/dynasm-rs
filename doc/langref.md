% Language Reference

# Lexical structure definition

## Base units

The following syntax units used in dynasm syntax are defined by the [rust grammar](https://doc.rust-lang.org/grammar.html) itself:

- `num_lit`
- `ident`
- `expr_path`
- `expr`
- `stmt`


Dynasm-rs defines the following base syntax units:

- `prefix : "cs" | "ds" | "es" | "fs" | "gs" | "ss" | "lock" | "rep" | "repne" | "repe" | "repnz" | "repz" ;`
- `static_reg` matches any valid register name as seen in table 4, or any previously defined alias
- `dynamic_reg_family` matches any valid register family from table 4
- `size : "BYTE" | "WORD" | "DWORD" | "AWORD" | "QWORD" | "OWORD" | "HWORD"`
- `nosplit : "NOSPLIT"`

## Entry point

The entry point of dynasm-rs is the dynasm! macro. It is structured as following

`dynasm : "dynasm" "!" "(" ident (";" line)* ")" ;`

Where line can be one of the following:

`line : (";" stmt) | directive | label | instruction ;`

## Directives

Directives are special commands given to the assembler that do not correspond to instructions directly.
They are executed at parse time, and each directive can have different parsing rules.

`directive : "." ident directive_parsing_rule;`

## Labels

`label : ident ":" | "->" ident ":" | "=>" expr ;`

## Instructions

`instruction : prefix* ident (arg ("," arg)* )? ;`

## Arguments

`arg : register | (size? ( memoryref | labelref | typemap | expr ));`

`typemap : register "=>" expr_path ("." ident | "[" size? regref "]" ("." ident)?) ;`

`memoryref : "[" nosplit? size? (regref | labelref) "]" ;`

`regref : regrefitem ("+" regrefitem)* ;`

`regrefitem : (register "*" num_lit | num_lit "*" register | register | expr) ;`

`labelref : (">" ident | "<" ident | "->" ident | "=>" expr | "extern" expr) ;`

`register = static_reg | dynamic_reg ;`

`dynamic_reg = dynamic_reg_family "(" expr ")" ;`

# Reference

## Directives

Dynasm-rs currently supports the following directives:


Table 1: dynasm-rs directives

Name      | Argument format | Description
----------|-----------------|------------
`.arch`   | A single identifier | Specifies the current architecture to assemble. Defaults to the current target architecture. Only `x64` and `x86` are supported as of now.
`.feature`| A comma-separated list of identifiers. | Set architectural features that are allowed to be used.
`.alias`  | An name followed by a register | Defines the name as an alias for the wanted register.
`.align`  | An expression of type usize | Pushes NOPs until the assembling head has reached the desired alignment.
`.byte`   | One or more expressions of the type `i8`  | Pushes the values into the assembling buffer.
`.word`   | One or more expressions of the type `i16` | Pushes the values into the assembling buffer.
`.dword`  | One or more expressions of the type `i32` | Pushes the values into the assembling buffer.
`.qword`  | One or more expressions of the type `i64` | Pushes the values into the assembling buffer.
`.bytes`  | An expression of that implements `IntoIterator<Item=u8>` or `IntoIterator<Item=&u8>` | Extends the assembling buffer with the iterator.

## Aliases

Dynasm-rs allows the user to define aliases for registers using the `.alias name, register` directive. These aliases can then be used at places where registers are allowed to be used. Note that aliases are defined in lexical parsing order and that their scoping is crate-global.

## Macros

While this is technically not a feature of dynasm-rs, there are a few rules that must be taken into account when using normal rust macros with dynasm-rs.

First of all, it is not possible to have `dynasm!` parse the result of a rust macro. This is a limitation of rust itself. The proper way to use rust macros with dynasm-rs is to have macros expand to a `dynasm!` call as can be seen in the following example:

```
macro_rules! fma {
    ($ops:ident, $accumulator:expr, $arg1:expr, $arg2:expr) => {dynasm!($ops
        ; imul $arg1, $arg2
        ; add $accumulator, $arg1
    )};
}
```

An important thing to notice here is which matchers are used for which parts of `dynasm!` syntax. The following table lists the correct matchers to be used for expanding to dynasm syntax elements. Note that `$a:expr` means that anything that parses to an expression like `$a:ident` and just raw token trees are allowed.

Table 2: dynasm-rs macro expansion rules

 Syntax element        | Matchers
:----------------------|:------------------------
Assembling buffer      | `$ops:expr`
Register reference     | `$reg:expr`
Memory reference       | `[ $mem:expr ]`
Any element inside a memory reference | `$elem:expr, $reg:ident`
Immediate              | `$imm:expr`
Local or global label name | `$label:ident`
Dynamic label          | `$label:expr`
Type map               | `$reg:expr => $type:path [ $mem:expr ] . $attr:ident`

## statements

To make code that uses a lot of macros less verbose, dynasm-rs allows bare rust statements to be inserted inside `dynasm!` invocations. This can be done by using a double semicolon instead of a single semicolon at the start of the line as displayed in the following equivalent examples:

```
dynasm!(ops
    ; mov rcx, rax
);
call_extern!(ops, extern_func);
dynasm!(ops
    ; mov rcx, rax
);

dynasm!(ops
    ; mov rcx, rax
    ;; call_extern!(ops, extern_func)
    ; mov rcx, rax
);
```

## Labels

In order to describe flow control effectively, dynasm-rs supports labels. However, since the assembly templates can be combined in a variety of ways at the mercy of the program using dynasm-rs, the semantics of these labels are somewhat different from how labels work in a static assembler.

Dynasm-rs distinguishes between three different types of labels: global, local and dynamic labels. Their syntax is as follows:

Table 3: dynasm-rs label types

Type    | Definition   | Reference
--------|--------------|-----------
Local   | `label:`     | `>label` or `<label`
GLobal  | `->label:`   | `->label`
Dynamic | `=>expr`     | `=>expr`
Extern  | `-`          | `extern expr`

### Local labels

On first sight, local label definitions are similar to how labels are normally used in static assemblers. The trick with local labels is however in how they can be referenced. Local labels referenced with the `>label` syntax will be resolved to the first definition of this label after this piece of code, while local labels referenced with the `<label` will be resolved to the last definition of this label before the reference site. Any valid rust identifier can be used as a local label name, and local labels can be defined multiple times.

### Global labels

Global labels can only be defined once, and all references to a global label will be resolved to this label. Any valid rust identifier can be used as a local label name.

### Dynamic labels

Dynamic labels are similar to global labels in that they can be defined only once, but instead of a name, they are identified by an expression. New dynamic labels can be created at runtime by the assembler. This expression is evaluated at the point where the label is defined or referenced, and the labels will be resolved at only at commit time.

## Instructions

The language used by dynasm-rs is a nasm-dialect. The largest difference is that instead of prefixing memory operands with segment registers, segment register overrides are prefixed to the entire instruction. Furthermore, it is currently not possible to override the size of the displacement used in memory operands.

This results in the following syntax for instructions. First, zero or more prefixes can be listed (these prefixes can be found in the base units section). The instruction mnemnonic is then mentioned, followed by zero or more comma separated operands.

### Operands

#### Register

There are two ways to reference registers in dynasm-rs, either via their static name, or via dynamic register references. Dynamic register references allow the exact register choice to be postponed to the runtime. Note that this does prevent optimizations to register-specific forms. However, the expression inside a dynamic register reference may be evaluated multiple times by dynasm-rs.

The following table lists all available static registers, their dynamic family name and their encoding when they are used dynamically.

Table 4: dynasm-rs registers (x64/x86)

Family              | 8-bit       | 8-bit high | 16-bit     | 32-bit      | 64-bit (x64 only) | RIP       | Floating Point | MMX    | 128-bit   | 256-bit   | Segment | Control | Debug | Bound
-------------------:|:------------|:-----------|:-----------|:------------|:------------------|:----------|:---------------|:-------|:----------|:----------|:--------|:--------|:------|:-----
Dynamic Encoding    | `Rb`        | `Rh`       | `Rw`       | `Rd`        | `Rq`              |           | `Rf`           | `Rm`   | `Rx`      | `Ry`      | `Rs`    | `RC`    | `RD`  | `RB`
                `0` | `al`/`r0b`  |            | `ax`/`r0w` | `eax`/`r0d` | `rax`/`r0`        |           | `st0`          | `mmx0` | `xmm0`    | `ymm0`    | `es`    | `cr0`   | `dr0` | `bnd0`
                `1` | `cl`/`r1b`  |            | `cx`/`r1w` | `ecx`/`r1d` | `rcx`/`r1`        |           | `st1`          | `mmx1` | `xmm1`    | `ymm1`    | `cs`    | `cr1`   | `dr1` | `bnd1`
                `2` | `dl`/`r2b`  |            | `dx`/`r2w` | `edx`/`r2d` | `rdx`/`r2`        |           | `st2`          | `mmx2` | `xmm2`    | `ymm2`    | `ss`    | `cr2`   | `dr2` | `bnd2`
                `3` | `bl`/`r3b`  |            | `bx`/`r3w` | `ebx`/`r3d` | `rbx`/`r3`        |           | `st3`          | `mmx3` | `xmm3`    | `ymm3`    | `ds`    | `cr3`   | `dr3` | `bnd3`
                `4` | `spl`/`r4b` | `ah`       | `sx`/`r4w` | `esx`/`r4d` | `rsx`/`r4`        |           | `st4`          | `mmx4` | `xmm4`    | `ymm4`    | `fs`    | `cr4`   | `dr4` |
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

Table 5: dynasm-rs memory reference formats

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

Table 6: dynasm-rs type map formats

Syntax | Equivalent expression
:------|:-----------
`rax => Type.attr`             | `(rax as *mut Type).attr`
`rax => Type[expr]`            | `(rax as *mut [Type])[expr]`
`rax => Type[rbx]`             | `(rax as *mut [Type])[rbx]`
`rax => Type[rbx + expr].attr` | `(rax as *mut [Type])[rbx + expr].attr `

#### Immediates

Any operand which does not match the previously discussed forms will be interpreted as an immediate argument. This operand will be evaluated as an expression at runtime and the resulting value will be encoded. The size of the encoded value can be determined by a size prefix. If such a a prefix is not given, dynasm-rs will try to infer it from the value of the immediate, but this is only possible if the immediate is a simple constant. As this might change in the future, you should use explicit size overrides if the encoded displacement size matters.
