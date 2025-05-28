// use bitvec::prelude::*;
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

pub struct Cpu {
    pub registers: Registers,
}
