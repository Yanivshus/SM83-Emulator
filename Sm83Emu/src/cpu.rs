
#[allow(dead_code)]
pub struct Cpu {
    registers: Registers,
}

#[allow(dead_code)]
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