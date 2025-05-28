mod cpu;
use crate::cpu::{Cpu, Registers};




fn main() {
    let reg = Registers {
        a: 1,
        b: 1,
        c: 1,
        d: 1,
        e: 1,
        f: 1,
        h: 1,
        l: 1,
        sp: 1,
        ie: 2,
        ir: 2,
        pc: 2
    };

    let cpu = Cpu{
        registers: reg
    };
    let p = cpu.registers.get_bc();
    println!("{p}");
    println!("Hello, world!");
}
