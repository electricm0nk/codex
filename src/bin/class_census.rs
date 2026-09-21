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
//! A prestige class is never a legitimate `Computed` measurement ALONE (PF1's
//! own prestige entry requirements only a carrier build can satisfy). F0c
//! (`epic-f-class-completion.md` §2, review finding 14) adds the
//! deterministic carrier build: each prestige class's converted `applies`
//! gate picks a carrier (`wizard`/`cleric`/`fighter`, or both `wizard` AND
//! `cleric` independently for the two dual-caster prestige classes), the
//! carrier's own level is the smallest one meeting every translatable
//! numeric entry-gate term (floor 5, capped so `carrier + prestige max_level
//! <= 20`), and the prestige class's own levels `1..=max_level` are swept in
//! that mix for its real, engine-derived status. Each row also carries its
//! own `alone_status` negative control (the same class swept with NO
//! carrier at all -- expected Blocked, `BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED`)
//! and a printed, non-blocking `entry_gate` verdict (`met`/`unmet`/`unknown`,
//! or `partially-met` for the dual-caster case). A gate that reads a
//! caster-level/spell-kind term ONLY through `Applies::Not` (a negation) or
//! `Applies::AtLeast` (one optional alternative among several) -- never as a
//! mandatory, positive top-level term -- cannot be grounded to a carrier
//! with confidence (F0-check fix for findings 1/4); such a row reports
//! `carrier: []`, `entry_gate: "unknown"`, `status: "Unknown"`, and
//! `carrier_unknown_reason` naming the gate, rather than a carrier the
//! gate's own text forbids. See
//! `docs/release/SD-36-consolidation/artifacts/epic-f/census-f0c.json` for a
//! committed sample.
//!
//! F0d (`epic-f-class-completion.md` §2 "Mix panel", §5 review finding 8)
//! adds the multiclass mix panel: every EXISTING multiclass
//! negative-control test's own (class, level) + (class, level) input,
//! extracted mechanically (`scripts/extract_multiclass_census_panel.py`,
//! never retyped) into the committed
//! `tests/fixtures/rules_core/multiclass_census_panel.json` (185 rows,
//! measured -- see that script's own doc comment for the exact grep
//! commands and per-source counts), re-swept here through the same shared
//! canonical fixture every other sweep in this bin uses, and reported
//! with a histogram of claim-blocking diagnostic ids across the panel
//! (`mix_panel_blocking_histogram` in the JSON; every row's blocking ids
//! are already deduplicated per row before the histogram counts rows, not
//! raw occurrences). See
//! `docs/release/SD-36-consolidation/artifacts/epic-f/census-f0d.json` for
//! a committed sample and
//! `docs/release/SD-36-consolidation/artifacts/epic-f/mix-panel-histogram.md`
//! for the human-readable top-blockers writeup.
//!
//! # Modes
//!
//! - `--json <path>`: sweep every non-prestige id, plus every prestige id in
//!   its carrier mix(es) and alone, write the full document to `<path>`, and
//!   print `ids=<N> computed=<M> blocked=<K>` to stdout (`N` = the full
//!   merged census, `M` = non-prestige classes whose every swept level
//!   reached `Computed`, `K = N - M` -- the prestige carrier-mix figures are
//!   printed separately, see below, and never fold into `M`/`K`: the epic's
//!   own two-part acceptance number keeps the two measurements distinct).
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

use codex::rules_core::class_census::{
    ClassSweepResult, census, load_mix_panel, load_sweep_fixture, mix_panel_blocking_histogram,
    sheet_dump_text, sweep_mix_panel, sweep_non_prestige, sweep_prestige,
};

/// The non-prestige sweep's own `(computed, blocked)` partition --
/// `results.len()` (`non_prestige_swept`) is its own denominator, NEVER the
/// full merged census `ids` (which also carries the prestige ids `results`
/// never contains at all). F0-check finding 2 (RED first): factored out of
/// `run_json` so the partition invariant can be pinned by a test that does
/// not have to shell out to the bin itself.
fn partition_non_prestige(results: &[ClassSweepResult]) -> (usize, usize) {
    let computed = results.iter().filter(|r| r.computed()).count();
    let blocked = results.len() - computed;
    (computed, blocked)
}

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

    // F0-check finding 2 (RED first): `blocked` is a count over the
    // NON-PRESTIGE sweep, exactly as `computed` is -- never `ids -
    // computed`. The old `ids - computed` silently folded every one of the
    // 74 prestige ids (never swept in `results` at all) into "blocked", so
    // `blocked` (93) + `computed` (42) equalled the FULL merged `ids` (135)
    // instead of `non_prestige_swept` (61) -- see
    // `partition_non_prestige_never_folds_in_the_prestige_ids` below.
    let (computed, blocked) = partition_non_prestige(&results);

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

    // F0c: every prestige id, in its own carrier mix(es) plus alone (the
    // negative control). Same silenced-panic-hook posture as the
    // non-prestige sweep above.
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let prestige_rows = sweep_prestige(&fixture, &entries);
    std::panic::set_hook(previous_hook);

    let prestige_alone_blocked = prestige_rows.iter().filter(|r| !r.alone.computed()).count();
    // `r.status` (not a raw `mixes.iter().all(..)`, which reads vacuously
    // true over an EMPTY `mixes` vec) is the row's own combined status --
    // F0-check finding 4: a row whose carrier could not be named at all
    // (empty `mixes`, `status == "unknown"`) must never silently count as
    // Computed.
    let prestige_mix_computed = prestige_rows.iter().filter(|r| r.status == "computed").count();
    let prestige_mix_unknown = prestige_rows.iter().filter(|r| r.status == "unknown").count();

    let prestige: Vec<serde_json::Value> = prestige_rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "class_id": row.class_id,
                "books": row.books,
                "max_level": row.max_level,
                "carrier": row.carriers.iter().map(|c| c.slug()).collect::<Vec<_>>(),
                "status": match row.status {
                    "computed" => "Computed",
                    "blocked" => "Blocked",
                    _ => "Unknown",
                },
                "entry_gate": row.entry_gate_status,
                "load_error": row.load_error,
                "carrier_unknown_reason": row.carrier_unknown_reason,
                "mixes": row.mixes.iter().map(|(sweep, gate)| serde_json::json!({
                    "carrier": sweep.carrier.slug(),
                    "carrier_level": sweep.carrier_level,
                    "status": if sweep.computed() { "Computed" } else { "Blocked" },
                    "levels_computed": sweep.levels_computed,
                    "levels_blocked": sweep.levels_blocked,
                    "blocking_diagnostics": sweep.blocking.iter().map(|b| serde_json::json!({
                        "id": b.id,
                        "message": b.message,
                        "levels": b.levels,
                    })).collect::<Vec<_>>(),
                    "entry_gate": {
                        "status": gate.status,
                        "met": gate.met,
                        "unmet": gate.unmet,
                        "unknown": gate.unknown,
                    },
                })).collect::<Vec<_>>(),
                "alone_status": if row.alone.computed() { "Computed" } else { "Blocked" },
                "alone_blocking_diagnostics": row.alone.blocking.iter().map(|b| serde_json::json!({
                    "id": b.id,
                    "message": b.message,
                    "levels": b.levels,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();

    // F0d: the multiclass mix panel -- every EXISTING multiclass
    // negative-control test's own (class, level) input, re-swept through
    // the shared canonical fixture, plus a histogram of claim-blocking
    // diagnostic ids across the panel. `load_mix_panel` reads the
    // committed, mechanically-extracted
    // `tests/fixtures/rules_core/multiclass_census_panel.json`; failure to
    // load it is a real error (the file is a committed dependency of this
    // bin, not optional), not a silently-empty panel.
    let mix_panel = load_mix_panel().unwrap_or_else(|e| {
        eprintln!("class_census: {e}");
        std::process::exit(1);
    });
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mix_panel_results = sweep_mix_panel(&fixture, &mix_panel);
    std::panic::set_hook(previous_hook);

    let mix_panel_computed = mix_panel_results.iter().filter(|r| r.computed).count();
    let mix_panel_blocked = mix_panel_results.len() - mix_panel_computed;
    let mix_panel_histogram = mix_panel_blocking_histogram(&mix_panel_results);

    let mix_panel_json: Vec<serde_json::Value> = mix_panel_results
        .iter()
        .map(|r| {
            serde_json::json!({
                "key": r.key,
                "source_file": r.source_file,
                "test_fn": r.test_fn,
                "classes": r.classes,
                "status": if r.computed { "Computed" } else { "Blocked" },
                "blocking_diagnostic_ids": r.blocking_diagnostic_ids,
            })
        })
        .collect();
    let mix_panel_histogram_json: serde_json::Value =
        serde_json::Value::Object(mix_panel_histogram.iter().map(|(id, n)| (id.clone(), serde_json::json!(n))).collect());

    let document = serde_json::json!({
        "generated_at": real_now_iso8601(),
        "generated_by": "cargo run --bin class_census -- --json <path>",
        "source_of_truth": "codex::rules_core::pilot_compute::build_pilot_headless_receipt",
        "input_posture": format!(
            "{} with the class swapped to the class under test, swept over 1..=max_level (per \
             the merged registry's own max_level, not a fixed 20), plus the canonical per-class \
             choice/spell seeds compose_character_input applies (pf1_adapter.rs, mirrored in \
             codex::rules_core::class_seeds). A non-prestige class counts as computed only when \
             every level in its own sweep reaches HeadlessReceiptStatus::Computed. A prestige \
             class is never measured alone -- 'alone_status' is a negative control, expected \
             Blocked -- its real Computed measurement is the carrier-mix sweep in 'mixes' (§2, \
             epic-f-class-completion.md, review finding 14): the carrier(s) its own converted \
             entry gate selects (wizard/cleric/fighter, or both independently for the \
             dual-caster case), at the smallest level meeting every translatable numeric \
             entry-gate term (floor 5, capped at carrier + prestige max_level <= 20). \
             'entry_gate' is printed rule text, non-blocking by design -- it never gates \
             'mixes[].status'.",
            codex::rules_core::class_seeds::FIXTURE_RELATIVE_PATH,
        ),
        "ids": ids,
        "computed": computed,
        "blocked": blocked,
        "non_prestige_swept": results.len(),
        "prestige_swept": prestige_rows.len(),
        "prestige_alone_blocked": prestige_alone_blocked,
        "prestige_mix_computed": prestige_mix_computed,
        "prestige_mix_unknown": prestige_mix_unknown,
        "classes": classes,
        "prestige": prestige,
        "mix_panel_swept": mix_panel_results.len(),
        "mix_panel_computed": mix_panel_computed,
        "mix_panel_blocked": mix_panel_blocked,
        "mix_panel": mix_panel_json,
        "mix_panel_blocking_histogram": mix_panel_histogram_json,
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
    println!(
        "prestige_swept={} prestige_alone_blocked={} prestige_mix_computed={} prestige_mix_unknown={}",
        prestige_rows.len(),
        prestige_alone_blocked,
        prestige_mix_computed,
        prestige_mix_unknown
    );
    println!(
        "mix_panel_swept={} mix_panel_computed={} mix_panel_blocked={}",
        mix_panel_results.len(),
        mix_panel_computed,
        mix_panel_blocked
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::class_census::load_sweep_fixture;

    /// F0-check finding 2 (RED first, `cargo test --locked -j 2 --bin
    /// class_census`): `partition_non_prestige`'s own `(computed, blocked)`
    /// pair must sum to the non-prestige population it was measured over
    /// (`results.len()`) -- NEVER the full merged census `ids`, which also
    /// carries the 74 prestige ids `sweep_non_prestige` never sweeps at
    /// all. This is the exact invariant whose violation (`blocked = ids -
    /// computed`) made `docs/architecture/status.md` print "93 of 135" for
    /// a population of 61.
    #[test]
    fn partition_non_prestige_never_folds_in_the_prestige_ids() {
        let entries = census();
        let ids = entries.len();
        let fixture = load_sweep_fixture().expect("shared deterministic fixture must load cleanly");

        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let results = sweep_non_prestige(&fixture, &entries);
        std::panic::set_hook(previous_hook);

        let (computed, blocked) = partition_non_prestige(&results);

        assert_eq!(
            computed + blocked,
            results.len(),
            "computed + blocked must partition exactly the non-prestige population swept"
        );
        // The real corpus has prestige ids at all (74, measured elsewhere),
        // so the non-prestige population is strictly smaller than the full
        // merged census -- proving the two denominators are genuinely
        // different, not coincidentally equal in this fixture.
        assert!(
            results.len() < ids,
            "non-prestige population ({}) must be strictly smaller than the full census ({ids}) \
             -- otherwise this test cannot distinguish the fixed denominator from the old bug",
            results.len()
        );
        assert_ne!(
            computed + blocked,
            ids,
            "computed + blocked ({}) must NOT equal the full merged ids ({ids}) -- that is \
             exactly the F0-check finding 2 regression shape (`blocked = ids - computed`)",
            computed + blocked
        );
    }
}
