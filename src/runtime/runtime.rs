use std::time::Duration;

use crate::chip8;

const INSTRUCTIONS_PER_SECOND: f64 = 700.0;
const TIMER_PER_SECOND: f64 = 60.0;

pub struct Runtime {
    emulator: chip8::Emulator,
    instruction_accumulator: f64,
    timer_accumulator: f64,
}

impl Runtime {
    pub fn load(&mut self, bytes: &[u8]) {
        self.reset();
        self.emulator.load(bytes);
    }

    pub fn reset(&mut self) {
        self.instruction_accumulator = 0.0;
        self.timer_accumulator = 0.0;
        self.emulator.reset()
    }

    pub fn display(&self) -> &chip8::Display {
        self.emulator.display()
    }

    pub fn run(&mut self, frame_time: Duration) {
        let instructions_to_execute = self.take_due_instructions(frame_time);
        let timers_to_tick = self.take_due_timer_ticks(frame_time);

        for _ in 0..instructions_to_execute {
            self.emulator.execute();
        }

        if timers_to_tick != 0 {
            self.emulator.tick_timers(timers_to_tick);
        }
    }

    /*
     * Here usize seems appropriate as even u16 overflows at frame_time > ~94 seconds
     */
    fn take_due_instructions(&mut self, frame_time: Duration) -> usize {
        let total_instructions =
            self.instruction_accumulator + frame_time.as_secs_f64() * INSTRUCTIONS_PER_SECOND;
        let instructions_to_execute = total_instructions.floor() as usize;

        self.instruction_accumulator = total_instructions - instructions_to_execute as f64;

        instructions_to_execute
    }

    /*
     * Clamp to u8::MAX as any value larger than that would have the same effect on emulator timers.
     */
    fn take_due_timer_ticks(&mut self, frame_time: Duration) -> u8 {
        let total_timer_ticks =
            self.timer_accumulator + frame_time.as_secs_f64() * TIMER_PER_SECOND;
        let ticks_to_decrement = total_timer_ticks.floor() as usize;

        self.timer_accumulator = total_timer_ticks - ticks_to_decrement as f64;

        u8::try_from(ticks_to_decrement).unwrap_or(u8::MAX)
    }

    pub fn step(&mut self) {
        self.emulator.execute();
    }

    pub fn run_steps(&mut self, steps: u8) {
        for _ in 0..steps {
            self.step();
        }
    }

    pub fn new() -> Runtime {
        Runtime {
            emulator: chip8::Emulator::new(),
            instruction_accumulator: 0.0,
            timer_accumulator: 0.0,
        }
    }
}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
