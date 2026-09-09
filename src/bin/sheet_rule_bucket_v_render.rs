//! `sheet_rule_bucket_v_render` -- the engine side of SD-35 AT-35-E4-002's bucket-V oracle run
//! (`epic-breakdown.md § AT-35-E4-002`).
//!
//! Tool side. For every unit id named in `--units <ids.json>` it seeds that rule as held
//! outright (`HeldSeed::rule_ids`), runs the **live evaluator** -- `render_sheet` over the
//! package loaded from `data/sheet_rules/`, the same path the desktop's `sheet_lines_for`
//! takes -- and writes the rendered sheet line as JSON. Nothing here reads a PCGen token, a
//! formula string or `raw_tokens`: the values compared against the oracle are the ones the
//! sheet renders, not the old string-formula path's.
//!
//! The character context is `--character <fixture>` when given, else the deterministic Human
//! Fighter 1 fixture (`tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`).
//! **A comparison against a PCGen carrier must pass the fixture that matches that carrier's
//! `.pcg`** -- a level-dependent value renders differently at level 1 and level 20, and a
//! context mismatch reads as a disagreement that is really a harness defect
//! (`bucket_v_parity.py carriers --context` writes the matching fixture).
//!
//! ```text
//! cargo run --locked --release --bin sheet_rule_bucket_v_render -- \
//!     --units <ids.json> [--character <fixture.txt>] --output <ours.json>
//! ```

use std::path::{Path, PathBuf};

use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::corpus_loader::load_sheet_rules;
use codex::rules_core::pilot_compute::{compute_pilot_base_chassis, PilotBaseChassisComputation};
use codex::rules_core::sheet_rule::{
    render_sheet, CharacterFacts, HeldSeed, SheetLine, SheetLineValue, SheetRulePackage,
};
use serde::Serialize;

const DETERMINISTIC_FIXTURE: &str =
    include_str!("../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

/// The form of a rendered value, in the vocabulary `sheet_rule_rendered:<form>` uses.
fn form_of(v: &SheetLineValue) -> &'static str {
    match v {
        SheetLineValue::Resolved(_) => "number",
        SheetLineValue::Dice(_) => "dice",
        SheetLineValue::Words => "words",
    }
}

#[derive(Serialize)]
struct LineOut {
    #[serde(flatten)]
    line: SheetLine,
    form: &'static str,
}

#[derive(Serialize)]
struct UnitOut {
    id: String,
    /// `true` when the package carries a rule with this id.
    in_package: bool,
    /// The unit's own rendered line, when `render_sheet` printed one.
    line: Option<LineOut>,
    /// Lines the seed produced whose id is not the unit's: `#bonusN` siblings and grants.
    siblings: Vec<LineOut>,
    /// `print: false` on the rule -- held, evaluated, deliberately not on the sheet.
    suppressed: bool,
}

fn facts_and_package(repo: &Path, character: &str, label: &str) -> (SheetRulePackage, CharacterFacts) {
    let load = load_sheet_rules(&repo.join("data/sheet_rules"));
    if !load.diagnostics.is_empty() {
        eprintln!(
            "sheet_rule_bucket_v_render: {} package diagnostic(s); first: {:?}",
            load.diagnostics.len(),
            load.diagnostics[0]
        );
        std::process::exit(2);
    }
    let loaded = load_character_input_fixture(character);
    let input = loaded.character_input.unwrap_or_else(|| {
        eprintln!("sheet_rule_bucket_v_render: {label} does not parse as a character input: {:?}", loaded.diagnostics);
        std::process::exit(2);
    });
    let base: PilotBaseChassisComputation = compute_pilot_base_chassis(&input);
    let computed = base.with_sheet_rules(&input, &load.package, &[]);
    let facts = CharacterFacts::from_character(&input, &computed);
    eprintln!(
        "sheet_rule_bucket_v_render: package {} rule files, {} rules; character context {label}",
        load.rule_files,
        load.package.rules.len()
    );
    (load.package, facts)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned();
    let (Some(units_path), Some(output)) = (arg("--units"), arg("--output")) else {
        eprintln!("usage: sheet_rule_bucket_v_render --units <ids.json> [--character <fixture.txt>] --output <ours.json>");
        std::process::exit(2);
    };
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let started = std::time::Instant::now();
    let ids: Vec<String> = serde_json::from_str(&std::fs::read_to_string(&units_path).unwrap_or_else(|e| {
        eprintln!("sheet_rule_bucket_v_render: reading {units_path}: {e}");
        std::process::exit(2);
    }))
    .unwrap_or_else(|e| {
        eprintln!("sheet_rule_bucket_v_render: parsing {units_path}: {e}");
        std::process::exit(2);
    });
    let (character_text, label) = match arg("--character") {
        Some(path) => (
            std::fs::read_to_string(&path).unwrap_or_else(|e| {
                eprintln!("sheet_rule_bucket_v_render: reading {path}: {e}");
                std::process::exit(2);
            }),
            path,
        ),
        None => (DETERMINISTIC_FIXTURE.to_string(), "deterministic_human_fighter_l1".to_string()),
    };
    let (package, facts) = facts_and_package(&repo, &character_text, &label);
    let loaded_at = started.elapsed().as_secs_f64();

    let mut out = Vec::with_capacity(ids.len());
    let render_started = std::time::Instant::now();
    let mut first_fifty_wall = 0.0_f64;
    for (i, id) in ids.iter().enumerate() {
        let seed = HeldSeed { rule_ids: vec![id.clone()], ..Default::default() };
        let lines = render_sheet(&package, &seed, &facts);
        let mut own = None;
        let mut siblings = Vec::new();
        for line in lines {
            let entry = LineOut { form: form_of(&line.value), line };
            if &entry.line.id == id {
                own = Some(entry);
            } else {
                siblings.push(entry);
            }
        }
        let rule = package.rule(id);
        out.push(UnitOut {
            id: id.clone(),
            in_package: rule.is_some(),
            suppressed: own.is_none() && rule.is_some_and(|r| !r.print),
            line: own,
            siblings,
        });
        if i + 1 == 50 {
            first_fifty_wall = render_started.elapsed().as_secs_f64();
            eprintln!("sheet_rule_bucket_v_render: first 50 units rendered in {first_fifty_wall:.2}s ({:.3}s/unit)", first_fifty_wall / 50.0);
        }
    }
    let render_wall = render_started.elapsed().as_secs_f64();
    let rendered = out.iter().filter(|u| u.line.is_some()).count();
    let doc = serde_json::json!({
        "generated_by": "sheet_rule_bucket_v_render",
        "character_context": label,
        "units": out,
        "timing": {
            "package_load_s": loaded_at,
            "first_50_render_s": first_fifty_wall,
            "render_s": render_wall,
        },
    });
    std::fs::write(&output, serde_json::to_string_pretty(&doc).unwrap()).unwrap_or_else(|e| {
        eprintln!("sheet_rule_bucket_v_render: writing {output}: {e}");
        std::process::exit(2);
    });
    println!(
        "sheet_rule_bucket_v_render: units={} rendered={} package_load={loaded_at:.1}s render={render_wall:.1}s -> {output}",
        ids.len(),
        rendered
    );
}
