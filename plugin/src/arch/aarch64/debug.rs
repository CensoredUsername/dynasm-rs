use super::ast::Modifier;
use super::aarch64data::{Opdata, Matcher, Command, Relocation, SpecialComm};
use crate::common::Size;

use std::fmt::Write;

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
                Matcher::Dot => write!(buf, ".").unwrap(),
                Matcher::Lit(s) => write!(buf, "{}", s).unwrap(),
                Matcher::LitInt(v) => write!(buf, "{}", v).unwrap(),
                Matcher::LitFloat(v) => write!(buf, "{}", v).unwrap(),
                Matcher::Ident => {
                    let name = names.next().unwrap();
                    write!(buf, "{}", name).unwrap();
                },
                Matcher::Cond => write!(buf, "<cond>").unwrap(),
                Matcher::Imm => {
                    let name = names.next().unwrap();
                    write!(buf, "#{}", name).unwrap();
                },
                Matcher::W => {
                    let name = names.next().unwrap();
                    write!(buf, "W{}", name).unwrap();
                },
                Matcher::X => {
                    let name = names.next().unwrap();
                    write!(buf, "X{}", name).unwrap();
                },
                Matcher::WSP => {
                    let name = names.next().unwrap();
                    write!(buf, "W{}|WSP", name).unwrap();
                },
                Matcher::XSP => {
                    let name = names.next().unwrap();
                    write!(buf, "X{}|SP", name).unwrap();
                },
                Matcher::B => {
                    let name = names.next().unwrap();
                    write!(buf, "B{}", name).unwrap();
                },
                Matcher::H => {
                    let name = names.next().unwrap();
                    write!(buf, "H{}", name).unwrap();
                },
                Matcher::S => {
                    let name = names.next().unwrap();
                    write!(buf, "S{}", name).unwrap();
                },
                Matcher::D => {
                    let name = names.next().unwrap();
                    write!(buf, "D{}", name).unwrap();
                },
                Matcher::Q => {
                    let name = names.next().unwrap();
                    write!(buf, "Q{}", name).unwrap();
                },
                Matcher::V(s) => {
                    let width = if i == 0 { 16 } else { 8 };
                    let name = names.next().unwrap();
                    write!(buf, "V{}.{}{}", name, size_to_string(s), width / s.in_bytes()).unwrap();
                },
                Matcher::VStatic(s, c) => {
                    let name = names.next().unwrap();
                    write!(buf, "V{}.{}{}", name, size_to_string(s), c).unwrap();
                },
                Matcher::VElement(s) => {
                    let name = names.next().unwrap();
                    let imm = names.next().unwrap();
                    write!(buf, "V{}.{}[{}]", name, size_to_string(s), imm).unwrap();
                },
                Matcher::VElementStatic(s, element) => {
                    let name = names.next().unwrap();
                    write!(buf, "V{}.{}[{}]", name, size_to_string(s), element).unwrap();
                },
                Matcher::RegList(a, s) => {
                    let width = if i == 0 { 16 } else { 8 };
                    let name = names.next().unwrap();
                    write!(buf, "{{V{}.{}{} * {}}}", name, size_to_string(s), width / s.in_bytes(), a).unwrap();
                },
                Matcher::RegListStatic(a, s, c) => {
                    let name = names.next().unwrap();
                    write!(buf, "{{V{}.{}{} * {}}}", name, size_to_string(s), c, a).unwrap();
                },
                Matcher::RegListElement(a, s) => {
                    let name = names.next().unwrap();
                    let imm = names.next().unwrap();
                    write!(buf, "{{V{}.{} * {}}}[{}]", name, size_to_string(s), a, imm).unwrap();
                },
                Matcher::Offset => buf.push_str(&names.next().unwrap()),
                Matcher::RefBase => {
                    let name = names.next().unwrap();
                    write!(buf, "[X{}|SP]", name).unwrap();
                },
                Matcher::RefOffset => {
                    let name = names.next().unwrap();
                    let imm = names.next().unwrap();
                    write!(buf, "[X{}|SP {{, #{} }} ]", name, imm).unwrap();
                },
                Matcher::RefPre => {
                    let name = names.next().unwrap();
                    let imm = names.next().unwrap();
                    write!(buf, "[X{}|SP, #{}]!", name, imm).unwrap();
                },
                Matcher::RefIndex => {
                    let name1 = names.next().unwrap();
                    let name2 = names.next().unwrap();
                    let imm = names.next().unwrap();
                    write!(buf, "[X{}|SP, W{}|X{} {{ , UXTW|LSL|SXTW|SXTX {{ #{} }} }} ]", name1, name2, name2, imm).unwrap();
                },
                Matcher::LitMod(m) => {
                    buf.push_str(m.as_str());
                    let name = names.next().unwrap();
                    if !m.expr_required() {
                        write!(buf, " {{ #{} }}", name).unwrap();
                    } else {
                        write!(buf, " #{}", name).unwrap();
                    }
                },
                Matcher::Mod(mods) => {
                    let mut required = false;
                    let mut unsigned_extends = String::new();
                    let mut signed_extends   = String::new();
                    let mut rest = Vec::new();
                    for m in *mods {
                        required = required || m.expr_required();
                        match m {
                            Modifier::LSL
                            | Modifier::LSR
                            | Modifier::ASR
                            | Modifier::ROR
                            | Modifier::MSL => rest.push(m.as_str()),
                            Modifier::SXTX
                            | Modifier::SXTW
                            | Modifier::SXTH
                            | Modifier::SXTB => signed_extends.push(m.as_str().chars().nth(3).unwrap()),
                            Modifier::UXTX
                            | Modifier::UXTW
                            | Modifier::UXTH
                            | Modifier::UXTB => unsigned_extends.push(m.as_str().chars().nth(3).unwrap()),
                        }
                    }
                    if !unsigned_extends.is_empty() {
                        if unsigned_extends.len() > 1 {
                            unsigned_extends = format!("UXT[{}]", unsigned_extends);
                        } else {
                            unsigned_extends = format!("UXT{}", unsigned_extends);
                        }
                        rest.push(&unsigned_extends);
                    }
                    if !signed_extends.is_empty() {
                        if signed_extends.len() > 1 {
                            signed_extends = format!("SXT[{}]", signed_extends);
                        } else {
                            signed_extends = format!("SXT{}", signed_extends);
                        }
                        rest.push(&signed_extends);
                    }
                    buf.push_str(&rest.join("|"));

                    let name = names.next().unwrap();
                    if !required {
                        write!(buf, " {{ #{} }}", name).unwrap();
                    } else {
                        write!(buf, " #{}", name).unwrap();
                    }
                },
                Matcher::End => ()
            }

        }

        for _ in 0 .. end_count {
            buf.push_str(" }");
        }
        let mut buf = buf.replace(" ., ", ".");

        if let Some(c) = constraints {
            let mut len = c.len() + buf.len();
            while len < 100 {
                buf.push(' ');
                len += 1;
            }
            buf.push_str(&c);
        }

        forms.push(buf);
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
        write!(s, "### {}\n```insref\n{}\n```\n", mnemnonic, formats.join("\n")).unwrap();
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
            Matcher::LitMod(_) => {
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
            return Err("Arg with no commands")
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
                | Command::Urange(_, _, _)
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
    let mut constraints = String::new();
    let mut prevname = "?";

    for arg in args {
        if let Some(ref name) = arg.name {
            emit_constraints(name, prevname, &arg.commands, &mut constraints);
            prevname = name;
        }
    }

    if constraints.is_empty() {
        None
    } else {
        let len = constraints.len();
        Some(format!(" ({})", &constraints[0 .. len - 2]))
    }
}

fn emit_constraints(name: &str, prevname: &str, commands: &[Command], buf: &mut String) {
    for command in commands {
        match command {
            Command::R4(_) => write!(buf, "{} is 0-15", name),
            Command::Ubits(_, bits)
            | Command::BUbits(bits) => write!(buf, "#{} < {}", name, 1u32 << bits),
            Command::Uscaled(_, bits, scale) => write!(buf, "#{} < {}, #{} = {} * N", name, 1u32 << (bits + scale), name, 1u32 << scale),
            Command::Ulist(_, list) => {
                let numbers = list.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
                write!(buf, "#{} = [{}]", name, numbers)
            },
            Command::Urange(_, min, max)
            | Command::BUrange(min, max) => write!(buf, "{} <= #{} <= {}", min, name, max),
            Command::Usub(_, bits, addval) => write!(buf, "{} <= #{} <= {}", *addval as u32 - (1u32 << bits) + 1, name, addval),
            Command::Usumdec(_, bits) => write!(buf, "1 <= #{} + #{} <= {}", prevname, name, (1u32 << bits)),
            Command::Ufields(fields) => write!(buf, "#{} < {}", name, 1u32 << fields.len()),
            Command::Sbits(_, bits)
            | Command::BSbits(bits) => write!(buf, "-{} <= #{} < {}", 1u32 << (bits - 1), name, 1u32 << (bits - 1)),
            Command::Sscaled(_, bits, scale) => write!(buf, "-{} <= #{} < {}, #{} = {} * N", 1u32 << (bits + scale - 1), name, 1u32 << (bits + scale - 1), name, 1u32 << scale),
            Command::Special(_, SpecialComm::WIDE_IMMEDIATE_W)
            | Command::Special(_, SpecialComm::WIDE_IMMEDIATE_X)
            | Command::Special(_, SpecialComm::INVERTED_WIDE_IMMEDIATE_W)
            | Command::Special(_, SpecialComm::INVERTED_WIDE_IMMEDIATE_X) => write!(buf, "#{} is a wide immediate", name),
            Command::Special(_, SpecialComm::LOGICAL_IMMEDIATE_W)
            | Command::Special(_, SpecialComm::LOGICAL_IMMEDIATE_X) => write!(buf, "#{} is a logical immediate", name),
            Command::Special(_, SpecialComm::FLOAT_IMMEDIATE)
            | Command::Special(_, SpecialComm::SPLIT_FLOAT_IMMEDIATE) => write!(buf, "#{} is a floating point immediate", name),
            Command::Special(_, SpecialComm::STRETCHED_IMMEDIATE) => write!(buf, "#{} is a stretched immediate", name),
            Command::Offset(Relocation::B) => write!(buf, "offset is 26 bit, 4-byte aligned"),
            Command::Offset(Relocation::BCOND) => write!(buf, "offset is 19 bit, 4-byte aligned"),
            Command::Offset(Relocation::ADR) => write!(buf, "offset is 21 bit"),
            Command::Offset(Relocation::ADRP) => write!(buf, "offset is 21 bit, 4K-page aligned"),
            Command::Offset(Relocation::TBZ) => write!(buf, "offset is 14 bit, 4-byte aligned"),
            Command::Offset(Relocation::LITERAL32) => write!(buf, "offset is 32 bit>"),
            Command::Offset(Relocation::LITERAL64) => write!(buf, "offset is 64 bit>"),
            _ => continue
        }.unwrap();

        write!(buf, ", ").unwrap();
        break;
    }
}
