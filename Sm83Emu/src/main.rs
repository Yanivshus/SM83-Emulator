use crate::cpu::{Cpu};
mod memory;
mod cpu;


fn main() {
    let mut cpu = Cpu::new(&String::from("/home/kaish/Downloads/Calc.gb"));
    let inst = cpu.decode_instrcution();
    cpu.execute_instruction(&inst);
    let inst = cpu.decode_instrcution();
    cpu.execute_instruction(&inst);
    let inst = cpu.decode_instrcution();
    cpu.execute_instruction(&inst);
    let inst = cpu.decode_instrcution();
    cpu.execute_instruction(&inst);
    let inst = cpu.decode_instrcution();
    cpu.execute_instruction(&inst);
    let inst = cpu.decode_instrcution();
    cpu.execute_instruction(&inst);
    let inst = cpu.decode_instrcution();
    cpu.execute_instruction(&inst);
    let inst = cpu.decode_instrcution();
    cpu.execute_instruction(&inst);
}
