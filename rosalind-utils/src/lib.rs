//! Shared utilities for Rust and Python solutions.
//!
//! To add a function, write it inside the `utils!` block at the bottom of this
//! file. It is then available from both languages with no other changes.
//! Its parameter and return types must be ones PyO3 can convert, such as
//! `&str`, `String`, numbers, `Vec`, `HashMap`, `Option` and tuples.
//!
//! Helpers that should be Rust-only go outside the block.

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
        fn _core(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
            use pyo3::types::PyModuleMethods;
            $(m.add_function(pyo3::wrap_pyfunction!($name, m)?)?;)*
            Ok(())
        }
    };
}

utils! {
    /// Fraction of bases in `seq` that are G or C.
    pub fn gc_content(seq: &str) -> f64 {
        let gc = seq.bytes().filter(|b| matches!(b, b'G' | b'C')).count();
        gc as f64 / seq.len() as f64
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
