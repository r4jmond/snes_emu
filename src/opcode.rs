use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::cpu::AddressingMode;

pub enum CpuMnemonic {
   BRK,
   LDA,
   LDX,
   LDY,
   TAX,
   INX,
   STA,
}

pub struct Opcode {
   pub hex: u8,
   pub mnemonic: CpuMnemonic,
   pub cycles: u8,
   pub addressing: AddressingMode,
}

impl Opcode {
   fn new(hex: u8, mnemonic: CpuMnemonic, cycles: u8, addressing: AddressingMode) -> Self {
      Opcode {
         hex,
         mnemonic,
         cycles,
         addressing
      }
   }
}

use CpuMnemonic::*;
lazy_static! {
   pub static ref CPU_OPS_CODES: Vec<Opcode> = vec![
      Opcode::new(0x00, BRK, 7, AddressingMode::Implied),

      Opcode::new(0xA9, LDA, 2, AddressingMode::Immediate),
      Opcode::new(0xA5, LDA, 3, AddressingMode::ZeroPage),
      Opcode::new(0xB5, LDA, 4, AddressingMode::ZeroPage_X),
      Opcode::new(0xAD, LDA, 4, AddressingMode::Absolute),
      Opcode::new(0xBD, LDA, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
      Opcode::new(0xB9, LDA, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),
      Opcode::new(0xA1, LDA, 6, AddressingMode::Indirect_X),
      Opcode::new(0xB1, LDA, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),

      Opcode::new(0xA2, LDX, 2, AddressingMode::Immediate),
      Opcode::new(0xA6, LDX, 3, AddressingMode::ZeroPage),
      Opcode::new(0xB6, LDX, 4, AddressingMode::ZeroPage_Y),
      Opcode::new(0xAE, LDX, 4, AddressingMode::Absolute),
      Opcode::new(0xBE, LDX, 4 /* +1 if page crossed */, AddressingMode::Absolute_Y),

      Opcode::new(0xA0, LDY, 2, AddressingMode::Immediate),
      Opcode::new(0xA4, LDY, 3, AddressingMode::ZeroPage),
      Opcode::new(0xB4, LDY, 4, AddressingMode::ZeroPage_X),
      Opcode::new(0xAC, LDY, 4, AddressingMode::Absolute),
      Opcode::new(0xBC, LDY, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),

      Opcode::new(0xAA, TAX, 2, AddressingMode::Implied),
      Opcode::new(0xE8, INX, 2, AddressingMode::Implied),

      Opcode::new(0x85, STA, 3, AddressingMode::ZeroPage),
      Opcode::new(0x95, STA, 4, AddressingMode::ZeroPage_X),
      Opcode::new(0x8D, STA, 4, AddressingMode::Absolute),
      Opcode::new(0x9D, STA, 5, AddressingMode::Absolute_X),
      Opcode::new(0x99, STA, 5, AddressingMode::Absolute_Y),
      Opcode::new(0x81, STA, 6, AddressingMode::Indirect_X),
      Opcode::new(0x91, STA, 6, AddressingMode::Indirect_Y),
   ];

   pub static ref OPCODES_MAP: HashMap<u8, &'static Opcode> = {
      let mut map = HashMap::new();
      CPU_OPS_CODES.iter()
         .for_each(|opcode| { map.insert(opcode.hex, opcode); });
      map
   };
}