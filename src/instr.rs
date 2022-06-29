use crate::vm::*;
use std::io;
use std::io::Write;
use std::process;

pub enum OpCode {
  BR = 0, // branch
  ADD,    // add
  LD,     // load
  ST,     // store
  JSR,    // jump register
  AND,    // bitwise and
  LDR,    // load register
  STR,    // store register
  RTI,    // unused
  NOT,    // bitwise not
  LDI,    // load indirect
  STI,    // store indirect
  JMP,    // jump
  RES,    // reserved (unused)
  LEA,    // load effective address
  TRAP,   // execute trap
}

pub fn get_op_code(instr: &u16) -> Option<OpCode> {
  match instr >> 12 {
    0 => Some(OpCode::BR),
    1 => Some(OpCode::ADD),
    2 => Some(OpCode::LD),
    3 => Some(OpCode::ST),
    4 => Some(OpCode::JSR),
    5 => Some(OpCode::AND),
    6 => Some(OpCode::LDR),
    7 => Some(OpCode::STR),
    8 => Some(OpCode::RTI),
    9 => Some(OpCode::NOT),
    10 => Some(OpCode::LDI),
    11 => Some(OpCode::STI),
    12 => Some(OpCode::JMP),
    13 => Some(OpCode::RES),
    14 => Some(OpCode::LEA),
    15 => Some(OpCode::TRAP),
    _ => None,
  }
}

pub fn execute_instruction(instr: u16, vm: &mut Vm) {
  let op_code = get_op_code(&instr);
  match op_code {
    Some(OpCode::BR) => unimplemented!(),
    Some(OpCode::ADD) => unimplemented!(),
    Some(OpCode::LD) => unimplemented!(),
    Some(OpCode::ST) => unimplemented!(),
    Some(OpCode::JSR) => unimplemented!(),
    Some(OpCode::AND) => unimplemented!(),
    Some(OpCode::LDR) => unimplemented!(),
    Some(OpCode::STR) => unimplemented!(),
    Some(OpCode::RTI) => unimplemented!(),
    Some(OpCode::NOT) => unimplemented!(),
    Some(OpCode::LDI) => unimplemented!(),
    Some(OpCode::STI) => unimplemented!(),
    Some(OpCode::JMP) => unimplemented!(),
    Some(OpCode::RES) => unimplemented!(),
    Some(OpCode::LEA) => lea(instr, vm),
    Some(OpCode::TRAP) => trap(instr, vm),
    _ => {}
  }
}

/// Extensão de sinal é uma operação para aumentar o número de bits de um número binário,
/// preservando o sinal do número (positivo ou negativo) e seu valor
/// Exemplo:
/// 00 1010 (6 bits) => 0000 0000 0000 1010 (16 bits)
fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
  if (x >> (bit_count - 1)) & 1 != 0 {
    x |= 0xffff << bit_count;
  }
  x
}

fn lea(instr: u16, vm: &mut Vm) {
  let dr = (instr >> 9) & 0x7;

  let pc_offset = sign_extend(instr & 0x1ff, 9);

  let val = vm.registers.pc as u32 + pc_offset as u32;

  vm.registers.update(dr, val as u16);

  vm.registers.update_r_cond_register(dr);
}

fn trap(instr: u16, vm: &mut Vm) {
  match instr & 0xFF {
    0x20 => {
      // get char
    }
    0x21 => {
      // out
    }
    0x22 => {
      // puts
      let mut index = vm.registers.r0;
      let mut ch = vm.read_memory(index) as u8;
      while ch != 0x0000 {
        print!("{}", ch as char);
        index += 1;
        ch = vm.read_memory(index) as u8;
      }
      io::stdout().flush().expect("failed to flush");
    }
    0x23 => {
      // in
    }
    0x24 => {
      // putsp
    }
    0x25 => {
      // halt
      println!("HALT detected");
      io::stdout().flush().expect("failed to flush");
      process::exit(1)
    }
    _ => process::exit(1),
  }
}
