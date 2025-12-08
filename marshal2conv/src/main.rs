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

    let input = std::fs::File::open(src).unwrap();
    let output = std::fs::File::create(dest).unwrap();
    common::conv_io(from, to, input, output);
}
