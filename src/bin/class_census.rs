//! `class_census` -- the permanent, corpus-wide class-census CLI.
//!
//! SD-36 Epic F (`docs/release/SD-36-consolidation/epic-f-class-completion.md`
//! §2), batch F0, step F0b. F0a (`src/rules_core/class_census.rs`'s
//! `census()`) merged the nine raw registries into one map of every distinct
//! class id the engine knows about (135 today, per the partition
//! `every_registry_is_swept_once` measures: 31 tabled, 3 Ultimate Combat, 20
//! untabled exotic base, 7 CRB NPC / `Ex-*`, 74 Prestige). F0b is this
//! binary: it sweeps every id in that map that is **not** Prestige, alone,
//! `1..=max_level`, the exact same real fixed loadout and canonical
//! per-class seeds `v06_class_state_dump` already established (via
//! `codex::rules_core::class_seeds`), and reports each one's real
//! engine-derived `HeadlessReceiptStatus`.
//!
//! Prestige ids are listed but never swept here: a prestige class computed
//! *alone* is not a legitimate measurement (PF1's own prestige classes have
//! entry requirements only a carrier build can satisfy). Each prestige row
//! is reported `"status": "not_swept_yet"` and the carrier-mix sweep is a
//! later F0 step. Reporting a confidently-wrong Blocked/Computed verdict for
//! a class this binary has not actually swept would be worse than saying so
//! plainly.
//!
//! # Modes
//!
//! - `--json <path>`: sweep every non-prestige id, write the full document
//!   to `<path>`, and print `ids=<N> computed=<M> blocked=<K>` to stdout
//!   (`N` = the full merged census, `M` = non-prestige classes whose every
//!   swept level reached `Computed`, `K = N - M`).
//! - `--sheet-dump <class-id>:<level>`: print one class/level's full
//!   headless receipt (every explanation and every diagnostic) as stable,
//!   sorted, timestamp-free text to stdout and exit. `<class-id>` may be
//!   given with or without its `class:` prefix. Mutually exclusive with
//!   `--json` in one invocation -- when both are given, `--sheet-dump` wins
//!   and `--json` is ignored, the same "the narrower ask wins" precedent
//!   `v06_class_state_dump`'s own sibling binaries use.
//!
//! This binary is an operator/ops surface (like `v06_class_state_dump`),
//! not an app runtime surface -- nothing in the shipped app calls it.

use std::process::Command;

use codex::rules_core::class_census::{ClassCensusEntry, census, load_sweep_fixture, sheet_dump_text, sweep_non_prestige};

fn real_now_iso8601() -> String {
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("system `date` must be available to stamp generated_at");
    String::from_utf8(output.stdout)
        .expect("date output is valid UTF-8")
        .trim()
        .to_string()
}

/// Parse `--sheet-dump`'s `<class-id>:<level>` argument. Splits on the
/// LAST `:` because a class id itself already contains one
/// (`class:fighter`), so `class:fighter:5` means class `class:fighter` at
/// level 5, not the reverse.
fn parse_sheet_dump_arg(raw: &str) -> Result<(String, u8), String> {
    let (id_part, level_part) = raw
        .rsplit_once(':')
        .ok_or_else(|| format!("--sheet-dump expects <class-id>:<level>, got {raw:?}"))?;
    let level: u8 = level_part
        .parse()
        .map_err(|e| format!("--sheet-dump level {level_part:?} is not a valid u8: {e}"))?;
    let class_name = id_part.strip_prefix("class:").unwrap_or(id_part);
    if class_name.is_empty() {
        return Err(format!("--sheet-dump class id is empty in {raw:?}"));
    }
    Ok((class_name.to_owned(), level))
}

fn run_sheet_dump(raw_arg: &str) -> i32 {
    let (class_name, level) = match parse_sheet_dump_arg(raw_arg) {
        Ok(pair) => pair,
        Err(message) => {
            eprintln!("class_census: {message}");
            return 2;
        }
    };

    let fixture = match load_sweep_fixture() {
        Ok(fixture) => fixture,
        Err(message) => {
            eprintln!("class_census: {message}");
            return 1;
        }
    };

    print!("{}", sheet_dump_text(&fixture, &class_name, level));
    0
}

fn run_json(json_path: &str) -> i32 {
    let fixture = match load_sweep_fixture() {
        Ok(fixture) => fixture,
        Err(message) => {
            eprintln!("class_census: {message}");
            return 1;
        }
    };

    let entries = census();
    let ids = entries.len();

    // Silence the default panic printer for the duration of the sweep, the
    // same posture `v06_class_state_dump` takes: every panic is caught by
    // `sweep_class` and preserved verbatim in the class's own
    // `engine.panic` blocking diagnostic, and the default hook dumping
    // dozens of backtraces to stderr would bury the real signal.
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let results = sweep_non_prestige(&fixture, &entries);
    std::panic::set_hook(previous_hook);

    let computed = results.iter().filter(|r| r.computed()).count();
    let blocked = ids - computed;

    let classes: Vec<serde_json::Value> = results
        .iter()
        .map(|r| {
            serde_json::json!({
                "class_id": r.class_id,
                "family": r.family.label(),
                "books": r.books,
                "registries": r.registries,
                "max_level": r.max_level,
                "status": if r.computed() { "Computed" } else { "Blocked" },
                "levels_computed": r.levels_computed,
                "levels_blocked": r.levels_blocked,
                "blocking_diagnostics": r
                    .blocking
                    .iter()
                    .map(|b| serde_json::json!({
                        "id": b.id,
                        "message": b.message,
                        "levels": b.levels,
                    }))
                    .collect::<Vec<_>>(),
            })
        })
        .collect();

    let mut prestige_entries: Vec<&ClassCensusEntry> =
        entries.values().filter(|e| e.is_prestige).collect();
    prestige_entries.sort_by(|a, b| a.class_id.cmp(&b.class_id));
    let prestige: Vec<serde_json::Value> = prestige_entries
        .iter()
        .map(|e| {
            serde_json::json!({
                "class_id": e.class_id,
                "family": e.family.label(),
                "books": e.books,
                "registries": e.registries,
                "max_level": e.max_level,
                "status": "not_swept_yet",
            })
        })
        .collect();

    let document = serde_json::json!({
        "generated_at": real_now_iso8601(),
        "generated_by": "cargo run --bin class_census -- --json <path>",
        "source_of_truth": "codex::rules_core::pilot_compute::build_pilot_headless_receipt",
        "input_posture": format!(
            "{} with the class swapped to the class under test, swept over 1..=max_level (per \
             the merged registry's own max_level, not a fixed 20), plus the canonical per-class \
             choice/spell seeds compose_character_input applies (pf1_adapter.rs, mirrored in \
             codex::rules_core::class_seeds). A class counts as computed only when every level \
             in its own sweep reaches HeadlessReceiptStatus::Computed. Prestige ids are listed, \
             never swept alone -- see this binary's module doc comment.",
            codex::rules_core::class_seeds::FIXTURE_RELATIVE_PATH,
        ),
        "ids": ids,
        "computed": computed,
        "blocked": blocked,
        "non_prestige_swept": results.len(),
        "prestige_not_swept_yet": prestige.len(),
        "classes": classes,
        "prestige": prestige,
    });

    let text = match serde_json::to_string_pretty(&document) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("class_census: could not serialise the census document: {e}");
            return 1;
        }
    };
    if let Err(e) = std::fs::write(json_path, format!("{text}\n")) {
        eprintln!("class_census: could not write {json_path}: {e}");
        return 1;
    }

    println!("ids={ids} computed={computed} blocked={blocked}");
    0
}

fn usage() -> String {
    "usage: class_census --json <path> | class_census --sheet-dump <class-id>:<level>".to_owned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut json_path: Option<String> = None;
    let mut sheet_dump_arg: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("class_census: --json needs a path\n{}", usage());
                    std::process::exit(2);
                };
                json_path = Some(value.clone());
                i += 2;
            }
            "--sheet-dump" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("class_census: --sheet-dump needs <class-id>:<level>\n{}", usage());
                    std::process::exit(2);
                };
                sheet_dump_arg = Some(value.clone());
                i += 2;
            }
            other => {
                eprintln!("class_census: unknown argument {other:?}\n{}", usage());
                std::process::exit(2);
            }
        }
    }

    let exit_code = if let Some(raw) = sheet_dump_arg {
        run_sheet_dump(&raw)
    } else if let Some(path) = json_path {
        run_json(&path)
    } else {
        eprintln!("class_census: nothing to do\n{}", usage());
        2
    };

    std::process::exit(exit_code);
}
