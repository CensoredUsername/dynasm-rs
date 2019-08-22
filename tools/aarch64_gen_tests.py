
import ast
import random
import re

def main():
    import sys
    infile = sys.argv[1]
    attempts = int(sys.argv[2])
    outfile = sys.argv[3]
    with open(infile, "r", encoding="utf-8") as f:
        templates = read_opdata_file(f)

    buf = []
    for template in templates:
        for _ in range(attempts):
            buf.append(template.create_entry())

    with open(outfile, "w", encoding="utf-8") as f:
        for dynasm, gas in buf:
            f.write("{}\t{}\n".format(dynasm, gas))


OPTIONAL_RE = re.compile(r"<.*?>")
FIX_GAS_RE = re.compile(r"\.([BHSDQ])([0-9]+)")
HAS_WREG_BEFORE_EXTEND = re.compile(r"[wW][^ ]+ <[ ,]")
class OpTemplate:
    def __init__(self, template, constraints):
        self.template = template
        self.gas_template = FIX_GAS_RE.sub(lambda m: ".{}{}".format(m.group(2), m.group(1)), template)
        self.constraints = constraints
        self.args = parse_template(template)

    def create_entry(self, keep_optionals=False):
        history = History()
        for (arg, i) in self.args:
            constraint = self.constraints[i]
            value = constraint.create_value(history)
            gas = arg.emit_gas(value)
            emitted = arg.emit_dynasm(value)
            if isinstance(constraint, RNext):
                if "(" in history.emitted[-1]:
                    emitted = arg.emit_dynasm(31, False)
                else:
                    emitted = arg.emit_dynasm(value, False)

            history.values.append(value)
            history.emitted.append(emitted)
            history.gas.append(gas)

        dynasm_string = SUBSTITUTION_RE.sub(lambda m: history.emitted[int(m.group(2))], self.template)
        gas_string = SUBSTITUTION_RE.sub(lambda m: history.gas[int(m.group(2))], self.gas_template)

        if not keep_optionals and not HAS_WREG_BEFORE_EXTEND.search(gas_string):
            while "<" in dynasm_string:
                if random.choice((True, False)):
                    break

                dynasm_string = OPTIONAL_RE.sub("", dynasm_string)
                gas_string    = OPTIONAL_RE.sub("", gas_string)

        dynasm_string = dynasm_string.replace("<", "").replace(">", "")
        gas_string = gas_string.replace("<", "").replace(">", "")

        gas_string = gas_string.replace("mov.inverted", "mov")
        gas_string = gas_string.replace("mov.logical", "mov")

        gas_string = REGLIST_RE.sub(reformat_reglist, gas_string)

        return dynasm_string, gas_string


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
        List=List,
        ModWX=ModWX,
        Range=Range,
        Range2=Range2,
        R=R,
        RNext=RNext,
        Special=Special,
    )

    for line in f:
        template, constraints = line.split("\t")
        template = ast.literal_eval(template)
        constraints = eval(constraints, context)
        templates.append(OpTemplate(template, constraints))

    return templates

SUBSTITUTION_RE = re.compile(r"<([a-zA-Z]+),([0-9]+)>")
def parse_template(template):
    matches = []
    for argty, argidx in SUBSTITUTION_RE.findall(template):
        if argty == "Imm":
            arg = Immediate()
        elif argty == "Ident":
            arg = Ident()
        elif argty == "Mod":
            arg = Modifier()
        elif argty == "Off":
            arg = Offset()
        elif argty in "WXBHSDQV" or argty in ("WSP", "XSP", "WX"):
            arg = Register(argty)
        else:
            raise NotImplementedError(argty)
        matches.append((arg, int(argidx)))
    return matches

# Implementation details Constraints

class Constraint:
    def __init__(self):
        pass

    def create_value(self, history):
        raise NotImplementedError()

class List(Constraint):
    """A constraint that allows a certain amount of options"""
    def __init__(self, *args):
        self.options = args

    def create_value(self, history):
        return random.choice(self.options)

class ModWX(Constraint):
    def create_value(self, history):
        prev = history.emitted[-1]
        if prev.startswith("X") or prev.startswith("x"):
            return random.choice(("LSL", "SXTX"))
        else:
            return random.choice(("UXTW", "SXTW"))

class Range(Constraint):
    """A constraint that allows any value in the specified range"""
    def __init__(self, min, max, scale):
        self.min = min
        self.max = max
        self.scale = scale

    def create_value(self, history):
        return random.randrange(self.min, self.max, self.scale)

class Range2(Range):
    """A special range constraint"""

    def create_value(self, history):
        prev = history.values[-1]
        return random.randrange(1, self.max - prev, self.scale)

class R(Constraint):
    """A constraint that allows a certain range of registers"""
    def __init__(self, count, scale=1):
        self.count = count
        self.scale = scale

    def create_value(self, history):
        return random.randrange(0, self.count, self.scale)

class RNext(Constraint):
    """A constraint that only allows the register after the previous arg"""
    def create_value(self, history):
        return (history.values[-1] + 1) % 32
        
class Special(Constraint):
    """A constraint that can emit various special integer encoding formats"""
    def __init__(self, type):
        self.type = type

    def create_value(self, history):
        if self.type == "wide_w":
            return make_wide_integer(False)
        elif self.type == "wide_x":
            return make_wide_integer(True)
        elif self.type == "inverted_w":
            return make_wide_integer(False) ^ 0xFFFFFFFF
        elif self.type == "inverted_x":
            return make_wide_integer(True) ^ 0xFFFFFFFFFFFFFFFF
        elif self.type == "logical_w":
            return make_logical_imm(False)
        elif self.type == "logical_x":
            return make_logical_imm(True)
        elif self.type == "float":
            return make_float_imm()
        elif self.type == "stretched":
            return make_stretched_imm()
        else:
            raise NotImplementedError(self.type)

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
        if self.family == "WX":
            self.family = random.choice("WX")

        if self.family == "W":
            if value == 31:
                return "wzr"
            return "w{}".format(value)
        elif self.family == "X":
            if value == 31:
                return "xzr"
            return "x{}".format(value)
        elif self.family == "WSP":
            if value == 31:
                return "wsp"
            return "w{}".format(value)
        elif self.family == "XSP":
            if value == 31:
                return "sp"
            return "x{}".format(value)
        elif self.family in "BHSDQV":
            return "{}{}".format(self.family.lower(), value)
        else:
            raise NotImplementedError(self.family)

    def emit_dynasm(self, value, allow_dynamic=True):
        # randomly choose dynamic vs static notation
        if random.choice((True, False)) or not allow_dynamic or ('SP' in self.family and value != 31):
            return self.emit_gas(value)
        else:
            if self.family == "WX":
                self.family = random.choice("WX")

            return "{}({})".format(self.family, value)

class Modifier(Arg):
    def emit_gas(self, value):
        return "{}".format(value)

    def emit_dynasm(self, value):
        return "{}".format(value)

class Immediate(Arg):
    def emit_gas(self, value):
        return "{}".format(value)

    def emit_dynasm(self, value):
        return "{}".format(value)

class Offset(Immediate):
    pass

class Ident(Arg):
    def emit_gas(self, value):
        return value.lower()

    def emit_dynasm(self, value):
        return value


def make_wide_integer(bit64):
    return random.randrange(0, 1<<16) << random.randrange(0, 64 if bit64 else 32, 16)

def make_float_imm():
    sign = random.randrange(0, 2)
    mantissa = random.randrange(0, 16)
    exponent = random.randrange(-3, 5)
    return ((-1.0) ** sign) * (2.0 ** exponent) * ((16.0 + mantissa) / 16.0)

def make_logical_imm(bit64):
    element_size = random.choice([2, 4, 8, 16, 32, 64] if bit64 else [2, 4, 8, 16, 32])
    ones = random.randrange(1, element_size)
    rotation = random.randrange(0, element_size)

    element = [0] * (element_size - ones) + [1] * ones
    l = element * ((64 if bit64 else 32) // element_size)
    rotated = l[rotation:] + l[:rotation]

    return int("".join(map(str, rotated)), 2)

def make_stretched_imm():
    imm = random.randrange(0, 256)
    imm |= (imm & 0x00000000000000F0) << 28
    imm |= (imm & 0x0000000C0000000C) << 14
    imm |= (imm & 0x0002000200020002) << 7
    imm &= 0x0101010101010101;
    imm |= imm << 1
    imm |= imm << 2
    imm |= imm << 4
    return imm

if __name__ == '__main__':
    main()
