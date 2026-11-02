//! Shared utilities for Rust and Python solutions.
//!
//! To add a function, write it inside the `utils!` block at the bottom of this
//! file. It is then available from both languages with no other changes.
//! Its parameter and return types must be ones PyO3 can convert, such as
//! `&str`, `String`, numbers, `Vec`, `HashMap`, `Option` and tuples.
//!
//! Helpers that should be Rust-only go outside the block.
//!
//! `read_input` and `write_output` are the exception: they need the solution's
//! folder, so they are macros here and plain functions in `rosalind_utils/__init__.py`.
#![feature(portable_simd)]

pub mod simd_string;

use simd_string::{find_proteins_u8, reverse_complement_u8, translate_u8};
use std::{fmt::Display, fs};

/// The contents of the solution's `input.txt`: `let text = read_input!();`
#[macro_export]
macro_rules! read_input {
    () => {
        $crate::read_input_from(env!("CARGO_MANIFEST_DIR"))
    };
}

/// Prints the answer and saves it to the solution's `output.txt`: `write_output!(answer);`
#[macro_export]
macro_rules! write_output {
    ($answer:expr) => {
        $crate::write_output_to(env!("CARGO_MANIFEST_DIR"), &$answer)
    };
}

#[doc(hidden)]
pub fn read_input_from(dir: &str) -> String {
    let path = format!("{dir}/input.txt");
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("could not read {path}: {e}"))
        .trim()
        .to_string()
}

#[doc(hidden)]
pub fn write_output_to(dir: &str, answer: &dyn Display) {
    println!("{answer}");
    let path = format!("{dir}/output.txt");
    fs::write(&path, format!("{answer}\n"))
        .unwrap_or_else(|e| panic!("could not write {path}: {e}"));
}

/// Exposes each function as normal Rust and registers it with the Python module.
macro_rules! utils {
    ($($(#[$attr:meta])* pub fn $name:ident $args:tt $(-> $ret:ty)? $body:block)*) => {
        $(
            $(#[$attr])*
            #[cfg_attr(feature = "python", pyo3::pyfunction)]
            pub fn $name $args $(-> $ret)? $body
        )*

        #[cfg(feature = "python")]
        #[pyo3::pymodule]
        mod _core {
            $(#[pymodule_export] use super::$name;)*
        }
    };
}

utils! {
    /// Fraction of bases in `seq` that are G or C.
    pub fn gc_content(seq: &str) -> f64 {
        let gc = seq.bytes().filter(|b| matches!(b, b'G' | b'C')).count();
        gc as f64 / seq.len() as f64
    }


    /// Reverse complement of DNA or RNA. If `dna` contains a `U`, every `A` pairs with `U`.
    pub fn reverse_complement(dna: &str) -> String {
        assert!(dna.is_ascii());
        String::from_utf8(reverse_complement_u8(dna.as_bytes())).unwrap()
    }

    pub fn dna_to_protin(dna: &str) -> String {
        String::from_utf8(translate_u8(dna.as_bytes())).unwrap()
    }

    /// Distinct proteins from all six reading frames of `dna`, longest first.
    pub fn find_proteins(dna: &str) -> Vec<String> {
        find_proteins_u8(dna.as_bytes())
    }


    /// Parses FASTA text into `(id, sequence)` pairs.
    pub fn parse_fasta(text: &str) -> Vec<(String, String)> {
        let mut records: Vec<(String, String)> = Vec::new();
        for line in text.lines().map(str::trim) {
            if let Some(id) = line.strip_prefix('>') {
                records.push((id.to_string(), String::new()));
            } else if let Some((_, seq)) = records.last_mut() {
                seq.push_str(line);
            }
        }
        records
    }
}
