use std::io::{self, Write};

#[unsafe(no_mangle)]
pub extern "C" fn run() {
    println!("Hello from WASM!");
    let _ = io::stdout().flush(); // Explicitly flush stdout
}