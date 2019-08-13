
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

pub fn encode_logical_immediate_32bit(value: u32) -> Option<u16> {
    let transitions = value ^ value.rotate_right(1);
    let element_size = (64u32).checked_div(transitions.count_ones())?;

    // confirm that the elements are identical
    if value != value.rotate_left(element_size) {
        return None;
    }

    let element = value & (1u32 << element_size).wrapping_sub(1);
    let ones = element.count_ones();
    let imms = (!(element_size - 1) & 0x3F) | ones;

    let immr = if (element & 1) != 0 {
        ones - (!element).leading_zeros()
    } else {
        element_size - element.leading_zeros()
    };

    Some(((imms as u16) << 6) | (immr as u16))
}

pub fn encode_logical_immediate_64bit(value: u64) -> Option<u16> {
    let transitions = value ^ value.rotate_right(1);
    let element_size = (128u32).checked_div(transitions.count_ones())?;

    // confirm that the elements are identical
    if value != value.rotate_left(element_size) {
        return None;
    }

    let element = value & (1u64 << element_size).wrapping_sub(1);
    let ones = element.count_ones();
    let imms = ((!(element_size - 1) & 0x7F) ^ 0x40) | ones;

    let immr = if (element & 1) != 0 {
        ones - (!element).leading_zeros()
    } else {
        element_size - element.leading_zeros()
    };

    Some(((imms as u16) << 6) | (immr as u16))
}

pub fn encode_stretched_immediate(value: u64) -> Option<u32> {
    // ensure the number is formatted correctly
    let mut test = value & 0x0101010101010101;
    test |= test << 1;
    test |= test << 2;
    test |= test << 4;
    if test != value {
        return None;
    }

    // do bitwise magic
    let mut masked = value & 0x8040201008040201;
    masked |= masked >> 32;
    masked |= masked >> 16;
    masked |= masked >> 8;
    let masked = masked as u32;
    Some(masked & 0xFF)
}

pub fn encode_wide_immediate(value: u64) -> Option<u32> {
    let offset = value.leading_zeros() & 0b110000;
    let masked = 0xFFFF & (value >> offset);
    if (masked << offset) == value {
        Some((masked as u32) | (offset << 12))
    } else {
        None
    }
}

pub fn encode_inverted_wide_immediate(value: u64) -> Option<u32> {
    encode_wide_immediate(!value)
}