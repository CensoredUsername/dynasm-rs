#!/usr/bin/python3

# this file takes the opmap exported by docs/insref/export.rs and generates a big list of test cases
# with it that can be compared with another assembler.
# syntax is riscv_gen_tests.py input_file test_multiplier output_file

import ast
import random
import re
from pathlib import Path
import argparse

RV32_BLACKLIST = {
    "lui",
    "c.lui",
    "auipc",
    "li",
}

RV64_BLACKLIST = {
    # current gas doesn't believe this op exists on rv64, but the docs say it does.
    # this is was fixed very recently so packages don't have it up to date yet
    "ssamoswap.w",
    "ssamoswap.w.aq",
    "ssamoswap.w.rl",
    "ssamoswap.w.aqrl",
    # we have some disagreements with gas for how these should be formatted
    # gas wants to encode the immediate before shifting, and for some reason
    # doesn't allow negative values for auipc / c.lui/ lui, which is fairly odd.
    # as those should actually be sign extended.
    "lui",
    "c.lui",
    "auipc",
    # the sequences gas generates for "li" are unpredictable.
    "li",
    "li.44",
    "li.56",
    "li.64"
}

def main():
    parser = argparse.ArgumentParser("riscv_gen_tests",  description="generate riscv testcases from the exported dynasm-rs opmap definitions")
    parser.add_argument("opmap_export_file", type=Path)
    parser.add_argument("test_multiplier", type=int)
    parser.add_argument("result_file", type=Path)
    args = parser.parse_args()

    import sys
    outfile = sys.argv[3]
    with args.opmap_export_file.open("r", encoding="utf-8") as f:
        templates = read_opdata_file(f)

    riscv32_tests = []
    riscv64_tests = []

    for template in templates:
        # filter out additionally
        # addi with offset
        # anything with offset in label
        # call reg, offset
        # as we will have different semantics for those
        if template.template.startswith("addi") and "Off" in template.template:
            print(f"skipping {template.template} because it is exempted")
            continue
        if "[" in template.template and "Off" in template.template:
            print(f"skipping {template.template} because it is exempted")
            continue
        if template.template.startswith("call") and ("x" in template.template or "X" in template.template):
            print(f"skipping {template.template} because it is exempted")
            continue

        for _ in range(args.test_multiplier):
            if "rv32" in template.architectures:
                if template.template.split()[0] not in RV32_BLACKLIST:
                    riscv32_tests.append(template.create_entry("rv32"))
                else:
                    print(f"skipping {template.template} because it is blacklisted for rv32")

            if "rv64" in template.architectures:
                if template.template.split()[0] not in RV64_BLACKLIST:
                    riscv64_tests.append(template.create_entry("rv64"))
                else:
                    print(f"skipping {template.template} because it is blacklisted for rv64")

    base_path = args.result_file.parent
    name = args.result_file.name

    with (base_path / ("rv32_" + name)).open("w", encoding="utf-8") as f:
        for dynasm, gas, extensions in riscv32_tests:
            f.write(f"{dynasm}\t{gas}\t{extensions}\n")

    with (base_path / ("rv64_" + name)).open("w", encoding="utf-8") as f:
        for dynasm, gas, extensions in riscv64_tests:
            f.write(f"{dynasm}\t{gas}\t{extensions}\n")

FIX_GAS_MEMREF = re.compile(r"\[ *([0-9a-zA-Z-_]+) *(?:, *([0-9a-zA-Z-+_.]+) *)?\]")
FIX_LI_MEMREF = re.compile(r"(li\.44|li\.56|li\.64)")
FIX_GAS_MOP = re.compile(r"(c\.mop\.|mop\.r\.|mop\.rr\.)[nN] +([0-9]+),?")
class OpTemplate:
    def __init__(self, template, constraints, architectures, extensions):
        self.template = template
        self.gas_template = template
        self.constraints = constraints
        self.architectures = architectures
        self.extensions = extensions

        self.args = parse_template(template)

    def create_entry(self, arch):
        history = History()

        for (arg, i) in self.args:
            constraint = self.constraints[i]
            value = constraint.create_value(history, arch)
            gas = arg.emit_gas(value)
            emitted = arg.emit_dynasm(value)

            history.values.append(value)
            history.emitted.append(emitted)
            history.gas.append(gas)

        dynasm_string = SUBSTITUTION_RE.sub(lambda m: history.emitted[int(m.group(2))], self.template)
        gas_string = SUBSTITUTION_RE.sub(lambda m: history.gas[int(m.group(2))], self.gas_template)

        # rewrite gas memory references to AT&T style
        gas_string = FIX_GAS_MEMREF.sub(lambda m: f"{m.group(2) or ''}({m.group(1)})", gas_string)
        gas_string = FIX_LI_MEMREF.sub(lambda m: "li", gas_string)

        # other gas fixups
        # gas doesn't have the mop.?.n variants so rewrite those
        gas_string = FIX_GAS_MOP.sub(lambda m: f"{m.group(1)}{m.group(2)}", gas_string)

        return dynasm_string, gas_string, random.choice(self.extensions)


REGLIST_RE = re.compile(r"\{v([0-9]+)(\.[^ ]+) *\* *([1234])\}")
def reformat_reglist(m):
    start = int(m.group(1))
    format = m.group(2)
    amount = int(m.group(3))

    items = []
    for i in range(amount):
        items.append("v{}{}".format((start + i) % 32, format))

    return "{{{}}}".format(", ".join(items))

class History:
    def __init__(self):
        self.values = []
        self.emitted = []
        self.gas = []

def read_opdata_file(f):
    templates = []
    context = dict(
        R=R,
        Rdifferent=Rdifferent,
        Range=Range,
        RangeNon0=RangeNon0,
        RList=RList,
        RoundingMode=RoundingMode,
        FenceSpec=FenceSpec,
        Csr=Csr,
        FloatingPointImmediate=FloatingPointImmediate,
        StackAdjustImmediate=StackAdjustImmediate
    )

    for line in f:
        template, constraints, architectures, extensions = line.split("\t")

        template = ast.literal_eval(template)
        architectures = ast.literal_eval(architectures)
        extensions = ast.literal_eval(extensions)
        constraints = eval(constraints, context)

        templates.append(OpTemplate(template, constraints, architectures, extensions))

    return templates

SUBSTITUTION_RE = re.compile(r"<([a-zA-Z]+),([0-9]+)>")
def parse_template(template):
    matches = []
    for argty, argidx in SUBSTITUTION_RE.findall(template):
        if argty == "Imm":
            arg = Immediate()
        elif argty == "Ident":
            arg = Ident()
        elif argty == "Off":
            arg = Offset()
        elif argty in "XF":
            arg = Register(argty)
        elif argty == "RegList":
            arg = RegList()
        else:
            raise NotImplementedError(argty)
        matches.append((arg, int(argidx)))
    return matches

# Implementation details Constraints

class Constraint:
    def __init__(self):
        pass

    def create_value(self, history, arch):
        raise NotImplementedError()

class List(Constraint):
    """A constraint that allows a certain amount of options"""
    def __init__(self, *args):
        self.options = args

    def create_value(self, history, arch):
        return random.choice(self.options)

class Range(Constraint):
    """A constraint that allows any value in the specified range"""

    def __init__(self, min, max, scale):
        self.min = min
        self.max = max
        self.scale = scale

    def create_value(self, history, arch):
        return random.randrange(self.min, self.max, self.scale)

class RangeNon0(Range):
    """A constraint that allows any value in the specified range, but not 0"""

    def create_value(self, history, arch):
        val = random.randrange(self.min, self.max, self.scale)
        while val == 0:
            val = random.randrange(self.min, self.max, self.scale)
        return val

class R(Constraint):
    """A constraint that allows a certain range of registers, as specified by a mask"""

    def __init__(self, mask):
        self.mask = mask

    def create_value(self, history, arch):
        options = []
        for i in range(32):
            if self.mask & (1 << i) != 0:
                options.append(i)

        return random.choice(options)

class Rdifferent(R):
    """A register, but not the same as the previous register"""

    def create_value(self, history, arch):
        value = super().create_value(history, arch)
        while value in history.values:
            value = super().create_value(history, arch)

        return value

class RList(Constraint):
    """A constraint that produces values for a register list"""

    def create_value(self, history, arch):
        return random.randrange(4, 16)

class RoundingMode(Constraint):
    """A constraint that produces values for a rounding mode"""

    def create_value(self, history, arch):
        return random.choice(("rne", "rtz", "rdn", "rup", "rmm", "dyn"))

class FenceSpec(Constraint):
    """A constraint that produces values for a fence specification"""

    def create_value(self, history, arch):
        return random.choice((
            "w", "r", "rw",
            "o", "ow", "or", "orw",
            "i", "iw", "ir", "irw", 
            "io", "iow", "ior", "iorw"
        ))

class Csr(Constraint):
    """A constraint that produces values for a csr"""

    def create_value(self, history, arch):
        if random.choice((False, True)):
            return random.choice(CSR_MAP)[0]
        else:
            return random.choice(CSR_MAP)[1]

class FloatingPointImmediate(Constraint):
    """A constraint that produces values for a floating point immediate"""

    def create_value(self, history, arch):
        return random.choice((
            "-1.0", "min", "1.52587890625e-05", "3.0517578125e-05",
            "3.90625e-03", "7.8125e-03", "0.0625", "0.125",
            "0.25", "0.3125", "0.375", "0.4375",
            "0.5", "0.625", "0.75", "0.875",
            "1.0", "1.25", "1.5", "1.75",
            "2.0", "2.5", "3.0", "4.0",
            "8.0", "16.0", "128.0", "256.0",
            "32768.0", "65536.0", "inf", "nan"
        ))

class StackAdjustImmediate(Constraint):

    def __init__(self, negated):
        self.negated = negated

    def create_value(self, history, arch):
        reglist_value = history.values[-1]
        if reglist_value == 15:
            reglist_value = 16

        if arch == "rv32":
            stack_adj = reglist_value // 4 * 16

        elif arch == "rv64":
            stack_adj = reglist_value // 2 * 16 - 16

        else:
            raise ValueError("Unknown architecture")

        extra = random.choice((0, 16, 32, 48))

        if self.negated:
            return -(stack_adj + extra)
        else:
            return stack_adj + extra

# Implementation details Args

class Arg:
    def __init__(self):
        pass

    def emit_gas(self, value):
        raise NotImplementedError()

    def emit_dynasm(self, value):
        raise NotImplementedError()

class Register(Arg):
    def __init__(self, family):
        self.family = family

    def emit_gas(self, value):
        if self.family == "X":
            return f"x{value}"
        elif self.family == "F":
            return f"f{value}"
        else:
            raise NotImplementedError(self.family)

    def emit_dynasm(self, value, allow_dynamic=True):
        # randomly choose dynamic vs static notation
        if random.choice((True, False)) or not allow_dynamic:
            return self.emit_gas(value)
        else:
            return f"{self.family}({value})"

class Immediate(Arg):
    def emit_gas(self, value):
        return f"{value}"

    def emit_dynasm(self, value):
        return f"{value}"

class Offset(Immediate):
    def emit_gas(self, value):
        if value >= 0:
            return ".+" + super().emit_gas(value)
        else:
            return "." + super().emit_gas(value)

class Ident(Immediate):
    pass

class RegList(Arg):
    """emits a reglist, value is between 4 and 15.
    this gets mapped to {ra}, {ra, s0}, ... {ra, s0-s12} """
    def emit_gas(self, value):
        return (
            "{ra}",
            "{ra, s0}",
            "{ra, s0-s1}",
            "{ra, s0-s2}",
            "{ra, s0-s3}",
            "{ra, s0-s4}",
            "{ra, s0-s5}",
            "{ra, s0-s6}",
            "{ra, s0-s7}",
            "{ra, s0-s8}",
            "{ra, s0-s9}",
            "{ra, s0-s11}",
        )[value - 4]

    def emit_dynasm(self, value):
        if random.choice((True, False)):
            return self.emit_gas(value)
        else:
            if value == 15:
                value = 12
            else:
                value = value - 4
            return f"{{ra; {value}}}"

# CSR mapping. just some of the basic ones that are always available
CSR_MAP = [
    ("fflags", 0x001),
    ("frm", 0x002),
    ("fcsr", 0x003),
    ("vstart", 0x008),
    ("vxsat", 0x009),
    ("vxrm", 0x00A),
    ("vcsr", 0x00F),
    ("ssp", 0x011),
    ("seed", 0x015),
    ("jvt", 0x017),
    ("cycle", 0xC00),
    ("time", 0xC01),
    ("instret", 0xC02),
    ("hpmcounter3", 0xC03),
    ("hpmcounter4", 0xC04),
    ("hpmcounter5", 0xC05),
    ("hpmcounter6", 0xC06),
    ("hpmcounter7", 0xC07),
    ("hpmcounter8", 0xC08),
    ("hpmcounter9", 0xC09),
    ("hpmcounter10", 0xC0A),
    ("hpmcounter11", 0xC0B),
    ("hpmcounter12", 0xC0C),
    ("hpmcounter13", 0xC0D),
    ("hpmcounter14", 0xC0E),
    ("hpmcounter15", 0xC0F),
    ("hpmcounter16", 0xC10),
    ("hpmcounter17", 0xC11),
    ("hpmcounter18", 0xC12),
    ("hpmcounter19", 0xC13),
    ("hpmcounter20", 0xC14),
    ("hpmcounter21", 0xC15),
    ("hpmcounter22", 0xC16),
    ("hpmcounter23", 0xC17),
    ("hpmcounter24", 0xC18),
    ("hpmcounter25", 0xC19),
    ("hpmcounter26", 0xC1A),
    ("hpmcounter27", 0xC1B),
    ("hpmcounter28", 0xC1C),
    ("hpmcounter29", 0xC1D),
    ("hpmcounter30", 0xC1E),
    ("hpmcounter31", 0xC1F),
]


if __name__ == '__main__':
    main()
