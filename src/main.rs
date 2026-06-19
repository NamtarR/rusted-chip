use std::{env, fs};

mod chip8;
mod runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut runner = runtime::Runner::new();

    let bytes = fs::read(path).unwrap();

    runner.load(&bytes);
    runner.run_steps(255);

    let result = runner.display();

    for row in result.iter() {
        for pixel in row.iter() {
            if *pixel { print!("##") } else { print!("  ") }
        }
        println!()
    }
}
