use std::time::Duration;

use super::beeper::Beeper;
use crate::chip8;

const INSTRUCTIONS_PER_SECOND: f64 = 700.0;
const TIMER_PER_SECOND: f64 = 60.0;

pub struct Runner {
    emulator: chip8::Emulator,
    beeper: Beeper,
    instruction_accumulator: f64,
    timer_accumulator: f64,
}

impl Runner {
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

        if self.emulator.sound_timer() > 0 {
            self.beeper.play();
        }

        if timers_to_tick != 0 {
            self.emulator.tick_timers(timers_to_tick);
        }

        if self.emulator.sound_timer() == 0 {
            self.beeper.pause();
        }
    }

    pub fn run_steps(&mut self, steps: u8) {
        for _ in 0..steps {
            self.emulator.execute();
        }
    }

    pub fn step(&mut self) {
        self.run_steps(1);
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) {
        self.emulator.set_key(key, pressed);
    }

    pub fn clear_keys(&mut self) {
        self.emulator.clear_keys();
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

    pub fn new() -> Runner {
        Runner {
            emulator: chip8::Emulator::new(),
            beeper: Beeper::new(),
            instruction_accumulator: 0.0,
            timer_accumulator: 0.0,
        }
    }
}

#[cfg(test)]
mod runner_tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    const ACCUMULATOR_EPSILON: f64 = 0.00000001;

    #[test]
    fn reset_clears_accumulators() {
        let mut runtime = Runner::new();

        runtime.instruction_accumulator = 0.5;
        runtime.timer_accumulator = 0.22232425;

        runtime.reset();

        assert_abs_diff_eq!(
            runtime.instruction_accumulator,
            0.0,
            epsilon = ACCUMULATOR_EPSILON
        );
        assert_abs_diff_eq!(
            runtime.timer_accumulator,
            0.0,
            epsilon = ACCUMULATOR_EPSILON
        );
    }

    #[test]
    fn take_due_instructions_calculates_instructions_to_execute() {
        let mut runtime = Runner::new();

        let instructions = runtime.take_due_instructions(Duration::from_millis(10));

        assert_eq!(instructions, 7);
        assert_abs_diff_eq!(
            runtime.instruction_accumulator,
            0.0,
            epsilon = ACCUMULATOR_EPSILON
        );
    }

    #[test]
    fn take_due_instructions_adds_to_accumulator_if_not_integer() {
        let mut runtime = Runner::new();

        runtime.instruction_accumulator = 0.9;

        let instructions = runtime.take_due_instructions(Duration::from_millis(8));

        assert_eq!(instructions, 6);
        assert_abs_diff_eq!(
            runtime.instruction_accumulator,
            0.5,
            epsilon = ACCUMULATOR_EPSILON
        );
    }

    #[test]
    fn take_due_instructions_accumulates_across_calls() {
        let mut runtime = Runner::new();

        let frame1_instructions = runtime.take_due_instructions(Duration::from_millis(1));
        let frame2_instructions = runtime.take_due_instructions(Duration::from_millis(1));

        assert_eq!(frame1_instructions, 0);
        assert_eq!(frame2_instructions, 1);
        assert_abs_diff_eq!(
            runtime.instruction_accumulator,
            0.4,
            epsilon = ACCUMULATOR_EPSILON
        );
    }

    #[test]
    fn take_due_timer_ticks_calculates_timers_decrements() {
        let mut runtime = Runner::new();

        let timer_ticks = runtime.take_due_timer_ticks(Duration::from_millis(12));

        assert_eq!(timer_ticks, 0);
        assert_abs_diff_eq!(
            runtime.timer_accumulator,
            0.72,
            epsilon = ACCUMULATOR_EPSILON
        );
    }

    #[test]
    fn take_due_timer_ticks_adds_to_accumulator_if_not_integer() {
        let mut runtime = Runner::new();

        runtime.timer_accumulator = 0.72;

        let timer_ticks = runtime.take_due_timer_ticks(Duration::from_millis(8));

        assert_eq!(timer_ticks, 1);
        assert_abs_diff_eq!(
            runtime.timer_accumulator,
            0.2,
            epsilon = ACCUMULATOR_EPSILON
        );
    }

    #[test]
    fn take_due_timer_ticks_accumulates_across_calls() {
        let mut runtime = Runner::new();

        let frame1_ticks = runtime.take_due_timer_ticks(Duration::from_millis(10));
        let frame2_ticks = runtime.take_due_timer_ticks(Duration::from_millis(10));

        assert_eq!(frame1_ticks, 0);
        assert_eq!(frame2_ticks, 1);
        assert_abs_diff_eq!(
            runtime.timer_accumulator,
            0.2,
            epsilon = ACCUMULATOR_EPSILON
        );
    }

    #[test]
    fn take_due_timer_ticks_clamps_to_u8_max() {
        let mut runtime = Runner::new();

        let timer_ticks = runtime.take_due_timer_ticks(Duration::from_secs(10));

        assert_eq!(timer_ticks, u8::MAX);
        assert_abs_diff_eq!(
            runtime.timer_accumulator,
            0.0,
            epsilon = ACCUMULATOR_EPSILON
        );
    }
}
