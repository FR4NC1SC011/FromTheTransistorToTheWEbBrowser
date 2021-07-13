use bit_field::BitField;
use num_traits::{WrappingShl, WrappingShr};
use std::os::raw::*;

use crate::address::{Address, AddressDiff};
use crate::Flags;
use crate::Mem;
use crate::CPU;

type Byte = c_uchar;
type Word = c_ushort;

impl Mem {
    fn initialize(&mut self) {
        for _ in 0..self.MAX_MEM {
            self.Data.push(0);
        }
    }

    pub fn new() -> Self {
        Mem {
            MAX_MEM: 1024 * 64,
            Data: Vec::new(),
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            PC: 0, // Program Counter
            SP: 0, // Stack Pointer

            A: 0, // Accumulator
            X: 0, // Index Register X
            Y: 0, // Index Register Y

            PS: 0b11111111, // Processor Status
            //  0bNVUBDIZC
            // C: Carry Flag,          0
            // Z: Zero Flag,           1
            // I: Interrupt Disable,   2
            // D: Decimal Mode,        3
            // B: Break Command,       4
            // U: Unused,              5
            // V: Overflow Flag,       6
            // N: Negative Flag,       7

            // Opcodes

            // LDA
            INS_LDA_IM: 0xA9,
            INS_LDA_ZP: 0xA5,
            INS_LDA_ZPX: 0xB5,
            INS_LDA_ABS: 0xAD,
            INS_LDA_ABSX: 0xBD,
            INS_LDA_ABSY: 0xB9,
            INS_LDA_INDX: 0xA1,
            INS_LDA_INDY: 0xB1,

            // LDX
            INS_LDX_IM: 0xA2,
            INS_LDX_ZP: 0xA6,
            INS_LDX_ZPY: 0xB6,
            INS_LDX_ABS: 0xAE,
            INS_LDX_ABSY: 0xBE,

            // LDY
            INS_LDY_IM: 0xA0,
            INS_LDY_ZP: 0xA4,
            INS_LDY_ZPX: 0xB4,
            INS_LDY_ABS: 0xAC,
            INS_LDY_ABSX: 0xBC,

            // STA
            INS_STA_ZP: 0x85,
            INS_STA_ZPX: 0x95,
            INS_STA_ABS: 0x8D,
            INS_STA_ABSX: 0x9D,
            INS_STA_ABSY: 0x99,
            INS_STA_INDX: 0x81,
            INS_STA_INDY: 0x91,

            // STX
            INS_STX_ZP: 0x86,
            INS_STX_ZPY: 0x96,
            INS_STX_ABS: 0x8E,

            // STY
            INS_STY_ZP: 0x84,
            INS_STY_ZPX: 0x94,
            INS_STY_ABS: 0x8C,

            // Jumps And Calls
            INS_JMP_ABS: 0x4C,
            INS_JMP_IND: 0x6C,
            INS_JSR: 0x20,
            INS_RTS: 0x60,

            // Stack Operations
            INS_TSX: 0xBA,
            INS_TXS: 0x9A,
            INS_PHA: 0x48,
            INS_PHP: 0x08,
            INS_PLA: 0x68,
            INS_PLP: 0x28,

            // Logical Operations
            INS_AND_IM: 0x29,
            INS_AND_ZP: 0x25,
            INS_AND_ZPX: 0x35,
            INS_AND_ABS: 0x2D,
            INS_AND_ABSX: 0x3D,
            INS_AND_ABSY: 0x39,
            INS_AND_INDX: 0x21,
            INS_AND_INDY: 0x31,

            INS_EOR_IM: 0x49,
            INS_EOR_ZP: 0x45,
            INS_EOR_ZPX: 0x55,
            INS_EOR_ABS: 0x4D,
            INS_EOR_ABSX: 0x5D,
            INS_EOR_ABSY: 0x59,
            INS_EOR_INDX: 0x41,
            INS_EOR_INDY: 0x51,

            INS_ORA_IM: 0x09,
            INS_ORA_ZP: 0x05,
            INS_ORA_ZPX: 0x15,
            INS_ORA_ABS: 0x0D,
            INS_ORA_ABSX: 0x1D,
            INS_ORA_ABSY: 0x19,
            INS_ORA_INDX: 0x01,
            INS_ORA_INDY: 0x11,

            INS_BIT_ZP: 0x24,
            INS_BIT_ABS: 0x2C,

            // Register Transfers
            INS_TAX: 0xAA,
            INS_TAY: 0xA8,
            INS_TXA: 0x8A,
            INS_TYA: 0x98,

            // Increments & Decrements
            INS_INC_ZP: 0xE6,
            INS_INC_ZPX: 0xF6,
            INS_INC_ABS: 0xEE,
            INS_INC_ABSX: 0xFE,

            INS_INX: 0xE8,

            INS_INY: 0xC8,

            INS_DEC_ZP: 0xC6,
            INS_DEC_ZPX: 0xD6,
            INS_DEC_ABS: 0xCE,
            INS_DEC_ABSX: 0xDE,

            INS_DEX: 0xCA,

            INS_DEY: 0x88,

            // Branches
            INS_BCC: 0x90,
            INS_BCS: 0xB0,
            INS_BEQ: 0xF0,
            INS_BMI: 0x30,
            INS_BNE: 0xD0,
            INS_BPL: 0x10,
            INS_BVC: 0x50,
            INS_BVS: 0x70,

            // Status Flags Changes
            INS_CLC: 0x18,
            INS_CLD: 0xD8,
            INS_CLI: 0x58,
            INS_CLV: 0xB8,
            INS_SEC: 0x38,
            INS_SED: 0xF8,
            INS_SEI: 0x78,

            // Arithmetic

            // Add With Carry
            INS_ADC_IM: 0x69,
            INS_ADC_ZP: 0x65,
            INS_ADC_ZPX: 0x75,
            INS_ADC_ABS: 0x6D,
            INS_ADC_ABSX: 0x7D,
            INS_ADC_ABSY: 0x79,
            INS_ADC_INDX: 0x61,
            INS_ADC_INDY: 0x71,

            // Sub With Carry
            INS_SBC_IM: 0xE9,
            INS_SBC_ZP: 0xE5,
            INS_SBC_ZPX: 0xF5,
            INS_SBC_ABS: 0xED,
            INS_SBC_ABSX: 0xFD,
            INS_SBC_ABSY: 0xF9,
            INS_SBC_INDX: 0xE1,
            INS_SBC_INDY: 0xF1,

            // Compare Accumulator
            INS_CMP_IM: 0xC9,
            INS_CMP_ZP: 0xC5,
            INS_CMP_ZPX: 0xD5,
            INS_CMP_ABS: 0xCD,
            INS_CMP_ABSX: 0xDD,
            INS_CMP_ABSY: 0xD9,
            INS_CMP_INDX: 0xC1,
            INS_CMP_INDY: 0xD1,

            // Compare X Register
            INS_CPX_IM: 0xE0,
            INS_CPX_ZP: 0xE4,
            INS_CPX_ABS: 0xEC,

            //Compare Y Register
            INS_CPY_IM: 0xC0,
            INS_CPY_ZP: 0xC4,
            INS_CPY_ABS: 0xCC,

            // Shifts

            // Arithmetic Shift Left
            INS_ASL_ACC: 0x0A,
            INS_ASL_ZP: 0x06,
            INS_ASL_ZPX: 0x16,
            INS_ASL_ABS: 0x0E,
            INS_ASL_ABSX: 0x1E,

            //  Logical Shift Right
            INS_LSR_ACC: 0x4A,
            INS_LSR_ZP: 0x46,
            INS_LSR_ZPX: 0x56,
            INS_LSR_ABS: 0x4E,
            INS_LSR_ABSX: 0x5E,

            // Rotate Left
            INS_ROL_ACC: 0x2A,
            INS_ROL_ZP: 0x26,
            INS_ROL_ZPX: 0x36,
            INS_ROL_ABS: 0x2E,
            INS_ROL_ABSX: 0x3E,

            // Rotate Right
            INS_ROR_ACC: 0x6A,
            INS_ROR_ZP: 0x66,
            INS_ROR_ZPX: 0x76,
            INS_ROR_ABS: 0x6E,
            INS_ROR_ABSX: 0x7E,

            // System Functions
            INS_BRK: 0x00,
            INS_NOP: 0xEA,
            INS_RTI: 0x40,
        }
    }

    pub fn reset(&mut self, memory: &mut Mem) {
        self.PC = 0xFFFC;
        self.SP = 0xFF;

        self.A = 0;
        self.X = 0;
        self.Y = 0;

        self.PS = 0b11111111;

        memory.initialize();
    }

    pub fn reset_vector(&mut self, memory: &mut Mem, address: Word) {
        self.PC = address;
        self.SP = 0xFF;

        self.A = 0;
        self.X = 0;
        self.Y = 0;

        self.PS = 0b11111111;

        memory.initialize();
    }

    fn fetch_word(&mut self, cycles: &mut isize, memory: &mut Mem) -> Word {
        // 6502 is little endian
        let mut data: Word = memory.Data[self.PC as usize] as Word;
        self.PC += 1;

        data |= WrappingShl::wrapping_shl(&(memory.Data[self.PC as usize] as Word), 8);
        self.PC += 1;

        *cycles -= 2;
        data
    }

    fn fetch_byte(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        let data: Byte = memory.Data[self.PC as usize];
        self.PC = self.PC.wrapping_add(1);
        *cycles = cycles.wrapping_sub(1);
        data
    }

    fn read_byte(&mut self, cycles: &mut isize, address: Word, memory: &mut Mem) -> Byte {
        let data: Byte = memory.Data[address as usize];
        *cycles = cycles.wrapping_sub(1);
        data
    }

    fn read_word(&mut self, cycles: &mut isize, address: Word, memory: &mut Mem) -> Word {
        let lo_byte: Byte = self.read_byte(cycles, address, memory);
        let hi_byte: Byte = self.read_byte(cycles, address + 1, memory);

        let mut data: Word = lo_byte as Word;
        data |= WrappingShl::wrapping_shl(&(hi_byte as Word), 8);

        data
    }

    fn write_byte(&mut self, value: Byte, cycles: &mut isize, address: Word, memory: &mut Mem) {
        memory.Data[address as usize] = value;
        *cycles = cycles.wrapping_sub(1);
    }

    // write 2 bytes
    fn write_word(&mut self, value: Byte, cycles: &mut isize, address: Word, memory: &mut Mem) {
        memory.Data[address as usize] = (value & 0xFF) as Byte;
        let mut x: u16 = value as Word;
        x = x.wrapping_shr(8);
        memory.Data[(address + 1) as usize] = x as Byte;
        *cycles -= 2;
    }

    // return the stack pointer as a full 16-bit address
    pub fn sp_to_address(&mut self) -> Word {
        0x100 as Word | self.SP as Word
    }

    fn push_byte_to_stack(&mut self, cycles: &mut isize, memory: &mut Mem, value: Byte) {
        memory.Data[self.sp_to_address() as usize] = value;
        *cycles -= 1;
        self.SP = self.SP.wrapping_sub(1);
        *cycles -= 1;
    }

    fn pop_byte_from_stack(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        self.SP += 1;
        *cycles -= 1;
        let sp_word: Word = self.sp_to_address();
        let value: Byte = memory.Data[sp_word as usize];
        *cycles -= 1;

        value
    }

    fn push_word_to_stack(&mut self, cycles: &mut isize, memory: &mut Mem, value: Word) {
        let mut sp_16_bit = self.sp_to_address();
        self.write_byte(value.wrapping_shr(8) as Byte, cycles, sp_16_bit, memory);
        self.SP = self.SP.wrapping_sub(1);
        sp_16_bit = self.sp_to_address();
        self.write_byte((value & 0xFF) as Byte, cycles, sp_16_bit, memory);
        self.SP = self.SP.wrapping_sub(1);
    }

    // push the PC onto the stack
    fn push_pc_to_stack(&mut self, cycles: &mut isize, memory: &mut Mem) {
        self.push_word_to_stack(cycles, memory, self.PC as Word);
    }

    // push the PC + 1 onto the stack
    fn push_pc_to_stack_plus_one(&mut self, cycles: &mut isize, memory: &mut Mem) {
        self.push_word_to_stack(cycles, memory, (self.PC + 1) as Word);
    }

    // push the PC + 2 onto the stack
    fn push_pc_to_stack_plus_two(&mut self, cycles: &mut isize, memory: &mut Mem) {
        self.push_word_to_stack(cycles, memory, (self.PC + 2) as Word);
    }

    // Push Processor Status onto the stack
    // Setting bits 4 & 5 on the  stack
    fn push_ps_to_stack(&mut self, cycles: &mut isize, memory: &mut Mem) {
        let ps_stack: Byte = self.PS | Flags::BreakFlagBit as u8 | Flags::UnusedFlagBit as u8; 
        self.push_byte_to_stack(cycles, memory, ps_stack);
    }

    // Pop Processor Status from the stack
    // Clearing bits 4 & 5 (Break & Unused)
    fn pop_ps_from_stack(&mut self, cycles: &mut isize, memory: &mut Mem) {
        self.PS = self.pop_byte_from_stack(cycles, memory);
        self.PS.set_bit(4, false);    // B
        self.PS.set_bit(5, false);    // U
    }

    fn pop_word_from_stack(&mut self, cycles: &mut isize, memory: &mut Mem) -> Word {
        let sp_16_bit = self.sp_to_address();

        let value_from_stack: Word = self.read_word(cycles, sp_16_bit + 1, memory);
        self.SP = self.SP.wrapping_add(2);

        *cycles -= 1;
        value_from_stack
    }

    fn zero_page_address(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        let zero_page_address: Byte = self.fetch_byte(cycles, memory);
        let value: Byte = self.read_byte(cycles, zero_page_address as Word, memory);

        value
    }

    fn zero_page_address_x(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        let mut zero_page_address_x: Byte = self.fetch_byte(cycles, memory);
        zero_page_address_x = zero_page_address_x.wrapping_add(self.X);
        *cycles -= 1;
        let value: Byte = self.read_byte(cycles, zero_page_address_x as Word, memory);

        value
    }

    fn zero_page_address_y(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        let mut zero_page_address_y: Byte = self.fetch_byte(cycles, memory);
        zero_page_address_y = zero_page_address_y.wrapping_add(self.Y);
        *cycles -= 1;
        let value: Byte = self.read_byte(cycles, zero_page_address_y as Word, memory);

        value
    }

    fn absolute_address(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        let abs_addrress: Word = self.fetch_word(cycles, memory);
        let value = self.read_byte(cycles, abs_addrress as Word, memory);

        value
    }

    fn absolute_address_x(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        let abs_address: Word = self.fetch_word(cycles, memory);
        let abs_address_plus_x: Word = abs_address + self.X as Word;
        let value = self.read_byte(cycles, abs_address_plus_x, memory);
        if abs_address_plus_x - abs_address >= 0xFF {
            *cycles -= 1;
        }

        value
    }

    fn absolute_address_y(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        let abs_address: Word = self.fetch_word(cycles, memory);
        let abs_address_plus_y: Word = abs_address + self.Y as Word;
        let value = self.read_byte(cycles, abs_address_plus_y, memory);
        if abs_address_plus_y - abs_address >= 0xFF {
            *cycles -= 1;
        }

        value
    }

    fn indirect_address_x(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
        zero_page_address += self.X;
        *cycles -= 1;
        let effective_address: Word = self.read_word(cycles, zero_page_address as Word, memory);
        let value: Byte = self.read_byte(cycles, effective_address, memory);

        value
    }

    fn indirect_address_y(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        let zero_page_address: Byte = self.fetch_byte(cycles, memory);
        let effective_address: Word = self.read_word(cycles, zero_page_address as Word, memory);
        let effective_address_y: Word = effective_address + self.Y as Word;
        let value = self.read_byte(cycles, effective_address_y, memory);
        if effective_address_y - effective_address >= 0xFF {
            *cycles -= 1;
        }

        value
    }

    fn add_with_carry(&mut self, operand: Byte) {
        let are_sign_bits_the_same: bool = !((self.A ^ operand) & Flags::NegativeFlagBit as Byte) != 0;

        let c_flag_value: Byte = if self.PS.get_bit(0) == true { 1 } else { 0 }; 
        self.A = self.A.wrapping_add(c_flag_value);
        let sum = self.A.checked_add(operand);
        
        match sum {
            Some(x) => {
                self.A = x;
                self.PS.set_bit(0, false);  // set Carry Flag
                self.ld_register_set_status(self.A);
            }

            None => {                      // Overflow
                self.A = self.A.wrapping_add(operand);
                self.PS.set_bit(0, true);  // set Carry Flag
                self.ld_register_set_status(self.A);
            }
        }

        // FIXME: Overflow Flag
        let v: bool = are_sign_bits_the_same && ((self.A ^ operand) & Flags::NegativeFlagBit as Byte) != 0;
        self.PS.set_bit(6, v); // V flag
    }

    fn rotate_left(&mut self, cycles: &mut isize, mut operand: Byte) -> Byte {
        let new_bit_0:  Byte = if self.PS.get_bit(0) { Flags::ZeroBit as Byte } else { 0 };  
        let c: bool = (operand & Flags::NegativeFlagBit as Byte) > 0;
        self.PS.set_bit(0, c);
        operand = operand << 1;
        operand |= new_bit_0;
        *cycles -= 1;

        operand
    }

    fn rotate_right(&mut self, cycles: &mut isize, mut operand: Byte) -> Byte {
        let old_bit_0: bool = (operand & Flags::ZeroBit as Byte) > 0;
        operand = operand >> 1;
        if self.PS.get_bit(0) {            // C
            operand |= Flags::NegativeFlagBit as Byte;
        }
        *cycles -= 1;
        self.PS.set_bit(0, old_bit_0);     // C

        operand
    }

    fn branch_if(&mut self, cycles: &mut isize, memory: &mut Mem, value: bool, condition: bool) {
        // TODO: review this function
        let offset: Byte = self.fetch_byte(cycles, memory);
        let address = CPU::signed_8_bit_to_16(offset).wrapping_add(self.PC);
        if value == condition {
            let old_pc: Word = self.PC;
            self.PC = address;
            *cycles -= 1;

            let page_changed: bool = self.PC >> 8 != old_pc >> 8;

            if page_changed {
                *cycles -= 2;
            }
        }
    }

    pub fn execute(&mut self, cycles: &mut isize, memory: &mut Mem) -> isize {
        let cycles_requested = *cycles;
        while cycles > &mut 0 {
            let ins: Byte = self.fetch_byte(cycles, memory);

            match ins {
                // Load Instructions
                0xA9 => {
                    println!("Instruction LDA Inmediate");
                    self.A = self.fetch_byte(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0xA2 => {
                    println!("Instruction LDX Inmediate");
                    self.X = self.fetch_byte(cycles, memory);
                    self.ld_register_set_status(self.X);
                }

                0xA0 => {
                    println!("Instruction LDY Inmediate");
                    self.Y = self.fetch_byte(cycles, memory);
                    self.ld_register_set_status(self.Y);
                }

                0xA5 => {
                    println!("Instruction LDA ZP");
                    self.A = self.zero_page_address(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0xA6 => {
                    println!("Instruction LDX ZP");
                    self.X = self.zero_page_address(cycles, memory);
                    self.ld_register_set_status(self.X);
                }

                0xA4 => {
                    println!("Instruction LDY ZP");
                    self.Y = self.zero_page_address(cycles, memory);
                    self.ld_register_set_status(self.Y);
                }

                0xB4 => {
                    println!("Instruction LDY ZPX");
                    self.Y = self.zero_page_address_x(cycles, memory);
                    self.ld_register_set_status(self.Y);
                }

                0xB5 => {
                    println!("Instruction LDA ZPX");
                    self.A = self.zero_page_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0xB6 => {
                    println!("Instruction LDX ZPY");
                    self.X = self.zero_page_address_y(cycles, memory);
                    self.ld_register_set_status(self.X);
                }

                0xAD => {
                    println!("Instruction LDA Absolute");
                    self.A = self.absolute_address(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0xAE => {
                    println!("Instruction LDX Absolute");
                    self.X = self.absolute_address(cycles, memory);
                    self.ld_register_set_status(self.X);
                }

                0xAC => {
                    println!("Instruction LDY Absolute");
                    self.Y = self.absolute_address(cycles, memory);
                    self.ld_register_set_status(self.Y);
                }

                0xBC => {
                    println!("Instruction LDY Absolute X");
                    self.Y = self.absolute_address_x(cycles, memory);
                    self.ld_register_set_status(self.Y);
                }

                0xBD => {
                    println!("Instruction LDA Absolute X");
                    self.A = self.absolute_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0xBE => {
                    println!("Instruction LDX Absolute Y");
                    self.X = self.absolute_address_y(cycles, memory);
                    self.ld_register_set_status(self.X);
                }

                0xB9 => {
                    println!("Instruction LDA Absolute Y");
                    self.A = self.absolute_address_y(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0xA1 => {
                    println!("Instruction LDA Indirect X");
                    self.A = self.indirect_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0xB1 => {
                    println!("Instruction LDA Indirect Y");
                    self.A = self.indirect_address_y(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                // Store Instructions
                0x85 => {
                    println!("Instruction STA Zero Page");
                    let zp_address: Byte = self.fetch_byte(cycles, memory);
                    self.write_byte(self.A, cycles, zp_address as Word, memory);
                }

                0x95 => {
                    println!("Instruction STA Zero Page X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.write_byte(self.A, cycles, zero_page_address as Word, memory);
                }

                0x86 => {
                    println!("Instruction STX Zero Page");
                    let zp_address: Byte = self.fetch_byte(cycles, memory);
                    self.write_byte(self.X, cycles, zp_address as Word, memory);
                }

                0x96 => {
                    println!("Instruction STX Zero Page Y ");
                    let mut zp_address: Byte = self.fetch_byte(cycles, memory);
                    zp_address = zp_address.wrapping_add(self.Y);
                    *cycles -= 1;
                    self.write_byte(self.X, cycles, zp_address as Word, memory);
                }

                0x84 => {
                    println!("Instruction STY Zero Page");
                    let zp_address: Byte = self.fetch_byte(cycles, memory);
                    self.write_byte(self.Y, cycles, zp_address as Word, memory);
                }

                0x94 => {
                    println!("Instruction STY Zero Page X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.write_byte(self.Y, cycles, zero_page_address as Word, memory);
                }

                0x8D => {
                    println!("Instriction STA Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.write_byte(self.A, cycles, abs_addrress as Word, memory);
                }

                0x8E => {
                    println!("Instriction STX Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.write_byte(self.X, cycles, abs_addrress as Word, memory);
                }

                0x8C => {
                    println!("Instruction STY Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.write_byte(self.Y, cycles, abs_addrress as Word, memory);
                }

                0x9D => {
                    println!("Instruction STA Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as Word;
                    self.write_byte(self.A, cycles, abs_address_plus_x, memory);
                    *cycles -= 1;
                }

                0x99 => {
                    println!("Instruction STA Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.Y as Word;
                    self.write_byte(self.A, cycles, abs_address_plus_x, memory);
                    *cycles -= 1;
                }

                0x81 => {
                    println!("Instruction STA Indirect X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as Word, memory);
                    self.write_byte(self.A, cycles, effective_address, memory);
                }

                0x91 => {
                    println!("Instruction STA Indirect Y");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as Word, memory);
                    let effective_address_y: Word = effective_address + self.Y as Word;
                    self.write_byte(self.A, cycles, effective_address_y, memory);
                    *cycles -= 1;
                }

                // Jumps and Calls

                // NB:
                //      An original 6502 has does not correctly fetch the target address
                //      if the indirect vector falls on a page boundary
                //      (e.g. $xxFF where xx is any value from $00 to $FF).
                //      In this case fetches the LSB from $xxFF as expected but takes
                //      the MSB from $xx00. This is fixed in some later chips like
                //      the 65SC02 so for compatibility always ensure the indirect vector
                //      is not at the end of the page.
                0x4C => {
                    println!("Instruction JMP Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.PC = abs_addrress;
                }

                0x6C => {
                    println!("Instruction JMP Indirect");
                    let mut abs_addrress: Word = self.fetch_word(cycles, memory);
                    abs_addrress = self.read_word(cycles, abs_addrress, memory);
                    self.PC = abs_addrress;
                }

                0x20 => {
                    println!("Instruction JSR");
                    let sub_addr: Word = self.fetch_word(cycles, memory);
                    self.push_pc_to_stack(cycles, memory);
                    self.PC = sub_addr;
                    *cycles -= 1;
                }

                0x60 => {
                    println!("Instruction RTS");
                    let return_address = self.pop_word_from_stack(cycles, memory);

                    // TODO: self.PC = return_address + 1 work on the video. Why?
                    self.PC = return_address;
                    *cycles -= 2;
                }

                // Stack Operations
                0xBA => {
                    println!("Instruction TSX");
                    self.X = self.SP;
                    *cycles -= 1;
                    self.ld_register_set_status(self.X);
                }

                0x9A => {
                    println!("Instruction TXS");
                    self.SP = self.X;
                    *cycles -= 1
                }

                0x48 => {
                    println!("Instruction PHA");
                    self.push_byte_to_stack(cycles, memory, self.A);
                }

                0x08 => {
                    println!("Instruction PHP");
                    self.push_ps_to_stack(cycles, memory);
                }

                0x68 => {
                    println!("Instruction PLA");
                    self.A = self.pop_byte_from_stack(cycles, memory);
                    *cycles -= 1;
                    self.ld_register_set_status(self.A);
                }

                0x28 => {
                    println!("Instruction PLP");
                    self.pop_ps_from_stack(cycles, memory);
                    *cycles -= 1;
                }

                // Logical Operations
                0x29 => {
                    println!("instruction AND Inmediate");
                    self.A &= self.fetch_byte(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x09 => {
                    println!("instruction ORA Inmediate");
                    self.A |= self.fetch_byte(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x49 => {
                    println!("instructin EOR Inmediate");
                    self.A ^= self.fetch_byte(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x25 => {
                    println!("Instruction AND ZP");
                    self.A &= self.zero_page_address(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x05 => {
                    println!("Instruction ORA ZP");
                    self.A |= self.zero_page_address(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x45 => {
                    println!("Instruction EOR ZP");
                    self.A ^= self.zero_page_address(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x35 => {
                    println!("Instruction AND ZPX");
                    self.A &= self.zero_page_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x15 => {
                    println!("Instruction ORA ZPX");
                    self.A |= self.zero_page_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x55 => {
                    println!("Instruction EOR ZPX");
                    self.A ^= self.zero_page_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x2D => {
                    println!("Instruction AND Absolute");
                    self.A &= self.absolute_address(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x0D => {
                    println!("Instruction ORA Absolute");
                    self.A |= self.absolute_address(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x4D => {
                    println!("Instruction EOR Absolute");
                    self.A ^= self.absolute_address(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x3D => {
                    println!("Instruction AND Absolute X");
                    self.A &= self.absolute_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x1D => {
                    println!("Instruction ORA Absolute X");
                    self.A |= self.absolute_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x5D => {
                    println!("Instruction EOR Absolute X");
                    self.A ^= self.absolute_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x39 => {
                    println!("Instruction AND Absolute Y");
                    self.A &= self.absolute_address_y(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x19 => {
                    println!("Instruction ORA Absolute Y");
                    self.A |= self.absolute_address_y(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x59 => {
                    println!("Instruction EOR Absolute Y");
                    self.A ^= self.absolute_address_y(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x21 => {
                    println!("Instruction AND Indirect X");
                    self.A &= self.indirect_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x01 => {
                    println!("Instruction ORA Indirect X");
                    self.A |= self.indirect_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x41 => {
                    println!("Instruction EOR Indirect X");
                    self.A ^= self.indirect_address_x(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x31 => {
                    println!("Instruction AND Indirect Y");
                    self.A &= self.indirect_address_y(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x11 => {
                    println!("Instruction ORA Indirect Y");
                    self.A |= self.indirect_address_y(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x51 => {
                    println!("Instruction EOR Indirect Y");
                    self.A ^= self.indirect_address_y(cycles, memory);
                    self.ld_register_set_status(self.A);
                }

                0x24 => {
                    println!("Instruction BIT ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let value = self.read_byte(cycles, zero_page_address as Word, memory);

                    let z = self.A & value;
                    let z_bool: bool;
                    if z == 0 {
                        z_bool = true;
                    } else {
                        z_bool = false;
                    }

                    self.PS.set_bit(1, z_bool);

                    let n = (value & Flags::NegativeFlagBit as Byte) != 0;
                    self.PS.set_bit(7, n);

                    let v = (value & Flags::OverflowFlagBit as Byte) != 0;
                    self.PS.set_bit(6, v);
                }

                0x2C => {
                    println!("Instruction BIT ABS");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let value = self.read_byte(cycles, abs_addrress as Word, memory);

                    let z = self.A & value;
                    let z_bool: bool;
                    if z == 0 {
                        z_bool = true;
                    } else {
                        z_bool = false;
                    }

                    self.PS.set_bit(1, z_bool);

                    let n = (value & Flags::NegativeFlagBit as Byte) != 0;
                    self.PS.set_bit(7, n);

                    let v = (value & Flags::OverflowFlagBit as Byte) != 0;
                    self.PS.set_bit(6, v);
                }

                0xAA => {
                    println!("Instruction TAX");
                    self.X = self.A;
                    *cycles -= 1;
                    self.ld_register_set_status(self.X);
                }

                0xA8 => {
                    println!("Instruction TAY");
                    self.Y = self.A;
                    *cycles -= 1;
                    self.ld_register_set_status(self.Y);
                }

                0x8A => {
                    println!("Instruction TXA");
                    self.A = self.X;
                    *cycles -= 1;
                    self.ld_register_set_status(self.A);
                }

                0x98 => {
                    println!("Instruction TYA");
                    self.A = self.Y;
                    *cycles -= 1;
                    self.ld_register_set_status(self.A);
                }

                0xE6 => {
                    println!("Instruction Increment Memory ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let mut value = self.read_byte(cycles, zero_page_address as Word, memory);
                    value += 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(value);
                }

                0xF6 => {
                    println!("Instruction Increment Memory ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    let mut value = self.read_byte(cycles, zero_page_address as Word, memory);
                    value += 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(value);
                }

                0xEE => {
                    println!("Instruction Increment Memory Absolute");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let mut value = self.read_byte(cycles, abs_address as Word, memory);
                    value += 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address.into(), memory);
                    self.ld_register_set_status(value);
                }

                0xFE => {
                    println!("Instruction Increment Memory Absolute X");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x = abs_address + self.X as Word;
                    let mut value = self.read_byte(cycles, abs_address_plus_x, memory);
                    value += 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address_plus_x.into(), memory);
                    self.ld_register_set_status(value);
                }

                0xC6 => {
                    println!("Instruction Decrement Memory ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let mut value = self.read_byte(cycles, zero_page_address as Word, memory);
                    value -= 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(value);
                }

                0xD6 => {
                    println!("Instruction Decrement Memory ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    let mut value = self.read_byte(cycles, zero_page_address as Word, memory);
                    value -= 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(value);
                }

                0xCE => {
                    println!("Instruction Decrement Memory Absolute");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let mut value = self.read_byte(cycles, abs_address as Word, memory);
                    value -= 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address.into(), memory);
                    self.ld_register_set_status(value);
                }

                0xDE => {
                    println!("Instruction Decrement Memory Absolute X");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x = abs_address + self.X as Word;
                    let mut value = self.read_byte(cycles, abs_address_plus_x, memory);
                    value -= 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address_plus_x.into(), memory);
                    self.ld_register_set_status(value);
                }

                0xE8 => {
                    println!("Instruction Inc X");
                    self.X = self.X.wrapping_add(1);
                    self.ld_register_set_status(self.X);
                    *cycles -= 1;
                }

                0xC8 => {
                    println!("Instruction Inc Y");
                    self.Y = self.Y.wrapping_add(1);
                    self.ld_register_set_status(self.Y);
                    *cycles -= 1;
                }

                0xCA => {
                    println!("Instruction Dec X");
                    self.X = self.X.wrapping_sub(1);
                    self.ld_register_set_status(self.X);
                    *cycles -= 1;
                }

                0x88 => {
                    println!("Instruction Dec Y");
                    self.Y = self.Y.wrapping_sub(1);
                    self.ld_register_set_status(self.Y);
                    *cycles -= 1;
                }

                // Branches
                0x90 => {
                    println!("Instruction BCC");
                    self.branch_if(cycles, memory, self.PS.get_bit(0), false);
                }

                0xB0 => {
                    println!("Instruction BCS");
                    self.branch_if(cycles, memory, self.PS.get_bit(0), true);
                }

                0xF0 => {
                    // TODO: review this function
                    println!("Instruction BEQ");
                    self.branch_if(cycles, memory, self.PS.get_bit(1), true);
                }

                0xD0 => {
                    println!("Instruction BNE");
                    self.branch_if(cycles, memory, self.PS.get_bit(1), false);
                }

                0x30 => {
                    println!("Instruction BMI");
                    self.branch_if(cycles, memory, self.PS.get_bit(7), true);
                }

                0x10 => {
                    println!("Instruction BPL");
                    self.branch_if(cycles, memory, self.PS.get_bit(7), false);
                }

                0x50 => {
                    println!("Instruction BVC");
                    self.branch_if(cycles, memory, self.PS.get_bit(6), false);
                }

                0x70 => {
                    println!("Instruction BVS");
                    self.branch_if(cycles, memory, self.PS.get_bit(6), true);
                }

                // Status Flags Changes
                0x18 => {
                    println!("Instruction CLC");
                    self.PS.set_bit(0, false);
                    *cycles -= 1;
                }

                0xD8 => {
                    println!("Instruction CLD");
                    self.PS.set_bit(3, false);
                    *cycles -= 1;
                }

                0x58 => {
                    println!("Instruction CLI");
                    self.PS.set_bit(2, false);
                    *cycles -= 1;
                }

                0xB8 => {
                    println!("Instruction CLV");
                    self.PS.set_bit(6, false);
                    *cycles -= 1;
                }

                0x38 => {
                    println!("Instruction SEC");
                    self.PS.set_bit(0, true);
                    *cycles -= 1;
                }

                0xF8 => {
                    println!("Instruction SED");
                    self.PS.set_bit(3, true);
                    *cycles -= 1;
                }

                0x78 => {
                    println!("Instruction SEI");
                    self.PS.set_bit(2, true);
                    *cycles -= 1;
                }

                // Arithmetic

                // ADC
                0x6D => {
                    println!("Instruction ADC Absolute");
                    let operand: Byte = self.absolute_address(cycles, memory);
                    self.add_with_carry(operand);
               }

                0x7D => {
                    println!("Instruction ADC Absolute X");
                    let operand: Byte = self.absolute_address_x(cycles, memory);
                    self.add_with_carry(operand);
               }

                0x79 => {
                    println!("Instruction ADC Absolute Y");
                    let operand: Byte = self.absolute_address_y(cycles, memory);
                    self.add_with_carry(operand);
               }

               0x69 => {
                    println!("Instruction ADC Inmediate");
                    let operand: Byte = self.fetch_byte(cycles, memory);
                    self.add_with_carry(operand);
                }

               0x65 => {
                    println!("Instruction ADC Zero Page");
                    let operand: Byte = self.zero_page_address(cycles, memory);
                    self.add_with_carry(operand);
                }

                0x75 => {
                    println!("Instruction ADC Zero Page X");
                    let operand: Byte = self.zero_page_address_x(cycles, memory);
                    self.add_with_carry(operand);
                }

               0x61 => {
                    println!("Instruction ADC Indirect X");
                    let operand: Byte = self.indirect_address_x(cycles, memory);
                    self.add_with_carry(operand);
                }

                0x71 => {
                    println!("Instruction ADC Indirect Y");
                    let operand: Byte = self.indirect_address_y(cycles, memory);
                    self.add_with_carry(operand);
                }

                // SBC
                0xED => {
                    println!("Instruction SBC Absolute");
                    let operand: Byte = self.absolute_address(cycles, memory);
                    self.add_with_carry(!operand);
               }

                0xFD => {
                    println!("Instruction SBC Absolute X");
                    let operand: Byte = self.absolute_address_x(cycles, memory);
                    self.add_with_carry(!operand);
               }

                0xF9 => {
                    println!("Instruction SBC Absolute Y");
                    let operand: Byte = self.absolute_address_y(cycles, memory);
                    self.add_with_carry(!operand);
               }

               0xE9 => {
                    println!("Instruction SBC Inmediate");
                    let operand: Byte = self.fetch_byte(cycles, memory);
                    self.add_with_carry(!operand);
                }

               0xE5 => {
                    println!("Instruction SBC Zero Page");
                    let operand: Byte = self.zero_page_address(cycles, memory);
                    self.add_with_carry(!operand);
                }

                0xF5 => {
                    println!("Instruction SBC Zero Page X");
                    let operand: Byte = self.zero_page_address_x(cycles, memory);
                    self.add_with_carry(!operand);
                }

               0xE1 => {
                    println!("Instruction SBC Indirect X");
                    let operand: Byte = self.indirect_address_x(cycles, memory);
                    self.add_with_carry(!operand);
                }

                0xF1 => {
                    println!("Instruction SBC Indirect Y");
                    let operand: Byte = self.indirect_address_y(cycles, memory);
                    self.add_with_carry(!operand);
                }

                // CMP

                0xC9 => {
                    println!("Instruction CMP Inmediate");
                    let operand: Byte = self.fetch_byte(cycles, memory);
                    let temp: Byte = self.A.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.A);
                }

                0xC5 => {
                    println!("Instruction CMP ZP");
                    let operand: Byte = self.zero_page_address(cycles, memory);
                    let temp: Byte = self.A.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.A);
                }

                0xD5 => {
                    println!("Instruction CMP ZPX");
                    let operand: Byte = self.zero_page_address_x(cycles, memory);
                    let temp: Byte = self.A.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.A);
                }

                0xCD => {
                    println!("Instruction CMP ABS");
                    let operand: Byte = self.absolute_address(cycles, memory);
                    let temp: Byte = self.A.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.A);
                }

                0xDD => {
                    println!("Instruction CMP ABS X");
                    let operand: Byte = self.absolute_address_x(cycles, memory);
                    let temp: Byte = self.A.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.A);
                }

                0xD9 => {
                    println!("Instruction CMP ABS Y");
                    let operand: Byte = self.absolute_address_y(cycles, memory);
                    let temp: Byte = self.A.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.A);
                }

                0xC1 => {
                    println!("Instruction CMP IND X");
                    let operand: Byte = self.indirect_address_x(cycles, memory);
                    let temp: Byte = self.A.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.A);
                }

                0xD1 => {
                    println!("Instruction CMP IND Y");
                    let operand: Byte = self.indirect_address_y(cycles, memory);
                    let temp: Byte = self.A.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.A);
                }

                // CPX 

                0xE0 => {
                    println!("Instruction CPX Inmediate");
                    let operand: Byte = self.fetch_byte(cycles, memory);
                    let temp: Byte = self.X.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.X);
                }

                0xE4 => {
                    println!("Instruction CPX ZP");
                    let operand: Byte = self.zero_page_address(cycles, memory);
                    let temp: Byte = self.X.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.X);
                }

                0xEC => {
                    println!("Instruction CPX ABS");
                    let operand: Byte = self.absolute_address(cycles, memory);
                    let temp: Byte = self.X.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.X);
                }

                // CPY 

                0xC0 => {
                    println!("Instruction CPY Inmediate");
                    let operand: Byte = self.fetch_byte(cycles, memory);
                    let temp: Byte = self.Y.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.Y);
                }

                0xC4 => {
                    println!("Instruction CPY ZP");
                    let operand: Byte = self.zero_page_address(cycles, memory);
                    let temp: Byte = self.Y.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.Y);
                }

                0xCC => {
                    println!("Instruction CPY ABS");
                    let operand: Byte = self.absolute_address(cycles, memory);
                    let temp: Byte = self.Y.wrapping_sub(operand);
                    self.cmp_register_set_status(operand, temp, self.Y);
                }

                // ASL

                0x0A => {
                    println!("Instruction ASL ACC");
                    self.PS.set_bit(0, self.A.get_bit(7));
                    self.A = self.A << 1;
                    self.ld_register_set_status(self.A);
                    *cycles -= 1;
                }

              0x06 => {
                    println!("Instruction ASL ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let mut value = self.read_byte(cycles, zero_page_address as Word, memory);
                    self.PS.set_bit(0, value.get_bit(7));
                    value =  value << 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(value);
               }

              0x16 => {
                    println!("Instruction ASL ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    let mut value = self.read_byte(cycles, zero_page_address as Word, memory);
                    self.PS.set_bit(0, value.get_bit(7));
                    value = value << 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(value);
               }

              0x0E => {
                    println!("Instruction ASL ABS");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let mut value = self.read_byte(cycles, abs_address as Word, memory);
                    self.PS.set_bit(0, value.get_bit(7));
                    value =  value << 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address.into(), memory);
                    self.ld_register_set_status(value);
               }

              0x1E => {
                    println!("Instruction ASL ABSX");
                    let mut abs_address_x: Word = self.fetch_word(cycles, memory);
                    abs_address_x = abs_address_x.wrapping_add(self.X as Word);
                    *cycles -= 1;
                    let mut value = self.read_byte(cycles, abs_address_x as Word, memory);
                    self.PS.set_bit(0, value.get_bit(7));
                    value =  value << 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address_x.into(), memory);
                    self.ld_register_set_status(value);
               }

                // LSR

                0x4A => {
                    println!("Instruction LSR ACC");
                    self.PS.set_bit(0, self.A.get_bit(0));
                    self.A = self.A >> 1;
                    self.ld_register_set_status(self.A);
                    *cycles -= 1;
                }

              0x46 => {
                    println!("Instruction LSR ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let mut value = self.read_byte(cycles, zero_page_address as Word, memory);
                    self.PS.set_bit(0, value.get_bit(0));
                    value =  value >> 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(value);
               }

              0x56 => {
                    println!("Instruction LSR ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    let mut value = self.read_byte(cycles, zero_page_address as Word, memory);
                    self.PS.set_bit(0, value.get_bit(0));
                    value = value >> 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(value);
               }

              0x4E => {
                    println!("Instruction LSR ABS");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let mut value = self.read_byte(cycles, abs_address as Word, memory);
                    self.PS.set_bit(0, value.get_bit(0));
                    value =  value >> 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address.into(), memory);
                    self.ld_register_set_status(value);
               }

              0x5E => {
                    println!("Instruction LSR ABSX");
                    let mut abs_address_x: Word = self.fetch_word(cycles, memory);
                    abs_address_x = abs_address_x.wrapping_add(self.X as Word);
                    *cycles -= 1;
                    let mut value = self.read_byte(cycles, abs_address_x as Word, memory);
                    self.PS.set_bit(0, value.get_bit(0));
                    value =  value >> 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address_x.into(), memory);
                    self.ld_register_set_status(value);
               }

               // ROL

                0x2A => {
                    println!("Instruction ROL ACC");
                    self.A = self.rotate_left(cycles, self.A);
                    self.ld_register_set_status(self.A);
                }

                0x26 => {
                    println!("Instruction ROL ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let value = self.read_byte(cycles, zero_page_address as Word, memory);
                    let result: Byte = self.rotate_left(cycles, value);
                    self.write_byte(result, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(result);
                }

                0x36 => {
                    println!("Instruction ROL ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    let value = self.read_byte(cycles, zero_page_address as Word, memory);
                    let result: Byte = self.rotate_left(cycles, value);
                    *cycles -= 1;
                    self.write_byte(result, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(result);
                }

                0x2E => {
                    println!("Instruction ROL ABS");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let value = self.read_byte(cycles, abs_address as Word, memory);
                    let result: Byte = self.rotate_left(cycles, value);
                    *cycles -= 1;
                    self.write_byte(result, cycles, abs_address.into(), memory);
                    self.ld_register_set_status(result);
                }

                0x3E => {
                    println!("Instruction ROL ABSX");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x = abs_address.wrapping_add(self.X as Word);
                    let value = self.read_byte(cycles, abs_address_plus_x, memory);
                    let result: Byte = self.rotate_left(cycles, value);
                    *cycles -= 1;
                    self.write_byte(result, cycles, abs_address_plus_x.into(), memory);
                    self.ld_register_set_status(result);
                }

               // ROR

                0x6A => {
                    println!("Instruction ROR ACC");
                    self.A = self.rotate_right(cycles, self.A);
                    self.ld_register_set_status(self.A);
                }

                0x66 => {
                    println!("Instruction ROR ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let value = self.read_byte(cycles, zero_page_address as Word, memory);
                    let result: Byte = self.rotate_right(cycles, value);
                    self.write_byte(result, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(result);
                }

                0x76 => {
                    println!("Instruction ROR ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    let value = self.read_byte(cycles, zero_page_address as Word, memory);
                    let result: Byte = self.rotate_right(cycles, value);
                    *cycles -= 1;
                    self.write_byte(result, cycles, zero_page_address.into(), memory);
                    self.ld_register_set_status(result);
                }

                0x6E => {
                    println!("Instruction ROR ABS");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let value = self.read_byte(cycles, abs_address as Word, memory);
                    let result: Byte = self.rotate_right(cycles, value);
                    *cycles -= 1;
                    self.write_byte(result, cycles, abs_address.into(), memory);
                    self.ld_register_set_status(result);
                }

                0x7E => {
                    println!("Instruction ROR ABSX");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x = abs_address.wrapping_add(self.X as Word);
                    let value = self.read_byte(cycles, abs_address_plus_x, memory);
                    let result: Byte = self.rotate_right(cycles, value);
                    *cycles -= 1;
                    self.write_byte(result, cycles, abs_address_plus_x.into(), memory);
                    self.ld_register_set_status(result);
                }

                // System Functions
                0x00 => {
                    println!("Instruction BRK");
                    self.push_pc_to_stack_plus_one(cycles, memory);
                    // self.push_pc_to_stack_plus_two(cycles, memory);
                    self.push_ps_to_stack(cycles, memory);
                    let interrupt_vector: Word = 0xFFFE;
                    self.PC = self.read_word(cycles, interrupt_vector, memory);
                    self.PS.set_bit(4, true);    // B
                    self.PS.set_bit(2, true);    // I
                }

                0xEA => {
                    println!("Instruction NOP");
                    *cycles -= 1;
                }
                
                0x40 => {
                    println!("Instruction RTI");
                    self.pop_ps_from_stack(cycles, memory);
                    self.PC = self.pop_word_from_stack(cycles, memory);
                }

                _ => {
                    unimplemented!("Instruction not handled {}", ins);
                }
            }
        }

        cycles_requested - *cycles
    }

    fn ld_register_set_status(&mut self, x: Byte) {
        self.PS = match x == 0 {
            false => *self.PS.set_bit(1, false),
            true => *self.PS.set_bit(1, true),
        };

        self.PS = match (x & 0b10000000) > 0 {
            false => *self.PS.set_bit(7, false),
            true => *self.PS.set_bit(7, true),
        };
    }

    fn cmp_register_set_status(&mut self, operand: Byte, temp: Byte, reg: Byte) {
        self.PS.set_bit(0, reg >= operand);                          // C
        self.PS.set_bit(1, reg == operand);                          // Z
        self.PS.set_bit(7, temp & Flags::NegativeFlagBit as Byte != 0);   // N
    }

    // function to convert a byte to a word when the value is signed
    fn signed_8_bit_to_16(value: Byte) -> Word {
        let mut value = u16::from(value);
        if value & 0x80 > 0 {
            value |= 0xff00;
        }
        return value;
    }
}
