use std::{fs};



struct Cpu {
    registers: Registers,
}


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

// will represent the program opcodes.
enum Opcode {
    Add(u8,u8) // adding to 8 bits registers together 
}

fn check_logo(barr: &[u8]) -> bool {
    // represent to original nintendo tile bmp
    let original_bytes:Vec<u8> = vec![0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
                                    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
                                    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E];
    if *barr == *original_bytes.as_slice() {
        return true;
    }
    false
}

// for now we will work without MBC -> small games
struct Mmu {
    buffer: Vec<u8>,
}

enum MmuError {
    
}

impl Mmu {
    // TODO : Make more self explanatory error message -> use Result<Self, Some Error Enum>
    fn new(file_name: &String) -> Result<Self> {
        let gb_file_result= fs::read(file_name);
        let gb_file = match gb_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem reading file {error:?}")
        };
        //
        // slize to check if the nintedo logo match
        if !check_logo(&gb_file[GbFileLocations::LOGO_S as usize..(GbFileLocations::LOGO_E as usize + 1)]) {
            
        }
        return None;
    }
    
    fn extract_opcodes(self: &Self) -> Vec<Opcode> {
        todo!()
    }
}

enum GbFileLocations {
    ENTRY_POINT_S = 0x100,
    LOGO_S = 0x104,
    LOGO_E = 0x133,
    TITLE_S = 0x134,
    TOTLE_E = 0x145,
    CGB_MODE = 0x143,
    LICENSEE_CODE_S = 0x144,
    LICENSEE_CODE_E = 0x146,
    CARTRIDGE_TYPE = 0x147,
    ROM_SIZE = 0x148,
    RAM_SIZE = 0x149,
    HEADER_CHECKSUM = 0x14D
}

fn main() {
    let gb = Mmu::new(&String::from("/home/kaish/Downloads/Calc.gb"));
}
