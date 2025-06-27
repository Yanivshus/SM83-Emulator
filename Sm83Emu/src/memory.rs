pub mod memory {
    use crate::cartridge::cartridge;
    use crate::cartridge::cartridge::get_cartridge_buffer;

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
            self.memory_map[address as usize]
        }

        // used in realy rear cases
        pub fn read_sbyte(self: &Self, address: u16) -> i8 {
            unsafe { std::mem::transmute(self.memory_map[address as usize]) }
        }

        pub fn write_byte(self: &mut Self, address: u16, data: u8) {
            self.memory_map[address as usize] = data; // write data to certion address.
        }
    }
}
