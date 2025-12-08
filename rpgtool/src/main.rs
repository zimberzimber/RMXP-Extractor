#![warn(clippy::unwrap_used)]

extern crate rpgtool_common as common;

use clap::Parser;
use common::Format;
use std::path::PathBuf;

mod conv;
mod pack;
mod script;
mod unpack;

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
    Unpack(UnpackScriptArgs),
    /// Pack a scripts directory back into a file.
    Pack(PackScriptArgs),
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
    #[arg(long = "fail-fast")]
    fail_on_error: bool,
    #[arg(long = "output-ext")]
    /// The file extension every input file uses.
    ///
    /// Optional, does not have to be specified.
    input_file_ext: Option<PathBuf>,
    #[arg(long = "input-ext")]
    /// The file extension every output file uses.
    ///
    // Optional, does not have to be specified.
    output_file_ext: Option<PathBuf>,
}

#[derive(clap::Args)]
struct UnpackScriptArgs {
    /// The packed script file.
    file: PathBuf,
    /// The directory containing script files.
    directory: PathBuf,
}

#[derive(clap::Args)]
struct PackScriptArgs {
    /// The packed script file.
    file: PathBuf,
    /// The directory containing script files.
    directory: PathBuf,
}

#[allow(unused)]
fn main() {
    match Cli::parse().command {
        Commands::Convert(conv_args) => conv::convert(conv_args),
        Commands::Pack(script_args) => pack::pack(script_args),
        Commands::Unpack(script_args) => unpack::unpack(script_args),
    }
}
