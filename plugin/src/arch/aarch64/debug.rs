use super::ast::Modifier;
use super::aarch64data::{Opdata, Matcher, Command, Relocation, SpecialComm};
use crate::common::Size;

pub fn format_opdata_list(name: &str, data: &[Opdata]) -> String {
    let mut forms = Vec::new();

    for data in data {
        forms.extend(format_opdata(name, data));
    }

    forms.join("\n")
}

pub fn format_opdata(name: &str, data: &Opdata) -> Vec<String> {

    let has_simd_full_width = data.matchers.iter().any(|m| match m {
        Matcher::V(_) | Matcher::RegList(_, _) => true,
        _ => false
    });

    let form_count = 1 + has_simd_full_width as u8;
    let mut forms = Vec::new();

    for i in 0 .. form_count {
        let mut buf = String::new();
        buf.push_str(">>> ");
        buf.push_str(name);

        let mut end_count = 0;

        let (constraints, names) = match constraints_and_names(data) {
            Ok(o) => o,
            Err(e) => panic!("Encountered a faulty op listing for {}: {}", name, e)
        };
        let mut names = names.into_iter();

        let mut first = true;
        for matcher in data.matchers {
            if let Matcher::End = matcher {
                end_count += 1;
                buf.push_str(" {");
                continue;
            }

            if first {
                buf.push_str(" ");
                first = false;
            } else {
                buf.push_str(", ");
            }

            match matcher {
                Matcher::Dot => buf.push_str("."),
                Matcher::Lit(s) => buf.push_str(s),
                Matcher::LitInt(v) => buf.push_str(&format!("{}", v)),
                Matcher::LitFloat(v) => buf.push_str(&format!("{}", v)),
                Matcher::Ident => {
                    buf.push_str(&names.next().unwrap());
                },
                Matcher::Cond => buf.push_str("<cond>"),
                Matcher::Imm => {
                    buf.push_str("#");
                    buf.push_str(&names.next().unwrap());
                },
                Matcher::W => {
                    buf.push_str("W");
                    buf.push_str(&names.next().unwrap());
                },
                Matcher::X => {
                    buf.push_str("X");
                    buf.push_str(&names.next().unwrap());
                },
                Matcher::WSP => {
                    buf.push_str("W");
                    buf.push_str(&names.next().unwrap());
                    buf.push_str("|WSP");
                },
                Matcher::XSP => {
                    buf.push_str("X");
                    buf.push_str(&names.next().unwrap());
                    buf.push_str("|SP");
                },
                Matcher::B => {
                    buf.push_str("B");
                    buf.push_str(&names.next().unwrap());
                },
                Matcher::H => {
                    buf.push_str("H");
                    buf.push_str(&names.next().unwrap());
                },
                Matcher::S => {
                    buf.push_str("S");
                    buf.push_str(&names.next().unwrap());
                },
                Matcher::D => {
                    buf.push_str("D");
                    buf.push_str(&names.next().unwrap());
                },
                Matcher::Q => {
                    buf.push_str("Q");
                    buf.push_str(&names.next().unwrap());
                },
                Matcher::V(s) => {
                    let width = if i == 0 { 16 } else { 8 };
                    let name = names.next().unwrap();
                    buf.push_str(&format!("V{}.{}{}", name, size_to_string(s), width / s.in_bytes()));
                },
                Matcher::VStatic(s, c) => {
                    let name = names.next().unwrap();
                    buf.push_str(&format!("V{}.{}{}", name, size_to_string(s), c));
                },
                Matcher::VElement(s) => {
                    let name = names.next().unwrap();
                    let imm = names.next().unwrap();
                    buf.push_str(&format!("V{}.{}[#{}]", name, size_to_string(s), imm));
                },
                Matcher::VElementStatic(s, element) => {
                    let name = names.next().unwrap();
                    buf.push_str(&format!("V{}.{}[{}]", name, size_to_string(s), element));
                },
                Matcher::RegList(a, s) => {
                    let width = if i == 0 { 16 } else { 8 };
                    let name = names.next().unwrap();
                    buf.push_str(&format!("{{V{}.{}{} * {}}}", name, size_to_string(s), width / s.in_bytes(), a));
                },
                Matcher::RegListStatic(a, s, c) => {
                    let name = names.next().unwrap();
                    buf.push_str(&format!("{{V{}.{}{} * {}}}", name, size_to_string(s), c, a));
                },
                Matcher::RegListElement(a, s) => {
                    let name = names.next().unwrap();
                    let imm = names.next().unwrap();
                    buf.push_str(&format!("{{V{}.{} * {}}}[#{}]", name, size_to_string(s), a, imm));
                },
                Matcher::Offset => buf.push_str(&names.next().unwrap()),
                Matcher::RefBase => {
                    let name = names.next().unwrap();
                    buf.push_str(&format!("[X{}|SP]", name));
                },
                Matcher::RefOffset => {
                    let name = names.next().unwrap();
                    let imm = names.next().unwrap();
                    buf.push_str(&format!("[X{}|SP, {{ #{} }} ]", name, imm));
                },
                Matcher::RefPre => {
                    let name = names.next().unwrap();
                    let imm = names.next().unwrap();
                    buf.push_str(&format!("[X{}|SP, #{}]!", name, imm));
                },
                Matcher::RefIndex => {
                    let name1 = names.next().unwrap();
                    let name2 = names.next().unwrap();
                    let imm = names.next().unwrap();
                    buf.push_str(&format!("[X{}|SP, W{}|X{} {{ , UXTW|LSL|SXTW|SXTX {{ #{} }} }} ]", name1, name2, name2, imm));
                },
                Matcher::Mod(mods) => {
                    let mut optional = true;
                    let mods: Vec<_> = mods.iter().map(|m| match m {
                        Modifier::LSL => {optional = false; "LSL"},
                        Modifier::LSR => {optional = false; "LSR"},
                        Modifier::ASR => {optional = false; "ASR"},
                        Modifier::ROR => {optional = false; "ROR"},
                        Modifier::SXTX => "SXTX",
                        Modifier::SXTW => "SXTW",
                        Modifier::SXTH => "SXTH",
                        Modifier::SXTB => "SXTB",
                        Modifier::UXTX => "UXTX",
                        Modifier::UXTW => "UXTW",
                        Modifier::UXTH => "UXTH",
                        Modifier::UXTB => "UXTB",
                        Modifier::MSL => {optional = false; "MSL"},
                    }).collect();
                    buf.push_str(&mods.join("|"));

                    let name = names.next().unwrap();
                    if optional {
                        buf.push_str(&format!(" {{ #{} }}", name));
                    } else {
                        buf.push_str(&format!(" #{}", name));
                    }
                },
                Matcher::End => ()
            }

        }

        for _ in 0 .. end_count {
            buf.push_str(" }");
        }

        let buf = buf.replace(" ., ", ".");

        forms.push(buf);
        forms.extend(constraints);
    }

    forms
}

pub fn size_to_string(size: &Size) -> &'static str {
    match size {
        Size::BYTE => "B",
        Size::WORD => "H",
        Size::DWORD => "S",
        Size::QWORD => "D",
        Size::OWORD => "Q",
        _ => unimplemented!()
    }
}

#[cfg(feature = "dynasm_opmap")]
pub fn create_opmap() -> String {
    let mut s = String::new();

    let mut mnemnonics: Vec<_> = super::aarch64data::mnemnonics().cloned().collect();
    mnemnonics.sort();

    for mnemnonic in mnemnonics {
        // get the data for this mnemnonic
        let data = super::aarch64data::get_mnemonic_data(mnemnonic).unwrap();
        // format the data for the opmap docs
        let formats = data.into_iter()
            .map(|x| format_opdata(mnemnonic, x))
            .flat_map(|x| x)
            .map(|x| x.replace(">>> ", ""))
            .collect::<Vec<_>>();

        // push mnemnonic name as title
        s.push_str("### ");
        s.push_str(mnemnonic);
        s.push_str("\n```insref\n");

        // push the formats
        s.push_str(&formats.join("\n"));
        s.push_str("\n```\n");
    }
    s
}

fn constraints_and_names(opdata: &Opdata) -> Result<(Option<String>, Vec<String>), &'static str> {
    let data = group_opdata(opdata)?;
    let constraints = format_constraints(&data);
    let names = data.into_iter().flat_map(|a| a.name).collect();
    Ok((constraints, names))
}

fn group_opdata(opdata: &Opdata) -> Result<Vec<ArgWithCommands>, &'static str> {
    let args = flatten_matchers(opdata.matchers);
    let (max_cursor, commands) = group_commands(opdata.commands);

    if args.len() != max_cursor {
        return Err("arg / command count mismatch");
    }

    let mut args: Vec<_> = args.into_iter().map(|(arg, can_be_default)| ArgWithCommands {
        arg,
        can_be_default,
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
    Modifier,
    JumpTarget,
    Lit
}

struct ArgWithCommands {
    pub arg: FlatArgTy,
    pub can_be_default: bool,
    pub commands: Vec<Command>,
    pub name: Option<String>,
}

/// Take a matcher array and return a vector of the types of flat arg each should produce
fn flatten_matchers(matchers: &[Matcher]) -> Vec<(FlatArgTy, bool)> {
    let mut args = Vec::new();
    let mut default = false;

    for matcher in matchers {
        match matcher {
            Matcher::Dot
            | Matcher::Lit(_)
            | Matcher::LitInt(_)
            | Matcher::LitFloat(_) => (),
            Matcher::Ident
            | Matcher::Cond
            | Matcher::Imm => args.push((FlatArgTy::Immediate, default)),
            Matcher::W
            | Matcher::X
            | Matcher::WSP
            | Matcher::XSP
            | Matcher::B
            | Matcher::H
            | Matcher::S
            | Matcher::D
            | Matcher::Q => args.push((FlatArgTy::Direct, default)),
            Matcher::V(_)
            | Matcher::VStatic(_, _)
            | Matcher::VElementStatic(_, _)
            | Matcher::RegList(_, _)
            | Matcher::RegListStatic(_, _, _) => args.push((FlatArgTy::Direct, default)),
            Matcher::VElement(_)
            | Matcher::RegListElement(_, _) => {
                args.push((FlatArgTy::Direct, default));
                args.push((FlatArgTy::Immediate, default));
            },
            Matcher::Offset => args.push((FlatArgTy::JumpTarget, default)),
            Matcher::RefBase => args.push((FlatArgTy::Direct, default)),
            Matcher::RefOffset => {
                args.push((FlatArgTy::Direct, default));
                args.push((FlatArgTy::Immediate, true));
            },
            Matcher::RefPre => {
                args.push((FlatArgTy::Direct, default));
                args.push((FlatArgTy::Immediate, default));
            },
            Matcher::RefIndex => {
                args.push((FlatArgTy::Direct, default));
                args.push((FlatArgTy::Direct, default));
                args.push((FlatArgTy::Modifier, true));
                args.push((FlatArgTy::Immediate, true));
            },
            Matcher::Mod(_) => {
                args.push((FlatArgTy::Modifier, default));
                args.push((FlatArgTy::Immediate, true));
            },
            Matcher::End => default = true,
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
            Command::A => {
                cursor += 1;
                continue;
            },
            Command::C => {
                cursor -= 1;
                continue;
            },
            Command::Rwidth(_) => {
                continue;
            },
            _ => ()
        }

        command_idx.push((command.clone(), cursor));
        match command {
            Command::R(_)
            | Command::R4(_)
            | Command::RNext
            | Command::Ubits(_, _)
            | Command::Uscaled(_, _, _)
            | Command::Ulist(_, _)
            | Command::Urange(_, _, _)
            | Command::Usub(_, _, _)
            | Command::Usumdec(_, _)
            | Command::Ufields(_)
            | Command::Sbits(_, _)
            | Command::Sscaled(_, _,_)
            | Command::Special(_, _)
            | Command::Rotates(_)
            | Command::ExtendsW(_)
            | Command::ExtendsX(_)
            | Command::Cond(_)
            | Command::CondInv(_)
            | Command::LitList(_, _)
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
            match arg.arg {
                FlatArgTy::Modifier => (),
                _ => return Err("Arg with no commands")
            }
        }

        for command in &arg.commands {
            let check = match command {
                Command::R(_)
                | Command::R4(_)
                | Command::RNext => arg.arg == FlatArgTy::Direct,
                Command::Ubits(_, _)
                | Command::Uscaled(_, _, _)
                | Command::Ulist(_, _)
                | Command::Urange(_, _, _)
                | Command::Usub(_, _, _)
                | Command::Usumdec(_, _)
                | Command::Ufields(_)
                | Command::Sbits(_, _)
                | Command::Sscaled(_, _,_)
                | Command::BUbits(_)
                | Command::BSbits(_)
                | Command::BUrange(_, _)
                | Command::Uslice(_, _, _)
                | Command::Sslice(_, _, _)
                | Command::Special(_, _) => arg.arg == FlatArgTy::Immediate,
                Command::Cond(_)
                | Command::CondInv(_)
                | Command::LitList(_, _) => arg.arg == FlatArgTy::Lit || arg.arg == FlatArgTy::Immediate,
                Command::Offset(_) => arg.arg == FlatArgTy::JumpTarget,
                Command::Rotates(_)
                | Command::ExtendsW(_)
                | Command::ExtendsX(_) => arg.arg == FlatArgTy::Modifier,
                Command::A
                | Command::C
                | Command::Rwidth(_) => unreachable!()
            };

            if !check {
                return Err("command / argtype mismatch");
            }

            let check = match command {
                Command::R(_)
                | Command::Ubits(_, _)
                | Command::Uscaled(_, _, _)
                | Command::Uslice(_, _, _)
                | Command::Urange(_, 0, _)
                | Command::Ulist(_, _)
                | Command::Ufields(_)
                | Command::Sbits(_, _)
                | Command::Sscaled(_, _, _)
                | Command::Sslice(_, _, _)
                | Command::BUbits(_)
                | Command::BSbits(_)
                | Command::Rotates(_)
                | Command::ExtendsW(_)
                | Command::ExtendsX(_) => true,
                Command::R4(_)
                | Command::RNext
                | Command::Urange(_, _, _)
                | Command::Usub(_, _, _)
                | Command::Usumdec(_, _)
                | Command::BUrange(_, _)
                | Command::Special(_, _)
                | Command::Cond(_)
                | Command::CondInv(_)
                | Command::LitList(_, _)
                | Command::Offset(_) => !arg.can_be_default,
                Command::A
                | Command::C
                | Command::Rwidth(_) => unreachable!()
            };

            if !check {
                return Err("default mismatch");
            }
        }
    }

    Ok(())
}

/// assign names to the args being used
fn name_args(args: &mut [ArgWithCommands]) {
    // iirc no op uses more than 4 unconstrained literals / immediates
    let reg_name_list = ["n", "m", "a", "b"];
    let mut reg_name_idx = 0;
    let imm_name_list = ["", "1", "2", "3"];
    let mut imm_name_idx = 0;

    for arg in args {
        match arg.arg {
            FlatArgTy::Direct => {
                match &arg.commands[0] {
                    Command::R(_)
                    | Command::R4(_) => {
                        arg.name = Some(format!("{}", reg_name_list[reg_name_idx]));
                        reg_name_idx += 1;
                    },
                    Command::RNext => {
                        arg.name = Some(format!("{}+1", reg_name_list[reg_name_idx - 1]));
                    },
                    _ => unreachable!()
                }
            },
            FlatArgTy::Immediate => {
                match &arg.commands[0] {
                    Command::Cond(_)
                    | Command::CondInv(_) => arg.name = None,
                    Command::LitList(_, name) => arg.name = Some(name.trim_end_matches('S').to_lowercase()),
                    Command::Ubits(_, _)
                    | Command::Uscaled(_, _, _)
                    | Command::Ulist(_, _)
                    | Command::Urange(_, _, _)
                    | Command::Usub(_, _, _)
                    | Command::Usumdec(_, _)
                    | Command::Ufields(_)
                    | Command::BUbits(_)
                    | Command::BUrange(_, _)
                    | Command::Uslice(_, _, _) => {
                        arg.name = Some(format!("uimm{}", imm_name_list[imm_name_idx]));
                        imm_name_idx += 1;
                    },
                    Command::Sbits(_, _)
                    | Command::Sscaled(_, _,_)
                    | Command::BSbits(_)
                    | Command::Sslice(_, _, _) => {
                        arg.name = Some(format!("simm{}", imm_name_list[imm_name_idx]));
                        imm_name_idx += 1;
                    },
                    Command::Special(_, _) => {
                        arg.name = Some(format!("imm{}", imm_name_list[imm_name_idx]));
                        imm_name_idx += 1;
                    },
                    _ => unreachable!()
                }
            },
            FlatArgTy::Modifier => arg.name = None,
            FlatArgTy::JumpTarget => match &arg.commands[0] {
                Command::Offset(_) => arg.name = Some("<offset>".to_string()),
                _ => unreachable!()
            },
            FlatArgTy::Lit => match &arg.commands[0] {
                Command::Cond(_)
                | Command::CondInv(_) => arg.name = None,
                Command::LitList(_, name) => arg.name = Some(name.trim_end_matches('S').to_lowercase()),
                _ => unreachable!()
            }
        }
    }
}

fn format_constraints(args: &[ArgWithCommands]) -> Option<String> {
    let mut constraints = Vec::new();
    let mut prevname = "???";

    for arg in args {
        if let Some(ref name) = arg.name {
            emit_constraints(name, prevname, &arg.commands, &mut constraints);
            prevname = name;
        }
    }

    if constraints.is_empty() {
        None
    } else {
        Some(format!("    where {}", constraints.join(", ")))
    }
}

fn emit_constraints(name: &str, prevname: &str, commands: &[Command], buf: &mut Vec<String>) {
    for command in commands {
        buf.push(match command {
            Command::R4(_) => format!("{} is register number 0-15", name),
            Command::Ubits(_, bits)
            | Command::BUbits(bits) => format!("0 <= #{} < {}", name, 1u32 << bits),
            Command::Uscaled(_, bits, scale) => format!("0 <= #{} < {} and #{} is a multiple of {}", name, 1u32 << (bits + scale), name, 1u32 << scale),
            Command::Ulist(_, list) => {
                let numbers = list.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
                format!("#{} is one of [{}]", name, numbers)
            },
            Command::Urange(_, min, max)
            | Command::BUrange(min, max) => format!("{} <= #{} <= {}", min, name, max),
            Command::Usub(_, bits, addval) => format!("0 <= #{} - {} < {}", addval, name, 1u32 << bits),
            Command::Usumdec(_, bits) => format!("0 < #{} + #{} <= {}", name, prevname, 1u32 << bits),
            Command::Ufields(fields) => format!("0 <= #{} < {}", name, 1u32 << fields.len()),
            Command::Sbits(_, bits)
            | Command::BSbits(bits) => format!("-{} <= #{} < {}", 1u32 << (bits - 1), name, 1u32 << (bits - 1)),
            Command::Sscaled(_, bits, scale) => format!("-{} <= #{} < {} and #{} is a multiple of {}", 1u32 << (bits + scale - 1), name, 1u32 << (bits + scale - 1), name, 1u32 << scale),
            Command::Special(_, SpecialComm::WIDE_IMMEDIATE_W)
            | Command::Special(_, SpecialComm::WIDE_IMMEDIATE_X)
            | Command::Special(_, SpecialComm::INVERTED_WIDE_IMMEDIATE_W)
            | Command::Special(_, SpecialComm::INVERTED_WIDE_IMMEDIATE_X) => format!("#{} is a wide immediate", name),
            Command::Special(_, SpecialComm::LOGICAL_IMMEDIATE_W)
            | Command::Special(_, SpecialComm::LOGICAL_IMMEDIATE_X) => format!("#{} is a logical immediate", name),
            Command::Special(_, SpecialComm::FLOAT_IMMEDIATE)
            | Command::Special(_, SpecialComm::SPLIT_FLOAT_IMMEDIATE) => format!("#{} is a floating point immediate", name),
            Command::Special(_, SpecialComm::STRETCHED_IMMEDIATE) => format!("#{} is a stretched immediate", name),
            Command::Offset(Relocation::B) => format!("offset is 26 bit, 4-byte aligned"),
            Command::Offset(Relocation::BCOND) => format!("offset is 19 bit, 4-byte aligned"),
            Command::Offset(Relocation::ADR) => format!("offset is a 21 bit"),
            Command::Offset(Relocation::ADRP) => format!("offset is 21 bit, 4K-page aligned"),
            Command::Offset(Relocation::TBZ) => format!("offset is 14 bit, 4-byte aligned"),
            Command::Offset(Relocation::LITERAL32) => format!("offset is 32 bit>"),
            Command::Offset(Relocation::LITERAL64) => format!("offset is 64 bit>"),
            _ => continue
        });
        break;
    }
}
