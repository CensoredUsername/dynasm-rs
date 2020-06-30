%  Dynasm-rs

In the search for faster interpreters, Just-In-Time compilation is often a useful tool.
This compiler extension attempts to make the writing of such programs easier and faster.

At its core, dynasm-rs is an assembler compiler. It reads assembly templates which it then
compiles into code that when executed will result in the proper machine code being emitted.

Dynasm is split up into two parts. The first is the compiler extension that performs the
translation of assembly templates into rust code, the second part is a
[small runtime](../runtime/dynasmrt/index.html) that handles the generation of the wanted
machine code.

Dynasm-rs supports the x86, x64 and aarch64 instruction set architectures.

Dynasm-rs is inspired by the LuaJIT DynASM project for C and C++.

# Documentation

The documentation of dynasm-rs is split up into several parts. To get started, you're advised
to read through the [tutorial](./tutorial.html). After this, you can read through the
[language reference](./langref_common.html) to learn about the syntax used by dynasm-rs. You can
also read through the [runtime documentation](../runtime/dynasmrt/index.html) to learn about the
runtime API. The instruction references lists all assembly mnemnonics
and formats supported by dynasm-rs. Finally, documentation on the
[internals on dynasm-rs](../plugin/dynasm/index.html) can be browsed here.

# Differences from LuaJit Dynasm

The following list summarizes some of the larger differences between LuaJIT dynasm and dynasm-rs.

## general

- LuaJIT dynasm uses full program analysis, allowing it to compile local and global labels down to
enums. Dynasm-rs however uses HashMaps keyed by static strings, meaning label resolution in dynasm-rs
can be a bit slower.
- LuaJIT local labels are integer literals. Dynasm-rs local labels are identifiers.
- Dynasm-rs does not (directly) support stand-alone files.
- LuaJIT dynasm uses a special preprocessor which detects lines starting with pipes (`|`) as dynasm
instructions, dynasm-rs uses the `dynasm!` procedural macro with lines starting with semicolons (`;`).
- LuaJIT has macros in its invocations, dynasm-rs uses rust macros that expand to `dynasm!` invocations.
- Dynasm-rs doesn't have typed aliases

## x64/x86

- LuaJIT uses the `mov64` mnemnonic to encode 64-bit displacement mov. Dynasm-rs uses the `movabs`
mnemnonic with a 64-bit immediate parameter to encode this.
- Dynasm-rs is not sensitive to the order of parameters inside a memory reference.
- The syntax used for type maps is significantly different. In LuaJit dynasm it is `Type:reg->attr`
in dynasm-rs it is `reg => Type.attr`.

## aarch64

- Unknown.