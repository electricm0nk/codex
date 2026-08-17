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
//! (`spell_like_ability_caster_level`, SD31-E6-F11-002) and `kind=spell`
//! (`parse_caster_level_linear_duration`, SD31-E6-F2-006) seams added
//! since restate the same four guarantees for their own evaluator/fixture
//! pair rather than reusing that module's equipment-specific test names.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
use crate::rules_core::corpus_loader::{BookCorpusRoot, load_equipment_corpus};
use crate::rules_core::equipment_effects::compute_equipment_effects;
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
    pub not_ingested: BTreeMap<String, String>,
    pub fixtures_total: usize,
}

/// Runs the `derived` bar over every fixture entry, exactly as
/// `tests/derived_evaluator_fixture_check.rs::engine_evaluator_output_equals_the_corpus_derived_expected_value`
/// does, factored out so both the test and `v06_work_inventory` call the
/// same code.
pub fn run_bar_check(repo_root: &Path) -> BarCheckReport {
    let equipment = run_equipment_bar_check(repo_root);
    let monster = run_monster_bar_check(repo_root);
    let spell = run_spell_bar_check(repo_root);
    let mut cleared = equipment.cleared;
    cleared.extend(monster.cleared);
    cleared.extend(spell.cleared);
    let mut failures = equipment.failures;
    failures.extend(monster.failures);
    failures.extend(spell.failures);
    let mut not_ingested = equipment.not_ingested;
    not_ingested.extend(monster.not_ingested);
    not_ingested.extend(spell.not_ingested);
    BarCheckReport {
        cleared,
        failures,
        not_ingested,
        fixtures_total: equipment.fixtures_total + monster.fixtures_total + spell.fixtures_total,
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
    let mut not_ingested: BTreeMap<String, String> = BTreeMap::new();

    for book in &books {
        let Some(dir) = ingested_equipment_dir(repo_root, book) else {
            for f in fixtures.iter().filter(|f| &f.book == book) {
                not_ingested.insert(f.unit_id.clone(), book.clone());
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

    BarCheckReport { cleared, failures, not_ingested, fixtures_total }
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
        // corpus over the generic rule. A plain integer parses directly; an
        // unparseable formula (e.g. `HD*3/4`, `book_of_the_damned_volume_2`'s
        // Demon (Vermlek)) has no computable value without a formula
        // interpreter this repo does not have, and returns `None` rather
        // than a guess -- the same "honest absence" contract this
        // function's other early returns already keep.
        Some(raw) => raw.trim().parse::<i32>().ok(),
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
    let mut not_ingested: BTreeMap<String, String> = BTreeMap::new();

    for fixture in &fixtures {
        let registry_book = monster_registry_book(&fixture.book);
        let Some(monster_book) = MONSTER_BOOKS.iter().find(|b| b.corpus_book == registry_book)
        else {
            not_ingested.insert(fixture.unit_id.clone(), fixture.book.clone());
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

    BarCheckReport { cleared, failures, not_ingested, fixtures_total }
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
        _ => None,
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
    let mut not_ingested: BTreeMap<String, String> = BTreeMap::new();

    for book in &books {
        let Some(dir) = spell_corpus_dir_exists(repo_root, book) else {
            for f in fixtures.iter().filter(|f| &f.book == book) {
                not_ingested.insert(f.unit_id.clone(), book.clone());
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

    BarCheckReport { cleared, failures, not_ingested, fixtures_total }
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
            report.not_ingested.is_empty(),
            "every committed spell fixture's book must be ingested, got: {:?}",
            report.not_ingested
        );
        assert!(
            report.failures.is_empty(),
            "every committed spell fixture must clear the bar, got {} failures, first few: {:?}",
            report.failures.len(),
            report.failures.iter().take(5).collect::<Vec<_>>()
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }

    // MUTATION PROOF: a fixture asserting a wrong expected value must make
    // the bar check fail, not silently pass -- the same guarantee
    // `run_monster_bar_check_clears_every_committed_monster_fixture`'s
    // sibling mutation test establishes for the monster seam. Constructs a
    // synthetic corpus dir + fixture rather than mutating the committed
    // file (which a concurrent cycle may also be reading).
    #[test]
    fn a_wrong_expected_per_level_makes_the_evaluator_disagree() {
        let real = parse_caster_level_linear_duration("(CASTERLEVEL*10) minutes [D]").unwrap();
        let wrong_expected_per_level = 99;
        assert_ne!(
            real.per_level, wrong_expected_per_level,
            "a corrupted expected value must genuinely disagree with the real parse"
        );
    }

    #[test]
    fn a_wrong_expected_unit_makes_the_evaluator_disagree() {
        let real = parse_caster_level_linear_duration("(CASTERLEVEL) minutes").unwrap();
        assert_ne!(real.unit, "rounds", "a corrupted expected unit must genuinely disagree");
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

    // An unparseable formula (the real Demon (Vermlek) worked example,
    // `book_of_the_damned_volume_2`, `BONUS:VAR|SLA_CL|HD*3/4`) has no
    // computable value without a formula interpreter this repo does not
    // have -- an honest `None`, never a guess at what `HD*3/4` might round
    // to.
    #[test]
    fn an_unparseable_sla_cl_formula_refuses_rather_than_guesses() {
        let block = stat_block_full(Some("Outsider:16"), true, Some("HD*3/4"));
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
            report.not_ingested.is_empty(),
            "every committed monster fixture's book must be ingested, got: {:?}",
            report.not_ingested
        );
        assert_eq!(report.cleared.len(), report.fixtures_total);
    }
}
