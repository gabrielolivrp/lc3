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

pub fn execute_instr(instr: u16, vm: &mut Vm) {
  let op_code = get_op_code(&instr);
  match op_code {
    Some(OpCode::BR) => br(instr, vm),
    Some(OpCode::ADD) => add(instr, vm),
    Some(OpCode::LD) => ld(instr, vm),
    Some(OpCode::ST) => unimplemented!(),
    Some(OpCode::JSR) => jsr(instr, vm),
    Some(OpCode::AND) => and(instr, vm),
    Some(OpCode::LDR) => ldr(instr, vm),
    Some(OpCode::STR) => unimplemented!(),
    Some(OpCode::RTI) => unimplemented!(),
    Some(OpCode::NOT) => not(instr, vm),
    Some(OpCode::LDI) => ldi(instr, vm),
    Some(OpCode::STI) => unimplemented!(),
    Some(OpCode::JMP) => jmp(instr, vm),
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
  // registrador destino
  let dr = (instr >> 9) & 0x7;
  // offset estendido para 16bits
  let pc_offset = sign_extend(instr & 0x1ff, 9);

  // o valor que vamos armazenar no registrador destino (dr), vai ser
  // o valor do contador de programa somado com o deslocamento
  let value = vm.registers.pc as u32 + pc_offset as u32; // u32 para prevenir overflow

  vm.registers.update(dr, value as u16);
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
      // endereço base
      let mut index = vm.registers.r0;
      // leitura do caracter que está no endereço
      let mut ch = vm.read_memory(index) as u8;

      // vai percorrer até encontrar o endereço 0x0000
      while ch != 0x0000 {
        // mostra o caracter na tela
        print!("{}", ch as char);
        // adiciona mais uam posição no endereço
        index += 1;
        // faz a leitura do caracter que está no novo endereço
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

fn add(instr: u16, vm: &mut Vm) {
  // registrador destino
  let dr = (instr >> 9) & 0x7;
  // registrador do primeiro operador
  let sr1 = (instr >> 6) & 0x7;
  // pega a flag que indica se o segundo operando, é imediato ou está
  // em algum outro registrador
  let imm_flag = (instr >> 5) & 0x1;
  // caso a operaçao ocorrer em modo imediato, o valor deve ser igual a 1
  if imm_flag == 1 {
    // pega o valor imediato
    let imm5 = sign_extend(instr & 0x1f, 5);
    // u32 para prevenir overflow
    let value = imm5 as u32 + vm.registers.get(sr1) as u32;
    vm.registers.update(dr, value as u16);
  } else {
    // pega o registrador do segundo
    let sr2 = instr & 0x7;
    // realiza a soma dos dois valores
    let value = vm.registers.get(sr1) as u32 + vm.registers.get(sr2) as u32;
    vm.registers.update(dr, value as u16);
  }
  vm.registers.update_r_cond_register(dr)
}

fn ldi(instr: u16, vm: &mut Vm) {
  // registrador destino
  let dr = (instr >> 9) & 0x7;
  // deslocamento já com o sinal estendido
  let pc_offset = sign_extend(instr & 0x1ff, 9);
  // realiza a leitura do endereço resultante da soma do registrador pc com o
  // valor de offset
  let indirect_address = vm.read_memory(vm.registers.pc + pc_offset);
  // realiza a leitura do endereço obtido
  let value = vm.read_memory(indirect_address);
  vm.registers.update(dr, value);
  vm.registers.update_r_cond_register(dr)
}

fn and(instr: u16, vm: &mut Vm) {
  // registrador destino
  let dr = (instr >> 9) & 0x7;
  // registrador do primeiro operando
  let sr1 = (instr >> 6) & 0x7;
  // flag que indica o modo da operação
  let imm_flag = (instr >> 5) & 0x1;
  // caso a flag seja igual a 1, a operação será em modo imediato
  if imm_flag == 1 {
    // valor imediato
    let imm5 = sign_extend(instr & 0x1F, 5);
    vm.registers.update(dr, vm.registers.get(sr1) & imm5);
  } else {
    // registrador do segundo operando
    let sr2 = instr & 0x7;
    // realiza a operação
    let value = vm.registers.get(sr1) & vm.registers.get(sr2);
    vm.registers.update(dr, value);
  }
  vm.registers.update_r_cond_register(dr)
}

fn not(instr: u16, vm: &mut Vm) {
  // registrador destino
  let dr = (instr >> 9) & 0x7;
  // registrador do operando
  let sr1 = (instr >> 6) & 0x7;
  // obtem o valor do operando
  let value = vm.registers.get(sr1);
  // salvar o valor do operando negado, no registrador destino
  vm.registers.update(dr, !value);
  vm.registers.update_r_cond_register(dr);
}

fn br(instr: u16, vm: &mut Vm) {
  let pc_offset = sign_extend(instr & 0x1ff, 9);
  let cond_flag = (instr >> 9) & 0x7;
  if cond_flag & vm.registers.cond != 0 {
    let value = vm.registers.pc as u32 + pc_offset as u32;
    vm.registers.pc = value as u16;
  }
}

fn jmp(instr: u16, vm: &mut Vm) {
  // registrador base
  let base_reg = (instr >> 6) & 0x7;
  // move o `pc` para o novo endereço
  vm.registers.pc = vm.registers.get(base_reg);
}

fn jsr(instr: u16, vm: &mut Vm) {
  // registrador base
  let base_reg = (instr >> 6) & 0x7;
  // offset extendido para 16bits
  let long_pc_offset = sign_extend(instr & 0x7ff, 11);
  let long_flag = (instr >> 11) & 1;

  vm.registers.r7 = vm.registers.pc;

  if long_flag != 0 {
    // caso JSR, o endereço para saltar é calculado a partir do deslocamento do PC offset
    let value = vm.registers.pc as u32 + long_pc_offset as u32;
    vm.registers.pc = value as u16;
  } else {
    // caso JSRR, o endereço para saltar esta no registrador base
    vm.registers.pc = vm.registers.get(base_reg);
  }
}

fn ld(instr: u16, vm: &mut Vm) {
  // registrador destino
  let dr = (instr >> 9) & 0x7;
  // deslocamento
  let pc_offset = sign_extend(instr & 0x1ff, 9);
  // endereço do valor na memória
  let mem = pc_offset as u32 + vm.registers.pc as u32;
  // obtem o valor que está na memória
  let value = vm.read_memory(mem as u16);
  // carrega no registrador destino
  vm.registers.update(dr, value);
  vm.registers.update_r_cond_register(dr);
}

fn ldr(instr: u16, vm: &mut Vm) {
  // registrador destino
  let dr = (instr >> 9) & 0x7;
  // registrador base
  let base_reg = (instr >> 6) & 0x7;
  // deslocamento
  let offset = sign_extend(instr & 0x3F, 6);
  // endereço do valor na memória
  let mem = vm.registers.get(base_reg) as u32 + offset as u32;
  // valor que está no endereço da memória obtido
  let value = vm.read_memory(mem as u16).clone();
  // salvar o valor na memória no registrador destino
  vm.registers.update(dr, value);
  vm.registers.update_r_cond_register(dr);
}
