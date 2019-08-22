import subprocess
import binascii

def read_test_strings(f):
    buf = []
    for line in f:
        if line:
            if not "\t" in line:
                print(line)
            dynasm, gas = line.split("\t")
            buf.append((dynasm.strip(), gas.strip()))

    return buf

def compile_with_as(asmstring):
    with open("test.s", "w", encoding="utf-8") as f:
        f.write(asmstring)
        f.write("\n")

    subprocess.run(["as", "-mcpu=all", "test.s", "-o", "test.o"], check=True)
    subprocess.run(["objcopy", "-O", "binary", "test.o", "test.bin"], check=True)

    with open("test.bin", "rb") as f:
        data = f.read()
    return data

def write_result(buf, f):
    for dynasm, gas, binary in buf:
        f.write("{}\t{}\t{}\n".format(dynasm, gas, binascii.hexlify(binary).decode("utf-8")))

def main():
    import sys
    with open(sys.argv[1], "r", encoding="utf-8") as f:
        test_strings = read_test_strings(f)

    buf = []
    for dynasm, gas in test_strings:
        try:
            binary = compile_with_as(gas)
            buf.append((dynasm, gas, binary))
        except:
            print("Error at {}".format(gas))

    with open(sys.argv[2], "w", encoding="utf-8") as f:
        write_result(buf, f)

if __name__ == '__main__':
    main()
