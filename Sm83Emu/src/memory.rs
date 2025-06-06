use std::{fs, io};
use std::fmt;



fn check_logo(barr: &[u8]) -> bool {
    // represent to original nintendo tile bmp
    let original_bytes:Vec<u8> = vec![0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
                                    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
                                    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E];
    return *barr == *original_bytes.as_slice();
}

fn compute_checksum(barr: &[u8], checksum: u8) -> bool {
    use GbFileLocations::*;
    let mut comp_checksum: u8 = 0;
    // run from 0x134 to 0x14c to calculate header checksum
    for i  in (TitleS as usize)..(HeaderCheckSum as usize)  {
        comp_checksum = comp_checksum.wrapping_sub(barr[i]);
        comp_checksum = comp_checksum.wrapping_sub(1);
    }
    return comp_checksum == checksum;
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


#[derive(Debug)] // means that the error only will implement the debug trait.
pub enum MmuError {
    NintendoLogoDoesntExists,
    FileReadingErr(io::Error),
    WrongChecksum
}

impl fmt::Display for MmuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NintendoLogoDoesntExists => write!(f, "The Nintendo logo doesn't exists in the binary, finished execution"),
            Self::FileReadingErr(err) => write!(f, "io problem {err:?}"),
            Self::WrongChecksum => write!(f, "Problem while calculating checksum"),
        }
    }
}

pub struct Mmu {
    buffer: Vec<u8>,
}

impl Mmu {
    pub fn new(file_name: &String) -> Result<Self, MmuError> {
        use GbFileLocations::*;
        let gb_file_result= fs::read(file_name);
        let gb_file = match gb_file_result {
            Ok(file) => file,
            Err(error) => return Err(MmuError::FileReadingErr(error))
        };

        // slice to check if the nintedo logo match
        if !check_logo(&gb_file[LogoS as usize..(LogoE as usize + 1)]) {
            return Err(MmuError::NintendoLogoDoesntExists);
        }

        let checksum: u8 = gb_file[HeaderCheckSum as usize]; // get checksum by the byte
        //let compute_checksum: u8 = 
        if !compute_checksum(&gb_file[0..(HeaderCheckSum as usize + 1)], checksum) {
            return Err(MmuError::WrongChecksum);
        }

        return Ok(Mmu{
            buffer: gb_file
        })
        
    }

    // return opcode from where the pc is.
    pub fn evaluate_opcode(self: &Self)  {
        todo!()
    }
}

