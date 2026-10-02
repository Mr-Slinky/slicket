//! Generates test fixture for ticket data

use std::io;
use std::fs;
use std::path::{Path};


const PATH: &str = "../slicket-store/fixtures/happy_path_tickets.sql";

/// Main entry point to the file; only public API for the module
pub fn generate_tickets() -> io::Result<()> {
    write_tickets(&build_tickets())
}

pub fn build_tickets() -> String {
    // let mut output = String::new();
    //
    // output
    todo!()
}

fn write_tickets(output: &str) -> io::Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))   // .../crates/xtask
        .join(PATH);

    fs::write(path, output)
}