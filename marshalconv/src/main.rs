#![warn(clippy::unwrap_used, clippy::pedantic, rust_2018_idioms)]

extern crate rpgtool_common as common;

use clap::{CommandFactory, Parser, error::ErrorKind};
use common::Format;
use std::path::PathBuf;

// TODO stdin?
/// Converts Ruby marshal files to other formats, and vice versa.
#[derive(Parser)]
struct Args {
    /// The source file.
    src: PathBuf,
    /// The destination file.
    dest: PathBuf,
    /// The formats to convert from/to.
    ///
    /// Input comes first.
    ///
    /// Required if the format cannot be determined via file extensions.
    #[arg(long, visible_short_alias = 'f', number_of_values = 2)]
    format: Option<Vec<Format>>,
}

fn main() {
    let Args { src, dest, format } = Args::parse();

    let [from, to] = match format.as_deref() {
        Some(&[from, to]) => [from, to],
        None => {
            let Some((from, to)) = Format::guess(&src).zip(Format::guess(&dest)) else {
                let mut command = Args::command();
                command
                    .error(
                        ErrorKind::DisplayHelp,
                        "unable to determine conversion formats, please specify with --format",
                    )
                    .exit()
            };
            [from, to]
        }
        _ => unreachable!(), // we enforce the number of values in clap
    };

    let input = match std::fs::File::open(&src) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("failed to open {}: {e}", src.display());
            return;
        }
    };
    let output = match std::fs::File::create(&dest) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("failed to open {}: {e}", dest.display());
            return;
        }
    };
    if let Err(e) = common::conv_io(from, to, input, output) {
        eprintln!("failed to convert: {e}");
    }
}
