use crate::instr::*;
use crate::registers::*;

pub const MEMORY_SIZE: usize = u16::MAX as usize;

pub struct Vm {
  pub memory: [u16; MEMORY_SIZE],
  pub registers: Registers,
}

impl Vm {
  pub fn new() -> Vm {
    Vm {
      memory: [0; MEMORY_SIZE],
      registers: Registers::new(),
    }
  }

  pub fn write_memory(&mut self, address: usize, value: u16) {
    self.memory[address] = value
  }

  pub fn read_memory(&mut self, address: u16) -> u16 {
    self.memory[address as usize]
  }
}

pub fn execute_program(vm: &mut Vm) {
  while vm.registers.pc < MEMORY_SIZE as u16 {
    // read instruction
    let instr = vm.read_memory(vm.registers.pc);
    // increment program counter
    vm.registers.pc += 1;
    // Extract OP code and execute instruction
    execute_instr(instr, vm);
  }
}
