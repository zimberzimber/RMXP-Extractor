use clap::{CommandFactory, error::ErrorKind};
use common::Format;

use super::{Cli, ConvArgs};

macro_rules! yeet {
    ($fail_on_error:expr) => {
        if $fail_on_error {
            return;
        } else {
            continue;
        }
    };
}

#[allow(unused)]
pub fn convert(args: ConvArgs) {
    let ConvArgs {
        src,
        dest,
        format,
        fail_on_error,
        input_file_ext,
        output_file_ext,
    } = args;

    let [from, to] = match format.as_deref() {
        Some(&[from, to]) => [from, to],
        // try to guess format if not specified
        None => {
            let maybe_from = input_file_ext.as_deref().and_then(Format::guess_from_ext);
            let maybe_to = output_file_ext.as_deref().and_then(Format::guess_from_ext);
            let Some((from, to)) = maybe_from.zip(maybe_to) else {
                // we couldn't guess the format, so error out and exit
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

    let input_file_ext = input_file_ext.as_deref().unwrap_or(from.file_ext());
    let output_file_ext = output_file_ext.as_deref().unwrap_or(to.file_ext());

    // try and create the destination dir, and if it doesn't exist then error out
    if let Err(e) = std::fs::create_dir(&dest)
        && e.kind() != std::io::ErrorKind::AlreadyExists
    {
        eprintln!("failed to create destination directory: {e}");
        return;
    }

    let mut read_dir = match std::fs::read_dir(&src) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to read {}: {e}", src.display());
            return;
        }
    };

    for entry in read_dir {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("failed to read directory entry: {e}");
                yeet!(fail_on_error)
            }
        };

        let src_path = entry.path();
        // if not a file *or* the file extension does not match what it should, print warning and continue
        if !entry.file_type().expect("couldn't get file type").is_file()
            || src_path.extension().is_none_or(|ext| ext != input_file_ext)
        {
            eprintln!("[WARN]: Ignoring {}", src_path.display());
            continue;
        }

        let filename = src_path.file_name().expect("entry should have a file name");
        let dest_path = dest.join(filename).with_extension(output_file_ext);

        let input = match std::fs::File::open(&src_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("couldn't open {}", src_path.display());
                yeet!(fail_on_error)
            }
        };
        let input = std::io::BufReader::new(input);

        let output = match std::fs::File::open(&dest_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("couldn't open {}", dest_path.display());
                yeet!(fail_on_error)
            }
        };
        let output = std::io::BufWriter::new(output);

        if let Err(e) = common::conv_io(from, to, input, output) {
            eprintln!("failed to convert {}: {e}", src_path.display());
            yeet!(fail_on_error)
        }
    }
}
