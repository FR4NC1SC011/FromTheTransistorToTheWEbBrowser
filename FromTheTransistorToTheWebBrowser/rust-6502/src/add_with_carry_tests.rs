#[cfg(test)]
mod add_with_carry_tests {

    use std::os::raw::*;
    use bit_field::BitField;

    type Byte = c_uchar;
    type Word = c_ushort;

    use crate::Mem;
    use crate::CPU;

    struct ADCTestData {
        carry: bool,
        a: Byte,
        operand: Byte,
        answer: Byte,

        expect_c: bool,
        expect_z: bool,
        expect_n: bool,
        expect_v: bool,
    }

    #[derive(PartialEq)]
    enum EOperation {
        Add,
        Subtract,
    }


    fn verify_unmodified_flags(cpu: CPU, cpu_copy: CPU) {
        assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2));    // I
        assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3));    // D
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4));    // B
    }
    
    fn test_adc_or_sbc_absolute(test: ADCTestData, operation: EOperation) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, test.carry);      // C
        cpu.A = test.a;
        cpu.PS.set_bit(1, !test.expect_z);  // Z
        cpu.PS.set_bit(6, !test.expect_v);  // V
        cpu.PS.set_bit(7, !test.expect_n);  // N

        let opcode = if operation == EOperation::Add { cpu.INS_ADC_ABS } 
                                                else { cpu.INS_SBC_ABS };

        mem.Data[0xFF00] = opcode;         
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = test.operand;
        let mut expected_cycles = 4;
        let cpu_copy = cpu;

        // when 
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, test.answer);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c);     // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z);     // Z
        assert_eq!(cpu.PS.get_bit(6), test.expect_v);     // V
        assert_eq!(cpu.PS.get_bit(7), test.expect_n);     // N
        verify_unmodified_flags(cpu, cpu_copy);

    }

 fn test_adc_or_sbc_absolute_x(test: ADCTestData, operation: EOperation) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, test.carry);      // C
        cpu.X = 0x10;
        cpu.A = test.a;
        cpu.PS.set_bit(1, !test.expect_z);  // Z
        cpu.PS.set_bit(6, !test.expect_v);  // V
        cpu.PS.set_bit(7, !test.expect_n);  // N

        let opcode = if operation == EOperation::Add { cpu.INS_ADC_ABSX } 
                                                else { cpu.INS_SBC_ABSX };

        mem.Data[0xFF00] = opcode;         
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000 + 0x10] = test.operand;
        let mut expected_cycles = 4;
        let cpu_copy = cpu;

        // when 
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, test.answer);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c);     // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z);     // Z
        assert_eq!(cpu.PS.get_bit(6), test.expect_v);     // V
        assert_eq!(cpu.PS.get_bit(7), test.expect_n);     // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn test_adc_or_sbc_absolute_y(test: ADCTestData, operation: EOperation) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, test.carry);      // C
        cpu.Y = 0x10;
        cpu.A = test.a;
        cpu.PS.set_bit(1, !test.expect_z);  // Z
        cpu.PS.set_bit(6, !test.expect_v);  // V
        cpu.PS.set_bit(7, !test.expect_n);  // N

        let opcode = if operation == EOperation::Add { cpu.INS_ADC_ABSY } 
                                                else { cpu.INS_SBC_ABSY };

        mem.Data[0xFF00] = opcode;         
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000 + 0x10] = test.operand;
        let mut expected_cycles = 4;
        let cpu_copy = cpu;

        // when 
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, test.answer);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c);     // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z);     // Z
        assert_eq!(cpu.PS.get_bit(6), test.expect_v);     // V
        assert_eq!(cpu.PS.get_bit(7), test.expect_n);     // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn test_adc_or_sbc_inmediate(test: ADCTestData, operation: EOperation) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, test.carry);      // C
        cpu.A = test.a;
        cpu.PS.set_bit(1, !test.expect_z);  // Z
        cpu.PS.set_bit(6, !test.expect_v);  // V
        cpu.PS.set_bit(7, !test.expect_n);  // N

        let opcode = if operation == EOperation::Add { cpu.INS_ADC_IM } 
                                                else { cpu.INS_SBC_IM };

        mem.Data[0xFF00] = opcode;         
        mem.Data[0xFF01] = test.operand;
        let mut expected_cycles = 2;
        let cpu_copy = cpu;

        // when 
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.A, test.answer);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c);     // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z);     // Z
        assert_eq!(cpu.PS.get_bit(6), test.expect_v);     // V
        assert_eq!(cpu.PS.get_bit(7), test.expect_n);     // N
        verify_unmodified_flags(cpu, cpu_copy);
    }


    fn test_adc_or_sbc_zero_page(test: ADCTestData, operation: EOperation) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, test.carry);      // C
        cpu.A = test.a;
        cpu.PS.set_bit(1, !test.expect_z);  // Z
        cpu.PS.set_bit(6, !test.expect_v);  // V
        cpu.PS.set_bit(7, !test.expect_n);  // N

        let opcode = if operation == EOperation::Add { cpu.INS_ADC_ZP } 
                                                else { cpu.INS_SBC_ZP };

        mem.Data[0xFF00] = opcode;         
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = test.operand;
        let mut expected_cycles = 3;
        let cpu_copy = cpu;

        // when 
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        assert_eq!(cpu.A, test.answer);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c);     // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z);     // Z
        assert_eq!(cpu.PS.get_bit(6), test.expect_v);     // V
        assert_eq!(cpu.PS.get_bit(7), test.expect_n);     // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

     fn test_adc_or_sbc_zero_page_x(test: ADCTestData, operation: EOperation) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, test.carry);      // C
        cpu.A = test.a;
        cpu.X = 0x10;
        cpu.PS.set_bit(1, !test.expect_z);  // Z
        cpu.PS.set_bit(6, !test.expect_v);  // V
        cpu.PS.set_bit(7, !test.expect_n);  // N

        let opcode = if operation == EOperation::Add { cpu.INS_ADC_ZPX } 
                                                else { cpu.INS_SBC_ZPX };

        mem.Data[0xFF00] = opcode;         
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042 + 0x10] = test.operand;
        let mut expected_cycles = 4;
        let cpu_copy = cpu;

        // when 
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, test.answer);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c);     // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z);     // Z
        assert_eq!(cpu.PS.get_bit(6), test.expect_v);     // V
        assert_eq!(cpu.PS.get_bit(7), test.expect_n);     // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn test_adc_or_sbc_indirect_x(test: ADCTestData, operation: EOperation) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, test.carry);      // C
        cpu.A = test.a;
        cpu.X = 0x04;
        cpu.PS.set_bit(1, !test.expect_z);  // Z
        cpu.PS.set_bit(6, !test.expect_v);  // V
        cpu.PS.set_bit(7, !test.expect_n);  // N

        let opcode = if operation == EOperation::Add { cpu.INS_ADC_INDX } 
                                                else { cpu.INS_SBC_INDX };

        mem.Data[0xFF00] = opcode;         
        mem.Data[0xFF01] = 0x02;         
        mem.Data[0x0006] = 0x00;            // 0x2 + 0x4
        mem.Data[0x0007] = 0x80;
        mem.Data[0x8000] = test.operand;
        let mut expected_cycles = 6;
        let cpu_copy = cpu;

        // when 
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6);
        assert_eq!(cpu.A, test.answer);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c);     // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z);     // Z
        assert_eq!(cpu.PS.get_bit(6), test.expect_v);     // V
        assert_eq!(cpu.PS.get_bit(7), test.expect_n);     // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn test_adc_or_sbc_indirect_y(test: ADCTestData, operation: EOperation) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, test.carry);      // C
        cpu.A = test.a;
        cpu.Y = 0x04;
        cpu.PS.set_bit(1, !test.expect_z);  // Z
        cpu.PS.set_bit(6, !test.expect_v);  // V
        cpu.PS.set_bit(7, !test.expect_n);  // N

        let opcode = if operation == EOperation::Add { cpu.INS_ADC_INDY } 
                                                else { cpu.INS_SBC_INDY };

        mem.Data[0xFF00] = opcode;         
        mem.Data[0xFF01] = 0x02;         
        mem.Data[0x0002] = 0x00;            // 0x2 + 0x4
        mem.Data[0x0003] = 0x80;
        mem.Data[0x8000 + 0x04] = test.operand;
        let mut expected_cycles = 5;
        let cpu_copy = cpu;

        // when 
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 5);
        assert_eq!(cpu.A, test.answer);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c);     // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z);     // Z
        assert_eq!(cpu.PS.get_bit(6), test.expect_v);     // V
        assert_eq!(cpu.PS.get_bit(7), test.expect_n);     // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn adc_can_add_zero_to_zero_and_get_zero() {
        let test = ADCTestData {
            carry: false,
            a: 0,
            operand: 0,
            answer: 0,
            expect_c: false,
            expect_n: false,
            expect_v: false,
            expect_z: true,
        };
        
        test_adc_or_sbc_absolute(test, EOperation::Add);
    }

    #[test]
    fn adc_abs_can_add_carry_and_zero_to_zero_and_get_one() {
        let test = ADCTestData {
            carry: true,
            a: 0,
            operand: 0,
            answer: 1,
            expect_c: false,
            expect_n: false,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_absolute(test, EOperation::Add);
    }

    #[test]
    fn adc_abs_can_add_two_unsigned_numbers() {
        let test = ADCTestData {
            carry: true,
            a: 20,
            operand: 17,
            answer: 38,
            expect_c: false,
            expect_n: false,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_absolute(test, EOperation::Add);
    }

    #[test]
    fn adc_abs_can_add_one_to_ff_and_it_will_cause_a_carry() {
        let test = ADCTestData {
            carry: false,
            a: 0xFF,
            operand: 1,
            answer: 0,
            expect_c: true,
            expect_n: false,
            expect_v: false,
            expect_z: true,
        };
        
        test_adc_or_sbc_absolute(test, EOperation::Add);
    }

    #[test]
    fn adc_abs_will_set_the_neg_flag_when_the_result_is_negative() {
        let x: u8 = u8::MAX;    // x = -1 as u8;
        let test = ADCTestData {
            carry: false,
            a: 0,
            operand: x,
            answer: x,
            expect_c: false,
            expect_n: true,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_absolute(test, EOperation::Add);
    }

    #[test]
    fn adc_abs_will_set_the_overflow_flag_when_signed_negative_addition_fails() {
        let x: u8 = u8::MAX;    // x = -1 as u8;
        let test = ADCTestData {
            carry: false,
            a: u8::MAX - 127,
            operand: x,
            answer: 127,
            expect_c: true,
            expect_n: false,
            expect_v: true,
            expect_z: false,
        };
        
        test_adc_or_sbc_absolute(test, EOperation::Add);
    }

    #[test]
    fn adc_abs_will_set_the_overflow_flag_when_signed_negative_addition_passed_due_to_initial_carry_flag() {
        let x: u8 = u8::MAX;    // x = -1 as u8;
        let test = ADCTestData {
            carry: true,
            a: u8::MAX - 127,
            operand: x,
            answer: u8::MAX - 127,
            expect_c: true,
            expect_n: true,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_absolute(test, EOperation::Add);
    }

    #[test]
    fn adc_abs_will_set_the_overflow_flag_when_signed_positive_addition_fails() {
        let test = ADCTestData {
            carry: false,
            a: 127,
            operand: 1,
            answer: 128,
            expect_c: false,
            expect_n: true,
            expect_v: true,
            expect_z: false,
        };
        
        test_adc_or_sbc_absolute(test, EOperation::Add);
    }

    #[test]
    fn adc_inmediate_can_add_two_unsigned_numbers() {
        let test = ADCTestData {
            carry: true,
            a: 20,
            operand: 17,
            answer: 38,
            expect_c: false,
            expect_n: false,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_inmediate(test, EOperation::Add);
    }

    // #[test]
    // fn adc_inmediate_can_add_a_positive_number_and_a_negative_number() {
    //     let test = ADCTestData {
    //         carry: true,
    //         a: 20,
    //         operand: u8::MAX - 16,
    //         answer: 4,
    //         expect_c: true,
    //         expect_n: false,
    //         expect_v: false,
    //         expect_z: false,
    //     };
    //     
    //     test_adc_or_sbc_inmediate(test, EOperation::Add);
    // }

    #[test]
    fn adc_zero_page_can_add_two_unsigned_numbers() {
        let test = ADCTestData {
            carry: true,
            a: 20,
            operand: 17,
            answer: 38,
            expect_c: false,
            expect_n: false,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_zero_page(test, EOperation::Add);
    }

    // #[test]
    // fn adc_zero_page_can_add_a_positive_number_and_a_negative_number() {
    //     let test = ADCTestData {
    //         carry: true,
    //         a: 20,
    //         operand: u8::MAX - 16,
    //         answer: 4,
    //         expect_c: true,
    //         expect_n: false,
    //         expect_v: false,
    //         expect_z: false,
    //     };
    //     
    //     test_adc_or_sbc_zero_page(test, EOperation::Add);
    // }

    #[test]
    fn adc_zero_page_x_can_add_two_unsigned_numbers() {
        let test = ADCTestData {
            carry: true,
            a: 20,
            operand: 17,
            answer: 38,
            expect_c: false,
            expect_n: false,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_zero_page_x(test, EOperation::Add);
    }

    // #[test]
    // fn adc_zero_page_x_can_add_a_positive_number_and_a_negative_number() {
    //     let test = ADCTestData {
    //         carry: true,
    //         a: 20,
    //         operand: u8::MAX - 16,
    //         answer: 4,
    //         expect_c: true,
    //         expect_n: false,
    //         expect_v: false,
    //         expect_z: false,
    //     };
    //     test_adc_or_sbc_zero_page_x(test, EOperation::Add);
    // }

    #[test]
    fn adc_abs_x_can_add_two_unsigned_numbers() {
        let test = ADCTestData {
            carry: true,
            a: 20,
            operand: 17,
            answer: 38,
            expect_c: false,
            expect_n: false,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_absolute_x(test, EOperation::Add);
    }

    // #[test]
    // fn adc_abs_x_can_add_a_positive_number_and_a_negative_number() {
    //     let test = ADCTestData {
    //         carry: true,
    //         a: 20,
    //         operand: u8::MAX - 16,
    //         answer: 4,
    //         expect_c: true,
    //         expect_n: false,
    //         expect_v: false,
    //         expect_z: false,
    //     };
    //     test_adc_or_sbc_absolute_x(test, EOperation::Add);
    // }

    #[test]
    fn adc_abs_y_can_add_two_unsigned_numbers() {
        let test = ADCTestData {
            carry: true,
            a: 20,
            operand: 17,
            answer: 38,
            expect_c: false,
            expect_n: false,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_absolute_y(test, EOperation::Add);
    }

    // #[test]
    // fn adc_abs_y_can_add_a_positive_number_and_a_negative_number() {
    //     let test = ADCTestData {
    //         carry: true,
    //         a: 20,
    //         operand: u8::MAX - 16,
    //         answer: 4,
    //         expect_c: true,
    //         expect_n: false,
    //         expect_v: false,
    //         expect_z: false,
    //     };
    //     test_adc_or_sbc_absolute_y(test, EOperation::Add);
    // }

    #[test]
    fn adc_ind_x_can_add_two_unsigned_numbers() {
        let test = ADCTestData {
            carry: true,
            a: 20,
            operand: 17,
            answer: 38,
            expect_c: false,
            expect_n: false,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_indirect_x(test, EOperation::Add);
    }

//     #[test]
//     fn adc_ind_x_can_add_a_positive_number_and_a_negative_number() {
//         let test = ADCTestData {
//             carry: true,
//             a: 20,
//             operand: u8::MAX - 16,
//             answer: 4,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: false,
//         };
//         test_adc_or_sbc_indirect_x(test, EOperation::Add);
//     }

    #[test]
    fn adc_ind_y_can_add_two_unsigned_numbers() {
        let test = ADCTestData {
            carry: true,
            a: 20,
            operand: 17,
            answer: 38,
            expect_c: false,
            expect_n: false,
            expect_v: false,
            expect_z: false,
        };
        
        test_adc_or_sbc_indirect_y(test, EOperation::Add);
    }

    // #[test]
    // fn adc_ind_y_can_add_a_positive_number_and_a_negative_number() {
    //     let test = ADCTestData {
    //         carry: true,
    //         a: 20,
    //         operand: u8::MAX - 16,
    //         answer: 4,
    //         expect_c: true,
    //         expect_n: false,
    //         expect_v: false,
    //         expect_z: false,
    //     };
    //     test_adc_or_sbc_indirect_y(test, EOperation::Add);
    // }

///////////////////////////////////////////////////////////////////////////
//                          SBC ABS Tests                                //
///////////////////////////////////////////////////////////////////////////

//     #[test]
//     fn sbc_abs_can_subtract_zero_to_zero_and_get_zero() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 0,
//             answer: 0,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: true,
//         };
//         
//         test_adc_or_sbc_absolute(test, EOperation::Subtract);
//     }
// 
// 
//     #[test]
//     fn sbc_abs_can_subtract_carry_and_zero_to_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 0,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_abs_can_subtract_one_from_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX - 1,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_abs_can_subtract_one_from_zero_with_a_carry_and_get_minus_two() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX - 1,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_abs_can_subtract_two_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 127,
//             operand: 1,
//             answer: 127,
//             expect_c: true,
//             expect_n: false,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_abs_can_subtract_pos_and_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: 127,
//             operand: u8::MAX,
//             answer: 128,
//             expect_c: false,
//             expect_n: true,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_abs_can_subtract_two_unsigned_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: 20,
//             operand: 17,
//             answer: 3,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_abs_can_subtract_two_negative_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 20,
//             operand: u8::MAX - 17,
//             answer: u8::MAX - 2,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute(test, EOperation::Subtract);
//     }
// 
// 
//     // SBC Zero Page
// 
//     #[test]
//     fn sbc_zp_can_subtract_zero_to_zero_and_get_zero() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 0,
//             answer: 0,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: true,
//         };
//         
//         test_adc_or_sbc_zero_page(test, EOperation::Subtract);
//     }
// 
// 
//     #[test]
//     fn sbc_zp_can_subtract_carry_and_zero_to_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 0,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zp_can_subtract_one_from_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zp_can_subtract_one_from_zero_with_a_carry_and_get_minus_two() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX - 1,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zp_can_subtract_two_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 127,
//             operand: 1,
//             answer: 127,
//             expect_c: true,
//             expect_n: false,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zp_can_subtract_pos_and_neg_numbers_and_get_signed_overflow() {
//         let x: u8 = u8::MAX;    // x = -1 as u8;
//         let test = ADCTestData {
//             carry: true,
//             a: 127,
//             operand: u8::MAX,
//             answer: 128,
//             expect_c: false,
//             expect_n: true,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zp_can_subtract_two_unsigned_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: 20,
//             operand: 17,
//             answer: 3,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zp_can_subtract_two_negative_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 19,
//             operand: u8::MAX - 16,
//             answer: u8::MAX - 2,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page(test, EOperation::Subtract);
//     }
// 
// 
//     // SBC Inmediate
// 
//     #[test]
//     fn sbc_im_can_subtract_zero_to_zero_and_get_zero() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 0,
//             answer: 0,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: true,
//         };
//         
//         test_adc_or_sbc_inmediate(test, EOperation::Subtract);
//     }
// 
// 
//     #[test]
//     fn sbc_im_can_subtract_carry_and_zero_to_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 0,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_inmediate(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_im_can_subtract_one_from_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_inmediate(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_im_can_subtract_one_from_zero_with_a_carry_and_get_minus_two() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX - 1,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_inmediate(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_im_can_subtract_two_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 127,
//             operand: 1,
//             answer: 127,
//             expect_c: true,
//             expect_n: false,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_inmediate(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_im_can_subtract_pos_and_neg_numbers_and_get_signed_overflow() {
//         let x: u8 = u8::MAX;    // x = -1 as u8;
//         let test = ADCTestData {
//             carry: true,
//             a: 127,
//             operand: u8::MAX,
//             answer: 128,
//             expect_c: false,
//             expect_n: true,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_inmediate(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_im_can_subtract_two_unsigned_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: 20,
//             operand: 17,
//             answer: 3,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_inmediate(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_im_can_subtract_two_negative_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 19,
//             operand: u8::MAX - 16,
//             answer: u8::MAX - 2,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_inmediate(test, EOperation::Subtract);
//     }
// 
//     // SBC Zero Page X 
// 
//     #[test]
//     fn sbc_zpx_can_subtract_zero_to_zero_and_get_zero() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 0,
//             answer: 0,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: true,
//         };
//         
//         test_adc_or_sbc_zero_page_x(test, EOperation::Subtract);
//     }
// 
// 
//     #[test]
//     fn sbc_zpx_can_subtract_carry_and_zero_to_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 0,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zpx_can_subtract_one_from_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zpx_can_subtract_one_from_zero_with_a_carry_and_get_minus_two() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX - 1,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zpx_can_subtract_two_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 127,
//             operand: 1,
//             answer: 127,
//             expect_c: true,
//             expect_n: false,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zpx_can_subtract_pos_and_neg_numbers_and_get_signed_overflow() {
//         let x: u8 = u8::MAX;    // x = -1 as u8;
//         let test = ADCTestData {
//             carry: true,
//             a: 127,
//             operand: u8::MAX,
//             answer: 128,
//             expect_c: false,
//             expect_n: true,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zpx_can_subtract_two_unsigned_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: 20,
//             operand: 17,
//             answer: 3,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_zpx_can_subtract_two_negative_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 19,
//             operand: u8::MAX - 16,
//             answer: u8::MAX - 2,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_zero_page_x(test, EOperation::Subtract);
//     }
// 
//     // SBC Abs X
// 
//     #[test]
//     fn sbc_absx_can_subtract_zero_to_zero_and_get_zero() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 0,
//             answer: 0,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: true,
//         };
//         
//         test_adc_or_sbc_absolute_x(test, EOperation::Subtract);
//     }
// 
// 
//     #[test]
//     fn sbc_absx_can_subtract_carry_and_zero_to_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 0,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absx_can_subtract_one_from_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absx_can_subtract_one_from_zero_with_a_carry_and_get_minus_two() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX - 1,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absx_can_subtract_two_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 127,
//             operand: 1,
//             answer: 127,
//             expect_c: true,
//             expect_n: false,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absx_can_subtract_pos_and_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: 127,
//             operand: u8::MAX,
//             answer: 128,
//             expect_c: false,
//             expect_n: true,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absx_can_subtract_two_unsigned_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: 20,
//             operand: 17,
//             answer: 3,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absx_can_subtract_two_negative_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 19,
//             operand: u8::MAX - 16,
//             answer: u8::MAX - 2,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_x(test, EOperation::Subtract);
//     }
// 
//     // SBC Absolute Y 
// 
//     #[test]
//     fn sbc_absy_can_subtract_zero_to_zero_and_get_zero() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 0,
//             answer: 0,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: true,
//         };
//         
//         test_adc_or_sbc_absolute_y(test, EOperation::Subtract);
//     }
// 
// 
//     #[test]
//     fn sbc_absy_can_subtract_carry_and_zero_to_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 0,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absy_can_subtract_one_from_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absy_can_subtract_one_from_zero_with_a_carry_and_get_minus_two() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX - 1,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absy_can_subtract_two_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 127,
//             operand: 1,
//             answer: 127,
//             expect_c: true,
//             expect_n: false,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absy_can_subtract_pos_and_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: 127,
//             operand: u8::MAX,
//             answer: 128,
//             expect_c: false,
//             expect_n: true,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absy_can_subtract_two_unsigned_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: 20,
//             operand: 17,
//             answer: 3,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_absy_can_subtract_two_negative_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 19,
//             operand: u8::MAX - 16,
//             answer: u8::MAX - 2,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_absolute_y(test, EOperation::Subtract);
//     }
// 
//     // SBC Indirect X
// 
//     #[test]
//     fn sbc_indx_can_subtract_zero_to_zero_and_get_zero() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 0,
//             answer: 0,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: true,
//         };
//         
//         test_adc_or_sbc_indirect_x(test, EOperation::Subtract);
//     }
// 
// 
//     #[test]
//     fn sbc_indx_can_subtract_carry_and_zero_to_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 0,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indx_can_subtract_one_from_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indx_can_subtract_one_from_zero_with_a_carry_and_get_minus_two() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX - 1,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indx_can_subtract_two_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 127,
//             operand: 1,
//             answer: 127,
//             expect_c: true,
//             expect_n: false,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indx_can_subtract_pos_and_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: 127,
//             operand: u8::MAX,
//             answer: 128,
//             expect_c: false,
//             expect_n: true,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indx_can_subtract_two_unsigned_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: 20,
//             operand: 17,
//             answer: 3,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_x(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indx_can_subtract_two_negative_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 19,
//             operand: u8::MAX - 16,
//             answer: u8::MAX - 2,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_x(test, EOperation::Subtract);
//     }
// 
// 
//     // SBC Indirect Y
// 
//     #[test]
//     fn sbc_indy_can_subtract_zero_to_zero_and_get_zero() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 0,
//             answer: 0,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: true,
//         };
//         
//         test_adc_or_sbc_indirect_y(test, EOperation::Subtract);
//     }
// 
// 
//     #[test]
//     fn sbc_indy_can_subtract_carry_and_zero_to_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 0,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indy_can_subtract_one_from_zero_and_get_minus_one() {
//         let test = ADCTestData {
//             carry: true,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indy_can_subtract_one_from_zero_with_a_carry_and_get_minus_two() {
//         let test = ADCTestData {
//             carry: false,
//             a: 0,
//             operand: 1,
//             answer: u8::MAX - 1,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indy_can_subtract_two_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 127,
//             operand: 1,
//             answer: 127,
//             expect_c: true,
//             expect_n: false,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indy_can_subtract_pos_and_neg_numbers_and_get_signed_overflow() {
//         let test = ADCTestData {
//             carry: true,
//             a: 127,
//             operand: u8::MAX,
//             answer: 128,
//             expect_c: false,
//             expect_n: true,
//             expect_v: true,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indy_can_subtract_two_unsigned_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: 20,
//             operand: 17,
//             answer: 3,
//             expect_c: true,
//             expect_n: false,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_y(test, EOperation::Subtract);
//     }
// 
//     #[test]
//     fn sbc_indy_can_subtract_two_negative_numbers() {
//         let test = ADCTestData {
//             carry: true,
//             a: u8::MAX - 19,
//             operand: u8::MAX - 16,
//             answer: u8::MAX - 2,
//             expect_c: false,
//             expect_n: true,
//             expect_v: false,
//             expect_z: false,
//         };
//         
//         test_adc_or_sbc_indirect_y(test, EOperation::Subtract);
//     }
}
