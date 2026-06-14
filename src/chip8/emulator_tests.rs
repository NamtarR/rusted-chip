use super::*;

#[test]
fn load_copies_bytes_to_program_start() {
    let mut emulator = Emulator::new();
    let bytes: [u8; 512] = [1; 512];

    emulator.load(&bytes);

    let memory = &emulator.memory[PROGRAM_START_INDEX..PROGRAM_START_INDEX + 512];

    assert_eq!(&bytes[..], memory);
}

#[test]
fn load_sets_pc_to_program_start() {
    let mut emulator = Emulator::new();
    let bytes: [u8; 16] = [1; 16];

    emulator.pc = 0x00;
    emulator.load(&bytes);

    assert_eq!(PROGRAM_START_ADDRESS, emulator.pc);
}

#[test]
fn load_accepts_largest_valid_rom() {
    let mut emulator = Emulator::new();
    let bytes: [u8; MAX_ROM_SIZE] = [1; MAX_ROM_SIZE];

    emulator.load(&bytes);

    let memory = &emulator.memory[PROGRAM_START_INDEX..PROGRAM_START_INDEX + MAX_ROM_SIZE];

    assert_eq!(&bytes[..], memory);
}

#[test]
#[should_panic]
fn load_rom_larger_than_memory_panics() {
    let mut emulator = Emulator::new();
    let bytes: [u8; MAX_ROM_SIZE + 1] = [1; MAX_ROM_SIZE + 1];

    emulator.load(&bytes);
}

#[test]
fn reset_restores_initial_state() {
    let mut emulator = Emulator::new();

    emulator.v = [1; REGISTER_COUNT];
    emulator.i = 128;
    emulator.pc = 0x400;
    emulator.memory = [1; MEMORY_SIZE];
    emulator.stack = [1; STACK_SIZE];
    emulator.stack_pointer = 13;
    emulator.display = [[true; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

    emulator.reset();

    assert_eq!(emulator.v, [0; REGISTER_COUNT]);
    assert_eq!(emulator.i, 0);
    assert_eq!(emulator.pc, PROGRAM_START_ADDRESS);
    assert_eq!(emulator.memory, [0; MEMORY_SIZE]);
    assert_eq!(emulator.stack, [0; STACK_SIZE]);
    assert_eq!(emulator.stack_pointer, 0);
    assert_eq!(emulator.display, [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT]);
}

#[test]
#[should_panic]
fn execute_unknown_instruction_panics() {
    let mut emulator = Emulator::new();

    emulator.memory[PROGRAM_START_INDEX] = 0x0F;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0xFF;

    emulator.execute();
}

#[test]
fn execute_valid_instruction_advances_pc() {
    let mut emulator = Emulator::new();

    emulator.memory[PROGRAM_START_INDEX] = 0x00;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0xE0;

    emulator.execute();

    assert_eq!(emulator.pc, PROGRAM_START_ADDRESS + 2);
}

#[test]
fn execute_00e0_clears_display() {
    let mut emulator = Emulator::new();

    emulator.display = [[true; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    emulator.memory[PROGRAM_START_INDEX] = 0x00;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0xE0;

    emulator.execute();

    assert_eq!(emulator.display, [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT]);
}

#[test]
fn execute_00ee_pops_stack() {
    let mut emulator = Emulator::new();

    emulator.stack[0] = 0x400;
    emulator.stack_pointer = 1;
    emulator.memory[PROGRAM_START_INDEX] = 0x00;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0xEE;

    emulator.execute();

    assert_eq!(emulator.pc, 0x400);
    assert_eq!(emulator.stack_pointer, 0);
    assert_eq!(emulator.stack[0], 0);
}

#[test]
#[should_panic]
fn execute_00ee_on_empty_stack_panics() {
    let mut emulator = Emulator::new();

    emulator.stack[0] = 0x400;
    emulator.stack_pointer = 0;
    emulator.memory[PROGRAM_START_INDEX] = 0x00;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0xEE;

    emulator.execute();
}

#[test]
fn execute_1nnn_jumps_to_address() {
    let mut emulator = Emulator::new();

    emulator.memory[PROGRAM_START_INDEX] = 0x1F;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0xED;

    emulator.execute();

    assert_eq!(emulator.pc, 0xFED);
}

#[test]
fn execute_2nnn_pushes_to_stack() {
    let mut emulator = Emulator::new();

    emulator.stack[0] = 0;
    emulator.stack_pointer = 0;
    emulator.memory[PROGRAM_START_INDEX] = 0x24;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x56;

    emulator.execute();

    assert_eq!(emulator.pc, 0x456);
    assert_eq!(emulator.stack_pointer, 1);
    assert_eq!(emulator.stack[0], 0x202);
}

#[test]
#[should_panic]
fn execute_2nnn_on_stack_overflow_panics() {
    let mut emulator = Emulator::new();

    emulator.pc = 0x200;
    emulator.stack_pointer = STACK_SIZE;
    emulator.memory[PROGRAM_START_INDEX] = 0x24;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x56;

    emulator.execute();
}

#[test]
fn execute_3xnn_skips_if_vx_equals_nn() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[4] = 0x32;

    emulator.memory[PROGRAM_START_INDEX] = 0x34;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x32;

    emulator.execute();

    assert_eq!(emulator.pc, 0x204);
}

#[test]
fn execute_3xnn_does_not_skip_if_vx_does_not_equal_nn() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[6] = 0x62;

    emulator.memory[PROGRAM_START_INDEX] = 0x36;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x32;

    emulator.execute();

    assert_eq!(emulator.pc, 0x202);
}

#[test]
fn execute_4xnn_skips_if_vx_does_not_equal_nn() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[6] = 0x62;

    emulator.memory[PROGRAM_START_INDEX] = 0x46;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x32;

    emulator.execute();

    assert_eq!(emulator.pc, 0x204);
}

#[test]
fn execute_4xnn_does_not_skip_if_vx_equals_nn() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[4] = 0x32;

    emulator.memory[PROGRAM_START_INDEX] = 0x44;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x32;

    emulator.execute();

    assert_eq!(emulator.pc, 0x202);
}

#[test]
fn execute_5xy0_skips_if_vx_equals_vy() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[4] = 0x32;
    emulator.v[5] = 0x32;

    emulator.memory[PROGRAM_START_INDEX] = 0x54;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x50;

    emulator.execute();

    assert_eq!(emulator.pc, 0x204);
}

#[test]
fn execute_5xy0_does_not_skip_if_vx_does_not_equal_vy() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[6] = 0x62;
    emulator.v[7] = 0x63;

    emulator.memory[PROGRAM_START_INDEX] = 0x56;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x70;

    emulator.execute();

    assert_eq!(emulator.pc, 0x202);
}

#[test]
#[should_panic]
fn execute_5xyn_panics() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[6] = 0x62;
    emulator.v[7] = 0x63;

    emulator.memory[PROGRAM_START_INDEX] = 0x56;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x71;

    emulator.execute();
}

#[test]
fn execute_6xnn_sets_vx_to_nn() {
    let mut emulator = Emulator::new();

    emulator.memory[PROGRAM_START_INDEX] = 0x63;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0xFE;

    emulator.execute();

    assert_eq!(emulator.v[0x3], 0xFE);
}

#[test]
fn execute_7xnn_adds_nn_to_vx() {
    let mut emulator = Emulator::new();

    emulator.v[0x2] = 0x15;
    emulator.memory[PROGRAM_START_INDEX] = 0x72;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x44;

    emulator.execute();

    assert_eq!(emulator.v[0x2], 0x59);
    assert_eq!(emulator.v[0xF], 0);
}

#[test]
fn execute_7xnn_adds_nn_to_vx_and_wraps_on_overflow() {
    let mut emulator = Emulator::new();

    emulator.v[0x2] = 0xF0;
    emulator.memory[PROGRAM_START_INDEX] = 0x72;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x20;

    emulator.execute();

    assert_eq!(emulator.v[0x2], 0x10);
    assert_eq!(emulator.v[0xF], 0);
}

#[test]
fn execute_8xy0_sets_vx_to_vy() {
    let mut emulator = Emulator::new();

    emulator.v[0x3] = 0x00;
    emulator.v[0x4] = 0xF1;
    emulator.memory[PROGRAM_START_INDEX] = 0x83;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x40;

    emulator.execute();

    assert_eq!(emulator.v[0x3], 0xF1);
}

#[test]
fn execute_8xy1_sets_vx_to_vx_or_vy() {
    let mut emulator = Emulator::new();

    emulator.v[0x3] = 0b11110000;
    emulator.v[0x4] = 0b11011001;
    emulator.memory[PROGRAM_START_INDEX] = 0x83;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x41;

    emulator.execute();

    assert_eq!(emulator.v[0x3], 0b11111001);
}

#[test]
fn execute_8xy2_sets_vx_to_vx_and_vy() {
    let mut emulator = Emulator::new();

    emulator.v[0x3] = 0b11110000;
    emulator.v[0x4] = 0b11011001;
    emulator.memory[PROGRAM_START_INDEX] = 0x83;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x42;

    emulator.execute();

    assert_eq!(emulator.v[0x3], 0b11010000);
}

#[test]
fn execute_8xy3_sets_vx_to_vx_xor_vy() {
    let mut emulator = Emulator::new();

    emulator.v[0x3] = 0b11110000;
    emulator.v[0x4] = 0b11011001;
    emulator.memory[PROGRAM_START_INDEX] = 0x83;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x43;

    emulator.execute();

    assert_eq!(emulator.v[0x3], 0b00101001);
}

#[test]
fn execute_8xy4_sets_vx_to_vx_plus_vy() {
    let mut emulator = Emulator::new();

    emulator.v[0x5] = 0x32;
    emulator.v[0x6] = 0x56;
    emulator.memory[PROGRAM_START_INDEX] = 0x85;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x64;

    emulator.execute();

    assert_eq!(emulator.v[0x5], 0x88);
    assert_eq!(emulator.v[0xF], 0);
}

#[test]
fn execute_8xy4_sets_vx_to_vx_plus_vy_and_sets_carry_flag() {
    let mut emulator = Emulator::new();

    emulator.v[0x5] = 0xF0;
    emulator.v[0x6] = 0xF1;
    emulator.memory[PROGRAM_START_INDEX] = 0x85;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x64;

    emulator.execute();

    assert_eq!(emulator.v[0x5], 0xE1);
    assert_eq!(emulator.v[0xF], 1);
}

#[test]
fn execute_8xy5_sets_vx_to_vx_minus_vy_and_sets_no_borrow_flag() {
    let mut emulator = Emulator::new();

    emulator.v[0x5] = 0x89;
    emulator.v[0x6] = 0x3A;
    emulator.memory[PROGRAM_START_INDEX] = 0x85;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x65;

    emulator.execute();

    assert_eq!(emulator.v[0x5], 0x4F);
    assert_eq!(emulator.v[0xF], 1);
}

#[test]
fn execute_8xy5_sets_vx_to_vx_minus_vy() {
    let mut emulator = Emulator::new();

    emulator.v[0x5] = 0xD3;
    emulator.v[0x6] = 0xF1;
    emulator.memory[PROGRAM_START_INDEX] = 0x85;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x65;

    emulator.execute();

    assert_eq!(emulator.v[0x5], 0xE2);
    assert_eq!(emulator.v[0xF], 0);
}

#[test]
fn execute_8xy6_shrs_vx_in_place_and_sets_vf_to_shifted_out_bit() {
    let mut emulator = Emulator::new();

    emulator.v[0xF] = 0;
    emulator.v[0x8] = 0b10001001;
    emulator.memory[PROGRAM_START_INDEX] = 0x88;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x06;

    emulator.execute();

    assert_eq!(emulator.v[0x8], 0b01000100);
    assert_eq!(emulator.v[0xF], 1);
}

#[test]
fn execute_8xye_shls_vx_in_place_and_sets_vf_to_shifted_out_bit() {
    let mut emulator = Emulator::new();

    emulator.v[0xF] = 1;
    emulator.v[0x8] = 0b01001001;
    emulator.memory[PROGRAM_START_INDEX] = 0x88;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x0E;

    emulator.execute();

    assert_eq!(emulator.v[0x8], 0b10010010);
    assert_eq!(emulator.v[0xF], 0);
}

#[test]
fn execute_8xy7_sets_vx_to_vy_minus_vx_and_sets_no_borrow_flag() {
    let mut emulator = Emulator::new();

    emulator.v[0x5] = 0xD3;
    emulator.v[0x6] = 0xF1;
    emulator.memory[PROGRAM_START_INDEX] = 0x85;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x67;

    emulator.execute();

    assert_eq!(emulator.v[0x5], 0x1E);
    assert_eq!(emulator.v[0xF], 1);
}

#[test]
fn execute_8xy7_sets_vx_to_vy_minus_vx() {
    let mut emulator = Emulator::new();

    emulator.v[0x5] = 0x89;
    emulator.v[0x6] = 0x3A;
    emulator.memory[PROGRAM_START_INDEX] = 0x85;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x67;

    emulator.execute();

    assert_eq!(emulator.v[0x5], 0xB1);
    assert_eq!(emulator.v[0xF], 0);
}

#[test]
#[should_panic]
fn execute_8xyn_panics() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[6] = 0x62;
    emulator.v[7] = 0x63;

    emulator.memory[PROGRAM_START_INDEX] = 0x86;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x7F;

    emulator.execute();
}

#[test]
fn execute_9xy0_skips_if_vx_does_not_equal_vy() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[6] = 0x62;
    emulator.v[7] = 0x63;

    emulator.memory[PROGRAM_START_INDEX] = 0x96;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x70;

    emulator.execute();

    assert_eq!(emulator.pc, 0x204);
}

#[test]
fn execute_9xy0_does_not_skip_if_vx_equals_vy() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[4] = 0x32;
    emulator.v[5] = 0x32;

    emulator.memory[PROGRAM_START_INDEX] = 0x94;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x50;

    emulator.execute();

    assert_eq!(emulator.pc, 0x202);
}

#[test]
#[should_panic]
fn execute_9xyn_panics() {
    let mut emulator = Emulator::new();

    emulator.pc = PROGRAM_START_ADDRESS;
    emulator.v[6] = 0x62;
    emulator.v[7] = 0x63;

    emulator.memory[PROGRAM_START_INDEX] = 0x96;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x71;

    emulator.execute();
}

#[test]
fn execute_annn_sets_i_to_nnn() {
    let mut emulator = Emulator::new();

    emulator.memory[PROGRAM_START_INDEX] = 0xA1;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x23;

    emulator.execute();

    assert_eq!(emulator.i, 0x123);
}

#[test]
fn execute_bnnn_jumps_to_address_v0_plus_nnn() {
    let mut emulator = Emulator::new();

    emulator.v[0] = 0x1A;
    emulator.memory[PROGRAM_START_INDEX] = 0xB6;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x54;

    emulator.execute();

    assert_eq!(emulator.pc, 0x66E);
}

#[test]
fn execute_cxnn_with_zero_mask_sets_vx_to_zero() {
    let mut emulator = Emulator::new();

    emulator.v[6] = 0x1A;
    emulator.memory[PROGRAM_START_INDEX] = 0xC6;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x00;

    emulator.execute();

    assert_eq!(emulator.v[6], 0);
}

#[test]
fn execute_cxnn_with_0f_mask_clears_high_nibble() {
    let mut emulator = Emulator::new();

    emulator.v[6] = 0x1A;
    emulator.memory[PROGRAM_START_INDEX] = 0xC6;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x0F;

    emulator.execute();

    assert_eq!(emulator.v[6] & 0xF0, 0);
}

#[test]
fn execute_cxnn_with_f0_mask_clears_low_nibble() {
    let mut emulator = Emulator::new();

    emulator.v[6] = 0x1A;
    emulator.memory[PROGRAM_START_INDEX] = 0xC6;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0xF0;

    emulator.execute();

    assert_eq!(emulator.v[6] & 0x0F, 0);
}

#[test]
fn execute_dxyn_updates_display_clears_vf_with_no_collision() {
    let mut emulator = Emulator::new();

    emulator.v[0x1] = 0x04; // x coordinate
    emulator.v[0x2] = 0x02; // y coordinate
    emulator.v[0xF] = 1; // overflow bit

    emulator.display[0x02][0x04] = true;
    emulator.display[0x03][0x08] = true;
    emulator.display[0x03][0x09] = true;
    emulator.display[0x03][0x0A] = true;
    emulator.display[0x03][0x0B] = true;

    emulator.memory[PROGRAM_START_INDEX + 0x100] = 0b01010101;
    emulator.memory[PROGRAM_START_INDEX + 0x101] = 0b11110000;

    emulator.i = PROGRAM_START_ADDRESS + 0x100;

    emulator.memory[PROGRAM_START_INDEX] = 0xD1;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x22;

    emulator.execute();

    assert_eq!(
        emulator.display[0x02][0x04..0x0C],
        [true, true, false, true, false, true, false, true]
    );
    assert_eq!(
        emulator.display[0x03][0x04..0x0C],
        [true, true, true, true, true, true, true, true]
    );
    assert_eq!(emulator.v[0xF], 0);
}

#[test]
fn execute_dxyn_updates_display_sets_vf_on_collision() {
    let mut emulator = Emulator::new();

    emulator.v[0x1] = 0x04; // x coordinate
    emulator.v[0x2] = 0x02; // y coordinate
    emulator.v[0xF] = 0; // overflow bit

    emulator.display[0x02][0x04] = true;

    emulator.memory[PROGRAM_START_INDEX + 0x100] = 0b11111111;

    emulator.i = PROGRAM_START_ADDRESS + 0x100;

    emulator.memory[PROGRAM_START_INDEX] = 0xD1;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x21;

    emulator.execute();

    assert_eq!(
        emulator.display[0x02][0x04..0x0C],
        [false, true, true, true, true, true, true, true]
    );

    assert_eq!(emulator.v[0xF], 1);
}

#[test]
fn execute_dxyn_wraps_starting_coordinates() {
    let mut emulator = Emulator::new();

    emulator.v[0x1] = 0x60; // x coordinate wraps to 0x20
    emulator.v[0x2] = 0x30; // y coordinate wraps to 0x10

    emulator.memory[PROGRAM_START_INDEX + 0x100] = 0b11110001;

    emulator.i = PROGRAM_START_ADDRESS + 0x100;

    emulator.memory[PROGRAM_START_INDEX] = 0xD1;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x21;

    emulator.execute();

    assert_eq!(
        emulator.display[0x10][0x20..0x28],
        [true, true, true, true, false, false, false, true]
    );
}

#[test]
fn execute_dxyn_clips_sprite_pixels_past_display_bounds() {
    let mut emulator = Emulator::new();

    emulator.v[0xA] = 0x3C; // x coordinate starts at pixel 60
    emulator.v[0xB] = 0x1F; // y coordinate starts at row 31

    emulator.memory[PROGRAM_START_INDEX + 0x100] = 0b11111111;
    emulator.memory[PROGRAM_START_INDEX + 0x101] = 0b11111111;

    emulator.i = PROGRAM_START_ADDRESS + 0x100;

    emulator.memory[PROGRAM_START_INDEX] = 0xDA;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0xB2;

    emulator.execute();

    assert_eq!(emulator.display[0x1F][0x3C..0x40], [true, true, true, true]);
    assert_eq!(
        emulator.display[0x00][0x3C..0x40],
        [false, false, false, false]
    );
    assert_eq!(
        emulator.display[0x1F][0x00..0x04],
        [false, false, false, false]
    );
    assert_eq!(
        emulator.display[0x00][0x00..0x04],
        [false, false, false, false]
    );
}

#[test]
#[should_panic]
fn execute_dxyn_past_memory_bounds_panics() {
    let mut emulator = Emulator::new();

    emulator.v[0x1] = 0x04; // x coordinate
    emulator.v[0x2] = 0x02; // y coordinate

    emulator.memory[PROGRAM_START_INDEX + 0x100] = 0b01010101;
    emulator.memory[PROGRAM_START_INDEX + 0x101] = 0b11110000;

    emulator.i = MEMORY_SIZE as u16 - 1;

    emulator.memory[PROGRAM_START_INDEX] = 0xD1;
    emulator.memory[PROGRAM_START_INDEX + 1] = 0x22;

    emulator.execute();
}
