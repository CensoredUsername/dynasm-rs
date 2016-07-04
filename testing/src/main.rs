#![feature(plugin)]
#![plugin(dynasm)]
extern crate dynasmrt;

fn main() {
    let mut ops = dynasmrt::AssemblingBuffer::new();
    let d = 3;
    let c = 4;

    dynasm!(ops
//         ; mov                                                    // no args
//         ; mov rax                                                // invalid args combination
         ; mov DWORD [rax], 1
         ; mov rax, QWORD -1                                        // immediate size override
//         ; movz DWORD [rax], 1                                    // invalid opcode
         ; mov BYTE [rax + rax + rcx], 1                            // odd memory ref
         ; mov BYTE [9*r15], 1                                      // odd scale
         ; fs imul sp, WORD [r8 * 2 + rcx + 0x77], 0x77             // 3 prefixes, opcode, ModRM, SIB, disp and immediate
//         ; mov rax, ecx                                           // arg size confusion
//         ; mov [rip*2], rax                                       // rip cannot be used as index
         ; mov QWORD [rax * 2 + rbx + c + d], 1                     // run time variables
    );

    println!("Generated assembly:");
    for i in ops.iter() {
        print!("{:02x }", i);
    }
    println!("");
}
