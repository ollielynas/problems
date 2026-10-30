//! Finding a Protein Motif
use std::fs;

/// Parse FASTA into (id, sequence) pairs, in file order.
#[allow(dead_code)]
fn fasta(text: &str) -> Vec<(String, String)> {
    text.split('>')
        .skip(1)
        .map(|block| {
            let mut lines = block.lines();
            let name = lines.next().unwrap_or("").trim().to_string();
            (name, lines.map(str::trim).collect())
        })
        .collect()
}

fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let text = fs::read_to_string(format!("{dir}/input.txt")).expect("could not read input.txt");

    let answer = text;
    println!("{answer}");
    fs::write(format!("{dir}/output.txt"), format!("{answer}\n")).expect("could not write output.txt");
}
