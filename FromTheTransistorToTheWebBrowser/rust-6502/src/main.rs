use std::os::raw::*;

//TODO: alias declaration?
// Byte = c_uchar
// Word = c_ushort

#[derive(Debug)]
struct Mem {
    MAX_MEM: u32,
    Data: Vec<c_uchar>,
}

#[derive(Debug)]
struct CPU {
    PC: c_ushort, // program counter
    SP: c_ushort, // stack pointer

    // Registers
    A: c_uchar, // Accumulator
    X: c_uchar,
    Y: c_uchar,

    // Status flags
    C: c_uchar,
    Z: c_uchar,
    I: c_uchar,
    D: c_uchar,
    B: c_uchar,
    V: c_uchar,
    N: c_uchar,

    // Opcodes
    INS_LDA_IM: c_uchar,
    INS_LDA_ZP: c_uchar,
}

impl Mem {
    fn initialize(&mut self) {
        for d in 0..self.MAX_MEM {
            self.Data.push(0);
        }
    }
}

impl CPU {
    fn reset(&mut self, memory: &mut Mem) {
        self.PC = 0xFFFC;
        self.SP = 0x0100;

        self.A = 0;
        self.X = 0;
        self.Y = 0;

        self.C = 0;
        self.Z = 0;
        self.I = 0;
        self.D = 0;
        self.B = 0;
        self.V = 0;
        self.N = 0;

        memory.initialize();
    }

    fn fetch_byte(&mut self, cycles: &mut usize, memory: &mut Mem) -> c_uchar {
        let data: c_uchar = memory.Data[self.PC as usize];
        self.PC += 1;
        *cycles -= 1;
        data
    }

    fn read_byte(&mut self, cycles: &mut usize, address: c_uchar, memory: &mut Mem) -> c_uchar {
        let data: c_uchar = memory.Data[address as usize];
        *cycles -= 1;
        data
    }

    fn execute(&mut self, cycles: &mut usize, memory: &mut Mem) {
        while cycles > &mut 0 {
            let ins: c_uchar = self.fetch_byte(cycles, memory);

            match ins {
                0xA9 => {
                    println!("Instruction Load A");
                    let value: c_uchar = self.fetch_byte(cycles, memory);
                    self.A = value;
                    self.lda_set_status();
                }

                0xA5 => {
                    println!("Instruction Load ZP");
                    let zero_page_address: c_uchar = self.fetch_byte(cycles, memory);
                    self.A = self.read_byte(cycles, zero_page_address, memory);
                    self.lda_set_status();
                }

                _ => eprintln!("Instruction not handled {}", ins),
            }
        }
    }

    fn lda_set_status(&mut self) {
        self.Z = match self.A == 0 {
            false => 0,
            true => 1,
        };

        self.N = match (self.A & 0b10000000) > 0 {
            false => 0,
            true => 1,
        };
    }
}

fn main() {
    let mut mem = Mem {
        MAX_MEM: 1024 * 64,
        Data: Vec::new(),
    };

    let mut cpu = CPU {
        PC: 0,
        SP: 0,

        A: 0,
        X: 0,
        Y: 0,

        C: 1,
        Z: 1,
        I: 1,
        D: 1,
        B: 1,
        V: 1,
        N: 1,

        // Opcodes
        INS_LDA_IM: 0xA9,
        INS_LDA_ZP: 0xA5,
    };

    println!("6502 Emulator with rust");

    println!("{:x?}, {:x?}", cpu, mem);
    cpu.reset(&mut mem);
    println!("{:x?}, {:x?}", cpu, mem);
    mem.Data[0xFFFC] = cpu.INS_LDA_ZP;
    mem.Data[0xFFFD] = 0x42;
    mem.Data[0x0042] = 0x84;
    cpu.execute(&mut 3, &mut mem);
    println!("{:x?}, {:x?}", cpu, mem);
}
