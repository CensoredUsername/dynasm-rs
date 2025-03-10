use super::riscvdata::{Command, Matcher, Relocation, Opdata, ISAFlags};
use super::RiscVTarget;

use std::fmt::Write;

#[cfg(feature = "dynasm_opmap")]
pub fn create_opmap() -> String {
    let mut s = String::new();

    let mut mnemonics: Vec<_> = super::riscvdata::mnemonics().cloned().collect();
    mnemonics.sort();
    for mnemonic in mnemonics {
        // get the data for this mnemonic
        let data = super::riscvdata::get_mnemonic_data(mnemonic).unwrap();
        let formats = data.into_iter()
            .map(|x| format!("{} {}", format_opdata(mnemonic, x), format_features(x)))
            .map(|x| x.replace(">>> ", ""))
            .collect::<Vec<_>>();

        // push mnemonic name as title
        write!(s, "### {}\n```insref\n{}\n```\n", mnemonic, formats.join("\n")).unwrap();
    }
    s
}


#[cfg(feature = "dynasm_extract")]
pub fn extract_opmap() -> String {
    let mut buf = Vec::new();

    let mut mnemonics: Vec<_> = super::riscvdata::mnemonics().cloned().collect();
    mnemonics.sort();

    for mnemonic in mnemonics {
        // get the data for this mnemonic
        let data = super::riscvdata::get_mnemonic_data(mnemonic).unwrap();

        buf.extend(
            data.into_iter()
            .map(|x| extract_opdata(mnemonic, x))
        );
    }

    buf.join("\n")
}


pub fn format_opdata_list(name: &str, data: &[Opdata], target: RiscVTarget) -> String {
    let mut forms = Vec::new();

    for data in data {
        if target.is_64_bit() && !data.isa_flags.contains(ISAFlags::RV64) {
            continue
        } else if target.is_32_bit() && !data.isa_flags.contains(ISAFlags::RV32) {
            continue
        }

        forms.push(format!("{} {}", format_opdata(name, data), format_features(data)));
    }

    forms.join("\n")
}


pub fn format_opdata(name: &str, data: &Opdata) -> String {
    let mut buf = format!(">>> {}", name);

    let (constraints, names) = match constraints_and_names(data) {
        Ok(o) => o,
        Err(e) => panic!("Encountered a faulty op listing for {}: {}", name, e)
    };

    let mut first = true;
    let mut names = &names[..];

    for matcher in data.matchers {
        if first {
            buf.push(' ');
            first = false;
        } else {
            buf.push_str(", ");
        }

        let (arg_names, rest) = names.split_at(match matcher {
            Matcher::RefOffset
            | Matcher::RefLabel => 2,
            Matcher::Lit(_)
            | Matcher::Reg(_) => 0,
            _ => 1
        });
        names = rest;

        match matcher {
            Matcher::X => write!(buf, "r{}", arg_names[0]).unwrap(),
            Matcher::F => write!(buf, "f{}", arg_names[0]).unwrap(),
            Matcher::Reg(regid) => write!(buf, "{}", regid.to_string()).unwrap(),
            Matcher::Xlist => write!(buf, "{{reg_list}}").unwrap(),
            Matcher::Ref => write!(buf, "[r{}]", arg_names[0]).unwrap(),
            Matcher::RefOffset => write!(buf, "[r{}, {}]", arg_names[0], arg_names[1]).unwrap(),
            Matcher::RefSp => write!(buf, "[sp, {}]", arg_names[0]).unwrap(),
            Matcher::RefLabel => write!(buf, "[r{}, {}]", arg_names[0], arg_names[1]).unwrap(),
            Matcher::Offset => buf.push_str(&arg_names[0]),
            Matcher::Imm => buf.push_str(&arg_names[0]),
            Matcher::Ident => buf.push_str(&arg_names[0]),
            Matcher::Lit(literal) => buf.push_str(literal.as_str()),
        }
    }

    if let Some(c) = constraints {
        let mut len = c.len() + buf.len();
        while len < 100 {
            buf.push(' ');
            len += 1;
        }
        buf.push_str(&c);
    } else {
        let mut len = buf.len();
        while len < 100 {
            buf.push(' ');
            len += 1;
        }
    }

    buf
}

fn constraints_and_names(opdata: &Opdata) -> Result<(Option<String>, Vec<String>), &'static str> {
    let data = group_opdata(opdata)?;
    let constraints = format_constraints(&data);
    let names = data.into_iter().map(|a| a.name.unwrap_or_else(|| "?".into())).collect();
    Ok((constraints, names))
}

fn group_opdata(opdata: &Opdata) -> Result<Vec<ArgWithCommands>, &'static str> {
    let args = flatten_matchers(opdata.matchers);
    let (max_cursor, commands) = group_commands(opdata.commands);

    if args.len() != max_cursor {
        return Err("arg / command count mismatch");
    }

    let mut args: Vec<_> = args.into_iter().map(|arg| ArgWithCommands {
        arg,
        commands: Vec::new(),
        name: None
    }).collect();

    for (command, idx) in commands {
        args[idx].commands.push(command);
    }

    // validate the commands - argtypes
    check_command_sanity(&args)?;

    name_args(&mut args);

    Ok(args)
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum FlatArgTy {
    Direct(bool), // boolean paramter is set to true if this one resulted from a memory reference
    Immediate,
    JumpTarget,
    Reglist,
}

struct ArgWithCommands {
    pub arg: FlatArgTy,
    pub commands: Vec<Command>,
    pub name: Option<String>,
}

/// Take a matcher array and return a vector of the types of flat arg each should produce
fn flatten_matchers(matchers: &[Matcher]) -> Vec<FlatArgTy> {
    let mut args = Vec::new();

    for matcher in matchers {
        match matcher {
            Matcher::X
            | Matcher::F
            | Matcher::Ref => args.push(FlatArgTy::Direct(false)),
            Matcher::Ident
            |  Matcher::RefSp
            | Matcher::Imm => args.push(FlatArgTy::Immediate),
            Matcher::RefOffset => {
                args.push(FlatArgTy::Direct(true));
                args.push(FlatArgTy::Immediate);
            },
            Matcher::RefLabel => {
                args.push(FlatArgTy::Direct(true));
                args.push(FlatArgTy::JumpTarget);
            },
            Matcher::Offset => args.push(FlatArgTy::JumpTarget),
            Matcher::Xlist => args.push(FlatArgTy::Reglist),
            Matcher::Lit(_)
            | Matcher::Reg(_) => ()
        }
    }
    args
}

/// Take a commands slice and calculate the expected amount of args / a vec of command, argidx
fn group_commands(commands: &[Command]) -> (usize, Vec<(Command, usize)>) {
    let mut cursor = 0;
    let mut command_idx = Vec::new();

    for command in commands {
        match command {
            Command::Next => {
                cursor += 1;
                continue;
            },
            Command::Repeat => {
                cursor -= 1;
                continue;
            },
            _ => ()
        }

        command_idx.push((command.clone(), cursor));
        match command {
            Command::R(_)
            | Command::Reven(_)
            | Command::Rno0(_)
            | Command::Rno02(_)
            | Command::Rpop(_)
            | Command::Rpops(_)
            | Command::Rpops2(_)
            | Command::Rlist(_)
            | Command::RoundingMode(_)
            | Command::FenceSpec(_)
            | Command::Csr(_)
            | Command::FloatingPointImmediate(_)
            | Command::SPImm(_, _)
            | Command::Offset(_) => cursor += 1,
            _ => ()
        }
    }

    (cursor, command_idx)
}

/// checks if the commands for each arg type make sense
fn check_command_sanity(args: &[ArgWithCommands]) -> Result<(), &'static str> {
    for arg in args {
        if arg.commands.is_empty() {
            return Err("Arg with no commands")
        }

        for command in &arg.commands {
            let check = match command {
                Command::R(_)
                | Command::Reven(_)
                | Command::Rno0(_)
                | Command::Rno02(_)
                | Command::Rpop(_)
                | Command::Rpops(_)
                | Command::Rpops2(_) => arg.arg == FlatArgTy::Direct(false) || arg.arg == FlatArgTy::Direct(true),
                Command::Rlist(_) => arg.arg == FlatArgTy::Reglist,
                Command::RoundingMode(_)
                | Command::FenceSpec(_)
                | Command::Csr(_)
                | Command::FloatingPointImmediate(_)
                | Command::SPImm(_, _)
                | Command::UImm(_, _)
                | Command::SImm(_, _)
                | Command::BigImm(_)
                | Command::UImmNo0(_, _)
                | Command::SImmNo0(_, _)
                | Command::UImmOdd(_, _)
                | Command::UImmRange(_, _)
                | Command::BitRange(_, _, _)
                | Command::RBitRange(_, _, _) => arg.arg == FlatArgTy::Immediate,
                Command::Offset(_) => arg.arg == FlatArgTy::JumpTarget,
                Command::Repeat
                | Command::Next => unreachable!()
            };

            if !check {
                return Err("command / argtype mismatch");
            }
        }
    }

    Ok(())
}

/// assign names to the args being used
fn name_args(args: &mut [ArgWithCommands]) {
    // iirc no op uses more than 4 unconstrained literals / immediates
    let reg_name_list;
    let mut reg_name_idx = 0;
    let imm_name_list = ["", "1", "2", "3"];
    let mut imm_name_idx = 0;

    // if this is a memory instruction
    if args.iter().any(|arg| arg.arg == FlatArgTy::Direct(true)) &&
       args.iter().filter(|arg| arg.arg == FlatArgTy::Direct(false)).count() == 1 {
        reg_name_list = ["v", "v", "v", "v"];
    } else {
        reg_name_list = ["d", "s1", "s2", "s3"];
    }

    for arg in args {
        match arg.arg {
            FlatArgTy::Direct(is_offset) => match &arg.commands[0] {
                Command::R(_)
                | Command::Reven(_)
                | Command::Rno0(_)
                | Command::Rno02(_)
                | Command::Rpop(_)
                | Command::Rpops(_)
                | Command::Rpops2(_) => if is_offset {
                    arg.name = Some("b".to_string())
                } else {
                    arg.name = Some(reg_name_list[reg_name_idx].to_string());
                    reg_name_idx += 1;
                },
                _ => unreachable!()
            },
            FlatArgTy::Reglist => match &arg.commands[0] {
                Command::Rlist(_) => arg.name = None,
                _ => unreachable!()
            }
            FlatArgTy::JumpTarget => match &arg.commands[0] {
                Command::Offset(_) => arg.name = Some("<offset>".to_string()),
                _ => unreachable!()
            },
            FlatArgTy::Immediate => match &arg.commands[0] {
                Command::RoundingMode(_) => arg.name = Some("<rounding mode>".to_string()),
                Command::FenceSpec(_) => arg.name = Some("<fence spec>".to_string()),
                Command::Csr(_) => arg.name = Some("<csr>".to_string()),
                Command::FloatingPointImmediate(_) => {
                    arg.name = Some(format!("fimm{}", imm_name_list[imm_name_idx]));
                    imm_name_idx += 1;
                },
                Command::SPImm(_, true) => arg.name = Some("nspimm".to_string()),
                Command::SPImm(_, false) => arg.name = Some("spimm".to_string()),
                Command::UImm(_, _)
                | Command::UImmNo0(_, _)
                | Command::UImmOdd(_, _)
                | Command::UImmRange(_, _) => {
                    arg.name = Some(format!("uimm{}", imm_name_list[imm_name_idx]));
                    imm_name_idx += 1;
                },
                Command::SImm(_, _)
                | Command::BigImm(_)
                | Command::SImmNo0(_, _) => {
                    arg.name = Some(format!("simm{}", imm_name_list[imm_name_idx]));
                    imm_name_idx += 1;
                },
                _ => unreachable!()
            }
        }
    }
}

fn format_constraints(args: &[ArgWithCommands]) -> Option<String> {
    let mut constraints = String::new();

    let mut prev_name = "?";

    for arg in args {
        if let Some(ref name) = arg.name {
            emit_constraints(name, prev_name, &arg.commands, &mut constraints);
            prev_name = name;
        }
    }

    if constraints.is_empty() {
        None
    } else {
        let len = constraints.len();
        Some(format!(" ({})", &constraints[0 .. len - 2]))
    }
}

fn emit_constraints(name: &str, prev_name: &str, commands: &[Command], buf: &mut String) {
    for command in commands {
        match command {
            Command::Reven(_) => write!(buf, "{} is even", name),
            Command::Rno0(_) => write!(buf, "{} cannot be 0", name),
            Command::Rno02(_) => write!(buf, "{} cannot be 0 or 2", name),
            Command::Rpop(_) => write!(buf, "{} is 8-15", name),
            Command::Rpops(_) => write!(buf, "{} is 8, 9, or 18-23", name),
            Command::Rpops2(_) => write!(buf, "{} is 8, 9, or 18-23, {} != {}", name, name, prev_name),

            Command::FloatingPointImmediate(_) => write!(buf, "{} is a floating point immediate", name),
            Command::SPImm(_, true) => write!(buf, "{} = -(round_up(reglist_space, 16) + [0|16|32|48])", name),
            Command::SPImm(_, false) => write!(buf, "{} = round_up(reglist_space, 16) + [0|16|32|48]", name),
            Command::UImm(bits, 0) => write!(buf, "{} <= {}", name, (1u32 << bits) - 1),
            Command::UImm(bits, scale) => write!(buf, "{} <= {}, {} = {} * N", name, (1u32 << bits) - (1u32 << scale), name, 1u32 << scale),
            Command::SImm(bits, 0) => write!(buf, "-{} <= {} <= {}", (1u32 << (bits - 1)), name, (1u32 << (bits - 1)) - 1),
            Command::BigImm(bits) => write!(buf, "-{} <= {} <= {}", (1u64 << (bits - 1)), name, (1u64 << (bits - 1)) - 1),
            Command::SImm(bits, scale) => write!(buf, "-{} <= {} <= {}, {} = {} * N", 1u32 << (bits - 1), name, (1u32 << (bits - 1)) - (1u32 << scale), name, 1u32 << scale),

            Command::UImmNo0(bits, 0) => write!(buf, "1 <= {} <= {}", name, (1u32 << bits) - 1),
            Command::UImmNo0(bits, scale) => write!(buf, "1 <= {} <= {}, {} = {} * N", name, (1u32 << bits) - (1u32 << scale), name, 1u32 << scale),
            Command::SImmNo0(bits, 0) => write!(buf, "-{} <= {} <= {}, {} != 0", (1u32 << (bits - 1)), name, (1u32 << (bits - 1)) - 1, name),
            Command::SImmNo0(bits, scale) => write!(buf, "-{} <= {} <= {}, {} != 0, {} = {} * N", 1u32 << (bits - 1), name, (1u32 << (bits - 1)) - (1u32 << scale), name, name, 1u32 << scale),
            Command::UImmOdd(bits, 0) => write!(buf, "{} <= {}", name, (1u32 << bits) - 1),
            Command::UImmOdd(bits, scale) => write!(buf, "{} <= {} <= {}, {} = {} * N + {}", (1u32 << scale) - 1, name, (1u32 << bits) - 1, name, 1u32 << scale, (1u32 << scale) - 1),
            Command::UImmRange(min, max) => write!(buf, "{} <= {} <= {}", min, name, max),

            Command::Offset(Relocation::B) => write!(buf, "offset is 11 bit, 2-byte aligned"),
            Command::Offset(Relocation::J) => write!(buf, "offset is 19 bits, 2-byte aligned"),
            Command::Offset(Relocation::BC) => write!(buf, "offset is 8 bits, 2-byte aligned"),
            Command::Offset(Relocation::JC) => write!(buf, "offset is 11 bits, 2-byte aligned"),
            Command::Offset(Relocation::HI20) => write!(buf, "offset is the 20 highest bits of a 32-bit offset"),
            Command::Offset(Relocation::LO12)
            | Command::Offset(Relocation::LO12S) => write!(buf, "offset is the 20 lowest bits of a 32-bit offset"),
            Command::Offset(Relocation::SPLIT32)
            | Command::Offset(Relocation::SPLIT32S) => write!(buf, "offset is 32 bits"),

            _ => continue
        }.unwrap();

        write!(buf, ", ").unwrap();
        break;
    }
}


pub fn format_features(data: &Opdata) -> String {
    let start = if data.isa_flags.contains(ISAFlags::RV32) && !data.isa_flags.contains(ISAFlags::RV64) {
        "RV32"
    } else if data.isa_flags.contains(ISAFlags::RV64) && !data.isa_flags.contains(ISAFlags::RV32) {
        "RV64"
    } else {
        "RV32/64"
    };

    let mut items = Vec::new();

    for ext_flags in data.ext_flags.iter() {
        let mut item = start.to_string();
        item.push_str(&ext_flags.to_string());
        items.push(item);
    }

    return format!("({})", items.join(" or "))
}



#[cfg(feature = "dynasm_extract")]
pub fn extract_opdata(name: &str, data: &Opdata) -> String {
    let mut buf = format!("\"{}", name);

    let mut first = true;
    let mut arg_idx = 0;

    let grouped = group_opdata(data).unwrap();
    let constraints = extract_constraints(&grouped);

    for matcher in data.matchers {
        if first {
            buf.push(' ');
            first = false;
        } else {
            buf.push_str(", ");
        }

        match matcher {
            Matcher::X => write!(buf, "<X,{}>", arg_idx).unwrap(),
            Matcher::F => write!(buf, "<F,{}>", arg_idx).unwrap(),
            Matcher::Reg(regid) => write!(buf, "{}", regid.to_string()).unwrap(),
            Matcher::Ref => write!(buf, "[<X,{}>]", arg_idx).unwrap(),
            Matcher::RefOffset => write!(buf, "[<X,{}>, <Imm,{}>]", arg_idx, arg_idx + 1).unwrap(),
            Matcher::RefSp => write!(buf, "[sp, <Imm,{}>]", arg_idx).unwrap(),
            Matcher::RefLabel => write!(buf, "[<X,{}>, <Off,{}>]", arg_idx, arg_idx + 1).unwrap(),
            Matcher::Imm => write!(buf, "<Imm,{}>", arg_idx).unwrap(),
            Matcher::Offset => write!(buf, "<Off,{}>", arg_idx).unwrap(),
            Matcher::Ident => write!(buf, "<Ident,{}>", arg_idx).unwrap(),
            Matcher::Xlist => write!(buf, "<RegList,{}>", arg_idx).unwrap(),
            Matcher::Lit(literal) => write!(buf, "{}", literal.as_str()).unwrap(),
        }

        arg_idx += match matcher {
            Matcher::RefOffset
            | Matcher::RefLabel => 2,
            Matcher::Lit(_)
            | Matcher::Reg(_) => 0,
            _ => 1
        };
    }

    write!(buf, "\"\t{{{}}}\t", constraints.join(", ")).unwrap();

    buf.push_str(&extract_arch_flags(data));

    buf
}


#[cfg(feature = "dynasm_extract")]
fn extract_arch_flags(data: &Opdata) -> String {
    let mut isa_entries = Vec::new();

    if data.isa_flags.contains(ISAFlags::RV32) {
        isa_entries.push("\"rv32\"")
    }
    if data.isa_flags.contains(ISAFlags::RV64) {
        isa_entries.push("\"rv64\"")
    }

    let mut ext_sets = Vec::new();

    for ext_flags in data.ext_flags.iter() {
        let mut ext_flags = ext_flags.to_string();
        ext_flags.make_ascii_lowercase();

        ext_sets.push(format!("\"{}\"", ext_flags));
    }

    format!("[{}]\t[{}]", isa_entries.join(", "), ext_sets.join(", "))
}


#[cfg(feature = "dynasm_extract")]
fn extract_constraints(args: &[ArgWithCommands]) -> Vec<String> {
    let mut constraints = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        for command in &arg.commands {
            let constraint = match command {
                Command::R(_) => format!("R(0xFFFFFFFF)"),
                Command::Reven(_) => format!("R(0x55555555)"),
                Command::Rno0(_) => format!("R(0xFFFFFFFE)"),
                Command::Rno02(_) => format!("R(0xFFFFFFFA)"),
                Command::Rpop(_) => format!("R(0x0000FF00)"),
                Command::Rpops(_) => format!("R(0x00FC0300)"),
                Command::Rpops2(_) => format!("Rdifferent(0x00FC0300)"),
                Command::Rlist(_) => format!("RList()"),

                Command::RoundingMode(_) => format!("RoundingMode()"),
                Command::FenceSpec(_) => format!("FenceSpec()"),
                Command::Csr(_) => format!("Csr()"),
                Command::FloatingPointImmediate(_) => format!("FloatingPointImmediate()"),
                Command::SPImm(_, true) => format!("StackAdjustImmediate(True)"),
                Command::SPImm(_, false) => format!("StackAdjustImmediate(False)"),

                Command::UImm(bits, scale) => format!("Range(0, {}, {})", 1u32 << bits, 1u32 << scale),
                Command::SImm(bits, scale) => format!("Range(-{}, {}, {})", 1u32 << (bits - 1), 1u32 << (bits - 1), 1u32 << scale),
                Command::BigImm(bits) => format!("Range(-{}, {}, 1)", 1u64 << (bits - 1), 1u64 << (bits - 1)),
                Command::UImmNo0(bits, scale) => format!("RangeNon0(0, {}, {})", 1u32 << bits, 1u32 << scale),
                Command::SImmNo0(bits, scale) => format!("RangeNon0(-{}, {}, {})", 1u32 << (bits - 1), 1u32 << (bits - 1), 1u32 << scale),
                Command::UImmOdd(bits, scale) => format!("Range({}, {}, {})", (1u32 << scale) - 1, 1u32 << bits, 1u32 << scale),
                Command::UImmRange(min, max) => format!("Range({}, {}, 1)", min, max + 1),

                Command::Offset(Relocation::B) => format!("Range(-{}, {}, {})", 1u32 << 11, 1u32 << 11, 2),
                Command::Offset(Relocation::J) => format!("Range(-{}, {}, {})", 1u32 << 19, 1u32 << 19, 2),
                Command::Offset(Relocation::BC) => format!("Range(-{}, {}, {})", 1u32 << 8, 1u32 << 8, 2),
                Command::Offset(Relocation::JC) => format!("Range(-{}, {}, {})", 1u32 << 11, 1u32 << 11, 2),
                Command::Offset(Relocation::HI20) => format!("Range(-{}, {}, {})", 1u32 << 31, 1u32 << 31, 1 << 12),
                Command::Offset(Relocation::LO12)
                | Command::Offset(Relocation::LO12S) => format!("Range(-{}, {}, 1)", 1u32 << 11, 1u32 << 11),
                Command::Offset(Relocation::SPLIT32)
                | Command::Offset(Relocation::SPLIT32S) => format!("Range(-{}, {}, 1)", 1u32 << 31, 1u32 << 31),

                _ => continue
            };
            constraints.push(format!("{}: {}", i, constraint));

            break;
        }
    }
    constraints
}
