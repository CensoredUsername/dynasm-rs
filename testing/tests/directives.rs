// Testcases for dynasm-rs directives
use dynasmrt::{dynasm, DynasmApi};


#[cfg(target_arch="x86_64")]
#[test]
fn test_default_arch() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; inc DWORD [rax*8 + rbx + 16]
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "FF, 44, C3, 10", "Default arch is x64");
}

#[cfg(target_arch="x86")]
#[test]
fn test_default_arch() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        // this instruction is encoded differently in x86 and x64
        ; inc eax
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "40", "Default arch is x86");
}

#[cfg(target_arch="aarch64")]
#[test]
fn test_default_arch() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; ldr x2, [x11, w12, uxtw #3]
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "62, 59, 6C, F8", "Default arch is aarch64");
}

#[test]
fn test_arch_switching() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch x64
        ; inc DWORD [rax*8 + rbx + 16]
        ; .arch x86
        ; inc eax
        ; .arch aarch64
        ; ldr x2, [x11, w12, uxtw #3]
        ; .arch unknown
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "FF, 44, C3, 10, 40, 62, 59, 6C, F8", "Switching between architectures");
}

#[test]
fn test_x64_features() {
    dynasm!(()
        ; .arch x64
        ; .feature fpu
        ; .feature mmx
        ; .feature tdnow
        ; .feature sse
        ; .feature sse2
        ; .feature sse3
        ; .feature vmx
        ; .feature ssse3
        ; .feature sse4a
        ; .feature sse41
        ; .feature sse42
        ; .feature sse5
        ; .feature avx
        ; .feature avx2
        ; .feature fma
        ; .feature bmi1
        ; .feature bmi2
        ; .feature tbm
        ; .feature rtm
        ; .feature invpcid
        ; .feature mpx
        ; .feature sha
        ; .feature prefetchwt1
        ; .feature cyrix
        ; .feature amd
        ; .feature directstores
        // multiple
        ; .feature sse, sse2
    );
}

#[test]
fn test_aliases() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch x64
        ; .alias first, rax
        ; .alias second, rbx
        ; inc DWORD [first*8 + second + 16]
        ; .arch x86
        ; .alias first_but_smaller, eax
        ; inc first_but_smaller
        ; .arch aarch64
        ; .alias one, x2
        ; .alias two, x11
        ; .alias three, w12
        ; ldr one, [two, three, uxtw #3]
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "FF, 44, C3, 10, 40, 62, 59, 6C, F8", "Register aliases");
}

#[test]
fn test_data_directives_unaligned_unsigned() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .u8  0x88
        ; .u16 0x9999
        ; .u32 0xAAAAAAAA
        ; .u64 0xBBBBBBBBBBBBBBBB
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "88, 99, 99, AA, AA, AA, AA, BB, BB, BB, BB, BB, BB, BB, BB", "Data directives");
}

#[test]
fn test_data_directives_unaligned_signed() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .i8  -0x44
        ; .i16 -0x1111
        ; .i32 -0x22222222
        ; .i64 -0x3333333333333333
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "BC, EF, EE, DE, DD, DD, DD, CD, CC, CC, CC, CC, CC, CC, CC", "Data directives");
}

#[test]
fn test_data_directives_aligned_unsigned() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch x64
        ; .u8 0x88
        ; .align 2
        ; .u16 0x9999
        ; .align 8
        ; .u32 0xAAAAAAAA
        ; .align 8
        ; .u64 0xBBBBBBBBBBBBBBBB
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "88, 90, 99, 99, 90, 90, 90, 90, AA, AA, AA, AA, 90, 90, 90, 90, BB, BB, BB, BB, BB, BB, BB, BB", "Data directives");
}

#[test]
fn test_data_directives_aligned_signed() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch x64
        ; .i8 -0x44
        ; .align 2
        ; .i16 -0x1111
        ; .align 8
        ; .i32 -0x22222222
        ; .align 8
        ; .i64 -0x3333333333333333
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "BC, 90, EF, EE, 90, 90, 90, 90, DE, DD, DD, DD, 90, 90, 90, 90, CD, CC, CC, CC, CC, CC, CC, CC", "Data directives");
}

#[test]
fn test_data_directives_unaligned_float() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch x64
        ; .f32 3.14159265359
        ; .f64 3.14159265359
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "DB, 0F, 49, 40, EA, 2E, 44, 54, FB, 21, 09, 40", "Data directives");
}

#[test]
fn test_data_directives_aligned_float() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch x64
        ; .f32 3.14159265359
        ; .align 8
        ; .f64 3.14159265359
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "DB, 0F, 49, 40, 90, 90, 90, 90, EA, 2E, 44, 54, FB, 21, 09, 40", "Data directives");
}

#[test]
fn test_bytes() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    let data: Vec<u8> = (0 .. 0xFF).collect();
    dynasm!(ops
        ; .bytes &data
    );

    let buf = ops.finalize();
    assert_eq!(data, buf, "bytes directive");
}
