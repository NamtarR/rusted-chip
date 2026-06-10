pub struct Emulator {
    v: [u8; 16],
    i: u16,
    pc: u16,
    memory: [u8; 4096],
    stack: [u16; 16],
    display: [[bool; 64]; 32],
}

impl Emulator {
    // TODO: Maybe switch to Result later instead of panicking right away
    pub fn load(&mut self, bytes: &[u8]) {
        assert!(
            bytes.len() <= self.memory.len() - 0x200,
            "Cannot load a ROM larger than 3584 bytes long"
        );

        let rom = &mut self.memory[0x200..0x200 + bytes.len()];
        rom.copy_from_slice(bytes);

        self.pc = 0x200;
    }

    pub fn execute(&mut self) {}

    pub fn reset(&mut self) {}

    pub fn new() -> Emulator {
        Emulator {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            memory: [0; 4096],
            stack: [0; 16],
            display: [[false; 64]; 32],
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

        let memory = &emulator.memory[0x200..0x200 + 512];

        assert_eq!(&bytes[..], memory);
    }

    #[test]
    fn load_sets_pc_to_program_start() {
        let mut emulator = Emulator::new();
        let bytes: [u8; 16] = [1; 16];

        emulator.pc = 0x00;
        emulator.load(&bytes);

        assert_eq!(0x200, emulator.pc);
    }

    #[test]
    fn load_accepts_largest_valid_rom() {
        let mut emulator = Emulator::new();
        let bytes: [u8; 3584] = [1; 3584];

        emulator.load(&bytes);

        let memory = &emulator.memory[0x200..0x200 + 3584];

        assert_eq!(&bytes[..], memory);
    }

    #[test]
    #[should_panic]
    fn load_rejects_rom_larger_than_memory() {
        let mut emulator = Emulator::new();
        let bytes: [u8; 3585] = [1; 3585];

        emulator.load(&bytes);
    }
}
