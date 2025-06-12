use std::fmt;
use crate::memory::memory::Mmu; 

#[allow(dead_code)]

// non perfiexd opcodes -> load and aritmatic ops
enum Opcodes {

}

#[allow(dead_code)]
// cb perfiexd opcodes -> what's left.
enum Cb_Opcodes {

}
#[allow(dead_code)]
pub struct Cpu {
    pub registers: Registers,
    pub mmu: Mmu
}

impl Cpu {
    fn new(cartridge: &String) -> Self {
        // create default registers and mapped mmu.
        Cpu { 
            registers: (Registers::default()),
            mmu: (Mmu::new(cartridge)) 
        }
    }

    // execute an instruction
    pub fn fetch_opcode(self: &Self) {
        let op: u8 = self.mmu.read_byte(self.registers.pc);
        // match op {
        //     0xCB => {

        //     }
        // }
        
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



pub fn set_flag(index: u8, number: u8) -> u8{
    number | (1 << index)
}
pub fn get_flag(index: u8, number: u8) -> u8{
    number & (1 << index)
}





