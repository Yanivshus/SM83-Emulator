use std::fs;



struct Cpu {
    registers: Registers,
}


// // ment to to be the container for the ram and rom, each memory operation will happen from here.
// struct Memory {
//     ram: [u8; 0xFFFF],
// }


fn get_bits(index: u8, number: u8) -> bool{
    let extracted: bool = ((number >> index) & 1) == 0;
    if extracted{
        return false;
    }
    return true;
}


struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    ir: u8,
    ie: u8,
    pc: u16
}


impl Registers {

    fn get_carry(self: &Self, elc: u8) -> bool{
        return get_bits(elc,self.f);
    }

    // methods to get registers as whole
    fn get_bc(self: &Self) -> u16{
        return (self.b as u16) << 8 | (self.c as u16);
    }

    fn set_bc(self: &mut Self, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = (bc >> 8) as u8;
    }

    fn get_de(self: &Self) -> u16{
        return (self.d as u16) << 8 | (self.e as u16);
    }

    fn set_de(self: &mut Self, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = (de >> 8) as u8;
    }

    fn get_hl(self: &Self) -> u16{
        return (self.h as u16) << 8 | (self.l as u16);
    }

    fn set_hl(self: &mut Self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = (hl >> 8) as u8;
    }

}



struct GameBoyFile {
    buffer: Vec<u8>
}

struct ProblemCreatingFile;
impl GameBoyFile {
    // TODO : Make more self explanatory error message -> use Result<Self, Some Error Enum>
    fn new(file_name: &String) -> Result<Self, E> {
        let gb_file_result= fs::read(file_name);
        let gb_file = match gb_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem reading file {error:?}")
        };
    }
}

fn main() {
    let reg = Registers {
        a: 1,
        b: 1,
        c: 1,
        d: 1,
        e: 1,
        f: 0b11001010,
        h: 1,
        l: 1,
        sp: 1,
        ie: 2,
        ir: 2,
        pc: 2
    };

    let cpu = Cpu{
        registers: reg
    };
    let p = cpu.registers.get_carry(5);
    println!("{p}");
    println!("Hello, world!");
}
