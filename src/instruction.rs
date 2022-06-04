use crate::vm::*;

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
    Some(OpCode::LEA) => unimplemented!(),
    Some(OpCode::TRAP) => unimplemented!(),
    _ => {}
  }
}
