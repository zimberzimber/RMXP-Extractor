use crate::{PackScriptArgs, script::Script};

pub fn pack(args: PackScriptArgs) {
    let PackScriptArgs { directory, file } = args;

    let mut scripts: Vec<Script> = vec![];

    let scripts_txt_path = directory.join("_scripts.txt");
    let scripts_txt = match std::fs::read_to_string(&scripts_txt_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("failed to read {}: {e}", scripts_txt_path.display());
            return;
        }
    };

    for name in scripts_txt.lines() {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() || trimmed_name.starts_with("#") {
            scripts.push(Script {
                name: name.to_owned(),
                text: String::new(),
            });
            continue;
        }

        let script_path = directory.join(trimmed_name).with_extension("rb");
        let script_text = match std::fs::read_to_string(&script_path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("failed to read {}: {e}", script_path.display());
                return;
            }
        };

        scripts.push(Script {
            name: name.to_owned(),
            text: script_text,
        });
    }

    let script_data = match alox_48::to_bytes(&scripts) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("failed to serialize scripts: {e}");
            return;
        }
    };

    if let Err(e) = std::fs::write(&file, script_data) {
        eprintln!("failed to write script data to {}: {e}", file.display())
    }
}
