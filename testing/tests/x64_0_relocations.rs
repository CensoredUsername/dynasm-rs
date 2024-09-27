#![allow(unused_imports)]

use dynasmrt::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi, DynasmError, LabelKind, DynamicLabel};


#[test]
fn test_local_jumps() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);

    dynasm!(ops
        ; .arch x64
        ; jmp BYTE >foo
        ; inc rax
        ; foo:
        ; dec rax
        ; jmp BYTE <foo

        ; jmp >bar
        ; inc rax
        ; bar:
        ; dec rax
        ; jmp <bar

        ; jmp >bar
        ; jmp >foo
        ; jmp <foo
        ; jmp <bar

        ; jmp BYTE >foo
        ; inc rax
        ; foo:
        ; dec rax
        ; jmp BYTE <foo

        ; jmp BYTE >bar
        ; inc rax
        ; bar:
        ; dec rax
        ; jmp BYTE <bar

        ; jmp >close
        ; close:
        ; jmp <close
    );

    let output = ops.finalize().unwrap();

    for i in &output {
        print!("\\x{:02x}", i);
    }
    println!("");

    let expected: &[u8] = b"\
\xeb\x03\x48\xff\xc0\x48\xff\xc8\xeb\xfb\xe9\x03\x00\x00\x00\x48\
\xff\xc0\x48\xff\xc8\xe9\xf8\xff\xff\xff\xe9\x1e\x00\x00\x00\xe9\
\x0f\x00\x00\x00\xe9\xdc\xff\xff\xff\xe9\xe4\xff\xff\xff\xeb\x03\
\x48\xff\xc0\x48\xff\xc8\xeb\xfb\xeb\x03\x48\xff\xc0\x48\xff\xc8\
\xeb\xfb\xe9\x00\x00\x00\x00\xe9\xfb\xff\xff\xff";
    assert!(&output == expected);
}


#[test]
fn test_global_jumps() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);

    dynasm!(ops
        ; .arch x64
        ; jmp BYTE ->minusone
        ; jmp BYTE ->plustwo
        ; jmp BYTE ->minusone
        ;->start:
        ; inc rax
        ;->plusone:
        ; inc rbx
        ;->plustwo:
        ; jmp BYTE ->end
        ; jmp BYTE ->start
        ;->minustwo:
        ; inc rcx
        ;->minusone:
        ; inc rdx
        ;->end:
        ; jmp BYTE ->plusone
        ; jmp BYTE ->minustwo
        ; jmp BYTE ->plusone
        );

    let output = ops.finalize().unwrap();

    for i in &output {
        print!("\\x{:02x}", i);
    }
    println!("");

    let expected: &[u8] = b"\
\xEB\x11\xEB\x08\xEB\x0D\x48\xFF\xC0\x48\xFF\xC3\xEB\x08\xEB\xF6\
\x48\xFF\xC1\x48\xFF\xC2\xEB\xF1\xEB\xF6\xEB\xED";
    assert!(&output == expected);
}


#[test]
fn test_dynamic_jumps() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);
    let minustwo = ops.new_dynamic_label();
    let minusone = ops.new_dynamic_label();
    let end = ops.new_dynamic_label();
    let start = ops.new_dynamic_label();
    let plusone = ops.new_dynamic_label();
    let plustwo = ops.new_dynamic_label();

    dynasm!(ops
        ; .arch x64
        ; jmp BYTE =>minusone
        ; jmp BYTE =>plustwo
        ; jmp BYTE =>minusone
        ;=>start
        ; inc rax
        ;=>plusone
        ; inc rbx
        ;=>plustwo
        ; jmp BYTE =>end
        ; jmp BYTE =>start
        ;=>minustwo
        ; inc rcx
        ;=>minusone
        ; inc rdx
        ;=>end
        ; jmp BYTE =>plusone
        ; jmp BYTE =>minustwo
        ; jmp BYTE =>plusone
    );

    let output = ops.finalize().unwrap();

    for i in &output {
        print!("\\x{:02x}", i);
    }
    println!("");

    let expected: &[u8] = b"\
\xEB\x11\xEB\x08\xEB\x0D\x48\xFF\xC0\x48\xFF\xC3\xEB\x08\xEB\xF6\
\x48\xFF\xC1\x48\xFF\xC2\xEB\xF1\xEB\xF6\xEB\xED";
    assert!(&output == expected);
}


#[test]
fn test_all_jumps() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);

    let label = ops.new_dynamic_label();

    // please never do this
    dynasm!(ops
        ; .arch x64
        ; jmp >label
        ; inc rax
        ; jmp ->label
        ; inc rbx
        ; jmp =>label
        ; inc rcx
        ; label:
        ; inc rdx
        ;->label:
        ; inc rbp
        ;=>label
        ; inc rsp
        ; jmp <label
        ; inc rsi
        ; jmp ->label
        ; inc rdi
        ; jmp =>label
    );

    let output = ops.finalize().unwrap();

    for i in &output {
        print!("\\x{:02x}", i);
    }
    println!("");

    let expected: &[u8] = b"\
\xe9\x13\x00\x00\x00\x48\xff\xc0\xe9\x0e\x00\x00\x00\x48\xff\xc3\
\xe9\x09\x00\x00\x00\x48\xff\xc1\x48\xff\xc2\x48\xff\xc5\x48\xff\
\xc4\xe9\xf2\xff\xff\xff\x48\xff\xc6\xe9\xed\xff\xff\xff\x48\xff\
\xc7\xe9\xe8\xff\xff\xff";
    assert!(&output == expected);
}


#[test]
fn test_bad_jumps() {
    // forward jump to a backwards label
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);
    dynasm!(ops
        ; .arch x64
        ; backwards:
        ; jmp >backwards
        ; forwards:
    );
    assert!(ops.finalize() == Err(DynasmError::UnknownLabel(LabelKind::Local("backwards"))));

    // backwards jump to a forward label
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);
    dynasm!(ops
        ; .arch x64
        ; backwards:
        ; jmp <forwards
        ; forwards:
    );
    assert!(ops.finalize() == Err(DynasmError::UnknownLabel(LabelKind::Local("forwards"))));

    // local jump to global labels
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);
    dynasm!(ops
        ; .arch x64
        ;->backwards:
        ; jmp <backwards
        ;->forwards:
    );
    assert!(ops.finalize() == Err(DynasmError::UnknownLabel(LabelKind::Local("backwards"))));

    // local jump to global labels
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);
    dynasm!(ops
        ; .arch x64
        ;->backwards:
        ; jmp >forwards
        ;->forwards:
    );
    assert!(ops.finalize() == Err(DynasmError::UnknownLabel(LabelKind::Local("forwards"))));

    // global jump to local labels
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);
    dynasm!(ops
        ; .arch x64
        ;backwards:
        ; jmp ->forwards
        ;forwards:
    );
    assert!(ops.finalize() == Err(DynasmError::UnknownLabel(LabelKind::Global("forwards"))));

    // global jump to local labels
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);
    dynasm!(ops
        ; .arch x64
        ;backwards:
        ; jmp ->backwards
        ;forwards:
    );
    assert!(ops.finalize() == Err(DynasmError::UnknownLabel(LabelKind::Global("backwards"))));

    // unknown dynamic label
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x64::X64Relocation>::new(0);
    let label = ops.new_dynamic_label();
    dynasm!(ops
        ; .arch x64
        ; jmp =>label
    );
    match ops.finalize() {
        Err(DynasmError::UnknownLabel(LabelKind::Dynamic(_))) => (),
        _ => assert!(false)
    }
}
