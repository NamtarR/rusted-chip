use super::font::{self, FONT_CHARACTER_SIZE};

const MEMORY_SIZE: usize = 4096;

const PROGRAM_START_ADDRESS: u16 = 0x200;
const PROGRAM_START_INDEX: usize = PROGRAM_START_ADDRESS as usize;

const MAX_ROM_SIZE: usize = MEMORY_SIZE - PROGRAM_START_INDEX;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const INPUT_KEYS_COUNT: usize = 16;

const FONT_START_ADDRESS: u16 = 0x050;
const FONT_START_INDEX: usize = FONT_START_ADDRESS as usize;
const FONT_END_INDEX: usize = 0x09F;

pub type Display = [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

/*
 * access pixel with inverted coords -> display[y][x]
 * because we have an array of 32 rows of 64 pixels
 * so we specify y coordinate first selecting a row
 * then specify x coordinate selecting a pixel in that row
 */
pub struct Emulator {
    v: [u8; REGISTER_COUNT],
    i: u16,
    pc: u16,
    memory: [u8; MEMORY_SIZE],
    stack: [u16; STACK_SIZE],
    stack_pointer: usize,
    display: Display,
    input: [bool; INPUT_KEYS_COUNT],
    delay_timer: u8,
    sound_timer: u8,
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
        let opcode = instruction & 0xF000;

        // Decode the rest of the instruction
        let x = usize::from((instruction & 0x0F00) >> 8); // more convenient as usize to be used in self.v[x]
        let y = usize::from((instruction & 0x00F0) >> 4); // more convenient as usize to be used in self.v[y]
        let n = (instruction & 0x000F) as u8;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        // Match and execute the instruction
        match opcode {
            0x0000 => match instruction {
                0x00E0 => self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT], // clear display
                0x00EE => {
                    assert!(
                        self.stack_pointer != 0,
                        "Cannot return from subroutine: stack is empty"
                    );

                    self.stack_pointer -= 1;
                    self.pc = self.stack[self.stack_pointer];
                    self.stack[self.stack_pointer] = 0;
                }

                _ => panic!("Unknown instruction"),
            },
            0x1000 => self.pc = nnn, // set PC to NNN
            0x2000 => {
                assert!(
                    self.stack_pointer < self.stack.len(),
                    "Cannot call subroutine: stack overflow"
                );

                self.stack[self.stack_pointer] = self.pc;
                self.pc = nnn;
                self.stack_pointer += 1;
            }
            0x3000 => {
                if self.v[x] == nn {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.v[x] != nn {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if n != 0 {
                    panic!("Unknown instruction")
                }
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            0x6000 => self.v[x] = nn, // set VX to NN
            0x7000 => self.v[x] = self.v[x].wrapping_add(nn), // add NN to VX (wraps on overflow)
            0x8000 => match n {
                0x0 => self.v[x] = self.v[y],
                0x1 => self.v[x] = self.v[x] | self.v[y],
                0x2 => self.v[x] = self.v[x] & self.v[y],
                0x3 => self.v[x] = self.v[x] ^ self.v[y],
                0x4 => {
                    let (result, overflow) = self.v[x].overflowing_add(self.v[y]);
                    self.v[x] = result;
                    self.v[0xF] = u8::from(overflow);
                }
                0x5 => {
                    let (result, overflow) = self.v[x].overflowing_sub(self.v[y]);
                    self.v[x] = result;
                    self.v[0xF] = u8::from(overflow) ^ 1;
                }
                0x6 => {
                    // Currently, implemented as shift in place. Will be configurable later.
                    self.v[0xF] = self.v[x] & 0b00000001;
                    self.v[x] >>= 1;
                }
                0x7 => {
                    let (result, overflow) = self.v[y].overflowing_sub(self.v[x]);
                    self.v[x] = result;
                    self.v[0xF] = u8::from(overflow) ^ 1;
                }
                0xE => {
                    // Currently, implemented as shift in place. Will be configurable later.
                    self.v[0xF] = (self.v[x] & 0b10000000) >> 7;
                    self.v[x] <<= 1;
                }
                _ => panic!("Unknown instruction"),
            },
            0x9000 => {
                if n != 0 {
                    panic!("Unknown instruction")
                }
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0xA000 => self.i = nnn, // set I to NNN
            // Currently, implemented as NNN + V0. Will be configurable later.
            0xB000 => self.pc = nnn + u16::from(self.v[0]),
            0xC000 => self.v[x] = rand::random_range(0..=0xFF) & nn,
            0xD000 => {
                assert!(
                    usize::from(self.i + u16::from(n)) <= MEMORY_SIZE,
                    "Cannot fetch sprite pixels from beyond available memory"
                );

                let x_start = usize::from(self.v[x]) % DISPLAY_WIDTH;
                let y_start = usize::from(self.v[y]) % DISPLAY_HEIGHT;
                let x_end = std::cmp::min(DISPLAY_WIDTH, x_start + 8);
                let y_end = std::cmp::min(DISPLAY_HEIGHT, y_start + usize::from(n));

                let mut overflow = false;

                for (row_index, row) in self.display[y_start..y_end].iter_mut().enumerate() {
                    let sprite_pixels = self.memory[usize::from(self.i) + row_index];
                    for (pixel_index, pixel) in row[x_start..x_end].iter_mut().enumerate() {
                        let display_pixel = *pixel;
                        let sprite_pixel = (sprite_pixels & (0b10000000 >> pixel_index)) != 0;

                        *pixel = display_pixel ^ sprite_pixel;

                        if display_pixel && sprite_pixel {
                            overflow = true;
                        }
                    }
                }

                self.v[0xF] = if overflow { 1 } else { 0 };
            }
            0xE000 => match nn {
                0x9E => {
                    assert!(self.v[x] <= 0xF, "Only keys from 0 to F are supported");

                    if self.input[usize::from(self.v[x])] {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    assert!(self.v[x] <= 0xF, "Only keys from 0 to F are supported");

                    if !self.input[usize::from(self.v[x])] {
                        self.pc += 2;
                    }
                }

                _ => panic!("Unknown instruction"),
            },
            0xF000 => match nn {
                0x0A => match self.input.iter().position(|key| *key) {
                    Some(key) => {
                        self.v[x] = key as u8;
                    }
                    None => {
                        self.pc -= 2;
                    }
                },
                0x07 => self.v[x] = self.delay_timer,
                0x15 => self.delay_timer = self.v[x],
                0x18 => self.sound_timer = self.v[x],
                0x1e => {
                    self.i = self.i + u16::from(self.v[x]);
                    self.v[0xF] = (self.i >> 12) as u8;
                }
                0x29 => {
                    // The original COSMAC VIP interpreter just took the last nibble of VX
                    // and used that as the character.
                    self.i = FONT_START_ADDRESS
                        + u16::from(self.v[x] & 0x0F) * FONT_CHARACTER_SIZE as u16
                }
                0x33 => {
                    assert!(
                        usize::from(self.i) <= MEMORY_SIZE - 3,
                        "Cannot write to memory beyond bounds"
                    );

                    let mut value = self.v[x];

                    for i in 0..3 {
                        self.memory[usize::from(self.i) + 2 - i] = value % 10;
                        value = value / 10;
                    }
                }
                0x55 => {
                    assert!(
                        usize::from(self.i) <= MEMORY_SIZE - x - 1,
                        "Cannot write to memory beyond bounds"
                    );

                    // In this case, I does not change. Will be configurable later.
                    for i in 0..x + 1 {
                        self.memory[usize::from(self.i) + i] = self.v[i];
                    }
                }
                0x65 => {
                    assert!(
                        usize::from(self.i) <= MEMORY_SIZE - x - 1,
                        "Cannot read into memory beyond bounds"
                    );

                    for i in 0..x + 1 {
                        self.v[i] = self.memory[usize::from(self.i) + i as usize];
                    }
                }

                _ => panic!("Unknown instruction"),
            },

            _ => panic!("Unknown instruction"),
        };
    }

    pub fn tick_timers(&mut self, decrement: u8) {
        self.delay_timer = self.delay_timer.saturating_sub(decrement);
        self.sound_timer = self.sound_timer.saturating_sub(decrement);
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn new() -> Emulator {
        let mut emulator = Emulator {
            v: [0; REGISTER_COUNT],
            i: 0,
            pc: PROGRAM_START_ADDRESS,
            memory: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            input: [false; INPUT_KEYS_COUNT],
            delay_timer: 0,
            sound_timer: 0,
        };

        emulator.load_font();

        emulator
    }

    fn load_font(&mut self) {
        let font_memory = &mut self.memory[FONT_START_INDEX..FONT_END_INDEX + 1];

        font_memory.copy_from_slice(&font::FONT);
    }
}

#[cfg(test)]
#[path = "emulator_tests.rs"]
mod tests;
