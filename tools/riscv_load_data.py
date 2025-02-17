# This python file loads the instruction encoding data contained in riscv_data/riscv-opcodes, making
# it available for further processing.
from pathlib import Path
import re
from ast import literal_eval
import csv


# parse results

class Bitrange:
    def __init__(self, bottom, top, value):
        self.bottom = bottom
        self.top = top
        self.value = value

    def __repr__(self):
        if self.top == self.bottom:
            return f"{self.bottom}={self.value}"
        else:
            return f"{self.top}..{self.bottom}={self.value}"


class Encoding:
    def __init__(self, bitranges, fields):
        self.bitranges = bitranges
        self.fields = fields

        self.bitsize = None
        self.template = 0
        self.mask = 0

    def validate_and_process(self, arg_lut_data: {str: (int, int)}):
        # validate bitranges aren't overlapping and construct the mask/template from them
        for bitrange in self.bitranges:
            mask = ((2 << (bitrange.top - bitrange.bottom)) - 1) << bitrange.bottom
            assert mask & self.mask == 0, "bad encoding data, overlapping masks found"

            self.mask |= mask
            self.template |= bitrange.value << bitrange.bottom

        fieldmask = 0
        for field in self.fields:
            if "=" in field:
                fieldname = field.split("=")[0]
            else:
                fieldname = field

            bottom, top = arg_lut_data.get_arg_bounds(fieldname)
            mask = ((2 << (top - bottom)) - 1) << bottom

            assert mask & fieldmask == 0, "bad encoding data, overlapping fields found"
            fieldmask |= mask

        assert fieldmask & self.mask == 0, f"overlapping fields and template: {fieldmask:X} & {self.mask:X}"

        totalmask = fieldmask | self.mask

        if totalmask == 0xFFFF:
            self.bitsize = 16
        elif totalmask == 0xFFFFFFFF:
            self.bitsize = 32
        else:
            raise AssertionError(f"not all bits are accounted for: {fieldmask:X} & {self.mask:X}")

class Instruction:
    def __init__(self, name):
        self.name = name
        self.extensions = []


# represents an instruction that is an alias of another instruction
class ImportedInstruction(Instruction):
    def __init__(self, name, parent_extension):
        super().__init__(name)
        self.parent_extension = parent_extension


# represents a fully specified instruction encoding
class ConcreteInstruction(Instruction):
    def __init__(self, name, encoding):
        super().__init__(name)
        self.encoding = encoding

    def copy_with_extensions(self, extensions):
        instruction = ConcreteInstruction(self.name, self.encoding)
        instruction.extensions = extensions
        return instruction


# represents a fully specified instruction encoding, that is an alias
# of a more general instruction
class PseudoInstruction(ConcreteInstruction):
    def __init__(self, name, encoding, parent_extension, parent_name):
        super().__init__(name, encoding)
        self.parent_extension = parent_extension
        self.parent_name = parent_name

    def copy_with_extensions(self, extensions):
        instruction = PseudoInstruction(self.name, self.encoding, self.parent_extension, self.parent_name)
        instruction.extensions = extensions
        return instruction


# lexing results

class Token:
    def __init__(self, raw: str, line: int, column: int):
        self.raw = raw
        self.line = line
        self.column = column

class Newline(Token):
    pass

class Symbol(Token):
    REGEX = re.compile(r"(\$|::|\.\.|=)")
    pass

class Number(Token):
    REGEX = re.compile(r"((:?0[xbXB]|[0-9])[0-9A-Fa-f]*)")
    def __init__(self, raw: str, line: int, column: int):
        super().__init__(literal_eval(raw), line, column)

class Name(Token):
    REGEX = re.compile(r"([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z0-9_]+)*)")

class End(Token):
    pass


# parsing implementation


class ParseError(Exception):
    pass


def tokenize(buffer) -> [Token]:
    WHITESPACE = re.compile(r"[^\S\n]+")
    COMMENT = re.compile(r"(#[^\n]*)")

    i = 0
    line = 1
    column = 0

    # advance whitespace
    if match := WHITESPACE.match(buffer, i):
        i += len(match.group(0))
        column += len(match.group(0))

    # until we hit the end of the buffer
    while i != len(buffer):
        # try lexing all possible token types
        if buffer[i] == "\n":
            yield Newline("\n", line, column)

            i += 1
            line += 1
            column = 0

        elif match := Symbol.REGEX.match(buffer, i):
            raw = match.group(1)
            yield Symbol(raw, line, column)
            i += len(raw)
            column += len(raw)

        elif match := Number.REGEX.match(buffer, i):
            raw = match.group(1)
            yield Number(raw, line, column)
            i += len(raw)
            column += len(raw)

        elif match := Name.REGEX.match(buffer, i):
            raw = match.group(1)
            yield Name(raw, line, column)
            i += len(raw)
            column += len(raw)

        elif match := COMMENT.match(buffer, i):
            raw = match.group(1)
            i += len(raw)
            column += len(raw)

        else:
            raise ParseError(f"Could not tokenize text at line {line}, column {column}: '{buffer[i:i+10]}'...")

        # and advance whitespace
        if match := WHITESPACE.match(buffer, i):
            i += len(match.group(0))
            column += len(match.group(0))

    yield End(None, line, column)


class Parser:
    # simple recursive descent parser for parsing the opcode encoding definitions.

    def __init__(self, tokens):
        # reverse the list, so we can cheaply pop from the end.
        self.tokens = iter(tokens)
        self.current = next(tokens)

    def advance(self):
        self.current = next(self.tokens)

    def unexpected(self):
        raise ParseError(f"Unexpected token {self.current.raw} at line {self.current.line}, column {self.current.column}")

    def expect(self, type, value=None):
        if not isinstance(self.current, type) or (value is not None and value != self.current.raw):
            raise ParseError(f"Unexpected token {self.current.raw} at line {self.current.line}, column {self.current.column}.\nExpected {type.__name__}")

        rv = self.current.raw
        self.advance()
        return rv

    def parse(self) -> [Instruction]:
        instructions = []

        while not isinstance(self.current, End):
            if isinstance(self.current, Newline):
                self.advance()

            elif self.current.raw == "$":
                self.advance()

                if self.current.raw == "pseudo_op":
                    self.advance()

                    parent_extension = self.expect(Name)
                    self.expect(Symbol, "::")
                    parent_name = self.expect(Name)

                    name = self.expect(Name)

                    encoding = self.parse_encoding()

                    instructions.append(PseudoInstruction(name, encoding, parent_extension, parent_name))

                elif self.current.raw == "import":
                    self.advance()

                    parent_extension = self.expect(Name)
                    self.expect(Symbol, "::")
                    name = self.expect(Name)

                    instructions.append(ImportedInstruction(name, parent_extension))

                else:
                    self.unexpected()

            else:
                name = self.expect(Name)

                encoding = self.parse_encoding()

                instructions.append(ConcreteInstruction(name, encoding))

        return instructions

    def parse_encoding(self) -> Encoding:
        fields = []
        bitranges = []

        while not (isinstance(self.current, Newline) or isinstance(self.current, End)):
            if isinstance(self.current, Name):
                field = self.current.raw
                self.advance()

                if self.current.raw == "=":
                    self.advance()
                    alias = self.expect(Name)

                    fields.append(f"{field}={alias}")

                else:
                    fields.append(field)

            else:
                offset = self.expect(Number)
                top = offset

                if self.current.raw == "..":
                    self.advance()

                    top = offset
                    offset = self.expect(Number)

                self.expect(Symbol, "=")

                value = self.expect(Number)

                bitranges.append(Bitrange(offset, top, value))

        return Encoding(bitranges, fields)

    @classmethod
    def parse_file(cls, path: Path) -> [Instruction]:
        with path.open("r", encoding="utf-8") as f:
            data = f.read()

        try:
            tokens = tokenize(data)
            instructions = cls(tokens).parse()

        except ParseError as e:
            raise ParseError(f"in {path}:\n{e.args[0]}")

        return instructions


class RISCVOpdataFile:
    def __init__(self, path, verified=True):
        self.name = path.name
        self.path = path
        self.verified = verified

        self.instructions = Parser.parse_file(path)


class ArgLutFile:
    def __init__(self, path: Path):
        self.path = path

        with open(self.path, encoding="utf-8") as f:
            csv_reader = csv.reader(f, skipinitialspace=True)
            self.data = {row[0]: (int(row[2]), int(row[1])) for row in csv_reader}

        # for mop
        self.data["mop_r_t_30"] = (30, 30)
        self.data["mop_r_t_27_26"] = (26, 27)
        self.data["mop_r_t_21_20"] = (20, 21)
        self.data["mop_rr_t_30"] = (30, 30)
        self.data["mop_rr_t_27_26"] = (26, 27)
        self.data["c_mop_t"] = (8, 10)

    # returns a tuple of (bottom, top)
    def get_arg_bounds(self, arg: str) -> (int, int):
        return self.data[arg]


# Class wrapping the riscv-opcodes/extensions archive
class RISCVExtensionsArchive:
    def __init__(self, data_folder: Path, arg_lut: ArgLutFile):
        self.base = Path(data_folder)

        verified = [i for i in self.base.iterdir() if i.name != "unratified"]
        unverified = [i for i in (self.base / "unratified").iterdir()]

        self.extensions = []
        self.extensions.extend(RISCVOpdataFile(i, True) for i in verified)
        self.extensions.extend(RISCVOpdataFile(i, False) for i in unverified)

        # resolve imports
        for extension in self.extensions:
            for instruction in extension.instructions:
                if isinstance(instruction, ImportedInstruction):
                    # yes this is O(N)...
                    parent_extension = next(i for i in self.extensions if i.name == instruction.parent_extension)
                    parent_instruction = next(i for i in parent_extension.instructions if i.name == instruction.name)

                    # link to the target instruction in the import instruction
                    instruction.target = parent_instruction

                else:
                    try:
                        instruction.encoding.validate_and_process(arg_lut)
                    except Exception as e:
                        raise Exception(f"in {extension.name}, {instruction.name}:\n{'\n'.join(e.args)}") from e


    def create_instruction_list(self, target: str, extensions: [str] = "i", include_unverified=False) -> [(Instruction, [str])]:
        assert target in ("rv32", "rv64")

        # flow here: we first select all files that contain the instructions we want to read
        # we then read all instructions from them, and aggregate them in one big set. any imported instructions
        # are replaced by their target
        # finally, we discard any duplicate instructions (as a result of ones appearing in multiple extensions)
        # we also filter any mention of them.

        extensions = set(extensions)

        # select all the files we'll be using
        picked_files = []
        for extension in self.extensions:
            chunks = extension.name.split('_')
            if chunks[0] != "rv" and chunks[0] != target:
                continue

            if not (extension.verified or include_unverified):
                continue

            if not all(i in extensions for i in chunks[1:]):
                continue

            # reduce the extension identifier to just the extension (by dropping the rv[32/64] part)
            label = "_".join(sorted(chunks[1:]))

            picked_files.append((extension, label))

        # create a list of instruction, required_extentions
        instructions = []
        for extension, label in picked_files:
            for instruction in extension.instructions:
                if isinstance(instruction, ImportedInstruction):
                    instructions.append((instruction.target, label))
                else:
                    instructions.append((instruction, label))

        # deduplicate instructions that are in multiple extension sets (which are the same object, thanks to our import handling)
        unique_instructions = {}
        for instruction, label in instructions:
            if instruction not in unique_instructions:
                unique_instructions[instruction] = [label]
            else:
                unique_instructions[instruction].append(label)

        # copy datastructures and set features 
        unique_instructions = [ins.copy_with_extensions(sorted(ext)) for ins, ext in unique_instructions.items()]

        # sort it all
        unique_instructions.sort(key=lambda i: (i.extensions, i.name))

        return unique_instructions

    # return an iterator of the known extensions.
    # target can be rv32 or rv64
    def get_available_extensions(self, target, include_unverified=False):
        assert target in ("rv32", "rv64")
        extensions = set()

        for extension in self.extensions:
            if include_unverified or extension.verified:
                chunks = extension.name.split('_')
                if chunks[0] == "rv" or chunks[0] == target:
                    extensions.update(chunks[1:])

        extensions = list(extensions)
        extensions.sort()

        return extensions



RISCV_OPCODES_PATH = Path(__file__).parent / "riscv_data" / "riscv-opcodes"
ARCHIVE_PATH = RISCV_OPCODES_PATH / "extensions"
ARG_LUT = ArgLutFile(RISCV_OPCODES_PATH / "arg_lut.csv")
ARCHIVE = RISCVExtensionsArchive(ARCHIVE_PATH, ARG_LUT)

def get_arg_lut():
    return ARG_LUT

def get_archive():
    return ARCHIVE



def main():
    archive = get_archive()

    rv32_extensions = archive.get_available_extensions("rv32")
    rv64_extensions = archive.get_available_extensions("rv64")

    # right now we're not dealing with the vector or privileged instruction sets
    unused_extensions = ("v", "s", "h", "zv", "aliases")

    rv32_extensions = [i for i in rv32_extensions if not i.startswith(unused_extensions)]
    rv64_extensions = [i for i in rv64_extensions if not i.startswith(unused_extensions)]

    print(rv32_extensions)
    print(rv64_extensions)

    instructions = archive.create_instruction_list("rv64", rv64_extensions)
    with Path("testfile.txt").open("w", encoding="utf-8") as f:
        for instruction in instructions:
            if instruction.encoding.bitsize == 32:
                f.write(f"{instruction.name} ({', '.join(instruction.extensions)}) 0x{instruction.encoding.template:08X} {' '.join(instruction.encoding.fields)}\n")
            else:
                f.write(f"{instruction.name} ({', '.join(instruction.extensions)}) 0x{instruction.encoding.template:04X} {' '.join(instruction.encoding.fields)}\n")

if __name__ == '__main__':
    main()
