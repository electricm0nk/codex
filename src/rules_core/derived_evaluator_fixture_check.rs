//! Shared logic for the `derived` wiring class's evaluator-vs-fixture check.
//!
//! Extracted from `tests/derived_evaluator_fixture_check.rs` (operator
//! directive 2026-08-13, "add the done rung for static and derived") so
//! `v06_work_inventory` can emit the SAME evidence the test suite proves,
//! rather than inventing a second implementation of the bar. The test file
//! keeps its own guarantee-3/guarantee-4 provenance checks (which do not
//! belong in a report the generator consumes) and now calls
//! [`run_bar_check`] for the bar itself, so the two can never drift apart.
//!
//! The bar, precisely: the engine's evaluator, run over the real corpus
//! record, must produce exactly the value the fixture's pinned corpus
//! field states. See `tests/derived_evaluator_fixture_check.rs`'s module
//! doc for the four independent guarantees the original (`kind=equipment`,
//! `compute_equipment_effects`) seam rests on — the `kind=monster`
//! (`spell_like_ability_caster_level`, SD31-E6-F11-002), `kind=spell`
//! `DURATION:` (`parse_caster_level_linear_duration`, SD31-E6-F2-006) and
//! `kind=spell` `RANGE:` (`spell_range_formula`, SD31-E6-F2-008) seams
//! added since restate the same four guarantees for their own
//! evaluator/fixture pair rather than reusing that module's
//! equipment-specific test names.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
use crate::rules_core::corpus_loader::{BookCorpusRoot, load_equipment_corpus};
use crate::rules_core::equipment_effects::compute_equipment_effects;
use crate::rules_core::pilot_compute::UNDINE_RACE_TRAIT_FORMULAS;
use crate::rules_core::pilot_compute::formula_interpreter::PcgenFormulaEvaluator;
use crate::rules_core::pilot_compute::formula_reproduction_harness::FormulaEvaluator;
use crate::rules_core::rules_tables::companion_chassis::companion_book;
use crate::rules_core::rules_tables::monster_chassis::{MonsterStatBlock, MONSTER_BOOKS};

pub const FIXTURE_RELATIVE_PATH: &str = "tests/fixtures/rules_core/derived-evaluator-fixtures.json";

/// One fixture row, in the shape the committed JSON carries.
#[derive(Debug, Clone)]
pub struct Fixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub corpus_field: String,
    pub expected_abilities: Vec<String>,
    pub expected_bonus: i16,
}

/// Reads the committed fixture. Panics on a missing/malformed file -- the
/// same "a hand-invented input must never look like evidence" posture the
/// test suite takes, and correct for a report a `--json-out` consumer trusts.
pub fn load_fixtures(repo_root: &Path) -> Vec<Fixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let entries = doc["entries"].as_array().expect("fixture carries an `entries` array");
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            Fixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                expected_abilities: expected["abilities"]
                    .as_array()
                    .expect("expected.abilities")
                    .iter()
                    .map(|a| a.as_str().expect("ability name").to_string())
                    .collect(),
                expected_bonus: i16::try_from(
                    expected["bonus"].as_i64().expect("expected.bonus"),
                )
                .expect("an ability bonus fits in i16"),
            }
        })
        .collect()
}

/// Where this repo's own ingest of `book` lives, and whether it exists.
/// Absence is a real, structural fact -- a book nobody has ingested has no
/// records for any evaluator to run over.
fn ingested_equipment_dir(repo_root: &Path, book: &str) -> Option<PathBuf> {
    let dir = repo_root.join("data").join("corpus").join(book);
    dir.join("equipment").is_dir().then_some(dir)
}

/// The result of running the `derived` bar check over every fixture entry.
pub struct BarCheckReport {
    /// `unit_id`s whose evaluator output matched the fixture's expected
    /// value exactly -- the ONLY units this instrument licenses moving to
    /// `fixture-verified`.
    pub cleared: BTreeSet<String>,
    /// `unit_id` -> reason, for every covered entry that did NOT clear
    /// (evaluator mismatch, no resolution, or no ability bonus produced).
    pub failures: BTreeMap<String, String>,
    /// `unit_id` -> book, for entries whose book has no ingest in this repo
    /// at all -- distinct from a failure because the evaluator was never
    /// reached.
    pub engine_does_not_hold: BTreeMap<String, String>,
    pub fixtures_total: usize,
}

/// Runs the `derived` bar over every fixture entry, exactly as
/// `tests/derived_evaluator_fixture_check.rs::engine_evaluator_output_equals_the_corpus_derived_expected_value`
/// does, factored out so both the test and `v06_work_inventory` call the
/// same code.
pub fn run_bar_check(repo_root: &Path) -> BarCheckReport {
    let equipment = run_equipment_bar_check(repo_root);
    let monster = run_monster_bar_check(repo_root);
    let monster_sla = run_monster_sla_bar_check(repo_root);
    let spell = run_spell_bar_check(repo_root);
    let spell_range = run_spell_range_bar_check(repo_root);
    let class_feature = run_class_feature_bar_check(repo_root);
    let monster_ability = run_monster_ability_bar_check(repo_root);
    let monster_ability_formula = run_monster_ability_formula_bar_check(repo_root);
    let companion = run_companion_bar_check(repo_root);
    let companion_skill = run_companion_skill_bar_check(repo_root);
    let companion_save_dc = run_companion_save_dc_bar_check(repo_root);
    let class_feature_description = run_class_feature_description_bar_check(repo_root);
    let race_trait_formula = run_race_trait_formula_bar_check(repo_root);
    let mut cleared = equipment.cleared;
    cleared.extend(monster.cleared);
    cleared.extend(monster_sla.cleared);
    cleared.extend(spell.cleared);
    cleared.extend(spell_range.cleared);
    cleared.extend(class_feature.cleared);
    cleared.extend(monster_ability.cleared);
    cleared.extend(monster_ability_formula.cleared);
    cleared.extend(companion.cleared);
    cleared.extend(companion_skill.cleared);
    cleared.extend(companion_save_dc.cleared);
    cleared.extend(class_feature_description.cleared);
    cleared.extend(race_trait_formula.cleared);
    let mut failures = equipment.failures;
    failures.extend(monster.failures);
    failures.extend(monster_sla.failures);
    failures.extend(spell.failures);
    failures.extend(spell_range.failures);
    failures.extend(class_feature.failures);
    failures.extend(monster_ability.failures);
    failures.extend(monster_ability_formula.failures);
    failures.extend(companion.failures);
    failures.extend(companion_skill.failures);
    failures.extend(companion_save_dc.failures);
    failures.extend(class_feature_description.failures);
    failures.extend(race_trait_formula.failures);
    let mut engine_does_not_hold = equipment.engine_does_not_hold;
    engine_does_not_hold.extend(monster.engine_does_not_hold);
    engine_does_not_hold.extend(monster_sla.engine_does_not_hold);
    engine_does_not_hold.extend(spell.engine_does_not_hold);
    engine_does_not_hold.extend(spell_range.engine_does_not_hold);
    engine_does_not_hold.extend(class_feature.engine_does_not_hold);
    engine_does_not_hold.extend(monster_ability.engine_does_not_hold);
    engine_does_not_hold.extend(monster_ability_formula.engine_does_not_hold);
    engine_does_not_hold.extend(companion.engine_does_not_hold);
    engine_does_not_hold.extend(companion_skill.engine_does_not_hold);
    engine_does_not_hold.extend(companion_save_dc.engine_does_not_hold);
    engine_does_not_hold.extend(class_feature_description.engine_does_not_hold);
    engine_does_not_hold.extend(race_trait_formula.engine_does_not_hold);
    // A unit that FAILED any seam must never be reported cleared by another
    // one. `cleared` is a union across seams and `failures` is keyed by
    // `unit_id`, so a unit covered by two seams could otherwise be stamped on
    // the strength of the seam it passed while the seam it failed only ever
    // showed up in a report nothing reads. Subtracting here keeps
    // `apply_done_rung_stamps`'s input honest for every seam added later, not
    // just today's.
    for id in failures.keys().chain(engine_does_not_hold.keys()) {
        cleared.remove(id);
    }
    BarCheckReport {
        cleared,
        failures,
        engine_does_not_hold,
        fixtures_total: equipment.fixtures_total
            + monster.fixtures_total
            + monster_sla.fixtures_total
            + spell.fixtures_total
            + spell_range.fixtures_total
            + class_feature.fixtures_total
            + monster_ability.fixtures_total
            + monster_ability_formula.fixtures_total
            + companion.fixtures_total
            + companion_skill.fixtures_total
            + class_feature_description.fixtures_total
            + race_trait_formula.fixtures_total,
    }
}

/// The `kind=equipment` half of [`run_bar_check`] — the original, sole
/// implementation before the `kind=monster` seam below existed. Unchanged
/// in behaviour; only its name moved, to make room for the merge.
fn run_equipment_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_fixtures(repo_root);
    let fixtures_total = fixtures.len();
    let books: BTreeSet<String> = fixtures.iter().map(|f| f.book.clone()).collect();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for book in &books {
        let Some(dir) = ingested_equipment_dir(repo_root, book) else {
            for f in fixtures.iter().filter(|f| &f.book == book) {
                engine_does_not_hold.insert(f.unit_id.clone(), book.clone());
            }
            continue;
        };
        let roots = [BookCorpusRoot { book_id: book.as_str(), dir: Path::new(&dir) }];
        let corpus = load_equipment_corpus(&roots);

        for fixture in fixtures.iter().filter(|f| &f.book == book) {
            let selection = vec![EquipmentSelection {
                item_id: fixture.record_key.clone(),
                equipped_or_active: true,
                active_state: ActiveState::EquippedActive,
                applied_modifiers: Vec::new(),
            }];
            let effects = compute_equipment_effects(&selection, &corpus);
            let Some(item) = effects.per_item.first() else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!("{:?} does not resolve against its own ingested book", fixture.record_key),
                );
                continue;
            };
            let Some(bonus) = &item.ability_bonus else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} but the evaluator produced no ability bonus at all",
                        fixture.corpus_field
                    ),
                );
                continue;
            };
            let abilities: Vec<String> =
                bonus.ability.split(',').map(str::trim).map(str::to_string).collect();
            if abilities != fixture.expected_abilities || bonus.bonus != fixture.expected_bonus {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row {:?} states {:?} {:+}, evaluator produced {:?} {:+}",
                        fixture.corpus_field,
                        fixture.expected_abilities,
                        fixture.expected_bonus,
                        abilities,
                        bonus.bonus
                    ),
                );
                continue;
            }
            cleared.insert(fixture.unit_id.clone());
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

// ---------------------------------------------------------------------------
// `kind = monster` — the missing evaluator seam this cycle (SD31-E6-F11-002)
// builds. Everything below is new; nothing above this line changed shape.
// ---------------------------------------------------------------------------

/// PF1's own "Spell-Like Abilities" universal monster rule (`Bestiary`
/// Appendix 1, "Universal Monster Rules"; verified against the public PRD
/// mirror `legacy.aonprd.com/bestiary/universalMonsterRules.html`, "Spell-
/// Like Abilities"): *"Unless otherwise noted... the creature's caster
/// level is equal to its Hit Dice."* PCGen encodes this per-monster with
/// `BONUS:VAR|SLA_CL|HD` (or, equivalently, `BONUS:VAR|SLA_CL|max(TL,1)` --
/// `TL` is PCGen's own internal alias for the same total-Hit-Dice value,
/// floored at 1 so a 0-HD edge case still has a caster level) inside every
/// `SPELLS:Innate|...|CASTERLEVEL=(max(TL,1))|...` line the row carries.
///
/// This repo's monster ingest does not carry a dedicated Hit Dice field
/// (`completeness: "chassis_only"`, `docs/release/SD-31-corpus-closure-
/// grind/artifacts/SD31-E6-F11-001-held-cell-map.md`) -- but the SAME
/// integer is already captured as the trailing segment of the
/// `MONSTERCLASS:<type>:<HD>` token every monster row carries
/// ([`MonsterStatBlock::monster_class`], e.g. `"Outsider (Fort/Will):20"`
/// for a 20-HD outsider). This function reads that trailing integer and
/// applies the rule above -- a genuine derivation (parse + rule
/// application), never a literal transcription: `monster_class`'s trailing
/// number and `challenge_rating` routinely differ (Linnorm (Crag) is
/// `MONSTERCLASS:Dragon:15` at `CR:14` -- 15 HD, not 14), so this is not a
/// disguised copy of a field the corpus already states as XP/SLA level.
///
/// Returns `None` when the monster carries no `MONSTERCLASS:` token at all,
/// or when its trailing segment is not a plain integer (e.g. a book that
/// spells it differently) -- an honest absence, never a guessed value.
///
/// **SD31-E6-F9-003: the rule's own "unless otherwise noted" clause is not
/// decorative.** A monster's `BONUS:VAR|SLA_CL|<value>` token
/// ([`MonsterStatBlock::sla_cl_token`]) states EITHER the generic rule
/// (`HD`, or the equivalent `max(TL,1)`/`(max(TL,1))`) OR a monster-specific
/// literal override -- Couatl carries `BONUS:VAR|SLA_CL|9` against 12 Hit
/// Dice; Demon (Glabrezu) carries `14` against 12 HD. Before this function
/// read `sla_cl_token` it always applied the generic HD rule regardless,
/// which silently served the WRONG caster level for every monster whose row
/// carries an override -- re-derived corpus-wide this cycle: of the 71
/// previously-uncovered `derived`+`grounded` `monster` units this cycle
/// fixtured, 66 carry a literal override and only 5 carry the bare
/// `HD`/`max(TL,1)` spelling the function used to assume unconditionally.
pub fn spell_like_ability_caster_level(monster: &MonsterStatBlock) -> Option<i32> {
    // SD31-E6-F1-002 (`OPEN-ISSUES.md` row 44): a monster with no
    // `BONUS:VAR|SLA_CL|` token has no spell-like abilities, and this
    // function has a real production caller now
    // (`apps/desktop/src-tauri/src/monster_catalog.rs`) that would otherwise
    // hand every monster with a readable `MONSTERCLASS:` a caster level it
    // has no use for -- a number on a screen with nothing to attach it to.
    // `has_spell_like_abilities` is a row-presence check (`MonsterStatBlock`'s
    // own doc comment), never a guess.
    if !monster.has_spell_like_abilities {
        return None;
    }
    let monster_class = monster.monster_class?;
    let hd_str = monster_class.rsplit(':').next()?;
    let hd = hd_str.trim().parse::<i32>().ok()?;

    match monster.sla_cl_token {
        // The two corpus-observed spellings of "apply the generic Universal
        // Monster Rule" (`dragon_magma`'s row wraps the second in a
        // redundant extra paren pair; both mean the same thing) -- and,
        // defensively, `None` (a book whose transcription predates this
        // field), so a monster this repo has not yet re-transcribed keeps
        // its prior, already-correct-for-the-bare-HD-population behaviour
        // rather than silently losing its caster level.
        Some("HD") | Some("max(TL,1)") | Some("(max(TL,1))") | None => Some(hd),
        // Every other value is the row's own STATED override -- trust the
        // corpus over the generic rule. A plain integer parses directly.
        //
        // W26-INTERPRETER-INTEGRATE (`OPERATOR-RULINGS-2026-08-21.md` §20):
        // a value that is not a plain integer is no longer an automatic
        // refusal -- `formula_interpreter::PcgenFormulaEvaluator` reads real
        // PCGen arithmetic now, and `HD*3/4`
        // (`book_of_the_damned_volume_2`'s Demon (Vermlek)) is exactly such
        // a formula: multiply and divide over the monster's OWN Hit Dice,
        // which this function already read two lines above to apply the
        // generic rule. `TL` is bound to the same value as `HD` (PCGen's own
        // `TL` == "total levels", which for a monster with only a
        // `MONSTERCLASS:` token and no PC class levels sums to exactly the
        // racial HD -- the same equivalence
        // `monster_ability_formula_save_dc`'s own `parse_formula_base_plus_
        // ability` already establishes and cites for this corpus). The
        // interpreter refuses -- returns `Err`, never a guess -- on any
        // identifier this repo has not bound (a race-specific bonus name
        // with no `DEFINE:` on the row, say), so `.ok()` below is still the
        // same honest-absence contract every other arm of this function
        // keeps: `Some(value)` only when the formula both parses AND
        // evaluates against the two variables this function can honestly
        // supply.
        Some(raw) => raw.trim().parse::<i32>().ok().or_else(|| {
            let vars = BTreeMap::from([("HD".to_string(), i64::from(hd)), ("TL".to_string(), i64::from(hd))]);
            PcgenFormulaEvaluator.evaluate(raw.trim(), &vars).ok().and_then(|v| i32::try_from(v).ok())
        }),
    }
}

/// One `kind=monster` fixture row. Deliberately a different shape from
/// [`Fixture`] (equipment's `expected.abilities`/`expected.bonus` has no
/// meaning for a monster's spell-like-ability caster level) rather than a
/// forced-generic union — see `monster_entries` in the committed fixture
/// JSON, a sibling top-level array to `entries`, not a variant folded into
/// it, so the equipment loader above needs no change at all to keep working.
#[derive(Debug, Clone)]
pub struct MonsterFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    /// Path of the upstream PCGen `.lst`, relative to the PCGen `data/`
    /// root — spelled exactly the way this repo's own corpus records spell
    /// their `source.path`.
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    pub corpus_field: String,
    pub monster_class_token: String,
    pub expected_spell_like_ability_caster_level: i32,
}

/// Reads the `monster_entries` array of the same committed fixture file
/// [`load_fixtures`] reads `entries` from.
pub fn load_monster_fixtures(repo_root: &Path) -> Vec<MonsterFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("monster_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            MonsterFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                monster_class_token: e["monster_class_token"]
                    .as_str()
                    .expect("monster_class_token")
                    .to_string(),
                expected_spell_like_ability_caster_level: i32::try_from(
                    expected["spell_like_ability_caster_level"]
                        .as_i64()
                        .expect("expected.spell_like_ability_caster_level"),
                )
                .expect("a caster level fits in i32"),
            }
        })
        .collect()
}

/// The one alias this seam needs: `monster_chassis::MONSTER_BOOKS` keys
/// Bestiary 1 by its actual corpus directory, `beastiary` (spelled that way
/// since SD-22; see the registry's own comment on that entry), while the
/// work-inventory `book` field for the exact same records is `bestiary`
/// (the engine-facing id `v06_work_inventory` and this fixture's `unit_id`s
/// both use). Every other registered monster book's corpus directory and
/// work-inventory `book` field are identical strings, so this alias is the
/// one exception, not a general translation table.
fn monster_registry_book(book: &str) -> &str {
    match book {
        "bestiary" => "beastiary",
        other => other,
    }
}

/// The `kind=monster` half of [`run_bar_check`]. Resolves each fixture
/// entry through the SAME `monster_chassis::MONSTER_BOOKS` registry
/// `v06_work_inventory`'s own `grounded` verdict for `monster` already
/// reads (`monster_resolve_returned_a_real_stat_block`) -- this is not a
/// second, parallel monster table, it is the one the engine already serves
/// to the desktop app's monster catalog.
fn run_monster_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_monster_fixtures(repo_root);
    let fixtures_total = fixtures.len();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for fixture in &fixtures {
        let registry_book = monster_registry_book(&fixture.book);
        let Some(monster_book) = MONSTER_BOOKS.iter().find(|b| b.corpus_book == registry_book)
        else {
            engine_does_not_hold.insert(fixture.unit_id.clone(), fixture.book.clone());
            continue;
        };
        // Resolved by `.key` (the corpus `KEY:` identity, == `record_key` ==
        // `data.corpus_key` in this repo's own `data/corpus/**/monster/*.json`
        // ingest -- `MonsterBook::monster_resolve`'s own contract), never by
        // `.name` (SD31-E6-F9-003 fix): the two coincide for every one of
        // this seam's original 7 fixtures (Demon (Balor) etc., where
        // `key == name`) but genuinely differ for records like Bestiary 4's
        // Gremlin (Grimple), whose `key` is `"Gremlin (Grimple)"` and whose
        // `name` is the bare `"Grimple"` -- matching on `.name` there is a
        // silent false-negative, not merely a style choice, and it is what
        // `monster_ingested_provenance` in
        // `tests/derived_evaluator_fixture_check_monster.rs`'s own
        // independent guarantee-4b check has always assumed.
        let Some(monster) = monster_book.monster_resolve(&fixture.record_key) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "{:?} does not resolve against {registry_book}'s registered monsters",
                    fixture.record_key
                ),
            );
            continue;
        };
        match spell_like_ability_caster_level(monster) {
            None => {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} ({}) but the evaluator produced no caster level \
                         at all",
                        fixture.corpus_field, fixture.monster_class_token
                    ),
                );
            }
            Some(cl) if cl == fixture.expected_spell_like_ability_caster_level => {
                cleared.insert(fixture.unit_id.clone());
            }
            Some(cl) => {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} ({}), expected caster level {}, evaluator \
                         produced {}",
                        fixture.corpus_field,
                        fixture.monster_class_token,
                        fixture.expected_spell_like_ability_caster_level,
                        cl
                    ),
                );
            }
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

// ---------------------------------------------------------------------------
// `kind = spell` — the missing evaluator seam this cycle (SD31-E6-F2-006)
// builds. Everything below is new; nothing above this line changed shape.
// ---------------------------------------------------------------------------

/// A PF1 caster-level-LINEAR duration formula: `(CASTERLEVEL)` or
/// `(CASTERLEVEL*N)` followed by a literal trailing unit. `OPEN-ISSUES.md`
/// row 119 traced `advanced_class_guide:spell:adhesive_blood`
/// (`DURATION:(CASTERLEVEL) minutes`) end to end and found this exact shape
/// is the dominant held-`spell` population (1046 of 1161 corpus-wide
/// `DURATION`+`CASTERLEVEL` tokens, re-derived at this cycle's tip).
///
/// **What this deliberately does NOT do.** A spell's DURATION scales with
/// the CASTING CHARACTER's caster level, which no corpus row states (unlike
/// a monster's own Hit Dice, `spell_like_ability_caster_level`'s bar) --
/// resolving `(CASTERLEVEL)` to a single number of minutes/rounds/etc.
/// without a live character would be exactly the fabrication
/// `SD31-E6-F1-002` already correctly refused for the monster
/// ability-score-scaling family. This function computes something
/// genuinely corpus-grounded instead: the FORMULA's own parameters
/// (multiplier + unit), independent of any live caster level, matching
/// `render_pcgen_desc`'s own existing policy of never resolving a
/// `CASTERLEVEL` argument tail (`src/rules_core/pcgen_desc.rs`'s
/// `dropped_args`) -- this is a structural derivation, not a resolved
/// magnitude.
#[derive(Debug, Clone, PartialEq)]
pub struct CasterLevelLinearFormula {
    pub per_level: i32,
    pub unit: String,
}

/// Parses `raw` (a corpus `DURATION:` token's value) for the single shape
/// this seam commits to. Refuses (returns `None`) on anything else --
/// `min(`/`max(` clamps, an additive constant, an "instantaneous or ..."
/// alternation, or a second `CASTERLEVEL` occurrence inside the trailing
/// unit text -- rather than guess. 115 of 1161 corpus-wide `DURATION`+
/// `CASTERLEVEL` tokens carry one of those shapes and are correctly refused
/// (`scripts/derive_spell_caster_level_duration_fixtures.py`'s own
/// `skipped_complex` count, re-derived at this cycle's tip: 449, which also
/// includes every `derived`-held spell whose magnitude is NOT a `DURATION`
/// formula at all -- `range_keyword`/`TARGETAREA`-shaped units this seam
/// does not attempt).
pub fn parse_caster_level_linear_duration(raw: &str) -> Option<CasterLevelLinearFormula> {
    let raw = raw.trim();
    let rest = raw.strip_prefix("(CASTERLEVEL")?;
    let (coefficient, rest) = if let Some(after_close) = rest.strip_prefix(')') {
        (1, after_close)
    } else {
        let after_star = rest.trim_start().strip_prefix('*')?;
        let after_star = after_star.trim_start();
        let close_idx = after_star.find(')')?;
        let (num_str, after_close) = after_star.split_at(close_idx);
        let n: i32 = num_str.trim().parse().ok()?;
        (n, &after_close[1..])
    };
    let unit = rest.trim();
    if unit.is_empty() || unit.contains("CASTERLEVEL") {
        return None;
    }
    Some(CasterLevelLinearFormula { per_level: coefficient, unit: unit.to_string() })
}

/// Renders a [`CasterLevelLinearFormula`] as player-facing text. A literal,
/// non-interpretive restatement of the formula ("N <unit> per caster
/// level") -- never the invented "official" Paizo phrasing this crate
/// cannot verify against a licensed source; see
/// `docs/governance/no-stub-mvp-doctrine.md`.
pub fn format_caster_level_linear_duration(formula: &CasterLevelLinearFormula) -> String {
    format!("{} {} per caster level", formula.per_level, formula.unit)
}

/// One `kind=spell` fixture row. See `spell_entries` in the committed
/// fixture JSON and its sibling `spell_derivation`/`spell_independence`
/// fields for the full provenance contract this mirrors from
/// [`MonsterFixture`].
#[derive(Debug, Clone)]
pub struct SpellFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    pub corpus_field: String,
    pub expected_per_level: i32,
    pub expected_unit: String,
}

/// Reads the `spell_entries` array of the committed fixture file.
pub fn load_spell_fixtures(repo_root: &Path) -> Vec<SpellFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("spell_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            SpellFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                expected_per_level: i32::try_from(
                    expected["per_level"].as_i64().expect("expected.per_level"),
                )
                .expect("a per-level multiplier fits in i32"),
                expected_unit: expected["unit"].as_str().expect("expected.unit").to_string(),
            }
        })
        .collect()
}

/// `SpellCatalogRow.book`/work-inventory `book` field -> its
/// `data/corpus/<dir>/spell/` directory name. Restates
/// `spell_resolver::spell_catalog_rows()`'s own 8-book chain (see that
/// module's doc comment) -- this seam reads the JSON cache those books
/// share, mirroring how the equipment half of this file reads
/// `data/corpus/<book>/equipment/` rather than the compiled table.
fn spell_corpus_dir_exists(repo_root: &Path, book: &str) -> Option<PathBuf> {
    let dir = repo_root.join("data").join("corpus").join(book);
    dir.join("spell").is_dir().then_some(dir)
}

/// Walks `data/corpus/<book>/spell/` once (it is nested by spell level for
/// some books, flat for others -- `WalkDir`-free recursive walk handles
/// both without assuming a depth) and returns every record's `DURATION:`
/// raw token, keyed by the record's `data.key` -- the same identity
/// [`SpellFixture::record_key`] and `SpellCatalogRow.key` both carry.
fn load_spell_durations(spell_dir: &Path) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut stack = vec![spell_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(read_dir) = std::fs::read_dir(&dir) else { continue };
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            let Some(key) = doc["data"]["key"].as_str() else { continue };
            let Some(tokens) = doc["data"]["raw_tokens"].as_array() else { continue };
            for t in tokens {
                if t["key"].as_str() == Some("DURATION") {
                    if let Some(v) = t["value"].as_str() {
                        out.insert(key.to_string(), v.to_string());
                    }
                    break;
                }
            }
        }
    }
    out
}

/// The 8 books `spell_resolver::spell_catalog_rows()` chains, as
/// `data/corpus/` directory names -- the set [`all_spell_caster_level_durations`]
/// walks. Restated here (rather than imported from `spell_resolver`, a
/// different crate module with its own `SPELL_BOOK_*` short-code
/// constants) because this list names `data/corpus/` directory names, not
/// wire-form short codes; the two are related by
/// [`spell_book_corpus_dir_for_short_code`] below.
const SPELL_CORPUS_BOOK_DIRS: &[&str] = &[
    "core_rulebook",
    "advanced_players_guide",
    "advanced_class_guide",
    "advanced_race_guide",
    "ultimate_intrigue",
    "ultimate_magic",
    "occult_adventures",
    "ultimate_combat",
    // W19-INTEGRATE: `inner_sea_gods` was found live, already carrying a
    // real `data/corpus/inner_sea_gods/spell/` cache (92 files, already
    // `raw_tokens`-enriched by `enrich_spell_raw_tokens.rs`'s own
    // `TARGET_BOOKS` since `SD31-E6-F10-001`) but MISSING from this list --
    // the exact same silent-gap shape the `ultimate_wilderness` entry below
    // fixes, discovered while fixing it. Neither this book's own `spell`
    // units nor a coverage test caught the gap before now; see
    // `spell_book_corpus_dir_coverage_tests` below.
    "inner_sea_gods",
    // Widened 8 -> 10 (W19-INTEGRATE): `ultimate_wilderness` gained a real
    // `data/corpus/ultimate_wilderness/spell/` cache this wave
    // (`cache_gen::spell_lane_dump`, wave-19 `ultimate_wilderness` lane +
    // integration-cycle follow-up) -- without this entry none of its 61
    // spell units could ever reach the `literal-verified`/`fixture-verified`
    // `done` rung, no matter how complete their data was (adversarial
    // review, confirmed finding).
    "ultimate_wilderness",
];

/// `spell_resolver::SPELL_BOOK_*` wire-form short code (`"CRB"`, `"APG"`,
/// ...) -> its `data/corpus/<dir>/spell/` directory name. Mirrors
/// [`monster_registry_book`]'s role for the monster seam: the one
/// alias table a consumer needs to cross from the wire-form identity to
/// the on-disk one.
pub fn spell_book_corpus_dir_for_short_code(short_code: &str) -> Option<&'static str> {
    match short_code {
        "CRB" => Some("core_rulebook"),
        "APG" => Some("advanced_players_guide"),
        "ACG" => Some("advanced_class_guide"),
        "ARG" => Some("advanced_race_guide"),
        "UI" => Some("ultimate_intrigue"),
        "UM" => Some("ultimate_magic"),
        "OA" => Some("occult_adventures"),
        "UC" => Some("ultimate_combat"),
        "ISG" => Some("inner_sea_gods"),
        "UW" => Some("ultimate_wilderness"),
        _ => None,
    }
}

/// W19-INTEGRATE (adversarial review, confirmed finding): unlike
/// `v06_work_inventory::spell_book_slug_for` (its own dedicated
/// `spell_book_slug_for_covers_every_catalog_book` test), this sibling
/// lookup returns `Option` and silently yields `None` for an unmapped book
/// code -- `apps/desktop/src-tauri/src/spell_catalog.rs`'s
/// `duration_for()`/`range_for()` then serve `null` for every row of that
/// book with no gate ever firing. This closed-set-coverage test is that
/// missing gate: it must be updated in the SAME commit that adds a book to
/// `spell_resolver::spell_catalog_rows()`, or it fails immediately.
#[cfg(test)]
mod spell_book_corpus_dir_coverage_tests {
    use super::spell_book_corpus_dir_for_short_code;

    #[test]
    fn every_catalog_book_short_code_resolves_a_corpus_dir() {
        // Mirrors `spell_resolver::SPELL_BOOK_*`'s full wire-form roster
        // rather than importing it, so this test does not silently pass by
        // construction if `spell_catalog_rows()` gains a book whose
        // constant this list forgets to restate.
        let codes = ["CRB", "APG", "ACG", "ARG", "UI", "UM", "OA", "UC", "ISG", "UW"];
        for code in codes {
            assert!(
                spell_book_corpus_dir_for_short_code(code).is_some(),
                "{code} has a spell_resolver::SPELL_BOOK_* constant but no \
                 spell_book_corpus_dir_for_short_code entry -- duration/range \
                 will silently serve null for every row of this book"
            );
        }
    }

    #[test]
    fn an_unmapped_code_yields_none_rather_than_a_guess() {
        assert_eq!(spell_book_corpus_dir_for_short_code("NOT-A-REAL-BOOK"), None);
    }
}

/// Every spell record, across all 8 ingested books, whose corpus `DURATION:`
/// token matches the caster-level-linear shape
/// [`parse_caster_level_linear_duration`] commits to -- keyed by
/// `(data/corpus book dir, record_key)`. Built for a real production
/// consumer (`apps/desktop/src-tauri/src/spell_catalog.rs`'s
/// `SpellCatalogEntryDto::duration`), so this walks every book once and
/// returns everything parseable, not only the units the committed fixture
/// batch happens to cover -- the fixture governs which units may be
/// counted `fixture-verified`/`done`; this function governs what a player
/// sees, and the two populations are allowed to differ (a record with no
/// fixture entry yet can still render its own honestly-parsed duration).
pub fn all_spell_caster_level_durations(
    repo_root: &Path,
) -> BTreeMap<(String, String), CasterLevelLinearFormula> {
    let mut out = BTreeMap::new();
    for book in SPELL_CORPUS_BOOK_DIRS {
        let Some(dir) = spell_corpus_dir_exists(repo_root, book) else { continue };
        for (key, raw) in load_spell_durations(&dir.join("spell")) {
            if let Some(formula) = parse_caster_level_linear_duration(&raw) {
                out.insert((book.to_string(), key), formula);
            }
        }
    }
    out
}

/// The `kind=spell` half of [`run_bar_check`]. Reads the SAME
/// `data/corpus/<book>/spell/` JSON cache the desktop app's spell catalog
/// is wired to read for its own `duration` field
/// (`apps/desktop/src-tauri/src/spell_catalog.rs`'s
/// `caster_level_duration_for`) -- this is not a second, parallel spell
/// table.
fn run_spell_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_spell_fixtures(repo_root);
    let fixtures_total = fixtures.len();
    let books: BTreeSet<String> = fixtures.iter().map(|f| f.book.clone()).collect();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for book in &books {
        let Some(dir) = spell_corpus_dir_exists(repo_root, book) else {
            for f in fixtures.iter().filter(|f| &f.book == book) {
                engine_does_not_hold.insert(f.unit_id.clone(), book.clone());
            }
            continue;
        };
        let durations = load_spell_durations(&dir.join("spell"));

        for fixture in fixtures.iter().filter(|f| &f.book == book) {
            let Some(raw) = durations.get(&fixture.record_key) else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "{:?} does not resolve against {book}'s ingested spell cache",
                        fixture.record_key
                    ),
                );
                continue;
            };
            match parse_caster_level_linear_duration(raw) {
                None => {
                    failures.insert(
                        fixture.unit_id.clone(),
                        format!(
                            "corpus row states {} but the evaluator produced no caster-level \
                             formula at all (raw DURATION: {raw:?})",
                            fixture.corpus_field
                        ),
                    );
                }
                Some(formula)
                    if formula.per_level == fixture.expected_per_level
                        && formula.unit == fixture.expected_unit =>
                {
                    cleared.insert(fixture.unit_id.clone());
                }
                Some(formula) => {
                    failures.insert(
                        fixture.unit_id.clone(),
                        format!(
                            "corpus row {:?} states {}/{:?}, evaluator produced {}/{:?}",
                            fixture.corpus_field,
                            fixture.expected_per_level,
                            fixture.expected_unit,
                            formula.per_level,
                            formula.unit
                        ),
                    );
                }
            }
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

/// A PF1 rules-defined `RANGE:` formula for the three caster-level-linear
/// SPELLRANGE keywords ("Close", "Medium", "Long"). Unlike
/// [`CasterLevelLinearFormula`] (a per-spell literal parsed out of that
/// spell's own `DURATION:` token, which can vary spell to spell), this
/// formula is a RULESET-level constant: every spell whose `RANGE:` token
/// names one of these three keywords shares the identical formula, stated
/// once by the pinned PCGen game mode itself --
/// `system/gameModes/Pathfinder/miscinfo.lst` (part of this repo's own
/// oracle pin, `scripts/pcgen-oracle-pin.env`'s `PCGEN_ORACLE_SPARSE_PATHS`
/// already covers `system/gameModes/Pathfinder`), re-derived 2026-08-17:
///
/// ```text
/// SPELLRANGE:CLOSE|floor(CASTERLEVEL/2)*5+25
/// SPELLRANGE:MEDIUM|(CASTERLEVEL*10)+100
/// SPELLRANGE:LONG|(CASTERLEVEL*40)+400
/// ```
///
/// A structural derivation, exactly like [`CasterLevelLinearFormula`]: it
/// states the formula's own base + rate, independent of any live caster
/// level, never a resolved feet-at-level-N number (matching
/// `render_pcgen_desc`'s own `CASTERLEVEL`-argument-drop policy, restated
/// at [`parse_caster_level_linear_duration`]'s own doc comment).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpellRangeFormula {
    pub base_ft: i32,
    pub rate_ft: i32,
    pub per_levels: i32,
}

/// Looks up the fixed [`SpellRangeFormula`] for a corpus `RANGE:` token's
/// value. Refuses (returns `None`) on anything other than the three exact
/// keywords the ruleset states a caster-level formula for --
/// `Personal`/`Touch`/a literal distance (`RANGE:30 ft.`)/`See text`/etc.
/// are real `RANGE:` shapes this seam does not attempt: `Personal`/`Touch`
/// carry no scaling at all, and a literal distance is already a resolved
/// number with no formula to state.
pub fn spell_range_formula(raw: &str) -> Option<SpellRangeFormula> {
    match raw.trim() {
        "Close" => Some(SpellRangeFormula { base_ft: 25, rate_ft: 5, per_levels: 2 }),
        "Medium" => Some(SpellRangeFormula { base_ft: 100, rate_ft: 10, per_levels: 1 }),
        "Long" => Some(SpellRangeFormula { base_ft: 400, rate_ft: 40, per_levels: 1 }),
        _ => None,
    }
}

/// Renders a [`SpellRangeFormula`] as player-facing text -- a literal,
/// non-interpretive restatement of the ruleset formula, never an invented
/// "official" Paizo phrasing this crate cannot verify against a licensed
/// source; see `docs/governance/no-stub-mvp-doctrine.md`.
pub fn format_spell_range_formula(formula: &SpellRangeFormula) -> String {
    if formula.per_levels == 1 {
        format!("{} ft. + {} ft. per caster level", formula.base_ft, formula.rate_ft)
    } else {
        format!(
            "{} ft. + {} ft. per {} caster levels",
            formula.base_ft, formula.rate_ft, formula.per_levels
        )
    }
}

/// One `kind=spell` `RANGE:` fixture row. Mirrors [`SpellFixture`]'s
/// provenance shape; `expected_*` restates the ruleset's own formula
/// (independently transcribed by `scripts/derive_spell_range_fixtures.py`
/// straight from the same pinned `miscinfo.lst` this module's doc comment
/// cites, never imported from this Rust code) so the bar check compares two
/// independent readings of the SAME oracle file, not the corpus row against
/// itself.
#[derive(Debug, Clone)]
pub struct SpellRangeFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    pub corpus_field: String,
    pub expected_base_ft: i32,
    pub expected_rate_ft: i32,
    pub expected_per_levels: i32,
}

/// Reads the `spell_range_entries` array of the committed fixture file.
/// Absent-key returns empty, same tolerant shape [`load_spell_fixtures`]
/// uses, so an older fixture file (before this seam existed) still parses.
pub fn load_spell_range_fixtures(repo_root: &Path) -> Vec<SpellRangeFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("spell_range_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            SpellRangeFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                expected_base_ft: i32::try_from(
                    expected["base_ft"].as_i64().expect("expected.base_ft"),
                )
                .expect("base_ft fits in i32"),
                expected_rate_ft: i32::try_from(
                    expected["rate_ft"].as_i64().expect("expected.rate_ft"),
                )
                .expect("rate_ft fits in i32"),
                expected_per_levels: i32::try_from(
                    expected["per_levels"].as_i64().expect("expected.per_levels"),
                )
                .expect("per_levels fits in i32"),
            }
        })
        .collect()
}

/// Walks `data/corpus/<book>/spell/` once and returns every record's
/// `RANGE:` raw token, keyed by the record's `data.key` -- the RANGE
/// sibling of [`load_spell_durations`], same recursive-walk shape.
fn load_spell_ranges(spell_dir: &Path) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut stack = vec![spell_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(read_dir) = std::fs::read_dir(&dir) else { continue };
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            let Some(key) = doc["data"]["key"].as_str() else { continue };
            let Some(tokens) = doc["data"]["raw_tokens"].as_array() else { continue };
            for t in tokens {
                if t["key"].as_str() == Some("RANGE") {
                    if let Some(v) = t["value"].as_str() {
                        out.insert(key.to_string(), v.to_string());
                    }
                    break;
                }
            }
        }
    }
    out
}

/// Every spell record, across all 8 ingested books, whose corpus `RANGE:`
/// token names one of the three caster-level-linear keywords
/// [`spell_range_formula`] resolves -- keyed by `(data/corpus book dir,
/// record_key)`. Built for a real production consumer
/// (`apps/desktop/src-tauri/src/spell_catalog.rs`'s
/// `SpellCatalogEntryDto::range`), same shape and same caveat as
/// [`all_spell_caster_level_durations`]: this walks every book once and
/// returns everything parseable, not only the fixture-covered subset.
pub fn all_spell_caster_level_ranges(repo_root: &Path) -> BTreeMap<(String, String), SpellRangeFormula> {
    let mut out = BTreeMap::new();
    for book in SPELL_CORPUS_BOOK_DIRS {
        let Some(dir) = spell_corpus_dir_exists(repo_root, book) else { continue };
        for (key, raw) in load_spell_ranges(&dir.join("spell")) {
            if let Some(formula) = spell_range_formula(&raw) {
                out.insert((book.to_string(), key), formula);
            }
        }
    }
    out
}

/// The `kind=spell` `RANGE:` half of [`run_bar_check`]. Reads the SAME
/// `data/corpus/<book>/spell/` JSON cache [`run_spell_bar_check`] and the
/// desktop app's spell catalog both read.
fn run_spell_range_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_spell_range_fixtures(repo_root);
    let fixtures_total = fixtures.len();
    let books: BTreeSet<String> = fixtures.iter().map(|f| f.book.clone()).collect();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for book in &books {
        let Some(dir) = spell_corpus_dir_exists(repo_root, book) else {
            for f in fixtures.iter().filter(|f| &f.book == book) {
                engine_does_not_hold.insert(f.unit_id.clone(), book.clone());
            }
            continue;
        };
        let ranges = load_spell_ranges(&dir.join("spell"));

        for fixture in fixtures.iter().filter(|f| &f.book == book) {
            let Some(raw) = ranges.get(&fixture.record_key) else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "{:?} does not resolve against {book}'s ingested spell cache",
                        fixture.record_key
                    ),
                );
                continue;
            };
            match spell_range_formula(raw) {
                None => {
                    failures.insert(
                        fixture.unit_id.clone(),
                        format!(
                            "corpus row states {} but the evaluator produced no caster-level \
                             range formula at all (raw RANGE: {raw:?})",
                            fixture.corpus_field
                        ),
                    );
                }
                Some(formula)
                    if formula.base_ft == fixture.expected_base_ft
                        && formula.rate_ft == fixture.expected_rate_ft
                        && formula.per_levels == fixture.expected_per_levels =>
                {
                    cleared.insert(fixture.unit_id.clone());
                }
                Some(formula) => {
                    failures.insert(
                        fixture.unit_id.clone(),
                        format!(
                            "corpus row {:?} states {}/{}/{}, evaluator produced {}/{}/{}",
                            fixture.corpus_field,
                            fixture.expected_base_ft,
                            fixture.expected_rate_ft,
                            fixture.expected_per_levels,
                            formula.base_ft,
                            formula.rate_ft,
                            formula.per_levels
                        ),
                    );
                }
            }
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

// ---------------------------------------------------------------------------
// `kind = class_feature` — the missing evaluator seam this cycle
// (SD31-E6-F11-003) builds. Everything below is new; nothing above this line
// changed shape.
//
// This is the seam `decisions.md`'s wave-12 finding named directly: Barbarian
// Superstition (`SD31-E4-F2-003`) was wired as a real production consumer
// (`pilot_compute::barbarian_superstition_save_bonus`) and still could not
// reach `done` -- it lands `derived`+`grounded`, and `doneness_verdict()`
// caps that at `held` without a `fixture-verified` stamp, which nothing
// before this seam could produce for `kind=class_feature`.
//
// The corpus states these formulas as PCGen `BONUS:VAR` arithmetic over a
// class-level variable this repo's `data/corpus/` ingest does NOT resolve to
// a literal number (it is the LIVE character's level in a base class, not a
// corpus-stated fact) -- so, exactly like `CasterLevelLinearFormula` and
// `SpellRangeFormula` above, this seam verifies the formula's own STRUCTURAL
// PARAMETERS (a floor-division coefficient and two additive offsets) against
// a hand-derived expectation, never a resolved live value.
// ---------------------------------------------------------------------------

/// A PF1 class-feature per-level scaling formula of the shape
/// `floor((LEVELVAR + offset_pre) / divisor) + offset_post`, re-derived
/// corpus-wide 2026-08-17 (`data/corpus/*/class_feature/**/*.json`'s
/// `BONUS:VAR` tokens, 23 corpus-wide records match this exact shape) as the
/// dominant scaling-formula family this kind's `derived`+`grounded` held
/// population carries. Every corpus-observed spelling reduces to these three
/// integers:
///
/// * `2+RagePowersLVL/4` (Rage Power ~ Superstition) → division binds
///   tighter than addition, so this is `offset_pre=0, divisor=4,
///   offset_post=2` -- the paren-free shape only ever adds AFTER the divide.
/// * `RogueTrapSenseLVL/3` (Rogue ~ Trap Sense) → no offset at all:
///   `offset_pre=0, divisor=3, offset_post=0`.
/// * `(RangerFavoredTerrainLVL+2)/5` (Ranger ~ Favored Terrain) → the parens
///   force the addition BEFORE the divide: `offset_pre=2, divisor=5,
///   offset_post=0`.
/// * `(BloodragerDRLVL-4)/3` (Bloodrager ~ Damage Reduction) → same
///   paren-before-divide shape with a negative pre-offset: `offset_pre=-4,
///   divisor=3, offset_post=0`.
/// * `SlayerStalkerLVL/5+1` (Slayer ~ Stalker) → the paren-free
///   divide-then-add shape again, offset written after the divide this time:
///   `offset_pre=0, divisor=5, offset_post=1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClassFeatureLevelScalingFormula {
    pub offset_pre: i32,
    pub divisor: i32,
    pub offset_post: i32,
}

/// Parses a corpus `BONUS:VAR|<name>|<formula>` token's formula half (the
/// text after the second `|`) for the shape
/// [`ClassFeatureLevelScalingFormula`] states. Returns the level-variable
/// NAME the formula divides (never resolved further -- see the module
/// section doc) alongside the parsed formula. Refuses (`None`) on anything
/// else: a `max(...)`/`min(...)` wrap, an ability-score term, or a formula
/// with no `/` at all are real corpus shapes this seam does not attempt
/// (`OPEN-ISSUES.md` follow-up), never guessed at.
pub fn parse_class_feature_level_scaling(
    raw: &str,
) -> Option<(String, ClassFeatureLevelScalingFormula)> {
    let raw = raw.trim();

    // `(<VAR><+|-><N>)/<D>` -- the parenthesised, divide-the-sum shape.
    if let Some(rest) = raw.strip_prefix('(') {
        let close = rest.find(')')?;
        let inner = &rest[..close];
        let after_paren = rest[close + 1..].trim();
        let divisor: i32 = after_paren.strip_prefix('/')?.trim().parse().ok()?;
        if divisor == 0 {
            return None;
        }
        let split_at = inner.rfind(['+', '-'])?;
        if split_at == 0 {
            return None; // a leading sign belongs to the variable, not an operator
        }
        let var = &inner[..split_at];
        let offset: i32 = inner[split_at..].parse().ok()?;
        if var.is_empty() || !is_valid_var_name(var) {
            return None;
        }
        return Some((
            var.to_string(),
            ClassFeatureLevelScalingFormula { offset_pre: offset, divisor, offset_post: 0 },
        ));
    }

    // `<N>+<VAR>/<D>` -- a bare leading integer added AFTER the divide
    // (division binds tighter than `+` with no parens present).
    if let Some(plus_idx) = raw.find('+') {
        let before = raw[..plus_idx].trim();
        let after = raw[plus_idx + 1..].trim();
        if let Ok(n) = before.parse::<i32>() {
            let (var, divisor) = parse_var_slash_int(after)?;
            return Some((
                var,
                ClassFeatureLevelScalingFormula { offset_pre: 0, divisor, offset_post: n },
            ));
        }
    }

    // `<VAR>/<D>` or `<VAR>/<D>+<N>` -- the plain and divide-then-add shapes.
    let slash_idx = raw.find('/')?;
    let var = raw[..slash_idx].trim();
    if var.is_empty() || !is_valid_var_name(var) {
        return None;
    }
    let rest = raw[slash_idx + 1..].trim();
    if let Some(plus_idx) = rest.find('+') {
        let divisor: i32 = rest[..plus_idx].trim().parse().ok()?;
        let offset_post: i32 = rest[plus_idx + 1..].trim().parse().ok()?;
        if divisor == 0 {
            return None;
        }
        return Some((
            var.to_string(),
            ClassFeatureLevelScalingFormula { offset_pre: 0, divisor, offset_post },
        ));
    }
    let divisor: i32 = rest.parse().ok()?;
    if divisor == 0 {
        return None;
    }
    Some((var.to_string(), ClassFeatureLevelScalingFormula { offset_pre: 0, divisor, offset_post: 0 }))
}

fn parse_var_slash_int(s: &str) -> Option<(String, i32)> {
    let slash_idx = s.find('/')?;
    let var = s[..slash_idx].trim();
    let divisor: i32 = s[slash_idx + 1..].trim().parse().ok()?;
    if var.is_empty() || !is_valid_var_name(var) || divisor == 0 {
        return None;
    }
    Some((var.to_string(), divisor))
}

/// A PCGen `VAR` name: letters, digits, underscore only. Guards every branch
/// above against silently accepting a fragment of a formula shape this seam
/// does not understand (a stray operator, a function call) as if it were a
/// bare variable name.
fn is_valid_var_name(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// One `kind=class_feature` fixture row. `class_level_alias` is the
/// declared-in-the-corpus RHS of a SIBLING `BONUS:VAR|<level_var>|<alias>`
/// token -- resolved by [`run_class_feature_bar_check`] by searching every
/// record in the SAME book (the level-variable's own definition is not
/// always on the same record as the formula that consumes it: `RagePowersLVL`
/// is defined on the `Barbarian ~ Rage Powers` pool-header ability and
/// consumed by `Rage Power ~ Superstition`, a sibling record -- the same
/// "follow the reference one hop further" lesson `decisions.md §15` names).
/// `class_level_alias` is asserted VERBATIM as the corpus states it, never
/// resolved past that one hop (e.g. Slayer ~ Stalker's `SlayerStalkerLVL`
/// aliases to `SlayerStudiedTargetLVL`, not further to `SlayerLVL`) -- the
/// same "never resolve a live value" posture the DURATION/RANGE seams keep.
#[derive(Debug, Clone)]
pub struct ClassFeatureFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub bonus_var_name: String,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    pub corpus_field: String,
    pub alias_upstream_line: u64,
    pub alias_corpus_field: String,
    pub expected_offset_pre: i32,
    pub expected_divisor: i32,
    pub expected_offset_post: i32,
    pub expected_level_var: String,
    pub expected_class_level_alias: String,
}

/// Reads the `class_feature_entries` array of the committed fixture file.
pub fn load_class_feature_fixtures(repo_root: &Path) -> Vec<ClassFeatureFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("class_feature_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            ClassFeatureFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                bonus_var_name: e["bonus_var_name"].as_str().expect("bonus_var_name").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                alias_upstream_line: e["alias_upstream_line"]
                    .as_u64()
                    .expect("alias_upstream_line"),
                alias_corpus_field: e["alias_corpus_field"]
                    .as_str()
                    .expect("alias_corpus_field")
                    .to_string(),
                expected_offset_pre: i32::try_from(
                    expected["offset_pre"].as_i64().expect("expected.offset_pre"),
                )
                .expect("offset_pre fits in i32"),
                expected_divisor: i32::try_from(
                    expected["divisor"].as_i64().expect("expected.divisor"),
                )
                .expect("divisor fits in i32"),
                expected_offset_post: i32::try_from(
                    expected["offset_post"].as_i64().expect("expected.offset_post"),
                )
                .expect("offset_post fits in i32"),
                expected_level_var: expected["level_var"]
                    .as_str()
                    .expect("expected.level_var")
                    .to_string(),
                expected_class_level_alias: expected["class_level_alias"]
                    .as_str()
                    .expect("expected.class_level_alias")
                    .to_string(),
            }
        })
        .collect()
}

/// Where this repo's own ingest of `book`'s `class_feature` kind lives, and
/// whether it exists -- the `class_feature` sibling of
/// [`ingested_equipment_dir`]/[`spell_corpus_dir_exists`].
fn class_feature_corpus_dir_exists(repo_root: &Path, book: &str) -> Option<PathBuf> {
    let dir = repo_root.join("data").join("corpus").join(book);
    dir.join("class_feature").is_dir().then_some(dir)
}

/// Walks `data/corpus/<book>/class_feature/` once (nested by class/ability
/// slug) and returns every record's `BONUS:VAR|<name>|<formula>` tokens,
/// keyed by the record's own `data.key` -- the `class_feature` sibling of
/// [`load_spell_durations`]/[`load_spell_ranges`]'s recursive walk, carrying
/// every `VAR` token (not just one field) because a class-feature bar check
/// needs BOTH the headline formula token and, potentially on a DIFFERENT
/// record in the same walk, the level-variable's own alias definition.
fn load_class_feature_bonus_vars(
    class_feature_dir: &Path,
) -> BTreeMap<String, Vec<(String, String)>> {
    let mut out: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    let mut stack = vec![class_feature_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(read_dir) = std::fs::read_dir(&dir) else { continue };
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            let Some(key) = doc["data"]["key"].as_str() else { continue };
            let Some(tokens) = doc["data"]["raw_tokens"].as_array() else { continue };
            let mut vars = Vec::new();
            for t in tokens {
                if t["key"].as_str() != Some("BONUS") {
                    continue;
                }
                let Some(v) = t["value"].as_str() else { continue };
                let Some(rest) = v.strip_prefix("VAR|") else { continue };
                let Some((name, formula)) = rest.split_once('|') else { continue };
                vars.push((name.to_string(), formula.to_string()));
            }
            if !vars.is_empty() {
                out.entry(key.to_string()).or_default().extend(vars);
            }
        }
    }
    out
}

/// Searches every record's `BONUS:VAR` tokens `bonus_vars` carries (the
/// WHOLE book, not one record -- see [`ClassFeatureFixture`]'s doc comment
/// on why the alias may live on a sibling record) for a token whose NAME is
/// `level_var`, and returns its formula text verbatim (the declared alias,
/// e.g. `"BarbarianLVL"` or, one hop short of a base class,
/// `"SlayerStudiedTargetLVL"`) -- `None` if no record in the book defines it.
fn find_level_var_alias(
    bonus_vars: &BTreeMap<String, Vec<(String, String)>>,
    level_var: &str,
) -> Option<String> {
    bonus_vars.values().flatten().find(|(name, _)| name == level_var).map(|(_, v)| v.clone())
}

/// The `kind=class_feature` half of [`run_bar_check`].
fn run_class_feature_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_class_feature_fixtures(repo_root);
    let fixtures_total = fixtures.len();
    let books: BTreeSet<String> = fixtures.iter().map(|f| f.book.clone()).collect();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for book in &books {
        let Some(dir) = class_feature_corpus_dir_exists(repo_root, book) else {
            for f in fixtures.iter().filter(|f| &f.book == book) {
                engine_does_not_hold.insert(f.unit_id.clone(), book.clone());
            }
            continue;
        };
        let bonus_vars = load_class_feature_bonus_vars(&dir.join("class_feature"));

        for fixture in fixtures.iter().filter(|f| &f.book == book) {
            let Some(record_vars) = bonus_vars.get(&fixture.record_key) else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "{:?} does not resolve against {book}'s ingested class_feature cache",
                        fixture.record_key
                    ),
                );
                continue;
            };
            let Some((_, raw_formula)) =
                record_vars.iter().find(|(name, _)| name == &fixture.bonus_var_name)
            else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} but carries no BONUS:VAR|{}| token at all",
                        fixture.corpus_field, fixture.bonus_var_name
                    ),
                );
                continue;
            };
            let Some((level_var, formula)) = parse_class_feature_level_scaling(raw_formula)
            else {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} but the evaluator could not parse a level-scaling \
                         formula from {raw_formula:?}",
                        fixture.corpus_field
                    ),
                );
                continue;
            };
            if level_var != fixture.expected_level_var
                || formula.offset_pre != fixture.expected_offset_pre
                || formula.divisor != fixture.expected_divisor
                || formula.offset_post != fixture.expected_offset_post
            {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row {:?} states level_var {:?} offset_pre {} divisor {} \
                         offset_post {}, evaluator produced level_var {:?} offset_pre {} \
                         divisor {} offset_post {}",
                        fixture.corpus_field,
                        fixture.expected_level_var,
                        fixture.expected_offset_pre,
                        fixture.expected_divisor,
                        fixture.expected_offset_post,
                        level_var,
                        formula.offset_pre,
                        formula.divisor,
                        formula.offset_post
                    ),
                );
                continue;
            }
            match find_level_var_alias(&bonus_vars, &level_var) {
                Some(alias) if alias == fixture.expected_class_level_alias => {
                    cleared.insert(fixture.unit_id.clone());
                }
                Some(alias) => {
                    failures.insert(
                        fixture.unit_id.clone(),
                        format!(
                            "level_var {level_var:?} aliases {alias:?} in {book}'s own class_feature \
                             corpus, fixture expected {:?}",
                            fixture.expected_class_level_alias
                        ),
                    );
                }
                None => {
                    failures.insert(
                        fixture.unit_id.clone(),
                        format!(
                            "no record in {book}'s class_feature corpus defines BONUS:VAR|{level_var}|, \
                             so the fixture's expected alias {:?} cannot be confirmed",
                            fixture.expected_class_level_alias
                        ),
                    );
                }
            }
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

// ---------------------------------------------------------------------------
// `kind = monster`, second seam — the spell-like-ability SAVE DC
// (SD31-W15-MONSTER-SLA-001). Sibling of `run_monster_bar_check` above, which
// covers the `BONUS:VAR|SLA_CL|` half of the SAME universal monster rule;
// this half covers the `SPELLS:…|<spell>,<dc>` half.
//
// **The caster-level seam is exhausted for the still-held population, and the
// evidence is stronger than a bare count.** Of the 316 `monster` units held at
// `derived`+`grounded` when this seam was built, exactly TWO carry a
// `BONUS:VAR|SLA_CL|` token at all — and `spell_like_ability_caster_level`
// can bank NEITHER of them:
//   * `bestiary:monster:dryad` is one of the 46 Bestiary 1 records that carry
//     no `corpus_key` and so do not resolve against `MONSTER_BOOKS` at all
//     (`OPEN-ISSUES.md` row 266);
//   * `book_of_the_damned_volume_2:monster:demon_vermlek` carries
//     `BONUS:VAR|SLA_CL|HD*3/4`, which that function already refuses as
//     unparseable rather than guessing at — see its own doc comment.
// So a second, genuinely different magnitude was needed, not a wider fixture
// set over the same one.
// ---------------------------------------------------------------------------

/// PF1's Spell-Like Abilities universal monster rule fixes the constant part
/// of a spell-like ability's save DC at **10**.
///
/// Pathfinder Roleplaying Game Bestiary, Appendix 1 "Universal Monster
/// Rules", *Spell-Like Abilities* (verified against the public PRD mirror
/// `legacy.aonprd.com/bestiary/universalMonsterRules.html`): *"The save DC
/// is Charisma-based unless otherwise noted"*, against the Core Rulebook's
/// own general statement of the same formula — a spell-like ability's save
/// DC is **10 + the spell's level + the creature's ability modifier**.
///
/// Named rather than inlined because it is the single number
/// [`spell_like_ability_save_dc`] applies, and therefore the single thing a
/// mutation test has to move to prove the seam can go red.
pub const SPELL_LIKE_ABILITY_SAVE_DC_BASE: i32 = 10;

/// The structural parameters PF1's spell-like-ability save-DC rule states for
/// one granted spell.
///
/// **What this deliberately does NOT do.** It does not resolve the DC to a
/// number. A monster's ability MODIFIER is not a corpus-stated fact in this
/// repo — `MonsterStatBlock::stat_adjustments` carries adjustments, never
/// scores, and `SD31-E6-F1-002` already refused to compute the score family
/// rather than fabricate one. Resolving `15+CHA` to a DC would be exactly
/// that fabrication. What IS derivable, with no live creature at all, is the
/// formula's own parameters: the ability the DC scales with, and — by the
/// rule above, run backwards over the corpus-stated constant — **the spell's
/// own level**. Same posture as [`parse_caster_level_linear_duration`]'s
/// `per_level`/`unit`: a structural derivation, not a resolved magnitude.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellLikeAbilitySaveDc {
    /// The granted spell's own level, derived from the DC token's constant
    /// part by [`SPELL_LIKE_ABILITY_SAVE_DC_BASE`].
    pub spell_level: i32,
    /// The ability whose modifier the DC scales with, spelled exactly as the
    /// row spells it (`CHA`; `INT` for the handful of monsters whose rows
    /// exercise the rule's own "unless otherwise noted" clause).
    pub ability: String,
}

/// Applies the rule above to one [`MonsterSpellLikeAbility`]'s
/// `save_dc_token`.
///
/// Refuses (`None`) rather than guessing on: a spell the row states no save
/// for (`save_dc_token` is `None` — a spell that allows no save is a real,
/// honest absence, not missing data); a token whose constant part is not a
/// plain integer; a token with no `+<ability>` tail; and a constant below
/// [`SPELL_LIKE_ABILITY_SAVE_DC_BASE`], which would imply a negative spell
/// level and therefore means the token is not this shape at all.
pub fn spell_like_ability_save_dc(
    sla: &crate::rules_core::rules_tables::monster_chassis::MonsterSpellLikeAbility,
) -> Option<SpellLikeAbilitySaveDc> {
    let raw = sla.save_dc_token?.trim();
    let (constant, ability) = raw.split_once('+')?;
    let constant: i32 = constant.trim().parse().ok()?;
    let ability = ability.trim();
    if ability.is_empty() || !ability.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let spell_level = constant - SPELL_LIKE_ABILITY_SAVE_DC_BASE;
    if spell_level < 0 {
        return None;
    }
    Some(SpellLikeAbilitySaveDc { spell_level, ability: ability.to_string() })
}

/// One `kind=monster` spell-like-ability save-DC fixture row. Deliberately
/// carries the INDEPENDENT provenance of the expected value — the spell's own
/// upstream `.lst` file, line and sha256, which is a DIFFERENT FILE from the
/// monster row the evaluator reads. That separation is the whole
/// non-circularity argument for this seam: the expected spell level is read
/// off PCGen's own spell definition, never off the monster row the evaluator
/// parses.
#[derive(Debug, Clone)]
pub struct MonsterSlaFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    /// The granted spell, spelled as the monster row spells it — the key this
    /// fixture resolves against `MonsterStatBlock::spell_like_abilities`.
    pub spell: String,
    /// The monster row's own DC token, verbatim (`15+CHA`).
    pub corpus_field: String,
    /// Where the expected spell level was read from — the SPELL's `.lst`.
    pub spell_level_lst: String,
    pub spell_level_lst_sha256: String,
    pub spell_level_line: u64,
    /// The spell record's own `CLASSES:` token, verbatim, which is what
    /// states the level.
    pub spell_level_corpus_field: String,
    pub expected_spell_level: i32,
    pub expected_ability: String,
}

/// Reads the `monster_sla_entries` array of the committed fixture file.
pub fn load_monster_sla_fixtures(repo_root: &Path) -> Vec<MonsterSlaFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("monster_sla_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            MonsterSlaFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                spell: e["spell"].as_str().expect("spell").to_string(),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                spell_level_lst: e["spell_level_lst"]
                    .as_str()
                    .expect("spell_level_lst")
                    .to_string(),
                spell_level_lst_sha256: e["spell_level_lst_sha256"]
                    .as_str()
                    .expect("spell_level_lst_sha256")
                    .to_string(),
                spell_level_line: e["spell_level_line"].as_u64().expect("spell_level_line"),
                spell_level_corpus_field: e["spell_level_corpus_field"]
                    .as_str()
                    .expect("spell_level_corpus_field")
                    .to_string(),
                expected_spell_level: i32::try_from(
                    expected["spell_level"].as_i64().expect("expected.spell_level"),
                )
                .expect("a spell level fits in i32"),
                expected_ability: expected["ability"]
                    .as_str()
                    .expect("expected.ability")
                    .to_string(),
            }
        })
        .collect()
}

/// The save-DC half of the `kind=monster` bar. Resolves each fixture entry
/// through the SAME `monster_chassis::MONSTER_BOOKS` registry
/// `run_monster_bar_check` and `v06_work_inventory`'s own `grounded` verdict
/// for `monster` already read.
///
/// **A unit clears only when EVERY fixture row naming it clears.** A monster
/// routinely grants a dozen spell-like abilities and the derivation script
/// emits one row per spell with a save DC; banking the unit on the first row
/// that happened to agree would be exactly the "evidence weaker than its
/// class requires" the anti-gaming rule forbids.
fn run_monster_sla_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_monster_sla_fixtures(repo_root);
    let fixtures_total = fixtures.len();

    let mut candidates: BTreeSet<String> = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for fixture in &fixtures {
        candidates.insert(fixture.unit_id.clone());
        let registry_book = monster_registry_book(&fixture.book);
        let Some(monster_book) = MONSTER_BOOKS.iter().find(|b| b.corpus_book == registry_book)
        else {
            engine_does_not_hold.insert(fixture.unit_id.clone(), fixture.book.clone());
            continue;
        };
        let Some(monster) = monster_book.monster_resolve(&fixture.record_key) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "{:?} does not resolve against {registry_book}'s registered monsters",
                    fixture.record_key
                ),
            );
            continue;
        };
        let Some(sla) =
            monster.spell_like_abilities.iter().find(|s| s.spell == fixture.spell)
        else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "{:?} grants no spell-like ability named {:?}",
                    fixture.record_key, fixture.spell
                ),
            );
            continue;
        };
        match spell_like_ability_save_dc(sla) {
            None => {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} for {:?} but the evaluator produced no save DC \
                         at all",
                        fixture.corpus_field, fixture.spell
                    ),
                );
            }
            Some(dc)
                if dc.spell_level == fixture.expected_spell_level
                    && dc.ability == fixture.expected_ability => {}
            Some(dc) => {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} for {:?}; {} states spell level {} ({}), \
                         evaluator produced spell level {} ({})",
                        fixture.corpus_field,
                        fixture.spell,
                        fixture.spell_level_lst,
                        fixture.expected_spell_level,
                        fixture.expected_ability,
                        dc.spell_level,
                        dc.ability
                    ),
                );
            }
        }
    }

    let cleared: BTreeSet<String> = candidates
        .into_iter()
        .filter(|id| !failures.contains_key(id) && !engine_does_not_hold.contains_key(id))
        .collect();
    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

#[cfg(test)]
mod class_feature_seam_tests {
    use super::*;

    // --- parser unit tests: one TDD red/green anchor per corpus-observed shape ---

    #[test]
    fn n_plus_var_slash_d_parses_as_offset_post() {
        // Rage Power ~ Superstition, `BONUS:VAR|SuperstitionSaveBonus|2+RagePowersLVL/4`
        // (`core_rulebook/cr_abilities_class.lst:493`). Division binds tighter
        // than `+` with no parens present, so the `2` is added AFTER the
        // divide, not before it.
        assert_eq!(
            parse_class_feature_level_scaling("2+RagePowersLVL/4"),
            Some((
                "RagePowersLVL".to_string(),
                ClassFeatureLevelScalingFormula { offset_pre: 0, divisor: 4, offset_post: 2 }
            ))
        );
    }

    #[test]
    fn bare_var_slash_d_parses_with_no_offset() {
        // Rogue ~ Trap Sense, `BONUS:VAR|TrapSenseBonus|RogueTrapSenseLVL/3`
        // (`core_rulebook/cr_abilities_class.lst:1618`).
        assert_eq!(
            parse_class_feature_level_scaling("RogueTrapSenseLVL/3"),
            Some((
                "RogueTrapSenseLVL".to_string(),
                ClassFeatureLevelScalingFormula { offset_pre: 0, divisor: 3, offset_post: 0 }
            ))
        );
    }

    #[test]
    fn parenthesised_var_plus_n_slash_d_parses_as_offset_pre() {
        // Ranger ~ Favored Terrain,
        // `BONUS:VAR|FavoredTerrainPool|(RangerFavoredTerrainLVL+2)/5`
        // (`core_rulebook/cr_abilities_class.lst:1445`). The parens force the
        // `+2` BEFORE the divide, unlike the paren-free shape above.
        assert_eq!(
            parse_class_feature_level_scaling("(RangerFavoredTerrainLVL+2)/5"),
            Some((
                "RangerFavoredTerrainLVL".to_string(),
                ClassFeatureLevelScalingFormula { offset_pre: 2, divisor: 5, offset_post: 0 }
            ))
        );
    }

    #[test]
    fn parenthesised_var_minus_n_slash_d_parses_with_a_negative_offset_pre() {
        // Bloodrager ~ Damage Reduction,
        // `BONUS:VAR|BloodragerDR|(BloodragerDRLVL-4)/3`
        // (`advanced_class_guide/acg_abilities_class.lst:341`).
        assert_eq!(
            parse_class_feature_level_scaling("(BloodragerDRLVL-4)/3"),
            Some((
                "BloodragerDRLVL".to_string(),
                ClassFeatureLevelScalingFormula { offset_pre: -4, divisor: 3, offset_post: 0 }
            ))
        );
    }

    #[test]
    fn var_slash_d_plus_n_parses_as_offset_post() {
        // Slayer ~ Stalker, `BONUS:VAR|SlayerStalkerBonus|SlayerStalkerLVL/5+1`
        // (`advanced_class_guide/acg_abilities_class.lst:1793`) -- the
        // paren-free shape again, this time with the `+1` written after the
        // divide rather than before it (`n_plus_var_slash_d` above).
        assert_eq!(
            parse_class_feature_level_scaling("SlayerStalkerLVL/5+1"),
            Some((
                "SlayerStalkerLVL".to_string(),
                ClassFeatureLevelScalingFormula { offset_pre: 0, divisor: 5, offset_post: 1 }
            ))
        );
    }

    // TDD red/green anchors: shapes this seam deliberately refuses rather
    // than guesses at.
    #[test]
    fn a_max_wrapped_formula_refuses_rather_than_guesses() {
        // Bard ~ Bardic Knowledge, `max(1,BardicKnowledgeLVL/2)` -- a real
        // corpus shape this seam does not attempt.
        assert_eq!(parse_class_feature_level_scaling("max(1,BardicKnowledgeLVL/2)"), None);
    }

    #[test]
    fn an_ability_score_term_refuses_rather_than_guesses() {
        // Paladin ~ Divine Grace, `max(CHA,0)` -- no `/` at all, and `CHA` is
        // an ability score, not a level variable.
        assert_eq!(parse_class_feature_level_scaling("max(CHA,0)"), None);
    }

    #[test]
    fn a_zero_divisor_refuses_rather_than_dividing_by_zero() {
        assert_eq!(parse_class_feature_level_scaling("SomeLVL/0"), None);
        assert_eq!(parse_class_feature_level_scaling("(SomeLVL+1)/0"), None);
    }

    #[test]
    fn a_bare_formula_with_no_slash_refuses() {
        assert_eq!(parse_class_feature_level_scaling("SomeLVL"), None);
        assert_eq!(parse_class_feature_level_scaling("3+SomeLVL"), None);
    }

    #[test]
    fn run_class_feature_bar_check_clears_every_committed_class_feature_fixture() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let report = run_class_feature_bar_check(&repo_root);
        assert!(
            report.fixtures_total > 0,
            "the committed fixture must carry at least one class_feature_entries row"
        );
        assert!(
            report.engine_does_not_hold.is_empty(),
            "every committed class_feature fixture's book must be ingested, got: {:?}",
            report.engine_does_not_hold
        );
        assert!(
            report.failures.is_empty(),
            "every committed class_feature fixture must clear the bar, got {} failures, first \
             few: {:?}",
            report.failures.len(),
            report.failures.iter().take(5).collect::<Vec<_>>()
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }

    /// A synthetic `repo_root` carrying one `class_feature` corpus record
    /// (`Rage Power ~ Superstition`-shaped: `2+ProbeLVL/4`, plus a sibling
    /// record defining `ProbeLVL`'s own alias) plus one fixture the caller
    /// corrupts -- same `ScratchRangeRoot`/`ScratchDurationRoot` pattern the
    /// spell seams above use, so a test can drive the REAL
    /// `run_class_feature_bar_check(&root)` end to end without touching the
    /// committed fixture.
    struct ScratchClassFeatureRoot {
        root: PathBuf,
    }

    impl ScratchClassFeatureRoot {
        /// The real corpus formula is fixed (`2+ProbeLVL/4`, offset_pre=0
        /// under `parse_class_feature_level_scaling`'s own N+VAR/D shape);
        /// every parameter here is what the FIXTURE claims via `expected`,
        /// so a caller can independently mutate any one of the four
        /// compared fields away from truth and prove `run_class_feature_
        /// bar_check` catches that specific mismatch (SD31-W13-INTEGRATE-001:
        /// `offset_pre` and `level_var` were previously never mutated at
        /// all -- `offset_pre` had in fact been dropped from this
        /// constructor's own parameter list, `let _ = expected_offset_pre;`
        /// dead code, one commit prior).
        fn new_full(
            name: &str,
            expected_offset_pre: i32,
            expected_divisor: i32,
            expected_offset_post: i32,
            expected_level_var: &str,
        ) -> Self {
            let root = std::env::temp_dir().join(format!(
                "codex_class_feature_mutation_proof_{name}_{}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&root);
            let cf_dir = root.join("data/corpus/core_rulebook/class_feature");
            std::fs::create_dir_all(&cf_dir).unwrap();
            std::fs::write(
                cf_dir.join("scratch_power.json"),
                r#"{"data":{"key":"Probe ~ Scratch Power","raw_tokens":[
                    {"key":"BONUS","value":"VAR|ScratchPowerBonus|2+ProbeLVL/4"}
                ]}}"#,
            )
            .unwrap();
            std::fs::write(
                cf_dir.join("scratch_pool_header.json"),
                r#"{"data":{"key":"Probe ~ Scratch Powers","raw_tokens":[
                    {"key":"BONUS","value":"VAR|ProbeLVL|ProbeClassLVL"}
                ]}}"#,
            )
            .unwrap();
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(
                    r#"{{"class_feature_entries":[{{
                        "unit_id":"scratch:class_feature:scratch_power",
                        "book":"core_rulebook",
                        "record_key":"Probe ~ Scratch Power",
                        "bonus_var_name":"ScratchPowerBonus",
                        "upstream_lst":"scratch.lst",
                        "upstream_lst_sha256":"0",
                        "upstream_line":1,
                        "corpus_field":"BONUS:VAR|ScratchPowerBonus|2+ProbeLVL/4",
                        "alias_upstream_line":1,
                        "alias_corpus_field":"BONUS:VAR|ProbeLVL|ProbeClassLVL",
                        "expected":{{
                            "offset_pre":{expected_offset_pre},
                            "divisor":{expected_divisor},
                            "offset_post":{expected_offset_post},
                            "level_var":"{expected_level_var}",
                            "class_level_alias":"ProbeClassLVL"
                        }}
                    }}]}}"#
                ),
            )
            .unwrap();
            ScratchClassFeatureRoot { root }
        }

        fn new(name: &str, expected_divisor: i32, expected_offset_post: i32) -> Self {
            Self::new_full(name, 0, expected_divisor, expected_offset_post, "ProbeLVL")
        }
    }

    impl Drop for ScratchClassFeatureRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    // MUTATION PROOF: a fixture whose `expected.divisor` is deliberately
    // wrong must make the REAL `run_class_feature_bar_check` report a
    // failure, not silently pass.
    #[test]
    fn a_wrong_expected_divisor_makes_run_class_feature_bar_check_report_a_failure() {
        let (_, real) = parse_class_feature_level_scaling("2+ProbeLVL/4").unwrap();
        let wrong_divisor = real.divisor + 1;
        let scratch = ScratchClassFeatureRoot::new("wrong_divisor", wrong_divisor, real.offset_post);
        let report = run_class_feature_bar_check(&scratch.root);

        assert!(
            report.cleared.is_empty(),
            "a fixture asserting a wrong expected divisor must never clear the bar, got {:?}",
            report.cleared
        );
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:class_feature:scratch_power"));
    }

    // MUTATION PROOF (SD31-W13-INTEGRATE-001, was missing entirely): a
    // fixture whose `expected.offset_pre` is deliberately wrong must also
    // make the real check fail. The real formula (`2+ProbeLVL/4`) has
    // offset_pre=0; asserting 1 must not clear the bar.
    #[test]
    fn a_wrong_expected_offset_pre_makes_run_class_feature_bar_check_report_a_failure() {
        let (_, real) = parse_class_feature_level_scaling("2+ProbeLVL/4").unwrap();
        assert_eq!(real.offset_pre, 0, "test assumption: real offset_pre is 0");
        let scratch =
            ScratchClassFeatureRoot::new_full("wrong_offset_pre", 1, real.divisor, real.offset_post, "ProbeLVL");
        let report = run_class_feature_bar_check(&scratch.root);

        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:class_feature:scratch_power"));
    }

    // MUTATION PROOF (SD31-W13-INTEGRATE-001, was missing entirely): a
    // fixture whose `expected.offset_post` is deliberately wrong must also
    // make the real check fail.
    #[test]
    fn a_wrong_expected_offset_post_makes_run_class_feature_bar_check_report_a_failure() {
        let (_, real) = parse_class_feature_level_scaling("2+ProbeLVL/4").unwrap();
        let wrong_offset_post = real.offset_post + 1;
        let scratch =
            ScratchClassFeatureRoot::new_full("wrong_offset_post", 0, real.divisor, wrong_offset_post, "ProbeLVL");
        let report = run_class_feature_bar_check(&scratch.root);

        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:class_feature:scratch_power"));
    }

    // MUTATION PROOF (SD31-W13-INTEGRATE-001, was missing entirely): a
    // fixture whose `expected.level_var` names the WRONG variable must also
    // make the real check fail -- distinct from the class_level_alias proof
    // below, which mutates the alias the level_var resolves TO, not the
    // level_var name itself.
    #[test]
    fn a_wrong_expected_level_var_makes_run_class_feature_bar_check_report_a_failure() {
        let (_, real) = parse_class_feature_level_scaling("2+ProbeLVL/4").unwrap();
        let scratch = ScratchClassFeatureRoot::new_full(
            "wrong_level_var",
            0,
            real.divisor,
            real.offset_post,
            "TotallyTheWrongLevelVar",
        );
        let report = run_class_feature_bar_check(&scratch.root);

        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:class_feature:scratch_power"));
    }

    // The same proof for `expected.class_level_alias`: a fixture claiming
    // the WRONG owning class for the level variable must also fail, not just
    // a wrong numeric coefficient -- this is the check that would have
    // caught a level-scaling formula silently pointing at the wrong class.
    #[test]
    fn a_wrong_expected_class_level_alias_makes_run_class_feature_bar_check_report_a_failure() {
        let root = std::env::temp_dir()
            .join(format!("codex_class_feature_mutation_proof_wrong_alias_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let cf_dir = root.join("data/corpus/core_rulebook/class_feature");
        std::fs::create_dir_all(&cf_dir).unwrap();
        std::fs::write(
            cf_dir.join("scratch_power.json"),
            r#"{"data":{"key":"Probe ~ Scratch Power","raw_tokens":[
                {"key":"BONUS","value":"VAR|ScratchPowerBonus|2+ProbeLVL/4"}
            ]}}"#,
        )
        .unwrap();
        std::fs::write(
            cf_dir.join("scratch_pool_header.json"),
            r#"{"data":{"key":"Probe ~ Scratch Powers","raw_tokens":[
                {"key":"BONUS","value":"VAR|ProbeLVL|ProbeClassLVL"}
            ]}}"#,
        )
        .unwrap();
        let fixture_dir = root.join("tests/fixtures/rules_core");
        std::fs::create_dir_all(&fixture_dir).unwrap();
        std::fs::write(
            fixture_dir.join("derived-evaluator-fixtures.json"),
            r#"{"class_feature_entries":[{
                "unit_id":"scratch:class_feature:scratch_power",
                "book":"core_rulebook",
                "record_key":"Probe ~ Scratch Power",
                "bonus_var_name":"ScratchPowerBonus",
                "upstream_lst":"scratch.lst",
                "upstream_lst_sha256":"0",
                "upstream_line":1,
                "corpus_field":"BONUS:VAR|ScratchPowerBonus|2+ProbeLVL/4",
                "alias_upstream_line":1,
                "alias_corpus_field":"BONUS:VAR|ProbeLVL|ProbeClassLVL",
                "expected":{
                    "offset_pre":0,
                    "divisor":4,
                    "offset_post":2,
                    "level_var":"ProbeLVL",
                    "class_level_alias":"TotallyTheWrongClassLVL"
                }
            }]}"#,
        )
        .unwrap();

        let report = run_class_feature_bar_check(&root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        let _ = std::fs::remove_dir_all(&root);
    }

    // The positive control for both mutation-proof tests above: a fixture
    // whose `expected` matches the real corpus row EXACTLY (divisor and
    // alias both correct) must clear the bar -- proving the two tests above
    // fail because the asserted value is wrong, not because the synthetic
    // harness always reports a failure.
    #[test]
    fn a_correct_expected_class_feature_formula_clears_run_class_feature_bar_check() {
        let (_, real) = parse_class_feature_level_scaling("2+ProbeLVL/4").unwrap();
        let scratch = ScratchClassFeatureRoot::new("correct", real.divisor, real.offset_post);
        let report = run_class_feature_bar_check(&scratch.root);

        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:class_feature:scratch_power"));
    }
}

#[cfg(test)]
mod class_feature_description_seam_tests {
    use super::*;

    /// SD-31 wave 27: [`ability_modifiers_from_fixture_inputs`] seeds exactly the named
    /// abilities and defaults every unnamed one to `0` -- the same "seed what is known, never
    /// guess the rest" posture as [`resolve_pcgen_var_chain`]'s own default seeding.
    #[test]
    fn ability_modifiers_from_fixture_inputs_seeds_named_abilities_and_defaults_the_rest() {
        let mut inputs: BTreeMap<String, i16> = BTreeMap::new();
        inputs.insert("INT".to_string(), 3);
        inputs.insert("WIS".to_string(), -1);
        let ability_modifiers = ability_modifiers_from_fixture_inputs(&inputs);
        assert_eq!(ability_modifiers.intelligence, 3);
        assert_eq!(ability_modifiers.wisdom, -1);
        assert_eq!(ability_modifiers.strength, 0);
        assert_eq!(ability_modifiers.dexterity, 0);
        assert_eq!(ability_modifiers.constitution, 0);
        assert_eq!(ability_modifiers.charisma, 0);
    }

    #[test]
    fn ability_modifiers_from_fixture_inputs_of_an_empty_map_is_all_zero() {
        let ability_modifiers = ability_modifiers_from_fixture_inputs(&BTreeMap::new());
        assert_eq!(ability_modifiers, crate::rules_core::pilot_compute::AbilityModifiers::default());
    }

    /// The real end-to-end gate: every committed `class_feature_description_entries` row --
    /// level-only (wave 26) and ability-modifier-dependent (wave 27) alike -- clears
    /// [`run_class_feature_description_bar_check`] against the REAL, live `data/corpus` ingest
    /// and the REAL production `resolve_pcgen_var_chain`. This is the seam family's own
    /// dedicated `cargo test` coverage -- before this wave it was exercised only by running the
    /// `derived_evaluator_fixture_check` binary, never by `cargo test`, so a regression here
    /// would have shipped silently past every other gate this repo runs on a normal PR.
    #[test]
    fn run_class_feature_description_bar_check_clears_every_committed_class_feature_description_fixture()
    {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let report = run_class_feature_description_bar_check(&repo_root);
        assert!(
            report.fixtures_total > 0,
            "the committed fixture must carry at least one class_feature_description_entries row"
        );
        assert!(
            report.failures.is_empty(),
            "every committed class_feature_description fixture must clear the bar, got {} \
             failures: {:?}",
            report.failures.len(),
            report.failures
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }

    /// Mutation-proof, without touching production code: a hand-built fixture whose
    /// `ability_modifier_inputs` names a DIFFERENT ability modifier value than the one the real
    /// corpus formula was independently derived against must NOT clear -- proving this bar-check
    /// is actually sensitive to `ability_modifier_inputs`, not merely present and vacuous. Uses
    /// the REAL live `Rogue ~ Master Strike` corpus record (`10+(MasterStrikeLVL/2)+INT`), so
    /// this exercises the real `class_feature_record_tokens()` lookup too, not a scratch corpus.
    #[test]
    fn a_wrong_ability_modifier_input_makes_run_class_feature_description_bar_check_report_a_failure()
    {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let real = crate::rules_core::pilot_compute::class_feature_grant_consumer::class_feature_record_tokens()
            .get("Rogue ~ Master Strike")
            .expect("Rogue ~ Master Strike must resolve against the live corpus");
        let mut ability_modifier_inputs = BTreeMap::new();
        // Deliberately the WRONG intelligence modifier -- one off the value this wave's real
        // committed fixture (see `derive_class_feature_description_fixtures.py`'s `TARGETS`)
        // independently derived the real corpus row against.
        ability_modifier_inputs.insert("INT".to_string(), 999i16);
        let mut expected_value_at_level_by_arg: BTreeMap<String, BTreeMap<u8, i64>> = BTreeMap::new();
        expected_value_at_level_by_arg
            .insert("MasterStrikeDC".to_string(), BTreeMap::from([(20u8, 23i64)]));
        let fixture = ClassFeatureDescriptionFixture {
            unit_id: "scratch:class_feature:wrong_ability_input".to_string(),
            book: "core_rulebook".to_string(),
            record_key: "Rogue ~ Master Strike".to_string(),
            class: real.class.clone(),
            class_level_var: "RogueLVL".to_string(),
            upstream_lst: "scratch".to_string(),
            upstream_lst_sha256: "scratch".to_string(),
            upstream_line: 0,
            corpus_field: "scratch".to_string(),
            expected_value_at_level_by_arg,
            ability_modifier_inputs,
        };
        // Reproduces `run_class_feature_description_bar_check`'s own per-fixture body directly
        // (that function only reads the committed fixture file, not an injectable list), against
        // the SAME real corpus record and the SAME real `resolve_pcgen_var_chain`.
        let _ = &repo_root;
        let ability_modifiers = ability_modifiers_from_fixture_inputs(&fixture.ability_modifier_inputs);
        let resolved = crate::rules_core::pilot_compute::class_feature_grant_consumer::resolve_pcgen_var_chain(
            &real.bonus_vars,
            &fixture.class_level_var,
            20,
            &ability_modifiers,
        );
        assert_ne!(
            resolved.get("MasterStrikeDC"),
            Some(&23),
            "an INT modifier of 999 must produce a wildly different DC than the real committed \
             fixture's INT input does, proving this bar-check is genuinely sensitive to \
             ability_modifier_inputs rather than vacuously true: {resolved:?}"
        );
    }
}

#[cfg(test)]
mod spell_range_seam_tests {
    use super::*;

    #[test]
    fn close_keyword_resolves_to_the_ruleset_formula() {
        assert_eq!(
            spell_range_formula("Close"),
            Some(SpellRangeFormula { base_ft: 25, rate_ft: 5, per_levels: 2 })
        );
    }

    #[test]
    fn medium_keyword_resolves_to_the_ruleset_formula() {
        assert_eq!(
            spell_range_formula("Medium"),
            Some(SpellRangeFormula { base_ft: 100, rate_ft: 10, per_levels: 1 })
        );
    }

    #[test]
    fn long_keyword_resolves_to_the_ruleset_formula() {
        assert_eq!(
            spell_range_formula("Long"),
            Some(SpellRangeFormula { base_ft: 400, rate_ft: 40, per_levels: 1 })
        );
    }

    // TDD red/green anchor: `RANGE:Personal` and `RANGE:Touch` carry no
    // caster-level scaling at all -- the ruleset states no `SPELLRANGE:`
    // formula for either -- so this seam must refuse rather than invent one.
    #[test]
    fn personal_and_touch_refuse_rather_than_guess() {
        assert_eq!(spell_range_formula("Personal"), None);
        assert_eq!(spell_range_formula("Touch"), None);
    }

    // A literal distance is already a resolved number, not a formula to
    // state -- refuse rather than fabricate a "base + rate" shape for it.
    #[test]
    fn a_literal_distance_refuses_rather_than_guesses() {
        assert_eq!(spell_range_formula("30 ft."), None);
        assert_eq!(spell_range_formula("See text"), None);
    }

    #[test]
    fn format_renders_the_close_formula_with_its_two_level_step() {
        let f = SpellRangeFormula { base_ft: 25, rate_ft: 5, per_levels: 2 };
        assert_eq!(format_spell_range_formula(&f), "25 ft. + 5 ft. per 2 caster levels");
    }

    #[test]
    fn format_renders_the_medium_formula_with_its_one_level_step() {
        let f = SpellRangeFormula { base_ft: 100, rate_ft: 10, per_levels: 1 };
        assert_eq!(format_spell_range_formula(&f), "100 ft. + 10 ft. per caster level");
    }

    #[test]
    fn run_spell_range_bar_check_clears_every_committed_fixture() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let report = run_spell_range_bar_check(&repo_root);
        assert!(
            report.fixtures_total > 0,
            "the committed fixture must carry at least one spell_range_entries row"
        );
        assert!(
            report.engine_does_not_hold.is_empty(),
            "every committed spell-range fixture's book must be ingested, got: {:?}",
            report.engine_does_not_hold
        );
        assert!(
            report.failures.is_empty(),
            "every committed spell-range fixture must clear the bar, got {} failures, first \
             few: {:?}",
            report.failures.len(),
            report.failures.iter().take(5).collect::<Vec<_>>()
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }

    /// A synthetic `repo_root` carrying exactly one spell corpus record
    /// (`RANGE:Close`) plus one fixture file whose `spell_range_entries`
    /// row is the caller's to corrupt -- lets a test drive the REAL
    /// `run_spell_range_bar_check(&root)` end to end without touching the
    /// committed fixture (which a concurrent cycle may also be reading).
    /// Same `std::env::temp_dir()` + pid-suffixed scratch-dir pattern as
    /// `wiring_class.rs`'s `ScratchBook`.
    struct ScratchRangeRoot {
        root: PathBuf,
    }

    impl ScratchRangeRoot {
        fn new(name: &str, expected_base_ft: i32, expected_rate_ft: i32, expected_per_levels: i32) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_spell_range_mutation_proof_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let spell_dir = root.join("data/corpus/core_rulebook/spell");
            std::fs::create_dir_all(&spell_dir).unwrap();
            std::fs::write(
                spell_dir.join("scratch_close_spell.json"),
                r#"{"data":{"key":"scratch_close_spell","raw_tokens":[{"key":"RANGE","value":"Close"}]}}"#,
            )
            .unwrap();
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(
                    r#"{{"spell_range_entries":[{{
                        "unit_id":"scratch:spell:scratch_close_spell",
                        "book":"core_rulebook",
                        "record_key":"scratch_close_spell",
                        "upstream_lst":"scratch.lst",
                        "upstream_lst_sha256":"0",
                        "upstream_line":1,
                        "corpus_field":"RANGE:Close",
                        "expected":{{"base_ft":{expected_base_ft},"rate_ft":{expected_rate_ft},"per_levels":{expected_per_levels}}}
                    }}]}}"#
                ),
            )
            .unwrap();
            ScratchRangeRoot { root }
        }
    }

    impl Drop for ScratchRangeRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    // MUTATION PROOF, made real (`OPEN-ISSUES.md` row 201, adversarial
    // review CONFIRMED wave 11): the prior version of this test asserted
    // `999 != 25` in isolation and never called `run_spell_range_bar_check`
    // at all -- none of that function's three failure branches was
    // exercised by any test. This version builds a synthetic corpus row +
    // a fixture whose `expected.base_ft` is deliberately wrong and drives
    // the REAL function, proving it actually reports the mismatch.
    #[test]
    fn a_wrong_expected_base_ft_makes_run_spell_range_bar_check_report_a_failure() {
        let real = spell_range_formula("Close").unwrap();
        let wrong_expected_base_ft = 999;
        assert_ne!(
            real.base_ft, wrong_expected_base_ft,
            "a corrupted expected value must genuinely disagree with the real formula"
        );

        let scratch = ScratchRangeRoot::new(
            "wrong_base_ft",
            wrong_expected_base_ft,
            real.rate_ft,
            real.per_levels,
        );
        let report = run_spell_range_bar_check(&scratch.root);

        assert!(
            report.cleared.is_empty(),
            "a fixture asserting a wrong expected base_ft must never clear the bar, got {:?}",
            report.cleared
        );
        assert_eq!(
            report.failures.len(),
            1,
            "the one corrupted fixture must be reported as a failure, got {:?}",
            report.failures
        );
        assert!(
            report.failures.contains_key("scratch:spell:scratch_close_spell"),
            "failures: {:?}",
            report.failures
        );
    }

    // The positive control for the same synthetic harness: a fixture whose
    // `expected` matches the real formula EXACTLY must clear the bar. This
    // proves the mutation-proof test above fails because the value is
    // wrong, not because the synthetic harness always reports a failure.
    #[test]
    fn a_correct_expected_base_ft_clears_run_spell_range_bar_check() {
        let real = spell_range_formula("Close").unwrap();
        let scratch =
            ScratchRangeRoot::new("correct_base_ft", real.base_ft, real.rate_ft, real.per_levels);
        let report = run_spell_range_bar_check(&scratch.root);

        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:spell:scratch_close_spell"));
    }
}

#[cfg(test)]
mod spell_seam_tests {
    use super::*;

    #[test]
    fn single_caster_level_with_no_multiplier_parses_as_one_per_level() {
        assert_eq!(
            parse_caster_level_linear_duration("(CASTERLEVEL) minutes"),
            Some(CasterLevelLinearFormula { per_level: 1, unit: "minutes".to_string() })
        );
    }

    // TDD red/green anchor: the real Adhesive Blood worked example
    // (`OPEN-ISSUES.md` row 119), `advanced_class_guide/acg_spells.lst:8`.
    #[test]
    fn adhesive_blood_shaped_duration_parses() {
        assert_eq!(
            parse_caster_level_linear_duration("(CASTERLEVEL) minutes"),
            Some(CasterLevelLinearFormula { per_level: 1, unit: "minutes".to_string() })
        );
    }

    #[test]
    fn explicit_multiplier_parses() {
        assert_eq!(
            parse_caster_level_linear_duration("(CASTERLEVEL*10) minutes [D]"),
            Some(CasterLevelLinearFormula { per_level: 10, unit: "minutes [D]".to_string() })
        );
    }

    #[test]
    fn spaced_multiplier_parses() {
        assert_eq!(
            parse_caster_level_linear_duration("(CASTERLEVEL * 50) ft."),
            Some(CasterLevelLinearFormula { per_level: 50, unit: "ft.".to_string() })
        );
    }

    #[test]
    fn min_clamp_refuses_rather_than_guesses() {
        assert_eq!(
            parse_caster_level_linear_duration(
                "(ConjurationNaturalistsCharmBonus+(CASTERLEVEL)) minutes [D]"
            ),
            None
        );
    }

    #[test]
    fn concentration_prefix_refuses_rather_than_guesses() {
        assert_eq!(
            parse_caster_level_linear_duration("Concentration, up to (CASTERLEVEL) rounds [D]"),
            None
        );
    }

    #[test]
    fn no_casterlevel_token_refuses() {
        assert_eq!(parse_caster_level_linear_duration("Permanent; see text"), None);
    }

    #[test]
    fn a_second_casterlevel_occurrence_in_the_unit_refuses() {
        assert_eq!(
            parse_caster_level_linear_duration("(CASTERLEVEL) rounds; CASTERLEVEL again"),
            None
        );
    }

    #[test]
    fn format_renders_the_literal_formula() {
        let f = CasterLevelLinearFormula { per_level: 1, unit: "minutes".to_string() };
        assert_eq!(format_caster_level_linear_duration(&f), "1 minutes per caster level");
        let f2 = CasterLevelLinearFormula { per_level: 10, unit: "rounds".to_string() };
        assert_eq!(format_caster_level_linear_duration(&f2), "10 rounds per caster level");
    }

    #[test]
    fn run_spell_bar_check_clears_every_committed_spell_fixture() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let report = run_spell_bar_check(&repo_root);
        assert!(
            report.fixtures_total > 0,
            "the committed fixture must carry at least one spell_entries row"
        );
        assert!(
            report.engine_does_not_hold.is_empty(),
            "every committed spell fixture's book must be ingested, got: {:?}",
            report.engine_does_not_hold
        );
        assert!(
            report.failures.is_empty(),
            "every committed spell fixture must clear the bar, got {} failures, first few: {:?}",
            report.failures.len(),
            report.failures.iter().take(5).collect::<Vec<_>>()
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }

    /// Duration-seam sibling of `spell_range_seam_tests::ScratchRangeRoot`:
    /// a synthetic `repo_root` carrying one `DURATION:(CASTERLEVEL*10)
    /// minutes [D]` spell record plus a fixture whose `spell_entries` row
    /// the caller corrupts, so a test can drive the REAL
    /// `run_spell_bar_check(&root)` rather than asserting the parser's
    /// output against a hand-typed wrong number in isolation.
    struct ScratchDurationRoot {
        root: PathBuf,
    }

    impl ScratchDurationRoot {
        fn new(name: &str, expected_per_level: i32, expected_unit: &str) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_spell_duration_mutation_proof_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let spell_dir = root.join("data/corpus/core_rulebook/spell");
            std::fs::create_dir_all(&spell_dir).unwrap();
            std::fs::write(
                spell_dir.join("scratch_duration_spell.json"),
                r#"{"data":{"key":"scratch_duration_spell","raw_tokens":[{"key":"DURATION","value":"(CASTERLEVEL*10) minutes [D]"}]}}"#,
            )
            .unwrap();
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(
                    r#"{{"spell_entries":[{{
                        "unit_id":"scratch:spell:scratch_duration_spell",
                        "book":"core_rulebook",
                        "record_key":"scratch_duration_spell",
                        "upstream_lst":"scratch.lst",
                        "upstream_lst_sha256":"0",
                        "upstream_line":1,
                        "corpus_field":"DURATION:(CASTERLEVEL*10) minutes [D]",
                        "expected":{{"per_level":{expected_per_level},"unit":{expected_unit:?}}}
                    }}]}}"#
                ),
            )
            .unwrap();
            ScratchDurationRoot { root }
        }
    }

    impl Drop for ScratchDurationRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    // MUTATION PROOF, made real (same defect class as
    // `spell_range_seam_tests`'s row-201 fix, found by auditing this
    // seam's OTHER assertions the same way per this card's own
    // instruction: a tautology rarely arrives alone). The prior version of
    // this test asserted `99 != 10` in isolation and never called
    // `run_spell_bar_check` at all. This version builds a synthetic corpus
    // row + a fixture whose `expected.per_level` is deliberately wrong and
    // drives the REAL function.
    #[test]
    fn a_wrong_expected_per_level_makes_run_spell_bar_check_report_a_failure() {
        let real = parse_caster_level_linear_duration("(CASTERLEVEL*10) minutes [D]").unwrap();
        let wrong_expected_per_level = 99;
        assert_ne!(
            real.per_level, wrong_expected_per_level,
            "a corrupted expected value must genuinely disagree with the real parse"
        );

        let scratch =
            ScratchDurationRoot::new("wrong_per_level", wrong_expected_per_level, &real.unit);
        let report = run_spell_bar_check(&scratch.root);

        assert!(
            report.cleared.is_empty(),
            "a fixture asserting a wrong expected per_level must never clear the bar, got {:?}",
            report.cleared
        );
        assert_eq!(
            report.failures.len(),
            1,
            "the one corrupted fixture must be reported as a failure, got {:?}",
            report.failures
        );
        assert!(
            report.failures.contains_key("scratch:spell:scratch_duration_spell"),
            "failures: {:?}",
            report.failures
        );
    }

    // Same real-function proof for the OTHER field this fixture asserts
    // (`expected.unit`): a wrong unit must also make the real function
    // disagree, not just the wrong per_level above.
    #[test]
    fn a_wrong_expected_unit_makes_run_spell_bar_check_report_a_failure() {
        // The scratch corpus row always states "(CASTERLEVEL*10) minutes [D]"
        // (`ScratchDurationRoot::new`) -- get its real parse so `per_level`
        // is correct and only `unit` is corrupted, isolating this test from
        // the sibling per_level-wrong test above.
        let real = parse_caster_level_linear_duration("(CASTERLEVEL*10) minutes [D]").unwrap();
        assert_ne!(real.unit, "rounds", "a corrupted expected unit must genuinely disagree");

        let scratch = ScratchDurationRoot::new("wrong_unit", real.per_level, "rounds");
        let report = run_spell_bar_check(&scratch.root);

        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }

    // The positive control: a fixture whose `expected` matches the real
    // parse exactly must clear the bar, proving the two tests above fail
    // because the value is wrong, not because the synthetic harness always
    // reports a failure.
    #[test]
    fn a_correct_expected_duration_clears_run_spell_bar_check() {
        let real = parse_caster_level_linear_duration("(CASTERLEVEL*10) minutes [D]").unwrap();
        let scratch = ScratchDurationRoot::new("correct", real.per_level, &real.unit);
        let report = run_spell_bar_check(&scratch.root);

        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:spell:scratch_duration_spell"));
    }
}

#[cfg(test)]
mod monster_seam_tests {
    use super::*;

    /// Every pre-existing test in this module (before SD31-E6-F9-003) is
    /// about the HD-parsing rule over the bare-`HD` `sla_cl_token` shape --
    /// Balor/Linnorm/Dragon all carry it verbatim in the real corpus -- so
    /// `stat_block` keeps defaulting to that shape and only the override
    /// tests below call [`stat_block_full`] directly with a different token.
    fn stat_block(monster_class: Option<&'static str>) -> MonsterStatBlock {
        stat_block_full(monster_class, true, Some("HD"))
    }

    /// Full control over every field [`spell_like_ability_caster_level`]
    /// reads: [`MonsterStatBlock::has_spell_like_abilities`] (SD31-E6-F1-002,
    /// `OPEN-ISSUES.md` row 44) and [`MonsterStatBlock::sla_cl_token`]
    /// (SD31-E6-F9-003, same row's own forecast follow-on).
    fn stat_block_full(
        monster_class: Option<&'static str>,
        has_spell_like_abilities: bool,
        sla_cl_token: Option<&'static str>,
    ) -> MonsterStatBlock {
        MonsterStatBlock {
            key: "test:monster:probe",
            name: "Probe",
            size: None,
            speeds: &[],
            race_type: None,
            race_subtype: None,
            challenge_rating: None,
            monster_class,
            source_page: None,
            natural_attacks: &[],
            ability_keys: &[],
            external_ability_refs: &[],
            stat_adjustments: &[],
            has_spell_like_abilities,
            sla_cl_token,
            spell_like_abilities: &[],
            source_file: "test.lst",
            source_line: 1,
        }
    }

    // TDD red/green anchor: the real Demon (Balor) worked example
    // (`MONSTERCLASS:Outsider (Fort/Will):20`) -- caster level 20, per the
    // Universal Monster Rule this function implements.
    #[test]
    fn demon_balor_shaped_monster_class_yields_its_trailing_hd_as_caster_level() {
        let block = stat_block(Some("Outsider (Fort/Will):20"));
        assert_eq!(spell_like_ability_caster_level(&block), Some(20));
    }

    // A `MONSTERCLASS` whose type segment is a bare word rather than a
    // parenthesised save pair (e.g. `Dragon:15`) must parse identically --
    // the rule reads the segment AFTER the last `:`, never assumes a
    // particular type-segment shape.
    #[test]
    fn bare_type_monster_class_still_yields_trailing_hd() {
        let block = stat_block(Some("Dragon:15"));
        assert_eq!(spell_like_ability_caster_level(&block), Some(15));
    }

    #[test]
    fn no_monster_class_token_yields_no_caster_level() {
        let block = stat_block(None);
        assert_eq!(spell_like_ability_caster_level(&block), None);
    }

    // A malformed trailing segment (never observed in the live corpus, but
    // this function must refuse rather than guess) yields `None`, not a
    // fabricated number.
    #[test]
    fn non_integer_trailing_segment_refuses_rather_than_guesses() {
        let block = stat_block(Some("Outsider (Fort/Will):unknown"));
        assert_eq!(spell_like_ability_caster_level(&block), None);
    }

    // SD31-E6-F1-002, `OPEN-ISSUES.md` row 44's fix: a monster with a
    // perfectly valid `MONSTERCLASS:` token but NO `BONUS:VAR|SLA_CL|` token
    // on its row has no spell-like abilities at all, and this evaluator must
    // not hand a production caller a caster level it has no meaning for.
    // TDD red/green anchor: the real Animated Object (Medium) worked example
    // (`b1_races.lst:13`, `MONSTERCLASS:Construct:3` -- a perfectly readable
    // HD token, but no `SLA_CL` on the row at all).
    #[test]
    fn a_valid_monster_class_with_no_spell_like_abilities_yields_no_caster_level() {
        let block = stat_block_full(Some("Construct:3"), false, None);
        assert_eq!(
            spell_like_ability_caster_level(&block),
            None,
            "a monster with no BONUS:VAR|SLA_CL| token has no spell-like abilities, regardless \
             of HD"
        );
    }

    // SD31-E6-F9-003 (`OPEN-ISSUES.md` row 44's own follow-on): the real
    // Couatl worked example (`b1_races.lst:74`) -- `BONUS:VAR|SLA_CL|9`
    // against `MONSTERCLASS:Couatl Outsider:12`. Before this fix the
    // function ignored the literal and always answered 12; the corpus's own
    // stated override is 9, and that is what a player must see.
    #[test]
    fn a_literal_sla_cl_override_wins_over_the_generic_hd_rule() {
        let block = stat_block_full(Some("Couatl Outsider:12"), true, Some("9"));
        assert_eq!(
            spell_like_ability_caster_level(&block),
            Some(9),
            "the row's own stated SLA_CL override must win over the generic HD rule"
        );
    }

    // The real Demon (Glabrezu) worked example (`b1_races.lst:95`) --
    // `BONUS:VAR|SLA_CL|14` against 12 HD -- a second, independent override
    // confirming the fix is not a Couatl-specific special case.
    #[test]
    fn a_second_literal_sla_cl_override_also_wins_over_hd() {
        let block = stat_block_full(Some("Outsider (Fort/Will):12"), true, Some("14"));
        assert_eq!(spell_like_ability_caster_level(&block), Some(14));
    }

    // `max(TL,1)` is a corpus-observed second spelling of "apply the generic
    // HD rule", equally valid to bare `HD` -- the real Azata (Ghaele) worked
    // example (`b1_races.lst:31`, `MONSTERCLASS:Outsider (Fort/Will):13`).
    #[test]
    fn max_tl_one_sla_cl_token_still_applies_the_generic_hd_rule() {
        let block = stat_block_full(Some("Outsider (Fort/Will):13"), true, Some("max(TL,1)"));
        assert_eq!(spell_like_ability_caster_level(&block), Some(13));
    }

    // `bestiary_2`'s Dragon (Magma) spells the same rule with a redundant
    // extra paren pair, `(max(TL,1))` -- the exact-set check must recognise
    // both spellings, not only the bare one.
    #[test]
    fn redundantly_parenthesised_max_tl_one_still_applies_the_generic_hd_rule() {
        let block = stat_block_full(Some("Dragon:9"), true, Some("(max(TL,1))"));
        assert_eq!(spell_like_ability_caster_level(&block), Some(9));
    }

    // W26-INTERPRETER-INTEGRATE (`OPERATOR-RULINGS-2026-08-21.md` §20):
    // `HD*3/4` -- the real Demon (Vermlek) worked example,
    // `book_of_the_damned_volume_2`, `BONUS:VAR|SLA_CL|HD*3/4` -- is a real
    // arithmetic formula over the monster's own Hit Dice, which
    // `formula_interpreter::PcgenFormulaEvaluator` can now read (`HD*3/4`
    // is plain multiply/divide, no unbound identifier). This test used to
    // assert `None` under §24.1's "no formula interpreter" ban; the
    // arithmetic is genuinely `16*3/4 = 12`, and refusing an evaluable
    // formula once the interpreter exists would be exactly the "leave a
    // real answer on the table" failure mode the ruling exists to fix. The
    // exact real-corpus value (`HD=4` -> `3`) is pinned separately by
    // `hd_times_three_quarters_matches_the_real_demon_vermlek_worked_example`
    // below and by `monster_entries`'s own
    // `book_of_the_damned_volume_2:monster:demon_vermlek` fixture row.
    #[test]
    fn a_multiply_divide_sla_cl_formula_now_evaluates_via_the_interpreter() {
        let block = stat_block_full(Some("Outsider:16"), true, Some("HD*3/4"));
        assert_eq!(spell_like_ability_caster_level(&block), Some(12));
    }

    // The real Demon (Vermlek) worked example itself
    // (`book_of_the_damned_volume_2/botd2_races.lst:7`,
    // `MONSTERCLASS:Outsider (Fort/Will):4`, `BONUS:VAR|SLA_CL|HD*3/4`) --
    // `4*3/4 = 3` exactly, no truncation ambiguity. Pinned independently by
    // `monster_entries`'s own fixture row for this unit.
    #[test]
    fn hd_times_three_quarters_matches_the_real_demon_vermlek_worked_example() {
        let block = stat_block_full(Some("Outsider (Fort/Will):4"), true, Some("HD*3/4"));
        assert_eq!(spell_like_ability_caster_level(&block), Some(3));
    }

    // An interpreter refusal (an unbound identifier the corpus has never
    // shown this repo, e.g. a race-specific bonus name with no `DEFINE:`
    // anywhere on the row) must still surface as `None`, never a guess --
    // the interpreter's own "never default to zero" contract, restated at
    // this seam's boundary so a future formula shape this repo cannot bind
    // fails exactly as honestly as an unparseable one always has.
    #[test]
    fn an_sla_cl_formula_naming_an_unbound_identifier_still_refuses() {
        let block = stat_block_full(Some("Outsider:16"), true, Some("HD*SomeRaceSpecificBonus"));
        assert_eq!(spell_like_ability_caster_level(&block), None);
    }

    // A monster whose row has not been re-transcribed with this field yet
    // (`sla_cl_token: None`) but whose `has_spell_like_abilities` is `true`
    // must keep the prior, already-correct-for-the-bare-HD-population
    // behaviour rather than silently losing its caster level -- a defensive
    // fallback, not a shape this repo's own registry currently produces
    // (every book is re-transcribed this same cycle).
    #[test]
    fn missing_sla_cl_token_with_the_presence_flag_still_set_falls_back_to_hd() {
        let block = stat_block_full(Some("Outsider (Fort/Will):20"), true, None);
        assert_eq!(spell_like_ability_caster_level(&block), Some(20));
    }

    #[test]
    fn run_monster_bar_check_clears_every_committed_monster_fixture() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let report = run_monster_bar_check(&repo_root);
        assert!(
            report.fixtures_total > 0,
            "the committed fixture must carry at least one monster_entries row"
        );
        assert!(
            report.failures.is_empty(),
            "every committed monster fixture must clear the bar, got failures: {:?}",
            report.failures
        );
        assert!(
            report.engine_does_not_hold.is_empty(),
            "every committed monster fixture's book must be ingested, got: {:?}",
            report.engine_does_not_hold
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }

    /// A scratch `repo_root` carrying ONLY a custom `monster_entries`
    /// fixture row -- unlike the equipment/spell seams, `run_monster_bar_check`
    /// resolves records through the compiled `monster_chassis::MONSTER_BOOKS`
    /// static registry, never the filesystem, so `repo_root` only ever
    /// controls which FIXTURE file is read; the real Demon (Balor) stat
    /// block still resolves regardless of which scratch root is passed.
    struct ScratchMonsterFixtureRoot {
        root: PathBuf,
    }

    impl ScratchMonsterFixtureRoot {
        fn new(name: &str, expected_caster_level: i32) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_monster_mutation_proof_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(
                    r#"{{"monster_entries":[{{
                        "unit_id":"scratch:monster:demon_balor",
                        "book":"bestiary",
                        "record_key":"Demon (Balor)",
                        "upstream_lst":"scratch.lst",
                        "upstream_lst_sha256":"0",
                        "upstream_line":1,
                        "corpus_field":"BONUS:VAR|SLA_CL|HD",
                        "monster_class_token":"Outsider (Fort/Will):20",
                        "expected":{{"spell_like_ability_caster_level":{expected_caster_level}}}
                    }}]}}"#
                ),
            )
            .unwrap();
            ScratchMonsterFixtureRoot { root }
        }
    }

    impl Drop for ScratchMonsterFixtureRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    // MUTATION PROOF, made real, driving `run_monster_bar_check` end to end
    // (the same shape wave 11's fix applied to the spell RANGE seam,
    // `OPEN-ISSUES.md` row 201 -- checked for and fixed here per this
    // card's own instruction): the OLD test
    // (`a_wrong_expected_caster_level_makes_the_bar_check_fail`, still above,
    // kept as a cheap red/green anchor on the evaluator alone) never called
    // `run_monster_bar_check` itself, only the bare evaluator function --
    // the production COMPARISON logic (`Some(cl) if cl == expected`) was
    // never actually exercised by any test. This one drives the real
    // function against a scratch fixture pointing at the real, resolved
    // Demon (Balor) with a deliberately wrong expected caster level (21 vs
    // the true 20) and confirms it reports a failure, not a vacuous pass.
    #[test]
    fn a_wrong_expected_caster_level_makes_run_monster_bar_check_report_a_failure() {
        let wrong_expected = 21;
        let scratch = ScratchMonsterFixtureRoot::new("wrong", wrong_expected);
        let report = run_monster_bar_check(&scratch.root);

        assert!(
            report.cleared.is_empty(),
            "a fixture asserting a wrong expected caster level must never clear the bar, got {:?}",
            report.cleared
        );
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:monster:demon_balor"));
    }

    // The positive control: a fixture whose `expected` matches the real,
    // resolved Demon (Balor) EXACTLY (caster level 20) must clear the bar --
    // proving the mutation-proof test above fails because the asserted
    // value is wrong, not because the synthetic harness always reports a
    // failure.
    #[test]
    fn a_correct_expected_caster_level_clears_run_monster_bar_check() {
        let scratch = ScratchMonsterFixtureRoot::new("correct", 20);
        let report = run_monster_bar_check(&scratch.root);

        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:monster:demon_balor"));
    }

    // --- the SAVE DC seam (SD31-W15-MONSTER-SLA-001) ---

    #[test]
    fn run_monster_sla_bar_check_clears_every_committed_monster_sla_fixture() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let report = run_monster_sla_bar_check(&repo_root);
        assert!(
            report.fixtures_total > 0,
            "the committed fixture must carry at least one monster_sla_entries row"
        );
        assert!(
            report.failures.is_empty(),
            "every committed monster_sla fixture must clear the bar, got failures: {:?}",
            report.failures
        );
        assert!(
            report.engine_does_not_hold.is_empty(),
            "every committed monster_sla fixture's book must be ingested, got: {:?}",
            report.engine_does_not_hold
        );
    }

    /// A scratch `repo_root` carrying ONLY custom `monster_sla_entries` rows.
    /// Like [`ScratchMonsterFixtureRoot`], `repo_root` only ever controls
    /// which FIXTURE file is read — the real Aboleth stat block resolves out
    /// of the compiled registry regardless.
    struct ScratchSlaFixtureRoot {
        root: PathBuf,
    }

    impl ScratchSlaFixtureRoot {
        /// One row per `(spell, expected_level)` pair, all naming the same
        /// real Bestiary 1 Aboleth.
        fn new(name: &str, rows: &[(&str, i32)]) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_monster_sla_mutation_proof_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            let entries: Vec<String> = rows
                .iter()
                .map(|(spell, level)| {
                    format!(
                        r#"{{"unit_id":"scratch:monster:aboleth","book":"bestiary",
                            "record_key":"Aboleth","upstream_lst":"scratch.lst",
                            "upstream_lst_sha256":"0","upstream_line":1,
                            "spell":"{spell}","corpus_field":"scratch",
                            "spell_level_lst":"scratch_spells.lst",
                            "spell_level_lst_sha256":"0","spell_level_line":1,
                            "spell_level_corpus_field":"CLASSES:Wizard={level}",
                            "expected":{{"spell_level":{level},"ability":"CHA"}}}}"#
                    )
                })
                .collect();
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(r#"{{"monster_sla_entries":[{}]}}"#, entries.join(",")),
            )
            .unwrap();
            ScratchSlaFixtureRoot { root }
        }
    }

    impl Drop for ScratchSlaFixtureRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    // Positive control: the real Aboleth's `Hypnotic Pattern` grant carries
    // `12+CHA`, and hypnotic pattern is a 2nd-level spell, so the rule
    // produces 2. Proves the synthetic harness can pass.
    #[test]
    fn a_correct_expected_spell_level_clears_run_monster_sla_bar_check() {
        let scratch = ScratchSlaFixtureRoot::new("correct", &[("Hypnotic Pattern", 2)]);
        let report = run_monster_sla_bar_check(&scratch.root);
        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:monster:aboleth"));
    }

    // MUTATION PROOF, driving the production function end to end: a wrong
    // expected level against the same real record must report a failure and
    // clear nothing.
    #[test]
    fn a_wrong_expected_spell_level_makes_run_monster_sla_bar_check_report_a_failure() {
        let scratch = ScratchSlaFixtureRoot::new("wrong", &[("Hypnotic Pattern", 3)]);
        let report = run_monster_sla_bar_check(&scratch.root);
        assert!(
            report.cleared.is_empty(),
            "a fixture asserting a wrong spell level must never clear, got {:?}",
            report.cleared
        );
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:monster:aboleth"));
    }

    // The all-or-nothing rule, which is the difference between this seam and
    // every earlier one: a unit with one CORRECT row and one WRONG row must
    // not clear on the strength of the correct one.
    #[test]
    fn one_wrong_row_disqualifies_a_unit_whose_other_rows_are_right() {
        let scratch = ScratchSlaFixtureRoot::new(
            "mixed",
            &[("Hypnotic Pattern", 2), ("Illusory Wall", 9)],
        );
        let report = run_monster_sla_bar_check(&scratch.root);
        assert!(
            report.cleared.is_empty(),
            "a unit with a failing row must not be banked on its passing rows, got {:?}",
            report.cleared
        );
        assert!(report.failures.contains_key("scratch:monster:aboleth"));
    }

    // A spell the resolved record does not grant is a failure, never a
    // silent skip — otherwise a fixture naming a typo'd spell would vanish.
    #[test]
    fn a_spell_the_record_does_not_grant_is_a_failure() {
        let scratch = ScratchSlaFixtureRoot::new("absent", &[("Wish", 9)]);
        let report = run_monster_sla_bar_check(&scratch.root);
        assert!(report.cleared.is_empty());
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }
}

// ---------------------------------------------------------------------------
// `kind = monster_ability` — the save-DC seam (SD31-W15-MONSTER-ABILITY).
//
// The `derived`+`grounded` `monster_ability` population is the second-largest
// held cell on the board (264 units). Every seam above it resolves a magnitude
// stated on ONE row. This one is different, and deliberately so: its expected
// value is fixed by TWO independent upstream rows that PF1's own printed rule
// ties together, so a fixture entry cannot be a restatement of the record the
// evaluator reads.
//
// PF1, `Bestiary` Appendix 1 (Universal Monster Rules), "Format":
//
//     "The save DC against a monster's special ability is equal to
//      10 + 1/2 the monster's racial HD + the monster's relevant ability
//      modifier."
//
// PCGen states the already-summed `10 + 1/2 racial HD` term on the ABILITY
// row, as the `DESC:` token's argument for the `%N` its prose introduces with
// the word `DC` (`...succeed at a DC %1 Will save...|15+WIS`). The
// ability-modifier term stays symbolic — it depends on the creature's live
// ability score, which this ingest deliberately does not compute (`SD31-E6-
// F1-002`: a monster's `BONUS:STAT` tokens are ADJUSTMENTS, never scores). It
// states the racial HD itself on a DIFFERENT row, in a different file, as the
// trailing segment of `MONSTERCLASS:<type>:<HD>`.
//
// So this seam's bar has two halves, and BOTH must hold for a unit to clear:
//
//   1. the engine's evaluator, [`monster_ability_save_dc`], run over the
//      compiled ability record, reproduces the fixture's pinned
//      `expected.save_dc_base` / `expected.ability`; and
//   2. [`universal_monster_rule_save_dc_base`], run over the OWNING monster's
//      compiled stat block, reproduces the same base independently.
//
// Half 2 is what makes half 1 non-circular, and it is checked live against the
// chassis rather than against a number copied into the fixture, so a change to
// either the printed-rule arithmetic or the owner's ingested Hit Dice turns
// this check red.
//
// **The linked-ability requirement.** PCGen namespaces a monster's own ability
// rows `<Monster> ~ <Ability>`. An ability whose owner has no monster row in
// its own book is an ORPHAN — a template-namespaced row no monster applies —
// and there is no racial HD to apply the printed rule to, so it is never
// fixtured and never credited.
//
// **Where the two derivations disagree, nothing is credited.**
// `scripts/derive_monster_ability_save_dc_fixtures.py --report` lists every
// such row rather than dropping it; 23 Bestiary 4 rows sit there today.
// ---------------------------------------------------------------------------

/// The six PF1 ability abbreviations, spelled as PCGen spells them in a
/// formula. A BARE abbreviation is the MODIFIER; PCGen spells the score
/// `<ABBREV>SCORE` (this corpus carries `CHASCORE` on one row), so the two are
/// distinguishable and only the modifier form is accepted.
const PF_ABILITY_ABBREVS: [&str; 6] = ["STR", "DEX", "CON", "INT", "WIS", "CHA"];

/// One monster ability's save DC, as far as the corpus states it: the summed
/// `10 + 1/2 racial HD` term, plus the NAME of the ability whose modifier is
/// added at play time.
///
/// Deliberately not a single integer. Resolving the ability-modifier term
/// would require the creature's live ability SCORE, which no corpus row states
/// (`MonsterStatBlock::stat_adjustments` is an adjustment, never a score) —
/// producing one would be exactly the fabrication `SD31-E6-F1-002` already
/// refused for this kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterAbilitySaveDc {
    pub base: i32,
    pub ability: &'static str,
    /// The 1-based `%N` slot of the row's `DESC:` argument list this came
    /// from, so a caller rendering the description knows which placeholder to
    /// replace.
    pub desc_argument_index: usize,
}

/// `10 + 1/2 racial HD`, per the Universal Monster Rule, from the owning
/// monster's own `MONSTERCLASS:<type>:<HD>` token.
///
/// Integer division truncates, which is the rule's own "1/2 HD" rounding
/// (PF1 rounds fractions down unless told otherwise). Returns `None` rather
/// than a guess when the row carries no `MONSTERCLASS:` or its trailing
/// segment is not an integer — the same honest-absence contract
/// [`spell_like_ability_caster_level`] keeps for the same token.
pub fn universal_monster_rule_save_dc_base(monster: &MonsterStatBlock) -> Option<i32> {
    let monster_class = monster.monster_class?;
    let hd = monster_class.rsplit(':').next()?.trim().parse::<i32>().ok()?;
    Some(10 + hd / 2)
}

/// Parses one `DESC:` argument of the shape `<base>+<STAT>` or `<STAT>+<base>`
/// into `(base, ability)`.
///
/// Both operand orders occur in the corpus (`15+WIS` and `CHA+15` are both
/// live spellings). Anything else — a bare variable name (`ClingDC`), a
/// full formula (`10+(HD/2)+CON`), a multiplication (`STR*1.5`) — returns
/// `None`, so a row this cannot read is reported uncovered rather than given
/// a guessed value.
fn parse_flat_base_plus_ability(arg: &str) -> Option<(i32, &'static str)> {
    let (lhs, rhs) = arg.split_once('+')?;
    let (lhs, rhs) = (lhs.trim(), rhs.trim());
    let ability_of = |s: &str| PF_ABILITY_ABBREVS.into_iter().find(|a| *a == s);
    if let Some(ability) = ability_of(rhs) {
        return lhs.parse::<i32>().ok().map(|base| (base, ability));
    }
    if let Some(ability) = ability_of(lhs) {
        return rhs.parse::<i32>().ok().map(|base| (base, ability));
    }
    None
}

/// The 1-based `%N` slots a description introduces with the literal word `DC`.
///
/// Scanned rather than regexed (this crate has no regex dependency, by
/// design). The word boundary matters: `...deals 3d8+%1 points...` is a damage
/// term, not a save DC, and must not be claimed by this seam.
fn dc_placeholder_slots(description: &str) -> Vec<usize> {
    let bytes = description.as_bytes();
    let mut slots = Vec::new();
    let mut i = 0usize;
    while let Some(found) = description[i..].find("DC") {
        let at = i + found;
        i = at + 2;
        let before_ok = at == 0 || !bytes[at - 1].is_ascii_alphanumeric();
        if !before_ok {
            continue;
        }
        let mut j = at + 2;
        // require at least one space between `DC` and `%N`
        let mut spaces = 0usize;
        while j < bytes.len() && bytes[j] == b' ' {
            j += 1;
            spaces += 1;
        }
        if spaces == 0 || j >= bytes.len() || bytes[j] != b'%' {
            continue;
        }
        j += 1;
        let start = j;
        while j < bytes.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        if j == start {
            continue;
        }
        if let Ok(n) = description[start..j].parse::<usize>()
            && n >= 1
        {
            slots.push(n);
        }
    }
    slots
}

/// **The evaluator.** The save DC one compiled monster-ability record states,
/// or `None` when the record states none this engine can read.
///
/// This is the function the monster catalog serves from and the function
/// [`run_monster_ability_bar_check`] holds against the fixture. It reads the
/// COMPILED `monster_chassis` tables (generated from `data/corpus/`) and
/// nothing else — never the upstream `.lst`, never the fixture.
pub fn monster_ability_save_dc(
    record: &crate::rules_core::rules_tables::monster_chassis::MonsterAbilityRecord,
) -> Option<MonsterAbilitySaveDc> {
    let description = record.description?;
    for slot in dc_placeholder_slots(description) {
        let Some(arg) = record.description_variables.get(slot - 1) else {
            continue;
        };
        if let Some((base, ability)) = parse_flat_base_plus_ability(arg) {
            return Some(MonsterAbilitySaveDc { base, ability, desc_argument_index: slot });
        }
    }
    None
}

// ---------------------------------------------------------------------------
// `kind = monster_ability`, second sub-seam (SD31-W16-MONSTER-ABILITY-001) —
// the FULL-FORMULA shape wave 15's receipt named as the honest next lever
// (`artifacts/SD31-W15-MONSTER-ABILITY-save-dc-seam.md`, "What remains held
// in this population"). Everything below is new; nothing above this line
// changed shape or behaviour.
// ---------------------------------------------------------------------------

/// Parses one `DESC:` argument stated as the FULL Universal Monster Rule
/// formula — `10+(HD/2)+CON` / `10+HD/2+CON`, coefficient and divisor
/// literal — into the ability whose modifier is added.
///
/// `TL` is accepted as a synonym for `HD`: PCGen's `TL` term is "total
/// levels" (`PCTLTermEvaluator.resolve()` returns
/// `display.getTotalLevels()`), which for a monster stat block with only a
/// `MONSTERCLASS:` token and no PC class levels layered on sums to exactly
/// the racial HD [`spell_like_ability_caster_level`]'s own doc comment
/// already established this for (`BONUS:VAR|SLA_CL|HD` /
/// `BONUS:VAR|SLA_CL|max(TL,1)` being equivalent on this corpus).
///
/// Deliberately does not resolve a base by itself — unlike
/// [`parse_flat_base_plus_ability`], this shape's ability row states no
/// independent constant, only the rule restated symbolically. The base comes
/// from [`universal_monster_rule_save_dc_base`] over the OWNING monster, in
/// [`monster_ability_formula_save_dc`]. A wrong coefficient (`8+TL/2+CON`) or
/// an extra term (`HD+10+HD/2+CON`) is a genuine deviation from the printed
/// rule and returns `None`, exactly like [`parse_flat_base_plus_ability`]
/// refuses to guess at a shape it cannot read.
fn parse_formula_base_plus_ability(arg: &str) -> Option<&'static str> {
    // None of this corpus's live spellings carries internal whitespace;
    // stripping it costs nothing on the rows that don't have any and makes
    // the match tolerant of the ones that might.
    let compact: String = arg.chars().filter(|c| !c.is_whitespace()).collect();
    let ability_of = |s: &str| PF_ABILITY_ABBREVS.into_iter().find(|a| *a == s);
    for divisor_var in ["HD", "TL"] {
        for prefix in [format!("10+({divisor_var}/2)+"), format!("10+{divisor_var}/2+")] {
            if let Some(stat) = compact.strip_prefix(prefix.as_str())
                && let Some(ability) = ability_of(stat)
            {
                return Some(ability);
            }
        }
    }
    None
}

/// **The evaluator, second sub-seam.** The save DC one compiled
/// monster-ability record states via the full-formula shape, resolved
/// against its OWNING monster's compiled racial HD — or `None` when the row
/// states no such formula, or the formula's shape does not match the printed
/// rule exactly.
///
/// Unlike [`monster_ability_save_dc`], this reads TWO compiled records (the
/// ability record and the owner's stat block) because, for this shape, the
/// ability row alone states no independent constant to read. That is what
/// makes this sub-seam one tier below the flat shape's two-row
/// cross-check — documented as such in
/// `scripts/derive_monster_ability_save_dc_fixtures.py`'s
/// `monster_ability_formula_independence` doc field — and it is still
/// genuinely non-circular: a wrong engine ingest of EITHER compiled record
/// (the ability row's formula shape, or the owner's `MONSTERCLASS`) turns
/// this check red.
pub fn monster_ability_formula_save_dc(
    record: &crate::rules_core::rules_tables::monster_chassis::MonsterAbilityRecord,
    owner: &crate::rules_core::rules_tables::monster_chassis::MonsterStatBlock,
) -> Option<MonsterAbilitySaveDc> {
    let description = record.description?;
    for slot in dc_placeholder_slots(description) {
        let Some(arg) = record.description_variables.get(slot - 1) else {
            continue;
        };
        if let Some(ability) = parse_formula_base_plus_ability(arg) {
            let base = universal_monster_rule_save_dc_base(owner)?;
            return Some(MonsterAbilitySaveDc { base, ability, desc_argument_index: slot });
        }
    }
    None
}

/// One `kind=monster_ability` fixture row, in the shape the committed JSON
/// carries. Its own shape again, not a forced union with [`MonsterFixture`]:
/// this seam pins TWO upstream rows, so it carries the owner's citation
/// alongside the ability's.
#[derive(Debug, Clone)]
pub struct MonsterAbilityFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    pub corpus_field: String,
    pub desc_argument_index: usize,
    pub desc_argument: String,
    pub owner_monster_key: String,
    pub owner_upstream_lst: String,
    pub owner_upstream_line: u64,
    pub owner_monster_class_token: String,
    pub owner_racial_hd: i32,
    pub universal_monster_rule_base: i32,
    pub expected_save_dc_base: i32,
    pub expected_ability: String,
}

/// Reads the `monster_ability_entries` array of the committed fixture file.
pub fn load_monster_ability_fixtures(repo_root: &Path) -> Vec<MonsterAbilityFixture> {
    load_monster_ability_fixtures_field(repo_root, "monster_ability_entries")
}

/// Reads the `monster_ability_formula_entries` array — the second sub-seam's
/// sibling of [`load_monster_ability_fixtures`]. Same committed file, same
/// row shape (an entry from either array resolves through the same owner
/// join), different array because the two shapes derive `expected` by
/// different routes (`scripts/derive_monster_ability_save_dc_fixtures.py`'s
/// `monster_ability_formula_derivation` doc field explains why).
pub fn load_monster_ability_formula_fixtures(repo_root: &Path) -> Vec<MonsterAbilityFixture> {
    load_monster_ability_fixtures_field(repo_root, "monster_ability_formula_entries")
}

fn load_monster_ability_fixtures_field(repo_root: &Path, field: &str) -> Vec<MonsterAbilityFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get(field).and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            MonsterAbilityFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                desc_argument_index: usize::try_from(
                    e["desc_argument_index"].as_u64().expect("desc_argument_index"),
                )
                .expect("a DESC argument index fits in usize"),
                desc_argument: e["desc_argument"].as_str().expect("desc_argument").to_string(),
                owner_monster_key: e["owner_monster_key"]
                    .as_str()
                    .expect("owner_monster_key")
                    .to_string(),
                owner_upstream_lst: e["owner_upstream_lst"]
                    .as_str()
                    .expect("owner_upstream_lst")
                    .to_string(),
                owner_upstream_line: e["owner_upstream_line"]
                    .as_u64()
                    .expect("owner_upstream_line"),
                owner_monster_class_token: e["owner_monster_class_token"]
                    .as_str()
                    .expect("owner_monster_class_token")
                    .to_string(),
                owner_racial_hd: i32::try_from(
                    e["owner_racial_hd"].as_i64().expect("owner_racial_hd"),
                )
                .expect("racial HD fits in i32"),
                universal_monster_rule_base: i32::try_from(
                    e["universal_monster_rule_base"]
                        .as_i64()
                        .expect("universal_monster_rule_base"),
                )
                .expect("a save DC base fits in i32"),
                expected_save_dc_base: i32::try_from(
                    expected["save_dc_base"].as_i64().expect("expected.save_dc_base"),
                )
                .expect("a save DC base fits in i32"),
                expected_ability: expected["ability"]
                    .as_str()
                    .expect("expected.ability")
                    .to_string(),
            }
        })
        .collect()
}

/// The `kind=monster_ability` half of [`run_bar_check`].
///
/// Resolves through the SAME `monster_chassis::MONSTER_BOOKS` registry
/// `v06_work_inventory`'s own `grounded` verdict for `monster_ability` already
/// reads, and the same one the desktop monster catalog serves from — not a
/// second, parallel table.
fn run_monster_ability_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_monster_ability_fixtures(repo_root);
    let fixtures_total = fixtures.len();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for fixture in &fixtures {
        let registry_book = monster_registry_book(&fixture.book);
        let Some(monster_book) = MONSTER_BOOKS.iter().find(|b| b.corpus_book == registry_book)
        else {
            engine_does_not_hold.insert(fixture.unit_id.clone(), fixture.book.clone());
            continue;
        };
        let Some(record) = monster_book.monster_ability_resolve(&fixture.record_key) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "{:?} does not resolve against {registry_book}'s registered monster abilities",
                    fixture.record_key
                ),
            );
            continue;
        };

        // Half 1: the engine's evaluator over the ability record.
        let Some(evaluated) = monster_ability_save_dc(record) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row states {} (DESC argument {}) but the evaluator produced no save \
                     DC at all",
                    fixture.corpus_field, fixture.desc_argument
                ),
            );
            continue;
        };
        if evaluated.base != fixture.expected_save_dc_base
            || evaluated.ability != fixture.expected_ability
        {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row states DESC argument {} (base {} + {}), evaluator produced base \
                     {} + {}",
                    fixture.desc_argument,
                    fixture.expected_save_dc_base,
                    fixture.expected_ability,
                    evaluated.base,
                    evaluated.ability
                ),
            );
            continue;
        }

        // Half 2: the printed Universal Monster Rule over the OWNING monster's
        // stat block, which the evaluator above never reads. This is the half
        // that makes the fixture's expected value independent of the record
        // under test, so it is computed live from the chassis rather than
        // compared against `universal_monster_rule_base` as a stored literal.
        let Some(owner) = monster_book.monster_resolve(&fixture.owner_monster_key) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "owner {:?} does not resolve against {registry_book}'s registered monsters, \
                     so the Universal Monster Rule has no racial HD to apply",
                    fixture.owner_monster_key
                ),
            );
            continue;
        };
        let Some(rule_base) = universal_monster_rule_save_dc_base(owner) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "owner {:?} states MONSTERCLASS {:?} but no readable racial HD",
                    fixture.owner_monster_key, fixture.owner_monster_class_token
                ),
            );
            continue;
        };
        if rule_base != fixture.expected_save_dc_base {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "the Universal Monster Rule over owner {:?} ({}) gives base {}, but the \
                     ability row states base {}",
                    fixture.owner_monster_key,
                    fixture.owner_monster_class_token,
                    rule_base,
                    fixture.expected_save_dc_base
                ),
            );
            continue;
        }

        cleared.insert(fixture.unit_id.clone());
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

/// The second sub-seam's half of [`run_bar_check`] — `kind=monster_ability`
/// rows whose DC argument states the FULL Universal Monster Rule formula
/// rather than a summed literal. Resolves through the SAME
/// `monster_chassis::MONSTER_BOOKS` registry [`run_monster_ability_bar_check`]
/// does; a separate function only because the per-fixture check itself
/// differs (the evaluator needs the owner resolved BEFORE it can produce a
/// value at all, since this shape's ability row states no independent
/// constant).
fn run_monster_ability_formula_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_monster_ability_formula_fixtures(repo_root);
    let fixtures_total = fixtures.len();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for fixture in &fixtures {
        let registry_book = monster_registry_book(&fixture.book);
        let Some(monster_book) = MONSTER_BOOKS.iter().find(|b| b.corpus_book == registry_book)
        else {
            engine_does_not_hold.insert(fixture.unit_id.clone(), fixture.book.clone());
            continue;
        };
        let Some(record) = monster_book.monster_ability_resolve(&fixture.record_key) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "{:?} does not resolve against {registry_book}'s registered monster abilities",
                    fixture.record_key
                ),
            );
            continue;
        };
        let Some(owner) = monster_book.monster_resolve(&fixture.owner_monster_key) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "owner {:?} does not resolve against {registry_book}'s registered monsters, \
                     so the Universal Monster Rule has no racial HD to apply",
                    fixture.owner_monster_key
                ),
            );
            continue;
        };

        // Independent half: the printed rule over the owner's compiled
        // MONSTERCLASS, which the fixture's expected value was pinned
        // against at derivation time. Checked live, not trusted from the
        // stored `universal_monster_rule_base` literal.
        let Some(rule_base) = universal_monster_rule_save_dc_base(owner) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "owner {:?} states MONSTERCLASS {:?} but no readable racial HD",
                    fixture.owner_monster_key, fixture.owner_monster_class_token
                ),
            );
            continue;
        };
        if rule_base != fixture.expected_save_dc_base {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "the Universal Monster Rule over owner {:?} ({}) gives base {}, but the \
                     fixture pins base {}",
                    fixture.owner_monster_key,
                    fixture.owner_monster_class_token,
                    rule_base,
                    fixture.expected_save_dc_base
                ),
            );
            continue;
        }

        // The evaluator: does the ability row's formula shape resolve to the
        // SAME base and ability the fixture pins?
        let Some(evaluated) = monster_ability_formula_save_dc(record, owner) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row states {} (DESC argument {}) but the evaluator recognized no \
                     Universal Monster Rule formula shape",
                    fixture.corpus_field, fixture.desc_argument
                ),
            );
            continue;
        };
        if evaluated.base != fixture.expected_save_dc_base || evaluated.ability != fixture.expected_ability
        {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row states DESC argument {} (expected base {} + {}), evaluator \
                     produced base {} + {}",
                    fixture.desc_argument,
                    fixture.expected_save_dc_base,
                    fixture.expected_ability,
                    evaluated.base,
                    evaluated.ability
                ),
            );
            continue;
        }

        cleared.insert(fixture.unit_id.clone());
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}


// ---------------------------------------------------------------------------
// `kind = companion` — the evaluator seam this cycle (SD31-W15-COMPANION-001)
// builds. Everything below is new; nothing above this line changed shape.
// ---------------------------------------------------------------------------

/// What a companion creature row's `BONUS:WEAPONPROF=<attack>|DAMAGE|<formula>`
/// token means, as a value this repo can evaluate.
///
/// # The rule this exists to serve
///
/// PF1 CRB p.182, *Natural Attacks*: **"If a creature has only one natural
/// attack, it adds 1-1/2 times its Strength bonus on damage rolls."** PCGen
/// encodes the *extra half* as a separate `BONUS:…|DAMAGE|max(0,(STR/2))`
/// token, because the base attack already applies the full modifier. The
/// `max(0,…)` wrapper is why a Strength PENALTY is never multiplied: the rule
/// is stated about a Strength *bonus*, and a penalty applies once, in full,
/// through the base attack alone.
///
/// # The rule is NOT a corpus invariant, and this type does not pretend it is
///
/// Re-derived corpus-wide 2026-08-19 over all 927 ingested `companion` records
/// (`data/corpus/*/companion/*.json`), crossing natural-attack count against
/// half-Strength-token presence:
///
/// ```text
/// (natural attacks, half-STR tokens) -> records
///   (0,0)  54    (0,1)   2
///   (1,0) 129    (1,1) 185
///   (2,0)  74    (2,1)   3
///   (3,0)   3
/// ```
///
/// **129 single-attack rows carry no such token at all**, and 5 rows carry one
/// where the count rule does not call for it. Upstream PCGen simply does not
/// state this rule uniformly. So the token's PRESENCE is never inferred from
/// the attack count anywhere in this seam — not by the chassis, not by the
/// catalog, and not by the fixture generator. What is evaluated is the formula
/// the row actually states, and what the fixture pins is what PF1's own
/// halve-and-round-down convention makes of that formula. A seam that derived
/// presence from the count would report 129 false failures and would be
/// asserting a rule against data that does not follow it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompanionStrengthDamage {
    /// `max(0,(STR/2))` / `max(0,STR/2)` — half the Strength MODIFIER, rounded
    /// down (PF1 CRB p.9: *"whenever you are asked to halve a number, round
    /// down"*), never below zero. 115 of the 227 held `derived` companion units
    /// carry exactly this and nothing else.
    HalfStrengthNeverNegative,
    /// `STR` — the full Strength modifier.
    FullStrength,
    /// `-STR` — the full Strength modifier, negated.
    NegatedFullStrength,
    /// A flat integer literal (`5`, `-5`) — no Strength term at all.
    Flat(i32),
}

impl CompanionStrengthDamage {
    /// The wire/fixture token for this shape. Spelled once, here, so the
    /// fixture's `expected.shape` and any consumer name the same string.
    pub fn shape_name(self) -> &'static str {
        match self {
            CompanionStrengthDamage::HalfStrengthNeverNegative => "half_strength_never_negative",
            CompanionStrengthDamage::FullStrength => "full_strength",
            CompanionStrengthDamage::NegatedFullStrength => "negated_full_strength",
            CompanionStrengthDamage::Flat(_) => "flat",
        }
    }
}

/// Parses one `BONUS:WEAPONPROF=<attack>|DAMAGE|` token's formula half.
///
/// **Refuses rather than guesses.** `STR/2` and `-(STR/2)` — 3 rows corpus-wide
/// — are deliberately NOT parsed: an unclamped halving's value at a NEGATIVE
/// odd Strength modifier depends on whether PCGen's formula engine floors or
/// truncates, this repo has no proof of which, and a wrong number in a damage
/// column is worse than an absent one (`companion_chassis`'s own
/// `parse_stat_adjustments` doctrine). `max(0,…)` has no such ambiguity: the
/// clamp decides every negative case and floor == truncate for every positive
/// one, so the clamped shape is exact over all integers.
pub fn parse_companion_strength_damage(formula: &str) -> Option<CompanionStrengthDamage> {
    let f: String = formula.chars().filter(|c| !c.is_whitespace()).collect();
    match f.as_str() {
        "max(0,(STR/2))" | "max(0,STR/2)" => {
            Some(CompanionStrengthDamage::HalfStrengthNeverNegative)
        }
        "STR" => Some(CompanionStrengthDamage::FullStrength),
        "-STR" => Some(CompanionStrengthDamage::NegatedFullStrength),
        other => other.parse::<i32>().ok().map(CompanionStrengthDamage::Flat),
    }
}

/// The extra damage the token grants at a given Strength MODIFIER.
///
/// `div_euclid(2)` rather than `/ 2`: Rust's `/` truncates toward zero, PF1
/// rounds DOWN, and the two disagree on every negative odd modifier. The clamp
/// hides that disagreement for [`CompanionStrengthDamage::HalfStrengthNeverNegative`]
/// specifically, so this is belt-and-braces there — but it is the rounding this
/// program's rule doctrine states, written once, where a later unclamped shape
/// would inherit it rather than re-decide it.
pub fn evaluate_companion_strength_damage(
    damage: CompanionStrengthDamage,
    strength_modifier: i32,
) -> i32 {
    match damage {
        CompanionStrengthDamage::HalfStrengthNeverNegative => {
            strength_modifier.div_euclid(2).max(0)
        }
        CompanionStrengthDamage::FullStrength => strength_modifier,
        CompanionStrengthDamage::NegatedFullStrength => -strength_modifier,
        CompanionStrengthDamage::Flat(n) => n,
    }
}

/// The player-facing rendering of the same parsed token — the PRODUCTION half
/// of this seam, called by `apps/desktop/src-tauri/src/companion_catalog.rs`.
///
/// A catalog browser has no character, so it cannot show the evaluated number;
/// it shows what the row grants, in the rule's own words. Same posture as
/// `format_spell_range_formula`, which `spell_catalog` calls for a formula
/// whose caster level is likewise not known at browse time.
pub fn format_companion_strength_damage(damage: CompanionStrengthDamage) -> String {
    match damage {
        CompanionStrengthDamage::HalfStrengthNeverNegative => {
            "+1/2 Str modifier (minimum +0)".to_string()
        }
        CompanionStrengthDamage::FullStrength => "+Str modifier".to_string(),
        CompanionStrengthDamage::NegatedFullStrength => "-Str modifier".to_string(),
        CompanionStrengthDamage::Flat(n) if n >= 0 => format!("+{n}"),
        CompanionStrengthDamage::Flat(n) => format!("{n}"),
    }
}

/// A PF1 skill-check bonus computed as the DIFFERENCE between two ability
/// modifiers -- PCGen's `BONUS:SKILL|<skills>|<A>-<B>` encoding.
///
/// # Why a subtraction needs its own type rather than reusing arithmetic inline
///
/// The formula names two DISTINCT ability scores (never the same one twice in
/// the 136 corpus-wide occurrences, re-derived 2026-08-19), so evaluating it
/// needs two modifiers, not one -- structurally different from
/// [`CompanionStrengthDamage`]'s single-Strength-term family. Keeping the two
/// ability names on the parsed value (rather than resolving immediately to a
/// number) is what lets [`format_companion_skill_ability_diff`] name them in
/// the catalog browser, which has no character and therefore no modifiers to
/// subtract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillAbilityDiffFormula {
    /// The ability whose modifier is added: `"DEX"` in `DEX-STR`.
    pub plus: &'static str,
    /// The ability whose modifier is subtracted: `"STR"` in `DEX-STR`.
    pub minus: &'static str,
}

/// The six PF1 ability abbreviations this parser accepts as either operand.
/// Refusing an unrecognised three-letter token (rather than accepting any
/// three uppercase letters) is what keeps a corrupted or novel corpus
/// spelling from silently parsing as a formula it is not.
const ABILITY_ABBREVIATIONS: [&str; 6] = ["STR", "DEX", "CON", "INT", "WIS", "CHA"];

/// Parses one `BONUS:SKILL|<skills>|` token's formula half.
///
/// **Refuses rather than guesses.** Accepts only `<ABBR>-<ABBR>` where both
/// sides are one of the six PF1 ability abbreviations and the two differ --
/// `companion_chassis::SkillAbilityDiffBonus`'s transcription already drops
/// anything with no `-` at all (a flat `TYPE=Racial` number), so a formula
/// reaching this parser is either exactly this shape or a corpus spelling
/// this seam has never seen and should refuse rather than mis-evaluate.
pub fn parse_companion_skill_ability_diff(formula: &str) -> Option<SkillAbilityDiffFormula> {
    let f: String = formula.chars().filter(|c| !c.is_whitespace()).collect();
    let (plus, minus) = f.split_once('-')?;
    if plus == minus {
        return None;
    }
    let plus = ABILITY_ABBREVIATIONS.iter().find(|&&a| a == plus)?;
    let minus = ABILITY_ABBREVIATIONS.iter().find(|&&a| a == minus)?;
    Some(SkillAbilityDiffFormula { plus, minus })
}

/// The skill-check bonus at the two given ability MODIFIERS (never scores --
/// same discipline `companion_chassis::StatAdjustment` states).
///
/// No rounding question arises here (unlike
/// [`evaluate_companion_strength_damage`]'s halving): a difference of two
/// already-integer modifiers is exact.
pub fn evaluate_companion_skill_ability_diff(
    formula: SkillAbilityDiffFormula,
    plus_modifier: i32,
    minus_modifier: i32,
) -> i32 {
    let _ = formula; // the ability NAMES only select which modifier is which; the caller supplies both.
    plus_modifier - minus_modifier
}

/// The player-facing rendering of the parsed token — the PRODUCTION half of
/// this seam, called by `apps/desktop/src-tauri/src/companion_catalog.rs`.
/// Same posture as [`format_companion_strength_damage`]: a catalog browser
/// has no character and therefore no modifiers to subtract, so it shows the
/// rule in words.
pub fn format_companion_skill_ability_diff(formula: SkillAbilityDiffFormula) -> String {
    let plus_word = ability_word(formula.plus);
    let minus_word = ability_word(formula.minus);
    format!("{plus_word} modifier \u{2212} {minus_word} modifier")
}

fn ability_word(abbr: &str) -> &'static str {
    match abbr {
        "STR" => "Str",
        "DEX" => "Dex",
        "CON" => "Con",
        "INT" => "Int",
        "WIS" => "Wis",
        "CHA" => "Cha",
        _ => "?",
    }
}

/// One `kind=companion` skill-bonus fixture row. A sibling top-level
/// `companion_skill_entries` array in the same committed fixture JSON, kept
/// separate from `companion_entries` for the reason [`MonsterFixture`] states
/// about `monster_entries`: `(strength_modifier, damage_bonus)` pairs and
/// `(plus_modifier, minus_modifier, skill_bonus)` triples are different
/// shapes and a forced-generic union would mean nothing for either.
#[derive(Debug, Clone)]
pub struct CompanionSkillFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    /// Every skill the token names, e.g. `["Climb", "Swim"]`.
    pub skills: Vec<String>,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    pub corpus_field: String,
    /// The ability the fixture expects the evaluator to ADD, e.g. `"DEX"` in
    /// `DEX-STR` — read off the corpus row's own formula text by the
    /// generator, independently of anything this evaluator parses. Wave-16
    /// adversarial review (circularity lens): without this pinned and
    /// compared, [`evaluate_companion_skill_ability_diff`] discarding its
    /// `formula` argument meant the bar check verified only that SOME
    /// ability-diff token existed on the shipped record, never that it named
    /// the SAME two abilities the corpus row does — a mutated shipped
    /// formula (e.g. `DEX-STR` -> `CHA-INT`) left the gate green. Compared
    /// against `parsed.plus`/`parsed.minus` below so a transcription
    /// regression in WHICH abilities are named turns this check red, not
    /// just a regression in the arithmetic.
    pub plus_ability: String,
    /// The ability the fixture expects the evaluator to SUBTRACT, e.g.
    /// `"STR"` in `DEX-STR`. Same provenance and purpose as `plus_ability`.
    pub minus_ability: String,
    /// `(plus_modifier, minus_modifier, expected_bonus)` triples, computed by
    /// `scripts/derive_companion_skill_bonus_fixtures.py` from a plain
    /// integer subtraction — never read back from this repo's evaluator.
    pub expected_at: Vec<(i32, i32, i32)>,
}

/// Reads the `companion_skill_entries` array of the same committed fixture
/// file [`load_fixtures`] reads `entries` from.
pub fn load_companion_skill_fixtures(repo_root: &Path) -> Vec<CompanionSkillFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("companion_skill_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            CompanionSkillFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                skills: e["skills"]
                    .as_array()
                    .expect("skills")
                    .iter()
                    .map(|s| s.as_str().expect("skill").to_string())
                    .collect(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                plus_ability: expected["plus_ability"].as_str().expect("expected.plus_ability").to_string(),
                minus_ability: expected["minus_ability"].as_str().expect("expected.minus_ability").to_string(),
                expected_at: expected["skill_bonus_at_modifiers"]
                    .as_array()
                    .expect("expected.skill_bonus_at_modifiers")
                    .iter()
                    .map(|p| {
                        (
                            i32::try_from(p["plus_modifier"].as_i64().expect("plus_modifier"))
                                .expect("a modifier fits in i32"),
                            i32::try_from(p["minus_modifier"].as_i64().expect("minus_modifier"))
                                .expect("a modifier fits in i32"),
                            i32::try_from(p["skill_bonus"].as_i64().expect("skill_bonus"))
                                .expect("a skill bonus fits in i32"),
                        )
                    })
                    .collect(),
            }
        })
        .collect()
}

/// The `kind=companion` skill-bonus half of [`run_bar_check`]. Runs against
/// the SHIPPED tables, exactly as [`run_companion_bar_check`] does and for
/// the same reason: a transcription that dropped the token must fail here,
/// not pass silently against a corpus file no player reads.
fn run_companion_skill_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_companion_skill_fixtures(repo_root);
    let fixtures_total = fixtures.len();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for fixture in &fixtures {
        // Unlike `run_companion_bar_check` (whose own comment explains why it
        // deliberately does not apply this alias — no committed
        // Strength-damage fixture had ever named `bestiary`), this seam DOES
        // pin a real `bestiary:companion:rat_dire` entry, so the spelling gap
        // `monster_registry_book` exists for is no longer merely theoretical
        // here. Reused rather than duplicated: `companion_chassis::
        // COMPANION_BOOKS` keys Bestiary 1 `beastiary` (spelled that way
        // since SD-22) while the work-inventory `book` field for the same
        // records is `bestiary`, the exact one-alias gap that function
        // states.
        let Some(book) = companion_book(monster_registry_book(&fixture.book)) else {
            engine_does_not_hold.insert(fixture.unit_id.clone(), fixture.book.clone());
            continue;
        };
        let Some(record) = book.companion_resolve(&fixture.record_key) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "{:?} does not resolve against {}'s registered companions",
                    fixture.record_key, fixture.book
                ),
            );
            continue;
        };
        let Some(bonus) = record
            .skill_ability_diff_bonuses
            .iter()
            .find(|b| b.skills.iter().map(|s| s.to_string()).collect::<Vec<_>>() == fixture.skills)
        else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row states {} but the shipped record carries no matching \
                     BONUS:SKILL|{}|… token at all",
                    fixture.corpus_field,
                    fixture.skills.join(",")
                ),
            );
            continue;
        };
        let Some(parsed) = parse_companion_skill_ability_diff(bonus.formula) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row states {} but the evaluator could not parse an ability-diff shape \
                     from {:?}",
                    fixture.corpus_field, bonus.formula
                ),
            );
            continue;
        };
        // The independence check `run_companion_bar_check`'s sibling seam
        // already carries (`parsed.shape_name() != fixture.expected_shape`):
        // confirm the evaluator parsed the SAME two abilities the corpus row
        // states, not merely SOME ability-diff shape. Without this, a
        // transcription regression that renamed the shipped formula (e.g.
        // `DEX-STR` -> `CHA-INT`) would leave every arithmetic check below
        // vacuously green, since the (plus_modifier, minus_modifier,
        // expected_bonus) triples are pinned as pure arithmetic and never
        // named which ability is which (wave-16 adversarial review finding).
        if parsed.plus != fixture.plus_ability || parsed.minus != fixture.minus_ability {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row states {} (expected ability-diff {}-{}) but the evaluator parsed \
                     {}-{} from the shipped record",
                    fixture.corpus_field, fixture.plus_ability, fixture.minus_ability, parsed.plus, parsed.minus
                ),
            );
            continue;
        }
        let mut mismatch = None;
        for &(plus_modifier, minus_modifier, expected_bonus) in &fixture.expected_at {
            let got = evaluate_companion_skill_ability_diff(parsed, plus_modifier, minus_modifier);
            if got != expected_bonus {
                mismatch = Some(format!(
                    "corpus row {:?} at modifiers ({plus_modifier}, {minus_modifier}): expected \
                     skill bonus {expected_bonus}, evaluator produced {got}",
                    fixture.corpus_field
                ));
                break;
            }
        }
        match mismatch {
            Some(message) => {
                failures.insert(fixture.unit_id.clone(), message);
            }
            None if fixture.expected_at.is_empty() => {
                // A fixture that pins no evaluated value asserts nothing about
                // the evaluator. Refused rather than counted (Decision 1(a)).
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "fixture for {:?} pins no (plus_modifier, minus_modifier, skill_bonus) \
                         triple at all, so it asserts nothing",
                        fixture.corpus_field
                    ),
                );
            }
            None => {
                cleared.insert(fixture.unit_id.clone());
            }
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

/// A companion ABILITY's save DC, stated entirely on its own `DESC:` token —
/// unlike [`MonsterAbilitySaveDc`]'s formula-shape sub-seam, whose ability row
/// states no independent constant and borrows the OWNING monster's Universal
/// Monster Rule base, a `kind=companion` ability row states its own base
/// constant inline (10, 11 or 12 corpus-wide, never assumed to be 10), so
/// this shape needs no owner join at all.
///
/// Deliberately not a single integer, same reason [`MonsterAbilitySaveDc`]
/// gives: resolving the ability-modifier term needs a live character this
/// catalog browser does not have.
///
/// `PartialOrd`/`Ord` derived so the bar check can de-duplicate multiple
/// `DESC:` variants' parsed shapes through a `BTreeSet` when checking they
/// all agree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompanionSaveDcFormula {
    pub base: i32,
    pub includes_half_hd: bool,
    pub ability: &'static str,
}

/// Parses one companion ability's `DESC:` trailing argument of the shape
/// `<base>+<ability>` or `<base>+<HD|TL>/2+<ability>` (both spellings the
/// pinned oracle states; a parenthesised `(HD/2)` form does not occur in this
/// kind's corpus today but is accepted for the same reason
/// `parse_formula_base_plus_ability` accepts it for `monster_ability`).
///
/// **Refuses rather than guesses.** A bare variable NAME (`ClingDC` — needs a
/// chassis field this seam does not add), a lone `HD`/`TL` term with no
/// ability at all (`1+HD/2` — Whiptail Centipede (Giant) ~ Wall Climber's
/// OTHER ability, a duration, not a DC), a multiplication (`4*CONSCORE`), or
/// an ability SCORE spelling (`CONSCORE`) all return `None`.
pub fn parse_companion_save_dc_formula(arg: &str) -> Option<CompanionSaveDcFormula> {
    let compact: String = arg.chars().filter(|c| !c.is_whitespace()).collect();
    let ability_of = |s: &str| ABILITY_ABBREVIATIONS.into_iter().find(|a| *a == s);
    for divisor_var in ["HD", "TL"] {
        for infix in [format!("+({divisor_var}/2)+"), format!("+{divisor_var}/2+")] {
            if let Some((lhs, rhs)) = compact.split_once(infix.as_str())
                && let Some(ability) = ability_of(rhs)
                && let Ok(base) = lhs.parse::<i32>()
            {
                return Some(CompanionSaveDcFormula { base, includes_half_hd: true, ability });
            }
        }
    }
    let (lhs, rhs) = compact.split_once('+')?;
    let ability = ability_of(rhs)?;
    let base = lhs.parse::<i32>().ok()?;
    Some(CompanionSaveDcFormula { base, includes_half_hd: false, ability })
}

/// The save DC at the given Hit Dice and ability MODIFIER (never a score).
/// PF1's "1/2 HD" rounds DOWN; Hit Dice is never negative in this corpus, so
/// `div_euclid` is exact (no negative-operand rounding question, unlike
/// [`evaluate_companion_strength_damage`]'s halving of a Strength modifier
/// that can itself be negative).
pub fn evaluate_companion_save_dc_formula(
    formula: CompanionSaveDcFormula,
    hit_dice: i32,
    ability_modifier: i32,
) -> i32 {
    let half_hd = if formula.includes_half_hd { hit_dice.div_euclid(2) } else { 0 };
    formula.base + half_hd + ability_modifier
}

/// The player-facing rendering — the PRODUCTION half of this seam, called by
/// `apps/desktop/src-tauri/src/companion_catalog.rs`. Same posture as
/// [`format_companion_skill_ability_diff`]: a catalog browser has no
/// character, so it shows the rule in words rather than a computed number.
pub fn format_companion_save_dc_formula(formula: CompanionSaveDcFormula) -> String {
    let ability = ability_word(formula.ability);
    if formula.includes_half_hd {
        format!("{} + 1/2 HD + {ability} modifier", formula.base)
    } else {
        format!("{} + {ability} modifier", formula.base)
    }
}

/// One `kind=companion` save-DC fixture row. A sibling top-level
/// `companion_save_dc_entries` array in the same committed fixture JSON.
#[derive(Debug, Clone)]
pub struct CompanionSaveDcFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    pub corpus_field: String,
    pub expected_base: i32,
    pub expected_includes_half_hd: bool,
    pub expected_ability: String,
    /// `(hit_dice, ability_modifier, save_dc)` triples, computed by
    /// `scripts/derive_companion_save_dc_fixtures.py` from PF1's own
    /// "1/2 HD rounds down" rule -- never read back from this repo's
    /// evaluator.
    pub expected_at: Vec<(i32, i32, i32)>,
}

/// Reads the `companion_save_dc_entries` array of the same committed fixture
/// file [`load_fixtures`] reads `entries` from.
pub fn load_companion_save_dc_fixtures(repo_root: &Path) -> Vec<CompanionSaveDcFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("companion_save_dc_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            CompanionSaveDcFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                expected_base: i32::try_from(expected["base"].as_i64().expect("expected.base"))
                    .expect("a base fits in i32"),
                expected_includes_half_hd: expected["includes_half_hd"]
                    .as_bool()
                    .expect("expected.includes_half_hd"),
                expected_ability: expected["ability"].as_str().expect("expected.ability").to_string(),
                expected_at: expected["save_dc_at"]
                    .as_array()
                    .expect("expected.save_dc_at")
                    .iter()
                    .map(|p| {
                        (
                            i32::try_from(p["hit_dice"].as_i64().expect("hit_dice"))
                                .expect("hit_dice fits in i32"),
                            i32::try_from(p["ability_modifier"].as_i64().expect("ability_modifier"))
                                .expect("ability_modifier fits in i32"),
                            i32::try_from(p["save_dc"].as_i64().expect("save_dc"))
                                .expect("save_dc fits in i32"),
                        )
                    })
                    .collect(),
            }
        })
        .collect()
}

/// The `kind=companion` save-DC half of [`run_bar_check`]. Runs against the
/// SHIPPED tables (`companion_chassis::COMPANION_BOOKS`), exactly as
/// [`run_companion_skill_bar_check`] does and for the same reason: a
/// transcription that dropped the `DESC:` argument must fail here, not pass
/// silently against a corpus file no player reads.
fn run_companion_save_dc_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_companion_save_dc_fixtures(repo_root);
    let fixtures_total = fixtures.len();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for fixture in &fixtures {
        // Joined on the inventory's book id directly, same choice
        // `run_companion_bar_check` makes and for the same reason: both
        // committed books (Ultimate Wilderness, Bestiary 4) match
        // `companion_chassis::COMPANION_BOOKS`'s own `corpus_book` spelling
        // with no `bestiary` -> `beastiary` alias needed.
        let Some(book) = companion_book(&fixture.book) else {
            engine_does_not_hold.insert(fixture.unit_id.clone(), fixture.book.clone());
            continue;
        };
        let Some(record) = book.companion_ability_resolve(&fixture.record_key) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "{:?} does not resolve against {}'s registered companion abilities",
                    fixture.record_key, fixture.book
                ),
            );
            continue;
        };
        // Every DESC: argument the shipped record carries, from the plain
        // field and from every conditional variant — a record may state the
        // SAME formula twice, once per PREVARLT/PREVARGTEQ
        // companion-advancement-tier gate (Assassin Bug (Giant) ~ Poison).
        let mut candidates: Vec<&'static str> = Vec::new();
        candidates.extend(record.description_variables.iter().copied());
        for variant in record.description_variants {
            candidates.extend(variant.variables.iter().copied());
        }
        let mut parsed_shapes: BTreeSet<CompanionSaveDcFormula> = BTreeSet::new();
        for candidate in &candidates {
            if let Some(shape) = parse_companion_save_dc_formula(candidate) {
                parsed_shapes.insert(shape);
            }
        }
        let parsed = match parsed_shapes.len() {
            0 => {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} but the shipped record carries no DESC: argument \
                         the evaluator can parse a save-DC shape from (candidates: {candidates:?})",
                        fixture.corpus_field
                    ),
                );
                continue;
            }
            1 => *parsed_shapes.iter().next().expect("len 1"),
            n => {
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "corpus row states {} but the shipped record carries {n} DISTINCT \
                         parseable save-DC shapes across its DESC: arguments — ambiguous",
                        fixture.corpus_field
                    ),
                );
                continue;
            }
        };
        // Assert IDENTITY, not just that SOME shape parsed (the wave-16
        // adversarial-review lesson: a bar check that counts things but
        // never checks WHICH things is a gate hole). A transcription
        // regression that silently changed the base, dropped the half-HD
        // term, or swapped the ability would otherwise leave every
        // arithmetic check below vacuously comparing against ITS OWN wrong
        // value.
        if parsed.base != fixture.expected_base
            || parsed.includes_half_hd != fixture.expected_includes_half_hd
            || parsed.ability != fixture.expected_ability
        {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row {:?} states base={} includes_half_hd={} ability={:?}, but the \
                     evaluator parsed base={} includes_half_hd={} ability={:?}",
                    fixture.corpus_field,
                    fixture.expected_base,
                    fixture.expected_includes_half_hd,
                    fixture.expected_ability,
                    parsed.base,
                    parsed.includes_half_hd,
                    parsed.ability
                ),
            );
            continue;
        }
        let mut mismatch = None;
        for &(hit_dice, ability_modifier, expected_dc) in &fixture.expected_at {
            let got = evaluate_companion_save_dc_formula(parsed, hit_dice, ability_modifier);
            if got != expected_dc {
                mismatch = Some(format!(
                    "corpus row {:?} at (hit_dice={hit_dice}, ability_modifier={ability_modifier}): \
                     expected save DC {expected_dc}, evaluator produced {got}",
                    fixture.corpus_field
                ));
                break;
            }
        }
        match mismatch {
            Some(message) => {
                failures.insert(fixture.unit_id.clone(), message);
            }
            None if fixture.expected_at.is_empty() => {
                // A fixture that pins no evaluated value asserts nothing
                // about the evaluator. Refused rather than counted (Decision
                // 1(a)).
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "fixture for {:?} pins no (hit_dice, ability_modifier, save_dc) triple at \
                         all, so it asserts nothing",
                        fixture.corpus_field
                    ),
                );
            }
            None => {
                cleared.insert(fixture.unit_id.clone());
            }
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

// ------------------------------------------------------------------------------------------
// SD-31 wave 26: `class_feature_description_entries` -- the formula-interpreter-backed
// `%N` DESC-placeholder resolution bar check.
// ------------------------------------------------------------------------------------------

/// One `class_feature_description_entries` fixture row: a real, live grant fact whose corpus
/// `DESC:` token carries an unresolved `%N` this repo's formula interpreter (`OPERATOR-RULINGS-
/// 2026-08-21.md` §20) can now resolve via a same-record `BONUS:VAR` chain seeded with the
/// character's class level. `expected_value_at_level_by_arg` is transcribed by `scripts/
/// derive_class_feature_description_fixtures.py`, straight from the pinned upstream `.lst`
/// bytes, through that script's own from-scratch (Python, cross-language) evaluator -- never
/// read back from this repo's Rust interpreter.
#[derive(Debug, Clone)]
pub struct ClassFeatureDescriptionFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub class: String,
    pub class_level_var: String,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    pub corpus_field: String,
    /// PCGen variable name -> (character level -> expected resolved integer).
    pub expected_value_at_level_by_arg: BTreeMap<String, BTreeMap<u8, i64>>,
    /// SD-31 wave 27: a FIXED, assumed test ability-modifier input (bare PCGen abbreviation --
    /// `STR`/`DEX`/`CON`/`INT`/`WIS`/`CHA` -- -> assumed modifier value), used ONLY when the
    /// target record's own `BONUS:VAR` formula references an ability abbreviation. Absent (empty
    /// map) for every fixture that does not need one -- the twelve wave-26 targets are all
    /// level-only chains and this defaults to an empty map for them, changing nothing about how
    /// they resolve. Unlike `expected_value_at_level_by_arg`, this is NOT "transcribed from the
    /// corpus" (a character's own ability score has no corpus representation at all): it is a
    /// fixed test input this script and this bar-check both agree to feed the SAME record, so
    /// that the fixture proves the FORMULA is evaluated correctly for a given input, the same
    /// thing every other fixture in this family proves for the level dimension. Parsed by
    /// [`load_class_feature_description_fixtures`] below (this struct is hand-parsed off
    /// `serde_json::Value`, not `#[derive(Deserialize)]`) -- absent from the JSON row entirely,
    /// as every pre-wave-27 entry is, parses to an empty map, never an error.
    pub ability_modifier_inputs: BTreeMap<String, i16>,
}

/// Reads the `class_feature_description_entries` array of the committed fixture file.
pub fn load_class_feature_description_fixtures(repo_root: &Path) -> Vec<ClassFeatureDescriptionFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("class_feature_description_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"]["value_at_level_by_arg"];
            let mut expected_value_at_level_by_arg: BTreeMap<String, BTreeMap<u8, i64>> = BTreeMap::new();
            if let Some(obj) = expected.as_object() {
                for (arg_name, by_level) in obj {
                    let mut per_level = BTreeMap::new();
                    if let Some(by_level_obj) = by_level.as_object() {
                        for (level_str, value) in by_level_obj {
                            let level: u8 = level_str.parse().expect("level key parses as u8");
                            per_level.insert(level, value.as_i64().expect("expected value is an i64"));
                        }
                    }
                    expected_value_at_level_by_arg.insert(arg_name.clone(), per_level);
                }
            }
            let mut ability_modifier_inputs: BTreeMap<String, i16> = BTreeMap::new();
            if let Some(obj) = e.get("ability_modifier_inputs").and_then(|v| v.as_object()) {
                for (abbrev, value) in obj {
                    let value = value
                        .as_i64()
                        .unwrap_or_else(|| panic!("ability_modifier_inputs.{abbrev} must be an integer"));
                    ability_modifier_inputs.insert(
                        abbrev.clone(),
                        i16::try_from(value)
                            .unwrap_or_else(|_| panic!("ability_modifier_inputs.{abbrev}={value} out of i16 range")),
                    );
                }
            }
            ClassFeatureDescriptionFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                class: e["class"].as_str().expect("class").to_string(),
                class_level_var: e["class_level_var"].as_str().expect("class_level_var").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                expected_value_at_level_by_arg,
                ability_modifier_inputs,
            }
        })
        .collect()
}

/// Builds the [`AbilityModifiers`] value a `class_feature_description_entries` fixture's own
/// `ability_modifier_inputs` names, defaulting every unnamed ability to `0` -- the same "seed
/// what is known, never guess the rest" posture the rest of this fixture family uses.
fn ability_modifiers_from_fixture_inputs(
    inputs: &BTreeMap<String, i16>,
) -> crate::rules_core::pilot_compute::AbilityModifiers {
    crate::rules_core::pilot_compute::AbilityModifiers {
        strength: inputs.get("STR").copied().unwrap_or(0),
        dexterity: inputs.get("DEX").copied().unwrap_or(0),
        constitution: inputs.get("CON").copied().unwrap_or(0),
        intelligence: inputs.get("INT").copied().unwrap_or(0),
        wisdom: inputs.get("WIS").copied().unwrap_or(0),
        charisma: inputs.get("CHA").copied().unwrap_or(0),
    }
}

/// The `class_feature_description_entries` half of [`run_bar_check`]. Runs the REAL production
/// resolver (`pilot_compute::class_feature_grant_consumer::resolve_pcgen_var_chain`, which
/// drives the proven `formula_interpreter::PcgenFormulaEvaluator`) against the SAME live corpus
/// record (`class_feature_grant_consumer::class_feature_record_tokens`) the shipped engine reads
/// -- never a second, hand-rolled Rust evaluator -- at every level the fixture pins, for every
/// PCGen variable name the fixture names. A unit clears only when EVERY (arg, level) pair
/// matches; any mismatch, or any level the production resolver could not reach at all, fails the
/// whole unit rather than partially crediting it.
fn run_class_feature_description_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_class_feature_description_fixtures(repo_root);
    let fixtures_total = fixtures.len();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for fixture in &fixtures {
        let Some(record) = crate::rules_core::pilot_compute::class_feature_grant_consumer::class_feature_record_tokens()
            .get(&fixture.record_key)
        else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "{:?} does not resolve against the live data/corpus/{}/class_feature ingest",
                    fixture.record_key, fixture.book
                ),
            );
            continue;
        };
        if record.class != fixture.class {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "fixture states class={:?} but the live ingest states class={:?}",
                    fixture.class, record.class
                ),
            );
            continue;
        }
        let fixture_ability_modifiers = ability_modifiers_from_fixture_inputs(&fixture.ability_modifier_inputs);
        let mut mismatch: Option<String> = None;
        'outer: for (arg_name, by_level) in &fixture.expected_value_at_level_by_arg {
            for (&level, &expected) in by_level {
                let resolved = crate::rules_core::pilot_compute::class_feature_grant_consumer::resolve_pcgen_var_chain(
                    &record.bonus_vars,
                    &fixture.class_level_var,
                    level,
                    &fixture_ability_modifiers,
                );
                match resolved.get(arg_name) {
                    Some(&got) if got == expected => {}
                    Some(&got) => {
                        mismatch = Some(format!(
                            "{:?} at level {level}: fixture (from the pinned upstream .lst, \
                             independently evaluated) expects {arg_name}={expected}, the real \
                             production resolver produced {got}",
                            fixture.corpus_field
                        ));
                        break 'outer;
                    }
                    None => {
                        mismatch = Some(format!(
                            "{:?} at level {level}: the real production resolver could not \
                             resolve {arg_name} at all (fixture expects {expected})",
                            fixture.corpus_field
                        ));
                        break 'outer;
                    }
                }
            }
        }
        match mismatch {
            Some(message) => {
                failures.insert(fixture.unit_id.clone(), message);
            }
            None if fixture.expected_value_at_level_by_arg.is_empty() => {
                // A fixture that pins no expected value asserts nothing about the resolver.
                // Refused rather than counted (Decision 1(a)).
                failures.insert(
                    fixture.unit_id.clone(),
                    format!("fixture for {:?} pins no expected value at all", fixture.corpus_field),
                );
            }
            None => {
                cleared.insert(fixture.unit_id.clone());
            }
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

/// One `kind=companion` fixture row. A sibling top-level `companion_entries`
/// array in the same committed fixture JSON, for the reason
/// [`MonsterFixture`] states about `monster_entries`: a forced-generic union
/// with equipment's `expected.abilities`/`expected.bonus` would mean nothing
/// here.
#[derive(Debug, Clone)]
pub struct CompanionFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    /// The `WEAPONPROF=` selector the token names — which is NOT guaranteed to
    /// be one of the record's natural attacks (`companion_chassis::
    /// NaturalAttackDamageBonus`'s own Parrot finding).
    pub attack: String,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    pub corpus_field: String,
    pub expected_shape: String,
    /// `(strength_modifier, damage_bonus)` pairs, computed by
    /// `scripts/derive_companion_strength_damage_fixtures.py` from PF1's
    /// halve-and-round-down convention — never read back from this repo's
    /// evaluator.
    pub expected_at: Vec<(i32, i32)>,
}

/// Reads the `companion_entries` array of the same committed fixture file
/// [`load_fixtures`] reads `entries` from.
pub fn load_companion_fixtures(repo_root: &Path) -> Vec<CompanionFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("companion_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let expected = &e["expected"];
            CompanionFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                attack: e["attack"].as_str().expect("attack").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                corpus_field: e["corpus_field"].as_str().expect("corpus_field").to_string(),
                expected_shape: expected["shape"].as_str().expect("expected.shape").to_string(),
                expected_at: expected["damage_bonus_at_strength_modifier"]
                    .as_array()
                    .expect("expected.damage_bonus_at_strength_modifier")
                    .iter()
                    .map(|p| {
                        (
                            i32::try_from(p["strength_modifier"].as_i64().expect("strength_modifier"))
                                .expect("a Strength modifier fits in i32"),
                            i32::try_from(p["damage_bonus"].as_i64().expect("damage_bonus"))
                                .expect("a damage bonus fits in i32"),
                        )
                    })
                    .collect(),
            }
        })
        .collect()
}

/// The `kind=companion` half of [`run_bar_check`].
///
/// Runs against the SHIPPED tables (`companion_chassis::COMPANION_BOOKS`) —
/// the same records `companion_catalog` serves and the reach gate judges —
/// rather than against `data/corpus/`, so a transcription that dropped the
/// token fails here rather than passing on a corpus file no player reads.
/// Same choice `run_monster_bar_check` makes, and for the same reason.
fn run_companion_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_companion_fixtures(repo_root);
    let fixtures_total = fixtures.len();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let mut engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    for fixture in &fixtures {
        // Joined on the INVENTORY's book id straight through, deliberately
        // without `monster_registry_book`'s `bestiary` -> `beastiary` spelling
        // map: no committed companion fixture names `bestiary` (its four
        // candidate units were all skipped by the derivation -- three carry no
        // damage token and one states the refused `max(0,STR)` shape), and
        // adding an untested mapping for a population that does not exist
        // would be a guess. If a later round DOES pin a `bestiary` companion,
        // this lands in `engine_does_not_hold` and
        // `run_companion_bar_check_clears_every_committed_companion_fixture`
        // fails on its `engine_does_not_hold.is_empty()` assertion -- loudly, which is
        // the right failure mode for a spelling this seam has never exercised.
        let Some(book) = companion_book(&fixture.book) else {
            engine_does_not_hold.insert(fixture.unit_id.clone(), fixture.book.clone());
            continue;
        };
        // Resolved by `.key` (the corpus `KEY:` identity), never by `.name` --
        // `Familiar (Fox)`'s key and name differ from the bare species in
        // several books, the same false-negative `run_monster_bar_check`
        // records for Gremlin (Grimple).
        let Some(record) = book.companion_resolve(&fixture.record_key) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "{:?} does not resolve against {}'s registered companions",
                    fixture.record_key, fixture.book
                ),
            );
            continue;
        };
        let Some(bonus) =
            record.natural_attack_damage_bonuses.iter().find(|b| b.attack == fixture.attack)
        else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row states {} but the shipped record carries no \
                     BONUS:WEAPONPROF={}|DAMAGE| token at all",
                    fixture.corpus_field, fixture.attack
                ),
            );
            continue;
        };
        let Some(parsed) = parse_companion_strength_damage(bonus.formula) else {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row states {} but the evaluator could not parse a Strength-damage \
                     shape from {:?}",
                    fixture.corpus_field, bonus.formula
                ),
            );
            continue;
        };
        if parsed.shape_name() != fixture.expected_shape {
            failures.insert(
                fixture.unit_id.clone(),
                format!(
                    "corpus row {:?} states shape {:?}, evaluator produced {:?}",
                    fixture.corpus_field,
                    fixture.expected_shape,
                    parsed.shape_name()
                ),
            );
            continue;
        }
        let mut mismatch = None;
        for &(strength_modifier, expected_bonus) in &fixture.expected_at {
            let got = evaluate_companion_strength_damage(parsed, strength_modifier);
            if got != expected_bonus {
                mismatch = Some(format!(
                    "corpus row {:?} at Strength modifier {strength_modifier}: expected damage \
                     bonus {expected_bonus}, evaluator produced {got}",
                    fixture.corpus_field
                ));
                break;
            }
        }
        match mismatch {
            Some(message) => {
                failures.insert(fixture.unit_id.clone(), message);
            }
            None if fixture.expected_at.is_empty() => {
                // A fixture that pins no evaluated value asserts nothing about
                // the evaluator. Refused rather than counted -- a gate that
                // cannot fail is worse than no gate (`decisions.md` Decision
                // 1(a)).
                failures.insert(
                    fixture.unit_id.clone(),
                    format!(
                        "fixture for {:?} pins no (strength_modifier, damage_bonus) pair at all, \
                         so it asserts nothing",
                        fixture.corpus_field
                    ),
                );
            }
            None => {
                cleared.insert(fixture.unit_id.clone());
            }
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

#[cfg(test)]
mod companion_seam_tests {
    use super::*;

    // --- parser unit tests: one TDD red/green anchor per corpus-observed shape ---

    #[test]
    fn the_parenthesised_half_strength_clamp_parses() {
        // Arctic Fox, `BONUS:WEAPONPROF=Bite|DAMAGE|max(0,(STR/2))`
        // (`ultimate_wilderness/uw_races_companion.lst:135`) -- 114 of the 117
        // pinned rows carry exactly this spelling.
        assert_eq!(
            parse_companion_strength_damage("max(0,(STR/2))"),
            Some(CompanionStrengthDamage::HalfStrengthNeverNegative)
        );
    }

    #[test]
    fn the_paren_free_half_strength_clamp_is_the_same_shape() {
        // Familiar (Cassisian), `BONUS:WEAPONPROF=Slam|DAMAGE|max(0,STR/2)`
        // (`bestiary_2/b2_races_familiar.lst:11`). PCGen spells the same thing
        // two ways and both ship verbatim on the record; only the PARSE
        // normalises them.
        assert_eq!(
            parse_companion_strength_damage("max(0,STR/2)"),
            Some(CompanionStrengthDamage::HalfStrengthNeverNegative)
        );
    }

    #[test]
    fn a_bare_and_a_negated_strength_term_each_parse() {
        assert_eq!(
            parse_companion_strength_damage("STR"),
            Some(CompanionStrengthDamage::FullStrength)
        );
        assert_eq!(
            parse_companion_strength_damage("-STR"),
            Some(CompanionStrengthDamage::NegatedFullStrength)
        );
    }

    #[test]
    fn an_integer_literal_parses_as_flat() {
        assert_eq!(parse_companion_strength_damage("5"), Some(CompanionStrengthDamage::Flat(5)));
        assert_eq!(parse_companion_strength_damage("-5"), Some(CompanionStrengthDamage::Flat(-5)));
    }

    // TDD red/green anchors: shapes this seam deliberately refuses rather than
    // guesses at, because an unclamped halving's value at a negative odd
    // Strength modifier depends on PCGen's floor-vs-truncate behaviour, which
    // this repo has no proof of.
    #[test]
    fn an_unclamped_halving_refuses_rather_than_guesses() {
        // `bestiary_4:companion:companion_dinosaur_diplodocus_tail_lash`.
        assert_eq!(parse_companion_strength_damage("STR/2"), None);
        assert_eq!(parse_companion_strength_damage("-(STR/2)"), None);
    }

    #[test]
    fn a_clamped_full_strength_term_refuses_rather_than_guesses() {
        // `bestiary:companion:tyrannosaurus_powerful_bite`, `max(0,STR)` -- a
        // real corpus shape, and NOT the same rule as `max(0,(STR/2))`.
        assert_eq!(parse_companion_strength_damage("max(0,STR)"), None);
    }

    // --- the rules arithmetic itself ---

    #[test]
    fn half_strength_rounds_down_and_never_goes_below_zero() {
        // PF1 CRB p.9 ("whenever you are asked to halve a number, round down")
        // and CRB p.182 (the 1-1/2x rule is stated about a Strength BONUS, so a
        // PENALTY is never multiplied -- which is what `max(0,...)` encodes).
        let d = CompanionStrengthDamage::HalfStrengthNeverNegative;
        assert_eq!(evaluate_companion_strength_damage(d, -4), 0);
        assert_eq!(evaluate_companion_strength_damage(d, -3), 0);
        assert_eq!(evaluate_companion_strength_damage(d, -1), 0);
        assert_eq!(evaluate_companion_strength_damage(d, 0), 0);
        assert_eq!(evaluate_companion_strength_damage(d, 1), 0);
        assert_eq!(evaluate_companion_strength_damage(d, 2), 1);
        assert_eq!(evaluate_companion_strength_damage(d, 3), 1);
        assert_eq!(evaluate_companion_strength_damage(d, 7), 3);
    }

    #[test]
    fn the_non_halving_shapes_evaluate_verbatim() {
        assert_eq!(
            evaluate_companion_strength_damage(CompanionStrengthDamage::FullStrength, 5),
            5
        );
        assert_eq!(
            evaluate_companion_strength_damage(CompanionStrengthDamage::NegatedFullStrength, 5),
            -5
        );
        assert_eq!(evaluate_companion_strength_damage(CompanionStrengthDamage::Flat(-5), 99), -5);
    }

    #[test]
    fn the_rendered_text_names_the_rule_rather_than_a_number() {
        // The PRODUCTION half. A catalog browser has no character, so a number
        // here would be invented; the rule's own words are not.
        assert_eq!(
            format_companion_strength_damage(CompanionStrengthDamage::HalfStrengthNeverNegative),
            "+1/2 Str modifier (minimum +0)"
        );
        assert_eq!(
            format_companion_strength_damage(CompanionStrengthDamage::Flat(5)),
            "+5"
        );
        assert_eq!(
            format_companion_strength_damage(CompanionStrengthDamage::Flat(-5)),
            "-5"
        );
    }

    // --- the corpus facts this seam rests on, asserted rather than assumed ---

    /// The single-natural-attack rule is NOT a corpus invariant, and this test
    /// pins that finding so a later cycle cannot quietly "fix" the seam by
    /// inferring the token's presence from the attack count. Re-derived over
    /// every registered book's shipped companion records.
    #[test]
    fn upstream_does_not_state_the_single_attack_rule_uniformly() {
        use crate::rules_core::rules_tables::companion_chassis::COMPANION_BOOKS;
        let mut one_attack_with_token = 0usize;
        let mut one_attack_without_token = 0usize;
        let mut multi_attack_with_token = 0usize;
        for book in COMPANION_BOOKS {
            for c in book.companions {
                let half = c
                    .natural_attack_damage_bonuses
                    .iter()
                    .filter(|b| {
                        parse_companion_strength_damage(b.formula)
                            == Some(CompanionStrengthDamage::HalfStrengthNeverNegative)
                    })
                    .count();
                match (c.natural_attacks.len(), half) {
                    (1, 1) => one_attack_with_token += 1,
                    (1, 0) => one_attack_without_token += 1,
                    (n, 1) if n >= 2 => multi_attack_with_token += 1,
                    _ => {}
                }
            }
        }
        assert!(
            one_attack_with_token > 0 && one_attack_without_token > 0,
            "both sides of the non-invariance must be non-empty for this finding to mean \
             anything: with={one_attack_with_token} without={one_attack_without_token}"
        );
        assert!(
            multi_attack_with_token > 0,
            "the rule is violated in BOTH directions upstream; if this reaches zero the corpus \
             changed and the seam's own doc comment must be re-derived"
        );
    }

    /// The Parrot finding, pinned: a `WEAPONPROF=` selector is NOT guaranteed
    /// to name one of the record's own natural attacks, so nothing in this seam
    /// may join the two and drop the misses.
    #[test]
    fn a_damage_bonus_selector_need_not_name_one_of_the_records_natural_attacks() {
        use crate::rules_core::rules_tables::companion_chassis::companion_book;
        let apg = companion_book("advanced_players_guide").expect("APG companions are registered");
        let parrot = apg.companion_resolve("Parrot").expect("APG ships a Parrot");
        assert!(
            parrot.natural_attack_damage_bonuses.iter().any(|b| b.attack == "Claw"),
            "Parrot's row states BONUS:WEAPONPROF=Claw|DAMAGE|…"
        );
        assert!(
            !parrot.natural_attacks.iter().any(|a| a.name == "Claw"),
            "…and Parrot has no Claw attack. If this ever becomes false the corpus changed and \
             `NaturalAttackDamageBonus`'s doc comment must be re-derived, not silently amended."
        );
    }

    // --- the bar check itself ---

    #[test]
    fn run_companion_bar_check_clears_every_committed_companion_fixture() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let report = run_companion_bar_check(&repo_root);
        assert!(
            report.fixtures_total > 0,
            "the committed fixture must carry at least one companion_entries row"
        );
        assert!(
            report.engine_does_not_hold.is_empty(),
            "every committed companion fixture's book must be registered, got: {:?}",
            report.engine_does_not_hold
        );
        assert!(
            report.failures.is_empty(),
            "every committed companion fixture must clear the bar, got {} failures, first few: \
             {:?}",
            report.failures.len(),
            report.failures.iter().take(5).collect::<Vec<_>>()
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }

    /// A scratch `repo_root` carrying one fixture the caller controls, pointed
    /// at a REAL shipped record (`ultimate_wilderness`'s Arctic Fox, whose row
    /// states `BONUS:WEAPONPROF=Bite|DAMAGE|max(0,(STR/2))`), so a test drives
    /// the REAL `run_companion_bar_check` end to end without touching the
    /// committed fixture. Same pattern as `ScratchMonsterFixtureRoot`.
    struct ScratchCompanionRoot {
        root: PathBuf,
    }

    impl ScratchCompanionRoot {
        fn new(name: &str, attack: &str, shape: &str, pairs: &[(i32, i32)]) -> Self {
            let root = std::env::temp_dir()
                .join(format!("codex_companion_mutation_proof_{name}_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            let at = pairs
                .iter()
                .map(|(m, b)| {
                    format!("{{\"strength_modifier\":{m},\"damage_bonus\":{b}}}")
                })
                .collect::<Vec<_>>()
                .join(",");
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(
                    r#"{{"companion_entries":[{{
                        "unit_id":"scratch:companion:arctic_fox",
                        "book":"ultimate_wilderness",
                        "record_key":"Arctic Fox",
                        "attack":"{attack}",
                        "upstream_lst":"scratch.lst",
                        "upstream_lst_sha256":"0",
                        "upstream_line":1,
                        "corpus_field":"BONUS:WEAPONPROF=Bite|DAMAGE|max(0,(STR/2))",
                        "expected":{{"shape":"{shape}","damage_bonus_at_strength_modifier":[{at}]}}
                    }}]}}"#
                ),
            )
            .unwrap();
            ScratchCompanionRoot { root }
        }
    }

    impl Drop for ScratchCompanionRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    /// The positive control, first: a fixture stating the TRUE values for the
    /// real, resolved Arctic Fox must clear the bar -- otherwise every mutation
    /// test below would pass for the wrong reason.
    #[test]
    fn a_correct_companion_fixture_clears_run_companion_bar_check() {
        let scratch = ScratchCompanionRoot::new(
            "correct",
            "Bite",
            "half_strength_never_negative",
            &[(-3, 0), (0, 0), (3, 1), (6, 3)],
        );
        let report = run_companion_bar_check(&scratch.root);
        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:companion:arctic_fox"));
    }

    /// MUTATION PROOF 1 -- a wrong evaluated value. This is the assertion the
    /// whole seam rests on: had the evaluator dropped the `max(0,…)` clamp it
    /// would produce `-2` at a Strength modifier of `-3`, so a fixture claiming
    /// `-2` must be reported as a failure rather than cleared.
    #[test]
    fn a_wrong_expected_damage_bonus_makes_run_companion_bar_check_report_a_failure() {
        let scratch = ScratchCompanionRoot::new(
            "wrongvalue",
            "Bite",
            "half_strength_never_negative",
            &[(-3, -2)],
        );
        let report = run_companion_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:companion:arctic_fox"));
    }

    /// MUTATION PROOF 2 -- a wrong SHAPE. `full_strength` and
    /// `half_strength_never_negative` agree at a Strength modifier of 0, so a
    /// seam that compared only the numbers at a lazily-chosen ladder could pass
    /// on a mis-parsed shape.
    #[test]
    fn a_wrong_expected_shape_makes_run_companion_bar_check_report_a_failure() {
        let scratch =
            ScratchCompanionRoot::new("wrongshape", "Bite", "full_strength", &[(0, 0)]);
        let report = run_companion_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }

    /// MUTATION PROOF 3 -- a token the shipped record does not carry. This is
    /// the case that catches a transcription regression: if a later regen
    /// dropped `natural_attack_damage_bonuses`, every fixture would land here.
    #[test]
    fn an_absent_damage_token_makes_run_companion_bar_check_report_a_failure() {
        let scratch = ScratchCompanionRoot::new(
            "notoken",
            "Gore",
            "half_strength_never_negative",
            &[(6, 3)],
        );
        let report = run_companion_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }

    /// MUTATION PROOF 4 -- a fixture that asserts NOTHING. A row pinning an
    /// empty ladder would otherwise clear the bar vacuously, which is exactly
    /// the "gate that cannot fail" Decision 1(a) forbids.
    #[test]
    fn a_fixture_pinning_no_values_at_all_is_refused_rather_than_cleared() {
        let scratch =
            ScratchCompanionRoot::new("empty", "Bite", "half_strength_never_negative", &[]);
        let report = run_companion_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }
}

#[cfg(test)]
mod companion_skill_seam_tests {
    use super::*;

    // --- parser unit tests: one TDD red/green anchor per corpus-observed shape ---

    #[test]
    fn the_dex_minus_str_shape_parses() {
        // `BONUS:SKILL|Climb,Swim|DEX-STR` -- 136 of the 136 corpus-wide
        // occurrences carry exactly this spelling (re-derived 2026-08-19).
        assert_eq!(
            parse_companion_skill_ability_diff("DEX-STR"),
            Some(SkillAbilityDiffFormula { plus: "DEX", minus: "STR" })
        );
    }

    #[test]
    fn whitespace_around_the_operator_is_tolerated() {
        assert_eq!(
            parse_companion_skill_ability_diff(" DEX - STR "),
            Some(SkillAbilityDiffFormula { plus: "DEX", minus: "STR" })
        );
    }

    #[test]
    fn every_other_ability_pairing_also_parses() {
        // The parser accepts any two DISTINCT PF1 ability abbreviations, not
        // just the one pairing the corpus happens to state today -- the same
        // discipline `companion_chassis::SkillAbilityDiffBonus`'s doc comment
        // states about not hard-coding the one spelling seen.
        assert_eq!(
            parse_companion_skill_ability_diff("WIS-CHA"),
            Some(SkillAbilityDiffFormula { plus: "WIS", minus: "CHA" })
        );
    }

    // TDD red/green anchors: shapes this seam deliberately refuses rather than
    // guesses at.

    #[test]
    fn a_flat_racial_bonus_refuses_rather_than_guesses() {
        // `BONUS:SKILL|Perception|4|TYPE=Racial` -- a different, already-static
        // quantity `transcribe_companion_tables.py`'s
        // `parse_skill_ability_diff_bonuses` never transcribes into this field
        // at all, but the parser refuses it too, belt-and-braces.
        assert_eq!(parse_companion_skill_ability_diff("4|TYPE=Racial"), None);
    }

    #[test]
    fn an_unrecognised_ability_abbreviation_refuses() {
        assert_eq!(parse_companion_skill_ability_diff("FOO-STR"), None);
        assert_eq!(parse_companion_skill_ability_diff("DEX-BAR"), None);
    }

    #[test]
    fn the_same_ability_on_both_sides_refuses() {
        // A difference of an ability with itself is always zero and states no
        // real rule; refusing it rather than "correctly" computing zero keeps
        // this parser from silently accepting a corpus typo.
        assert_eq!(parse_companion_skill_ability_diff("STR-STR"), None);
    }

    #[test]
    fn no_operator_at_all_refuses() {
        assert_eq!(parse_companion_skill_ability_diff("DEX"), None);
        assert_eq!(parse_companion_skill_ability_diff(""), None);
    }

    // --- the rules arithmetic itself ---

    #[test]
    fn the_skill_bonus_is_the_plain_difference_of_the_two_modifiers() {
        let f = SkillAbilityDiffFormula { plus: "DEX", minus: "STR" };
        assert_eq!(evaluate_companion_skill_ability_diff(f, 3, 1), 2);
        assert_eq!(evaluate_companion_skill_ability_diff(f, 1, 3), -2);
        assert_eq!(evaluate_companion_skill_ability_diff(f, 0, 0), 0);
        assert_eq!(evaluate_companion_skill_ability_diff(f, -3, 5), -8);
        assert_eq!(evaluate_companion_skill_ability_diff(f, 5, -3), 8);
    }

    #[test]
    fn the_rendered_text_names_the_rule_rather_than_a_number() {
        // The PRODUCTION half. A catalog browser has no character, so a number
        // here would be invented; the rule's own words are not.
        assert_eq!(
            format_companion_skill_ability_diff(SkillAbilityDiffFormula {
                plus: "DEX",
                minus: "STR"
            }),
            "Dex modifier \u{2212} Str modifier"
        );
        assert_eq!(
            format_companion_skill_ability_diff(SkillAbilityDiffFormula {
                plus: "WIS",
                minus: "CHA"
            }),
            "Wis modifier \u{2212} Cha modifier"
        );
    }

    // --- the corpus fact this seam rests on, asserted rather than assumed ---

    /// Pins the zero-variance finding the module doc and the fixture
    /// generator's doc comment both state, so a later cycle cannot quietly
    /// "simplify" the parser to a hard-coded `"DEX-STR"` string match without
    /// this test forcing a re-derivation first.
    #[test]
    fn every_registered_skill_ability_diff_bonus_states_the_same_formula_and_skills() {
        use crate::rules_core::rules_tables::companion_chassis::COMPANION_BOOKS;
        let mut total = 0usize;
        for book in COMPANION_BOOKS {
            for c in book.companions {
                for b in c.skill_ability_diff_bonuses {
                    total += 1;
                    assert_eq!(
                        b.formula, "DEX-STR",
                        "{}:{:?} states a skill-ability-diff formula other than DEX-STR -- the \
                         module doc's zero-variance claim needs re-deriving",
                        book.corpus_book, c.key
                    );
                    assert_eq!(b.skills, ["Climb", "Swim"]);
                }
            }
        }
        assert!(
            total > 0,
            "no registered companion carries a skill-ability-diff bonus at all; this test would \
             then be asserting nothing"
        );
    }

    // --- the bar check itself ---

    #[test]
    fn run_companion_skill_bar_check_clears_every_committed_companion_skill_fixture() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let report = run_companion_skill_bar_check(&repo_root);
        assert!(
            report.fixtures_total > 0,
            "the committed fixture must carry at least one companion_skill_entries row"
        );
        assert!(
            report.engine_does_not_hold.is_empty(),
            "every committed companion_skill fixture's book must be registered, got: {:?}",
            report.engine_does_not_hold
        );
        assert!(
            report.failures.is_empty(),
            "every committed companion_skill fixture must clear the bar, got {} failures, first \
             few: {:?}",
            report.failures.len(),
            report.failures.iter().take(5).collect::<Vec<_>>()
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }

    /// A scratch `repo_root` carrying one fixture the caller controls, pointed
    /// at a REAL shipped record (`ultimate_wilderness`'s Arctic Fox, whose row
    /// states `BONUS:SKILL|Climb,Swim|DEX-STR` -- the same record the sibling
    /// Strength-damage seam's own scratch tests use), so a test drives the
    /// REAL `run_companion_skill_bar_check` end to end without touching the
    /// committed fixture.
    struct ScratchCompanionSkillRoot {
        root: PathBuf,
    }

    impl ScratchCompanionSkillRoot {
        fn new(name: &str, skills: &[&str], triples: &[(i32, i32, i32)]) -> Self {
            let root = std::env::temp_dir().join(format!(
                "codex_companion_skill_mutation_proof_{name}_{}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&root);
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            let skills_json =
                skills.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(",");
            let at = triples
                .iter()
                .map(|(p, m, b)| {
                    format!(
                        "{{\"plus_modifier\":{p},\"minus_modifier\":{m},\"skill_bonus\":{b}}}"
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(
                    r#"{{"companion_skill_entries":[{{
                        "unit_id":"scratch:companion:arctic_fox_skill",
                        "book":"ultimate_wilderness",
                        "record_key":"Arctic Fox",
                        "skills":[{skills_json}],
                        "upstream_lst":"scratch.lst",
                        "upstream_lst_sha256":"0",
                        "upstream_line":1,
                        "corpus_field":"BONUS:SKILL|Climb,Swim|DEX-STR",
                        "expected":{{"plus_ability":"DEX","minus_ability":"STR","skill_bonus_at_modifiers":[{at}]}}
                    }}]}}"#
                ),
            )
            .unwrap();
            ScratchCompanionSkillRoot { root }
        }
    }

    impl Drop for ScratchCompanionSkillRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    /// The positive control, first: a fixture stating the TRUE values for the
    /// real, resolved Arctic Fox must clear the bar.
    #[test]
    fn a_correct_companion_skill_fixture_clears_run_companion_skill_bar_check() {
        let scratch = ScratchCompanionSkillRoot::new(
            "correct",
            &["Climb", "Swim"],
            &[(3, 1, 2), (0, 0, 0), (-3, 5, -8)],
        );
        let report = run_companion_skill_bar_check(&scratch.root);
        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:companion:arctic_fox_skill"));
    }

    /// MUTATION PROOF 1 -- a wrong evaluated value. Had the evaluator SWAPPED
    /// the two operands (computed `minus - plus` instead of `plus - minus`),
    /// it would produce `-2` rather than the true `2` at (plus=3, minus=1), so
    /// a fixture claiming `-2` must be reported as a failure.
    #[test]
    fn a_wrong_expected_skill_bonus_makes_run_companion_skill_bar_check_report_a_failure() {
        let scratch =
            ScratchCompanionSkillRoot::new("wrongvalue", &["Climb", "Swim"], &[(3, 1, -2)]);
        let report = run_companion_skill_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:companion:arctic_fox_skill"));
    }

    /// MUTATION PROOF 2 -- a skill list the shipped record does not carry.
    /// This is the case that catches a transcription regression: if a later
    /// regen dropped `skill_ability_diff_bonuses` or renamed a skill, every
    /// fixture naming it would land here.
    #[test]
    fn a_skill_list_the_record_does_not_carry_makes_run_companion_skill_bar_check_report_a_failure()
    {
        let scratch = ScratchCompanionSkillRoot::new("wrongskills", &["Climb"], &[(3, 1, 2)]);
        let report = run_companion_skill_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }

    /// MUTATION PROOF 3 -- a fixture that asserts NOTHING. A row pinning an
    /// empty ladder would otherwise clear the bar vacuously, which is exactly
    /// the "gate that cannot fail" Decision 1(a) forbids.
    #[test]
    fn a_fixture_pinning_no_values_at_all_is_refused_rather_than_cleared() {
        let scratch = ScratchCompanionSkillRoot::new("empty", &["Climb", "Swim"], &[]);
        let report = run_companion_skill_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }
}

#[cfg(test)]
mod companion_save_dc_seam_tests {
    use super::*;

    // --- parser unit tests: one TDD red/green anchor per corpus-observed shape ---

    #[test]
    fn the_base_plus_half_hd_plus_ability_shape_parses() {
        // `Companion (Dinosaur (Dimorphodon)) ~ Poison`
        // (`b4_abilities_companion.lst:19`) -- the most common corpus-wide
        // spelling, 22 of 25 records.
        assert_eq!(
            parse_companion_save_dc_formula("10+HD/2+CON"),
            Some(CompanionSaveDcFormula { base: 10, includes_half_hd: true, ability: "CON" })
        );
    }

    #[test]
    fn a_different_base_constant_and_ability_also_parse() {
        // The base is NOT always 10 (unlike `monster_ability`'s Universal
        // Monster Rule shape) -- `Flowering Lattice ~ Pollen` states 12.
        assert_eq!(
            parse_companion_save_dc_formula("12+HD/2+CON"),
            Some(CompanionSaveDcFormula { base: 12, includes_half_hd: true, ability: "CON" })
        );
        assert_eq!(
            parse_companion_save_dc_formula("12+HD/2+DEX"),
            Some(CompanionSaveDcFormula { base: 12, includes_half_hd: true, ability: "DEX" })
        );
    }

    #[test]
    fn the_flat_shape_with_no_half_hd_term_parses() {
        // `Isitoq ~ Daze` (`b4_abilities_companion.lst:88`) states no HD term
        // at all -- a flat base plus ability modifier.
        assert_eq!(
            parse_companion_save_dc_formula("11+CHA"),
            Some(CompanionSaveDcFormula { base: 11, includes_half_hd: false, ability: "CHA" })
        );
        assert_eq!(
            parse_companion_save_dc_formula("10+CON"),
            Some(CompanionSaveDcFormula { base: 10, includes_half_hd: false, ability: "CON" })
        );
    }

    #[test]
    fn whitespace_is_tolerated() {
        assert_eq!(
            parse_companion_save_dc_formula(" 10 + HD / 2 + CON "),
            Some(CompanionSaveDcFormula { base: 10, includes_half_hd: true, ability: "CON" })
        );
    }

    #[test]
    fn the_tl_divisor_variable_also_parses() {
        // No live `kind=companion` corpus row states `TL` today (only
        // `monster_ability`'s `puffball_poison` BONUS:VAR sibling does, a
        // different token shape this seam does not read) -- accepted anyway
        // for the same forward-compatibility reason
        // `parse_formula_base_plus_ability` accepts it.
        assert_eq!(
            parse_companion_save_dc_formula("10+TL/2+WIS"),
            Some(CompanionSaveDcFormula { base: 10, includes_half_hd: true, ability: "WIS" })
        );
    }

    // TDD red/green anchors: shapes this seam deliberately refuses rather than
    // guesses at.

    #[test]
    fn a_bare_variable_name_refuses() {
        // The `monster_ability` named-variable shape (`ClingDC`) -- needs a
        // chassis field this seam does not add.
        assert_eq!(parse_companion_save_dc_formula("DiseaseDC"), None);
        assert_eq!(parse_companion_save_dc_formula("PuffballPoisonDC"), None);
    }

    #[test]
    fn a_half_hd_term_with_no_ability_at_all_refuses() {
        // `Whiptail Centipede (Giant) ~ Wall Climber`'s OTHER DESC argument
        // (`1+HD/2`) -- a duration, not a save DC. Must not be misread as
        // `base=1, ability=<garbage>`.
        assert_eq!(parse_companion_save_dc_formula("1+HD/2"), None);
    }

    #[test]
    fn a_multiplication_refuses() {
        assert_eq!(parse_companion_save_dc_formula("4*CONSCORE"), None);
        assert_eq!(parse_companion_save_dc_formula("CONSCORE*6"), None);
    }

    #[test]
    fn an_ability_score_spelling_refuses() {
        // `CONSCORE` is the SCORE, not the modifier; only the bare
        // abbreviation is accepted, same discipline
        // `PF_ABILITY_ABBREVS`'s doc comment states for the sibling seam.
        assert_eq!(parse_companion_save_dc_formula("10+CONSCORE"), None);
    }

    #[test]
    fn an_unrecognised_ability_abbreviation_refuses() {
        assert_eq!(parse_companion_save_dc_formula("10+FOO"), None);
    }

    #[test]
    fn no_operator_at_all_refuses() {
        assert_eq!(parse_companion_save_dc_formula("HD"), None);
        assert_eq!(parse_companion_save_dc_formula(""), None);
    }

    // --- the rules arithmetic itself ---

    #[test]
    fn the_half_hd_shape_floors_hit_dice_and_adds_the_ability_modifier() {
        let f = CompanionSaveDcFormula { base: 10, includes_half_hd: true, ability: "CON" };
        assert_eq!(evaluate_companion_save_dc_formula(f, 0, 0), 10);
        assert_eq!(evaluate_companion_save_dc_formula(f, 1, 3), 13); // floor(1/2)=0
        assert_eq!(evaluate_companion_save_dc_formula(f, 2, -2), 9); // floor(2/2)=1
        assert_eq!(evaluate_companion_save_dc_formula(f, 5, 4), 16); // floor(5/2)=2
        assert_eq!(evaluate_companion_save_dc_formula(f, 20, 2), 22); // floor(20/2)=10
    }

    #[test]
    fn the_flat_shape_ignores_hit_dice_entirely() {
        let f = CompanionSaveDcFormula { base: 11, includes_half_hd: false, ability: "CHA" };
        // Same ability_modifier, wildly different hit_dice -- must produce the
        // IDENTICAL DC, proving the evaluator does not apply a half-HD term
        // this shape never stated.
        assert_eq!(evaluate_companion_save_dc_formula(f, 0, 3), 14);
        assert_eq!(evaluate_companion_save_dc_formula(f, 1, 3), 14);
        assert_eq!(evaluate_companion_save_dc_formula(f, 20, 3), 14);
    }

    #[test]
    fn the_rendered_text_names_the_rule_rather_than_a_number() {
        assert_eq!(
            format_companion_save_dc_formula(CompanionSaveDcFormula {
                base: 10,
                includes_half_hd: true,
                ability: "CON"
            }),
            "10 + 1/2 HD + Con modifier"
        );
        assert_eq!(
            format_companion_save_dc_formula(CompanionSaveDcFormula {
                base: 11,
                includes_half_hd: false,
                ability: "CHA"
            }),
            "11 + Cha modifier"
        );
    }

    // --- the corpus fact this seam rests on, asserted rather than assumed ---

    /// Pins the shape census the module doc and the fixture generator's doc
    /// comment both state (25 records, two books), so a later cycle cannot
    /// quietly narrow the parser without this test forcing a re-derivation.
    #[test]
    fn every_committed_companion_save_dc_fixture_resolves_to_exactly_one_parseable_shape() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixtures = load_companion_save_dc_fixtures(&repo_root);
        assert_eq!(fixtures.len(), 25, "committed companion_save_dc_entries count moved");
        for f in &fixtures {
            let book = companion_book(&f.book)
                .unwrap_or_else(|| panic!("{}: book {:?} not registered", f.unit_id, f.book));
            let record = book.companion_ability_resolve(&f.record_key).unwrap_or_else(|| {
                panic!("{}: {:?} does not resolve", f.unit_id, f.record_key)
            });
            let mut candidates: Vec<&'static str> = record.description_variables.to_vec();
            for variant in record.description_variants {
                candidates.extend(variant.variables.iter().copied());
            }
            let shapes: BTreeSet<_> =
                candidates.iter().filter_map(|c| parse_companion_save_dc_formula(c)).collect();
            assert_eq!(
                shapes.len(),
                1,
                "{}: expected exactly one parseable save-DC shape, got {shapes:?} from \
                 candidates {candidates:?}",
                f.unit_id
            );
        }
    }

    // --- the bar check itself ---

    #[test]
    fn run_companion_save_dc_bar_check_clears_every_committed_companion_save_dc_fixture() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let report = run_companion_save_dc_bar_check(&repo_root);
        assert!(
            report.fixtures_total > 0,
            "the committed fixture must carry at least one companion_save_dc_entries row"
        );
        assert!(
            report.engine_does_not_hold.is_empty(),
            "every committed companion_save_dc fixture's book must be registered, got: {:?}",
            report.engine_does_not_hold
        );
        assert!(
            report.failures.is_empty(),
            "every committed companion_save_dc fixture must clear the bar, got {} failures, \
             first few: {:?}",
            report.failures.len(),
            report.failures.iter().take(5).collect::<Vec<_>>()
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }

    /// A scratch `repo_root` carrying one fixture the caller controls, pointed
    /// at a REAL shipped record (`bestiary_4`'s `Companion (Dinosaur
    /// (Dimorphodon)) ~ Poison`, whose row states `DESC:...|10+HD/2+CON`), so
    /// a test drives the REAL `run_companion_save_dc_bar_check` end to end
    /// without touching the committed fixture.
    struct ScratchCompanionSaveDcRoot {
        root: PathBuf,
    }

    impl ScratchCompanionSaveDcRoot {
        fn new(name: &str, base: i32, includes_half_hd: bool, ability: &str, at: &[(i32, i32, i32)]) -> Self {
            let root = std::env::temp_dir().join(format!(
                "codex_companion_save_dc_mutation_proof_{name}_{}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&root);
            let fixture_dir = root.join("tests/fixtures/rules_core");
            std::fs::create_dir_all(&fixture_dir).unwrap();
            let at_json = at
                .iter()
                .map(|(hd, am, dc)| {
                    format!("{{\"hit_dice\":{hd},\"ability_modifier\":{am},\"save_dc\":{dc}}}")
                })
                .collect::<Vec<_>>()
                .join(",");
            std::fs::write(
                fixture_dir.join("derived-evaluator-fixtures.json"),
                format!(
                    r#"{{"companion_save_dc_entries":[{{
                        "unit_id":"scratch:companion:dimorphodon_poison",
                        "book":"bestiary_4",
                        "record_key":"Companion (Dinosaur (Dimorphodon)) ~ Poison",
                        "upstream_lst":"scratch.lst",
                        "upstream_lst_sha256":"0",
                        "upstream_line":1,
                        "corpus_field":"DESC:...|10+HD/2+CON",
                        "expected":{{"base":{base},"includes_half_hd":{includes_half_hd},"ability":"{ability}","save_dc_at":[{at_json}]}}
                    }}]}}"#
                ),
            )
            .unwrap();
            ScratchCompanionSaveDcRoot { root }
        }
    }

    impl Drop for ScratchCompanionSaveDcRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    /// The positive control, first: a fixture stating the TRUE values for the
    /// real, resolved Dimorphodon Poison ability must clear the bar.
    #[test]
    fn a_correct_companion_save_dc_fixture_clears_run_companion_save_dc_bar_check() {
        let scratch = ScratchCompanionSaveDcRoot::new(
            "correct",
            10,
            true,
            "CON",
            &[(0, 0, 10), (5, 4, 16), (20, -1, 19)],
        );
        let report = run_companion_save_dc_bar_check(&scratch.root);
        assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
        assert_eq!(report.cleared.len(), 1);
        assert!(report.cleared.contains("scratch:companion:dimorphodon_poison"));
    }

    /// MUTATION PROOF 1 -- a wrong evaluated value. Had the evaluator dropped
    /// the half-HD term entirely, it would produce `14` rather than the true
    /// `16` at (hd=5, ability_modifier=4), so a fixture claiming `99` must be
    /// reported as a failure.
    #[test]
    fn a_wrong_expected_save_dc_makes_run_companion_save_dc_bar_check_report_a_failure() {
        let scratch =
            ScratchCompanionSaveDcRoot::new("wrongvalue", 10, true, "CON", &[(5, 4, 99)]);
        let report = run_companion_save_dc_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
        assert!(report.failures.contains_key("scratch:companion:dimorphodon_poison"));
    }

    /// MUTATION PROOF 2 -- a wrong expected ABILITY. Asserts IDENTITY, not
    /// just that some shape parsed (wave-16 review lesson): the shipped
    /// record states CON, not WIS, so a fixture claiming WIS must fail even
    /// though the (base, includes_half_hd) halves still match.
    #[test]
    fn a_wrong_expected_ability_makes_run_companion_save_dc_bar_check_report_a_failure() {
        let scratch =
            ScratchCompanionSaveDcRoot::new("wrongability", 10, true, "WIS", &[(5, 4, 16)]);
        let report = run_companion_save_dc_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }

    /// MUTATION PROOF 3 -- a wrong expected BASE constant.
    #[test]
    fn a_wrong_expected_base_makes_run_companion_save_dc_bar_check_report_a_failure() {
        let scratch = ScratchCompanionSaveDcRoot::new("wrongbase", 12, true, "CON", &[(5, 4, 16)]);
        let report = run_companion_save_dc_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }

    /// MUTATION PROOF 4 -- a wrong `includes_half_hd`. The shipped record DOES
    /// carry the half-HD term; a fixture claiming it does not must fail.
    #[test]
    fn a_wrong_expected_includes_half_hd_makes_run_companion_save_dc_bar_check_report_a_failure() {
        let scratch = ScratchCompanionSaveDcRoot::new("wronghalfhd", 10, false, "CON", &[(5, 4, 14)]);
        let report = run_companion_save_dc_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }

    /// MUTATION PROOF 5 -- a fixture that asserts NOTHING. A row pinning an
    /// empty ladder would otherwise clear the bar vacuously (Decision 1(a)).
    #[test]
    fn a_fixture_pinning_no_values_at_all_is_refused_rather_than_cleared() {
        let scratch = ScratchCompanionSaveDcRoot::new("empty", 10, true, "CON", &[]);
        let report = run_companion_save_dc_bar_check(&scratch.root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }

    /// MUTATION PROOF 6 -- a `record_key` the shipped book does not carry.
    #[test]
    fn an_unresolvable_record_key_makes_run_companion_save_dc_bar_check_report_a_failure() {
        let root = std::env::temp_dir().join(format!(
            "codex_companion_save_dc_mutation_proof_noresolve_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        let fixture_dir = root.join("tests/fixtures/rules_core");
        std::fs::create_dir_all(&fixture_dir).unwrap();
        std::fs::write(
            fixture_dir.join("derived-evaluator-fixtures.json"),
            r#"{"companion_save_dc_entries":[{
                "unit_id":"scratch:companion:no_such_ability",
                "book":"bestiary_4",
                "record_key":"No Such Ability At All",
                "upstream_lst":"scratch.lst",
                "upstream_lst_sha256":"0",
                "upstream_line":1,
                "corpus_field":"DESC:...|10+HD/2+CON",
                "expected":{"base":10,"includes_half_hd":true,"ability":"CON","save_dc_at":[{"hit_dice":5,"ability_modifier":4,"save_dc":16}]}
            }]}"#,
        )
        .unwrap();
        let report = run_companion_save_dc_bar_check(&root);
        let _ = std::fs::remove_dir_all(&root);
        assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
        assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    }
}


// -------------------------------------------------------------------------------------------
// Folded into SD-33 from `worktree-wf_be4660f2-72a-3` (2026-08-26) per
// `docs/release/SD-31-corpus-closure-grind/artifacts/OPEN-ISSUES.md` row 365's remediation
// path (a). The seam + fixtures below are unchanged from the branch (reviewer-confirmed sound);
// the branch's race-level `FORMULA_RACE_TRAIT_RACES` doneness-credit const was NOT folded — see
// `src/rules_core/pilot_compute/mod.rs`'s own fold-note next to `explain_undine_formula_race_trait`.
// -------------------------------------------------------------------------------------------

// ---------------------------------------------------------------------------------------------
// `kind=race_trait`, FORMULA shape (SD31-W26-RACETRAIT-001).
// ---------------------------------------------------------------------------------------------
//
// The first consumer of `formula_interpreter::PcgenFormulaEvaluator` anywhere in this codebase
// (`grep -rn PcgenFormulaEvaluator src/` before this addition returns only the evaluator's own
// module) — wave 25b built and proved the interpreter but shipped no consumer of it, per
// `OPERATOR-RULINGS-2026-08-21.md` §20's own condition: "every interpreted value must clear
// `derived_evaluator_fixture_check` ... An interpreted value with no fixture is not done." This
// is that gate for `src/rules_core/pilot_compute/mod.rs`'s
// `explain_undine_formula_race_trait`/`UNDINE_RACE_TRAIT_FORMULAS`.
//
// Runs against the SHIPPED table (`UNDINE_RACE_TRAIT_FORMULAS`), exactly as
// `run_companion_skill_bar_check` runs against `record.skill_ability_diff_bonuses` and for the
// same reason: a transcription that corrupted the formula text in `pilot_compute::mod.rs` must
// fail HERE, not pass silently against a corpus file no player-facing code reads.

/// One `kind=race_trait` formula fixture row — a sibling top-level
/// `race_trait_formula_entries` array in the same committed fixture JSON.
#[derive(Debug, Clone)]
pub struct RaceTraitFormulaFixture {
    pub unit_id: String,
    pub book: String,
    pub record_key: String,
    pub upstream_lst: String,
    pub upstream_lst_sha256: String,
    pub upstream_line: u64,
    /// field name -> raw formula text, as the generator re-verified against
    /// the pinned oracle. Compared against `UNDINE_RACE_TRAIT_FORMULAS`
    /// below so a transcription regression in EITHER the shipped table or
    /// the fixture turns this check red, never just the arithmetic.
    pub formulas: BTreeMap<String, String>,
    /// `(TL, CON, CHA, {field: expected_value})` at each of the ten sample
    /// points `scripts/derive_race_trait_formula_fixtures.py` computed with
    /// its own from-scratch Python function per formula shape — never read
    /// back from this repo's evaluator.
    pub expected_at: Vec<(i64, i64, i64, BTreeMap<String, i64>)>,
}

/// Reads the `race_trait_formula_entries` array of the same committed
/// fixture file [`load_fixtures`] reads `entries` from.
pub fn load_race_trait_formula_fixtures(repo_root: &Path) -> Vec<RaceTraitFormulaFixture> {
    let path = repo_root.join(FIXTURE_RELATIVE_PATH);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the committed fixture must be readable at {path:?}: {e}"));
    let doc: serde_json::Value =
        serde_json::from_str(&text).expect("the committed fixture must be valid JSON");
    let Some(entries) = doc.get("race_trait_formula_entries").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .map(|e| {
            let formulas: BTreeMap<String, String> = e["formulas"]
                .as_object()
                .expect("formulas")
                .iter()
                .map(|(k, v)| (k.clone(), v.as_str().expect("formula value").to_string()))
                .collect();
            let expected_at = e["expected_at_sample_points"]
                .as_array()
                .expect("expected_at_sample_points")
                .iter()
                .map(|p| {
                    let expected: BTreeMap<String, i64> = p["expected"]
                        .as_object()
                        .expect("expected")
                        .iter()
                        .map(|(k, v)| (k.clone(), v.as_i64().expect("expected value fits in i64")))
                        .collect();
                    (
                        p["TL"].as_i64().expect("TL"),
                        p["CON"].as_i64().expect("CON"),
                        p["CHA"].as_i64().expect("CHA"),
                        expected,
                    )
                })
                .collect();
            RaceTraitFormulaFixture {
                unit_id: e["unit_id"].as_str().expect("unit_id").to_string(),
                book: e["book"].as_str().expect("book").to_string(),
                record_key: e["record_key"].as_str().expect("record_key").to_string(),
                upstream_lst: e["upstream_lst"].as_str().expect("upstream_lst").to_string(),
                upstream_lst_sha256: e["upstream_lst_sha256"]
                    .as_str()
                    .expect("upstream_lst_sha256")
                    .to_string(),
                upstream_line: e["upstream_line"].as_u64().expect("upstream_line"),
                formulas,
                expected_at,
            }
        })
        .collect()
}

/// The `kind=race_trait` formula half of [`run_bar_check`].
fn run_race_trait_formula_bar_check(repo_root: &Path) -> BarCheckReport {
    let fixtures = load_race_trait_formula_fixtures(repo_root);
    let fixtures_total = fixtures.len();

    let mut cleared = BTreeSet::new();
    let mut failures: BTreeMap<String, String> = BTreeMap::new();
    let engine_does_not_hold: BTreeMap<String, String> = BTreeMap::new();

    let evaluator = PcgenFormulaEvaluator;

    for fixture in &fixtures {
        let mut mismatch: Option<String> = None;

        for (field, expected_formula) in &fixture.formulas {
            let Some((_, _, shipped_formula)) =
                UNDINE_RACE_TRAIT_FORMULAS.iter().find(|(_, f, _)| f == field)
            else {
                mismatch = Some(format!(
                    "fixture names field {field:?} but UNDINE_RACE_TRAIT_FORMULAS carries no \
                     entry for it at all"
                ));
                break;
            };
            // The independence check: confirm the SHIPPED table states the
            // SAME formula text the fixture (independently re-derived from
            // the oracle) expects, not merely SOME formula for this field --
            // the same posture `run_companion_skill_bar_check` takes for
            // `parsed.plus`/`parsed.minus` against `fixture.plus_ability`.
            if shipped_formula != expected_formula {
                mismatch = Some(format!(
                    "fixture expects {field}={expected_formula:?} but UNDINE_RACE_TRAIT_FORMULAS \
                     states {field}={shipped_formula:?}"
                ));
                break;
            }
        }
        if let Some(message) = mismatch {
            failures.insert(fixture.unit_id.clone(), message);
            continue;
        }

        if fixture.expected_at.is_empty() {
            // A fixture that pins no sample point asserts nothing about the
            // evaluator. Refused rather than counted -- a gate that cannot
            // fail is worse than no gate (`decisions.md` Decision 1(a)).
            failures.insert(
                fixture.unit_id.clone(),
                format!("fixture for {:?} pins no sample point at all, so it asserts nothing", fixture.record_key),
            );
            continue;
        }

        let mut all_matched = true;
        for (tl, con, cha, expected) in &fixture.expected_at {
            let mut vars: BTreeMap<String, i64> = BTreeMap::new();
            vars.insert("TL".to_owned(), *tl);
            vars.insert("CON".to_owned(), *con);
            vars.insert("CHA".to_owned(), *cha);

            for (field, expected_value) in expected {
                let Some((_, _, formula)) = UNDINE_RACE_TRAIT_FORMULAS.iter().find(|(_, f, _)| f == field)
                else {
                    // Already reported as a mismatch above; unreachable here.
                    all_matched = false;
                    continue;
                };
                match evaluator.evaluate(formula, &vars) {
                    Ok(actual) if actual == *expected_value => {}
                    Ok(actual) => {
                        failures.insert(
                            fixture.unit_id.clone(),
                            format!(
                                "at TL={tl} CON={con} CHA={cha}, {field} expected \
                                 {expected_value} but PcgenFormulaEvaluator produced {actual} \
                                 for formula {formula:?}"
                            ),
                        );
                        all_matched = false;
                    }
                    Err(e) => {
                        failures.insert(
                            fixture.unit_id.clone(),
                            format!(
                                "at TL={tl} CON={con} CHA={cha}, {field}'s formula {formula:?} \
                                 refused to evaluate: {e}"
                            ),
                        );
                        all_matched = false;
                    }
                }
                if !all_matched {
                    break;
                }
            }
            if !all_matched {
                break;
            }
        }
        if all_matched {
            cleared.insert(fixture.unit_id.clone());
        }
    }

    BarCheckReport { cleared, failures, engine_does_not_hold, fixtures_total }
}

#[cfg(test)]
mod race_trait_formula_bar_check_tests {
    use super::*;
    use crate::rules_core::pilot_compute::formula_reproduction_harness::FormulaEvalError;

    fn repo_root() -> std::path::PathBuf {
        std::path::PathBuf::from(
            std::env::var("CODEX_REPO_ROOT").unwrap_or_else(|_| ".".to_string()),
        )
    }

    /// The real gate, run against the real committed fixture and the real
    /// shipped `UNDINE_RACE_TRAIT_FORMULAS` table: every entry must clear.
    #[test]
    fn run_race_trait_formula_bar_check_clears_every_committed_fixture() {
        let report = run_race_trait_formula_bar_check(&repo_root());
        assert!(
            report.failures.is_empty(),
            "every committed race_trait_formula fixture must clear: {:?}",
            report.failures
        );
        assert!(report.engine_does_not_hold.is_empty());
        assert_eq!(report.fixtures_total, 3, "3 Undine alternate-trait records are fixture-pinned");
        assert_eq!(report.cleared.len(), 3);
    }

    /// Anti-gaming mutation proof (Decision 1(a)): a wrong-but-plausible
    /// evaluator must be caught. Mirrors `harness_detects_a_deliberately_
    /// wrong_evaluator` in `formula_reproduction_harness.rs` and every other
    /// bar check's own mutation test in this file -- a gate that cannot
    /// fail is worse than no gate.
    struct OffByOneEvaluator;
    impl FormulaEvaluator for OffByOneEvaluator {
        fn evaluate(
            &self,
            formula: &str,
            vars: &BTreeMap<String, i64>,
        ) -> Result<i64, FormulaEvalError> {
            PcgenFormulaEvaluator.evaluate(formula, vars).map(|v| v + 1)
        }
    }

    #[test]
    fn a_mutated_evaluator_is_caught_by_the_race_trait_formula_gate() {
        let fixtures = load_race_trait_formula_fixtures(&repo_root());
        assert!(!fixtures.is_empty(), "the committed fixture must carry at least one entry");
        let evaluator = OffByOneEvaluator;
        let mut any_mismatch = false;
        for fixture in &fixtures {
            for (tl, con, cha, expected) in &fixture.expected_at {
                let mut vars: BTreeMap<String, i64> = BTreeMap::new();
                vars.insert("TL".to_owned(), *tl);
                vars.insert("CON".to_owned(), *con);
                vars.insert("CHA".to_owned(), *cha);
                for (field, expected_value) in expected {
                    let (_, _, formula) =
                        UNDINE_RACE_TRAIT_FORMULAS.iter().find(|(_, f, _)| f == field).unwrap();
                    let actual = evaluator.evaluate(formula, &vars).unwrap();
                    if actual != *expected_value {
                        any_mismatch = true;
                    }
                }
            }
        }
        assert!(
            any_mismatch,
            "an evaluator that is off by one on every result must disagree with at least one \
             pinned expected value -- if this fails, the fixture itself cannot detect a wrong \
             evaluator"
        );
    }

    /// The shipped table and the committed fixture must state the IDENTICAL
    /// formula text for every field -- proves the independence check inside
    /// `run_race_trait_formula_bar_check` itself is reachable and correct,
    /// not merely present in the source.
    #[test]
    fn a_transcription_regression_in_the_shipped_table_is_caught() {
        let fixtures = load_race_trait_formula_fixtures(&repo_root());
        for fixture in &fixtures {
            for (field, expected_formula) in &fixture.formulas {
                let (_, _, shipped_formula) =
                    UNDINE_RACE_TRAIT_FORMULAS.iter().find(|(_, f, _)| f == field).unwrap_or_else(|| {
                        panic!("UNDINE_RACE_TRAIT_FORMULAS carries no entry for {field:?}")
                    });
                assert_eq!(
                    shipped_formula, expected_formula,
                    "shipped formula for {field} must match the independently-derived fixture"
                );
            }
        }
    }
}
