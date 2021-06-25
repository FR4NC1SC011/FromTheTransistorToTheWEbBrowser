use bit_field::BitField;
use num_traits::{WrappingShl, WrappingShr};
use std::os::raw::*;

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
            //  0bCZIDBUVN
            // C: Carry Flag,
            // Z: Zero Flag,
            // I: Interrupt Disable,
            // D: Decimal Mode,
            // B: Break Command,
            // U: Unused,
            // V: Overflow Flag,
            // N: Negative Flag,

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
            INS_BEQ: 0xF0,
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
        memory.Data[address as usize] = (value & 0xFF) as u8;
        let mut x: u16 = value as u16;
        x = x.wrapping_shr(8);
        memory.Data[(address + 1) as usize] = x as u8;
        *cycles -= 2;
    }

    // return the stack pointer as a full 16-bit address
    pub fn sp_to_address(&mut self) -> Word {
        0x100 as u16 | self.SP as u16
    }

    fn push_byte_to_stack(&mut self, cycles: &mut isize, memory: &mut Mem, value: Byte) {
        memory.Data[self.sp_to_address() as usize] = value;
        *cycles -= 1;
        self.SP -= 1;
        *cycles -= 1;
    }

    fn pop_byte_from_stack(&mut self, cycles: &mut isize, memory: &mut Mem) -> Byte {
        self.SP += 1;
        let sp_word: Word = self.sp_to_address();
        let value: Byte = memory.Data[sp_word as usize];
        *cycles -= 3;

        value
    }

    fn push_word_to_stack(&mut self, cycles: &mut isize, memory: &mut Mem, value: Word) {
        let mut sp_16_bit = self.sp_to_address();
        self.write_byte(value.wrapping_shr(8) as u8, cycles, sp_16_bit, memory);
        self.SP -= 1;
        sp_16_bit = self.sp_to_address();
        self.write_byte((value & 0xFF) as u8, cycles, sp_16_bit, memory);
        self.SP -= 1;
    }

    // push the PC - 1 onto the stack
    fn push_pc_to_stack(&mut self, cycles: &mut isize, memory: &mut Mem) {
        self.push_word_to_stack(cycles, memory, self.PC);
    }

    fn pop_word_from_stack(&mut self, cycles: &mut isize, memory: &mut Mem) -> Word {
        let sp_16_bit = self.sp_to_address();

        let value_from_stack: Word = self.read_word(cycles, sp_16_bit + 1, memory);
        self.SP = self.SP.wrapping_add(2);

        *cycles -= 1;
        value_from_stack
    }

    // load program into memory
    pub fn load_prg(&mut self, program: [Byte; 14], num_bytes: u32, memory: &mut Mem) -> Word {
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

    pub fn execute(&mut self, cycles: &mut isize, memory: &mut Mem) -> isize {
        let cycles_requested = *cycles;
        while cycles > &mut 0 {
            let ins: Byte = self.fetch_byte(cycles, memory);

            match ins {
                // Load Instructions
                0xA9 => {
                    println!("Instruction LDA Inmediate");
                    self.A = self.fetch_byte(cycles, memory);
                    self.lda_register_set_status();
                }

                0xA2 => {
                    println!("Instruction LDX Inmediate");
                    self.X = self.fetch_byte(cycles, memory);
                    self.ldx_register_set_status();
                }

                0xA0 => {
                    println!("Instruction LDY Inmediate");
                    self.Y = self.fetch_byte(cycles, memory);
                    self.ldy_register_set_status();
                }

                0xA5 => {
                    println!("Instruction LDA ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.A = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_register_set_status();
                }

                0xA6 => {
                    println!("Instruction LDX ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.X = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.ldx_register_set_status();
                }

                0xA4 => {
                    println!("Instruction LDY ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.Y = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.ldy_register_set_status();
                }

                0xB4 => {
                    println!("Instruction LDY ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.Y = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.ldy_register_set_status();
                }

                0xB5 => {
                    println!("Instruction LDA ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.A = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_register_set_status();
                }

                0xB6 => {
                    println!("Instruction LDX ZPY");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.Y);
                    *cycles -= 1;
                    self.X = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.ldx_register_set_status();
                }

                0xAD => {
                    println!("Instruction LDA Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.A = self.read_byte(cycles, abs_addrress as u16, memory);
                    self.lda_register_set_status();
                }

                0xAE => {
                    println!("Instruction LDX Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.X = self.read_byte(cycles, abs_addrress as u16, memory);
                    self.ldx_register_set_status();
                }

                0xAC => {
                    println!("Instruction LDY Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.Y = self.read_byte(cycles, abs_addrress as u16, memory);
                    self.ldy_register_set_status();
                }

                0xBC => {
                    println!("Instruction LDY Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as u16;
                    self.Y = self.read_byte(cycles, abs_address_plus_x, memory);
                    if abs_address_plus_x - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.ldy_register_set_status();
                }

                0xBD => {
                    println!("Instruction LDA Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as u16;
                    self.A = self.read_byte(cycles, abs_address_plus_x, memory);
                    if abs_address_plus_x - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0xBE => {
                    println!("Instruction LDX Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_y: Word = abs_addrress + self.Y as u16;
                    self.X = self.read_byte(cycles, abs_address_plus_y, memory);
                    if abs_address_plus_y - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.ldx_register_set_status();
                }

                0xB9 => {
                    println!("Instruction LDA Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_y: Word = abs_addrress + self.Y as u16;
                    self.A = self.read_byte(cycles, abs_address_plus_y, memory);
                    if abs_address_plus_y - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0xA1 => {
                    println!("Instruction LDA Indirect X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as u16, memory);
                    self.A = self.read_byte(cycles, effective_address, memory);
                    self.lda_register_set_status();
                }

                0xB1 => {
                    println!("Instruction LDA Indirect Y");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as u16, memory);
                    let effective_address_y: Word = effective_address + self.Y as u16;
                    self.A = self.read_byte(cycles, effective_address_y, memory);
                    if effective_address_y - effective_address >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                // Store Instructions
                0x85 => {
                    println!("Instruction STA Zero Page");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.write_byte(self.A, cycles, zero_page_address as u16, memory);
                }

                0x95 => {
                    println!("Instruction STA Zero Page X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.write_byte(self.A, cycles, zero_page_address as u16, memory);
                }

                0x86 => {
                    println!("Instruction STX Zero Page");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.write_byte(self.X, cycles, zero_page_address as u16, memory);
                }

                0x84 => {
                    println!("Instruction STY Zero Page");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.write_byte(self.Y, cycles, zero_page_address as u16, memory);
                }

                0x94 => {
                    println!("Instruction STY Zero Page X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.write_byte(self.Y, cycles, zero_page_address as u16, memory);
                }

                0x8D => {
                    println!("Instriction STA Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.write_byte(self.A, cycles, abs_addrress as u16, memory);
                }

                0x8E => {
                    println!("Instriction STX Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.write_byte(self.X, cycles, abs_addrress as u16, memory);
                }

                0x8C => {
                    println!("Instruction STY Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.write_byte(self.Y, cycles, abs_addrress as u16, memory);
                }

                0x9D => {
                    println!("Instruction STA Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as u16;
                    self.write_byte(self.A, cycles, abs_address_plus_x, memory);
                    *cycles -= 1;
                }

                0x99 => {
                    println!("Instruction STA Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.Y as u16;
                    self.write_byte(self.A, cycles, abs_address_plus_x, memory);
                    *cycles -= 1;
                }

                0x81 => {
                    println!("Instruction STA Indirect X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as u16, memory);
                    self.write_byte(self.A, cycles, effective_address, memory);
                }

                0x91 => {
                    println!("Instruction STA Indirect Y");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as u16, memory);
                    let effective_address_y: Word = effective_address + self.Y as u16;
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
                    self.ldx_register_set_status();
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
                    self.push_byte_to_stack(cycles, memory, self.PS);
                }

                0x68 => {
                    println!("Instructin PLA");
                    self.A = self.pop_byte_from_stack(cycles, memory);
                    self.lda_register_set_status();
                }

                0x28 => {
                    println!("Instructin PLP");
                    self.PS = self.pop_byte_from_stack(cycles, memory);
                }

                // Logical Operations
                0x29 => {
                    println!("instruction AND Inmediate");
                    self.A &= self.fetch_byte(cycles, memory);
                    self.lda_register_set_status();
                }

                0x09 => {
                    println!("instruction ORA Inmediate");
                    self.A |= self.fetch_byte(cycles, memory);
                    self.lda_register_set_status();
                }

                0x49 => {
                    println!("instructin EOR Inmediate");
                    self.A ^= self.fetch_byte(cycles, memory);
                    self.lda_register_set_status();
                }

                0x25 => {
                    println!("Instruction AND ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.A &= self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_register_set_status();
                }

                0x05 => {
                    println!("Instruction ORA ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.A |= self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_register_set_status();
                }

                0x45 => {
                    println!("Instruction EOR ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.A ^= self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_register_set_status();
                }

                0x35 => {
                    println!("Instruction AND ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.A &= self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_register_set_status();
                }

                0x15 => {
                    println!("Instruction ORA ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.A |= self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_register_set_status();
                }

                0x55 => {
                    println!("Instruction EOR ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.A ^= self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_register_set_status();
                }

                0x2D => {
                    println!("Instruction AND Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.A &= self.read_byte(cycles, abs_addrress as u16, memory);
                    self.lda_register_set_status();
                }

                0x0D => {
                    println!("Instruction ORA Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.A |= self.read_byte(cycles, abs_addrress as u16, memory);
                    self.lda_register_set_status();
                }

                0x4D => {
                    println!("Instruction EOR Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.A ^= self.read_byte(cycles, abs_addrress as u16, memory);
                    self.lda_register_set_status();
                }

                0x3D => {
                    println!("Instruction AND Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as u16;
                    self.A &= self.read_byte(cycles, abs_address_plus_x, memory);
                    if abs_address_plus_x - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0x1D => {
                    println!("Instruction ORA Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as u16;
                    self.A |= self.read_byte(cycles, abs_address_plus_x, memory);
                    if abs_address_plus_x - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0x5D => {
                    println!("Instruction EOR Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as u16;
                    self.A ^= self.read_byte(cycles, abs_address_plus_x, memory);
                    if abs_address_plus_x - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0x39 => {
                    println!("Instruction AND Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_y: Word = abs_addrress + self.Y as u16;
                    self.A &= self.read_byte(cycles, abs_address_plus_y, memory);
                    if abs_address_plus_y - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0x19 => {
                    println!("Instruction ORA Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_y: Word = abs_addrress + self.Y as u16;
                    self.A |= self.read_byte(cycles, abs_address_plus_y, memory);
                    if abs_address_plus_y - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0x59 => {
                    println!("Instruction EOR Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_y: Word = abs_addrress + self.Y as u16;
                    self.A ^= self.read_byte(cycles, abs_address_plus_y, memory);
                    if abs_address_plus_y - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0x21 => {
                    println!("Instruction AND Indirect X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as u16, memory);
                    self.A &= self.read_byte(cycles, effective_address, memory);
                    self.lda_register_set_status();
                }

                0x01 => {
                    println!("Instruction ORA Indirect X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as u16, memory);
                    self.A |= self.read_byte(cycles, effective_address, memory);
                    self.lda_register_set_status();
                }

                0x41 => {
                    println!("Instruction EOR Indirect X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as u16, memory);
                    self.A ^= self.read_byte(cycles, effective_address, memory);
                    self.lda_register_set_status();
                }

                0x31 => {
                    println!("Instruction AND Indirect Y");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as u16, memory);
                    let effective_address_y: Word = effective_address + self.Y as u16;
                    self.A &= self.read_byte(cycles, effective_address_y, memory);
                    if effective_address_y - effective_address >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0x11 => {
                    println!("Instruction ORA Indirect Y");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as u16, memory);
                    let effective_address_y: Word = effective_address + self.Y as u16;
                    self.A |= self.read_byte(cycles, effective_address_y, memory);
                    if effective_address_y - effective_address >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0x51 => {
                    println!("Instruction EOR Indirect Y");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word =
                        self.read_word(cycles, zero_page_address as u16, memory);
                    let effective_address_y: Word = effective_address + self.Y as u16;
                    self.A ^= self.read_byte(cycles, effective_address_y, memory);
                    if effective_address_y - effective_address >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0x24 => {
                    println!("Instruction BIT ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let value = self.read_byte(cycles, zero_page_address as u16, memory);

                    let z = self.A & value;
                    let z_bool: bool;
                    if z == 0 {
                        z_bool = true;
                    } else {
                        z_bool = false;
                    }

                    self.PS.set_bit(1, z_bool);

                    let n = (value & Flags::NegativeFlagBit as u8) != 0;
                    self.PS.set_bit(7, n);

                    let v = (value & Flags::OverflowFlagBit as u8) != 0;
                    self.PS.set_bit(6, v);
                }

                0x2C => {
                    println!("Instruction BIT ABS");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let value = self.read_byte(cycles, abs_addrress as u16, memory);

                    let z = self.A & value;
                    let z_bool: bool;
                    if z == 0 {
                        z_bool = true;
                    } else {
                        z_bool = false;
                    }

                    self.PS.set_bit(1, z_bool);

                    let n = (value & Flags::NegativeFlagBit as u8) != 0;
                    self.PS.set_bit(7, n);

                    let v = (value & Flags::OverflowFlagBit as u8) != 0;
                    self.PS.set_bit(6, v);
                }

                0xAA => {
                    println!("Instruction TAX");
                    self.X = self.A;
                    *cycles -= 1;
                    self.ldx_register_set_status();
                }

                0xA8 => {
                    println!("Instruction TAY");
                    self.Y = self.A;
                    *cycles -= 1;
                    self.ldy_register_set_status();
                }

                0x8A => {
                    println!("Instruction TXA");
                    self.A = self.X;
                    *cycles -= 1;
                    self.lda_register_set_status();
                }

                0x98 => {
                    println!("Instruction TYA");
                    self.A = self.Y;
                    *cycles -= 1;
                    self.lda_register_set_status();
                }

                0xE6 => {
                    println!("Instruction Increment Memory ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let mut value = self.read_byte(cycles, zero_page_address as u16, memory);
                    value += 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ldm_register_set_status(value);
                }

                0xF6 => {
                    println!("Instruction Increment Memory ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    let mut value = self.read_byte(cycles, zero_page_address as u16, memory);
                    value += 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ldm_register_set_status(value);
                }

                0xEE => {
                    println!("Instruction Increment Memory Absolute");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let mut value = self.read_byte(cycles, abs_address as u16, memory);
                    value += 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address.into(), memory);
                    self.ldm_register_set_status(value);
                }

                0xFE => {
                    println!("Instruction Increment Memory Absolute X");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x = abs_address + self.X as Word;
                    let mut value = self.read_byte(cycles, abs_address_plus_x, memory);
                    value += 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address_plus_x.into(), memory);
                    self.ldm_register_set_status(value);
                }

                0xC6 => {
                    println!("Instruction Decrement Memory ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let mut value = self.read_byte(cycles, zero_page_address as u16, memory);
                    value -= 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ldm_register_set_status(value);
                }

                0xD6 => {
                    println!("Instruction Decrement Memory ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    let mut value = self.read_byte(cycles, zero_page_address as u16, memory);
                    value -= 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, zero_page_address.into(), memory);
                    self.ldm_register_set_status(value);
                }

                0xCE => {
                    println!("Instruction Decrement Memory Absolute");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let mut value = self.read_byte(cycles, abs_address as u16, memory);
                    value -= 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address.into(), memory);
                    self.ldm_register_set_status(value);
                }

                0xDE => {
                    println!("Instruction Decrement Memory Absolute X");
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x = abs_address + self.X as Word;
                    let mut value = self.read_byte(cycles, abs_address_plus_x, memory);
                    value -= 1;
                    *cycles -= 1;
                    self.write_byte(value, cycles, abs_address_plus_x.into(), memory);
                    self.ldm_register_set_status(value);
                }

                0xE8 => {
                    println!("Instruction Inc X");
                    self.X = self.X.wrapping_add(1);
                    self.ldx_register_set_status();
                    *cycles -= 1;
                }

                0xC8 => {
                    println!("Instruction Inc Y");
                    self.Y = self.Y.wrapping_add(1);
                    self.ldy_register_set_status();
                    *cycles -= 1;
                }

                0xCA => {
                    println!("Instruction Dec X");
                    self.X = self.X.wrapping_sub(1);
                    self.ldx_register_set_status();
                    *cycles -= 1;
                }

                0x88 => {
                    println!("Instruction Dec Y");
                    self.Y = self.Y.wrapping_sub(1);
                    self.ldy_register_set_status();
                    *cycles -= 1;
                }

                // Branches
                0xF0 => {
                    println!("Instruction BEQ");
                    let offset: Byte = self.fetch_byte(cycles, memory);
                    if self.PS.get_bit(1) {
                        self.PC += offset as Word;
                        *cycles -= 1;
                    }
                }

                _ => {
                    unimplemented!("Instruction not handled {}", ins);
                }
            }
        }

        cycles_requested - *cycles
    }

    fn lda_register_set_status(&mut self) {
        self.PS = match self.A == 0 {
            false => *self.PS.set_bit(1, false),
            true => *self.PS.set_bit(1, true),
        };

        self.PS = match (self.A & 0b10000000) > 0 {
            false => *self.PS.set_bit(7, false),
            true => *self.PS.set_bit(7, true),
        };
    }

    fn ldx_register_set_status(&mut self) {
        self.PS = match self.X == 0 {
            false => *self.PS.set_bit(1, false),
            true => *self.PS.set_bit(1, true),
        };

        self.PS = match (self.X & 0b10000000) > 0 {
            false => *self.PS.set_bit(7, false),
            true => *self.PS.set_bit(7, true),
        };
    }

    fn ldy_register_set_status(&mut self) {
        self.PS = match self.Y == 0 {
            false => *self.PS.set_bit(1, false),
            true => *self.PS.set_bit(1, true),
        };

        self.PS = match (self.Y & 0b10000000) > 0 {
            false => *self.PS.set_bit(7, false),
            true => *self.PS.set_bit(7, true),
        };
    }

    fn ldm_register_set_status(&mut self, m: Byte) {
        self.PS = match m == 0 {
            false => *self.PS.set_bit(1, false),
            true => *self.PS.set_bit(1, true),
        };

        self.PS = match (m & 0b10000000) > 0 {
            false => *self.PS.set_bit(7, false),
            true => *self.PS.set_bit(7, true),
        };
    }
}
