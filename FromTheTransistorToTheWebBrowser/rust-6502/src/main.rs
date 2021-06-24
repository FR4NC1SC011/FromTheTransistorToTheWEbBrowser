mod and_eor_ora_tests;
mod increments_and_decrements_tests;
mod jumps_and_calls_tests;
mod load_tests;
mod mos6502;
mod programs_tests;
mod stack_operations_tests;
mod store_tests;
mod transfer_register_tests;

use mos6502::*;

use crate::mos6502::*;
use bit_field::BitField;
use std::os::raw::*;

type Byte = c_uchar;
type Word = c_ushort;

fn main() {
    println!("6502 Emulator with rust");

    let mut mem = Mem::new();
    let mut cpu = CPU::new();
    let mut cpu_copy = CPU::new();

    // given:
    cpu.reset(&mut mem);
    cpu_copy.reset(&mut mem);

    let prg: [Byte; 14] = [
        0x00, 0x10, 0xA9, 0xFF, 0x85, 0x90, 0x8D, 0x00, 0x80, 0x49, 0xCC, 0x4C, 0x02, 0x10,
    ];

    // when
    let start_address = cpu.load_prg(prg, 14, &mut mem);
    cpu.PC = start_address;

    let mut clock: i32 = 1000;
    loop {
        if clock <= 0 {
            break;
        }

        clock -= cpu.execute(&mut 1, &mut mem) as i32;
        println!("A: {}, X: {}, Y: {}", cpu.A, cpu.X, cpu.Y);
        println!("PC: {}, SP: {}", cpu.PC, cpu.SP);
        println!("PS: {}", cpu.PS);
    }
}
