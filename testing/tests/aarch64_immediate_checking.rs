extern crate dynasmrt;

use dynasmrt::{dynasm, SimpleAssembler, DynasmApi};

// custom tests to validate aarch64 oversize immediate errors


macro_rules! dynasm {
    ($ops:ident $($t:tt)*) => {
        dynasmrt::dynasm!($ops
            ; .arch aarch64
            $($t)*
        )
    }
}

// passthrough function, hides immediates from static analysis.
fn hide<T>(t: T) -> T { t }



// tests are structured like this
// in range immediate:
//  test with static immediates, to validate static encoding
//  test with dynamic immediates, to validate dynamic encoding
// out of range immediate:
//  test with dynamic immediates, should panic

// there's no test for static out of range immediates, as these don't even compile. 


// Ubits

#[test]
fn test_ubits_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; add x1, x2, x3, LSL 0
        ; add x1, x2, x3, LSL 63
        ; add x1, x2, x3, LSL hide(0)
        ; add x1, x2, x3, LSL hide(63)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "41 00 03 8B 41 FC 03 8B 41 00 03 8B 41 FC 03 8B", "Ubits encoding");
}

#[test]
#[should_panic]
fn test_ubits_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; add x1, x2, x3, LSL hide(64)

    );
}

// Uscaled

#[test]
fn test_uscaled_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldr x1, [x2, 0]
        ; ldr x1, [x2, 32760]
        ; ldr x1, [x2, hide(0)]
        ; ldr x1, [x2, hide(32760)]
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "41 00 40 F9 41 FC 7F F9 41 00 40 F9 41 FC 7F F9", "Uscaled encoding");
}

#[test]
#[should_panic]
fn test_uscaled_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldr x1, [x2, hide(32768)]
    );
}

#[test]
#[should_panic]
fn test_uscaled_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldr x1, [x2, hide(32759)]
    );
}

#[test]
#[should_panic]
fn test_uscaled_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldr x1, [x2, hide(1)]
    );
}

// Ulist

#[test]
fn test_ulist_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; add x0, x1, 2047, LSL 0
        ; add x0, x1, 2047, LSL 12
        ; add x0, x1, 2047, LSL hide(0)
        ; add x0, x1, 2047, LSL hide(12)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "20 FC 1F 91 20 FC 5F 91 20 FC 1F 91 20 FC 5F 91", "Ulist encoding");
}

#[test]
#[should_panic]
fn test_ulist_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; add x0, x1, 2047, LSL hide(1)
    );
}

#[test]
#[should_panic]
fn test_ulist_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; add x0, x1, 2047, LSL hide(13)
    );
}

// Urange

#[test]
fn test_urange_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; add x0, x1, x2, UXTX 0
        ; add x0, x1, x2, UXTX 4
        ; add x0, x1, x2, UXTX hide(0)
        ; add x0, x1, x2, UXTX hide(4)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "20 60 22 8B 20 70 22 8B 20 60 22 8B 20 70 22 8B", "Urange encoding");
}

#[test]
#[should_panic]
fn test_urange_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; add x0, x1, x2, UXTX hide(5)
    );
}

// Usubone

#[test]
fn test_usubone_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fcvtzs x1, d2, 1
        ; fcvtzs x1, d2, 64
        ; fcvtzs x1, d2, hide(1)
        ; fcvtzs x1, d2, hide(64)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "41 FC 58 9E 41 00 58 9E 41 FC 58 9E 41 00 58 9E", "Usubone encoding");
}

#[test]
#[should_panic]
fn test_usubone_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fcvtzs x1, d2, hide(0)
    );
}

#[test]
#[should_panic]
fn test_usubone_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fcvtzs x1, d2, hide(65)
    );
}

// Usubzero and Usubmod

#[test]
fn test_usubzero_usubmod_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; lsl x0, x1, 0
        ; lsl x0, x1, 63
        ; lsl x0, x1, hide(0)
        ; lsl x0, x1, hide(63)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "20 FC 40 D3 20 00 41 D3 20 FC 40 D3 20 00 41 D3", "Usubzero/Usubmod encoding");
}

#[test]
#[should_panic]
fn test_usubzero_usubmod_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; lsl x0, x1, hide(64)
    );
}

// Usum

#[test]
fn test_usum_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; bfxil x1, x2, 32, 1
        ; bfxil x1, x2, 32, 32
        ; bfxil x1, x2, 32, hide(1)
        ; bfxil x1, x2, 32, hide(32)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "41 80 60 B3 41 FC 60 B3 41 80 60 B3 41 FC 60 B3", "Usum encoding");
}

#[test]
#[should_panic]
fn test_usum_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; bfxil x1, x2, 32, hide(0)
    );
}

#[test]
#[should_panic]
fn test_usum_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; bfxil x1, x2, 32, hide(33)
    );
}

// Ufields

#[test]
fn test_ufields_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fcmla v1.H8, v2.H8, v3.H[0], 180
        ; fcmla v1.H8, v2.H8, v3.H[3], 180
        ; fcmla v1.H8, v2.H8, v3.H[hide(0)], 180
        ; fcmla v1.H8, v2.H8, v3.H[hide(3)], 180
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "41 50 43 6F 41 58 63 6F 41 50 43 6F 41 58 63 6F", "Ufields encoding");
}

#[test]
#[should_panic]
fn test_ufields_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fcmla v1.H8, v2.H8, v3.H[hide(4)], 180
    );
}

// Sbits

#[test]
fn test_sbits_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldr x1, [x2], -256
        ; ldr x1, [x2], 255
        ; ldr x1, [x2], hide(-256)
        ; ldr x1, [x2], hide(255)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "41 04 50 F8 41 F4 4F F8 41 04 50 F8 41 F4 4F F8", "Sbits encoding");
}

#[test]
#[should_panic]
fn test_sbits_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldr x1, [x2], hide(-257)
    );
}

#[test]
#[should_panic]
fn test_sbits_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldr x1, [x2], hide(256)
    );
}

// Sscaled

#[test]
fn test_sscaled_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldp x1, x0, [x2, 504]
        ; ldp x1, x0, [x2, -512]
        ; ldp x1, x0, [x2, hide(504)]
        ; ldp x1, x0, [x2, hide(-512)]
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "41 80 5F A9 41 00 60 A9 41 80 5F A9 41 00 60 A9", "Sscaled encoding");
}

#[test]
#[should_panic]
fn test_sscaled_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldp x1, x0, [x2, hide(512)]
    );
}

#[test]
#[should_panic]
fn test_sscaled_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldp x1, x0, [x2, hide(-520)]
    );
}

#[test]
#[should_panic]
fn test_sscaled_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldp x1, x0, [x2, hide(4)]
    );
}

// CUbits

#[test]
fn test_cubits_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; tbnz x1, 0, 0
        ; tbnz x1, 63, 0
        ; tbnz x1, hide(0), 0
        ; tbnz x1, hide(63), 0
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "01 00 00 37 01 00 F8 B7 01 00 00 37 01 00 F8 B7", "CUbits encoding");
}

#[test]
#[should_panic]
fn test_cubits_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; tbnz x1, hide(64), 0
    );
}

// CUsum

#[test]
fn test_cusum_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; bfc x1, 0, 1
        ; bfc x1, 32, 1
        ; bfc x1, 32, 32
        ; bfc x1, hide(0), hide(1)
        ; bfc x1, hide(32), hide(1)
        ; bfc x1, hide(32), hide(32)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "E1 03 40 B3 E1 03 60 B3 E1 7F 60 B3 E1 03 40 B3 E1 03 60 B3 E1 7F 60 B3", "CUsum encoding");
}

#[test]
#[should_panic]
fn test_cusum_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; bfc x1, hide(0), hide(0)
    );
}

#[test]
#[should_panic]
fn test_cusum_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; bfc x1, hide(32), hide(33)
    );
}

// CSscaled

#[test]
fn test_csscaled_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldraa x1, [x2, 0xff8]
        ; ldraa x1, [x2, -0x1000]
        ; ldraa x1, [x2, hide(0xff8)]
        ; ldraa x1, [x2, hide(-0x1000)]
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "41 F4 3F F8 41 04 60 F8 41 F4 3F F8 41 04 60 F8", "CSscaled encoding");
}

#[test]
#[should_panic]
fn test_csscaled_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldraa x1, [x2, hide(-0x1008)]
    );
}

#[test]
#[should_panic]
fn test_csscaled_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldraa x1, [x2, hide(0x1000)]
    );
}

#[test]
#[should_panic]
fn test_csscaled_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; ldraa x1, [x2, hide(0x4)]
    );
}

// CUrange

#[test]
fn test_curange_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fcvtzs s1, s2, 1
        ; fcvtzs s1, s2, 32
        ; fcvtzs s1, s2, hide(1)
        ; fcvtzs s1, s2, hide(32)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "41 FC 3F 5F 41 FC 20 5F 41 FC 3F 5F 41 FC 20 5F", "CUrange encoding");
}

#[test]
#[should_panic]
fn test_curange_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fcvtzs s1, s2, hide(0)
    );
}

#[test]
#[should_panic]
fn test_curange_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fcvtzs s1, s2, hide(33)
    );
}

// inverted wide immediate 32

#[test]
fn test_inverted_wide_32_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.inverted w1, 0xFFFF1234
        ; mov.inverted w1, 0x1234FFFF
        ; mov.inverted w1, hide(0xFFFF1234)
        ; mov.inverted w1, hide(0x1234FFFF)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "61 B9 9D 12 61 B9 BD 12 61 B9 9D 12 61 B9 BD 12", "Inverted wide 32 encoding");
}

#[test]
#[should_panic]
fn test_inverted_wide_32_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.inverted w1, hide(0xEFFF1234)
    );
}

#[test]
#[should_panic]
fn test_inverted_wide_32_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.inverted w1, hide(0x1234FFFE)
    );
}

#[test]
#[should_panic]
fn test_inverted_wide_32_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.inverted w1, hide(0xFF1234FF)
    );
}

// inverted wide immediate 64

#[test]
fn test_inverted_wide_64_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.inverted x1, 0xFFFFFFFFFFFF1234
        ; mov.inverted x1, 0xFFFFFFFF1234FFFF
        ; mov.inverted x1, 0xFFFF1234FFFFFFFF
        ; mov.inverted x1, 0x1234FFFFFFFFFFFF
        ; mov.inverted x1, hide(0xFFFFFFFFFFFF1234)
        ; mov.inverted x1, hide(0xFFFFFFFF1234FFFF)
        ; mov.inverted x1, hide(0xFFFF1234FFFFFFFF)
        ; mov.inverted x1, hide(0x1234FFFFFFFFFFFF)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "61 B9 9D 92 61 B9 BD 92 61 B9 DD 92 61 B9 FD 92 61 B9 9D 92 61 B9 BD 92 61 B9 DD 92 61 B9 FD 92", "Inverted wide 64 encoding");
}

#[test]
#[should_panic]
fn test_inverted_wide_64_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.inverted x1, hide(0xEFFFFFFFFFFF1234)
    );
}

#[test]
#[should_panic]
fn test_inverted_wide_64_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.inverted x1, hide(0x1234FFFFFFFFFFFE)
    );
}

#[test]
#[should_panic]
fn test_inverted_wide_64_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.inverted x1, hide(0xFF1234FFFFFFFFFF)
    );
}

#[test]
#[should_panic]
fn test_inverted_wide_64_fail_3() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.inverted x1, hide(0xFFFFFFFFFF1234FF)
    );
}

// wide immediate 32

#[test]
fn test_wide_32_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov w1, 0x00001234
        ; mov w1, 0x12340000
        ; mov w1, hide(0x00001234)
        ; mov w1, hide(0x12340000)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "81 46 82 52 81 46 A2 52 81 46 82 52 81 46 A2 52", "Wide 32 encoding");
}

#[test]
#[should_panic]
fn test_wide_32_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov w1, hide(0x80001234)
    );
}

#[test]
#[should_panic]
fn test_wide_32_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov w1, hide(0x12340001)
    );
}

#[test]
#[should_panic]
fn test_wide_32_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov w1, hide(0x00123400)
    );
}

// wide immediate 64

#[test]
fn test_wide_64_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov x1, 0x0000000000001234
        ; mov x1, 0x0000000012340000
        ; mov x1, 0x0000123400000000
        ; mov x1, 0x1234000000000000
        ; mov x1, hide(0x0000000000001234)
        ; mov x1, hide(0x0000000012340000)
        ; mov x1, hide(0x0000123400000000)
        ; mov x1, hide(0x1234000000000000)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "81 46 82 D2 81 46 A2 D2 81 46 C2 D2 81 46 E2 D2 81 46 82 D2 81 46 A2 D2 81 46 C2 D2 81 46 E2 D2", "Wide 64 encoding");
}

#[test]
#[should_panic]
fn test_wide_64_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov x1, hide(0x8000000000001234)
    );
}

#[test]
#[should_panic]
fn test_wide_64_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov x1, hide(0x1234000000000001)
    );
}

#[test]
#[should_panic]
fn test_wide_64_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov x1, hide(0x0012340000000000)
    );
}

#[test]
#[should_panic]
fn test_wide_64_fail_3() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov x1, hide(0x0000000000123400)
    );
}

// stretched immediate

#[test]
fn test_stretched_64_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; movi d1, 0xFF00FF0000FF00FF
        ; movi d1, 0x00FF00FF00FF00FF
        ; movi d1, 0xFFFFFFFFFFFFFFFF
        ; movi d1, 0x0000000000000000
        ; movi d1, hide(0xFF00FF0000FF00FF)
        ; movi d1, hide(0x00FF00FF00FF00FF)
        ; movi d1, hide(0xFFFFFFFFFFFFFFFF)
        ; movi d1, hide(0x0000000000000000)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "A1 E4 05 2F A1 E6 02 2F E1 E7 07 2F 01 E4 00 2F A1 E4 05 2F A1 E6 02 2F E1 E7 07 2F 01 E4 00 2F", "Stretched 64 encoding");
}

#[test]
#[should_panic]
fn test_stretched_64_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; movi d1, hide(0x8040201008040201)
    );
}

#[test]
#[should_panic]
fn test_stretched_64_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; movi d1, hide(0x0101010101010101)
    );
}

#[test]
#[should_panic]
fn test_stretched_64_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; movi d1, hide(0xF0F0F0F0F0F0F0F0)
    );
}

#[test]
#[should_panic]
fn test_stretched_64_fail_3() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; movi d1, hide(0xF0F0F0F0F0F0F0F0)
    );
}

// logical immediate 32

#[test]
fn test_logical_32_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical w1, 0x55555555
        ; mov.logical w1, 0x11111111
        ; mov.logical w1, 0x01010101
        ; mov.logical w1, 0x01000100
        ; mov.logical w1, 0x0FFE0FFE
        ; mov.logical w1, hide(0x55555555)
        ; mov.logical w1, hide(0x11111111)
        ; mov.logical w1, hide(0x01010101)
        ; mov.logical w1, hide(0x01000100)
        ; mov.logical w1, hide(0x0FFE0FFE)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "E1 F3 00 32 E1 E3 00 32 E1 C3 00 32 E1 83 08 32 E1 AB 0F 32 E1 F3 00 32 E1 E3 00 32 E1 C3 00 32 E1 83 08 32 E1 AB 0F 32", "Logical 32 encoding");
}

#[test]
#[should_panic]
fn test_logical_32_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical w1, hide(0x5555AAAA)
    );
}

#[test]
#[should_panic]
fn test_logical_32_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical w1, hide(0x00000000)
    );
}

#[test]
#[should_panic]
fn test_logical_32_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical w1, hide(0xFFFFFFFF)
    );
}

#[test]
#[should_panic]
fn test_logical_32_fail_3() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical w1, hide(0x10003000)
    );
}

#[test]
#[should_panic]
fn test_logical_32_fail_4() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical w1, hide(0x10010010)
    );
}

// logical immediate 64

#[test]
fn test_logical_64_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical x1, 0x5555555555555555
        ; mov.logical x1, 0x1111111111111111
        ; mov.logical x1, 0x0101010101010101
        ; mov.logical x1, 0x0100010001000100
        ; mov.logical x1, 0x0F0000000F000000
        ; mov.logical x1, 0x000000000FFFFE00
        ; mov.logical x1, hide(0x5555555555555555)
        ; mov.logical x1, hide(0x1111111111111111)
        ; mov.logical x1, hide(0x0101010101010101)
        ; mov.logical x1, hide(0x0100010001000100)
        ; mov.logical x1, hide(0x0F0000000F000000)
        ; mov.logical x1, hide(0x000000000FFFFE00)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "E1 F3 00 B2 E1 E3 00 B2 E1 C3 00 B2 E1 83 08 B2 E1 0F 08 B2 E1 4B 77 B2 E1 F3 00 B2 E1 E3 00 B2 E1 C3 00 B2 E1 83 08 B2 E1 0F 08 B2 E1 4B 77 B2", "Logical 64 encoding");
}

#[test]
#[should_panic]
fn test_logical_64_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical x1, hide(0x55555555AAAAAAAA)
    );
}

#[test]
#[should_panic]
fn test_logical_64_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical x1, hide(0x0000000000000000)
    );
}

#[test]
#[should_panic]
fn test_logical_64_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical x1, hide(0xFFFFFFFFFFFFFFFF)
    );
}

#[test]
#[should_panic]
fn test_logical_64_fail_3() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical x1, hide(0x1000300010003000)
    );
}

#[test]
#[should_panic]
fn test_logical_64_fail_4() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; mov.logical x1, hide(0x1001001001001001)
    );
}

// float immediate

#[test]
fn test_float_32_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fmov d1, 0.125
        ; fmov d1, 31.0
        ; fmov d1, -0.125
        ; fmov d1, -31.0
        ; fmov d1, hide(0.125)
        ; fmov d1, hide(31.0)
        ; fmov d1, hide(-0.125)
        ; fmov d1, hide(-31.0)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "01 10 68 1E 01 F0 67 1E 01 10 78 1E 01 F0 77 1E 01 10 68 1E 01 F0 67 1E 01 10 78 1E 01 F0 77 1E", "Float 32 encoding");
}

#[test]
#[should_panic]
fn test_float_32_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fmov d1, hide(0.0625)
    );
}

#[test]
#[should_panic]
fn test_float_32_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fmov d1, hide(32.0)
    );
}

#[test]
#[should_panic]
fn test_float_32_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fmov d1, hide(1.001)
    );
}

#[test]
#[should_panic]
fn test_float_32_fail_3() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fmov d1, hide(0.0)
    );
}

// split float immediate

#[test]
fn test_split_float_32_pass() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fmov v1.d2, 0.125
        ; fmov v1.d2, 31.0
        ; fmov v1.d2, -0.125
        ; fmov v1.d2, -31.0
        ; fmov v1.d2, hide(0.125)
        ; fmov v1.d2, hide(31.0)
        ; fmov v1.d2, hide(-0.125)
        ; fmov v1.d2, hide(-31.0)
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(" ");
    assert_eq!(hex, "01 F4 02 6F E1 F7 01 6F 01 F4 06 6F E1 F7 05 6F 01 F4 02 6F E1 F7 01 6F 01 F4 06 6F E1 F7 05 6F", "Split float 32 encoding");
}

#[test]
#[should_panic]
fn test_split_float_32_fail_0() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fmov v1.d2, hide(0.0625)
    );
}

#[test]
#[should_panic]
fn test_split_float_32_fail_1() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fmov v1.d2, hide(32.0)
    );
}

#[test]
#[should_panic]
fn test_split_float_32_fail_2() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fmov v1.d2, hide(1.001)
    );
}

#[test]
#[should_panic]
fn test_split_float_32_fail_3() {
    let mut ops = SimpleAssembler::new();
    dynasm!(ops
        ; fmov v1.d2, hide(0.0)
    );
}

