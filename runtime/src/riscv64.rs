
// these should explicitly never be inlined, as this is the slow path.
// that's also why these aren't made generic.

/// Handler for `u32` out-of-range riscv64 immediates.
#[inline(never)]
pub fn immediate_out_of_range_unsigned_32(immediate: u32) -> ! {
    panic!("Cannot assemble this RISC-V instruction. Immediate {immediate} is out of range.")
}

/// Handler for `i32` out-of-range riscv64 immediates.
#[inline(never)]
pub fn immediate_out_of_range_signed_32(immediate: i32) -> ! {
    panic!("Cannot assemble this RISC-V instruction. Immediate {immediate} is out of range.")
}

/// Handler for invalid riscv64 registers.
#[inline(never)]
pub fn invalid_register(immediate: i32) -> ! {
    panic!("Cannot assemble this RISC-V instruction. Register x{immediate} cannot be encoded.")
}
