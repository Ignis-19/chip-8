use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::path::Path;
use std::time::Duration;

mod cpu;
mod font;
mod mem;

const SCALE: u32 = 7;
const WINDOW_NAME: &str = "CHIP-8 Emulator";
const WINDOW_WIDTH: u32 = cpu::SCREEN_WIDTH as u32 * SCALE;
const WINDOW_HEIGHT: u32 = cpu::SCREEN_HEIGHT as u32 * SCALE;
const BG_COLOR: Color = Color::RGB(49, 57, 66);
const FG_COLOR: Color = Color::RGB(216, 222, 233);
const FRAME_RATE: Duration = Duration::new(0, 1_000_000_000 / 60);

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsytem = sdl_ctx.video().unwrap();
    let window = video_subsytem
        .window(WINDOW_NAME, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    canvas.set_draw_color(BG_COLOR);
    canvas.clear();
    canvas.present();

    let program_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| show_error_message("missing path to rom file", canvas.window()));

    let mut cpu = cpu::Cpu::new();
    cpu.load_rom(Path::new(&program_path))
        .unwrap_or_else(|err| show_error_message(&err.to_string(), canvas.window()));

    'emu: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'emu,
                Event::KeyDown { scancode, .. } => {
                    if let Some(key) = get_key(scancode) {
                        cpu.key_press(key);
                    }
                }
                Event::KeyUp { scancode, .. } => {
                    if let Some(key) = get_key(scancode) {
                        cpu.key_release(key);
                    }
                }
                _ => (),
            }
        }

        if cpu.draw_flag {
            for (i, pixel) in cpu.display().iter().enumerate() {
                let color = if *pixel { FG_COLOR } else { BG_COLOR };
                let x = (i % cpu::SCREEN_WIDTH) as i32 * SCALE as i32;
                let y = (i / cpu::SCREEN_WIDTH) as i32 * SCALE as i32;
                let rect = sdl2::rect::Rect::new(x, y, SCALE, SCALE);

                canvas.set_draw_color(color);
                canvas.fill_rect(rect).unwrap();
            }

            canvas.present();
            cpu.draw_flag = false;
        }

        for _ in 0..10 {
            cpu.tick();
        }

        cpu.tick_timers();
        ::std::thread::sleep(FRAME_RATE);
    }
}

fn get_key(scancode: Option<Scancode>) -> Option<usize> {
    match scancode {
        Some(Scancode::Num1) => Some(0x1),
        Some(Scancode::Num2) => Some(0x2),
        Some(Scancode::Num3) => Some(0x3),
        Some(Scancode::Num4) => Some(0xC),
        Some(Scancode::Q) => Some(0x4),
        Some(Scancode::W) => Some(0x5),
        Some(Scancode::E) => Some(0x6),
        Some(Scancode::R) => Some(0xD),
        Some(Scancode::A) => Some(0x7),
        Some(Scancode::S) => Some(0x8),
        Some(Scancode::D) => Some(0x9),
        Some(Scancode::F) => Some(0xE),
        Some(Scancode::Z) => Some(0xA),
        Some(Scancode::X) => Some(0x0),
        Some(Scancode::C) => Some(0xB),
        Some(Scancode::V) => Some(0xF),
        _ => None,
    }
}

fn show_error_message(message: &str, window: &sdl2::video::Window) -> ! {
    use sdl2::messagebox::{show_simple_message_box, MessageBoxFlag};

    show_simple_message_box(MessageBoxFlag::ERROR, "Error", message, window).unwrap();
    std::process::exit(1);
}
