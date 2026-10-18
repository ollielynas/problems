//! Complementing a Strand of DNA
use std::fs;

fn main() {
    let text = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))
        .expect("could not read input.txt");
    println!("{text}");
}
