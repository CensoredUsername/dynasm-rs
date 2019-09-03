use crate::relocations::Aarch64Relocation;

pub type Assembler = crate::Assembler<Aarch64Relocation>;
pub type AssemblyModifier<'a> = crate::Modifier<'a, Aarch64Relocation>;
pub type UncommittedModifier<'a> = crate::UncommittedModifier<'a>;


/// Helper function for validating that a given value can be encoded as a 32-bit logical immediate
pub fn encode_logical_immediate_32bit(value: u32) -> Option<u16> {
    let transitions = value ^ value.rotate_right(1);
    let element_size = (64u32).checked_div(transitions.count_ones())?;

    // confirm that the elements are identical
    if value != value.rotate_left(element_size) {
        return None;
    }

    let element = value & (1u32 << element_size).wrapping_sub(1);
    let ones = element.count_ones();
    let imms = (!((element_size << 1) - 1) & 0x3F) | (ones - 1);

    let immr = if (element & 1) != 0 {
        ones - (!element).trailing_zeros()
    } else {
        element_size - element.trailing_zeros()
    };

    Some(((immr as u16) << 6) | (imms as u16))
}

/// Helper function for validating that a given value can be encoded as a 64-bit logical immediate
pub fn encode_logical_immediate_64bit(value: u64) -> Option<u16> {
    let transitions = value ^ value.rotate_right(1);
    let element_size = (128u32).checked_div(transitions.count_ones())?;

    // confirm that the elements are identical
    if value != value.rotate_left(element_size) {
        return None;
    }

    let element = value & (1u64 << element_size).wrapping_sub(1);
    let ones = element.count_ones();
    let imms = (!((element_size << 1) - 1) & 0x7F) | (ones - 1);

    let immr = if (element & 1) != 0 {
        ones - (!element).trailing_zeros()
    } else {
        element_size - element.trailing_zeros()
    };

    let n = imms & 0x40 == 0;
    let imms = imms & 0x3F;

    Some(((n as u16) << 12) | ((immr as u16) << 6) | (imms as u16))
}

/// Helper function for validating that a given value can be encoded as a floating point immediate
pub fn encode_floating_point_immediate(value: f32) -> Option<u8> {
    // floating point ARM immediates are encoded as
    // abcdefgh => aBbbbbbc defgh000 00000000 00000000
    // where B = !b
    // which means we can just slice out "a" and "bcdefgh" and assume the rest was correct

    let bits = value.to_bits();

    let check = (bits >> 25) & 0x3F;
    if (check == 0b100000 || check == 0b011111) && (bits & 0x7FFFF) == 0 {
        Some((((bits >> 24) & 0x80) | ((bits >> 19) & 0x7F)) as u8)
    } else {
        None
    }
}