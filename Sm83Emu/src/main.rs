use crate::cpu::Cpu;
mod cartridge;
mod cpu;
mod gfx;
mod memory;

fn main() {
    let mut cpu = Cpu::new(&String::from("/home/kaish/Downloads/Calc.gb"));
    cpu.registers.a = 0b11001100;
    cpu.execute_instruction(&cpu::Instruction::RLCA);
    println!("{}", cpu.registers);
}
