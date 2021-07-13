#[cfg(test)]
mod compare_register_tests {

    use bit_field::BitField;
    use std::os::raw::*;

    type Byte = c_uchar;
    type Word = c_ushort;

    use crate::Mem;
    use crate::CPU;

    struct CMPTestData {
        register_value: Byte,
        operand: Byte,

        expect_c: bool,
        expect_z: bool,
        expect_n: bool,
    }

    impl CMPTestData {
        fn compare_two_identical_values() -> Self {
            let test = Self {
                register_value: 26,
                operand: 26,
                expect_c: true,
                expect_z: true,
                expect_n: false,
            };

            test
        }

        fn compare_large_positive_to_small_positive() -> Self {
            let test = Self {
                register_value: 48,
                operand: 26,
                expect_c: true,
                expect_z: false,
                expect_n: false,
            };

            test
        }

        fn compare_negative_number_to_a_positive() -> Self {
            let test = Self {
                register_value: 130, // Negative Number!!!
                operand: 26,
                expect_c: true,
                expect_z: false,
                expect_n: false,
            };

            test
        }

        fn compare_two_values_that_result_in_a_negative_flag_set() -> Self {
            let test = Self {
                register_value: 8,
                operand: 26,
                expect_c: false,
                expect_z: false,
                expect_n: true,
            };

            test
        }
    }

    #[derive(PartialEq)]
    enum ERegister {
        A,
        X,
        Y,
    }

    fn compare_inmediate(test: CMPTestData, register_to_compare: ERegister) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, !test.expect_c); // C
        cpu.PS.set_bit(1, !test.expect_z); // Z
        cpu.PS.set_bit(7, !test.expect_n); // N

        let mut register: *mut Byte = &mut cpu.A;

        let mut opcode: Byte = cpu.INS_CMP_IM;

        match register_to_compare {
            ERegister::X => {
                register = &mut cpu.X;
                opcode = cpu.INS_CPX_IM;
            }
            ERegister::Y => {
                register = &mut cpu.Y;
                opcode = cpu.INS_CPY_IM;
            }
            ERegister::A => {
                register = &mut cpu.A;
                opcode = cpu.INS_CMP_IM;
            }
        }

        unsafe {
            *register = test.register_value;
        }

        mem.Data[0xFF00] = opcode;
        mem.Data[0xFF01] = test.operand;
        let mut expected_cycles = 2;
        let cpu_copy = cpu;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        unsafe {
            assert_eq!(*register, test.register_value);
        }
        assert_eq!(cpu.PS.get_bit(0), test.expect_c); // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z); // Z
        assert_eq!(cpu.PS.get_bit(7), test.expect_n); // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn compare_zero_page(test: CMPTestData, register_to_compare: ERegister) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, !test.expect_c); // C
        cpu.PS.set_bit(1, !test.expect_z); // Z
        cpu.PS.set_bit(7, !test.expect_n); // N

        let mut register: *mut Byte = &mut cpu.A;

        let mut opcode: Byte = cpu.INS_CMP_ZP;

        match register_to_compare {
            ERegister::X => {
                register = &mut cpu.X;
                opcode = cpu.INS_CPX_ZP;
            }
            ERegister::Y => {
                register = &mut cpu.Y;
                opcode = cpu.INS_CPY_ZP;
            }
            ERegister::A => {
                register = &mut cpu.A;
                opcode = cpu.INS_CMP_ZP;
            }
        }

        unsafe {
            *register = test.register_value;
        }

        mem.Data[0xFF00] = opcode;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = test.operand;
        let mut expected_cycles = 3;
        let cpu_copy = cpu;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        unsafe {
            assert_eq!(*register, test.register_value);
        }
        assert_eq!(cpu.PS.get_bit(0), test.expect_c); // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z); // Z
        assert_eq!(cpu.PS.get_bit(7), test.expect_n); // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn compare_zero_page_x(test: CMPTestData, register_to_compare: ERegister) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, !test.expect_c); // C
        cpu.PS.set_bit(1, !test.expect_z); // Z
        cpu.PS.set_bit(7, !test.expect_n); // N

        cpu.A = test.register_value;
        cpu.X = 4;

        mem.Data[0xFF00] = cpu.INS_CMP_ZPX;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042 + 0x4] = test.operand;
        let mut expected_cycles = 4;
        let cpu_copy = cpu;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, test.register_value);
        assert_eq!(cpu.X, 4);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c); // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z); // Z
        assert_eq!(cpu.PS.get_bit(7), test.expect_n); // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn compare_absolute(test: CMPTestData, register_to_compare: ERegister) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, !test.expect_c); // C
        cpu.PS.set_bit(1, !test.expect_z); // Z
        cpu.PS.set_bit(7, !test.expect_n); // N

        let mut register: *mut Byte = &mut cpu.A;

        let mut opcode: Byte = cpu.INS_CMP_ABS;

        match register_to_compare {
            ERegister::X => {
                register = &mut cpu.X;
                opcode = cpu.INS_CPX_ABS;
            }
            ERegister::Y => {
                register = &mut cpu.Y;
                opcode = cpu.INS_CPY_ABS;
            }
            ERegister::A => {
                register = &mut cpu.A;
                opcode = cpu.INS_CMP_ABS;
            }
        }

        unsafe {
            *register = test.register_value;
        }

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
        unsafe {
            assert_eq!(*register, test.register_value);
        }
        assert_eq!(cpu.PS.get_bit(0), test.expect_c); // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z); // Z
        assert_eq!(cpu.PS.get_bit(7), test.expect_n); // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn compare_absolute_x(test: CMPTestData) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, !test.expect_c); // C
        cpu.PS.set_bit(1, !test.expect_z); // Z
        cpu.PS.set_bit(7, !test.expect_n); // N

        cpu.A = test.register_value;
        cpu.X = 4;

        mem.Data[0xFF00] = cpu.INS_CMP_ABSX;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000 + 4] = test.operand;
        let mut expected_cycles = 4;
        let cpu_copy = cpu;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, test.register_value);
        assert_eq!(cpu.X, 4);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c); // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z); // Z
        assert_eq!(cpu.PS.get_bit(7), test.expect_n); // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn compare_absolute_y(test: CMPTestData) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, !test.expect_c); // C
        cpu.PS.set_bit(1, !test.expect_z); // Z
        cpu.PS.set_bit(7, !test.expect_n); // N

        cpu.A = test.register_value;
        cpu.Y = 4;

        mem.Data[0xFF00] = cpu.INS_CMP_ABSY;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000 + 4] = test.operand;
        let mut expected_cycles = 4;
        let cpu_copy = cpu;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, test.register_value);
        assert_eq!(cpu.Y, 4);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c); // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z); // Z
        assert_eq!(cpu.PS.get_bit(7), test.expect_n); // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn compare_indirect_x(test: CMPTestData) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, !test.expect_c); // C
        cpu.PS.set_bit(1, !test.expect_z); // Z
        cpu.PS.set_bit(7, !test.expect_n); // N

        cpu.A = test.register_value;
        cpu.X = 4;

        mem.Data[0xFF00] = cpu.INS_CMP_INDX;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x42 + 4] = 0x00;
        mem.Data[0x42 + 5] = 0x80;
        mem.Data[0x8000] = test.operand;
        let mut expected_cycles = 6;
        let cpu_copy = cpu;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6);
        assert_eq!(cpu.A, test.register_value);
        assert_eq!(cpu.X, 4);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c); // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z); // Z
        assert_eq!(cpu.PS.get_bit(7), test.expect_n); // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn compare_indirect_y(test: CMPTestData) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, !test.expect_c); // C
        cpu.PS.set_bit(1, !test.expect_z); // Z
        cpu.PS.set_bit(7, !test.expect_n); // N

        cpu.A = test.register_value;
        cpu.Y = 4;

        mem.Data[0xFF00] = cpu.INS_CMP_INDY;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x42] = 0x00;
        mem.Data[0x43] = 0x80;
        mem.Data[0x8000 + 4] = test.operand;
        let mut expected_cycles = 5;
        let cpu_copy = cpu;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 5);
        assert_eq!(cpu.A, test.register_value);
        assert_eq!(cpu.Y, 4);
        assert_eq!(cpu.PS.get_bit(0), test.expect_c); // C
        assert_eq!(cpu.PS.get_bit(1), test.expect_z); // Z
        assert_eq!(cpu.PS.get_bit(7), test.expect_n); // N
        verify_unmodified_flags(cpu, cpu_copy);
    }

    // -- INMEDIATE

    #[test]
    fn cmp_inmediate_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_inmediate(test, ERegister::A);
    }

    #[test]
    fn cmp_inmediate_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_inmediate(test, ERegister::A);
    }

    #[test]
    fn cmp_inmediate_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_inmediate(test, ERegister::A);
    }

    #[test]
    fn cmp_inmediate_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_inmediate(test, ERegister::A);
    }

    // -- ZERO PAGE

    #[test]
    fn cmp_zero_page_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_zero_page(test, ERegister::A);
    }

    #[test]
    fn cmp_zero_page_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_zero_page(test, ERegister::A);
    }

    #[test]
    fn cmp_zero_page_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_zero_page(test, ERegister::A);
    }

    #[test]
    fn cmp_zero_page_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_zero_page(test, ERegister::A);
    }

    // -- ZERO PAGE X

    #[test]
    fn cmp_zero_page_x_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_zero_page_x(test, ERegister::A);
    }

    #[test]
    fn cmp_zero_page_x_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_zero_page_x(test, ERegister::A);
    }

    #[test]
    fn cmp_zero_page_x_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_zero_page_x(test, ERegister::A);
    }

    #[test]
    fn cmp_zero_page_x_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_zero_page_x(test, ERegister::A);
    }

    // -- ABSOLUTE

    #[test]
    fn cmp_absolute_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_absolute(test, ERegister::A);
    }

    #[test]
    fn cmp_absolute_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_absolute(test, ERegister::A);
    }

    #[test]
    fn cmp_absolute_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_absolute(test, ERegister::A);
    }

    #[test]
    fn cmp_absolute_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_absolute(test, ERegister::A);
    }

    // -- ABSOLUTE X

    #[test]
    fn cmp_absolute_x_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_absolute_x(test);
    }

    #[test]
    fn cmp_absolute_x_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_absolute_x(test);
    }

    #[test]
    fn cmp_absolute_x_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_absolute_x(test);
    }

    #[test]
    fn cmp_absolute_x_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_absolute_x(test);
    }

    // -- ABSOLUTE Y

    #[test]
    fn cmp_absolute_y_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_absolute_y(test);
    }

    #[test]
    fn cmp_absolute_y_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_absolute_y(test);
    }

    #[test]
    fn cmp_absolute_y_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_absolute_y(test);
    }

    #[test]
    fn cmp_absolute_y_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_absolute_y(test);
    }

    // -- INDIRECT X

    #[test]
    fn cmp_indirect_x_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_indirect_x(test);
    }

    #[test]
    fn cmp_indirect_x_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_indirect_x(test);
    }

    #[test]
    fn cmp_indirect_x_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_indirect_x(test);
    }

    #[test]
    fn cmp_indirect_x_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_indirect_x(test);
    }

    // -- INDIRECT Y

    #[test]
    fn cmp_indirect_y_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_indirect_y(test);
    }

    #[test]
    fn cmp_indirect_y_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_indirect_y(test);
    }

    #[test]
    fn cmp_indirect_y_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_indirect_y(test);
    }

    #[test]
    fn cmp_indirect_y_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_indirect_y(test);
    }

    // CPX Inmediate

    #[test]
    fn cpx_inmediate_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_inmediate(test, ERegister::X);
    }

    #[test]
    fn cpx_inmediate_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_inmediate(test, ERegister::X);
    }

    #[test]
    fn cpx_inmediate_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_inmediate(test, ERegister::X);
    }

    #[test]
    fn cpx_inmediate_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_inmediate(test, ERegister::X);
    }

    // CPY Inmediate

    #[test]
    fn cpy_inmediate_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_inmediate(test, ERegister::Y);
    }

    #[test]
    fn cpy_inmediate_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_inmediate(test, ERegister::Y);
    }

    #[test]
    fn cpy_inmediate_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_inmediate(test, ERegister::Y);
    }

    #[test]
    fn cpy_inmediate_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_inmediate(test, ERegister::Y);
    }

    // CPX Zero Page

    #[test]
    fn cpx_zero_page_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_zero_page(test, ERegister::X);
    }

    #[test]
    fn cpx_zero_page_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_zero_page(test, ERegister::X);
    }

    #[test]
    fn cpx_zero_page_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_zero_page(test, ERegister::X);
    }

    #[test]
    fn cpx_zero_page_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_zero_page(test, ERegister::X);
    }

    // CPY ZERO PAGE

    #[test]
    fn cpy_zero_page_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_zero_page(test, ERegister::Y);
    }

    #[test]
    fn cpy_zero_page_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_zero_page(test, ERegister::Y);
    }

    #[test]
    fn cpy_zero_page_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_zero_page(test, ERegister::Y);
    }

    #[test]
    fn cpy_zero_page_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_zero_page(test, ERegister::Y);
    }

    // -- CPX ABSOLUTE

    #[test]
    fn cpx_absolute_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_absolute(test, ERegister::X);
    }

    #[test]
    fn cpx_absolute_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_absolute(test, ERegister::X);
    }

    #[test]
    fn cpx_absolute_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_absolute(test, ERegister::X);
    }

    #[test]
    fn cpx_absolute_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_absolute(test, ERegister::X);
    }

    // -- CPY ABSOLUTE

    #[test]
    fn cpy_absolute_can_compare_two_identical_values() {
        let test: CMPTestData = CMPTestData::compare_two_identical_values();
        compare_absolute(test, ERegister::Y);
    }

    #[test]
    fn cpy_absolute_can_compare_a_large_positive_to_a_small_positive() {
        let test: CMPTestData = CMPTestData::compare_large_positive_to_small_positive();
        compare_absolute(test, ERegister::Y);
    }

    #[test]
    fn cpy_absolute_can_compare_a_negative_to_positive() {
        let test: CMPTestData = CMPTestData::compare_negative_number_to_a_positive();
        compare_absolute(test, ERegister::Y);
    }

    #[test]
    fn cpy_absolute_can_compare_two_values_that_result_in_negative_flag_set() {
        let test: CMPTestData =
            CMPTestData::compare_two_values_that_result_in_a_negative_flag_set();
        compare_absolute(test, ERegister::Y);
    }

    fn verify_unmodified_flags(cpu: CPU, cpu_copy: CPU) {
        assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2)); // I
        assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3)); // D
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4)); // B
        assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6)); // V
    }
}
