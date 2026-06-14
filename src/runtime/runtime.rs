use crate::chip8;

pub struct Runtime {
    emulator: chip8::Emulator,
}

impl Runtime {
    pub fn load(&mut self, bytes: &[u8]) {
        self.emulator.load(bytes);
    }

    pub fn step(&mut self) {
        self.emulator.execute();
    }

    pub fn run_steps(&mut self, steps: u8) {
        for _ in 0..steps {
            self.step();
        }
    }

    pub fn display(&self) -> &chip8::Display {
        self.emulator.display()
    }

    pub fn new() -> Runtime {
        Runtime {
            emulator: chip8::Emulator::new(),
        }
    }
}
