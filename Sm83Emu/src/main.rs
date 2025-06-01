use core::fmt;
use std::{fs, io};

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[derive(Debug)] // means that the error only will implement the debug trait.
enum MmuError {
    NintendoLogoDoesntExists,
    FileReadingErr(io::Error),
}

impl fmt::Display for MmuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NintendoLogoDoesntExists => write!(f, "The Nintendo logo doesn't exists in the binary, finished execution"),
            Self::FileReadingErr(err) => write!(f, "io problem {err:?}")
        }
    }
}

impl Mmu {
    // TODO : Make more self explanatory error message -> use Result<Self, Some Error Enum>
    fn new(file_name: &String) -> Result<Self, MmuError> {
        let gb_file_result= fs::read(file_name);
        let gb_file = match gb_file_result {
            Ok(file) => file,
            Err(error) => return Err(MmuError::FileReadingErr(error))
        };
        //
        // slize to check if the nintedo logo match
        if !check_logo(&gb_file[GbFileLocations::LogoS as usize..(GbFileLocations::LogoE as usize + 1)]) {
            return Err(MmuError::NintendoLogoDoesntExists);
        }
        return Ok(Mmu{
            buffer: gb_file
        })
        
    }
    
    fn extract_opcodes(self: &Self) -> Vec<Opcode> {
        todo!()
    }
}

#[allow(dead_code)]
enum GbFileLocations {
    EntryPoint = 0x100,
    LogoS = 0x104,
    LogoE = 0x133,
    TitleS = 0x134,
    TitleE = 0x145,
    CgbMode = 0x143,
    LicenseeCodeS = 0x144,
    LicenseeCodeE = 0x146,
    CartirdgeType = 0x147,
    RomSize = 0x148,
    RamSize = 0x149,
    HeaderCheckSum = 0x14D
}

fn main() {
    let gb = Mmu::new(&String::from("/home/kaish/Downloads/Calc.gb"));
}
