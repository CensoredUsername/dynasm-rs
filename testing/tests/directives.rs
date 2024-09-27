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
fn test_data_directives_unaligned() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .byte 0x00
        ; .word 0x1111
        ; .dword 0x22222222
        ; .qword 0x3333333333333333
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "00, 11, 11, 22, 22, 22, 22, 33, 33, 33, 33, 33, 33, 33, 33", "Register aliases");
}

#[test]
fn test_data_directives_aligned() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch x64
        ; .byte 0x00
        ; .align 2
        ; .word 0x1111
        ; .align 8
        ; .dword 0x22222222
        ; .align 8
        ; .qword 0x3333333333333333
    );

    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "00, 90, 11, 11, 90, 90, 90, 90, 22, 22, 22, 22, 90, 90, 90, 90, 33, 33, 33, 33, 33, 33, 33, 33", "Register aliases");
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
