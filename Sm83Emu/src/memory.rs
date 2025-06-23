// module to handle Cartridge loading and verifying it
// end result will return the buffer of the cartridge
pub mod cartridge {
    use std::fmt;
    use std::{fs, io};

    // check if a given set of bytes is equal to the logo binary.
    fn check_logo(barr: &[u8]) -> bool {
        // represent to original nintendo tile bmp
        let original_bytes: Vec<u8> = vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        return *barr == *original_bytes.as_slice();
    }

    // compute checksum by given formula and check if equal
    fn compute_checksum(barr: &[u8], checksum: u8) -> bool {
        use GbFileLocations::*;
        let mut comp_checksum: u8 = 0;
        // run from 0x134 to 0x14c to calculate header checksum
        for i in (TitleS as usize)..(HeaderCheckSum as usize) {
            comp_checksum = comp_checksum.wrapping_sub(barr[i]);
            comp_checksum = comp_checksum.wrapping_sub(1);
        }
        return comp_checksum == checksum;
    }

    // all the importent locations in the Cartridge header
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
        HeaderCheckSum = 0x14D,
    }

    #[allow(dead_code)]
    #[derive(Debug)] // means that the error only will implement the debug trait.
    pub enum CartridgeErr {
        NintendoLogoDoesntExists,
        FileReadingErr(io::Error),
        WrongChecksum,
    }

    // display the error in a nice format.
    impl fmt::Display for CartridgeErr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::NintendoLogoDoesntExists => write!(
                    f,
                    "The Nintendo logo doesn't match to the original one, Locking cpu..."
                ),
                Self::FileReadingErr(err) => write!(f, "io problem {err:?} locking cpu..."),
                Self::WrongChecksum => {
                    write!(f, "Problem while calculating checksum, locking cpu...")
                }
            }
        }
    }

    pub fn get_cartridge_buffer(file_name: &str) -> Result<Vec<u8>, CartridgeErr> {
        use GbFileLocations::*;
        let gb_file_result = fs::read(file_name);
        let gb_file = match gb_file_result {
            Ok(file) => file,
            Err(error) => return Err(CartridgeErr::FileReadingErr(error)),
        };

        // slice to check if the nintedo logo match
        if !check_logo(&gb_file[LogoS as usize..(LogoE as usize + 1)]) {
            return Err(CartridgeErr::NintendoLogoDoesntExists);
        }

        let checksum: u8 = gb_file[HeaderCheckSum as usize]; // get checksum by the byte
        //let compute_checksum: u8 =
        if !compute_checksum(&gb_file[0..(HeaderCheckSum as usize + 1)], checksum) {
            return Err(CartridgeErr::WrongChecksum);
        }

        return Ok(gb_file);
    }
}

pub mod memory {
    use crate::memory::cartridge;
    use cartridge::get_cartridge_buffer;

    const ROM_SIZE: usize = 0x10000;
    pub const VRAM_START: usize = 0x8000;
    pub const VRAM_END: usize = 0x9FFF;

    pub struct Mmu {
        pub memory_map: [u8; ROM_SIZE], // will store the main program memory.
    }

    impl Mmu {
        pub fn new(cartridge_name: &str) -> Self {
            let crt: Result<Vec<u8>, cartridge::CartridgeErr> =
                get_cartridge_buffer(cartridge_name);
            // get the buffer or else panic and print error.
            let cartridge_buffer: Vec<u8> = match crt {
                Ok(mm) => mm,
                Err(error) => panic!("{:?}", error),
            };

            // copy array to rom, mapping it.
            let mut rom: [u8; ROM_SIZE] = [0; ROM_SIZE];
            for n in 0..cartridge_buffer.len() {
                rom[n] = cartridge_buffer[n];
            }

            // set values for i/o ports.
            rom[0xFF05] = 0x00;
            rom[0xFF06] = 0x00;
            rom[0xFF07] = 0x00;
            rom[0xFF10] = 0x80;
            rom[0xFF11] = 0xBF;
            rom[0xFF12] = 0xF3;
            rom[0xFF14] = 0xBF;
            rom[0xFF16] = 0x3F;
            rom[0xFF17] = 0x00;
            rom[0xFF19] = 0xBF;
            rom[0xFF1A] = 0x7F;
            rom[0xFF1B] = 0xFF;
            rom[0xFF1C] = 0x9F;
            rom[0xFF1E] = 0xBF;
            rom[0xFF20] = 0xFF;
            rom[0xFF21] = 0x00;
            rom[0xFF22] = 0x00;
            rom[0xFF23] = 0xBF;
            rom[0xFF24] = 0x77;
            rom[0xFF25] = 0xF3;
            rom[0xFF26] = 0xF1;
            rom[0xFF40] = 0x91;
            rom[0xFF42] = 0x00;
            rom[0xFF43] = 0x00;
            rom[0xFF45] = 0x00;
            rom[0xFF47] = 0xFC;
            rom[0xFF48] = 0xFF;
            rom[0xFF49] = 0xFF;
            rom[0xFF4A] = 0x00;
            rom[0xFF4B] = 0x00;
            rom[0xFFFF] = 0x00;

            return Mmu { memory_map: rom };
        }

        pub fn read_byte(self: &Self, address: u16) -> u8 {
            return self.memory_map[address as usize];
        }

        // used in realy rear cases
        pub fn read_sbyte(self: &Self, address: u16) -> i8 {
            return self.memory_map[address as usize].try_into().unwrap();
        }

        pub fn write_byte(self: &mut Self, address: u16, data: u8) {
            self.memory_map[address as usize] = data; // write data to certion address.
        }
    }
}
