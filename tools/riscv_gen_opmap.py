import riscv_load_data
import copy

from pathlib import Path

# we get basic encoding data from the riscv_opcodes project
# this however isn't quite enough to fully translate to instruction definitions. The main problem
# is that this project focusses on the encoding of these instructions, it doesn't fully specify the
# assembly syntax, in particular the argument format. The problem is, nothing really does! There
# is no actual fully-featured risc-v assembly specification, even though the manuals provide many
# examples in assembly!
#
# the riscv_opmap project is therefore incomplete in at least the following ways for our purposes
# - it does not provide any information on the ordering of arguments
# - with memory references, there's no information which register is the index and which is the base
# - several instructions (like fli) encode something that is definitely not a register in a register field
# - several instructions have something listed as a field that ends up being part of the mnemnonic
#     (like sc.d, which can actually be sc.d.aqrl)
# - several instructions have different variants when floating point in normal registers extensions
#     are enabled (zfinx, zdinx, zhinx), and these are not provided
# - there's no information if a certain register ought to be a floating point register or not!
#
#
# table summarizing what most encountered arg values are
# rd: destination register, of any type
# rs[123]: source register, of any type
# aq/rl/aqrl: acquire/release bits for atomic instructions. means the instruction has .aq,.rl and .aqrl variants
# rd_rs1_n0: this arg is both the source and destination register. it cannot be x0
# c_rs1_n0
# c_rs2_n0: register 2: it cannot be x0
# c_nzimm*: a nonzero immediate (part)
# rd_rs1_p: dest and source 1 register, can only be one of the popular 8 registers (x8-x15)
# rm: rounding mode, often found with floating point instructions. can be (RNE, RTZ, RDN, RUP, RMM, ..., ..., DYN). defaults to DYN. is just an optional
#     argument at the end of the instruction
# rs2=rs1: second source register matches the first source register, so just double encode that one
# pred, succ: idk, seem to just be funny immediates for now
# jim: jump immediate, i.e. an offset
# shamtw/shamtd: shift ammount
# c_mop_t and mop_rr_t: just some weird immediates
# csr: status registers. Just treat as immediates for now
# bs: blocksize. 
# c_rlist: register list: oh jolly why do we have these
# c_index


def derive_arg_matchers(instruction: riscv_load_data.Instruction) -> str:
    matchers = []
    has_combined_immediate = False

    for field in instruction.encoding.fields:
        if field.startswith(("rd", "rs", "c_rd", "c_rs", "c_sreg")):
            if "=" not in field:
                matchers.append("F" if instruction.name.startswith("f") else "X")

        elif field == "c_rlist":
            matchers.append("Xlist")

        elif field in ("shamtw", "shamtd", "c_index", "bs", "rnum"):
            matchers.append("Imm")

        elif field in ("aq", "rl", "aqrl", "fm"):
            pass # handled in instruction memnonic

        elif "imm" in field or "mop" in field:
            has_combined_immediate = True

        elif field in ("pred", "succ", "csr", "rm"):
            matchers.append("Ident")

        else:
            raise Exception(f"unhandled field {field}")

    if has_combined_immediate:
        matchers.append("Imm")

    return ", ".join(matchers)

class Encoder:
    def __init__(self):
        self.stored = set()
        self.fields = []

    def derive_encoders(self, instruction: riscv_load_data.Instruction):
        try:
            for field in instruction.encoding.fields:
                self.map_field(field)
            return self.finalize()
        except Exception as e:
            print(instruction.encoding.fields, instruction.name, instruction.extensions)
            raise e

    def finalize(self) -> str:
        self.emit_split_immediates()
        return ', '.join(self.fields)

    def map_field(self, field) -> str:
        # dirty solution to handling encoder behaviour
        lut = riscv_load_data.get_arg_lut()
        if "=" in field:
            bot, top = lut.get_arg_bounds(field.split("=")[0])
        else:
            bot, top = lut.get_arg_bounds(field)
        bits = top + 1 - bot

        # all the register types
        if field in ("rd", "rs1", "rs2", "rs3", "c_rs2"):
            self.fields.append(f"R({bot})")
            return

        if field in ("rd_rs1_n0", "rd_n0", "rs1_n0", "c_rs1_n0", "c_rs2_n0"):
            self.fields.append(f"Rno0({bot})")
            return

        if field == "rd_n2":
            self.fields.append(f"Rno02({bot})")
            return

        if field in ("rd_p", "rs1_p", "rs2_p", "rd_rs1_p"):
            self.fields.append(f"Rpop({bot})")
            return

        if field == "rs2=rs1":
            self.fields.append(f"Repeat, R({bot})")
            return

        if field in ("c_sreg1", "c_sreg2"):
            self.fields.append(f"Rpops({bot})")
            return

        if field == "c_rlist":
            self.fields.append(f"Rlist({bot})")
            return

        # special things
        if field in ("aq", "rl", "aqrl", "fm"):
            return

        if field == "rm":
            self.fields.append(f"RoundingMode({bot})")
            return

        if field == "csr":
            self.fields.append(f"Csr({bot})")
            return

        if field in ("pred", "succ"):
            self.fields.append(f"FenceSpec({bot})")
            return

        # simple immediates
        # unsigned
        if field in ("bs", "shamtd", "shamtw", "zimm"):
            self.fields.append(f"UImm({bits}, 0), BitRange({bot}, {bits}, 0), Next")
            return

        if field == "c_index":
            self.fields.append(f"UImmRange(0xFF, 0xFF), BitRange({bot}, {bits}, 0), Next")
            return

        if field in ("imm20", "imm12"):
            self.fields.append(f"SImm({bits}, 0), BitRange({bot}, {bits}, 0), Next")
            return

        # other non-split immediates
        if field == "c_uimm2":
            self.fields.append(f"UImm(2, 0), Bits({bot}, &[0, 1]), Next")
            return

        if field == "c_uimm1":
            self.fields.append(f"UImm(1, 0), BitRange({bot}, {bits}, 0), Next")
            return

        if field == "c_mop_t":
            self.fields.append(f"UImmOdd(3, 1), BitRange({bot}, 2, 1), Next")
            return

        if field == "c_nzuimm10":
            self.fields.append(f"UImmNo0(10, 2), Bits({bot}, &[5, 4, 9, 8, 7, 6, 2, 3]), Next")
            return

        if field == "c_imm12":
            self.fields.append(f"SImm(12, 1), Bits({bot}, &[11, 4, 9, 8, 10, 6, 7, 3, 2, 1, 5]), Next")
            return

        if field == "c_nzuimm5":
            self.fields.append(f"UImmNo0(5, 0), BitRange({bot}, 5, 0), Next")
            return

        if field == "c_uimm8sp_s":
            self.fields.append(f"UImm(8, 2), Bits({bot}, &[5, 4, 3, 2, 7, 6]), Next")
            return

        if field == "c_uimm9sp_s":
            self.fields.append(f"UImm(9, 3), Bits({bot}, &[5, 4, 3, 8, 7, 6]), Next")
            return

        if field == "c_spimm":
            self.fields.append(f"UImm(6, 4), BitRange({bot}, 2, 4), Next")
            return

        if field == "jimm20":
            self.fields.append(f'Offset(J)')
            return

        if field == "rnum":
            self.fields.append(f"UImmRange(0, 10), BitRange({bot}, 4, 0), Next")
            return

        # checking for combined immediates
        if field.endswith(("lo", "hi")) or field in ("mop_r_t_30", "mop_r_t_27_26", "mop_r_t_21_20", "mop_rr_t_30", "mop_rr_t_27_26"):
            self.stored.add(field)
            return

        raise Exception(f"Unhandled field {field}")

    def emit_split_immediates(self):
        # dirty solution to handling encoder behaviour
        lut = riscv_load_data.get_arg_lut()
        if not self.stored:
            return

        # signed 6-bit immediate, total immediate cannot be 0
        if self.stored == {"c_nzimm6lo", "c_nzimm6hi"}:
            lo, _ = lut.get_arg_bounds("c_nzimm6lo")
            hi, _ = lut.get_arg_bounds("c_nzimm6hi")
            self.fields.append(f"SImmNo0(6, 0), BitRange({lo}, 5, 0), BitRange({hi}, 1, 5), Next")
            return

        # signed 4-bit scaled 10-bit immediate, total immediate cannot be 0
        if self.stored == {"c_nzimm10lo", "c_nzimm10hi"}:
            lo, _ = lut.get_arg_bounds("c_nzimm10lo")
            hi, _ = lut.get_arg_bounds("c_nzimm10hi")
            self.fields.append(f"SImmNo0(10, 4), Bits({lo}, &[4, 6, 8, 7, 5]), BitRange({hi}, 1, 9), Next")
            return

        # signed 6-bit immediate
        if self.stored == {"c_imm6lo", "c_imm6hi"}:
            lo, _ = lut.get_arg_bounds("c_imm6lo")
            hi, _ = lut.get_arg_bounds("c_imm6hi")
            self.fields.append(f"SImm(6, 0), BitRange({lo}, 5, 0), BitRange({hi}, 1, 5), Next")
            return

        # signed  1-bit scaled 9-bit jump offset/immediate
        if self.stored == {"c_bimm9lo", "c_bimm9hi"}:
            lo, _ = lut.get_arg_bounds("c_bimm9lo")
            hi, _ = lut.get_arg_bounds("c_bimm9hi")
            self.fields.append(f"SImm(9, 1), Bits({lo}, &[7, 6, 2, 1, 5]), Bits({hi}, &[8, 4, 3]), Next")
            return

        # signed 12-bit scaled 18-bit immediate that cannot be 0
        if self.stored == {"c_nzimm18lo", "c_nzimm18hi"}:
            lo, _ = lut.get_arg_bounds("c_nzimm18lo")
            hi, _ = lut.get_arg_bounds("c_nzimm18hi")
            self.fields.append(f"SImmNo0(18, 12), BitRange({lo}, 5, 12), BitRange({hi}, 17, 1), Next")
            return

        # unsigned 2-bit scaled 7-bit immediate
        if self.stored == {"c_uimm7lo", "c_uimm7hi"}:
            lo, _ = lut.get_arg_bounds("c_uimm7lo")
            hi, _ = lut.get_arg_bounds("c_uimm7hi")
            self.fields.append(f"UImm(7, 2), BitRange({lo}, 3, 5), BitRange({hi}, 2, 3), Next")
            return

        # unsigned, 2-bit scaled 8-bit immediate. this isn't always correct
        if self.stored == {"c_uimm8splo", "c_uimm8sphi"}:
            lo, _ = lut.get_arg_bounds("c_uimm8splo")
            hi, _ = lut.get_arg_bounds("c_uimm8sphi")
            self.fields.append(f"UImm(8, 2), Bits({lo}, &[4, 3, 2, 7, 6]), BitRange({hi}, 1, 5), Next")
            return

        # unsigned 6-bit immediate that cannot be 0
        if self.stored == {"c_nzuimm6lo", "c_nzuimm6hi"}:
            lo, _ = lut.get_arg_bounds("c_nzuimm6lo")
            hi, _ = lut.get_arg_bounds("c_nzuimm6hi")
            self.fields.append(f"UImmNo0(6, 0), BitRange({lo}, 5, 0), BitRange({hi}, 1, 5), Next")
            return

        # unsigned 5-bit immediate that cannot be 0. sometimes appears on its own
        if self.stored == {"c_nzuimm6lo",}:
            lo, _ = lut.get_arg_bounds("c_nzuimm6lo")
            self.fields.append(f"UImmNo0(5, 0), BitRange({lo}, 5, 0), Next")
            return

        # unsigned, 3-bit scaled, 8-bit immediate
        if self.stored == {"c_uimm8lo", "c_uimm8hi"}:
            lo, _ = lut.get_arg_bounds("c_uimm8lo")
            hi, _ = lut.get_arg_bounds("c_uimm8hi")
            self.fields.append(f"UImm(8, 3), BitRange({lo}, 2, 6), BitRange({hi}, 3, 3), Next")
            return

        # unsigned, 3-bit scaled, 9-bit immediate
        if self.stored == {"c_uimm9splo", "c_uimm9sphi"}:
            lo, _ = lut.get_arg_bounds("c_uimm9splo")
            hi, _ = lut.get_arg_bounds("c_uimm9sphi")
            self.fields.append(f"UImm(9, 3), Bits({lo}, &[4, 3, 8, 7, 6]), BitRange({hi}, 1, 5), Next")
            return

        # signed 12-bit immediate
        if self.stored == {"imm12lo", "imm12hi"}:
            lo, _ = lut.get_arg_bounds("imm12lo")
            hi, _ = lut.get_arg_bounds("imm12hi")
            self.fields.append(f"SImm(12, 0), BitRange({lo}, 5, 0), BitRange({hi}, 7, 5), Next")
            return

        # signed 5-bit scaled 12-bit immediate, sometimes appears on its own
        if self.stored == {"imm12hi",}:
            hi, _ = lut.get_arg_bounds("imm12hi")
            self.fields.append(f"SImm(12, 5), BitRange({hi}, 7, 5), Next")
            return


        # 2-bit scaled, signed 12-bit offset
        if self.stored == {"bimm12lo", "bimm12hi"}:
            lo, _ = lut.get_arg_bounds("bimm12lo")
            hi, _ = lut.get_arg_bounds("bimm12hi")
            self.fields.append(f'Offset(B)')
            return

        # unsigned 3-bit immediate
        if self.stored == {"mop_rr_t_27_26", "mop_rr_t_30"}:
            lo, _ = lut.get_arg_bounds("mop_rr_t_27_26")
            hi, _ = lut.get_arg_bounds("mop_rr_t_30")
            self.fields.append(f"UImm(3, 0), BitRange({lo}, 2, 0), BitRange({hi}, 1, 2), Next")
            return

        # unsigned 5-bit immediate
        if self.stored == {"mop_r_t_21_20", "mop_r_t_27_26", "mop_r_t_30"}:
            lo, _ = lut.get_arg_bounds("mop_r_t_21_20")
            mid, _ = lut.get_arg_bounds("mop_r_t_27_26")
            hi, _ = lut.get_arg_bounds("mop_r_t_30")
            self.fields.append(f"UImm(5, 0), BitRange({lo}, 2, 0), BitRange({mid}, 2, 2), BitRange({hi}, 1, 4), Next")
            return

        raise Exception(f"Unhandled split immediate {self.stored}")


def derive_arg_encoders(instruction: riscv_load_data.Instruction) -> str:
    return ""


FIELDS = {}

def format_comment(instruction: riscv_load_data.Instruction):
    pseudo = ""
    if isinstance(instruction, riscv_load_data.PseudoInstruction):
        pseudo = f" (subformat of {instruction.parent_extension}::{instruction.parent_name})"

    return f'// {instruction.name} {", ".join(instruction.encoding.fields)}{pseudo} ({', '.join(instruction.extensions)})'


def format_instruction(instruction: riscv_load_data.Instruction, isaflags: str) -> str:
    if instruction.encoding.bitsize == 16:
        template = f"Compressed(0x{instruction.encoding.template:04X})"
    else:
        template = f"Single(0x{instruction.encoding.template:08X})"

    matchers = derive_arg_matchers(instruction)
    # encoders = derive_arg_encoders(encoding)

    encoders = Encoder().derive_encoders(instruction)

    return f'{template}, {isaflags}, [{matchers}] => [{encoders}]'

def format_extensions(instruction: riscv_load_data.Instruction) -> str:
    entries = []
    for extension in instruction.extensions:
        individual = extension.split("_")
        individual = [f"Ex_{i[0].upper()}{i[1:].lower()}" for i in individual]
        entries.append(" | ".join(individual))

    return f"[{', '.join(entries)}]"

class Mnemnonic:
    FLAG_RV32 = 1
    FLAG_RV64 = 2
    def __init__(self, name):
        self.name = name
        self.format_rv32 = []
        self.format_rv64 = []
        self.format_both = []

    def add_format_rv32(self, instruction):
        self.format_rv32.append(instruction)

    def add_format_rv64(self, instruction):
        self.format_rv64.append(instruction)

    def deduplicate(self):
        # basically, if both the instruction encoding and required features
        # are identical, merge them.
        new_format_rv32 = []
        new_format_rv64 = []

        if len(self.format_rv32) == len(self.format_rv64):
            for rv32, rv64 in zip(self.format_rv32, self.format_rv64):
                if (rv32.encoding.fields == rv64.encoding.fields and
                    rv32.encoding.template == rv64.encoding.template and
                    rv32.encoding.bitsize == rv64.encoding.bitsize and
                    rv32.extensions == rv64.extensions):

                    self.format_both.append(rv64)
                else:
                    new_format_rv64.append(rv64)
                    new_format_rv32.append(rv32)

            self.format_rv32 = new_format_rv32
            self.format_rv64 = new_format_rv64


    def sort_key(self):
        # features needed can differ for different target variants, so make a best guess here
        if self.format_both:
            extensions = self.format_both[0].extensions
        elif self.format_rv32:
            extensions = self.format_rv32[0].extensions
        else:
            extensions = self.format_rv64[0].extensions

        return (extensions, self.name)

    def format_opdata(self, lines: []):
        lines.append(f'"{self.name}" = [')
        for instruction in self.format_both:
            instrdata = format_instruction(instruction, "RV32 | RV64")
            lines.append(f"    {format_comment(instruction)}")
            lines.append(f"    {instrdata}, {format_extensions(instruction)};")

        for instruction in self.format_rv32:
            instrdata = format_instruction(instruction, "RV32       ")
            lines.append(f"    {format_comment(instruction)}")
            lines.append(f"    {instrdata}, {format_extensions(instruction)};")

        for instruction in self.format_rv64:
            instrdata = format_instruction(instruction, "       RV64")
            lines.append(f"    {format_comment(instruction)}")
            lines.append(f"    {instrdata}, {format_extensions(instruction)};")

        lines.append('],')


def format_mnemnonics(mnemnonics: [Mnemnonic], outfile: Path):
    lines = []
    last_extensionlist = None
    lines.append("// The base of this file was generated by tools/riscv_gen_opmap.py")
    lines.append("Ops!(")
    lines.append("")

    for mnemnonic in mnemnonics:
        extensionlist = mnemnonic.sort_key()[0]
        if extensionlist != last_extensionlist:
            last_extensionlist = extensionlist
            lines.append(f"\n// Extension(s) {', '.join(extensionlist)}\n")

        mnemnonic.format_opdata(lines)

    lines.append("")
    lines.append(")")
    lines.append("")

    with outfile.open("w", encoding="utf-8") as f:
        f.write('\n'.join(lines))


# if an instruction has aq and rl fields, we need to expand it the relevant extra mnemnonics
def expand_aqrl(instructions):
    new_instructions = []
    lut = riscv_load_data.get_arg_lut()
    aq = 1 << lut.get_arg_bounds("aq")[0]
    rl =  1 << lut.get_arg_bounds("rl")[0]
    aqrl = aq | rl

    for instruction in instructions:
        new_instructions.append(instruction)
        if "aq" in instruction.encoding.fields and "rl" in instruction.encoding.fields:
            instruction.encoding.mask |= aqrl

            aq_variant = copy.deepcopy(instruction)
            rl_variant = copy.deepcopy(instruction)
            aqrl_variant = copy.deepcopy(instruction)

            aq_variant.encoding.template |= aq
            rl_variant.encoding.template |= rl
            aqrl_variant.encoding.template |= aqrl

            aq_variant.name += ".aq"
            rl_variant.name += ".rl"
            aqrl_variant.name += ".aqrl"

            new_instructions.append(aq_variant)
            new_instructions.append(rl_variant)
            new_instructions.append(aqrl_variant)

    return new_instructions


# roundmode is an optional argument with a default value. Split instructions into variants
# taking it, and variants having it encoded to DYN by default
def expand_roundmode(instructions):
    new_instructions = []
    lut = riscv_load_data.get_arg_lut()
    dyn = 7 << lut.get_arg_bounds("rm")[0]
    
    for instruction in instructions:
        new_instructions.append(instruction)
        if "rm" in instruction.encoding.fields:

            default_variant = copy.deepcopy(instruction)
            default_variant.encoding.mask |= dyn
            default_variant.encoding.template |= dyn
            default_variant.encoding.fields.remove("rm")

            new_instructions.append(default_variant)

    return new_instructions




def strip_trailing_rv32(instructions):
    # a bunch of instructions have _rv32 or .rv32 in riscv_opcodes after their name to prevent
    # name conflicts. we can handle these so strip this.
    for instruction in instructions:
        if instruction.name.endswith(("_rv32", ".rv32")):
            instruction.name = instruction.name[:-5]

def main():
    archive = riscv_load_data.get_archive()

    rv32_extensions = archive.get_available_extensions("rv32")
    rv64_extensions = archive.get_available_extensions("rv64")

    # right now we're not dealing with the vector or privileged instruction sets
    unused_extensions = ("v", "s", "h", "zv", "aliases")

    rv32_extensions = [i for i in rv32_extensions if not i.startswith(unused_extensions)]
    rv64_extensions = [i for i in rv64_extensions if not i.startswith(unused_extensions)]


    rv32_instruction_list = archive.create_instruction_list("rv32", rv32_extensions)
    rv64_instruction_list = archive.create_instruction_list("rv64", rv64_extensions)

    strip_trailing_rv32(rv32_instruction_list)

    rv32_instruction_list = expand_roundmode(expand_aqrl(rv32_instruction_list))
    rv64_instruction_list = expand_roundmode(expand_aqrl(rv64_instruction_list))

    # deduplicate common instructions, create a list on name basis
    mnemnonics = {}

    for instruction in rv32_instruction_list:
        if instruction.name not in mnemnonics:
            mnemnonics[instruction.name] = Mnemnonic(instruction.name)
        mnemnonics[instruction.name].add_format_rv32(instruction)

    for instruction in rv64_instruction_list:
        if instruction.name not in mnemnonics:
            mnemnonics[instruction.name] = Mnemnonic(instruction.name)
        mnemnonics[instruction.name].add_format_rv64(instruction)

    for mnemnonic in mnemnonics.values():
        mnemnonic.deduplicate()

    mnemnonics = list(mnemnonics.values())
    mnemnonics.sort(key=lambda i: i.sort_key())

    format_mnemnonics(mnemnonics, Path("riscv_opmap_template.rs"))






if __name__ == '__main__':
    main()