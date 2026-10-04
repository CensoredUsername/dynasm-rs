use dynasmrt::{dynasm, DynasmApi};
use std::mem::{offset_of, size_of};

#[repr(C)]
struct Inner {
    a: u8,
    b: u32,
    pair: (u16, u64),
}

#[repr(C)]
struct Frame {
    first: u8,
    inner: Inner,
    text_section_limit: u32,
}

fn asm<F: FnOnce(&mut dynasmrt::VecAssembler<dynasmrt::x64::X64Relocation>)>(f: F) -> Vec<u8> {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);
    f(&mut ops);
    ops.finalize().unwrap()
}

#[test]
fn typemap_negative_index() {
    let expected = asm(|ops| dynasm!(ops
        ; .arch x64
        ; mov eax, DWORD [BYTE rbp + (offset_of!(Frame, text_section_limit) as i8 - size_of::<Frame>() as i8)]
    ));
    let actual = asm(|ops| dynasm!(ops
        ; .arch x64
        ; mov eax, DWORD rbp => Frame[BYTE -1].text_section_limit
    ));
    assert_eq!(expected, actual);
}

#[test]
fn typemap_nested_fields() {
    let expected = asm(|ops| dynasm!(ops
        ; .arch x64
        ; mov eax, DWORD [rbp + offset_of!(Frame, inner.b) as i32]
        ; mov rax, QWORD [rbp + offset_of!(Frame, inner.pair.1) as i32]
        ; mov ax, WORD [rbp + offset_of!(Frame, inner.pair.0) as i32]
    ));
    let actual = asm(|ops| dynasm!(ops
        ; .arch x64
        ; mov eax, DWORD rbp => Frame.inner.b
        ; mov rax, QWORD rbp => Frame.inner.pair.1
        ; mov ax, WORD rbp => Frame.inner.pair.0
    ));
    assert_eq!(expected, actual);
}

#[repr(C)]
struct Wrapper<T> {
    pad: u8,
    value: (u16, (u8, T)),
}

// the type and the field specifier are handed to offset_of! as-is
#[test]
fn typemap_offset_of_syntax() {
    let expected = asm(|ops| dynasm!(ops
        ; .arch x64
        ; mov eax, DWORD [rbp + offset_of!((u8, u32), 1) as i32]
        ; mov eax, DWORD [rbp + offset_of!(Wrapper<u32>, value.1.1) as i32]
    ));
    let actual = asm(|ops| dynasm!(ops
        ; .arch x64
        ; mov eax, DWORD rbp => (u8, u32).1
        ; mov eax, DWORD rbp => Wrapper<u32>.value.1.1
    ));
    assert_eq!(expected, actual);
}
