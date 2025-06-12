use core::panic;

use cpu::Registers;

use crate::cpu::{Cpu};
use crate::memory::memory::Mmu;
mod memory;
mod cpu;

// will represent the program opcodes.


// for now we will work without MBC -> small games  

fn main() {
    let mm = Mmu::new(&String::from("/home/kaish/Downloads/Calc.gb"));
    print!("{:#X}",mm.memory_map.len());
}
