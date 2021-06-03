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

    INS_LDA_IM: c_uchar, // Load Accumulator
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

    fn execute(&mut self, cycles: &mut usize, memory: &mut Mem) {
        while cycles > &mut 0 {
            let ins: c_uchar = self.fetch_byte(cycles, memory);

            match ins {
                INS_LDA_IM => {
                    println!("Instruction Load A");
                    let value: c_uchar = self.fetch_byte(cycles, memory);
                    self.A = value;

                    self.Z = match self.A == 0 {
                        false => 0,
                        true => 1,
                    };

                    self.N = match (self.A & 0b10000000) > 0 {
                        false => 0,
                        true => 1,
                    };
                }

                _ => eprintln!("Instruction not handled {}", ins),
            }
        }
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

        INS_LDA_IM: 0xA9,
    };

    println!("6502 Emulator with rust");

    // TODO: print values in hex
    println!("{:?}, {:?}", cpu, mem);
    cpu.reset(&mut mem);
    println!("{:?}, {:?}", cpu, mem);
    mem.Data[0xFFFC] = cpu.INS_LDA_IM;
    mem.Data[0xFFFD] = 0x42;
    cpu.execute(&mut 2, &mut mem);
    println!("{:?}, {:?}", cpu, mem);
}
