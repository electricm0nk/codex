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
//! record through the shipping `compute_equipment_effects` seam, must
//! produce exactly the ability list and magnitude the fixture's pinned
//! corpus field states. See `tests/derived_evaluator_fixture_check.rs`'s
//! module doc for the four independent guarantees this rests on.

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
    let mut cleared = equipment.cleared;
    cleared.extend(monster.cleared);
    let mut failures = equipment.failures;
    failures.extend(monster.failures);
    let mut not_ingested = equipment.not_ingested;
    not_ingested.extend(monster.not_ingested);
    BarCheckReport {
        cleared,
        failures,
        not_ingested,
        fixtures_total: equipment.fixtures_total + monster.fixtures_total,
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
    hd_str.trim().parse::<i32>().ok()
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
        let Some(monster) = monster_book.monsters.iter().find(|m| m.name == fixture.record_key)
        else {
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

#[cfg(test)]
mod monster_seam_tests {
    use super::*;

    fn stat_block(monster_class: Option<&'static str>) -> MonsterStatBlock {
        stat_block_with_sla(monster_class, true)
    }

    /// [`stat_block`] plus explicit control over
    /// [`MonsterStatBlock::has_spell_like_abilities`] (SD31-E6-F1-002,
    /// `OPEN-ISSUES.md` row 44) -- every pre-existing test in this module is
    /// about the HD-parsing rule, not the presence gate, so `stat_block`
    /// keeps defaulting to `true` and only the presence-gate test below calls
    /// this directly with `false`.
    fn stat_block_with_sla(
        monster_class: Option<&'static str>,
        has_spell_like_abilities: bool,
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
        let block = stat_block_with_sla(Some("Construct:3"), false);
        assert_eq!(
            spell_like_ability_caster_level(&block),
            None,
            "a monster with no BONUS:VAR|SLA_CL| token has no spell-like abilities, regardless \
             of HD"
        );
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
