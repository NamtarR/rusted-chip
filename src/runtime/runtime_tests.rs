use approx::assert_abs_diff_eq;

use super::*;

const ACCUMULATOR_EPSILON: f64 = 0.00000001;

#[test]
fn reset_clears_accumulators() {
    let mut runtime = Runtime::new();

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
    let mut runtime = Runtime::new();

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
    let mut runtime = Runtime::new();

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
    let mut runtime = Runtime::new();

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
    let mut runtime = Runtime::new();

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
    let mut runtime = Runtime::new();

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
    let mut runtime = Runtime::new();

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
    let mut runtime = Runtime::new();

    let timer_ticks = runtime.take_due_timer_ticks(Duration::from_secs(10));

    assert_eq!(timer_ticks, u8::MAX);
    assert_abs_diff_eq!(
        runtime.timer_accumulator,
        0.0,
        epsilon = ACCUMULATOR_EPSILON
    );
}
