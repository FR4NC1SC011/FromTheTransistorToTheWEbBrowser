mod mos6502; 
mod load_tests;
mod store_tests;
mod jumps_and_calls_tests;
mod stack_operations_tests;
mod and_eor_ora_tests;

use mos6502::*;

fn main() {
    let mut mem = mos6502::Mem {
        MAX_MEM: 1024 * 64,
        Data: Vec::new(),
    };

    let mut mem_copy = mos6502::Mem {
        MAX_MEM: 1024 * 64,
        Data: Vec::new(),
    };

    let mut cpu = mos6502::CPU::new();

    println!("6502 Emulator with rust");

    //         println!("{}", cpu.A);
    //         cpu.reset(&mut mem);
    //         cpu.X = 5;
    //         // start - inline a little program
    //         mem.Data[0xFFFC] = cpu.INS_LDA_ZPX;
    //         mem.Data[0xFFFD] = 0x42;
    //         mem.Data[0x0047] = 0x37;
    //         // end - inline a little program
    //
    //         println!("{}", cpu.A);
    //
    //         // when:
    //         cpu.execute(&mut 4, &mut mem);
    //
    //         // then:
    //         assert_eq!(cpu.A, 0x37);
    //         println!("{}", cpu.A);

    // println!("{:x?}, {:x?}", cpu, mem);

    let mut mem = Mem::new();
    let mut cpu = CPU::new();
    let mut cpu_copy = CPU::new();

    // given: 
    cpu.reset_vector(&mut mem, 0xFF00);
    cpu_copy.reset_vector(&mut mem, 0xFF00);
    mem.Data[0xFF00] = cpu.INS_JSR;
    mem.Data[0xFF01] = 0x00;
    mem.Data[0xFF02] = 0x80;
    mem.Data[0x8000] = cpu.INS_RTS;
    mem.Data[0xFF03] = cpu.INS_LDA_IM;
    mem.Data[0xFF04] = 0x42;

    let mut expected_cycles = 6 + 6 + 2;

    let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

    // then:
    assert_eq!(actual_cycles, 6 + 6 + 2);
    assert_eq!(cpu.A, 0x42);





//     cpu.reset(&mut mem);
//     cpu.reset(&mut mem_copy);
//     assert_eq!(mem.Data[0xFFFC], mem_copy.Data[0xFFFC]);
//     assert_eq!(mem.Data[0xFFFD], mem_copy.Data[0xFFFD]);
//     assert_eq!(mem.Data[0xFFFE], mem_copy.Data[0xFFFE]);
//     assert_eq!(mem.Data[0x4242], mem_copy.Data[0x4242]);
//     assert_eq!(mem.Data[0x4243], mem_copy.Data[0x4243]);
//     // println!("{:x?}, {:x?}", cpu, mem);
//     mem.Data[0xFFFC] = cpu.INS_JSR;
//     mem.Data[0xFFFD] = 0x42;
//     mem.Data[0xFFFE] = 0x42;
//     println!(
//         "0xFFFC: {} |  0xFFFD: {}",
//         mem.Data[0xFFFD], mem.Data[0xFFFE]
//     );
//     println!("PC: {}", cpu.PC);
//     mem.Data[0x4242] = cpu.INS_LDA_IM;
//     mem.Data[0x4243] = 0x84;
//     cpu.execute(&mut 9, &mut mem);
//     // println!("{:x?}, {:x?}", cpu, mem);
//     assert_ne!(mem.Data[0xFFFC], mem_copy.Data[0xFFFC]);
//     assert_ne!(mem.Data[0xFFFD], mem_copy.Data[0xFFFD]);
//     assert_ne!(mem.Data[0xFFFE], mem_copy.Data[0xFFFE]);
//     assert_ne!(mem.Data[0x4242], mem_copy.Data[0x4242]);
//     assert_ne!(mem.Data[0x4243], mem_copy.Data[0x4243]);
// 
//     assert_eq!(cpu.A, 0x84);
// 
//     println!("A: {}", cpu.A);
}
