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
#[allow(non_camel_case_types)]
pub enum Instruction {
    NOP, //0x00
    LDBCn16(u16), //0x01
    LDBCA, //0x02
    INCBC, //0x03
    INCB, //0x04
    DECB, //0x05
    LDBn8(u8), //0x06
    RLCA, //0x07
    LDa16SP(u16), //0x08
    ADDHLBC, //0x09
    LDABC, //0x0A
    DECBC, //0x0B
    INCC, //0x0C
    DECC, //0x0D
    LDCn8(u8), //0x0E
    RRCA, //0x0F
    STOPn8(u8), //0x10
    LDDEn16(u16), //0x11
    LDDEA, //0x12
    INCDE, //0x13
    INCD, //0x14
    DECD, //0x15
    LDDn8(u8), //0x16
    RLA, //0x17
    JRe8(i8), //0x18
    ADDHLDE, //0x19
    LDADE, //0x1A
    DECDE, //0x1B
    INCE, //0x1C
    DECE, //0x1D
    LDEn8(u8), //0x1E
    RRA, //0x1F
    JRNZe8(i8), //0x20
    LDHLn16(u16), //0x21
    LDHLA, //0x22
    INCHL, //0x23
    INCH, //0x24
    DECH, //0x25
    LDHn8(u8), //0x26
    DAA, //0x27
    JRZe8(i8), //0x28
    ADDHLHL, //0x29
    LDAHL, //0x2A
    DECHL, //0x2B
    INCL, //0x2C
    DECL, //0x2D
    LDLn8(u8), //0x2E
    CPL, //0x2F
    JRNCe8(i8), //0x30
    LDSPn16(u16), //0x31
    LDHLA32, //0x32
    INCSP, //0x33
    INCHL34, //0x34
    DECHL35, //0x35
    LDHLn8(u8), //0x36
    SCF, //0x37
    JRCe8(i8), //0x38
    ADDHLSP, //0x39
    LDAHL3A, //0x3A
    DECSP, //0x3B
    INCA, //0x3C
    DECA, //0x3D
    LDAn8(u8), //0x3E
    CCF, //0x3F
    LDBB, //0x40
    LDBC, //0x41
    LDBD, //0x42
    LDBE, //0x43
    LDBH, //0x44
    LDBL, //0x45
    LDBHL, //0x46
    LDBA, //0x47
    LDCB, //0x48
    LDCC, //0x49
    LDCD, //0x4A
    LDCE, //0x4B
    LDCH, //0x4C
    LDCL, //0x4D
    LDCHL, //0x4E
    LDCA, //0x4F
    LDDB, //0x50
    LDDC, //0x51
    LDDD, //0x52
    LDDE, //0x53
    LDDH, //0x54
    LDDL, //0x55
    LDDHL, //0x56
    LDDA, //0x57
    LDEB, //0x58
    LDEC, //0x59
    LDED, //0x5A
    LDEE, //0x5B
    LDEH, //0x5C
    LDEL, //0x5D
    LDEHL, //0x5E
    LDEA, //0x5F
    LDHB, //0x60
    LDHC, //0x61
    LDHD, //0x62
    LDHE, //0x63
    LDHH, //0x64
    LDHL, //0x65
    LDHHL, //0x66
    LDHA, //0x67
    LDLB, //0x68
    LDLC, //0x69
    LDLD, //0x6A
    LDLE, //0x6B
    LDLH, //0x6C
    LDLL, //0x6D
    LDLHL, //0x6E
    LDLA, //0x6F
    LDHLB, //0x70
    LDHLC, //0x71
    LDHLD, //0x72
    LDHLE, //0x73
    LDHLH, //0x74
    LDHLL, //0x75
    HALT, //0x76
    LDHLA77, //0x77
    LDAB, //0x78
    LDAC, //0x79
    LDAD, //0x7A
    LDAE, //0x7B
    LDAH, //0x7C
    LDAL, //0x7D
    LDAHL7E, //0x7E
    LDAA, //0x7F
    ADDAB, //0x80
    ADDAC, //0x81
    ADDAD, //0x82
    ADDAE, //0x83
    ADDAH, //0x84
    ADDAL, //0x85
    ADDAHL, //0x86
    ADDAA, //0x87
    ADCAB, //0x88
    ADCAC, //0x89
    ADCAD, //0x8A
    ADCAE, //0x8B
    ADCAH, //0x8C
    ADCAL, //0x8D
    ADCAHL, //0x8E
    ADCAA, //0x8F
    SUBAB, //0x90
    SUBAC, //0x91
    SUBAD, //0x92
    SUBAE, //0x93
    SUBAH, //0x94
    SUBAL, //0x95
    SUBAHL, //0x96
    SUBAA, //0x97
    SBCAB, //0x98
    SBCAC, //0x99
    SBCAD, //0x9A
    SBCAE, //0x9B
    SBCAH, //0x9C
    SBCAL, //0x9D
    SBCAHL, //0x9E
    SBCAA, //0x9F
    ANDAB, //0xA0
    ANDAC, //0xA1
    ANDAD, //0xA2
    ANDAE, //0xA3
    ANDAH, //0xA4
    ANDAL, //0xA5
    ANDAHL, //0xA6
    ANDAA, //0xA7
    XORAB, //0xA8
    XORAC, //0xA9
    XORAD, //0xAA
    XORAE, //0xAB
    XORAH, //0xAC
    XORAL, //0xAD
    XORAHL, //0xAE
    XORAA, //0xAF
    ORAB, //0xB0
    ORAC, //0xB1
    ORAD, //0xB2
    ORAE, //0xB3
    ORAH, //0xB4
    ORAL, //0xB5
    ORAHL, //0xB6
    ORAA, //0xB7
    CPAB, //0xB8
    CPAC, //0xB9
    CPAD, //0xBA
    CPAE, //0xBB
    CPAH, //0xBC
    CPAL, //0xBD
    CPAHL, //0xBE
    CPAA, //0xBF
    RETNZ, //0xC0
    POPBC, //0xC1
    JPNZa16(u16), //0xC2
    JPa16(u16), //0xC3
    CALLNZa16(u16), //0xC4
    PUSHBC, //0xC5
    ADDAn8(u8), //0xC6
    RST00, //0xC7
    RETZ, //0xC8
    RET, //0xC9
    JPZa16(u16), //0xCA
    PREFIX, //0xCB
    CALLZa16(u16), //0xCC
    CALLa16(u16), //0xCD
    ADCAn8(u8), //0xCE
    RST08, //0xCF
    RETNC, //0xD0
    POPDE, //0xD1
    JPNCa16(u16), //0xD2
    ILLEGAL_D3, //0xD3
    CALLNCa16(u16), //0xD4
    PUSHDE, //0xD5
    SUBAn8(u8), //0xD6
    RST10, //0xD7
    RETC, //0xD8
    RETI, //0xD9
    JPCa16(u16), //0xDA
    ILLEGAL_DB, //0xDB
    CALLCa16(u16), //0xDC
    ILLEGAL_DD, //0xDD
    SBCAn8(u8), //0xDE
    RST18, //0xDF
    LDHa8A, //0xE0
    POPHL, //0xE1
    LDHCA, //0xE2
    ILLEGAL_E3, //0xE3
    ILLEGAL_E4, //0xE4
    PUSHHL, //0xE5
    ANDAn8(u8), //0xE6
    RST20, //0xE7
    ADDSPe8(i8), //0xE8
    JPHL, //0xE9
    LDa16A(u16), //0xEA
    ILLEGAL_EB, //0xEB
    ILLEGAL_EC, //0xEC
    ILLEGAL_ED, //0xED
    XORAn8(u8), //0xEE
    RST28, //0xEF
    LDHAa8, //0xF0
    POPAF, //0xF1
    LDHAC, //0xF2
    DI, //0xF3
    ILLEGAL_F4, //0xF4
    PUSHAF, //0xF5
    ORAn8(u8), //0xF6
    RST30, //0xF7
    LDHLSPe8(i8), //0xF8
    LDSPHL, //0xF9
    LDAa16(u16), //0xFA
    EI, //0xFB
    ILLEGAL_FC, //0xFC
    ILLEGAL_FD, //0xFD
    CPAn8(u8), //0xFE
    RST38, //0xFF

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

    fn fetch_sbyte(self: &Self) -> i8 {
        let byte: i8 = self.mmu.read_sbyte(self.registers.pc);
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
                Instruction::LDBCn16(word)
            },
            0x02 => Instruction::LDBCA,
            0x03 => Instruction::INCBC,
            0x04 => Instruction::INCB,
            0x05 => Instruction::DECB, 
            0x06 => { 
                let byte = self.fetch_byte();
                Instruction::LDBn8(byte)
            },
            0x07 => Instruction::RLCA,
            0x08 => {
                let word = self.fetch_word();
                Instruction::LDa16SP(word)
            },
            0x09 => Instruction::ADDHLBC,
            0x0A => Instruction::LDABC,
            0x0B => Instruction::DECBC,
            0x0C => Instruction::INCC,
            0x0D => Instruction::DECC,
            0x0E => {
                let byte = self.fetch_byte();
                Instruction::LDCn8(byte)
            },  
            0x0F => Instruction::RRCA,
            0x10 => {
                let byte = self.fetch_byte();
                Instruction::STOPn8(byte)
            },
            
            0x11 => {
                let word = self.fetch_word();
                Instruction::LDDEn16(word)
            },
            
            0x12 => Instruction::LDDEA,
            0x13 => Instruction::INCDE,
            0x14 => Instruction::INCD,
            0x15 => Instruction::DECD,
            0x16 => {
                let byte = self.fetch_byte();
                Instruction::LDDn8(byte)
            },
            0x17 => Instruction::RLA,
            0x18 => {
                let byte = self.fetch_sbyte();
                Instruction::JRe8(byte)
            },
            0x19 => Instruction::ADDHLDE,
            0x1A => Instruction::LDADE,
            0x1B => Instruction::DECDE,
            0x1C => Instruction::INCE,
            0x1D => Instruction::DECE,
            0x1E => {
                let byte = self.fetch_byte();
                Instruction::LDEn8(byte)
            },
            0x1F => Instruction::RRA,
            0x20 => {
                let byte = self.fetch_sbyte();
                Instruction::JRNZe8(byte)
            },
            0x21 => {
                let word = self.fetch_word();
                Instruction::LDHLn16(word)
            },
            0x22 => Instruction::LDHLA,
            0x23 => Instruction::INCHL,
            0x24 => Instruction::INCH,
            0x25 => Instruction::DECH,
            0x26 => {
                let byte = self.fetch_byte();
                Instruction::LDHn8(byte)
            }
            0x27 => Instruction::DAA,
            0x28 => {
                let byte = self.fetch_sbyte();
                Instruction::JRZe8(byte)
            },
            
            0x29 => Instruction::ADDHLHL,
            0x2A => Instruction::LDAHL,
            0x2B => Instruction::DECHL,
            0x2C => Instruction::INCL,
            0x2D => Instruction::DECL,
            0x2E => {
                let byte = self.fetch_byte();
                Instruction::LDLn8(byte)
            },
            0x2F => Instruction::CPL,
            0x30 => {
                let byte = self.fetch_sbyte();
                Instruction::JRNCe8(byte)
            },
            0x31 => {
                let word = self.fetch_word();
                Instruction::LDSPn16(word)
            },
            0x32 => Instruction::LDHLA32,
            0x33 => Instruction::INCSP,
            0x34 => Instruction::INCHL,
            0x35 => Instruction::DECHL,
            0x36 => {
                let byte = self.fetch_byte();
                Instruction::LDHLn8(byte)
            },
            0x37 => Instruction::SCF,
            0x38 => {
                let byte = self.fetch_sbyte();
                Instruction::JRCe8(byte)
            },
            0x39 => Instruction::ADDHLSP,
            0x3A => Instruction::LDAHL3A,
            0x3B => Instruction::DECSP,
            0x3C => Instruction::INCA,
            0x3D => Instruction::DECA,
            0x3E => {
                let byte = self.fetch_byte();
                Instruction::LDAn8(byte)
            },
            0x3F => Instruction::CCF,
            0x40 => Instruction::LDBB,
            0x41 => Instruction::LDBC,
            0x42 => Instruction::LDBD,
            0x43 => Instruction::LDBE,
            0x44 => Instruction::LDBH,
            0x45 => Instruction::LDBL,
            0x46 => Instruction::LDBHL,
            0x47 => Instruction::LDBA,
            0x48 => Instruction::LDCB,
            0x49 => Instruction::LDCC,
            0x4A => Instruction::LDCD,
            0x4B => Instruction::LDCE,
            0x4C => Instruction::LDCH,
            0x4D => Instruction::LDCL,
            0x4E => Instruction::LDCHL,
            0x4F => Instruction::LDCA,
            0x50 => Instruction::LDDB,
            0x51 => Instruction::LDDC,
            0x52 => Instruction::LDDD,
            0x53 => Instruction::LDDE,
            0x54 => Instruction::LDDH,
            0x55 => Instruction::LDDL,
            0x56 => Instruction::LDDHL,
            0x57 => Instruction::LDDA,
            0x58 => Instruction::LDEB,
            0x59 => Instruction::LDEC,
            0x5A => Instruction::LDED,
            0x5B => Instruction::LDEE,
            0x5C => Instruction::LDEH,
            0x5D => Instruction::LDEL,
            0x5E => Instruction::LDEHL,
            0x5F => Instruction::LDEA,
            0x60 => Instruction::LDHB,
            0x61 => Instruction::LDHC,
            0x62 => Instruction::LDHD,
            0x63 => Instruction::LDHE,
            0x64 => Instruction::LDHH,
            0x65 => Instruction::LDHL,
            0x66 => Instruction::LDHHL,
            0x67 => Instruction::LDHA,
            0x68 => Instruction::LDLB,
            0x69 => Instruction::LDLC,
            0x6A => Instruction::LDLD,
            0x6B => Instruction::LDLE,
            0x6C => Instruction::LDLH,
            0x6D => Instruction::LDLL,
            0x6E => Instruction::LDLHL,
            0x6F => Instruction::LDLA,
            0x70 => Instruction::LDHLB,
            0x71 => Instruction::LDHLC,
            0x72 => Instruction::LDHLD,
            0x73 => Instruction::LDHLE,
            0x74 => Instruction::LDHLH,
            0x75 => Instruction::LDHLL,
            0x76 => Instruction::HALT,
            0x77 => Instruction::LDHLA77,
            0x78 => Instruction::LDAB,
            0x79 => Instruction::LDAC,
            0x7A => Instruction::LDAD,
            0x7B => Instruction::LDAE,
            0x7C => Instruction::LDAH,
            0x7D => Instruction::LDAL,
            0x7E => Instruction::LDAHL7E,
            0x7F => Instruction::LDAA,
            0x80 => Instruction::ADDAB,
            0x81 => Instruction::ADDAC,
            0x82 => Instruction::ADDAD,
            0x83 => Instruction::ADDAE,
            0x84 => Instruction::ADDAH,
            0x85 => Instruction::ADDAL,
            0x86 => Instruction::ADDAHL,
            0x87 => Instruction::ADDAA,
            0x88 => Instruction::ADCAB,
            0x89 => Instruction::ADCAC,
            0x8A => Instruction::ADCAD,
            0x8B => Instruction::ADCAE,
            0x8C => Instruction::ADCAH,
            0x8D => Instruction::ADCAL,
            0x8E => Instruction::ADCAHL,
            0x8F => Instruction::ADCAA,
            0x90 => Instruction::SUBAB,
            0x91 => Instruction::SUBAC,
            0x92 => Instruction::SUBAD,
            0x93 => Instruction::SUBAE,
            0x94 => Instruction::SUBAH,
            0x95 => Instruction::SUBAL,
            0x96 => Instruction::SUBAHL,
            0x97 => Instruction::SUBAA,
            0x98 => Instruction::SBCAB,
            0x99 => Instruction::SBCAC,
            0x9A => Instruction::SBCAD,
            0x9B => Instruction::SBCAE,
            0x9C => Instruction::SBCAH,
            0x9D => Instruction::SBCAL,
            0x9E => Instruction::SBCAHL,
            0x9F => Instruction::SBCAA,
            0xA0 => Instruction::ANDAB,
            0xA1 => Instruction::ANDAC,
            0xA2 => Instruction::ANDAD,
            0xA3 => Instruction::ANDAE,
            0xA4 => Instruction::ANDAH,
            0xA5 => Instruction::ANDAL,
            0xA6 => Instruction::ANDAHL,
            0xA7 => Instruction::ANDAA,
            0xA8 => Instruction::XORAB,
            0xA9 => Instruction::XORAC,
            0xAA => Instruction::XORAD,
            0xAB => Instruction::XORAE,
            0xAC => Instruction::XORAH,
            0xAD => Instruction::XORAL,
            0xAE => Instruction::XORAHL,
            0xAF => Instruction::XORAA,
            0xB0 => Instruction::ORAB,
            0xB1 => Instruction::ORAC,
            0xB2 => Instruction::ORAD,
            0xB3 => Instruction::ORAE,
            0xB4 => Instruction::ORAH,
            0xB5 => Instruction::ORAL,
            0xB6 => Instruction::ORAHL,
            0xB7 => Instruction::ORAA,
            0xB8 => Instruction::CPAB,
            0xB9 => Instruction::CPAC,
            0xBA => Instruction::CPAD,
            0xBB => Instruction::CPAE,
            0xBC => Instruction::CPAH,
            0xBD => Instruction::CPAL,
            0xBE => Instruction::CPAHL,
            0xBF => Instruction::CPAA,
            0xC0 => Instruction::RETNZ,
            0xC1 => Instruction::POPBC,
            0xC2 => {
                let word = self.fetch_word();
                Instruction::JPNZa16(word)
            },
            0xC3 => {
                let word = self.fetch_word();
                Instruction::JPa16(word)
            },
            0xC4 => {
                let word = self.fetch_word();
                Instruction::CALLNZa16(word)
            },
            0xC5 => Instruction::PUSHBC,
            0xC6 => {
                let byte = self.fetch_byte();
                Instruction::ADDAn8(byte)
            },
            0xC7 => Instruction::RST00,
            0xC8 => Instruction::RETZ,
            0xC9 => Instruction::RET,
            0xCA => {
                let word = self.fetch_word();
                Instruction::JPZa16(word)
            },
            0xCB => Instruction::PREFIX,
            0xCC => {
                let word = self.fetch_word();
                Instruction::CALLZa16(word)
            },
            0xCD => {
                let word = self.fetch_word();
                Instruction::CALLa16(word)
            },
            0xCE => {
                let byte = self.fetch_byte();
                Instruction::ADCAn8(byte)
            },
            0xCF => Instruction::RST08,
            0xD0 => Instruction::RETNC,
            0xD1 => Instruction::POPDE,
            0xD2 => {
                let word = self.fetch_word();
                Instruction::JPNCa16(word)
            },
            0xD3 => Instruction::ILLEGAL_D3,
            0xD4 => {
                let word = self.fetch_word();
                Instruction::CALLNCa16(word)
            },
            0xD5 => Instruction::PUSHDE,
            0xD6 => {
                let byte = self.fetch_byte();
                Instruction::SUBAn8(byte)
            },
            0xD7 => Instruction::RST10,
            0xD8 => Instruction::RETC,
            0xD9 => Instruction::RETI,
            0xDA => {
                let word = self.fetch_word();
                Instruction::JPCa16(word)
            },
            0xDB => Instruction::ILLEGAL_DB,
            0xDC => {
                let word = self.fetch_word();
                Instruction::CALLCa16(word)
            },
            0xDD => Instruction::ILLEGAL_DD,
            0xDE => {
                let byte = self.fetch_byte();
                Instruction::SBCAn8(byte)
            },
            0xDF => Instruction::RST18,
            0xE0 => Instruction::LDHa8A,
            0xE1 => Instruction::POPHL,
            0xE2 => Instruction::LDHCA,
            0xE3 => Instruction::ILLEGAL_E3,
            0xE4 => Instruction::ILLEGAL_E4,
            0xE5 => Instruction::PUSHHL,
            0xE6 => {
                let byte = self.fetch_byte();
                Instruction::ANDAn8(byte)
            },
            0xE7 => Instruction::RST20,
            0xE8 => {
                let byte = self.fetch_sbyte();
                Instruction::ADDSPe8(byte)
            },
            0xE9 => Instruction::JPHL,
            0xEA => {
                let word = self.fetch_word();
                Instruction::LDa16A(word)
            },
            0xEB => Instruction::ILLEGAL_EB,
            0xEC => Instruction::ILLEGAL_EC,
            0xED => Instruction::ILLEGAL_ED,
            0xEE => {
                let byte = self.fetch_byte();
                Instruction::XORAn8(byte)
            },
            0xEF => Instruction::RST28,
            0xF0 => Instruction::LDHAa8,
            0xF1 => Instruction::POPAF,
            0xF2 => Instruction::LDHAC,
            0xF3 => Instruction::DI,
            0xF4 => Instruction::ILLEGAL_F4,
            0xF5 => Instruction::PUSHAF,
            0xF6 => {
                let byte = self.fetch_byte();
                Instruction::ORAn8(byte)
            },
            0xF7 => Instruction::RST30,
            0xF8 => {
                let byte = self.fetch_sbyte();
                Instruction::LDHLSPe8(byte)
            },
            0xF9 => Instruction::LDSPHL,
            0xFA => {
                let word = self.fetch_word();
                Instruction::LDAa16(word)
            },
            0xFB => Instruction::EI,
            0xFC => Instruction::ILLEGAL_FC,
            0xFD => Instruction::ILLEGAL_FD,
            0xFE => {
                let byte = self.fetch_byte();
                Instruction::CPAn8(byte)
            },
            0xFF => Instruction::RST38,

            _ => Instruction::UNKNOWN
            
        } 
    }

    fn execute_instruction(self: &Self, instruction: &Instruction) {
        match instruction {
            Instruction::NOP => println!("{}",0x00),
            Instruction::LDBCn16(u16) => println!("{}",0x01),
            Instruction::LDBCA => println!("{}",0x02),
            Instruction::INCBC => println!("{}",0x03),
            Instruction::INCB => println!("{}",0x04),
            Instruction::DECB => println!("{}",0x05),
            Instruction::LDBn8(u8) => println!("{}",0x06),
            Instruction::RLCA => println!("{}",0x07),
            Instruction::LDa16SP(u16) => println!("{}",0x08),
            Instruction::ADDHLBC => println!("{}",0x09),
            Instruction::LDABC => println!("{}",0x0A),
            Instruction::DECBC => println!("{}",0x0B),
            Instruction::INCC => println!("{}",0x0C),
            Instruction::DECC => println!("{}",0x0D),
            Instruction::LDCn8(u8) => println!("{}",0x0E),
            Instruction::RRCA => println!("{}",0x0F),
            Instruction::STOPn8(u8) => println!("{}",0x10),
            Instruction::LDDEn16(u16) => println!("{}",0x11),
            Instruction::LDDEA => println!("{}",0x12),
            Instruction::INCDE => println!("{}",0x13),
            Instruction::INCD => println!("{}",0x14),
            Instruction::DECD => println!("{}",0x15),
            Instruction::LDDn8(u8) => println!("{}",0x16),
            Instruction::RLA => println!("{}",0x17),
            Instruction::JRe8(i8) => println!("{}",0x18),
            Instruction::ADDHLDE => println!("{}",0x19),
            Instruction::LDADE => println!("{}",0x1A),
            Instruction::DECDE => println!("{}",0x1B),
            Instruction::INCE => println!("{}",0x1C),
            Instruction::DECE => println!("{}",0x1D),
            Instruction::LDEn8(u8) => println!("{}",0x1E),
            Instruction::RRA => println!("{}",0x1F),
            Instruction::JRNZe8(i8) => println!("{}",0x20),
            Instruction::LDHLn16(u16) => println!("{}",0x21),
            Instruction::LDHLA => println!("{}",0x22),
            Instruction::INCHL => println!("{}",0x23),
            Instruction::INCH => println!("{}",0x24),
            Instruction::DECH => println!("{}",0x25),
            Instruction::LDHn8(u8) => println!("{}",0x26),
            Instruction::DAA => println!("{}",0x27),
            Instruction::JRZe8(i8) => println!("{}",0x28),
            Instruction::ADDHLHL => println!("{}",0x29),
            Instruction::LDAHL => println!("{}",0x2A),
            Instruction::DECHL => println!("{}",0x2B),
            Instruction::INCL => println!("{}",0x2C),
            Instruction::DECL => println!("{}",0x2D),
            Instruction::LDLn8(u8) => println!("{}",0x2E),
            Instruction::CPL => println!("{}",0x2F),
            Instruction::JRNCe8(i8) => println!("{}",0x30),
            Instruction::LDSPn16(u16) => println!("{}",0x31),
            Instruction::LDHLA32 => println!("{}",0x32),
            Instruction::INCSP => println!("{}",0x33),
            Instruction::INCHL34 => println!("{}",0x34),
            Instruction::DECHL35 => println!("{}",0x35),
            Instruction::LDHLn8(u8) => println!("{}",0x36),
            Instruction::SCF => println!("{}",0x37),
            Instruction::JRCe8(i8) => println!("{}",0x38),
            Instruction::ADDHLSP => println!("{}",0x39),
            Instruction::LDAHL3A => println!("{}",0x3A),
            Instruction::DECSP => println!("{}",0x3B),
            Instruction::INCA => println!("{}",0x3C),
            Instruction::DECA => println!("{}",0x3D),
            Instruction::LDAn8(u8) => println!("{}",0x3E),
            Instruction::CCF => println!("{}",0x3F),
            Instruction::LDBB => println!("{}",0x40),
            Instruction::LDBC => println!("{}",0x41),
            Instruction::LDBD => println!("{}",0x42),
            Instruction::LDBE => println!("{}",0x43),
            Instruction::LDBH => println!("{}",0x44),
            Instruction::LDBL => println!("{}",0x45),
            Instruction::LDBHL => println!("{}",0x46),
            Instruction::LDBA => println!("{}",0x47),
            Instruction::LDCB => println!("{}",0x48),
            Instruction::LDCC => println!("{}",0x49),
            Instruction::LDCD => println!("{}",0x4A),
            Instruction::LDCE => println!("{}",0x4B),
            Instruction::LDCH => println!("{}",0x4C),
            Instruction::LDCL => println!("{}",0x4D),
            Instruction::LDCHL => println!("{}",0x4E),
            Instruction::LDCA => println!("{}",0x4F),
            Instruction::LDDB => println!("{}",0x50),
            Instruction::LDDC => println!("{}",0x51),
            Instruction::LDDD => println!("{}",0x52),
            Instruction::LDDE => println!("{}",0x53),
            Instruction::LDDH => println!("{}",0x54),
            Instruction::LDDL => println!("{}",0x55),
            Instruction::LDDHL => println!("{}",0x56),
            Instruction::LDDA => println!("{}",0x57),
            Instruction::LDEB => println!("{}",0x58),
            Instruction::LDEC => println!("{}",0x59),
            Instruction::LDED => println!("{}",0x5A),
            Instruction::LDEE => println!("{}",0x5B),
            Instruction::LDEH => println!("{}",0x5C),
            Instruction::LDEL => println!("{}",0x5D),
            Instruction::LDEHL => println!("{}",0x5E),
            Instruction::LDEA => println!("{}",0x5F),
            Instruction::LDHB => println!("{}",0x60),
            Instruction::LDHC => println!("{}",0x61),
            Instruction::LDHD => println!("{}",0x62),
            Instruction::LDHE => println!("{}",0x63),
            Instruction::LDHH => println!("{}",0x64),
            Instruction::LDHL => println!("{}",0x65),
            Instruction::LDHHL => println!("{}",0x66),
            Instruction::LDHA => println!("{}",0x67),
            Instruction::LDLB => println!("{}",0x68),
            Instruction::LDLC => println!("{}",0x69),
            Instruction::LDLD => println!("{}",0x6A),
            Instruction::LDLE => println!("{}",0x6B),
            Instruction::LDLH => println!("{}",0x6C),
            Instruction::LDLL => println!("{}",0x6D),
            Instruction::LDLHL => println!("{}",0x6E),
            Instruction::LDLA => println!("{}",0x6F),
            Instruction::LDHLB => println!("{}",0x70),
            Instruction::LDHLC => println!("{}",0x71),
            Instruction::LDHLD => println!("{}",0x72),
            Instruction::LDHLE => println!("{}",0x73),
            Instruction::LDHLH => println!("{}",0x74),
            Instruction::LDHLL => println!("{}",0x75),
            Instruction::HALT => println!("{}",0x76),
            Instruction::LDHLA77 => println!("{}",0x77),
            Instruction::LDAB => println!("{}",0x78),
            Instruction::LDAC => println!("{}",0x79),
            Instruction::LDAD => println!("{}",0x7A),
            Instruction::LDAE => println!("{}",0x7B),
            Instruction::LDAH => println!("{}",0x7C),
            Instruction::LDAL => println!("{}",0x7D),
            Instruction::LDAHL7E => println!("{}",0x7E),
            Instruction::LDAA => println!("{}",0x7F),
            Instruction::ADDAB => println!("{}",0x80),
            Instruction::ADDAC => println!("{}",0x81),
            Instruction::ADDAD => println!("{}",0x82),
            Instruction::ADDAE => println!("{}",0x83),
            Instruction::ADDAH => println!("{}",0x84),
            Instruction::ADDAL => println!("{}",0x85),
            Instruction::ADDAHL => println!("{}",0x86),
            Instruction::ADDAA => println!("{}",0x87),
            Instruction::ADCAB => println!("{}",0x88),
            Instruction::ADCAC => println!("{}",0x89),
            Instruction::ADCAD => println!("{}",0x8A),
            Instruction::ADCAE => println!("{}",0x8B),
            Instruction::ADCAH => println!("{}",0x8C),
            Instruction::ADCAL => println!("{}",0x8D),
            Instruction::ADCAHL => println!("{}",0x8E),
            Instruction::ADCAA => println!("{}",0x8F),
            Instruction::SUBAB => println!("{}",0x90),
            Instruction::SUBAC => println!("{}",0x91),
            Instruction::SUBAD => println!("{}",0x92),
            Instruction::SUBAE => println!("{}",0x93),
            Instruction::SUBAH => println!("{}",0x94),
            Instruction::SUBAL => println!("{}",0x95),
            Instruction::SUBAHL => println!("{}",0x96),
            Instruction::SUBAA => println!("{}",0x97),
            Instruction::SBCAB => println!("{}",0x98),
            Instruction::SBCAC => println!("{}",0x99),
            Instruction::SBCAD => println!("{}",0x9A),
            Instruction::SBCAE => println!("{}",0x9B),
            Instruction::SBCAH => println!("{}",0x9C),
            Instruction::SBCAL => println!("{}",0x9D),
            Instruction::SBCAHL => println!("{}",0x9E),
            Instruction::SBCAA => println!("{}",0x9F),
            Instruction::ANDAB => println!("{}",0xA0),
            Instruction::ANDAC => println!("{}",0xA1),
            Instruction::ANDAD => println!("{}",0xA2),
            Instruction::ANDAE => println!("{}",0xA3),
            Instruction::ANDAH => println!("{}",0xA4),
            Instruction::ANDAL => println!("{}",0xA5),
            Instruction::ANDAHL => println!("{}",0xA6),
            Instruction::ANDAA => println!("{}",0xA7),
            Instruction::XORAB => println!("{}",0xA8),
            Instruction::XORAC => println!("{}",0xA9),
            Instruction::XORAD => println!("{}",0xAA),
            Instruction::XORAE => println!("{}",0xAB),
            Instruction::XORAH => println!("{}",0xAC),
            Instruction::XORAL => println!("{}",0xAD),
            Instruction::XORAHL => println!("{}",0xAE),
            Instruction::XORAA => println!("{}",0xAF),
            Instruction::ORAB => println!("{}",0xB0),
            Instruction::ORAC => println!("{}",0xB1),
            Instruction::ORAD => println!("{}",0xB2),
            Instruction::ORAE => println!("{}",0xB3),
            Instruction::ORAH => println!("{}",0xB4),
            Instruction::ORAL => println!("{}",0xB5),
            Instruction::ORAHL => println!("{}",0xB6),
            Instruction::ORAA => println!("{}",0xB7),
            Instruction::CPAB => println!("{}",0xB8),
            Instruction::CPAC => println!("{}",0xB9),
            Instruction::CPAD => println!("{}",0xBA),
            Instruction::CPAE => println!("{}",0xBB),
            Instruction::CPAH => println!("{}",0xBC),
            Instruction::CPAL => println!("{}",0xBD),
            Instruction::CPAHL => println!("{}",0xBE),
            Instruction::CPAA => println!("{}",0xBF),
            Instruction::RETNZ => println!("{}",0xC0),
            Instruction::POPBC => println!("{}",0xC1),
            Instruction::JPNZa16(u16) => println!("{}",0xC2),
            Instruction::JPa16(u16) => println!("{}",0xC3),
            Instruction::CALLNZa16(u16) => println!("{}",0xC4),
            Instruction::PUSHBC => println!("{}",0xC5),
            Instruction::ADDAn8(u8) => println!("{}",0xC6),
            Instruction::RST00 => println!("{}",0xC7),
            Instruction::RETZ => println!("{}",0xC8),
            Instruction::RET => println!("{}",0xC9),
            Instruction::JPZa16(u16) => println!("{}",0xCA),
            Instruction::PREFIX => println!("{}",0xCB),
            Instruction::CALLZa16(u16) => println!("{}",0xCC),
            Instruction::CALLa16(u16) => println!("{}",0xCD),
            Instruction::ADCAn8(u8) => println!("{}",0xCE),
            Instruction::RST08 => println!("{}",0xCF),
            Instruction::RETNC => println!("{}",0xD0),
            Instruction::POPDE => println!("{}",0xD1),
            Instruction::JPNCa16(u16) => println!("{}",0xD2),
            Instruction::ILLEGAL_D3 => println!("{}",0xD3),
            Instruction::CALLNCa16(u16) => println!("{}",0xD4),
            Instruction::PUSHDE => println!("{}",0xD5),
            Instruction::SUBAn8(u8) => println!("{}",0xD6),
            Instruction::RST10 => println!("{}",0xD7),
            Instruction::RETC => println!("{}",0xD8),
            Instruction::RETI => println!("{}",0xD9),
            Instruction::JPCa16(u16) => println!("{}",0xDA),
            Instruction::ILLEGAL_DB => println!("{}",0xDB),
            Instruction::CALLCa16(u16) => println!("{}",0xDC),
            Instruction::ILLEGAL_DD => println!("{}",0xDD),
            Instruction::SBCAn8(u8) => println!("{}",0xDE),
            Instruction::RST18 => println!("{}",0xDF),
            Instruction::LDHa8A => println!("{}",0xE0),
            Instruction::POPHL => println!("{}",0xE1),
            Instruction::LDHCA => println!("{}",0xE2),
            Instruction::ILLEGAL_E3 => println!("{}",0xE3),
            Instruction::ILLEGAL_E4 => println!("{}",0xE4),
            Instruction::PUSHHL => println!("{}",0xE5),
            Instruction::ANDAn8(u8) => println!("{}",0xE6),
            Instruction::RST20 => println!("{}",0xE7),
            Instruction::ADDSPe8(i8) => println!("{}",0xE8),
            Instruction::JPHL => println!("{}",0xE9),
            Instruction::LDa16A(u16) => println!("{}",0xEA),
            Instruction::ILLEGAL_EB => println!("{}",0xEB),
            Instruction::ILLEGAL_EC => println!("{}",0xEC),
            Instruction::ILLEGAL_ED => println!("{}",0xED),
            Instruction::XORAn8(u8) => println!("{}",0xEE),
            Instruction::RST28 => println!("{}",0xEF),
            Instruction::LDHAa8 => println!("{}",0xF0),
            Instruction::POPAF => println!("{}",0xF1),
            Instruction::LDHAC => println!("{}",0xF2),
            Instruction::DI => println!("{}",0xF3),
            Instruction::ILLEGAL_F4 => println!("{}",0xF4),
            Instruction::PUSHAF => println!("{}",0xF5),
            Instruction::ORAn8(u8) => println!("{}",0xF6),
            Instruction::RST30 => println!("{}",0xF7),
            Instruction::LDHLSPe8(i8) => println!("{}",0xF8),
            Instruction::LDSPHL => println!("{}",0xF9),
            Instruction::LDAa16(u16) => println!("{}",0xFA),
            Instruction::EI => println!("{}",0xFB),
            Instruction::ILLEGAL_FC => println!("{}",0xFC),
            Instruction::ILLEGAL_FD => println!("{}",0xFD),
            Instruction::CPAn8(u8) => println!("{}",0xFE),
            Instruction::RST38 => println!("{}",0xFF),
            Instruction::UNKNOWN => panic!("ahhhhhhhhhhhhhhh"),
            _ => panic!("ah"),
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





