%  Home

In the search for faster interpreters, Just-In-Time compilation is often a useful tool.
This compiler extension attempts to make the writing of such programs easier and faster.

At its core, dynasm-rs is an assembler compiler. It reads assembly templates which it then
compiles into code that when executed will result in the proper machine code being emitted.

Dynasm is split up into two parts. The first is the compiler extension that performs the
translation of assembly templates into rust code, the second part is a
[small runtime](../runtime/dynasmrt/index.html) that handles the generation of the wanted
machine code.

For now dynasm-rs only supports the x64 instruction set.

Dynasm-rs is inspired by the LuaJIT DynASM project for C and C++.

# Documentation

The documentation of dynasm-rs is split up into several parts. To get started, you're advised
to read through the [tutorial](./tutorial.html). After this, you can read through the
[language reference](./langref.html) to learn about the syntax used by dynasm-rs. You can
also read through the [runtime documentation](../runtime/dynasmrt/index.html) to learn about the
runtime API. The [instruction reference](./instructionref.html) lists all assembly mnemnonics
and formats supported by dynasm-rs. Finally, documentation on the
[internals on dynasm-rs](../plugin/dynasm/index.html) can be referenced.
