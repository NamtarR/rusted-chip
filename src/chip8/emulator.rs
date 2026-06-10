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

    pub fn execute(&mut self) {}

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
}
