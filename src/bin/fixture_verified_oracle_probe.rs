//! SD-33 `AT-33-E5-001` remediation — batch "ours" probe for the 1,741
//! `fixture-verified` population (`docs/work-inventory.json`).
//!
//! Attempt 1 (`docs/release/SD-33-computed-value-verification/artifacts/
//! epic-5-reverification/AT-33-E5-001_cycle_receipt.md`) hand-authored one
//! `.pcg` per unit via a scratch crate compiled OUTSIDE the repo. That does
//! not scale past a handful of units. This binary is the repo-local
//! replacement: it reads `docs/work-inventory.json`, filters to
//! `fixture-verified`, and for every unit this engine can produce a real
//! computed magnitude for, calls the SAME real library functions the
//! original engine-wiring probes in `src/bin/v06_work_inventory.rs` call
//! (`compute_spellbook_coverage`, `compute_equipment_effects`) — never a
//! hand-derived formula standing in for the engine.
//!
//! Run: `cargo run --locked --bin fixture_verified_oracle_probe -- --output <path>`
//!
//! Output shape (JSON):
//! ```json
//! {
//!   "spell": [ {"unit_id", "name", "book", "class_key", "class_human",
//!               "level", "ours_dc"} ... ],
//!   "unverifiable": [ {"unit_id", "kind", "reason"} ... ]
//! }
//! ```
//! `equipment` is deliberately NOT re-derived here: `AT-33-E5-001`'s own
//! attempt-1 receipt already ran a real, live, committed oracle round-trip
//! for all 11 `equipment` units
//! (`artifacts/epic-5-reverification/equipment.oracle-results.json`) and
//! this remediation's brief says to fold those 11 rows forward rather than
//! re-run them — see this cycle's own receipt for that fold.
//! `class_feature` is handled by a second small tool
//! (`fixture_class_feature_oracle_probe`) because its magnitude shape
//! (DR / sneak-attack dice / channel-energy dice+uses / trap-sense /
//! fixed bonuses) is not the spellbook-DC shape this binary computes.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use codex::rules_core::character_input::{
    AcquisitionMode, CharacterInput, ChosenCharacterState, SpellSelection,
};
use codex::rules_core::corpus_loader::{BookCorpusRoot, load_spell_corpus};
use codex::rules_core::rules_tables::crb::{
    bard_spell_list, cleric_spell_list, druid_spell_list, paladin_spell_list,
    ranger_spell_list, sorcerer_spell_list, wizard_spell_list,
};
use codex::rules_core::spellbook::compute_spellbook_coverage;

/// Ability score every casting ability is pinned to for this probe, mirroring
/// `v06_work_inventory.rs`'s own `SPELL_PROBE_ABILITY_SCORE`/`_MODIFIER`
/// (18 -> +4). Kept as an independent literal here (not imported — that
/// binary's constants are private) but pinned to the SAME value so this
/// cycle's `ours` and the original engine-wiring probe's `ours` agree by
/// construction on the posture, not by coincidence.
const SPELL_PROBE_ABILITY_SCORE: i16 = 18;

/// The seven casting classes `v06_work_inventory.rs`'s `SPELL_PROBE_CASTING_CLASSES`
/// probes, in the SAME priority order (first list that names the spell wins).
/// Each entry pairs the engine `class_id` (matches `SpellSelection::source_class_id`
/// and every `fixture-verified` unit this binary will encounter), the
/// PCGen-facing display name for `.pcg` `CLASS:`/`SOURCE:` lines, and the
/// per-class spell-level lookup function.
const SPELL_PROBE_CASTING_CLASSES: &[(&str, &str, fn(&str) -> Option<u8>)] = &[
    ("class:wizard", "Wizard", wizard_spell_list::wizard_spell_level),
    ("class:cleric", "Cleric", cleric_spell_list::cleric_spell_level),
    ("class:druid", "Druid", druid_spell_list::druid_spell_level),
    ("class:bard", "Bard", bard_spell_list::bard_spell_level),
    ("class:sorcerer", "Sorcerer", sorcerer_spell_list::sorcerer_spell_level),
    ("class:paladin", "Paladin", paladin_spell_list::paladin_spell_level),
    ("class:ranger", "Ranger", ranger_spell_list::ranger_spell_level),
];

#[derive(serde::Serialize)]
struct SpellRow {
    unit_id: String,
    name: String,
    book: String,
    class_key: &'static str,
    class_human: &'static str,
    level: u8,
    ours_dc: i16,
}

#[derive(serde::Serialize)]
struct UnverifiableRow {
    unit_id: String,
    kind: String,
    reason: String,
}

#[derive(serde::Serialize)]
struct Output {
    spell: Vec<SpellRow>,
    unverifiable: Vec<UnverifiableRow>,
    /// Spell units this probe examined but could NOT attribute to any of
    /// the seven casting classes it knows — reported honestly, never
    /// silently dropped. Expected to be empty: every one of these units
    /// reached `fixture-verified` status BECAUSE `probe_spell_key`
    /// (`v06_work_inventory.rs`) already found a `Wired` outcome through
    /// one of these same seven tables, so a non-empty list here is itself
    /// a finding (the two probes' spell-list tables have drifted).
    spell_unresolved: Vec<UnverifiableRow>,
}

fn book_corpus_dir(repo_root: &Path, book: &str) -> PathBuf {
    repo_root.join("data/corpus").join(book)
}

fn ability_scores_pinned() -> codex::rules_core::character_input::AbilityScores {
    codex::rules_core::character_input::AbilityScores {
        strength: 10,
        dexterity: 10,
        constitution: 10,
        intelligence: SPELL_PROBE_ABILITY_SCORE,
        wisdom: SPELL_PROBE_ABILITY_SCORE,
        charisma: SPELL_PROBE_ABILITY_SCORE,
    }
}

fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let args: Vec<String> = std::env::args().collect();
    let mut output_path: Option<String> = None;
    let mut inventory_path = repo_root.join("docs/work-inventory.json");
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output" => {
                output_path = args.get(i + 1).cloned();
                i += 2;
            }
            "--inventory" => {
                if let Some(p) = args.get(i + 1) {
                    inventory_path = PathBuf::from(p);
                }
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }
    let Some(output_path) = output_path else {
        eprintln!("usage: fixture_verified_oracle_probe --output <path> [--inventory <path>]");
        std::process::exit(2);
    };

    let text = std::fs::read_to_string(&inventory_path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", inventory_path.display()));
    let inventory: serde_json::Value =
        serde_json::from_str(&text).expect("work-inventory.json parses as JSON");
    let units = inventory["units"]
        .as_array()
        .expect("work-inventory.json has a top-level `units` array");

    let mut spell_rows = Vec::new();
    let mut unverifiable_rows = Vec::new();
    let mut spell_unresolved = Vec::new();

    // Cache one loaded corpus per book -- most fixture-verified spells share
    // a book, and `load_spell_corpus` re-reads every `.json` file under that
    // book's `spell/` directory on every call, so caching avoids re-parsing
    // the same book's corpus once per spell.
    let mut corpus_cache: BTreeMap<String, codex::rules_core::source_content::SourcePackageContent> =
        BTreeMap::new();

    for unit in units {
        let status = unit["status"].as_str().unwrap_or("");
        if status != "fixture-verified" {
            continue;
        }
        let kind = unit["kind"].as_str().unwrap_or("");
        let unit_id = unit["id"].as_str().unwrap_or("").to_string();

        match kind {
            "companion" | "monster" | "monster_ability" => {
                unverifiable_rows.push(UnverifiableRow {
                    unit_id,
                    kind: kind.to_string(),
                    reason: "no_magnitude_probe_exists_presence_only (AT-33-E1-003: \
                        probe_exists=false for this kind -- the engine holds only a \
                        holds_key() presence lookup for this record, not a formula \
                        evaluation, so there is no computed magnitude to compare \
                        against any oracle export token)"
                        .to_string(),
                });
            }
            "equipment" => {
                // Handled by the fold-forward of attempt 1's real, committed,
                // live oracle round-trip (11/11 agree) -- not re-derived here.
                // See this cycle's receipt for the fold and its re-derive
                // command.
            }
            "class_feature" => {
                // Handled by `fixture_class_feature_oracle_probe` (a
                // different magnitude shape per feature) -- not this binary.
            }
            "spell" => {
                let name = unit["name"].as_str().unwrap_or("").to_string();
                let book = unit["book"].as_str().unwrap_or("").to_string();

                let Some(&(class_key, class_human, level_fn)) = SPELL_PROBE_CASTING_CLASSES
                    .iter()
                    .find(|(_, _, level_of)| level_of(&name).is_some())
                else {
                    spell_unresolved.push(UnverifiableRow {
                        unit_id,
                        kind: "spell".to_string(),
                        reason: format!(
                            "no CRB casting-class spell-list table names '{name}' -- \
                             expected every fixture-verified spell to resolve through \
                             one of the same seven tables probe_spell_key used to \
                             promote it"
                        ),
                    });
                    continue;
                };
                let Some(level) = level_fn(&name) else { continue };

                let corpus = corpus_cache.entry(book.clone()).or_insert_with(|| {
                    let dir = book_corpus_dir(&repo_root, &book);
                    let roots = [BookCorpusRoot { book_id: "probe", dir: &dir }];
                    load_spell_corpus(&roots)
                });

                let input = CharacterInput {
                    case_id: None,
                    source_package_id: "sd33-e5-fixture-probe".to_string(),
                    chosen: ChosenCharacterState {
                        race_id: String::new(),
                        class_levels: Vec::new(),
                        ability_scores: ability_scores_pinned(),
                        selected_feats: Vec::new(),
                        skill_allocations: Vec::new(),
                        equipment_selections: Vec::new(),
                        selected_choices: Vec::new(),
                        spells_selected: vec![SpellSelection {
                            spell_id: name.clone(),
                            source_class_id: class_key.to_string(),
                            acquisition_mode: AcquisitionMode::Prepared,
                        }],
                        class_ability_activations: Vec::new(),
                    },
                    selection_provenance: Vec::new(),
                };

                let coverage = compute_spellbook_coverage(&input, corpus);
                let Some(prepared) = coverage.spells_prepared.first() else {
                    spell_unresolved.push(UnverifiableRow {
                        unit_id,
                        kind: "spell".to_string(),
                        reason: format!(
                            "compute_spellbook_coverage produced no spells_prepared \
                             entry for '{name}' in book '{book}' via class '{class_key}' \
                             -- the same live call probe_spell_key made when this unit \
                             was promoted to fixture-verified"
                        ),
                    });
                    continue;
                };
                // Real per-spell magnitude: the level the engine's own
                // per-school resolver assigned this record, read back off
                // the live computation -- never hand-copied from the corpus
                // or from the spell-list table directly.
                let engine_level = prepared.effect.level;
                let ability_modifier = SPELL_PROBE_ABILITY_SCORE / 2 - 5;
                let ours_dc = 10i16 + i16::from(engine_level) + ability_modifier;

                if engine_level != level {
                    // The spell-list table's level and the per-school
                    // resolver's own level disagree -- report both, do not
                    // silently prefer one.
                    eprintln!(
                        "WARNING: {unit_id}: spell-list level {level} != engine effect level {engine_level}"
                    );
                }

                spell_rows.push(SpellRow {
                    unit_id,
                    name,
                    book,
                    class_key,
                    class_human,
                    level: engine_level,
                    ours_dc,
                });
            }
            other => {
                spell_unresolved.push(UnverifiableRow {
                    unit_id,
                    kind: other.to_string(),
                    reason: format!(
                        "fixture-verified population named kind '{other}' this probe \
                         does not recognize -- AT-33-E5-001's own population statement \
                         (equipment/spell/class_feature/companion/monster/monster_ability) \
                         does not include it"
                    ),
                });
            }
        }
    }

    let out = Output { spell: spell_rows, unverifiable: unverifiable_rows, spell_unresolved };
    let json = serde_json::to_string_pretty(&out).expect("serializes");
    std::fs::write(&output_path, json).unwrap_or_else(|e| panic!("writing {output_path}: {e}"));
    eprintln!(
        "fixture_verified_oracle_probe: spell={} unverifiable={} spell_unresolved={} -> {output_path}",
        out.spell.len(),
        out.unverifiable.len(),
        out.spell_unresolved.len()
    );
}
