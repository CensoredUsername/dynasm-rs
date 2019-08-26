import xml.etree.ElementTree as ET
import os
import sys
import re
import copy

class IClass:
    def __init__(self, mnemonic, fields, bits, name, template, instrclass, arch_variant):
        self.mnemonic = mnemonic
        self.fields = fields
        self.bits = bits
        self.name = name
        self.template = template
        self.instrclass = instrclass
        self.arch_variant = arch_variant

    def __repr__(self):
        s = "{} ({}) => 0b{}".format(self.mnemonic, self.instrclass, self.bits)
        for name, width, lobit in self.fields:
            s += " | {}({}) << {}".format(name, width, lobit)
        s += "\n// " + self.name
        s += "\n// " + self.template
        return s

def analyse_file(fname):
    tree = ET.parse(fname)
    root = tree.getroot()

    heading = root.find("heading")
    if heading is not None:
        heading = heading.text

    for iclass in root.iter("iclass"):

        # figure out what variant this instruction is of
        variants = iclass.find("arch_variants")
        if variants:
            arch_variant = [i.get("name") for i in variants]
            assert len(arch_variant) == 1
            arch_variant = arch_variant[0]
        else:
            arch_variant = None

        bits = ["x"] * 32
        fields = [] # (name, hibit, len)

        # figure out the base template
        regdiagram = iclass.find("regdiagram")
        for box in regdiagram.iter("box"):
            hibit = int(box.get("hibit"))
            width = int(box.get("width", "1"))
            name = box.get("name")

            i = hibit
            for c in box.iter("c"):
                width = int(c.get("colspan", "1"))

                if width == 1 and c.text is not None:
                    bit = c.text.lstrip("(").rstrip(")")
                    bits[i] = bit

                i -= width

            if name is not None:
                fields.append((name, hibit, width))

        # find all the different encodings belonging to this instruction
        for encoding in iclass.findall("./encoding"):
            # what mnemonic does this encoding variant use?
            mnemonic = None
            for docvar in encoding.iter("docvar"):
                if docvar.get("key") == "mnemonic":
                    if mnemonic is None:
                        mnemonic = docvar.get("value")
                elif docvar.get("key") == "alias_mnemonic":
                    mnemonic = docvar.get("value")

            if mnemonic is None:
                break # malformatted input

            # figure out any encoding-specific bits
            enc_bits = bits[:]
            for box in encoding.iter("box"):
                hibit = int(box.get("hibit"))
                width = int(box.get("width", "1"))

                i = hibit
                for c in box.iter("c"):
                    width = int(c.get("colspan", "1"))

                    if width == 1 and c.text is not None:
                        bit = c.text.lstrip("(").rstrip(")")
                        enc_bits[i] = bit

                    i -= width

            # trim declared fields that have been filled up completely now
            enc_fields = []
            for field in fields:
                name, hibit, width = field

                bits_known = True
                i = hibit
                for _ in range(width):
                    if enc_bits[i] != "0" and enc_bits[i] != "1":
                        bits_known = False

                    i -= 1

                if not bits_known:
                    enc_fields.append(field)

            # format bits
            enc_bits = "".join(str(i) for i in reversed(enc_bits))

            # figure out the instruction class this instruction belongs to
            try:
                instrclass = next(i for i in encoding.iter("docvar") if i.get("key") == "instr-class").get("value")
            except StopIteration:
                instrclass = "general"

            # get the assembly templates and filter out any aliasing declarations
            templates = ["".join(i.itertext()) for i in encoding.findall("./asmtemplate")]
            assert len(templates) == 1
            template = templates[0]

            template = template[len(mnemonic):].replace(" ", "")

            enc_fields = tuple((name, width, hibit - width + 1) for name, hibit, width in enc_fields)

            # handle widening/narrowing prefix
            if template.startswith("{2}"):
                template = template[3:]
                # This is normally implemented via a bitfield in bit 30 called Q, which is 1 for the 2 variant.
                # assert such a field exists
                assert ("Q", 1, 30) in enc_fields
                enc_fields = tuple(i for i in enc_fields if i != ("Q", 1, 30))

                yield IClass(mnemonic + "2", enc_fields, enc_bits[0] + "1" + enc_bits[2:], heading, template, instrclass, arch_variant)
                yield IClass(mnemonic      , enc_fields, enc_bits[0] + "0" + enc_bits[2:], heading, template, instrclass, arch_variant)

            else:
                yield IClass(mnemonic, enc_fields, enc_bits, heading, template, instrclass, arch_variant)


def read_op_defs(opcode_folder):
    import os

    files = os.listdir(opcode_folder)

    ops = []
    for fname in files:
        if fname.endswith(".xml") and not fname.startswith("onebigfile"):
            try:
                ops.extend(analyse_file(os.path.join(opcode_folder, fname)))
            except Exception as e:
                print(fname, e)

    return ops

def determine_instr_classes(ops):
    instrclasses = set()
    for i in ops:
        instrclasses.add(i.instrclass)
    return instrclasses

def filter_instr_classes(ops, wanted_classes):
    return [op for op in ops if op.instrclass in wanted_classes]

def filter_arch_versions(ops, arch_variants):
    return [op for op in ops if op.arch_variant is None or op.arch_variant in arch_variants]

def group_instr_aliases(ops):
    mnemonics = {}
    for op in ops:
        if op.mnemonic not in mnemonics:
            mnemonics[op.mnemonic] = []
        mnemonics[op.mnemonic].append(op)

    ops = [(m, ops) for m, ops in mnemonics.items()]

    ops.sort(key=lambda item: item[0])

    return ops

def group_templates(ops):
    templates = {}

    for m, group in ops:
        for op in group:
            template = op.template
            if template.startswith(m):
                template = template[len(m):]

            entry = (
                template.replace(" ", ""),
                tuple(sorted(op.fields))
            )

            if entry not in templates:
                templates[entry] = []
            templates[entry].append(m)

    templates = list((template, field, mnemonics) for (template, field), mnemonics in templates.items())
    templates.sort()

    return templates

def assign_matchers(ops):
    import template_map
    import copy

    for m, group in ops:
        newgroup = []
        for op in group:
            template = op.template
            if template.startswith(m):
                template = template[len(m):]
            template = template.replace(" ", "")

            for matcher in template_map.TEMPLATE_MAPPING.get(template, ["Unimp"]):
                matched = copy.copy(op)
                matched.matcher = matcher
                newgroup.append(matched)

        group[:] = newgroup

def merge_matchers(ops):
    for m, group in ops:
        # merge exactly matching matchers
        newgroup = []
        matchers = {}
        for op in group:
            if op.matcher in matchers:
                continue
            newgroup.append(op)
            matchers[op.matcher] = op.bits

        # merge matches whose only difference is the size flag
        discards = set()
        for op in newgroup:
            test = re.sub("\\bW", "X", op.matcher)
            if "W" in op.matcher and test in matchers:
                discards.add(test)
                op.matcher = re.sub("\\bW", "WX", op.matcher)

                short_variant = op.bits
                long_variant = matchers[test]
                for i in range(32):
                    if short_variant[i] == "0" and long_variant[i] == "1":
                        op.fields.append(('sf', 31 - i, 1))
                    elif short_variant[i] == "1" and long_variant[i] == "0":
                        op.fields.append(('isf', 31 - i, 1))

        group[:] = [op for op in newgroup if op.matcher not in discards]


def emit_opmap(ops, f):
    for m, group in ops:
        f.write("\"{}\" = [\n".format(m.lower()))

        write_names = len({op.name for op in group}) > 1

        last_name = None
        for op in group:
            if write_names and op.name != last_name:
                last_name = op.name
                f.write("    // {}\n".format(op.name))

            bits = op.bits.replace("x", "0").replace("N", "0").replace("Z", "0").replace("z", "0")
            bits = bits[0:8] + "_" + bits[8:16] + "_" + bits[16:24] + "_" + bits[24:32]

            f.write("    0b{} = [{}] => [{}];\n".format(bits, op.matcher, op.processor))


        f.write("]\n")

def generate_translation_files(ops, instrclass, f):
    relevant = []
    for m, group in ops:
        for op in group:
            if op.instrclass != instrclass:
                continue
            relevant.append(op)

    templates = {}
    for op in relevant:
        entry = (op.template, op.fields)
        if entry not in templates:
            templates[entry] = []
        templates[entry].append(op.mnemonic)

    templates = [(i, j, k) for (i, j), k in templates.items()]
    templates.sort()

    for template, fields, ops in templates:
        f.write("""
tlentry({},
    {}, {},
    matcher   = {},
    processor = {},
)
""".format(ops, repr(template), fields, repr("Unimp"), repr("Unimp"))
        )

class TranslationEntry:
    def __init__(self, ops, template, fields, matcher_processors, forget, names, bits, priority):
        self.ops = ops
        self.template = template
        self.fields = fields
        self.matcher_processors = matcher_processors
        self.forget = forget
        self.names = names
        self.bits = bits
        self.priority = priority

    @classmethod
    def entry(cls, ops, template, fields, matcher=None, processor=None, matchers=None, processors=None, forget=False, names=None, bits=None, priority=0):
        m = []
        p = []
        if matcher is not None:
            m.append(matcher)
        if processor is not None:
            p.append(processor)
        if matchers is not None:
            m.extend(matchers)
        if processors is not None:
            p.extend(processors)

        assert not (names and bits), "both names and bits set"

        if not forget:
            assert m, "No matchers"
            assert p, "No processors"
            assert len(m) == len(p), "Amount of matchers and processors does not match"
            if names:
                assert len(names) == len(m), "wrong amount of names"
        return cls(ops, template, fields, list(zip(m, p)), forget, names, bits, priority)

class TranslationMap:
    def __init__(self):
        self.map = []
        self.table = {}

    def load_file(self, f, name):
        def entry(*args, **kwargs):
            self.map.append(TranslationEntry.entry(*args, **kwargs))

        g = {"tlentry": entry}
        code = f.read()
        try:
            exec(code, g) # dirty but works
        except Exception as e:
            raise Exception("In {}".format(name)) from e

    def build_lookup_table(self):
        self.table = {}
        for entry in self.map:
            for op in entry.ops:
                if op not in self.table:
                    self.table[op] = {}
                if (entry.template, entry.fields) in self.table[op]:
                    raise Exception("Duplicate entry, found {}, {} for {}".format(entry.template, entry.fields, op))
                self.table[op][(entry.template, entry.fields)] = entry

    def lookup_iclass(self, op):
        return self.table[op.mnemonic][(op.template, op.fields)]

def tl_assign_matchers(ops, map):
    for m, group in ops:
        newgroup = []
        priority = []
        for op in group:

            tlentry = map.lookup_iclass(op)
            if tlentry.forget:
                continue 
            elif len(tlentry.matcher_processors) == 1:
                op.matcher = tlentry.matcher_processors[0][0]
                op.processor = tlentry.matcher_processors[0][1]
                newgroup.append((op, tlentry.priority))

            elif tlentry.names:
                found = False
                for (matcher, processor), name in zip(tlentry.matcher_processors, tlentry.names):
                    if name == op.name:
                        new = copy.deepcopy(op)
                        new.matcher = matcher
                        new.processor = processor
                        newgroup.append((new, tlentry.priority))
                        found = True
                assert found, "Never encountered a specific title during title lookup"

            elif tlentry.bits:
                found = False
                for (matcher, processor), bits in zip(tlentry.matcher_processors, tlentry.bits):
                    if bits == op.bits:
                        new = copy.deepcopy(op)
                        new.matcher = matcher
                        new.processor = processor
                        newgroup.append((new, tlentry.priority))
                        found = True
                assert found, "Never encountered a specific bits during bits lookup: {}".format(bits)

            else:
                for matcher, processor in tlentry.matcher_processors:
                    new = copy.deepcopy(op)
                    new.matcher = matcher
                    new.processor = processor
                    newgroup.append((new, tlentry.priority))

        newgroup.sort(key=lambda x: x[1], reverse=True) # sort by priority

        group[:] = [i for i, _ in newgroup]

def tl_merge_statics(ops):
    for m, group in ops:
        for op in group:
            statics = re.findall(r"(?:, )?\bStatic\(([0-9]+), 0b([01]+)\)", op.processor)
            if statics:
                bits = list(op.bits)
                for offset, static in statics:
                    op.processor = re.sub(r"((?:, )?\bStatic\([0-9]+, 0b[01]+\))", "", op.processor)
                    for i, s in enumerate(reversed(static)):
                        bitpos = 31 - (i + int(offset))
                        if s == "1":
                            bits[bitpos] = "1"
                        elif s == "0" and bits[bitpos] != "1":
                            bits[bitpos] = "0"

                bits = "".join(bits)
                print("found statics: {}, merged with {} into {}".format(statics, op.bits, bits))
                op.bits = bits

def main():
    import sys

    ops = read_op_defs("../temp/A64_v85A_ISA_xml_00bet10.tar/ISA_v85A_A64_xml_00bet10/ISA_v85A_A64_xml_00bet10")

    # filter out variants we don't care about
    # print(determine_instr_classes(ops))

    ops = filter_arch_versions(ops, {'ARMv8.1', 'ARMv8.2', 'ARMv8.3', 'ARMv8.4'})

    # available classes: general, system, float, fpsimd, advsimd, sve
    ops = filter_instr_classes(ops, {"general", "system", "float", "fpsimd", "advsimd"})

    ops = group_instr_aliases(ops)

    templates = group_templates(ops)

    # for instrclass in ("advsimd",):
    #     with open("tl_{}_template.py".format(instrclass), "w", encoding="utf-8") as f:
    #         generate_translation_files(ops, instrclass, f)


    # dump debugging data
    for t, fields, m in templates:
        print(m)
        print("{} => {}".format(repr(t), fields))

    for m, o in ops:
        print("############## {} ############".format(m))
        for op in o:
            print(op)

    # load translation files
    tlmap = TranslationMap()
    for file in ("tl_general.py", "tl_system.py", "tl_float.py", "tl_fpsimd.py", "tl_advsimd.py"):
        with open(os.path.join("aarch64_data", file), "r", encoding="utf-8") as f:
            tlmap.load_file(f, file)

    tlmap.build_lookup_table()

    # add the opmap data

    tl_assign_matchers(ops, tlmap)
    #assign_matchers(ops)
    #merge_matchers(ops)

    # compile static data into bits
    tl_merge_statics(ops)

    #emit the actual opmap

    with open("../plugin/src/arch/aarch64/opmap.rs", "w", encoding="utf-8") as f:
        f.write("// This file was generated bo tools/aarch64_gen_opmap.py\nOps!(\n\n")
        emit_opmap(ops, f)
        f.write("\n);\n")

if __name__ == '__main__':
    main()

