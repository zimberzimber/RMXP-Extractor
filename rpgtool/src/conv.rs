use std::io::{Read, Write};

use alox_48::Value;
use clap::{CommandFactory, error::ErrorKind};
use serde::Serialize;

use common::{DeserializeValue, Format, SerializeValue};

use super::{Cli, ConvArgs};

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

    let mut read_dir = std::fs::read_dir(&src).unwrap();
    for entry in read_dir {
        let entry = entry.unwrap();
        let src_path = entry.path();
        // if not a file *or* the file extension does not match what it should, print warning and continue
        if !entry.file_type().unwrap().is_file()
            || src_path.extension().is_none_or(|ext| ext != input_file_ext)
        {
            eprintln!("[WARN]: Ignoring {}", src_path.display());
            continue;
        }

        let filename = src_path.file_name().unwrap();
        let dest_path = dest.join(filename).with_extension(output_file_ext);

        let input = std::fs::File::open(src_path).unwrap();
        let input = std::io::BufReader::new(input);
        let output = std::fs::File::create(dest_path).unwrap();
        let output = std::io::BufWriter::new(output);

        conv_file(from, to, input, output);
    }
}

fn conv_file<R, W>(from: Format, to: Format, mut input: R, mut output: W)
where
    R: Read,
    W: Write,
{
    let value: Value = match from {
        Format::Marshal => {
            let mut data = vec![];
            input.read_to_end(&mut data).unwrap();
            alox_48::from_bytes(&data).unwrap()
        }
        Format::Json => {
            let value: DeserializeValue = serde_json::from_reader(input).unwrap();
            value.0
        }
        Format::Ron => {
            let value: DeserializeValue = ron::Options::default().from_reader(input).unwrap();
            value.0
        }
        Format::Yaml => {
            let value: DeserializeValue = serde_yaml_ng::from_reader(input).unwrap();
            value.0
        }
    };

    match to {
        Format::Marshal => {
            let data = alox_48::to_bytes(value).unwrap();
            output.write_all(&data).unwrap();
        }
        Format::Json => {
            let mut ser = serde_json::Serializer::pretty(output);
            SerializeValue(&value).serialize(&mut ser).unwrap();
        }
        Format::Ron => {
            let config = ron::ser::PrettyConfig::default();
            ron::Options::default()
                .to_io_writer_pretty(output, &SerializeValue(&value), config)
                .unwrap();
        }
        Format::Yaml => {
            let mut ser = serde_yaml_ng::Serializer::new(output);
            SerializeValue(&value).serialize(&mut ser).unwrap();
        }
    }
}
