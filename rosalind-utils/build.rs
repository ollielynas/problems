//! Regenerates `src/rosalind_utils/_core.pyi` on Python builds, so editors know
//! the signatures of the functions in the `utils!` block.
use std::{env, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    // Skip Rust-only builds, and stop maturin's own build from running this again.
    if env::var_os("CARGO_FEATURE_PYTHON").is_none() || env::var_os("ROSALIND_STUBS").is_some() {
        return;
    }

    let status = Command::new("uvx")
        .args(["maturin", "generate-stubs", "--out", "src", "-q"])
        .env("ROSALIND_STUBS", "1")
        // A separate target directory avoids waiting on the outer build's lock.
        .env("CARGO_TARGET_DIR", "target/stubs")
        .status();

    // Stubs only help the editor, so a failure here should not stop the build.
    if !status.is_ok_and(|s| s.success()) {
        println!("cargo:warning=could not regenerate _core.pyi, so it may be out of date");
    }
}
