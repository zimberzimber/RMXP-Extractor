use crate::{UnpackScriptArgs, structured::Script};
use std::io::Write as _;

pub fn unpack(args: UnpackScriptArgs) {
    let UnpackScriptArgs { directory, file } = args;

    // try and create the destination dir, and if it doesn't exist then error out
    if let Err(e) = std::fs::create_dir(&directory)
        && e.kind() != std::io::ErrorKind::AlreadyExists
    {
        eprintln!("failed to create destination directory: {e}");
        return;
    }

    let script_data = match std::fs::read(&file) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("failed to read {}: {e}", file.display());
            return;
        }
    };

    let scripts: Vec<Script> = match alox_48::from_bytes(&script_data) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("failed to decode scripts: {e}");
            return;
        }
    };

    let mut file = match std::fs::File::create(directory.join("_scripts.txt")) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("failed to create _scripts.txt: {e}");
            return;
        }
    };

    for script in scripts {
        if let Err(e) = writeln!(file, "{}", script.name) {
            eprintln!("failed to write to _scripts.txt: {e}");
            return;
        }

        let trimmed_name = script.name.trim();
        // continue if comment or script name is only whitespace/empty
        if trimmed_name.is_empty() || trimmed_name.starts_with('#') {
            continue;
        }

        let new_path = directory.join(trimmed_name).with_extension("rb");

        // handle the case where scripts are structured as folders
        if let Some(parent) = new_path.parent()
            && parent != directory
            && let Err(e) = std::fs::create_dir_all(parent)
        {
            eprintln!(
                "failed to create script directory at {}: {e}",
                parent.display()
            );
            return;
        }

        if let Err(e) = std::fs::write(&new_path, script.text) {
            eprintln!(
                "failed to write script {trimmed_name} to {}: {e}",
                new_path.display()
            );
        }
    }
}
