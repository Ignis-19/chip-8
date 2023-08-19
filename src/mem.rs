use crate::cpu::START_ADDRESS;
use crate::font::FONT_SPRITE;

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
        self.memory[0..FONT_SPRITE.len()].copy_from_slice(&FONT_SPRITE);
    }

    pub fn load_program(&mut self, program: &[u8]) {
        let (_reserved, memory) = self.memory.split_at_mut(START_ADDRESS);
        memory[0..program.len()].copy_from_slice(program);
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let mut word = [0u8; 2];
        let addr = address as usize;

        word.copy_from_slice(&self.memory[addr..addr + 2]);

        u16::from_be_bytes(word)
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        let word = value.to_be_bytes();
        let addr = address as usize;

        self.memory[addr..addr + 2].copy_from_slice(&word);
    }
}
