import json


def get_name(instruction):
    values = ['n8','n16', 'e8']

    mnemonic = instruction["mnemonic"]
    operands = instruction["operands"]
    full_opcode = "" + str(mnemonic)
    for op in operands:
        full_opcode += str(op["name"])

    if 'e8' in full_opcode:
        full_opcode += '(i8)'

    if 'n8' in full_opcode:
        full_opcode += '(u8)'

    if 'n16' in full_opcode:
        full_opcode += '(u16)'
    
    if 'a16' in full_opcode:
        full_opcode += '(u16)'
    
    full_opcode = full_opcode.replace('$', '')
    return full_opcode


def main():
    file_data = open("Opcodes.json").read()
    opjson = json.loads(file_data)
    list_rust_opcodes_without_byte = ""
    list_rust_opcodes_byte = ""
    list_rust_opcodes_to_exectute = ""

    for opcode in range(0x00, 0x100):
        op = f"0x{opcode:02X}"
        full = get_name(opjson['unprefixed'][op])
        
        list_rust_opcodes_without_byte += full + ', //' + op + "\n"
        list_rust_opcodes_byte += op + " => Instruction::" + full + ",\n"
        list_rust_opcodes_to_exectute += "Instruction::" + full + ' => println!("{}",' + op + '),\n'

    file_data1 = open("rust_generated_opcodes_without_byte.txt", "w")
    file_data1.write(list_rust_opcodes_without_byte)

    file_data2 = open("rust_generated_opcodes.txt", "w")
    file_data2.write(list_rust_opcodes_byte)

    file_data3 = open("rust_rust_opcodes_to_exectute.txt", "w")
    file_data3.write(list_rust_opcodes_to_exectute)
    
    
    

if __name__ == '__main__':
    main()