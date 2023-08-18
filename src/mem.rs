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
        self.memory[0..0x50].copy_from_slice(&FONT_SPRITE);
    }
}
