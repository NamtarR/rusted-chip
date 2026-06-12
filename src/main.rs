use std::{env, fs};

mod chip8;
mod runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut runtime = runtime::Runtime::new();

    let bytes = fs::read(path).unwrap();

    runtime.load(&bytes);
    runtime.run_steps(24);

    let result = runtime.display();

    for row in result.iter() {
        for pixel in row.iter() {
            if *pixel { print!("##") } else { print!("  ") }
        }
        println!()
    }
}
