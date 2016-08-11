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
    let opsizes = if data.flags.contains(AUTO_SIZE) {"qwd"}
             else if data.flags.contains(AUTO_NO32) {"qw"}
             else if data.flags.contains(AUTO_REXW) {"qd"}
             else if data.flags.contains(AUTO_VEXL) {"ho"}
             else                                   {"!"};

    let mut forms = Vec::new();
    for opsize in opsizes.chars() {
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

fn format_arg(ty: char, mut size: char, opsize: char) -> String {
    if size == '*' {
        size = if opsize == 'q' && (ty == 'i' || ty == 'o') {
            'd'
        } else {
            opsize
        };
    }

    fn format_size(size: char) -> &'static str {
        match size {
            'b' => "8",
            'w' => "16",
            'd' => "32",
            'q' => "64",
            'p' => "80",
            'o' => "128",
            'h' => "256",
            _ => ""
        }
    }

    match ty {
        'i' => format!("imm{}",      format_size(size)),
        'o' => format!("rel{}off",   format_size(size)),
        'm' => format!("mem{}",      format_size(size)),
        'k' => format!("vm32addr{}", format_size(size)),
        'l' => format!("vm64addr{}", format_size(size)),
        'r' => format!("reg{}",      format_size(size)),
        'f' => format!("st"),
        'x' => format!("mmx"),
        'y' => format!("{}mm", if size == 'h' {"y"} else {"x"}),
        's' => format!("segreg"),
        'c' => format!("creg"),
        'd' => format!("dreg"),
        'v' => format!("reg/mem{}", format_size(size)),
        'u' => format!("mmx/mem{}", format_size(size)),
        'w' => format!("{}mm/mem{}", if size == 'h' {"y"} else {"x"}, format_size(size)),
        'A'...'P' => {
            let i = ty as usize - 'A' as usize;
            match size {
                'b' => if i < 4 { format!("{}l", REGS[i]) }
                  else if i < 8 { format!("{}",  REGS[i]) }
                  else          { format!("{}b", REGS[i]) },
                'w' => if i < 4 { format!("{}x", REGS[i]) }
                  else if i < 8 { format!("{}",  REGS[i]) }
                  else          { format!("{}w", REGS[i]) },
                'd' => if i < 4 { format!("e{}x",REGS[i]) }
                  else if i < 8 { format!("e{}", REGS[i]) }
                  else          { format!("{}d", REGS[i]) },
                'q' => if i < 4 { format!("r{}x",REGS[i]) }
                  else if i < 8 { format!("r{}", REGS[i]) }
                  else          { format!("r{}", REGS[i]) },
                _ => panic!("invalid formatting data")
            }
        },
        'Q'...'V' => SEGREGS[ty as usize - 'Q' as usize].to_string(),
        'W' => "cr8".to_string(),
        'X' => "st0".to_string(),
        _ => panic!("invalid formatting data")
    }
}
