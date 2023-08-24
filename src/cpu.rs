use crate::font::FONT_ADDRESS;
use crate::mem::Mem;
use std::{io::Error, path::Path};

pub const OPCODE_SIZE: u16 = 2;
pub const START_ADDRESS: u16 = 0x200;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

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
    pub draw_flag: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut ram = Mem::new();
        ram.load_font();

        Self {
            pc: START_ADDRESS,
            ram,
            stack: [0; 16],
            stack_pointer: 0,
            i_reg: 0,
            v_reg: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            keypad: [false; 16],
            draw_flag: true,
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.process(opcode);
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDRESS;
        self.ram = Mem::new();
        self.ram.load_font();
        self.stack_pointer = 0;
        self.i_reg = 0;
        self.v_reg = [0; 16];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keypad = [false; 16];
        self.clear_display();
    }

    pub fn load_rom(&mut self, path: &Path) -> Result<(), Error> {
        let program = std::fs::read(path)?;

        self.ram.load_program(&program);

        Ok(())
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn display(&self) -> &[bool] {
        &self.display
    }

    pub fn key_press(&mut self, key: usize) {
        self.keypad[key] = true;
    }

    pub fn key_release(&mut self, key: usize) {
        self.keypad[key] = false;
    }

    fn fetch(&mut self) -> u16 {
        let op = self.ram.read_opcode(self.pc);
        self.pc += OPCODE_SIZE;

        op
    }

    fn skip_next_instruction(&mut self) {
        self.pc += OPCODE_SIZE;
    }

    pub fn clear_display(&mut self) {
        self.op_00e0();
    }

    // CLS
    fn op_00e0(&mut self) {
        for pixel in self.display.iter_mut() {
            *pixel = false;
        }

        self.draw_flag = true;
    }

    // RET
    fn op_00ee(&mut self) {
        self.pc = self.stack_pop();
    }

    // JP addr
    fn op_1nnn(&mut self, addr: u16) {
        self.pc = addr;
    }

    // CALL addr
    fn op_2nnn(&mut self, addr: u16) {
        self.stack_push(self.pc);
        self.pc = addr;
    }

    // SE Vx, byte
    fn op_3xnn(&mut self, x: usize, byte: u8) {
        if self.v_reg[x] == byte {
            self.skip_next_instruction();
        };
    }

    // SNE Vx, byte
    fn op_4xnn(&mut self, x: usize, byte: u8) {
        if self.v_reg[x] != byte {
            self.skip_next_instruction();
        };
    }

    // SE Vx, Vy
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v_reg[x] == self.v_reg[y] {
            self.skip_next_instruction();
        };
    }

    // LD Vx, byte
    fn op_6xnn(&mut self, x: usize, byte: u8) {
        self.v_reg[x] = byte;
    }

    // ADD Vx, byte
    fn op_7xnn(&mut self, x: usize, byte: u8) {
        self.v_reg[x] = self.v_reg[x].wrapping_add(byte);
    }

    // LD Vx, Vy
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v_reg[x] = self.v_reg[y];
    }

    // OR Vx, Vy
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v_reg[x] |= self.v_reg[y];
    }

    // AND Vx, Vy
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v_reg[x] &= self.v_reg[y];
    }

    // XOR Vx, Vy
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v_reg[x] ^= self.v_reg[y];
    }

    // ADD Vx, Vy
    fn op_8xy4(&mut self, x: usize, y: usize) {
        match self.v_reg[x].checked_add(self.v_reg[y]) {
            Some(value) => {
                self.v_reg[x] = value;
                self.v_reg[0xF] = 0;
            }
            None => {
                self.v_reg[x] = self.v_reg[x].wrapping_add(self.v_reg[y]);
                self.v_reg[0xF] = 1;
            }
        };
    }

    // SUB Vx, Vy
    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.v_reg[0xF] = if self.v_reg[x] > self.v_reg[y] { 1 } else { 0 };
        self.v_reg[x] = self.v_reg[x].wrapping_sub(self.v_reg[y]);
    }

    // SHR Vx, _Vy
    fn op_8xy6(&mut self, x: usize, _y: usize) {
        // self.v_reg[x] = self.v_reg[_y];
        self.v_reg[0xF] = self.v_reg[x] & 0b0000_0001;

        self.v_reg[x] >>= 1;
    }

    // SUBN Vx, Vy
    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.v_reg[0xF] = if self.v_reg[y] > self.v_reg[x] { 1 } else { 0 };
        self.v_reg[x] = self.v_reg[y].wrapping_sub(self.v_reg[x]);
    }

    // SHL Vx, _Vy
    fn op_8xye(&mut self, x: usize, _y: usize) {
        // self.v_reg[x] = self.v_reg[_y];
        self.v_reg[0xF] = (self.v_reg[x] & 0b1000_0000) >> 7;

        self.v_reg[x] <<= 1;
    }

    // SNE Vx, Vy
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v_reg[x] != self.v_reg[y] {
            self.skip_next_instruction();
        }
    }

    // LD I, addr
    fn op_annn(&mut self, addr: u16) {
        self.i_reg = addr;
    }

    // JP V0, addr
    fn op_bnnn(&mut self, addr: u16) {
        self.pc = addr + self.v_reg[0x0] as u16;
    }

    // RND Vx, byte
    fn op_cxnn(&mut self, x: usize, byte: u8) {
        let random = rand::random::<u8>();

        self.v_reg[x] = random & byte;
    }

    // DRW Vx, Vy, nibble
    fn op_dxyn(&mut self, x: usize, y: usize, nibble: usize) {
        let offset_x = self.v_reg[x] as usize % SCREEN_WIDTH;
        let offset_y = self.v_reg[y] as usize % SCREEN_HEIGHT;

        self.v_reg[0xF] = 0;

        let sprite = self.ram.read(self.i_reg, nibble);
        for (i, pixel_row) in sprite.iter().enumerate() {
            if offset_y + i >= SCREEN_HEIGHT {
                break;
            }

            for j in 0..8 {
                if offset_x + j >= SCREEN_WIDTH {
                    break;
                }

                let pixel = pixel_row & (0b1000_0000 >> j) != 0;
                let pos = (offset_y + i) * SCREEN_WIDTH + (offset_x + j);

                if self.display[pos] && pixel {
                    self.v_reg[0xF] = 1;
                }

                self.display[pos] ^= pixel;
            }
        }

        self.draw_flag = true;
    }

    // SKP Vx
    fn op_ex9e(&mut self, x: usize) {
        if self.keypad[self.v_reg[x] as usize] {
            self.skip_next_instruction();
        }
    }

    // SKNP Vx
    fn op_exa1(&mut self, x: usize) {
        if !self.keypad[self.v_reg[x] as usize] {
            self.skip_next_instruction();
        }
    }

    // LD Vx, DT
    fn op_fx07(&mut self, x: usize) {
        self.v_reg[x] = self.delay_timer;
    }

    // LD Vx, K
    fn op_fx0a(&mut self, x: usize) {
        match self.keypad.iter().position(|key| *key) {
            Some(key_index) => self.v_reg[x] = key_index as u8,
            None => self.pc -= OPCODE_SIZE,
        }
    }

    // LD DT, Vx
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.v_reg[x];
    }

    // LD ST, Vx
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.v_reg[x];
    }

    // ADD I, Vx
    fn op_fx1e(&mut self, x: usize) {
        self.i_reg += self.v_reg[x] as u16;
    }

    // LD F, Vx
    fn op_fx29(&mut self, x: usize) {
        // font located in 0x50
        // each sprite is 5 byte long
        self.i_reg = FONT_ADDRESS + ((self.v_reg[x] & 0xF) * 5) as u16;
    }

    // LD B, Vx
    fn op_fx33(&mut self, x: usize) {
        let num = self.v_reg[x];

        // BCD Conversion
        let byte1 = num / 100;
        let byte2 = (num / 10) % 10;
        let byte3 = num % 10;

        self.ram.write(self.i_reg, &[byte1, byte2, byte3]);
    }

    // LD [I], Vx
    fn op_fx55(&mut self, x: usize) {
        self.ram.write(self.i_reg, &self.v_reg[0..=x]);
    }

    // LD Vx, [I]
    fn op_fx65(&mut self, x: usize) {
        let src = self.ram.read(self.i_reg, x + 1);
        self.v_reg[0..=x].copy_from_slice(src);
    }

    fn process(&mut self, opcode: u16) {
        let nibbles = extract_nibbles(opcode);

        // param extraction
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;
        let nn = (opcode & 0xFF) as u8;
        let nnn = opcode & 0xFFF;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.op_00e0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00ee(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xnn(x, nn),
            (0x4, _, _, _) => self.op_4xnn(x, nn),
            (0x5, _, _, 0x0) => self.op_5xy0(x, y),
            (0x6, _, _, _) => self.op_6xnn(x, nn),
            (0x7, _, _, _) => self.op_7xnn(x, nn),
            (0x8, _, _, 0x0) => self.op_8xy0(x, y),
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),
            (0x8, _, _, 0x6) => self.op_8xy6(x, y),
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),
            (0x8, _, _, 0xE) => self.op_8xye(x, y),
            (0x9, _, _, 0x0) => self.op_9xy0(x, y),
            (0xA, _, _, _) => self.op_annn(nnn),
            (0xB, _, _, _) => self.op_bnnn(nnn),
            (0xC, _, _, _) => self.op_cxnn(x, nn),
            (0xD, _, _, _) => self.op_dxyn(x, y, n),
            (0xE, _, 0x9, 0xE) => self.op_ex9e(x),
            (0xE, _, 0xA, 0x1) => self.op_exa1(x),
            (0xF, _, 0x0, 0x7) => self.op_fx07(x),
            (0xF, _, 0x0, 0xA) => self.op_fx0a(x),
            (0xF, _, 0x1, 0x5) => self.op_fx15(x),
            (0xF, _, 0x1, 0x8) => self.op_fx18(x),
            (0xF, _, 0x1, 0xE) => self.op_fx1e(x),
            (0xF, _, 0x2, 0x9) => self.op_fx29(x),
            (0xF, _, 0x3, 0x3) => self.op_fx33(x),
            (0xF, _, 0x5, 0x5) => self.op_fx55(x),
            (0xF, _, 0x6, 0x5) => self.op_fx65(x),
            _ => (),
        }
    }

    fn stack_push(&mut self, value: u16) {
        self.stack[self.stack_pointer as usize] = value;
        self.stack_pointer += 1;
    }

    fn stack_pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }
}

fn extract_nibbles(word: u16) -> (u8, u8, u8, u8) {
    let nibble_1 = ((word & 0xF000) >> 12) as u8;
    let nibble_2 = ((word & 0xF00) >> 8) as u8;
    let nibble_3 = ((word & 0xF0) >> 4) as u8;
    let nibble_4 = (word & 0xF) as u8;

    (nibble_1, nibble_2, nibble_3, nibble_4)
}

#[cfg(test)]
mod tests;
