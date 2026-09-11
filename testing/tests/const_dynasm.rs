#![cfg(not(feature = "runtime_computations"))]
#![allow(dead_code)]

#[derive(Copy, Clone)]
struct ConstAssembler<const SIZE: usize> {
    buffer: [u8; SIZE],
    bytes: usize,
}

impl<const SIZE: usize> ConstAssembler<SIZE> {
    pub const fn new() -> Self {
        Self {
            buffer: [0; SIZE],
            bytes: 0,
        }
    }

    pub const fn extend(&mut self, buffer: &[u8]) {
        let mut i = 0;
        while i < buffer.len() {
            self.buffer[self.bytes] = buffer[i];
            self.bytes += 1;
            i += 1;
        }
    }

    pub const fn offset(&self) -> usize {
        self.bytes
    }

    pub const fn push(&mut self, byte: u8) {
        self.buffer[self.bytes] = byte;
        self.bytes += 1;
    }

    pub const fn align(&mut self, alignment: usize, with: u8) {
        let mut to_add = self.bytes - (self.bytes % alignment);
        while to_add != 0 {
            self.buffer[self.bytes] = with;
            self.bytes += 1;
            to_add -= 1;
        }
    }

    pub const fn push_i8(&mut self, value: i8) {
        self.push(value as u8);
    }

    pub const fn push_i16(&mut self, value: i16) {
        self.extend(&i16::to_le_bytes(value));
    }

    pub const fn push_i32(&mut self, value: i32) {
        self.extend(&i32::to_le_bytes(value));
    }

    pub const fn push_i64(&mut self, value: i64) {
        self.extend(&i64::to_le_bytes(value));
    }

    pub const fn push_u16(&mut self, value: u16) {
        self.extend(&u16::to_le_bytes(value));
    }

    pub const fn push_u32(&mut self, value: u32) {
        self.extend(&u32::to_le_bytes(value));
    }

    pub const fn push_u64(&mut self, value: u64) {
        self.extend(&u64::to_le_bytes(value));
    }

    pub const fn runtime_error(&self, msg: &'static str) {
        panic!("{}", msg);
    }

    pub const fn local_label(&mut self, _name: &'static str) {}
    pub const fn forward_reloc(
        &mut self,
        _name: &'static str,
        _target_offset: isize,
        _field_offset: u8,
        _ref_offset: u8,
        _kind: u8,
    ) {
    }
    pub const fn backward_reloc(
        &mut self,
        _name: &'static str,
        _target_offset: isize,
        _field_offset: u8,
        _ref_offset: u8,
        _kind: u8,
    ) {
    }
}

#[test]
fn x64_const_dynasm() {
    const {
        let mut out = ConstAssembler::<64>::new();
        let reg = 1;
        let imm = 3;
        dynasmrt::dynasm!(out
            ; .arch x64

            ;   jmp >start
            ; start:
            ;   syscall
            ;   add rax, Rq(1)
            ;   add Rq(reg), [ BYTE 32 + Rq(reg) + Rq(reg) * 4 ]
            ;   add rax, BYTE imm
            ;   ja <start
        );
    };
}

#[test]
fn riscv_const_dynasm() {
    const {
        let mut out = ConstAssembler::<64>::new();
        let reg = 1;
        dynasmrt::dynasm!(out
            ; .arch riscv64

            ;   j >start
            ; start:
            ;   ecall
            ;   add x10, x10, X(1)
            ;   add X(reg), X(reg), X(reg)
            ;   bgtu x10, x11, <start
        );
    };
}

#[test]
fn aarch64_const_dynasm() {
    const {
        let mut out = ConstAssembler::<64>::new();
        let reg = 1;
        dynasmrt::dynasm!(out
            ; .arch aarch64

            ;   b >start
            ; start:
            ;   svc #0
            ;   add x0, x0, X(1)
            ;   add X(reg), X(reg), X(reg), lsl #2
            ;   b.hi <start
        );
    };
}
