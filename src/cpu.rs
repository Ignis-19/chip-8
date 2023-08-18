use crate::mem::Mem;
use std::{io::Error, path::Path};

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const START_ADDRESS: usize = 0x200;

#[allow(unused)]
pub struct Cpu {
    pc: u16,
    ram: Mem,
    stack: [u16; 16],
    stack_pointer: u8,
    i_reg: u16,
    v_reg: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    keypad: [bool; 16],
}

impl Cpu {
    pub fn new() -> Self {
        let mut ram = Mem::new();
        ram.load_font();

        Self {
            pc: START_ADDRESS as u16,
            ram,
            ..Default::default()
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch();
            // TODO: decode() & execute()
        }
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDRESS as u16;
        self.ram = Mem::new();
        self.ram.load_font();
        self.stack_pointer = 0;
        self.i_reg = 0;
        self.v_reg = [0; 16];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keypad = [false; 16];
    }

    pub fn load_rom(&mut self, path: &Path) -> Result<(), Error> {
        let program = std::fs::read(path)?;

        self.ram.load_program(&program);

        Ok(())
    }

    fn fetch(&mut self) -> u16 {
        self.pc += 2;
        self.ram.read_u16(self.pc)
    }

    fn push(&mut self, value: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = value;
    }

    fn pop(&mut self, value: u16) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
