use super::*;

fn set_up_cpu(program: &[u8]) -> Cpu {
    let mut cpu = Cpu::new();
    cpu.ram.load_program(program);

    cpu
}

#[test]
fn test_00e0() {
    let mut cpu = set_up_cpu(&[0x00, 0xE0]);
    cpu.display[13] = true;
    cpu.display[45] = true;

    cpu.tick();

    assert!(cpu.display.iter().all(|px| *px == false));
}

#[test]
fn op_00ee() {
    let mut cpu = set_up_cpu(&[0x00, 0xEE]);
    cpu.stack_push(599);

    cpu.tick();

    assert_eq!(cpu.pc, 599);
}

#[test]
fn test_1nnn() {
    let mut cpu = set_up_cpu(&[0x1A, 0xBC]);

    cpu.tick();

    assert_eq!(cpu.pc, 0xABC);
}

#[test]
fn test_2nnn() {
    let mut cpu = set_up_cpu(&[0x23, 0xCD]);

    cpu.tick();

    assert_eq!(cpu.pc, 0x3CD);
    assert_eq!(cpu.stack_pop(), 0x202);
}

#[test]
fn test_3xnn() {
    let mut cpu = set_up_cpu(&[0x30, 0xEE, 0x30, 0xAA]);
    cpu.v_reg[0] = 0xEE;

    assert_eq!(cpu.pc, 0x200);

    cpu.tick();

    assert_eq!(cpu.pc, 0x204);

    cpu.tick();

    assert_eq!(cpu.pc, 0x206);
}

#[test]
fn test_4xnn() {
    let mut cpu = set_up_cpu(&[0x40, 0x88]);

    cpu.tick();

    assert_eq!(cpu.pc, 0x204);
}

#[test]
fn test_5xy0() {
    let mut cpu = set_up_cpu(&[0x5A, 0xF0]);
    cpu.v_reg[0xA] = 0xFF;
    cpu.v_reg[0xF] = 0xFF;

    cpu.tick();

    assert_eq!(cpu.pc, 0x204);
}

#[test]
fn test_6xnn() {
    let mut cpu = set_up_cpu(&[0x6B, 0x32]);

    cpu.tick();

    assert_eq!(cpu.v_reg[0xB], 0x32);
}

#[test]
fn test_7xnn() {
    let mut cpu = set_up_cpu(&[0x7A, 0xC]);
    cpu.v_reg[0xA] = 0x3;

    cpu.tick();

    assert_eq!(cpu.v_reg[0xA], 0xF);
}

#[test]
fn test_8xy0() {
    let mut cpu = set_up_cpu(&[0x80, 0x10]);
    cpu.v_reg[0x1] = 0x12;

    cpu.tick();

    assert_eq!(cpu.v_reg[0x0], 0x12);
}

#[test]
fn test_8xy1() {
    let mut cpu = set_up_cpu(&[0x80, 0x11]);
    cpu.v_reg[0x0] = 0b1001;
    cpu.v_reg[0x1] = 0b1101;

    cpu.tick();

    assert_eq!(cpu.v_reg[0x0], 0b1101);
}

#[test]
fn test_8xy2() {
    let mut cpu = set_up_cpu(&[0x80, 0x12]);
    cpu.v_reg[0x0] = 0b0011;
    cpu.v_reg[0x1] = 0b1010;

    cpu.tick();

    assert_eq!(cpu.v_reg[0x0], 0b10);
}

#[test]
fn test_8xy3() {
    let mut cpu = set_up_cpu(&[0x80, 0x13]);
    cpu.v_reg[0x0] = 0b1001;
    cpu.v_reg[0x1] = 0b1101;

    cpu.tick();

    assert_eq!(cpu.v_reg[0x0], 0b100);
}

#[test]
fn test_8xy4() {
    let mut cpu = set_up_cpu(&[0x80, 0x14, 0x80, 0x14]);
    cpu.v_reg[0x0] = 0x1B;
    cpu.v_reg[0x1] = 0xC;

    cpu.tick();

    assert_eq!(cpu.v_reg[0x0], 0x27);
    assert_eq!(cpu.v_reg[0xF], 0);

    cpu.v_reg[0x1] = 0xF1;

    cpu.tick();

    assert_eq!(cpu.v_reg[0x0], 0x18);
    assert_eq!(cpu.v_reg[0xF], 1);
}

#[test]
fn test_8xy5() {
    let mut cpu = set_up_cpu(&[0x80, 0x15]);
    cpu.v_reg[0x0] = 0xA;
    cpu.v_reg[0x1] = 0x6;

    cpu.tick();

    assert_eq!(cpu.v_reg[0x0], 0x4);
    assert_eq!(cpu.v_reg[0xF], 1);
}

#[test]
fn test_8xy6() {
    let mut cpu = set_up_cpu(&[0x80, 0x16]);
    cpu.v_reg[0x0] = 0b10010;

    cpu.tick();

    assert_eq!(cpu.v_reg[0xF], 0);
    assert_eq!(cpu.v_reg[0x0], 0b1001);
}

#[test]
fn test_8xy7() {
    let mut cpu = set_up_cpu(&[0x80, 0x17]);
    cpu.v_reg[0x0] = 0x43;
    cpu.v_reg[0x1] = 0x30;

    cpu.tick();

    assert_eq!(cpu.v_reg[0xF], 0);
    assert_eq!(cpu.v_reg[0x0], 0xED);
}

#[test]
fn test_8xye() {
    let mut cpu = set_up_cpu(&[0x80, 0x0E]);
    cpu.v_reg[0x0] = 0b1001_1000;

    cpu.tick();

    assert_eq!(cpu.v_reg[0xF], 1);
    assert_eq!(cpu.v_reg[0x0], 0b11_0000);
}

#[test]
fn test_9xy0() {
    let mut cpu = set_up_cpu(&[0x90, 0x10]);
    cpu.v_reg[0x0] = 0x12;
    cpu.v_reg[0x1] = 0x13;

    cpu.tick();

    assert_eq!(cpu.pc, 0x204);
}

#[test]
fn test_annn() {
    let mut cpu = set_up_cpu(&[0xA1, 0xFC]);

    cpu.tick();

    assert_eq!(cpu.i_reg, 0x1FC);
}

#[test]
fn test_bnnn() {
    let mut cpu = set_up_cpu(&[0xBA, 0x11]);
    cpu.v_reg[0x0] = 0x3;

    cpu.tick();

    assert_eq!(cpu.pc, 0xA14);
}

#[test]
fn test_dxyn() {
    let mut cpu = set_up_cpu(&[0xD0, 0x12]);
    cpu.display[SCREEN_WIDTH] = true;
    cpu.ram.write(0, &[0b1011_0001, 0b1111_0010]);

    cpu.tick();

    assert_eq!(
        &cpu.display[0..8],
        &[true, false, true, true, false, false, false, true]
    );
    assert_eq!(
        &cpu.display[SCREEN_WIDTH..SCREEN_WIDTH + 8],
        &[false, true, true, true, false, false, true, false]
    );

    assert_eq!(cpu.v_reg[0xF], 1);
}

#[test]
fn test_ex9e() {
    let mut cpu = set_up_cpu(&[0xE0, 0x9E]);
    cpu.v_reg[0x0] = 0x7;
    cpu.keypad[0x7] = true;

    cpu.tick();

    assert_eq!(cpu.pc, 0x204)
}

#[test]
fn test_exa1() {
    let mut cpu = set_up_cpu(&[0xE0, 0xA1]);
    cpu.v_reg[0x0] = 0x7;
    cpu.keypad[0x7] = false;

    cpu.tick();

    assert_eq!(cpu.pc, 0x204)
}

#[test]
fn test_fx07() {
    let mut cpu = set_up_cpu(&[0xFC, 0x07]);
    cpu.delay_timer = 41;

    cpu.tick();

    assert_eq!(cpu.v_reg[0xC], 41);
}

#[test]
fn test_fx0a() {
    let mut cpu = set_up_cpu(&[0xF0, 0x0A]);

    cpu.tick();

    assert_eq!(cpu.pc, 0x200);

    cpu.keypad[0xE] = true;

    cpu.tick();

    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.v_reg[0x0], 0xE);
}

#[test]
fn test_fx15() {
    let mut cpu = set_up_cpu(&[0xF0, 0x15]);
    cpu.v_reg[0x0] = 56;

    cpu.tick();

    assert_eq!(cpu.delay_timer, 56);
}

#[test]
fn test_fx18() {
    let mut cpu = set_up_cpu(&[0xF0, 0x18]);
    cpu.v_reg[0x0] = 56;

    cpu.tick();

    assert_eq!(cpu.sound_timer, 56);
}

#[test]
fn test_fx1e() {
    let mut cpu = set_up_cpu(&[0xF0, 0x1E]);
    cpu.i_reg = 32;
    cpu.v_reg[0x0] = 56;

    cpu.tick();

    assert_eq!(cpu.i_reg, 88);
}

#[test]
fn test_fx29() {
    let mut cpu = set_up_cpu(&[0xF0, 0x29]);
    cpu.v_reg[0x0] = 1;

    cpu.tick();

    assert_eq!(cpu.i_reg, FONT_ADDRESS as u16 + 5);
}

#[test]
fn test_fx33() {
    let mut cpu = set_up_cpu(&[0xF0, 0x33]);
    cpu.v_reg[0x0] = 149;
    cpu.i_reg = 400;

    cpu.tick();

    assert_eq!(cpu.ram.read(400, 3), &[0x1, 0x4, 0x9]);
}

#[test]
fn test_fx55() {
    let mut cpu = set_up_cpu(&[0xF4, 0x55]);
    cpu.v_reg[0..=4].copy_from_slice(&[0x0, 0x1, 0x2, 0x3, 0x21]);
    cpu.i_reg = 400;

    cpu.tick();

    assert_eq!(cpu.ram.read(400, 5), &[0x0, 0x1, 0x2, 0x3, 0x21]);
}

#[test]
fn test_fx65() {
    let mut cpu = set_up_cpu(&[0xF3, 0x65]);
    cpu.ram.write(400, &[0x12, 0x34, 0x56, 0x78]);
    cpu.i_reg = 400;

    cpu.tick();

    assert_eq!(&cpu.v_reg[0..=3], &[0x12, 0x34, 0x56, 0x78]);
}
