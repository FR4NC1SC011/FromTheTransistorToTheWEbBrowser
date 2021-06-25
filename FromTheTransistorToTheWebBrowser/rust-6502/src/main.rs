mod and_eor_ora_tests;
mod branches_tests;
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

#[derive(Debug)]
pub struct Mem {
    pub MAX_MEM: u32,
    pub Data: Vec<Byte>,
}

pub enum Flags {
    NegativeFlagBit = 0b10000000,
    OverflowFlagBit = 0b01000000,
    BreakFlagBit = 0b000010000,
    UnusedFlagBit = 0b000100000,
    InterruptDisableFlagBit = 0b000000100,
    ZeroBit = 0b00000001,
}

#[derive(Debug, Clone, Copy)]
pub struct CPU {
    pub PC: Word, // program counter
    pub SP: Byte, // stack pointer
    // pub PS: Byte, // process status

    // Registers
    pub A: Byte, // Accumulator
    pub X: Byte,
    pub Y: Byte,

    // Status flags
    // pub PS: (Byte, Byte, Byte, Byte, Byte, Byte, Byte, Byte),
    pub PS: Byte,

    // pub C: Byte,
    // pub Z: Byte,
    // pub I: Byte,
    // pub D: Byte,
    // pub B: Byte,
    // pub Unused: Byte,
    // pub V: Byte,
    // pub N: Byte,

    // Opcodes

    // LDA
    pub INS_LDA_IM: Byte,
    pub INS_LDA_ZP: Byte,
    pub INS_LDA_ZPX: Byte,
    pub INS_LDA_ABS: Byte,
    pub INS_LDA_ABSX: Byte,
    pub INS_LDA_ABSY: Byte,
    pub INS_LDA_INDX: Byte,
    pub INS_LDA_INDY: Byte,

    // LDX
    pub INS_LDX_IM: Byte,
    pub INS_LDX_ZP: Byte,
    pub INS_LDX_ZPY: Byte,
    pub INS_LDX_ABS: Byte,
    pub INS_LDX_ABSY: Byte,

    // LDY
    pub INS_LDY_IM: Byte,
    pub INS_LDY_ZP: Byte,
    pub INS_LDY_ZPX: Byte,
    pub INS_LDY_ABS: Byte,
    pub INS_LDY_ABSX: Byte,

    // STA
    pub INS_STA_ZP: Byte,
    pub INS_STA_ZPX: Byte,
    pub INS_STA_ABS: Byte,
    pub INS_STA_ABSX: Byte,
    pub INS_STA_ABSY: Byte,
    pub INS_STA_INDX: Byte,
    pub INS_STA_INDY: Byte,

    // STX
    pub INS_STX_ZP: Byte,
    pub INS_STX_ZPY: Byte,
    pub INS_STX_ABS: Byte,

    // STY
    pub INS_STY_ZP: Byte,
    pub INS_STY_ZPX: Byte,
    pub INS_STY_ABS: Byte,

    // Jumps And Calls
    pub INS_JMP_ABS: Byte,
    pub INS_JMP_IND: Byte,
    pub INS_JSR: Byte,
    pub INS_RTS: Byte,

    // Stack Operations
    pub INS_TSX: Byte,
    pub INS_TXS: Byte,
    pub INS_PHA: Byte,
    pub INS_PHP: Byte,
    pub INS_PLA: Byte,
    pub INS_PLP: Byte,

    // Logical Operations
    pub INS_AND_IM: Byte,
    pub INS_AND_ZP: Byte,
    pub INS_AND_ZPX: Byte,
    pub INS_AND_ABS: Byte,
    pub INS_AND_ABSX: Byte,
    pub INS_AND_ABSY: Byte,
    pub INS_AND_INDX: Byte,
    pub INS_AND_INDY: Byte,

    pub INS_EOR_IM: Byte,
    pub INS_EOR_ZP: Byte,
    pub INS_EOR_ZPX: Byte,
    pub INS_EOR_ABS: Byte,
    pub INS_EOR_ABSX: Byte,
    pub INS_EOR_ABSY: Byte,
    pub INS_EOR_INDX: Byte,
    pub INS_EOR_INDY: Byte,

    pub INS_ORA_IM: Byte,
    pub INS_ORA_ZP: Byte,
    pub INS_ORA_ZPX: Byte,
    pub INS_ORA_ABS: Byte,
    pub INS_ORA_ABSX: Byte,
    pub INS_ORA_ABSY: Byte,
    pub INS_ORA_INDX: Byte,
    pub INS_ORA_INDY: Byte,

    pub INS_BIT_ZP: Byte,
    pub INS_BIT_ABS: Byte,

    // Register Transfers
    pub INS_TAX: Byte,
    pub INS_TAY: Byte,
    pub INS_TXA: Byte,
    pub INS_TYA: Byte,

    // Increments & Decrements
    pub INS_INC_ABS: Byte,
    pub INS_INC_ABSX: Byte,
    pub INS_INC_ZP: Byte,
    pub INS_INC_ZPX: Byte,

    pub INS_INX: Byte,
    pub INS_INY: Byte,

    pub INS_DEC_ABS: Byte,
    pub INS_DEC_ABSX: Byte,
    pub INS_DEC_ZP: Byte,
    pub INS_DEC_ZPX: Byte,

    pub INS_DEX: Byte,
    pub INS_DEY: Byte,

    // Branches
    pub INS_BEQ: Byte,
}

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
