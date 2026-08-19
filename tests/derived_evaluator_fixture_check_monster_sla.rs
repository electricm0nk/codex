//! The SECOND `kind=monster` seam of the `derived` wiring class's
//! evaluator-vs-fixture check (SD31-W15-MONSTER-SLA-001) — the
//! spell-like-ability SAVE DC, over `derived-evaluator-fixtures.json`'s
//! `monster_sla_entries` array.
//!
//! Sibling of `tests/derived_evaluator_fixture_check_monster.rs`, which
//! covers the `BONUS:VAR|SLA_CL|` half of the same universal monster rule.
//! A second seam was needed because the first is exhausted: of the 316
//! `monster` units the board still held at `derived`+`grounded` when this
//! seam was built, exactly ONE carries a `BONUS:VAR|SLA_CL|` token at all.
//!
//! # The bar
//!
//! PF1's own "Spell-Like Abilities" universal monster rule (`Bestiary`
//! Appendix 1; the Core Rulebook states the same formula generally): a
//! spell-like ability's save DC is **10 + the spell's level + the creature's
//! ability modifier**. PCGen encodes it per-spell as the trailing
//! `,<constant>+<ABILITY>` of a `SPELLS:` grant.
//!
//! The modifier is never resolved — a monster's ability SCORES are not a
//! corpus-stated fact in this repo (`SD31-E6-F1-002` refused to compute that
//! family rather than fabricate it), and `MonsterStatBlock::stat_adjustments`
//! carries adjustments, not scores. What IS derivable with no live creature
//! is the formula's own parameters: the scaling ability, and — by running the
//! rule backwards over the corpus-stated constant — **the spell's own level**.
//!
//! # Why this seam is non-circular in a way the others are not
//!
//! Every other seam in this family reads its expected value out of the SAME
//! corpus field the evaluator parses, and rests its independence on
//! provenance pinning plus a separately-written parser. This one reads the
//! expected value out of a **different file**: the granted spell's own PCGen
//! record, whose `CLASSES:<classes>=<level>` token states the level directly.
//! The evaluator never opens that file. So the check is a genuine agreement
//! test between two independently authored corpus facts, joined only by the
//! printed rule — and
//! [`monster_sla_expected_value_is_transcribed_from_the_spell_record_not_the_dc_token`]
//! below asserts that property directly rather than leaving it as prose.
//!
//! # The same four independent guarantees
//!
//! 1. **Different source artifact** — expected values come from the upstream
//!    PCGen spell `.lst` bytes; the engine evaluates this repo's own compiled
//!    `monster_chassis::MONSTER_BOOKS` registry, generated from a third
//!    artifact (`data/corpus/**/*.json`).
//! 2. **Committed first.** The `monster_sla_entries` rows and the seam that
//!    reads them landed in the same commit as this file.
//! 3. **Re-derivable from the pinned corpus field** —
//!    [`monster_sla_expected_values_are_re_derivable_from_the_pinned_spell_record`],
//!    a reference `CLASSES:` parser this file alone owns.
//! 4. **Anchored to the same upstream bytes the engine ingested** —
//!    [`monster_sla_pinned_fields_are_byte_identical_to_the_upstream_lsts`]
//!    (re-hashes BOTH pinned files) and
//!    [`monster_sla_engine_ingest_cites_the_same_upstream_bytes`].
//!
//! # Mutation-proof
//!
//! [`moving_the_save_dc_base_constant_makes_every_committed_fixture_fail`]
//! re-runs the production rule with the constant moved by one and requires
//! every committed fixture to stop matching — the strongest available
//! statement that this check can go red, since that constant IS the rule.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use codex::rules_core::derived_evaluator_fixture_check::{
    load_monster_sla_fixtures, spell_like_ability_save_dc, SPELL_LIKE_ABILITY_SAVE_DC_BASE,
};
use codex::rules_core::rules_tables::monster_chassis::MONSTER_BOOKS;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn pcgen_data_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("PCGEN_CORPUS_ROOT") {
        return Some(PathBuf::from(root));
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join("workspace").join("repos").join("pcgen").join("data"))
}

/// Bestiary 1's corpus directory is `beastiary`; its work-inventory `book`
/// field is `bestiary`. The same one alias the seam itself carries.
fn monster_corpus_dir(book: &str) -> &str {
    match book {
        "bestiary" => "beastiary",
        other => other,
    }
}

fn monster_ingested_provenance(book: &str) -> BTreeMap<String, (String, u64, String)> {
    let mut out = BTreeMap::new();
    let dir = repo_root().join("data").join("corpus").join(monster_corpus_dir(book)).join("monster");
    let Ok(entries) = std::fs::read_dir(&dir) else { return out };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        let Some(key) = value["data"]["corpus_key"].as_str() else { continue };
        let source = &value["source"];
        let (Some(source_path), Some(line), Some(sha)) =
            (source["path"].as_str(), source["line"].as_u64(), source["sha256"].as_str())
        else {
            continue;
        };
        out.insert(key.to_string(), (source_path.to_string(), line, sha.to_string()));
    }
    out
}

/// The reference reader for a spell record's `CLASSES:` token, written
/// independently of both `derived_evaluator_fixture_check.rs` and
/// `scripts/derive_monster_sla_spell_level_fixtures.py` — same PCGen
/// grammar, third implementation, so no two of them can silently agree on a
/// shared bug.
///
/// `CLASSES:Bard,Cleric=6|Sorcerer,Wizard=7` states TWO levels;
/// `CLASSES:Bard,Sorcerer,Wizard=2` states one. Returns every level stated.
fn reference_levels_from_classes_token(token: &str) -> BTreeSet<i32> {
    let mut out = BTreeSet::new();
    let Some(value) = token.strip_prefix("CLASSES:") else { return out };
    for chunk in value.split('|') {
        let Some((_classes, level)) = chunk.rsplit_once('=') else { continue };
        let level = level.trim();
        if let Ok(level) = level.parse::<i32>() {
            out.insert(level);
        }
    }
    out
}

#[test]
fn reference_classes_reader_handles_both_single_and_multi_level_spells() {
    assert_eq!(
        reference_levels_from_classes_token("CLASSES:Bard,Sorcerer,Wizard=2"),
        BTreeSet::from([2])
    );
    assert_eq!(
        reference_levels_from_classes_token("CLASSES:Bard,Cleric=6|Sorcerer,Wizard=7"),
        BTreeSet::from([6, 7])
    );
    // Not a CLASSES token at all, and a token stating no level: both empty,
    // never a guessed default.
    assert!(reference_levels_from_classes_token("SCHOOL:Illusion").is_empty());
    assert!(reference_levels_from_classes_token("CLASSES:Bard").is_empty());
}

/// Guarantee 3: every committed row's `expected.spell_level` reproduces from
/// its own pinned `spell_level_corpus_field`, via a reference reader this
/// file alone owns.
#[test]
fn monster_sla_expected_values_are_re_derivable_from_the_pinned_spell_record() {
    let fixtures = load_monster_sla_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "an empty monster_sla_entries would make this suite vacuous");
    for fixture in &fixtures {
        let levels = reference_levels_from_classes_token(&fixture.spell_level_corpus_field);
        assert_eq!(
            levels.len(),
            1,
            "{}: {:?}'s pinned spell record states levels {:?} — a fixture may only be built \
             on a spell whose own record determines exactly ONE level, because otherwise the \
             expected value could not be transcribed and would have to be computed from the \
             DC token under test",
            fixture.unit_id,
            fixture.spell,
            levels
        );
        assert_eq!(
            levels.into_iter().next(),
            Some(fixture.expected_spell_level),
            "{}: fixture states spell level {} for {:?}, but its own pinned {:?} says otherwise",
            fixture.unit_id,
            fixture.expected_spell_level,
            fixture.spell,
            fixture.spell_level_corpus_field
        );
    }
}

/// THE non-circularity assertion, stated as a test rather than left as
/// prose: the expected value is what the SPELL RECORD says, and the DC token
/// is what the EVALUATOR reads. The two files must be different files.
///
/// A fixture whose `spell_level_lst` was the monster's own `.lst` would be
/// reading its expectation out of the same bytes it is testing, which is
/// precisely the circularity that makes a fixture worth nothing.
#[test]
fn monster_sla_expected_value_is_transcribed_from_the_spell_record_not_the_dc_token() {
    let fixtures = load_monster_sla_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "an empty monster_sla_entries would make this suite vacuous");
    for fixture in &fixtures {
        assert_ne!(
            fixture.spell_level_lst, fixture.upstream_lst,
            "{}: the expected spell level for {:?} was read from the SAME file as the DC token \
             under test ({}), which makes the fixture a restatement of the corpus row rather \
             than an independent check of it",
            fixture.unit_id, fixture.spell, fixture.upstream_lst
        );
        assert!(
            fixture.spell_level_corpus_field.starts_with("CLASSES:"),
            "{}: the expected spell level must come from the spell record's own CLASSES: token, \
             got {:?}",
            fixture.unit_id,
            fixture.spell_level_corpus_field
        );
    }
}

/// Guarantee 4a: BOTH pinned files still hash to their recorded sha256, the
/// monster row still carries the DC token verbatim, and the spell record's
/// pinned line still carries the `CLASSES:` token verbatim.
#[test]
fn monster_sla_pinned_fields_are_byte_identical_to_the_upstream_lsts() {
    let fixtures = load_monster_sla_fixtures(&repo_root());
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("skipped: neither PCGEN_CORPUS_ROOT nor HOME is set");
        return;
    };
    if !data_root.is_dir() {
        eprintln!("skipped: no PCGen checkout at {data_root:?}");
        return;
    }

    let mut file_text: BTreeMap<PathBuf, (String, Vec<String>)> = BTreeMap::new();
    let mut read = |path: PathBuf, cache: &mut BTreeMap<PathBuf, (String, Vec<String>)>| {
        cache
            .entry(path.clone())
            .or_insert_with(|| {
                let bytes = std::fs::read(&path)
                    .unwrap_or_else(|e| panic!("upstream file {path:?} must be readable: {e}"));
                let sha = sha256_hex(&bytes);
                let lines =
                    String::from_utf8_lossy(&bytes).split('\n').map(str::to_string).collect();
                (sha, lines)
            })
            .clone()
    };

    let mut wrong = Vec::new();
    for fixture in &fixtures {
        let monster_path = data_root.join(&fixture.upstream_lst);
        let (sha, lines) = read(monster_path, &mut file_text);
        if sha != fixture.upstream_lst_sha256 {
            wrong.push(format!(
                "{}: {} now hashes to {sha}, fixture recorded {}",
                fixture.unit_id, fixture.upstream_lst, fixture.upstream_lst_sha256
            ));
            continue;
        }
        let index = usize::try_from(fixture.upstream_line).expect("line number fits in usize");
        match index.checked_sub(1).and_then(|i| lines.get(i)) {
            None => wrong.push(format!(
                "{}: {} has no line {}",
                fixture.unit_id, fixture.upstream_lst, fixture.upstream_line
            )),
            Some(line) => {
                // The DC token never appears alone — it is the tail of a
                // `<spell>,<dc>` segment, so require the WHOLE segment.
                let segment = format!("{},{}", fixture.spell, fixture.corpus_field);
                if !line.contains(&segment) {
                    wrong.push(format!(
                        "{}: line {} of {} does not contain {:?} verbatim",
                        fixture.unit_id, fixture.upstream_line, fixture.upstream_lst, segment
                    ));
                }
            }
        }

        let spell_path = data_root.join(&fixture.spell_level_lst);
        let (spell_sha, spell_lines) = read(spell_path, &mut file_text);
        if spell_sha != fixture.spell_level_lst_sha256 {
            wrong.push(format!(
                "{}: {} now hashes to {spell_sha}, fixture recorded {}",
                fixture.unit_id, fixture.spell_level_lst, fixture.spell_level_lst_sha256
            ));
            continue;
        }
        let index =
            usize::try_from(fixture.spell_level_line).expect("line number fits in usize");
        match index.checked_sub(1).and_then(|i| spell_lines.get(i)) {
            None => wrong.push(format!(
                "{}: {} has no line {}",
                fixture.unit_id, fixture.spell_level_lst, fixture.spell_level_line
            )),
            Some(line) => {
                if !line.contains(&fixture.spell_level_corpus_field) {
                    wrong.push(format!(
                        "{}: line {} of {} does not contain {:?} verbatim",
                        fixture.unit_id,
                        fixture.spell_level_line,
                        fixture.spell_level_lst,
                        fixture.spell_level_corpus_field
                    ));
                }
                // …and it must be the record for THIS spell, not merely a
                // line that happens to carry the same CLASSES token. PCGen's
                // record identity is `KEY:` where present, else column 0.
                let identity_ok = line.contains(&format!("KEY:{}", fixture.spell))
                    || line.split('\t').next().map(str::trim) == Some(fixture.spell.as_str());
                if !identity_ok {
                    wrong.push(format!(
                        "{}: line {} of {} is not the record for {:?}",
                        fixture.unit_id,
                        fixture.spell_level_line,
                        fixture.spell_level_lst,
                        fixture.spell
                    ));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Guarantee 4b: this repo's own ingest of the same `record_key` cites the
/// same upstream `(path, line, sha256)` the fixture pins for the MONSTER row.
#[test]
fn monster_sla_engine_ingest_cites_the_same_upstream_bytes() {
    let fixtures = load_monster_sla_fixtures(&repo_root());
    let mut provenance_by_book: BTreeMap<String, BTreeMap<String, (String, u64, String)>> =
        BTreeMap::new();
    let mut mismatched: Vec<String> = Vec::new();
    let mut compared = 0usize;

    for fixture in &fixtures {
        let records = provenance_by_book
            .entry(fixture.book.clone())
            .or_insert_with(|| monster_ingested_provenance(&fixture.book));
        let Some((path, line, sha)) = records.get(&fixture.record_key) else {
            mismatched.push(format!(
                "{}: book {} records no `.lst` provenance for {:?}",
                fixture.unit_id, fixture.book, fixture.record_key
            ));
            continue;
        };
        if path != &fixture.upstream_lst
            || *line != fixture.upstream_line
            || sha != &fixture.upstream_lst_sha256
        {
            mismatched.push(format!(
                "{}: fixture read {}:{} (sha {}), ingest cites {}:{} (sha {})",
                fixture.unit_id,
                fixture.upstream_lst,
                fixture.upstream_line,
                fixture.upstream_lst_sha256,
                path,
                line,
                sha
            ));
            continue;
        }
        compared += 1;
    }

    assert!(
        mismatched.is_empty(),
        "{} monster_sla fixture(s) disagree with the engine's own ingest provenance:\n{}",
        mismatched.len(),
        mismatched.join("\n")
    );
    assert_eq!(compared, fixtures.len(), "every committed row must be cross-checked");
}

/// Every committed row resolves through the real registry and the real
/// evaluator produces exactly the fixture's expected value — the positive
/// half of the mutation pair below.
#[test]
fn the_real_evaluator_reproduces_every_committed_expected_spell_level() {
    let fixtures = load_monster_sla_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "an empty monster_sla_entries would make this suite vacuous");
    let mut wrong = Vec::new();
    for fixture in &fixtures {
        let registry_book =
            if fixture.book == "bestiary" { "beastiary" } else { fixture.book.as_str() };
        let Some(book) = MONSTER_BOOKS.iter().find(|b| b.corpus_book == registry_book) else {
            wrong.push(format!("{}: book {registry_book} is not registered", fixture.unit_id));
            continue;
        };
        let Some(monster) = book.monster_resolve(&fixture.record_key) else {
            wrong.push(format!("{}: {:?} does not resolve", fixture.unit_id, fixture.record_key));
            continue;
        };
        let Some(sla) = monster.spell_like_abilities.iter().find(|s| s.spell == fixture.spell)
        else {
            wrong.push(format!("{}: no SLA named {:?}", fixture.unit_id, fixture.spell));
            continue;
        };
        match spell_like_ability_save_dc(sla) {
            Some(dc)
                if dc.spell_level == fixture.expected_spell_level
                    && dc.ability == fixture.expected_ability => {}
            other => wrong.push(format!(
                "{}: {:?} expected level {} ({}), evaluator produced {:?}",
                fixture.unit_id,
                fixture.spell,
                fixture.expected_spell_level,
                fixture.expected_ability,
                other
            )),
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// MUTATION-PROOF. The rule this seam applies is one constant:
/// `spell level = DC constant - 10`. This test re-runs the production rule
/// with that constant moved by one and requires EVERY committed fixture to
/// stop matching. If a single one still matched, the fixture would not be
/// pinned to the rule at all.
///
/// Moving the real constant in `derived_evaluator_fixture_check.rs` and
/// re-running the suite reproduces this by hand; this test makes the same
/// statement in a form that runs on every gate, so the seam cannot silently
/// become vacuous later.
#[test]
fn moving_the_save_dc_base_constant_makes_every_committed_fixture_fail() {
    let fixtures = load_monster_sla_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "an empty monster_sla_entries would make this suite vacuous");
    let mut still_matching = Vec::new();
    for fixture in &fixtures {
        let constant: i32 = fixture
            .corpus_field
            .split('+')
            .next()
            .and_then(|c| c.trim().parse().ok())
            .unwrap_or_else(|| panic!("{}: DC token {:?} has no constant", fixture.unit_id, fixture.corpus_field));
        // The mutation: the rule's base off by one.
        let mutated_level = constant - (SPELL_LIKE_ABILITY_SAVE_DC_BASE + 1);
        if mutated_level == fixture.expected_spell_level {
            still_matching.push(format!(
                "{}: {:?} still matches with the rule constant moved to {}",
                fixture.unit_id,
                fixture.spell,
                SPELL_LIKE_ABILITY_SAVE_DC_BASE + 1
            ));
        }
    }
    assert!(
        still_matching.is_empty(),
        "{} fixture(s) survive a mutation of the rule they exist to pin — they are vacuous:\n{}",
        still_matching.len(),
        still_matching.join("\n")
    );
}

/// The evaluator's own refusals, each an honest absence rather than a guess.
#[test]
fn the_evaluator_refuses_every_shape_it_cannot_ground() {
    use codex::rules_core::rules_tables::monster_chassis::MonsterSpellLikeAbility;
    let sla = |dc: Option<&'static str>| MonsterSpellLikeAbility {
        label: "Innate",
        times: Some("3"),
        time_unit: None,
        caster_level_token: Some("12"),
        spell: "Fireball",
        save_dc_token: dc,
    };
    // A spell that allows no save states no DC — a real absence.
    assert_eq!(spell_like_ability_save_dc(&sla(None)), None);
    // No `+<ability>` tail at all.
    assert_eq!(spell_like_ability_save_dc(&sla(Some("15"))), None);
    // A non-integer constant.
    assert_eq!(spell_like_ability_save_dc(&sla(Some("CHA+CHA"))), None);
    // A constant below the rule's own base would imply a negative spell
    // level, so the token is not this shape.
    assert_eq!(spell_like_ability_save_dc(&sla(Some("9+CHA"))), None);
    // The real shape, both corpus-observed abilities.
    let dc = spell_like_ability_save_dc(&sla(Some("15+CHA"))).expect("real shape parses");
    assert_eq!(dc.spell_level, 5);
    assert_eq!(dc.ability, "CHA");
    let dc = spell_like_ability_save_dc(&sla(Some("11+INT"))).expect("real shape parses");
    assert_eq!(dc.spell_level, 1);
    assert_eq!(dc.ability, "INT");
}
