test_file = input("File: ")

file = open(test_file, "r")
data = file.read()
file.close()

opcodes = data.split(' ')

hex = '0x'
hex_opcodes = []

for opcode in opcodes:
    hex_opcode = hex + opcode
    hex_opcodes.append(hex_opcode)


for op in hex_opcodes:
    print(op, end=", ")


