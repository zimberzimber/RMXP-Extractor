#![warn(clippy::unwrap_used, clippy::pedantic, rust_2018_idioms)]

extern crate rpgtool_common as common;

use clap::{CommandFactory, Parser, error::ErrorKind};
use common::Format;
use std::path::PathBuf;

// TODO stdin?
/// Converts Ruby marshal files to other formats, and vice versa.
#[derive(Parser)]
struct Cli {
    #[arg(long, exclusive = true)]
    completions: Option<clap_complete::Shell>,
    /// The source file.
    #[arg(value_hint = clap_complete::ValueHint::FilePath, required_unless_present="completions")]
    src: Option<PathBuf>,
    /// The destination file.
    #[arg(value_hint = clap_complete::ValueHint::FilePath, required_unless_present="completions")]
    dest: Option<PathBuf>,
    /// The formats to convert from/to.
    ///
    /// Input comes first.
    ///
    /// Required if the format cannot be determined via file extensions.
    #[arg(long, visible_short_alias = 'f', number_of_values = 2)]
    format: Option<Vec<Format>>,
}

fn main() {
    let Cli {
        completions,
        src,
        dest,
        format,
    } = Cli::parse();

    if let Some(shell) = completions {
        let mut cmd = Cli::command();
        let name = cmd.get_name().to_owned();
        clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
        return;
    }

    let src = src.expect("should be present");
    let dest = dest.expect("should be present");

    let [from, to] = match format.as_deref() {
        Some(&[from, to]) => [from, to],
        None => {
            let Some((from, to)) = Format::guess(&src).zip(Format::guess(&dest)) else {
                let mut command = Cli::command();
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
    let value: common::Value = match common::conv_read(from, input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to parse {}: {e}", src.display());
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
    if let Err(e) = common::conv_write(value, to, output) {
        eprintln!("failed to convert {}: {e}", src.display());
    }
}
