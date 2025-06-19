use core::panic;
use std::fmt;
use crate::memory::memory::Mmu; 

// non perfiexd opcodes -> load and aritmatic ops
#[allow(dead_code)]
pub struct Cpu {
    pub registers: Registers,
    pub mmu: Mmu
}


/*
enum with all of the cpu instructions.
registers with some of the letters small is the same as [reg] => meaning value at address value of reg.
*/
pub enum Instruction {
    NOP, //00
    LDBCN16(u16), //01
    LdBcA, //02
    INCBC, //03
    INCB, // 04
    DECB, //05
    LDBN8(u8), // 06
    RLCA, // 07
    Lda16Sp(u16), // 08
    ADDHLBC, // 09
    LdABc, //0A
    DECBC, //0B
    INCC, // 0C
    DECC, // 0D
    LDCN8(u8), // 0E
    RRCA, // 0F
    STOPN8(u8), //10
    LDDEN16(u16), //11
    LddEA, //12
    INCDE, //13
    INCD, //14
    DECD, //15
    LDDN8(u8), // 16
    RLA , //17



    UNKNOWN
}

enum Target {
    B,C,D,E,H,L,Hl,A
}

impl Cpu {
    

    fn new(cartridge: &String) -> Self {
        // create default registers and mapped mmu.
        Cpu { 
            registers: (Registers::default()),
            mmu: (Mmu::new(cartridge)) 
        }
    }

    fn fetch_byte(self: &Self) -> u8 {
        let byte: u8 = self.mmu.read_byte(self.registers.pc);
        let (_, res) = self.registers.pc.overflowing_add(1); // adding one to pc after fetching a byte.
        if res { // won't happen probably.
            panic!("PC CAN'T OVERFLOW FOR NOW -> UNTIL MBC IMPLEMENTED.");
        }
        byte
    }

    fn fetch_word(self: &Self) -> u16 {
        let low: u16 = self.fetch_byte() as u16;
        let high: u16 = self.fetch_byte() as u16;

        (high << 8) | low
    }

    // execute an instruction
    // TODO : think of design or opcodes fetching
    pub fn decode_instrcution(self: &Self) -> Instruction {
        let op = self.fetch_byte();
        match op {
            0x00 => Instruction::NOP,
            0x01 => {
                let word = self.fetch_word();
                Instruction::LDBCN16(word)
            },
            0x02 => Instruction::LdBcA,
            0x03 => Instruction::INCBC,
            0x04 => Instruction::INCB,
            0x05 => Instruction::DECB,
            0x06 => {
                let byte = self.fetch_byte();
                Instruction::LDBN8(byte)
            },
            0x07 => Instruction::RLCA,
            0x08 => {
                let word = self.fetch_word();
                Instruction::Lda16Sp(word) // address to look inside
            }
            0x09 => Instruction::ADDHLBC,
            0x0A => Instruction::LdABc,
            0x0B => Instruction::DECBC,
            0x0C => Instruction::INCC,
            0x0D => Instruction::DECC,
            0x0E => {
                let byte = self.fetch_byte();
                Instruction::LDCN8(byte)
            },
            0x0F => Instruction::RRCA,
            0x10 => {
                let byte = self.fetch_byte();
                Instruction::STOPN8(byte)
            }
            0x11 => {
                let word = self.fetch_word();
                Instruction::LDDEN16(word)
            }
            0x12 => Instruction::LddEA,
            0x13 => Instruction::INCDE,
            0x14 => Instruction::INCD,
            0x15 => Instruction::DECD,
            0x16 => {
                let byte = self.fetch_byte();
                Instruction::LDDN8(byte)
            },
            0x17 => Instruction::RLA,
            
            _ => Instruction::UNKNOWN
            
        }
        


        

        
    }



    

}


#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub ir: u8,
    pub ie: u8,
    pub pc: u16
}

impl Default for Registers {
    fn default() -> Self {
        // default values from: http://www.codeslinger.co.uk/pages/projects/gameboy/hardware.html
        Registers { a: (0x01),
            b: (0xFF),
            c: (0x13),
            d: (0),
            e: (0xC1),
            f: (0), 
            h: (0x84), 
            l: (0x03), 
            sp: (0xFFFE), 
            ir: (0), 
            ie: (0), 
            pc: (0x100) 
        }
    }
}

// trait to print the registers in a nice format
impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "
        registers =>
        a: 0x{:#X}, 
        b: 0x{:#X},
        c: 0x{:#X},
        d: 0x{:#X},
        e: {:b},
        f: 0x{:#X},
        h: 0x{:#X},
        l: 0x{:#X},
        sp: 0x{:#X},
        ir: 0x{:#X},
        ie: 0x{:#X},
        pc: 0x{:#X}", self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l, self.sp, self.ir, self.ie, self.pc)
    }
}


#[allow(dead_code)]
pub enum Flags {
    C = 0, // Carry Flag 
    N = 1, // Add/Subtract
    Pv = 2, // 	Parity/Overflow Flag
    Y = 3, // unused
    H = 4, // Half Carry Flag 
    X = 5, // unused
    Z = 6, // zero flag
    S = 7, // sign flag
}

#[allow(dead_code)]
impl Registers {

    // methods to get registers as whole
    pub fn get_bc(self: &Self) -> u16{
        return (self.b as u16) << 8 | (self.c as u16);
    }

    pub fn set_bc(self: &mut Self, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = (bc >> 8) as u8;
    }

    pub fn get_de(self: &Self) -> u16{
        return (self.d as u16) << 8 | (self.e as u16);
    }

    pub fn set_de(self: &mut Self, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = (de >> 8) as u8;
    }

    pub fn get_hl(self: &Self) -> u16{
        return (self.h as u16) << 8 | (self.l as u16);
    }

    pub fn set_hl(self: &mut Self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = (hl >> 8) as u8;
    }

}





fn set_flag(index: u8, number: u8) -> u8{
    number | (1 << index)
}
fn get_flag(index: u8, number: u8) -> u8{
    number & (1 << index)
}





