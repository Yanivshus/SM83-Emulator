use core::panic;

use crate::memory::{Mmu};
use crate::cpu::{Cpu};
mod memory;
mod cpu;

// will represent the program opcodes.


// for now we will work without MBC -> small games

fn main() {

    let gb: Result<Mmu, memory::MmuError> = Mmu::new(&String::from("/home/kaish/Downloads/Calc.gb"));
    let fl = match gb {
        Ok(mm) => mm,
        Err(error) => panic!("hell yeahhh")
    };

    println!("{:#X}", fl.buffer[0x150]);



}
