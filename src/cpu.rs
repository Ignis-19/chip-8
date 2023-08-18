use crate::mem::Mem;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[allow(unused)]
pub struct Cpu {
    pc: u16,
    ram: Mem,
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    i_reg: u16,
    v_reg: [u8; 16],
}

impl Cpu {
    fn new() -> Self {
        let mut ram = Mem::new();
        ram.load_font();

        Self {
            ram,
            ..Default::default()
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
