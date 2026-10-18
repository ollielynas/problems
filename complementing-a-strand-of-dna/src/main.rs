//! Complementing a Strand of DNA
use std::fs;

fn main() {
    let text = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))
        .expect("could not read input.txt");
    println!(
        "{}",
        text.chars()
            .rev()
            .map(|x| match x {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                a => a,
            })
            .collect::<String>()
    );
}
