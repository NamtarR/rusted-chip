const MEMORY_SIZE: usize = 4096;
const PROGRAM_START_INDEX: usize = 0x200;
const PROGRAM_START_ADDRESS: u16 = 0x200;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const MAX_ROM_SIZE: usize = MEMORY_SIZE - PROGRAM_START_INDEX;

pub struct Emulator {
    v: [u8; REGISTER_COUNT],
    i: u16,
    pc: u16,
    memory: [u8; MEMORY_SIZE],
    stack: [u16; STACK_SIZE],
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Emulator {
    // TODO: Maybe switch to Result later instead of panicking right away
    pub fn load(&mut self, bytes: &[u8]) {
        assert!(
            bytes.len() <= MAX_ROM_SIZE,
            "Cannot load a ROM larger than {0} bytes long",
            MAX_ROM_SIZE
        );

        let rom = &mut self.memory[PROGRAM_START_INDEX..PROGRAM_START_INDEX + bytes.len()];
        rom.copy_from_slice(bytes);

        self.pc = PROGRAM_START_ADDRESS;
    }

    pub fn execute(&mut self) {
        let pc = usize::from(self.pc);
        assert!(
            pc < MEMORY_SIZE - 1,
            "Cannot fetch instruction from beyond available memory"
        );

        // Fetch two consecutive u8 from memory at pc
        let byte1 = u16::from(self.memory[pc]);
        let byte2 = u16::from(self.memory[pc + 1]);

        // Combine these two bytes into a u16
        let instruction = (byte1 << 8) | byte2;

        // Increase pc by 2
        self.pc += 2;

        // Mask the first 4 bits and match with instructions
        /*
         * First set of instructions to implement
         * 00E0 (clear screen)
         * 1NNN (jump)
         * 6XNN (set register VX)
         * 7XNN (add value to register VX)
         * ANNN (set index register I)
         * DXYN (display/draw)
         */

        let opcode = instruction & 0xF000;
        let x = usize::from((instruction & 0x0F00) >> 8); // more convenient as usize to be used in self.v[x]
        let y = usize::from((instruction & 0x00F0) >> 4); // more convenient as usize to be used in self.v[y]
        let n = instruction & 0x000F;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        match opcode {
            0x0000 => match instruction {
                0x00E0 => self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT], // clear display

                _ => panic!("Unknown instruction"),
            },
            0x1000 => self.pc = nnn,  // set PC to NNN
            0x6000 => self.v[x] = nn, // set VX to NN
            0x7000 => self.v[x] = self.v[x].wrapping_add(nn), // add NN to VX (wraps on overflow)
            0xA000 => self.i = nnn,   // set I to NNN
            0xD000 => println!("Draw"),

            _ => panic!("Unknown instruction"),
        };

        // Decode the rest of the instruction
        // Execute the instruction
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn new() -> Emulator {
        Emulator {
            v: [0; REGISTER_COUNT],
            i: 0,
            pc: PROGRAM_START_ADDRESS,
            memory: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn load_rejects_rom_larger_than_memory() {
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
        emulator.display = [[true; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        emulator.reset();

        assert_eq!(emulator.v, [0; REGISTER_COUNT]);
        assert_eq!(emulator.i, 0);
        assert_eq!(emulator.pc, PROGRAM_START_ADDRESS);
        assert_eq!(emulator.memory, [0; MEMORY_SIZE]);
        assert_eq!(emulator.stack, [0; STACK_SIZE]);
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
    fn execute_1nnn_jumps_to_address() {
        let mut emulator = Emulator::new();

        emulator.memory[PROGRAM_START_INDEX] = 0x1F;
        emulator.memory[PROGRAM_START_INDEX + 1] = 0xED;

        emulator.execute();

        assert_eq!(emulator.pc, 0xFED);
    }

    #[test]
    fn execute_6xnn_sets_vx_to_nn() {
        let mut emulator = Emulator::new();

        emulator.memory[PROGRAM_START_INDEX] = 0x63;
        emulator.memory[PROGRAM_START_INDEX + 1] = 0xFE;

        emulator.execute();

        assert_eq!(emulator.v[3], 0xFE);
    }

    #[test]
    fn execute_7xnn_adds_nn_to_vx() {
        let mut emulator = Emulator::new();

        emulator.v[2] = 0x15;
        emulator.memory[PROGRAM_START_INDEX] = 0x72;
        emulator.memory[PROGRAM_START_INDEX + 1] = 0x44;

        emulator.execute();

        assert_eq!(emulator.v[2], 0x59);
    }

    #[test]
    fn execute_7xnn_wraps_on_overflow() {
        let mut emulator = Emulator::new();

        emulator.v[2] = 0xF0;
        emulator.memory[PROGRAM_START_INDEX] = 0x72;
        emulator.memory[PROGRAM_START_INDEX + 1] = 0x20;

        emulator.execute();

        assert_eq!(emulator.v[2], 0x10);
    }

    #[test]
    fn execute_annn_sets_i_to_nnn() {
        let mut emulator = Emulator::new();

        emulator.memory[PROGRAM_START_INDEX] = 0xA1;
        emulator.memory[PROGRAM_START_INDEX + 1] = 0x23;

        emulator.execute();

        assert_eq!(emulator.i, 0x123);
    }
}
