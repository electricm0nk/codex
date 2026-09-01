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
//!
//! ## `--remainder <path>` mode (SD-33 remediation wave 2, `spell-remainder` slice)
//!
//! Widens this SAME binary (never forked) to the 815-unit remainder
//! `AT-33-E5-001`'s own next-cycle plan named: the 598 `fixture-verified`
//! `spell` units carrying evidence `spell_list_entry_with_resolved_level`
//! (not `spell_effect_probe_observed_computed_delta`) plus all 217
//! `literal-verified` `spell` units (`AT-33-E5-002`'s own named remainder).
//! Uses the casting-ability mapping derived from the pinned PCGen oracle
//! checkout (`scripts/oracle_harness/derive_spell_casting_ability_mapping.py`
//! -> `scripts/oracle_harness/spell_casting_ability_mapping.json`, embedded
//! below) to state, per unit, EXACTLY why no `ours` value exists when none
//! does -- never a guess, always a live call into the real engine functions
//! this binary already uses for the base population.

use std::collections::{BTreeMap, HashMap};
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
/// (`class:<id>`, PCGen display name, spell-level lookup fn) per row below.
type SpellProbeCastingClass = (&'static str, &'static str, fn(&str) -> Option<u8>);
const SPELL_PROBE_CASTING_CLASSES: &[SpellProbeCastingClass] = &[
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

// --- `--remainder` mode ---------------------------------------------------

/// The casting-ability mapping derived from the pinned PCGen oracle
/// checkout's own `CLASS:<Name> ... SPELLSTAT:<ABBREV>` declarations
/// (`scripts/oracle_harness/derive_spell_casting_ability_mapping.py`).
/// Embedded at compile time so this binary never re-shells out to Python or
/// re-parses the oracle checkout at run time; the JSON file is the
/// committed, independently re-derivable source of truth.
const CASTING_ABILITY_MAPPING_JSON: &str =
    include_str!("../../scripts/oracle_harness/spell_casting_ability_mapping.json");

/// The same seven classes `casting_ability_for_class`
/// (`src/rules_core/spellbook.rs`, private) recognizes — restated here
/// because that function is private to its module. Matches
/// `SPELL_PROBE_CASTING_CLASSES`'s own class set above, by human-readable
/// PCGen class name (as it appears in a corpus record's `CLASSES:` token)
/// rather than `class:`-prefixed id, since remainder classification reads
/// straight off the corpus token.
const ENGINE_MAPPED_CLASSES: &[&str] =
    &["Wizard", "Cleric", "Druid", "Ranger", "Sorcerer", "Bard", "Paladin"];

/// The three books `spellbook.rs`'s per-school `resolve_*_spell_effect`
/// functions read (`crb::spell_list` widened to `apg`/`acg` — confirmed by
/// reading `src/rules_core/spellbook/illusion.rs` and its eight siblings,
/// every one following the identical three-table widening pattern). A
/// remainder unit from any OTHER book cannot produce a `SpellEffect`
/// through this engine seam regardless of casting-class mapping.
const ENGINE_SPELL_LIST_BOOKS: &[&str] =
    &["core_rulebook", "advanced_players_guide", "advanced_class_guide"];

fn corpus_key_to_class_human(name: &str) -> Option<(&'static str, &'static str)> {
    SPELL_PROBE_CASTING_CLASSES
        .iter()
        .find(|(_, human, _)| *human == name)
        .map(|(key, human, _)| (*key, *human))
}

/// One corpus spell record's raw facts, read directly off its own JSON file
/// (not through `SourcePackageContent`, which does not expose raw tokens) —
/// classification only, never fed to `compute_spellbook_coverage` itself.
struct RawSpellFacts {
    level: Option<u8>,
    classes_named: Vec<String>,
    has_domains_token: bool,
}

/// Indexes every real corpus spell JSON under `data/corpus/<book>/spell/`
/// (recursive — `AT-33-E5-002`'s own "known hazard" precedent: the corpus
/// nests spell records by level subdirectory) once per book, keyed by the
/// record's own `data.key` field (matches `unit.corpus_key`, not filename).
fn index_book_spell_corpus(repo_root: &Path, book: &str) -> HashMap<String, RawSpellFacts> {
    let mut index = HashMap::new();
    let dir = repo_root.join("data/corpus").join(book).join("spell");
    let mut stack = vec![dir];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            let Some(data) = json.get("data") else { continue };
            let Some(key) = data.get("key").and_then(|v| v.as_str()) else { continue };
            let level = data.get("level").and_then(|v| v.as_u64()).map(|v| v as u8);
            let mut classes_named = Vec::new();
            let mut has_domains_token = false;
            if let Some(tokens) = data.get("raw_tokens").and_then(|v| v.as_array()) {
                for tok in tokens {
                    let tkey = tok.get("key").and_then(|v| v.as_str()).unwrap_or("");
                    let tval = tok.get("value").and_then(|v| v.as_str()).unwrap_or("");
                    if tkey == "CLASSES" {
                        for part in tval.split(',') {
                            if let Some((cls, _lvl)) = part.trim().split_once('=') {
                                let cls = cls.trim().to_string();
                                if !classes_named.contains(&cls) {
                                    classes_named.push(cls);
                                }
                            }
                        }
                    } else if tkey == "DOMAINS" {
                        has_domains_token = true;
                    }
                }
            }
            index.insert(key.to_string(), RawSpellFacts { level, classes_named, has_domains_token });
        }
    }
    index
}

/// Attempts a real `compute_spellbook_coverage` call for `spell_name`,
/// via `class_human`'s engine class id, against `book`'s real corpus —
/// the exact mechanism the base population above uses, reused (never
/// forked) rather than re-implemented for the remainder.
/// Returns `(engine_level, ours_dc)` — the level is read back off the SAME
/// live `compute_spellbook_coverage` call the DC comes from (the engine's
/// own per-school `SPELL_LIST` table, via `prepared.effect.level`), never
/// from a separate per-class table. The base population's own `--output`
/// mode (above) follows this identical discipline (`engine_level` variable)
/// specifically because the per-class level table and the per-school table
/// can disagree for a given spell — using two different tables for the join
/// key vs. the DC arithmetic would silently misjoin against the oracle
/// export (a bug this cycle's own first draft made and caught: 8 of 100
/// "already reachable" units showed a spurious `oracle-ours=1` delta before
/// this fix, traced to exactly that mismatch — e.g. `Blood Biography` reads
/// level 3 on `wizard_spell_list` but level 2 on `crb::spell_list`'s own
/// generic entry).
fn try_real_spell_save_dc(
    name: &str,
    class_key: &'static str,
    corpus: &codex::rules_core::source_content::SourcePackageContent,
) -> Option<(u8, u8)> {
    let input = CharacterInput {
        case_id: None,
        source_package_id: "sd33-r2-spell-remainder-probe".to_string(),
        chosen: ChosenCharacterState {
            race_id: String::new(),
            class_levels: Vec::new(),
            ability_scores: ability_scores_pinned(),
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            selected_traits: Vec::new(),
            spells_selected: vec![SpellSelection {
                spell_id: name.to_string(),
                source_class_id: class_key.to_string(),
                acquisition_mode: AcquisitionMode::Prepared,
            }],
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    };
    let coverage = compute_spellbook_coverage(&input, corpus);
    coverage.spells_prepared.first().map(|prepared| {
        let ability_modifier = SPELL_PROBE_ABILITY_SCORE / 2 - 5;
        let engine_level = prepared.effect.level;
        let dc = (10i16 + i16::from(engine_level) + ability_modifier) as u8;
        (engine_level, dc)
    })
}

fn run_remainder_mode(repo_root: &Path, inventory_path: &Path, output_path: &str) {
    let mapping_json: serde_json::Value =
        serde_json::from_str(CASTING_ABILITY_MAPPING_JSON).expect("mapping JSON parses");
    let ability_for_class: BTreeMap<String, String> = mapping_json["mapping"]
        .as_object()
        .expect("mapping JSON has a `mapping` object")
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
        .collect();

    let text = std::fs::read_to_string(inventory_path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", inventory_path.display()));
    let inventory: serde_json::Value =
        serde_json::from_str(&text).expect("work-inventory.json parses as JSON");
    let units = inventory["units"]
        .as_array()
        .expect("work-inventory.json has a top-level `units` array");

    let mut spell_rows = Vec::new();
    let mut unverifiable_rows = Vec::new();
    let mut corpus_cache: BTreeMap<String, codex::rules_core::source_content::SourcePackageContent> =
        BTreeMap::new();
    let mut raw_index_cache: BTreeMap<String, HashMap<String, RawSpellFacts>> = BTreeMap::new();

    for unit in units {
        let status = unit["status"].as_str().unwrap_or("");
        let kind = unit["kind"].as_str().unwrap_or("");
        if kind != "spell" {
            continue;
        }
        let evidence = unit["evidence"].as_str().unwrap_or("");
        let in_population = (status == "fixture-verified"
            && evidence == "spell_list_entry_with_resolved_level")
            || status == "literal-verified";
        if !in_population {
            continue;
        }

        let unit_id = unit["id"].as_str().unwrap_or("").to_string();
        let name = unit["name"].as_str().unwrap_or("").to_string();
        let book = unit["book"].as_str().unwrap_or("").to_string();
        let corpus_key = unit["corpus_key"].as_str().unwrap_or(&name).to_string();

        if evidence == "spell_effect_probe_observed_computed_delta" {
            // Already reachable through the exact base-population mechanism
            // above (this evidence value means `probe_spell_key` already
            // found a `Wired` outcome for it) — reuse it directly rather
            // than re-deriving reachability.
            let Some(&(class_key, class_human, level_fn)) = SPELL_PROBE_CASTING_CLASSES
                .iter()
                .find(|(_, _, level_of)| level_of(&name).is_some())
            else {
                unverifiable_rows.push(UnverifiableRow {
                    unit_id,
                    kind: "spell".to_string(),
                    reason: format!(
                        "evidence=spell_effect_probe_observed_computed_delta claims a \
                         casting-class match for '{name}' but no SPELL_PROBE_CASTING_CLASSES \
                         table names it in this probe — the two probes' tables have drifted"
                    ),
                });
                continue;
            };
            if level_fn(&name).is_none() {
                continue;
            }
            let corpus = corpus_cache.entry(book.clone()).or_insert_with(|| {
                let dir = book_corpus_dir(repo_root, &book);
                let roots = [BookCorpusRoot { book_id: "probe", dir: &dir }];
                load_spell_corpus(&roots)
            });
            if let Some((engine_level, ours_dc)) = try_real_spell_save_dc(&name, class_key, corpus) {
                spell_rows.push(SpellRow {
                    unit_id,
                    name,
                    book,
                    class_key,
                    class_human,
                    level: engine_level,
                    ours_dc: i16::from(ours_dc),
                });
            } else {
                unverifiable_rows.push(UnverifiableRow {
                    unit_id,
                    kind: "spell".to_string(),
                    reason: format!(
                        "evidence=spell_effect_probe_observed_computed_delta claims a real \
                         computed delta exists for '{name}' via class '{class_key}', but this \
                         cycle's live compute_spellbook_coverage call produced no \
                         spells_prepared entry for it in book '{book}'"
                    ),
                });
            }
            continue;
        }

        if evidence == "spell_list_entry_with_description_but_no_corpus_level" {
            unverifiable_rows.push(UnverifiableRow {
                unit_id,
                kind: "spell".to_string(),
                reason: format!(
                    "no_corpus_level: corpus record for '{corpus_key}' in book '{book}' carries \
                     no resolvable spell level (evidence={evidence}) — no computed magnitude \
                     (save DC, caster level, or bonus spells) can be derived without a level"
                ),
            });
            continue;
        }

        // evidence == "spell_list_entry_with_resolved_level": the named
        // AT-33-E5-001 blocker population.
        let raw_index = raw_index_cache
            .entry(book.clone())
            .or_insert_with(|| index_book_spell_corpus(repo_root, &book));
        let Some(facts) = raw_index.get(&corpus_key) else {
            unverifiable_rows.push(UnverifiableRow {
                unit_id,
                kind: "spell".to_string(),
                reason: format!(
                    "missing_corpus_record: no corpus spell JSON under data/corpus/{book}/spell/ \
                     carries data.key == '{corpus_key}'"
                ),
            });
            continue;
        };
        let Some(_level) = facts.level else {
            unverifiable_rows.push(UnverifiableRow {
                unit_id,
                kind: "spell".to_string(),
                reason: format!(
                    "no_corpus_level: corpus record for '{corpus_key}' in book '{book}' has a \
                     null level field despite evidence=spell_list_entry_with_resolved_level"
                ),
            });
            continue;
        };
        if facts.classes_named.is_empty() {
            let reason = if facts.has_domains_token {
                format!(
                    "no_class_list_binding: '{corpus_key}' ({book}) carries a DOMAINS token but \
                     no CLASSES token — this spell is granted via domain access (e.g. a Cleric's \
                     domain slot), not any per-class spell list; no engine mechanism in this \
                     codebase models a domain-granted spell's governing class or ability, so no \
                     `ours` DC/level/bonus-spells value is derivable"
                )
            } else {
                format!(
                    "no_class_list_binding: '{corpus_key}' ({book}) carries neither a CLASSES \
                     nor a DOMAINS token — no data exists in this record to derive a governing \
                     class or ability from at all"
                )
            };
            unverifiable_rows.push(UnverifiableRow { unit_id, kind: "spell".to_string(), reason });
            continue;
        }

        // Try every named class that IS one of the engine's mapped seven,
        // in the corpus's own listed order, via a REAL live
        // compute_spellbook_coverage call (never assumed from book scope
        // alone) — a genuine widening attempt, not a lookup-table guess.
        let mapped_named: Vec<&String> = facts
            .classes_named
            .iter()
            .filter(|c| ENGINE_MAPPED_CLASSES.contains(&c.as_str()))
            .collect();

        if mapped_named.is_empty() {
            let named_with_ability: Vec<String> = facts
                .classes_named
                .iter()
                .map(|c| {
                    let ability = ability_for_class.get(c).cloned().unwrap_or_else(|| "unknown (not in the pinned-oracle-derived mapping)".to_string());
                    format!("{c} ({ability})")
                })
                .collect();
            unverifiable_rows.push(UnverifiableRow {
                unit_id,
                kind: "spell".to_string(),
                reason: format!(
                    "no_save_dc_computed: '{corpus_key}' ({book}) is on the spell list of \
                     [{}], none of which is one of the seven classes \
                     src/rules_core/spellbook.rs::casting_ability_for_class covers \
                     (Wizard/Cleric/Druid/Ranger/Sorcerer/Bard/Paladin) — this engine's \
                     compute_spellbook_coverage computes no spell_save_dc entry for any other \
                     class's spell selection, confirmed by this cycle's own casting-ability \
                     mapping (scripts/oracle_harness/spell_casting_ability_mapping.json, \
                     derived from the pinned PCGen oracle's own CLASS:...SPELLSTAT: \
                     declarations), which states each named class's real governing ability but \
                     cannot make the engine compute a DC it has no consumer for",
                    named_with_ability.join(", ")
                ),
            });
            continue;
        }

        let mut resolved = false;
        for cls in &mapped_named {
            let Some((class_key, class_human)) = corpus_key_to_class_human(cls) else { continue };
            let corpus = corpus_cache.entry(book.clone()).or_insert_with(|| {
                let dir = book_corpus_dir(repo_root, &book);
                let roots = [BookCorpusRoot { book_id: "probe", dir: &dir }];
                load_spell_corpus(&roots)
            });
            if let Some((engine_level, ours_dc)) = try_real_spell_save_dc(&corpus_key, class_key, corpus) {
                spell_rows.push(SpellRow {
                    unit_id: unit_id.clone(),
                    name: corpus_key.clone(),
                    book: book.clone(),
                    class_key,
                    class_human,
                    level: engine_level,
                    ours_dc: i16::from(ours_dc),
                });
                resolved = true;
                break;
            }
        }
        if !resolved {
            let attempted: Vec<&str> = mapped_named.iter().map(|s| s.as_str()).collect();
            let book_in_scope = ENGINE_SPELL_LIST_BOOKS.contains(&book.as_str());
            unverifiable_rows.push(UnverifiableRow {
                unit_id,
                kind: "spell".to_string(),
                reason: format!(
                    "no_engine_spell_list_entry: '{corpus_key}' names mapped class(es) \
                     [{}] (a real spell_save_dc formula exists for {} via \
                     casting_ability_for_class), but a live compute_spellbook_coverage call \
                     for every one of them produced no spells_prepared entry — \
                     resolve_<school>_spell_effect (src/rules_core/spellbook/*.rs) reads only \
                     core_rulebook/advanced_players_guide/advanced_class_guide's SPELL_LIST \
                     tables (confirmed by reading spellbook/illusion.rs's own widening \
                     comment), and this record's own book is '{book}'{}",
                    attempted.join(", "),
                    attempted.join("/"),
                    if book_in_scope {
                        " (which IS one of those three — a genuine, unexplained resolver miss, \
                          not merely a book-scope gap; worth a follow-up cycle's direct look)"
                    } else {
                        " (which is outside that three-book scope)"
                    }
                ),
            });
        }
    }

    let out = Output { spell: spell_rows, unverifiable: unverifiable_rows, spell_unresolved: Vec::new() };
    let json = serde_json::to_string_pretty(&out).expect("serializes");
    std::fs::write(output_path, json).unwrap_or_else(|e| panic!("writing {output_path}: {e}"));
    eprintln!(
        "fixture_verified_oracle_probe --remainder: spell={} unverifiable={} -> {output_path}",
        out.spell.len(),
        out.unverifiable.len()
    );
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
    let mut remainder_output_path: Option<String> = None;
    let mut inventory_path = repo_root.join("docs/work-inventory.json");
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output" => {
                output_path = args.get(i + 1).cloned();
                i += 2;
            }
            "--remainder" => {
                remainder_output_path = args.get(i + 1).cloned();
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

    if let Some(remainder_output_path) = remainder_output_path {
        run_remainder_mode(&repo_root, &inventory_path, &remainder_output_path);
        return;
    }

    let Some(output_path) = output_path else {
        eprintln!(
            "usage: fixture_verified_oracle_probe --output <path> [--inventory <path>]\n   or: fixture_verified_oracle_probe --remainder <path> [--inventory <path>]"
        );
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
                        selected_traits: Vec::new(),
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
