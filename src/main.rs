pub mod instruction;
pub mod registers;
pub mod vm;

// use crate::registers::*;
use crate::vm::*;

use byteorder::{BigEndian, ReadBytesExt};
use std::env;
use std::{fs, io};

fn main() {
  let args: Vec<String> = env::args().collect();
  let f = fs::File::open(args[1].clone()).expect("Unable to read file");
  let mut f = io::BufReader::new(f);
  let base_address = f.read_u16::<BigEndian>().expect("error");
  let mut address = base_address as usize;
  let mut vm = Vm::new();
  loop {
    match f.read_u16::<BigEndian>() {
      Ok(instruction) => {
        vm.write_memory(address, instruction);
        address += 1
      }
      Err(err) => {
        if err.kind() == std::io::ErrorKind::UnexpectedEof {
          println!("OK")
        } else {
          println!("failed: {}", err)
        }
        break;
      }
    }
  }

  execute_program(&mut vm);
}
