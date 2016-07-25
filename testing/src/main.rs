#![feature(plugin)]
#![plugin(dynasm)]
extern crate dynasmrt;

macro_rules! test {
    () => (mov rax, rbx)
}

fn main() {
    let mut ops = dynasmrt::AssemblingBuffer::new();
    let d = 3;
    let c = 4;

    // interesting testcases
    dynasm!(ops
        // no args
        ; ret
        // immediate
        ; ret 16
        // register
        ; inc rax
        // memory ref
        ; inc DWORD [16]
        ; inc DWORD [rax]
        ; inc DWORD [rax*2]
        ; inc DWORD [rax*3]
        ; inc DWORD [rax*4]
        ; inc DWORD [rax*5]
        ; inc DWORD [rax*8]
        ; inc DWORD [rax*9]
        ; inc DWORD [rax + 16]
        ; inc DWORD [rax*8 + 16]
        ; inc DWORD [rax + rbx]
        ; inc DWORD [rax + rbx + 16]
        ; inc DWORD [rax*8 + rbx + 16]
        // special memoryref cases
        ; inc DWORD [rsp]
        // ; inc DWORD [rsp + rax] // fixme: rsp can be used as base. just not as index
        // ; inc DWORD [rax + rsp] // fixme: rsp can be used as base. just not as index
        ; inc DWORD [rbp]
        ; inc DWORD [rbp + 16]
        ; inc DWORD [rbp*8]
        ; inc DWORD [rip]
        ; inc DWORD [rip + 16]
        // multi arg forms
        ; mov rax, rbx
        ; mov rax, [rbx]
        ; mov [rbx], rax
        ; mov rax, 1
        ; mov [rax], BYTE 1
        ; imul rax, rbx, 1
        ; imul rax, [rbx], 1
        // prefixes
        ; fs inc DWORD [rax]
        ; lock fs inc DWORD [rax]
        ; rep stosq
        // really long instructions
        ; fs imul r9w, [r10w*8 + r11w + 0x66778899], 0x1122
        ; fs imul r9,  [r10w*8 + r11w + 0x66778899], 0x11223344
        ; fs mov r9, QWORD 0x1122334455667788 // I'm actually not sure if it's valid to use extended registers with instructions that encode the register in the opcode byte
        ; fs movabs rax, 0x1122334455667788
        // funky syntax features
        ; inc BYTE [rax]
        ; inc WORD [rax]
        ; inc DWORD [rax]
        ; inc QWORD [rax]
        // very odd memoryrefs
        ; mov rax, [rbx + rbx * 3 + 2 + c + rax + d]
        // labels
        ; a: // local
        ; -> b: // global
        ; => 1 // dynamic
        // jumps
        ; jmp <a
        ; jmp -> b
        ; jmp => 1
        // dynamic registers
        ; inc Rb(1)
        ; inc Rw(1)
        ; inc Rd(1)
        ; inc Rq(1)
        ; mov Rb(7), [Rb(3)*4 + rax]
    );

    ops.encode_relocs();

    println!("Generated assembly:");
    for i in ops.iter() {
        print!("{:02x }", i);
    }
    println!("");
}
