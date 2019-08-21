import subprocess
import binascii

def read_test_strings(f):
    buf = []
    for line in f:
        dynasm, gas = line.split("\t")
        buf.append((dynasm, gas))

    return buf

def compile_with_as(asmstring):
    with open("test.s", "w", encoding="utf-8") as f:
        f.write(asmstring)

    subprocess.run(["as", "test.s", "-o", "test.o"], check=True)
    subprocess.run(["objcopy", "-O", "binary", "test.o", "test.bin"], check=True)

    with open("test.bin", "rb") as f:
        data = f.read()
    return data

def write_result(buf, f):
    for dynasm, gas, binary in f:
        f.write("{}\t{}\t{}".format(dynasm, gas, binascii.hexlify(binary)))

def main():
    import sys
    test_strings = read_test_strings(sys.argv[1])

    buf = []
    for dynasm, gas in test_strings:
        try:
            binary = compile_with_as(gas)
        except:
            Print("Error at {}".format(gas))
            raise
        buf.append((dynasm, gas, binary))

    with open(sys.argv[2]):
        write_result(buf, f)

if __name__ == '__main__':
    main()
