//! Overlap Graphs
use std::{collections::HashMap, fs};

/// Parse FASTA into (id, sequence) pairs, in file order.
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
    let input = fasta(&text);
    let k = 3;

    let mut starts: HashMap<&str, Vec<&str>> = HashMap::new();
    for (id, seq) in &input {
        if seq.len() >= k {
            starts.entry(&seq[..k]).or_default().push(id);
        }
    }

    let mut edges = Vec::new();
    for (id, seq) in &input {
        if seq.len() < k {
            continue;
        }
        let suffix = &seq[seq.len() - k..];
        if let Some(targets) = starts.get(suffix) {
            for &t in targets {
                if t != id {
                    edges.push(format!("{id} {t}"));
                }
            }
        }
    }

    let answer = edges.join("\n");
    println!("{answer}");
    fs::write(format!("{dir}/output.txt"), format!("{answer}\n"))
        .expect("could not write output.txt");
}
