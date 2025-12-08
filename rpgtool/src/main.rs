#![warn(clippy::unwrap_used)]

use clap::Parser;
use common::Format;
use std::path::PathBuf;

mod conv;

/// Utility for working with RPG Maker XP - VX Ace projects.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Convert an entire project to another data format.
    Convert(ConvArgs),
    /// Unpack a scripts file into a directory.
    Unpack(ScriptArgs),
    /// Pack a scripts directory back into a file.
    Pack(ScriptArgs),
}

#[derive(clap::Args)]
struct ConvArgs {
    /// The source directory.
    src: PathBuf,
    /// The destination directory.
    dest: PathBuf,
    /// The formats to convert from/to.
    ///
    /// Input comes first.
    ///
    /// Required if the format cannot be determined via file extensions.
    #[arg(long, visible_short_alias = 'f', number_of_values = 2)]
    format: Option<Vec<Format>>,
    /// Exit on error instead of ignoring it.
    #[arg(long = "fail_fast")]
    fail_on_error: bool,
    #[arg(long)]
    /// The file extension every *input* file uses.
    input_file_ext: Option<PathBuf>,
    #[arg(long)]
    /// The file extension every *output* file uses.
    output_file_ext: Option<PathBuf>,
}

#[derive(clap::Args)]
struct ScriptArgs {
    /// The directory containing script files.
    directory: PathBuf,
    /// The packed script file.
    file: PathBuf,
}

#[allow(unused)]
fn main() {
    match Cli::parse().command {
        Commands::Convert(conv_args) => conv::convert(conv_args),
        Commands::Unpack(script_args) => todo!(),
        Commands::Pack(script_args) => todo!(),
    }
}
