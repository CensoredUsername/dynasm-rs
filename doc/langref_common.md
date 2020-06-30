% Language Reference

# Lexical structure definition

## Base units

The following syntax units used in dynasm syntax are defined by the [rust grammar](https://doc.rust-lang.org/grammar.html) itself:

- `num_lit`
- `ident`
- `expr_path`
- `expr`
- `stmt`

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
`labelref : (">" ident | "<" ident | "->" ident | "=>" expr | "extern" expr) ;`

## Instructions

The assembly dialect used by dynasm-rs and parsing rules for it differ based on the target architecture (configured using the `.arch` directive).
Check the documentation for the specific architecture.

# Reference

## Directives

Dynasm-rs currently supports the following directives:


Table 1: dynasm-rs directives

Name      | Argument format | Description
----------|-----------------|------------
`.arch`   | A single identifier | Specifies the current architecture to assemble. Defaults to the current target architecture. Only `x64`, `x86` and `aarch64` are supported as of now.
`.feature`| A comma-separated list of identifiers. | Set architectural features that are allowed to be used.
`.alias`  | An name followed by a register | Defines the name as an alias for the wanted register.
`.align`  | An expression of type usize | Pushes NOPs until the assembling head has reached the desired alignment.
`.byte`   | One or more expressions of the type `i8`  | Pushes the values into the assembling buffer.
`.word`   | One or more expressions of the type `i16` | Pushes the values into the assembling buffer.
`.dword`  | One or more expressions of the type `i32` | Pushes the values into the assembling buffer.
`.qword`  | One or more expressions of the type `i64` | Pushes the values into the assembling buffer.
`.bytes`  | An expression of that implements `IntoIterator<Item=u8>` or `IntoIterator<Item=&u8>` | Extends the assembling buffer with the iterator.

Directives are normally local to the current `dynasm!` invocation. However, if the `filelocal` feature is used they will be processed in lexical order over the whole file. This feature only works on a nightly compiler and might be removed in the future.

## Aliases

Dynasm-rs allows the user to define aliases for registers using the `.alias name, register` directive. These aliases can then be used at places where registers are allowed to be used. Note that aliases are defined in lexical parsing order.

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

## Statements

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

Table 2: dynasm-rs label types

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

### Extern labels

Extern labels allow emitted machine code to directly reference fixed addresses as branch targets. This is only supported on architectures featuring absolute branch targets, like `x86`.
