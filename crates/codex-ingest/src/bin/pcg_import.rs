//! `pcg_import <file.pcg> --out <character.json> [--data-root <dir>]`
//!
//! Converts a PCGen character to the JSON the app's `import_character` reads, with the import
//! report inside it. `--data-root` is the directory holding `data/`: a packaged app passes its
//! resource directory, and with none the build checkout is read, which is right for a dev run.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use codex_ingest::pcg_import;

fn parse_args(args: &[String]) -> Result<(PathBuf, PathBuf, Option<PathBuf>), String> {
    let usage = "usage: pcg_import <file.pcg> --out <character.json> [--data-root <dir>]";
    let (mut input, mut out, mut data_root) = (None, None, None);
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--out" => out = Some(PathBuf::from(rest.next().ok_or(usage)?)),
            "--data-root" => data_root = Some(PathBuf::from(rest.next().ok_or(usage)?)),
            _ if input.is_none() && !arg.starts_with("--") => input = Some(PathBuf::from(arg)),
            _ => return Err(usage.to_owned()),
        }
    }
    Ok((input.ok_or(usage)?, out.ok_or(usage)?, data_root))
}

fn checkout_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (input, out, data_root) = match parse_args(&args) {
        Ok(parsed) => parsed,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };
    let root = match data_root {
        Some(root) => {
            // The class records are read through the engine's own root.
            codex::set_data_root(root.clone());
            root
        }
        None => checkout_root(),
    };
    match pcg_import::run(&input, &out, &root) {
        Ok(file) => {
            println!(
                "{}: {} report lines, {} custom records, {} unknown keys",
                file.display_label,
                file.report.lines.len(),
                file.custom.feats.len() + file.custom.equipment.len() + file.custom.spells.len(),
                file.report.unknown_keys.len()
            );
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
