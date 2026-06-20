use std::{env, fs};

mod chip8;
mod runtime;
mod tui;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let rom_bytes = fs::read(path).unwrap();

    let mut runner = runtime::Runner::new();

    runner.load(&rom_bytes);

    tui::App::start(&mut runner)
}
