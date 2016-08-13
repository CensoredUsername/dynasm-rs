use compiler::{Opdata, FormatStringIterator};
use x64data::flags::*;

pub fn format_opdata_list(name: &str, data: &[Opdata]) -> String {
    let mut forms = Vec::new();
    for data in data {
        forms.extend(format_opdata(name, data));
    }
    return forms.join("\n");
}

pub fn format_opdata(name: &str, data: &Opdata) -> Vec<String> {
    let opsizes = if data.flags.contains(AUTO_SIZE) {&b"qwd"[..]}
             else if data.flags.contains(AUTO_NO32) {&b"qw"[..]}
             else if data.flags.contains(AUTO_REXW) {&b"qd"[..]}
             else if data.flags.contains(AUTO_VEXL) {&b"ho"[..]}
             else                                   {&b"!"[..]};

    let mut forms = Vec::new();
    for opsize in opsizes.iter().cloned() {
        let mut buf = String::new();
        buf.push_str(">>> ");
        buf.push_str(name);
        let mut first = true;
        for (ty, size) in FormatStringIterator::new(data.args) {
            if first {
                buf.push_str(" ");
                first = false;
            } else {
                buf.push_str(", ");
            }
            buf.push_str(&format_arg(ty, size, opsize))
        }
        forms.push(buf);
    }
    forms
}

static REGS: [&'static str; 16] = ["a",  "d",  "c",   "b",   "bp",  "sp",  "si",  "di", 
                                   "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"];
static SEGREGS: [&'static str; 6] = ["es", "cs", "ss", "ds", "fs", "gs"];

fn format_arg(ty: u8, mut size: u8, opsize: u8) -> String {
    if size == b'*' {
        size = if opsize == b'q' && (ty == b'i' || ty == b'o') {
            b'd'
        } else {
            opsize
        };
    }

    fn format_size(size: u8) -> &'static str {
        match size {
            b'b' => "8",
            b'w' => "16",
            b'd' => "32",
            b'q' => "64",
            b'p' => "80",
            b'o' => "128",
            b'h' => "256",
            _ => ""
        }
    }

    match ty {
        b'i' => format!("imm{}",      format_size(size)),
        b'o' => format!("rel{}off",   format_size(size)),
        b'm' => format!("mem{}",      format_size(size)),
        b'k' => format!("vm32addr{}", format_size(size)),
        b'l' => format!("vm64addr{}", format_size(size)),
        b'r' => format!("reg{}",      format_size(size)),
        b'f' => format!("st"),
        b'x' => format!("mmx"),
        b'y' => format!("{}mm", if size == b'h' {"y"} else {"x"}),
        b's' => format!("segreg"),
        b'c' => format!("creg"),
        b'd' => format!("dreg"),
        b'v' => format!("reg/mem{}", format_size(size)),
        b'u' => format!("mmx/mem{}", format_size(size)),
        b'w' => format!("{}mm/mem{}", if size == b'h' {"y"} else {"x"}, format_size(size)),
        b'A'...b'P' => {
            let i = ty as usize - 'A' as usize;
            match size {
                b'b' => if i < 4 { format!("{}l", REGS[i]) }
                   else if i < 8 { format!("{}",  REGS[i]) }
                   else          { format!("{}b", REGS[i]) },
                b'w' => if i < 4 { format!("{}x", REGS[i]) }
                   else if i < 8 { format!("{}",  REGS[i]) }
                   else          { format!("{}w", REGS[i]) },
                b'd' => if i < 4 { format!("e{}x",REGS[i]) }
                   else if i < 8 { format!("e{}", REGS[i]) }
                   else          { format!("{}d", REGS[i]) },
                b'q' => if i < 4 { format!("r{}x",REGS[i]) }
                   else if i < 8 { format!("r{}", REGS[i]) }
                   else          { format!("r{}", REGS[i]) },
                _ => panic!("invalid formatting data")
            }
        },
        b'Q'...b'V' => SEGREGS[ty as usize - 'Q' as usize].to_string(),
        b'W' => "cr8".to_string(),
        b'X' => "st0".to_string(),
        _ => panic!("invalid formatting data")
    }
}
