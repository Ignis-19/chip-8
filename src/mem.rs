use crate::cpu::START_ADDRESS;
use crate::font::{FONT_ADDRESS, FONT_SPRITE};

pub const MEM_SIZE: usize = 4096;

pub struct Mem {
    memory: [u8; MEM_SIZE],
}

impl Mem {
    pub fn new() -> Self {
        Self {
            memory: [0; MEM_SIZE],
        }
    }

    pub fn load_font(&mut self) {
        self.write(FONT_ADDRESS, &FONT_SPRITE);
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.write(START_ADDRESS, &program);
    }

    pub fn read(&self, addr: u16, length: usize) -> &[u8] {
        let start = addr as usize;
        let end = start + length;

        &self.memory[start..end]
    }

    pub fn write(&mut self, addr: u16, value: &[u8]) {
        let start = addr as usize;
        let end = start + value.len();

        self.memory[start..end].copy_from_slice(value);
    }

    pub fn read_opcode(&self, addr: u16) -> u16 {
        let mut word = [0u8; 2];
        let addr = addr as usize;

        word.copy_from_slice(&self.memory[addr..addr + 2]);

        u16::from_be_bytes(word)
    }
}
