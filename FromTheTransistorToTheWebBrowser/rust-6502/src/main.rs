mod add_with_carry_tests;
mod address;
mod and_eor_ora_tests;
mod branches_tests;
mod compare_register_tests;
mod increments_and_decrements_tests;
mod jumps_and_calls_tests;
mod load_tests;
mod mos6502;
mod programs_tests;
mod shifts_tests;
mod stack_operations_tests;
mod status_flags_tests;
mod store_tests;
mod system_functions_tests;
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

    // Registers
    pub A: Byte, // Accumulator
    pub X: Byte,
    pub Y: Byte,

    // Status flags
    pub PS: Byte,

    // Opcodes

    // Load / Store Operations

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
    pub INS_BCC: Byte,
    pub INS_BCS: Byte,
    pub INS_BEQ: Byte,
    pub INS_BMI: Byte,
    pub INS_BNE: Byte,
    pub INS_BPL: Byte,
    pub INS_BVC: Byte,
    pub INS_BVS: Byte,

    // Status Flags Changes
    pub INS_CLC: Byte,
    pub INS_CLD: Byte,
    pub INS_CLI: Byte,
    pub INS_CLV: Byte,
    pub INS_SEC: Byte,
    pub INS_SED: Byte,
    pub INS_SEI: Byte,

    // Arithmetic

    // Add With Carry
    pub INS_ADC_IM: Byte,
    pub INS_ADC_ZP: Byte,
    pub INS_ADC_ZPX: Byte,
    pub INS_ADC_ABS: Byte,
    pub INS_ADC_ABSX: Byte,
    pub INS_ADC_ABSY: Byte,
    pub INS_ADC_INDX: Byte,
    pub INS_ADC_INDY: Byte,

    // Sub With Carry
    pub INS_SBC_IM: Byte,
    pub INS_SBC_ZP: Byte,
    pub INS_SBC_ZPX: Byte,
    pub INS_SBC_ABS: Byte,
    pub INS_SBC_ABSX: Byte,
    pub INS_SBC_ABSY: Byte,
    pub INS_SBC_INDX: Byte,
    pub INS_SBC_INDY: Byte,

    // Compare Accumulator
    pub INS_CMP_IM: Byte,
    pub INS_CMP_ZP: Byte,
    pub INS_CMP_ZPX: Byte,
    pub INS_CMP_ABS: Byte,
    pub INS_CMP_ABSX: Byte,
    pub INS_CMP_ABSY: Byte,
    pub INS_CMP_INDX: Byte,
    pub INS_CMP_INDY: Byte,

    // Compare X Register
    pub INS_CPX_IM: Byte,
    pub INS_CPX_ZP: Byte,
    pub INS_CPX_ABS: Byte,

    // Compare Y Register
    pub INS_CPY_IM: Byte,
    pub INS_CPY_ZP: Byte,
    pub INS_CPY_ABS: Byte,

    // Shifts

    // Arithmetic Shift Left
    pub INS_ASL_ACC: Byte,
    pub INS_ASL_ZP: Byte,
    pub INS_ASL_ZPX: Byte,
    pub INS_ASL_ABS: Byte,
    pub INS_ASL_ABSX: Byte,

    // Logial Shift Right
    pub INS_LSR_ACC: Byte,
    pub INS_LSR_ZP: Byte,
    pub INS_LSR_ZPX: Byte,
    pub INS_LSR_ABS: Byte,
    pub INS_LSR_ABSX: Byte,

    // Rotate Left
    pub INS_ROL_ACC: Byte,
    pub INS_ROL_ZP: Byte,
    pub INS_ROL_ZPX: Byte,
    pub INS_ROL_ABS: Byte,
    pub INS_ROL_ABSX: Byte,

    // Rotate Right
    pub INS_ROR_ACC: Byte,
    pub INS_ROR_ZP: Byte,
    pub INS_ROR_ZPX: Byte,
    pub INS_ROR_ABS: Byte,
    pub INS_ROR_ABSX: Byte,

    // System Functions
    pub INS_BRK: Byte,
    pub INS_NOP: Byte,
    pub INS_RTI: Byte,
}

impl CPU {
    // load program into memory
    pub fn load_prg(&mut self, program: [Byte; 309], num_bytes: u32, memory: &mut Mem) -> Word {
        let mut load_address: Word = 0;

        if !program.is_empty() && num_bytes > 2 {
            let mut at: u32 = 0;

            let lo: Word = program[at as usize] as Word;

            at = at + 1;
            let hi_byte: Word = program[at as usize] as Word;

            let hi: Word = hi_byte.wrapping_shl(8) as Word;

            load_address = lo | hi;

            let mut i = load_address;
            loop {
                if u32::from(i) >= load_address as u32 + num_bytes - 2 {
                    break;
                }

                at = at + 1;
                memory.Data[i as usize] = program[at as usize];
                i += 1;
            }
        }

        load_address
    }
}

fn main() {
    println!("6502 Emulator with rust");

    let mut mem = Mem::new();
    let mut cpu = CPU::new();
    let mut cpu_copy = CPU::new();

    // given:
    cpu.reset(&mut mem);
    cpu_copy.reset(&mut mem);

    // The program below contains the entire source code of the game snake6502
    // Using 6502js Assembler ->  https://github.com/skilldrick/6502js
    /*
     ___           _        __ ___  __ ___
    / __|_ _  __ _| |_____ / /| __|/  \_  )
    \__ \ ' \/ _` | / / -_) _ \__ \ () / /
    |___/_||_\__,_|_\_\___\___/___/\__/___|
                                             */

    let prg: [Byte; 309] = [
        0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02,
        0x85, 0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9,
        0x0f, 0x85, 0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85,
        0x00, 0xa5, 0xfe, 0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20,
        0x8d, 0x06, 0x20, 0xc3, 0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c,
        0x38, 0x06, 0xa5, 0xff, 0xc9, 0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0,
        0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60, 0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85,
        0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0, 0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01,
        0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02, 0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05,
        0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06, 0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00,
        0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07, 0xe6, 0x03, 0xe6, 0x03, 0x20,
        0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06, 0xb5, 0x11, 0xc5, 0x11,
        0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c, 0x35, 0x07, 0x60,
        0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02, 0x4a, 0xb0,
        0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9, 0x20,
        0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
        0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10,
        0xb0, 0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5,
        0x10, 0x29, 0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe,
        0x91, 0x00, 0x60, 0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10,
        0x60, 0xa2, 0x00, 0xea, 0xea, 0xca, 0xd0, 0xfb, 0x60,
    ];

    let start_address = cpu.load_prg(prg, 309, &mut mem);
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
