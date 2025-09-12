# Release notes

The `dynasm-rs` project consists out of two crates: The procedural macro crate `dynasm` and the runtime support crate `dynasmrt`. The versions of these two crates are synchronized and should always match. From version 0.7.0 onwards `dynasmrt` depends on `dynasm` itself to simplify this relationship. Any version listings below therefore refers to both the `dynasm` and `dynasmrt` crate version.

Version 4.0.1
=============

Bugfix
------
This bugfix release fixes a failure to compile the `dynasmrt` crate on non-unix RISC-V.


Version 4.0.0
=============

Summary
-------
This release adds ergonomics improvements to dynamic registers. Where it was previously required to explicitly convert the type for dynamic registers as follows:
```rust
let dyn_reg = dynasmrt::x64::Rq::RAX;

dynasm!(ops
    ; add Rq(dyn_reg.into()), 5
);
```
This is no longer necessary. Instead, the conversion is now performed implicitly via `Into<u8>`.
```rust
let dyn_reg = dynasmrt::x64::Rq::RAX;

dynasm!(ops
    ; add Rq(dyn_reg), 5
);
```

General
-------
- dependencies have been updated

Plugin
------
- Dynamic registers now accept any value that is convertible via `Into<u8>`. This significantly improves the ergonomics of using dynamic register values with custom wrapper types. Unfortunately, this might break code which already does this conversion explicitly, leading to an unknown type error. To remedy this, simply remove the explicit conversion.

Runtime
-------
- Assembler has gained a `new_with_capacity` method.

Version 3.2.1
=============

Runtime
-------
- Cache invalidation on aarch64 Apple hardware now does not crash with an illegal instruction exception.

Version 3.2.0
=============

Architecture support
--------------------
- RISC-V targets got support for the `Zfinx`, `Zhinx`, `Zdinx`, `Zhinxmin` and `Zfhmin` instruction
  set extensions.
- RISC-V targets now automatically expand the `B` collection of ISA EXTENSIONS
- RISC-V targets now ignore the `Ztso` extension instead of it causing an error
  (as it doesn't actually add any new instructions).

Version 3.1.0
=============

Summary
-------

This release adds support for several architectures from the RISC-V family of instruction sets.
These are `riscv64i`, `riscv64e`, `riscv32i` and `riscv32e`. There's also support for a large set
of instruction set extensions. This architecture support is introduced with the same standards as
currently supported targets, meaning that they come with full runtime support, immediate checking
and cache management support out of the box.

This architecture support has been sponsored by [Wasmer](https://wasmer.io).

General
-------
- dependencies have been updated
- handled many clippy warnings (Thanks to waywardmonkeys!)

Architecture support
--------------------
- added `riscv32i`, `riscv32e`, `riscv64i` and `riscv64e` architecture targets.
- `x64` gained support for the `RDPRU` instruction (Thanks to eigenform!).

Version 3.0.1
=============

Bugfix
------
- `x64` immediate const evaluation now handles negative displacements properly again, generating
  8-bit displacements where possible.

Version 3.0.0
=============

Summary
-------
This release brings significant improvements to the `aarch64` experience. The runtime assemblers now
handle cache invalidation internally where necessary. Furthermore, where previously overly large
immediates would just wrap during encoding, they are now fully checked and error at compile time, or
panic at runtime. One major backwards compatibility break is that the syntax for data directives has
changed. This syntax now uses the relevant rust type names, and supports significantly more types.
Next to this, several long-standing bugs have been fixed, dependencies have been updated, and the
crate has moved to rust edition 2021. The minimum supported rust version is now `1.77`.

General
-------
- Moved to rust edition 2021
- Dependencies have been updated.
- minimum supported rust version increased to `1.77` to allow for the use of `mem::offset_of!`.
- data directives now use `.u8`, `.i16`, `.f32` syntax.
- Several documentation improvements have been made.
- More tests have been introduced.

Architecture support
--------------------
- `x64` gained support for the `MOVDIRI` instruction
- `x64` gained support for the `ZWORD` operand size (needed for AVX512 support in the future)
- the `aarch64` assemblers now handle cache control internally. It is no longer needed for the user to handle this.
- Immediates in `aarch64` assembly are now fully size checked, where possible at compile time. Attempting to assemble an impossible immediate at runtime will cause a panic.

Bugfixes
--------
- All documentation/example crates now compile on stable rustc.
- Worked around a [rustc compiler bug](https://github.com/rust-lang/rust/issues/67062) that was causing operator precedence issues in emitted code.
- Fixed x64/x86 register names used in compiler error debug messages.
- Renamed several documentation bin targets due to artefact name collision errors in the workspace.
- Fix unneeded parenthesis warnings in emitted code.
- Eliminated warnings during plugin build.
- the aarch64 instruction `mov w0, immu32` now expects a `u32` as immediate instead of an `u64`.

Several of these changes were the result of external contributions, we'd like to thank everyone who contributed to this release.

Version 2.0.0
=============

Plugin
------
- Be stricter on X/W or XSP/WSP differentiation in x64.

Runtime
-------
- Significant performance optimization of label processing.
- Rework of various components.
- Additional APIs to reuse Assemblers, or to allocate them with pre-reserved buffers.

Documentation
-------------
- Various fixes

Version 1.2.3
=============

Plugin
------
- add x64 ud1 opcode.
- Bugfix the REPNE prefix so it can actually be used.

Version 1.2.2
=============

Global
------
Fixes travis integration displayed in crates.io

Version 1.2.1
=============

Runtime
-------
- Fix overflow in aarch64 logical immediate encoding at runtime.

Version 1.2.0
=============

Runtime
-------
- update memmap2 version.

Version 1.1.0
=============

Runtime
-------
- Added missing label management methods to `VecAssembler`.

Version 1.0.1
=============

Runtime
-------
- Switched from the unmaintained `memmap` crate to `memmap2`.

Version 1.0.0
=============

Global
------
- First release on rust stable (1.45+)!

Plugin
------
- Use new Span::mixed_site hygiene to replace call_site hygiene wherever relevant.

Version 0.7.1
=============

Runtime
-------
- Fixed an issue where calls to Modifier.extend would cause invalid code to be emitted.

Version 0.7.0
=============

Global
------
- Updated everything to the 2018 edition of rust.
- The crate will be able to be used on a rust 1.45 stable compiler!

Plugin
------
- File-local directives now requires the `filelocal` feature and a nightly compiler.
- Now compiles and is usable on a stable 1.45 rust compiler.
- Added `dynasm_backwards!`, which emits templates in reverse order. This allows writing assemblers that compile code backwards.

Runtime
-------
- `dynasmrt` now re-exports the `dynasm!` macro from `dynasm`, so projects only need the `dynasmrt` dependency.
- Refactoring PatchLoc and Relocation to generalize relocation handling between architectures.
- Added register family listings, to facilitate dynamic register usage. This feature was contributed by vext01.

Version 0.6.0
=============

Runtime
-------
- Significant internal refactoring: Architectural support is now abstracted through the Relocation trait, that all assemblers are generic over. Aliases for the previous architecture-specific assemblers are kept for backwards compatibility.
- Made assembler components public to facilitate writing custom assemblers.

Tests
-----
- Reduced compile time significantly by migrating simple instruction format tests to `SimpleAssembler`.

Version 0.5.2
=============

Plugin
------
- Remove a problematic semicolon that might cause errors in the future.
- README update.

Version 0.5.1
=============

Runtime
-------
- Fix a bug that caused emitted code corruption corruption in Aarch64's B and ADRP relocation calculations.

Version 0.5.0
=============

Plugin
------
- We now support an arbitrary offset added to any jump. This feature allows direct addressing of literal pools or arrays of values without generating labels for every single item in them.

Runtime
-------
- Added LitPool, a tool to automatically build literal pools.
- Impossible relocations no longer panic. Instead they cause an error to be emitted from `Assembler::commit()`.
- Significant refactoring.

Version 0.4.0
=============

Architecture support
--------------------
- Added Aarch64 support, thanks to sponsoring from [Wasmer](https://wasmer.io).

Plugin
------
- Remove use of mem::unitialized when calculating the offsets of members in structs in generated code.
-Optimization of opmap and register tables to reduce compile time and file size.

Documentation
-------------
- Split architecture-specific DSL documentation and generic DSL documentation.
- Ported examples to Aarch64.

Version 0.3.2
=============

Plugin
------
- x64/x86: Always escape into SIB encoding with a displacement when a register is used as a memory reference base, to allow RBP to be encoded.
- x64/x86: Properly handle dynamic registers in VEX prefixes

Version 0.3.1
=============

Plugin
------
- Fixed an issue where dynamic registers caused wrong code generation.

Runtime
-------
- Added an API for returning the offset of a dynamic label directly.

Version 0.3.0
=============

Architecture support
--------------------
- Fixed SETB and SETNB not being present in x64/x86

Plugin
------
- The plugin has been updated to the new procedural macro system. This means it should no longer break when new compiler versions are released.
- Directives are now scoped file-local instead of crate-local.

Runtime
-------
- The alter API can now return values produced by the user-defined closure.

Versions 0.2.1, 0.2.2 and 0.2.3
=============

Plugin
------
- Updates so it stays compatible with rustc as the plugin API changes.

Version 0.2.0
=============

Architecture support
--------------------
- Added support for x86
- Enabled relocation support in x64 when additional immediate fields are present.

Plugin
------
- Significant refactoring of the x64 backend to also handle x86
- Added x64 feature selection support.

Runtime
-------
- Added managed relocation support to handle non-relative relocations.

Versions 0.1.4, 0.1.3, 0.1.2, 0.1.1
===================================

Plugin
------
- Updates so it stays compatible with rustc as the plugin API changes.

Versions <= 0.1.0
========================

Global
------
- Initial library experiments, starting with basic x64 support, refactoring to support multiple architectures, and cleanup.
