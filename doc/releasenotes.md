# Release notes

The `dynasm-rs` project consists out of two crates: The procedural macro crate `dynasm` and the runtime support crate `dynasmrt`. The versions of these two crates are synchronized and should always match. From version 0.7.0 onwards `dynasmrt` depends on `dynasm` itself to simplify this relationship. Any version listings below therefore refers to both the `dynasm` and `dynasmrt` crate version.

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