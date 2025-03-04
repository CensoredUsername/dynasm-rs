use super::riscvdata::{Command, Matcher, Relocation, Opdata, ISAFlags};
use super::RiscVTarget;

use std::fmt::Write;

#[cfg(feature = "dynasm_opmap")]
pub fn create_opmap() -> String {
    let mut s = String::new();

    let mut mnemonics: Vec<_> = riscvdata::mnemonics().cloned().collect();
    mnemonics.sort();
    for mnemnonic in mnemonics {
        // get the data for this mnemnonic
        let data = riscvdata::get_mnemonic_data(mnemonic).unwrap();
        let formats = data.into_iter()
            .map(|x| format_opdata(mnemnonic, x))
            .flat_map(|x| x)
            .map(|x| x.replace(">>> ", ""))
            .collect::<Vec<_>>();

        // push mnemnonic name as title
        write!(s, "### {}\n```insref\n{}\n```\n", mnemnonic, formats.join("\n")).unwrap();
    }
    s
}


#[cfg(feature = "dynasm_extract")]
pub fn extract_opmap() -> String {
    let mut buf = Vec::new();

    let mut mnemnonics: Vec<_> = super::aarch64data::mnemnonics().cloned().collect();
    mnemnonics.sort();

    for mnemnonic in mnemnonics {
        // get the data for this mnemnonic
        let data = super::aarch64data::get_mnemonic_data(mnemnonic).unwrap();

        buf.extend(
            data.into_iter()
            .map(|x| extract_opdata(mnemnonic, x))
            .flat_map(|x| x)
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

        forms.push(format_opdata(name, data));
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
            Matcher::RefOffset => 2,
            Matcher::Xlist => 0,
            _ => 1
        });
        names = rest;

        match matcher {
            Matcher::X => write!(buf, "r{}", arg_names[0]).unwrap(),
            Matcher::F => write!(buf, "f{}", arg_names[0]).unwrap(),
            Matcher::Xlist => write!(buf, "{{ra, [s0-s11]}}").unwrap(),
            Matcher::Ref => write!(buf, "(x{})", arg_names[0]).unwrap(),
            Matcher::RefOffset => write!(buf, "{}(x{})", arg_names[1], arg_names[0]).unwrap(),
            Matcher::Offset => buf.push_str(&arg_names[0]),
            Matcher::Imm => buf.push_str(&arg_names[0]),
            Matcher::Ident => buf.push_str(&arg_names[0]),
        }
    }

    if let Some(c) = constraints {
        let mut len = c.len() + buf.len();
        while len < 100 {
            buf.push(' ');
            len += 1;
        }
        buf.push_str(&c);
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
    Direct,
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
            | Matcher::Ref => args.push(FlatArgTy::Direct),
            Matcher::Ident
            | Matcher::Imm => args.push(FlatArgTy::Immediate),
            Matcher::RefOffset => {
                args.push(FlatArgTy::Direct);
                args.push(FlatArgTy::Immediate);
            },
            Matcher::Offset => args.push(FlatArgTy::JumpTarget),
            Matcher::Xlist => args.push(FlatArgTy::Reglist),
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
            | Command::Rlist(_)
            | Command::RoundingMode(_)
            | Command::FenceSpec(_)
            | Command::Csr(_)
            | Command::FloatingPointImmediate(_)
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
                | Command::Rpops(_) => arg.arg == FlatArgTy::Direct,
                Command::Rlist(_) => arg.arg == FlatArgTy::Reglist,
                Command::RoundingMode(_)
                | Command::FenceSpec(_)
                | Command::Csr(_)
                | Command::FloatingPointImmediate(_)
                | Command::UImm(_, _)
                | Command::SImm(_, _)
                | Command::NImm(_, _)
                | Command::UImmNo0(_, _)
                | Command::SImmNo0(_, _)
                | Command::UImmOdd(_, _)
                | Command::UImmRange(_, _)
                | Command::BitRange(_, _, _)
                | Command::NBitRange(_, _, _) => arg.arg == FlatArgTy::Immediate,
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
    let reg_name_list = ["d", "s1", "s2", "s3"];
    let mut reg_name_idx = 0;
    let imm_name_list = ["", "1", "2", "3"];
    let mut imm_name_idx = 0;

    for arg in args {
        match arg.arg {
            FlatArgTy::Direct => match &arg.commands[0] {
                Command::R(_)
                | Command::Reven(_)
                | Command::Rno0(_)
                | Command::Rno02(_)
                | Command::Rpop(_)
                | Command::Rpops(_) => {
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
                Command::UImm(_, _)
                | Command::UImmNo0(_, _)
                | Command::UImmOdd(_, _)
                | Command::UImmRange(_, _) => {
                    arg.name = Some(format!("uimm{}", imm_name_list[imm_name_idx]));
                    imm_name_idx += 1;
                },
                Command::SImm(_, _)
                | Command::SImmNo0(_, _) => {
                    arg.name = Some(format!("simm{}", imm_name_list[imm_name_idx]));
                    imm_name_idx += 1;
                },
                Command::NImm(_, _) => {
                    arg.name = Some(format!("nimm{}", imm_name_list[imm_name_idx]));
                    imm_name_idx += 1;
                },
                _ => unreachable!()
            }
        }
    }
}

fn format_constraints(args: &[ArgWithCommands]) -> Option<String> {
    let mut constraints = String::new();

    for arg in args {
        if let Some(ref name) = arg.name {
            emit_constraints(name, &arg.commands, &mut constraints);
        }
    }

    if constraints.is_empty() {
        None
    } else {
        let len = constraints.len();
        Some(format!(" ({})", &constraints[0 .. len - 2]))
    }
}

fn emit_constraints(name: &str, commands: &[Command], buf: &mut String) {
    for command in commands {
        match command {
            Command::Reven(_) => write!(buf, "{} is even", name),
            Command::Rno0(_) => write!(buf, "{} cannot be 0", name),
            Command::Rno02(_) => write!(buf, "{} cannot be 0 or 2", name),
            Command::Rpop(_) => write!(buf, "{} is 8-15", name),
            Command::Rpops(_) => write!(buf, "{} is 8, 9, or 18-23)", name),

            Command::FloatingPointImmediate(_) => write!(buf, "{} is a floating point immediate", name),
            Command::UImm(bits, 0) => write!(buf, "{} <= {}", name, (1u32 << bits) - 1),
            Command::UImm(bits, scale) => write!(buf, "{} <= {}, {} = {} * N", name, (1u32 << bits) - (1u32 << scale), name, 1u32 << scale),
            Command::SImm(bits, 0) => write!(buf, "-{} <= {} <= {}", (1u32 << (bits - 1)), name, (1u32 << (bits - 1)) - 1),
            Command::SImm(bits, scale) => write!(buf, "-{} <= {} <= {}, {} = {} * N", 1u32 << (bits - 1), name, (1u32 << (bits - 1)) - (1u32 << scale), name, 1u32 << scale),
            Command::NImm(bits, 0) => write!(buf, "-{} <= {} <= 0", (1u32 << bits) - 1, name),
            Command::NImm(bits, scale) => write!(buf, "-{} <= {} <= 0, {} = -{} * N", (1u32 << bits) - (1u32 << scale), name, name, 1u32 << scale),
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
            Command::Offset(Relocation::AUIPC) => write!(buf, "offset is 20 bits, 4K-page aligned"),
            Command::Offset(Relocation::JALR) => write!(buf, "offset is 12 bits"),

            _ => continue
        }.unwrap();

        write!(buf, ", ").unwrap();
        break;
    }
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
            Matcher::Ref => write!(buf, "<Imm,{}>(<XSP,{}>)", arg_idx + 1, arg_idx).unwrap(),
            Matcher::RefOffset => write!(buf, "(<XSP,{}>)", arg_idx).unwrap(),
            Matcher::Imm => write!(buf, "<Imm,{}>", arg_idx).unwrap(),
            Matcher::Offset => write!(buf, "<Off,{}>", arg_idx).unwrap(),
            Matcher::Ident => write!(buf, "<Ident,{}>", arg_idx).unwrap(),
            Matcher::Xlist => write!(buf, "<RegList,{}>", arg_idx).unwrap(),
        }

        arg_idx += match matcher {
            Matcher::RefOffset => 2,
            _ => 1
        };
    }

    write!(buf, "\"\t {{{}}}", constraints.join(", ")).unwrap();

    buf
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
                Command::Rlist(_) => format!("RList"),

                Command::RoundingMode(_) => format!("RoundingMode"),
                Command::FenceSpec(_) => format!("FenceSpec"),
                Command::Csr(_) => format!("Csr"),
                Command::FloatingPointImmediate(_) => format!("FloatingPointImmediate"),

                Command::UImm(bits, scale) => format!("Range(0, {}, {})", 1u32 << bits, 1u32 << scale),
                Command::SImm(bits, scale) => format!("Range(-{}, {}, {})", 1u32 << (bits - 1), 1u32 << (bits - 1), 1u32 << scale),
                Command::NImm(bits, scale) => format!("Range(-{}, 1, {})", (1u32 << bits) - 1, 1u32 << scale),
                Command::UImmNo0(bits, scale) => format!("Range(1, {}, {})", 1u32 << bits, 1u32 << scale),
                Command::SImmNo0(bits, scale) => format!("RangeNon0(-{}, {}, {})", 1u32 << (bits - 1), 1u32 << (bits - 1), 1u32 << scale),
                Command::UImmOdd(bits, scale) => format!("Range({}, {}, {})", (1u32 << scale) - 1, 1u32 << bits, 1u32 << scale),
                Command::UImmRange(min, max) => format!("Range({}, {}, 1)", min, max + 1),

                Command::Offset(Relocation::B) => format!("Range(-{}, {}, {})", 1u32 << 11, 1u32 << 11, 2),
                Command::Offset(Relocation::J) => format!("Range(-{}, {}, {})", 1u32 << 19, 1u32 << 19, 2),
                Command::Offset(Relocation::BC) => format!("Range(-{}, {}, {})", 1u32 << 8, 1u32 << 8, 2),
                Command::Offset(Relocation::JC) => format!("Range(-{}, {}, {})", 1u32 << 11, 1u32 << 11, 2),
                Command::Offset(Relocation::AUIPC) => format!("Range(-{}, {}, {})", 1u32 << 31, 1u32 << 31, 1<<12),
                Command::Offset(Relocation::JALR) => format!("Range(-{}, {}, {})", 1u32 << 11, 1u32 << 11, 1),

                _ => continue
            };
            constraints.push(format!("{}: {}", i, constraint));

            break;
        }
    }
    constraints
}
