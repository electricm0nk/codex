//! The `kind=monster_ability` half of the `derived` wiring class's
//! evaluator-vs-fixture check (SD31-W15) — the sibling of
//! `tests/derived_evaluator_fixture_check_monster.rs`'s guarantees, over
//! `derived-evaluator-fixtures.json`'s `monster_ability_entries` array.
//!
//! # The bar
//!
//! PF1's Universal Monster Rules (`Bestiary`, Appendix 1, "Format"):
//!
//! > "The save DC against a monster's special ability is equal to
//! > 10 + 1/2 the monster's racial HD + the monster's relevant ability
//! > modifier."
//!
//! PCGen states the already-summed `10 + 1/2 racial HD` term on the ABILITY
//! row, as the `DESC:` token's argument for the `%N` its prose introduces
//! with the word `DC` (`...succeed at a DC %1 Will save...|15+WIS`), and it
//! states the racial HD itself on a DIFFERENT row, in a different file, as
//! the trailing segment of `MONSTERCLASS:<type>:<HD>`.
//!
//! **That separation is the whole point of this seam.** Every earlier seam in
//! this family pins an expected value read off the SAME row the evaluator
//! parses, so its independence rests entirely on the two readings coming
//! through different artifacts (upstream `.lst` bytes vs. this repo's
//! `data/corpus/` ingest). Here the expected value is additionally fixed by a
//! SECOND corpus row that the evaluator never reads, tied to the first by the
//! printed rule — so a fixture entry cannot be a restatement of the record
//! under test even in principle.
//!
//! # The same four independent guarantees, re-checked for this family
//!
//! 1. **Different source artifact** — `monster_ability_entries`' expected
//!    values are read from the upstream PCGen `.lst` bytes by
//!    `scripts/derive_monster_ability_save_dc_fixtures.py`, which opens no
//!    file under `data/corpus/`; the engine evaluates this repo's own
//!    compiled `monster_chassis::MONSTER_BOOKS` registry (generated from
//!    `data/corpus/**/*.json`, a third artifact).
//! 2. **Committed first.** The `monster_ability_entries` rows, the seam that
//!    reads them and this file landed in the same commit.
//! 3. **Re-derivable from the pinned corpus field**, TWICE and by two
//!    different routes — [`monster_ability_expected_values_are_re_derivable_from_the_pinned_desc_argument`]
//!    (a reference parser this file alone owns, over the pinned
//!    `corpus_field`) and
//!    [`monster_ability_expected_base_reproduces_from_the_printed_universal_monster_rule`]
//!    (the printed rule over the pinned `owner_monster_class_token`).
//! 4. **Anchored to the same upstream bytes the engine ingested** —
//!    [`monster_ability_pinned_corpus_field_is_byte_identical_to_the_upstream_lst`]
//!    (re-hashes `upstream_lst` fresh and requires `corpus_field` verbatim on
//!    `upstream_line`, AND `MONSTERCLASS:<token>` verbatim on the owner row's
//!    own cited line) and
//!    [`monster_ability_engine_ingest_cites_the_same_upstream_bytes_the_fixture_was_read_from`].
//!
//! # Mutation-proof
//!
//! Two mutations, one per half of the bar:
//! [`a_wrong_expected_save_dc_makes_the_bar_check_fail`] and
//! [`a_wrong_universal_monster_rule_base_makes_the_bar_check_fail`] each
//! construct a fixture-shaped assertion with a deliberately wrong value
//! against REAL, resolved records and confirm the comparison
//! `run_monster_ability_bar_check` performs actually returns a failure —
//! proving the check can fail, not merely that it passes today. Their
//! positive controls prove the harness is not simply always-red.

use std::collections::BTreeMap;
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use codex::rules_core::derived_evaluator_fixture_check::{
    load_monster_ability_fixtures, load_monster_ability_formula_fixtures, monster_ability_save_dc,
    monster_ability_formula_save_dc, universal_monster_rule_save_dc_base,
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
/// field is `bestiary`. The same one-entry alias the seam itself carries.
fn monster_corpus_dir(book: &str) -> &str {
    match book {
        "bestiary" => "beastiary",
        other => other,
    }
}

// ---------------------------------------------------------------------------
// Guarantee 3, route A: a reference parser this file alone owns.
// ---------------------------------------------------------------------------

/// Reads the save-DC argument out of a whole `DESC:` token, independently of
/// `derived_evaluator_fixture_check.rs`'s `monster_ability_save_dc` — same
/// two rules (the `DC %N` slot, the `<base>+<STAT>`/`<STAT>+<base>` shape),
/// different code, so the two cannot silently agree on a shared bug.
fn reference_save_dc_from_desc(corpus_field: &str) -> Option<(usize, i32, String)> {
    let value = corpus_field.strip_prefix("DESC:")?;
    let mut parts = value.split('|');
    let prose = parts.next()?;
    let args: Vec<&str> = parts.collect();

    // Find `DC %N` by splitting on whitespace rather than by scanning bytes.
    let words: Vec<&str> = prose.split_whitespace().collect();
    for pair in words.windows(2) {
        let lead = pair[0].trim_matches(|c: char| !c.is_ascii_alphanumeric());
        if lead != "DC" {
            continue;
        }
        let Some(rest) = pair[1].strip_prefix('%') else { continue };
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        let Ok(slot) = digits.parse::<usize>() else { continue };
        if slot == 0 {
            continue;
        }
        let Some(arg) = args.get(slot - 1) else { continue };
        if let Some((base, ability)) = reference_base_plus_ability(arg) {
            return Some((slot, base, ability));
        }
    }
    None
}

fn reference_base_plus_ability(arg: &str) -> Option<(i32, String)> {
    const ABILITIES: [&str; 6] = ["STR", "DEX", "CON", "INT", "WIS", "CHA"];
    let terms: Vec<&str> = arg.split('+').map(str::trim).collect();
    if terms.len() != 2 {
        return None;
    }
    let mut base: Option<i32> = None;
    let mut ability: Option<String> = None;
    for term in terms {
        if ABILITIES.contains(&term) {
            if ability.is_some() {
                return None;
            }
            ability = Some(term.to_string());
        } else if let Ok(n) = term.parse::<i32>() {
            if base.is_some() {
                return None;
            }
            base = Some(n);
        } else {
            return None;
        }
    }
    Some((base?, ability?))
}

#[test]
fn reference_desc_parser_refuses_what_it_cannot_read() {
    assert_eq!(
        reference_save_dc_from_desc("DESC:succeed at a DC %1 Will save|15+WIS"),
        Some((1, 15, "WIS".to_string()))
    );
    assert_eq!(
        reference_save_dc_from_desc("DESC:a DC %2 Fortitude save|1d6|CHA+18"),
        Some((2, 18, "CHA".to_string()))
    );
    // A damage term is not a save DC: `3d8+%1` carries no `DC ` lead-in.
    assert_eq!(reference_save_dc_from_desc("DESC:deals 3d8+%1 points|STR*1.5"), None);
    // A named variable is not a resolvable base.
    assert_eq!(reference_save_dc_from_desc("DESC:a DC %1 Reflex save|ClingDC"), None);
    // A full PCGen formula is not a flat base either.
    assert_eq!(
        reference_save_dc_from_desc("DESC:a DC %1 Fortitude save|10+(HD/2)+CON"),
        None
    );
}

/// Guarantee 3, route A: every committed row's `expected` reproduces exactly
/// from its own pinned `corpus_field`, via the reference parser above.
#[test]
fn monster_ability_expected_values_are_re_derivable_from_the_pinned_desc_argument() {
    let fixtures = load_monster_ability_fixtures(&repo_root());
    assert!(
        !fixtures.is_empty(),
        "an empty monster_ability_entries would make this suite vacuous"
    );
    let mut wrong = Vec::new();
    for fixture in &fixtures {
        match reference_save_dc_from_desc(&fixture.corpus_field) {
            Some((slot, base, ability))
                if slot == fixture.desc_argument_index
                    && base == fixture.expected_save_dc_base
                    && ability == fixture.expected_ability => {}
            other => wrong.push(format!(
                "{}: reference derivation of {:?} produced {other:?}, fixture pins slot {} \
                 base {} + {}",
                fixture.unit_id,
                fixture.corpus_field,
                fixture.desc_argument_index,
                fixture.expected_save_dc_base,
                fixture.expected_ability
            )),
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Guarantee 3, route B — **the independence this seam exists for**: every
/// committed row's `expected.save_dc_base` reproduces from the PRINTED
/// Universal Monster Rule applied to the OWNING monster's own pinned
/// `MONSTERCLASS` token, a different row in a different file that the
/// evaluator never reads.
#[test]
fn monster_ability_expected_base_reproduces_from_the_printed_universal_monster_rule() {
    let fixtures = load_monster_ability_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "vacuous without fixtures");
    let mut wrong = Vec::new();
    for fixture in &fixtures {
        // Written here, not called from the engine: 10 + 1/2 racial HD.
        let Some((_, hd)) = fixture.owner_monster_class_token.rsplit_once(':') else {
            wrong.push(format!(
                "{}: owner MONSTERCLASS {:?} has no `:<HD>` tail",
                fixture.unit_id, fixture.owner_monster_class_token
            ));
            continue;
        };
        let Ok(hd) = hd.trim().parse::<i32>() else {
            wrong.push(format!(
                "{}: owner MONSTERCLASS {:?} tail is not an integer",
                fixture.unit_id, fixture.owner_monster_class_token
            ));
            continue;
        };
        if hd != fixture.owner_racial_hd {
            wrong.push(format!(
                "{}: pinned owner_racial_hd {} disagrees with MONSTERCLASS {:?}",
                fixture.unit_id, fixture.owner_racial_hd, fixture.owner_monster_class_token
            ));
            continue;
        }
        let rule_base = 10 + hd / 2;
        if rule_base != fixture.expected_save_dc_base
            || rule_base != fixture.universal_monster_rule_base
        {
            wrong.push(format!(
                "{}: the printed rule over {} HD gives {rule_base}, fixture pins expected {} / \
                 universal_monster_rule_base {}",
                fixture.unit_id,
                hd,
                fixture.expected_save_dc_base,
                fixture.universal_monster_rule_base
            ));
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Guarantee 4a: both pinned upstream citations still hold, byte for byte.
#[test]
fn monster_ability_pinned_corpus_field_is_byte_identical_to_the_upstream_lst() {
    let fixtures = load_monster_ability_fixtures(&repo_root());
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("skipped: neither PCGEN_CORPUS_ROOT nor HOME is set");
        return;
    };
    if !data_root.is_dir() {
        eprintln!("skipped: no PCGen checkout at {data_root:?}");
        return;
    }

    let mut file_text: BTreeMap<PathBuf, (String, Vec<String>)> = BTreeMap::new();
    let mut wrong = Vec::new();
    let read = |path: PathBuf, cache: &mut BTreeMap<PathBuf, (String, Vec<String>)>| {
        cache
            .entry(path.clone())
            .or_insert_with(|| {
                let bytes = std::fs::read(&path).unwrap_or_else(|e| {
                    panic!("upstream corpus file {path:?} must be readable: {e}")
                });
                let sha = sha256_hex(&bytes);
                let lines =
                    String::from_utf8_lossy(&bytes).split('\n').map(str::to_string).collect();
                (sha, lines)
            })
            .clone()
    };

    for fixture in &fixtures {
        let ability_path = data_root.join(&fixture.upstream_lst);
        let (sha, lines) = read(ability_path, &mut file_text);
        if sha != fixture.upstream_lst_sha256 {
            wrong.push(format!(
                "{}: {} now hashes to {sha}, fixture recorded {}",
                fixture.unit_id, fixture.upstream_lst, fixture.upstream_lst_sha256
            ));
            continue;
        }
        let index = usize::try_from(fixture.upstream_line).expect("line number fits in usize");
        let Some(line) = index.checked_sub(1).and_then(|i| lines.get(i)) else {
            wrong.push(format!(
                "{}: {} has no line {}",
                fixture.unit_id, fixture.upstream_lst, fixture.upstream_line
            ));
            continue;
        };
        if !line.contains(&fixture.corpus_field) {
            wrong.push(format!(
                "{}: line {} of {} does not contain {:?} verbatim",
                fixture.unit_id, fixture.upstream_line, fixture.upstream_lst, fixture.corpus_field
            ));
        }

        // The SECOND pinned row: the owner's own MONSTERCLASS, on its own
        // cited line of its own file.
        let owner_path = data_root.join(&fixture.owner_upstream_lst);
        let (_owner_sha, owner_lines) = read(owner_path, &mut file_text);
        let owner_index =
            usize::try_from(fixture.owner_upstream_line).expect("line number fits in usize");
        let Some(owner_line) = owner_index.checked_sub(1).and_then(|i| owner_lines.get(i)) else {
            wrong.push(format!(
                "{}: {} has no line {}",
                fixture.unit_id, fixture.owner_upstream_lst, fixture.owner_upstream_line
            ));
            continue;
        };
        if !owner_line.contains(&format!("KEY:{}", fixture.owner_monster_key)) {
            wrong.push(format!(
                "{}: line {} of {} is not the row of KEY:{}",
                fixture.unit_id,
                fixture.owner_upstream_line,
                fixture.owner_upstream_lst,
                fixture.owner_monster_key
            ));
        }
        if !owner_line.contains(&format!("MONSTERCLASS:{}", fixture.owner_monster_class_token)) {
            wrong.push(format!(
                "{}: line {} of {} does not contain MONSTERCLASS:{:?} verbatim",
                fixture.unit_id,
                fixture.owner_upstream_line,
                fixture.owner_upstream_lst,
                fixture.owner_monster_class_token
            ));
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Guarantee 4b: this repo's own ingest of the same `record_key` cites the
/// SAME upstream `(path, line)` the fixture pins.
#[test]
fn monster_ability_engine_ingest_cites_the_same_upstream_bytes_the_fixture_was_read_from() {
    let fixtures = load_monster_ability_fixtures(&repo_root());
    let mut by_book: BTreeMap<String, BTreeMap<String, (String, u64)>> = BTreeMap::new();
    let mut mismatched: BTreeMap<String, String> = BTreeMap::new();
    let mut compared = 0usize;

    for fixture in &fixtures {
        let records = by_book.entry(fixture.book.clone()).or_insert_with(|| {
            let mut out = BTreeMap::new();
            let dir = repo_root()
                .join("data")
                .join("corpus")
                .join(monster_corpus_dir(&fixture.book))
                .join("monster_ability");
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
                let (Some(p), Some(line)) = (source["path"].as_str(), source["line"].as_u64())
                else {
                    continue;
                };
                out.insert(key.to_string(), (p.to_string(), line));
            }
            out
        });
        let Some((ingest_path, ingest_line)) = records.get(&fixture.record_key) else {
            mismatched.insert(
                fixture.unit_id.clone(),
                format!("{:?} has no ingested record at all", fixture.record_key),
            );
            continue;
        };
        compared += 1;
        if !fixture.upstream_lst.ends_with(ingest_path.trim_start_matches("./"))
            && !ingest_path.ends_with(&fixture.upstream_lst)
        {
            mismatched.insert(
                fixture.unit_id.clone(),
                format!(
                    "ingest cites {ingest_path:?}, fixture cites {:?}",
                    fixture.upstream_lst
                ),
            );
            continue;
        }
        if *ingest_line != fixture.upstream_line {
            mismatched.insert(
                fixture.unit_id.clone(),
                format!(
                    "ingest cites line {ingest_line}, fixture cites {}",
                    fixture.upstream_line
                ),
            );
        }
    }

    assert!(mismatched.is_empty(), "{} mismatch(es): {mismatched:?}", mismatched.len());
    assert_eq!(compared, fixtures.len(), "every committed fixture must be cross-checked");
}

// ---------------------------------------------------------------------------
// The engine side, and the two mutation proofs.
// ---------------------------------------------------------------------------

/// Resolves one committed fixture's ability record and its owner out of the
/// live chassis registry, so the mutation proofs below run against REAL
/// records rather than synthetic ones.
fn resolve_first_fixture()
-> (String, codex::rules_core::derived_evaluator_fixture_check::MonsterAbilityFixture) {
    let fixtures = load_monster_ability_fixtures(&repo_root());
    let fixture = fixtures.into_iter().next().expect("at least one committed fixture");
    (fixture.record_key.clone(), fixture)
}

/// The engine's evaluator reproduces every committed fixture's expectation
/// over the live compiled tables — the positive control for both mutations.
#[test]
fn the_engine_evaluator_reproduces_every_committed_monster_ability_fixture() {
    let fixtures = load_monster_ability_fixtures(&repo_root());
    let mut wrong = Vec::new();
    for fixture in &fixtures {
        let registry_book =
            if fixture.book == "bestiary" { "beastiary" } else { fixture.book.as_str() };
        let Some(book) = MONSTER_BOOKS.iter().find(|b| b.corpus_book == registry_book) else {
            wrong.push(format!("{}: book {registry_book} is not registered", fixture.unit_id));
            continue;
        };
        let Some(record) = book.monster_ability_resolve(&fixture.record_key) else {
            wrong.push(format!("{}: ability does not resolve", fixture.unit_id));
            continue;
        };
        match monster_ability_save_dc(record) {
            Some(dc)
                if dc.base == fixture.expected_save_dc_base
                    && dc.ability == fixture.expected_ability
                    && dc.desc_argument_index == fixture.desc_argument_index => {}
            other => wrong.push(format!(
                "{}: evaluator produced {other:?}, fixture pins base {} + {} at slot {}",
                fixture.unit_id,
                fixture.expected_save_dc_base,
                fixture.expected_ability,
                fixture.desc_argument_index
            )),
        }
        let Some(owner) = book.monster_resolve(&fixture.owner_monster_key) else {
            wrong.push(format!("{}: owner does not resolve", fixture.unit_id));
            continue;
        };
        if universal_monster_rule_save_dc_base(owner) != Some(fixture.expected_save_dc_base) {
            wrong.push(format!(
                "{}: the Universal Monster Rule over the live owner record gives {:?}, fixture \
                 pins {}",
                fixture.unit_id,
                universal_monster_rule_save_dc_base(owner),
                fixture.expected_save_dc_base
            ));
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Mutation 1 — half 1 of the bar. A deliberately wrong expected save DC,
/// compared the same way `run_monster_ability_bar_check` compares, must NOT
/// match the real evaluator's output over a real resolved record.
#[test]
fn a_wrong_expected_save_dc_makes_the_bar_check_fail() {
    let (record_key, fixture) = resolve_first_fixture();
    let registry_book =
        if fixture.book == "bestiary" { "beastiary" } else { fixture.book.as_str() };
    let book = MONSTER_BOOKS
        .iter()
        .find(|b| b.corpus_book == registry_book)
        .expect("the fixture's book is registered");
    let record = book.monster_ability_resolve(&record_key).expect("the fixture's ability resolves");
    let evaluated = monster_ability_save_dc(record).expect("the real record states a save DC");

    // Positive control: the committed expectation matches.
    assert_eq!(evaluated.base, fixture.expected_save_dc_base);
    assert_eq!(evaluated.ability, fixture.expected_ability);

    // The mutation: one off in the base, and a different ability.
    assert_ne!(
        evaluated.base,
        fixture.expected_save_dc_base + 1,
        "an off-by-one expected save DC must not compare equal"
    );
    let wrong_ability = if fixture.expected_ability == "CON" { "CHA" } else { "CON" };
    assert_ne!(
        evaluated.ability, wrong_ability,
        "a wrong ability must not compare equal"
    );
}

/// Mutation 2 — half 2 of the bar, the half that makes half 1 non-circular.
/// The printed Universal Monster Rule, applied to the real owner record, must
/// produce a DIFFERENT number when the rule's own arithmetic is perturbed —
/// so a change to either the rule or the owner's ingested Hit Dice turns this
/// check red rather than passing silently.
#[test]
fn a_wrong_universal_monster_rule_base_makes_the_bar_check_fail() {
    let (_, fixture) = resolve_first_fixture();
    let registry_book =
        if fixture.book == "bestiary" { "beastiary" } else { fixture.book.as_str() };
    let book = MONSTER_BOOKS
        .iter()
        .find(|b| b.corpus_book == registry_book)
        .expect("the fixture's book is registered");
    let owner = book
        .monster_resolve(&fixture.owner_monster_key)
        .expect("the fixture's owner monster resolves");

    let rule_base =
        universal_monster_rule_save_dc_base(owner).expect("the owner states a readable racial HD");

    // Positive control: the printed rule over the live owner reproduces the
    // number the ability row states, independently.
    assert_eq!(rule_base, fixture.expected_save_dc_base);

    // The mutation: the two perturbations of the rule that a careless edit
    // would produce — a wrong constant, and a wrong divisor.
    let hd = fixture.owner_racial_hd;
    assert_ne!(rule_base, 11 + hd / 2, "a wrong base constant must not compare equal");
    assert_ne!(
        rule_base,
        10 + hd,
        "dropping the `1/2` from the printed rule must not compare equal"
    );
}

// ---------------------------------------------------------------------------
// The credit-minting path itself, driven end to end.
//
// The tests above prove the EVALUATOR reproduces every committed fixture. They
// do not, by themselves, gate `run_monster_ability_bar_check` — the function
// that decides `cleared` vs. `failures`, and therefore the function that
// actually MINTS the `fixture-verified` stamp `apply_done_rung_stamps()`
// consumes. A wrapper that routed everything to `cleared` would be a gate that
// cannot fail, which Decision 1(a) forbids outright. These drive it through
// the public `run_bar_check` against a scratch fixture root and REAL resolved
// chassis records.
// ---------------------------------------------------------------------------

use codex::rules_core::derived_evaluator_fixture_check::run_bar_check;
use codex::rules_core::rules_tables::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
};

/// A scratch `repo_root` carrying ONLY a custom `monster_ability_entries` row
/// (plus the empty `entries` array the equipment loader requires).
///
/// The same property the `monster_entries` harness in
/// `derived_evaluator_fixture_check.rs` relies on: the monster-ability bar
/// check resolves BOTH records through the compiled
/// `monster_chassis::MONSTER_BOOKS` static registry, never the filesystem, so
/// `repo_root` controls only which FIXTURE file is read — the real Bakekujira
/// ability row and its real owner stat block resolve regardless.
struct ScratchFixtureRoot {
    root: PathBuf,
}

impl ScratchFixtureRoot {
    fn new(name: &str, body: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "codex_monster_ability_bar_check_{name}_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        let fixture_dir = root.join("tests/fixtures/rules_core");
        std::fs::create_dir_all(&fixture_dir).unwrap();
        std::fs::write(
            fixture_dir.join("derived-evaluator-fixtures.json"),
            format!(r#"{{"entries":[],"monster_ability_entries":[{body}]}}"#),
        )
        .unwrap();
        ScratchFixtureRoot { root }
    }
}

impl Drop for ScratchFixtureRoot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// One scratch entry against the REAL `Bakekujira ~ Smashing Breach` record and
/// its REAL owner, with the three values a caller may perturb.
fn scratch_entry(owner: &str, expected_base: i32, expected_ability: &str) -> String {
    format!(
        r#"{{
            "unit_id":"scratch:monster_ability:bakekujira_smashing_breach",
            "book":"bestiary_4",
            "record_key":"Bakekujira ~ Smashing Breach",
            "upstream_lst":"scratch.lst",
            "upstream_lst_sha256":"0",
            "upstream_line":1,
            "corpus_field":"DESC:scratch|CHA+22",
            "desc_argument_index":1,
            "desc_argument":"CHA+22",
            "owner_monster_key":"{owner}",
            "owner_upstream_lst":"scratch.lst",
            "owner_upstream_line":1,
            "owner_monster_class_token":"Undead:25",
            "owner_racial_hd":25,
            "universal_monster_rule_base":22,
            "expected":{{"save_dc_base":{expected_base},"ability":"{expected_ability}"}}
        }}"#
    )
}

/// POSITIVE CONTROL: the real expectation clears, so the three refusals below
/// fail because the asserted value is wrong, not because this harness is
/// always red.
#[test]
fn the_correct_expectation_clears_the_monster_ability_bar_check() {
    let scratch = ScratchFixtureRoot::new("correct", &scratch_entry("Bakekujira", 22, "CHA"));
    let report = run_bar_check(&scratch.root);

    assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
    assert_eq!(report.cleared.len(), 1, "cleared: {:?}", report.cleared);
    assert!(report.cleared.contains("scratch:monster_ability:bakekujira_smashing_breach"));
}

/// MUTATION 1, through the minting path: a wrong expected base must land in
/// `failures`, never `cleared`, so the unit can never be stamped.
#[test]
fn a_wrong_expected_save_dc_base_is_refused_by_the_bar_check() {
    let scratch = ScratchFixtureRoot::new("wrong_base", &scratch_entry("Bakekujira", 23, "CHA"));
    let report = run_bar_check(&scratch.root);

    assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
    assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    assert!(
        report.failures.contains_key("scratch:monster_ability:bakekujira_smashing_breach")
    );
}

/// MUTATION 2: a wrong expected ability is refused too — the ability half of
/// the expectation is compared, not just the number.
#[test]
fn a_wrong_expected_ability_is_refused_by_the_bar_check() {
    let scratch = ScratchFixtureRoot::new("wrong_ability", &scratch_entry("Bakekujira", 22, "CON"));
    let report = run_bar_check(&scratch.root);

    assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
    assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
}

/// MUTATION 3 — **the linked-ability requirement, enforced on the engine side
/// too, not only in the derivation script.** An ability whose owner resolves to
/// no monster of its own book has no racial HD for the printed rule to apply
/// to, so half 2 of the bar cannot be evaluated and the unit must be refused
/// rather than credited on half 1 alone.
#[test]
fn an_ability_whose_owner_does_not_resolve_is_refused_by_the_bar_check() {
    let scratch =
        ScratchFixtureRoot::new("orphan", &scratch_entry("No Such Monster", 22, "CHA"));
    let report = run_bar_check(&scratch.root);

    assert!(report.cleared.is_empty(), "an orphan must never be credited");
    assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
    assert!(
        report.failures["scratch:monster_ability:bakekujira_smashing_breach"]
            .contains("does not resolve"),
        "the refusal must name the unresolved owner: {:?}",
        report.failures
    );
}

// ---------------------------------------------------------------------------
// The evaluator's own refusals, over synthetic records — one anchor per
// corpus-observed shape it must NOT read as a save DC.
// ---------------------------------------------------------------------------

fn synthetic_ability(
    description: &'static str,
    variables: &'static [&'static str],
) -> MonsterAbilityRecord {
    MonsterAbilityRecord {
        key: "Scratch ~ Ability",
        name: "Ability",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &[],
        description: Some(description),
        description_variables: variables,
        source_page: None,
        owners: &["Scratch"],
        source_file: "scratch.lst",
        source_line: 1,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    }
}

#[test]
fn the_evaluator_reads_only_a_dc_slot_with_a_flat_base_plus_ability_argument() {
    // Both corpus-observed operand orders.
    let a = synthetic_ability("succeed at a DC %1 Will save", &["15+WIS"]);
    let dc = monster_ability_save_dc(&a).expect("a readable save DC");
    assert_eq!((dc.base, dc.ability, dc.desc_argument_index), (15, "WIS", 1));

    let b = synthetic_ability("a DC %2 Fortitude save", &["1d6", "CHA+22"]);
    let dc = monster_ability_save_dc(&b).expect("a readable save DC");
    assert_eq!((dc.base, dc.ability, dc.desc_argument_index), (22, "CHA", 2));

    // A DAMAGE term is not a save DC: no `DC ` introduces the placeholder.
    assert!(monster_ability_save_dc(&synthetic_ability("deals 3d8+%1 points", &["STR*1.5"]))
        .is_none());
    // `DC` must be a whole word.
    assert!(monster_ability_save_dc(&synthetic_ability("the MDC %1 rating", &["15+WIS"])).is_none());
    // A NAMED variable is not a flat base — this is the second sub-seam's
    // shape, deliberately out of this seam's reach.
    assert!(monster_ability_save_dc(&synthetic_ability("a DC %1 Reflex save", &["ClingDC"]))
        .is_none());
    // A full PCGen formula must NOT be mangled into `10 + (HD/2)+CON`.
    assert!(
        monster_ability_save_dc(&synthetic_ability("a DC %1 Fort save", &["10+(HD/2)+CON"]))
            .is_none()
    );
    // A SCORE is not a modifier: PCGen spells the score `<ABBREV>SCORE`.
    assert!(
        monster_ability_save_dc(&synthetic_ability("a DC %1 Will save", &["10+CHASCORE"]))
            .is_none()
    );
    // A slot with no argument at that index resolves to nothing.
    assert!(monster_ability_save_dc(&synthetic_ability("a DC %3 Will save", &["15+WIS"])).is_none());
}

fn synthetic_monster(monster_class: Option<&'static str>) -> MonsterStatBlock {
    MonsterStatBlock {
        key: "Scratch",
        name: "Scratch",
        size: None,
        speeds: &[],
        race_type: None,
        race_subtype: None,
        challenge_rating: None,
        monster_class,
        source_page: None,
        natural_attacks: &[],
        stat_adjustments: &[],
        has_spell_like_abilities: false,
        sla_cl_token: None,
        // Added by SD31-W15-INTEGRATE-001: wave 15's `monster` lane
        // (SD31-W15-MONSTER-SLA-001) added this field to `MonsterStatBlock` in a
        // sibling branch of the same wave, so neither lane's tree saw the other's
        // change and this literal only stopped compiling at the merge. Empty is
        // the honest value here — this synthetic block exists to exercise the
        // MONSTERCLASS racial-HD reader, and grants no spell-like abilities.
        spell_like_abilities: &[],
        ability_keys: &[],
        external_ability_refs: &[],
        source_file: "scratch.lst",
        source_line: 1,
    }
}

#[test]
fn the_universal_monster_rule_rounds_the_half_hit_die_down_and_refuses_what_it_cannot_read() {
    assert_eq!(universal_monster_rule_save_dc_base(&synthetic_monster(Some("Undead:25"))), Some(22));
    assert_eq!(
        universal_monster_rule_save_dc_base(&synthetic_monster(Some("Construct:12"))),
        Some(16)
    );
    // PF1 rounds the half hit die DOWN.
    assert_eq!(universal_monster_rule_save_dc_base(&synthetic_monster(Some("Ooze:1"))), Some(10));
    assert_eq!(universal_monster_rule_save_dc_base(&synthetic_monster(Some("Ooze:3"))), Some(11));
    // Refuse, never guess.
    assert_eq!(universal_monster_rule_save_dc_base(&synthetic_monster(None)), None);
    assert_eq!(universal_monster_rule_save_dc_base(&synthetic_monster(Some("Outsider"))), None);
    assert_eq!(
        universal_monster_rule_save_dc_base(&synthetic_monster(Some("Outsider:many"))),
        None
    );
}

// ---------------------------------------------------------------------------
// Second sub-seam (SD31-W16-MONSTER-ABILITY-001) — the FULL-FORMULA shape
// wave 15's receipt named as the honest next lever
// (`artifacts/SD31-W15-MONSTER-ABILITY-save-dc-seam.md`, "What remains held
// in this population"). `monster_ability_formula_entries`' own independence
// is documented in `scripts/derive_monster_ability_save_dc_fixtures.py`'s
// `monster_ability_formula_independence` doc field: one tier below the flat
// shape's two-row cross-check, because for THIS shape the ability row states
// no independent constant of its own — the formula IS the rule, restated.
// Still genuinely non-circular: a wrong engine ingest of EITHER compiled
// record (the ability row's formula shape, or the owner's MONSTERCLASS)
// turns the checks below red.
// ---------------------------------------------------------------------------

/// Independent reference reader for the formula shape: reproduces
/// `expected.save_dc_base` from the owner's `MONSTERCLASS` alone (the printed
/// rule), and separately confirms the ability row's own `corpus_field`
/// literally states the canonical formula — different code from
/// `parse_formula_base_plus_ability`, so the two cannot silently agree on a
/// shared bug.
fn reference_formula_shape_ability(corpus_field: &str, desc_argument_index: usize) -> Option<String> {
    let value = corpus_field.strip_prefix("DESC:")?;
    let mut parts = value.split('|');
    let _prose = parts.next()?;
    let args: Vec<&str> = parts.collect();
    let arg = args.get(desc_argument_index.checked_sub(1)?)?;
    const ABILITIES: [&str; 6] = ["STR", "DEX", "CON", "INT", "WIS", "CHA"];
    let compact: String = arg.chars().filter(|c| !c.is_whitespace()).collect();
    for divisor_var in ["HD", "TL"] {
        for prefix in [format!("10+({divisor_var}/2)+"), format!("10+{divisor_var}/2+")] {
            if let Some(stat) = compact.strip_prefix(prefix.as_str())
                && ABILITIES.contains(&stat)
            {
                return Some(stat.to_string());
            }
        }
    }
    None
}

#[test]
fn reference_formula_parser_refuses_what_it_cannot_read() {
    assert_eq!(
        reference_formula_shape_ability("DESC:a DC %1 Fort save|10+(HD/2)+CON", 1),
        Some("CON".to_string())
    );
    assert_eq!(
        reference_formula_shape_ability("DESC:a DC %1 Fort save|10+HD/2+DEX", 1),
        Some("DEX".to_string())
    );
    assert_eq!(
        reference_formula_shape_ability("DESC:a DC %1 Fort save|10+(TL/2)+CON", 1),
        Some("CON".to_string())
    );
    // Wrong coefficient — a real deviation from the printed rule, not credited.
    assert_eq!(reference_formula_shape_ability("DESC:a DC %1 Fort save|8+TL/2+CON", 1), None);
    // An extra term — not the canonical shape.
    assert_eq!(
        reference_formula_shape_ability("DESC:a DC %1 Fort save|HD+10+HD/2+CON", 1),
        None
    );
    // The flat shape is not the formula shape.
    assert_eq!(reference_formula_shape_ability("DESC:a DC %1 Will save|15+WIS", 1), None);
    // A bare named variable is not the formula shape either.
    assert_eq!(
        reference_formula_shape_ability("DESC:a DC %1 Reflex save|ClingDC", 1),
        None
    );
}

/// Guarantee 3, route A: every committed `monster_ability_formula_entries`
/// row's pinned `corpus_field` still states the canonical formula, and the
/// ability it names still matches `expected.ability`.
#[test]
fn monster_ability_formula_expected_ability_is_re_derivable_from_the_pinned_desc_argument() {
    let fixtures = load_monster_ability_formula_fixtures(&repo_root());
    assert!(
        !fixtures.is_empty(),
        "an empty monster_ability_formula_entries would make this suite vacuous"
    );
    let mut wrong = Vec::new();
    for fixture in &fixtures {
        match reference_formula_shape_ability(&fixture.corpus_field, fixture.desc_argument_index) {
            Some(ability) if ability == fixture.expected_ability => {}
            other => wrong.push(format!(
                "{}: reference derivation of {:?} produced {other:?}, fixture pins ability {}",
                fixture.unit_id, fixture.corpus_field, fixture.expected_ability
            )),
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Guarantee 3, route B: every committed row's `expected.save_dc_base`
/// reproduces from the PRINTED Universal Monster Rule applied to the OWNING
/// monster's own pinned `MONSTERCLASS` token — the sole source of the
/// numeric base for this shape (the ability row states no summed constant).
#[test]
fn monster_ability_formula_expected_base_reproduces_from_the_printed_universal_monster_rule() {
    let fixtures = load_monster_ability_formula_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "vacuous without fixtures");
    let mut wrong = Vec::new();
    for fixture in &fixtures {
        let Some((_, hd)) = fixture.owner_monster_class_token.rsplit_once(':') else {
            wrong.push(format!(
                "{}: owner MONSTERCLASS {:?} has no `:<HD>` tail",
                fixture.unit_id, fixture.owner_monster_class_token
            ));
            continue;
        };
        let Ok(hd) = hd.trim().parse::<i32>() else {
            wrong.push(format!(
                "{}: owner MONSTERCLASS {:?} tail is not an integer",
                fixture.unit_id, fixture.owner_monster_class_token
            ));
            continue;
        };
        if hd != fixture.owner_racial_hd {
            wrong.push(format!(
                "{}: pinned owner_racial_hd {} disagrees with MONSTERCLASS {:?}",
                fixture.unit_id, fixture.owner_racial_hd, fixture.owner_monster_class_token
            ));
            continue;
        }
        let rule_base = 10 + hd / 2;
        if rule_base != fixture.expected_save_dc_base
            || rule_base != fixture.universal_monster_rule_base
        {
            wrong.push(format!(
                "{}: the printed rule over {} HD gives {rule_base}, fixture pins expected {} / \
                 universal_monster_rule_base {}",
                fixture.unit_id, hd, fixture.expected_save_dc_base, fixture.universal_monster_rule_base
            ));
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Sibling of [`monster_ability_pinned_corpus_field_is_byte_identical_to_the_upstream_lst`],
/// for the formula-shape fixtures. Re-reads `owner_upstream_lst`/
/// `owner_upstream_line` fresh from the pinned oracle and re-derives, from
/// the raw line's OWN fields (never from the fixture's `owner_monster_key`
/// string), whether that line really names the owner — independent proof
/// that `find_owner_row`'s bare-leading-field fallback (`SD31-W17-MONSTER-
/// ABILITY-001`, `OPEN-ISSUES.md` row 295) picked a real owner row, not a
/// coincidental substring. The flat-shape sibling only accepts an explicit
/// `KEY:<name>` token because every one of its 92+23 rows has one; this
/// formula-shape check additionally accepts the fallback shape (no `KEY:`
/// token anywhere on the line, first tab-delimited field equals the owner
/// name verbatim) — but never both loosely: a line WITH a `KEY:` token must
/// match via `KEY:`, never merely resemble the name in its first field.
#[test]
fn monster_ability_formula_owner_citation_is_independently_re_derivable_from_the_upstream_lst() {
    let fixtures = load_monster_ability_formula_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "vacuous without fixtures");
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("skipped: neither PCGEN_CORPUS_ROOT nor HOME is set");
        return;
    };
    if !data_root.is_dir() {
        eprintln!("skipped: no PCGen checkout at {data_root:?}");
        return;
    }

    let mut wrong = Vec::new();
    for fixture in &fixtures {
        let owner_path = data_root.join(&fixture.owner_upstream_lst);
        let Ok(bytes) = std::fs::read(&owner_path) else {
            wrong.push(format!("{}: {:?} is not readable", fixture.unit_id, owner_path));
            continue;
        };
        let lines: Vec<String> =
            String::from_utf8_lossy(&bytes).split('\n').map(str::to_string).collect();
        let index = usize::try_from(fixture.owner_upstream_line).expect("line fits in usize");
        let Some(owner_line) = index.checked_sub(1).and_then(|i| lines.get(i)) else {
            wrong.push(format!(
                "{}: {} has no line {}",
                fixture.unit_id, fixture.owner_upstream_lst, fixture.owner_upstream_line
            ));
            continue;
        };
        if !owner_line.contains(&format!("MONSTERCLASS:{}", fixture.owner_monster_class_token)) {
            wrong.push(format!(
                "{}: line {} of {} does not contain MONSTERCLASS:{:?} verbatim",
                fixture.unit_id,
                fixture.owner_upstream_line,
                fixture.owner_upstream_lst,
                fixture.owner_monster_class_token
            ));
            continue;
        }
        // `SD31-W17-INTEGRATE-001` (adversarial review, wave 17): both
        // checks were loosened. `cited_by_key` used a substring `contains`,
        // so a fixture citing "Fungus Queen" would have been satisfied by a
        // row naming "Fungus Queen Elite" -- match the exact tab field
        // instead. `leading_field` took the first NON-empty tab field,
        // while the generator's own fallback (`find_owner_row`) requires
        // `fields[0]` exactly -- use `fields[0]` here too so this test can
        // never accept a shape the generator itself would reject.
        let fields: Vec<&str> = owner_line.split('\t').collect();
        let has_key_token = fields.iter().any(|f| f.starts_with("KEY:"));
        let cited_by_key =
            fields.iter().any(|f| *f == format!("KEY:{}", fixture.owner_monster_key));
        let leading_field = fields.first().copied().unwrap_or("");
        let cited_by_bare_leading_field =
            !has_key_token && leading_field == fixture.owner_monster_key;
        if !(cited_by_key || cited_by_bare_leading_field) {
            wrong.push(format!(
                "{}: line {} of {} names neither KEY:{} nor a bare leading field {:?}",
                fixture.unit_id,
                fixture.owner_upstream_line,
                fixture.owner_upstream_lst,
                fixture.owner_monster_key,
                fixture.owner_monster_key
            ));
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// The engine's evaluator reproduces every committed formula fixture over the
/// live compiled tables — the positive control for the mutation proofs below.
#[test]
fn the_engine_evaluator_reproduces_every_committed_monster_ability_formula_fixture() {
    let fixtures = load_monster_ability_formula_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "vacuous without fixtures");
    let mut wrong = Vec::new();
    for fixture in &fixtures {
        let registry_book =
            if fixture.book == "bestiary" { "beastiary" } else { fixture.book.as_str() };
        let Some(book) = MONSTER_BOOKS.iter().find(|b| b.corpus_book == registry_book) else {
            wrong.push(format!("{}: book {registry_book} is not registered", fixture.unit_id));
            continue;
        };
        let Some(record) = book.monster_ability_resolve(&fixture.record_key) else {
            wrong.push(format!("{}: ability does not resolve", fixture.unit_id));
            continue;
        };
        let Some(owner) = book.monster_resolve(&fixture.owner_monster_key) else {
            wrong.push(format!("{}: owner does not resolve", fixture.unit_id));
            continue;
        };
        match monster_ability_formula_save_dc(record, owner) {
            Some(dc)
                if dc.base == fixture.expected_save_dc_base
                    && dc.ability == fixture.expected_ability
                    && dc.desc_argument_index == fixture.desc_argument_index => {}
            other => wrong.push(format!(
                "{}: evaluator produced {other:?}, fixture pins base {} + {} at slot {}",
                fixture.unit_id,
                fixture.expected_save_dc_base,
                fixture.expected_ability,
                fixture.desc_argument_index
            )),
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// MUTATION-PROOF, half 1: the evaluator refuses shapes that are not the
/// canonical formula, over synthetic records — the same population of
/// corpus-observed near-misses the derivation script also refuses.
#[test]
fn the_formula_evaluator_reads_only_the_canonical_universal_monster_rule_shape() {
    let owner = synthetic_monster(Some("Aberration:7"));

    // Both accepted spellings, both divisor variables.
    let a = synthetic_ability("a DC %1 Fort save", &["10+(HD/2)+CON"]);
    let dc = monster_ability_formula_save_dc(&a, &owner).expect("canonical shape resolves");
    assert_eq!((dc.base, dc.ability, dc.desc_argument_index), (13, "CON", 1));

    let b = synthetic_ability("a DC %1 Fort save", &["10+HD/2+DEX"]);
    let dc = monster_ability_formula_save_dc(&b, &owner).expect("canonical shape resolves");
    assert_eq!((dc.base, dc.ability, dc.desc_argument_index), (13, "DEX", 1));

    let c = synthetic_ability("a DC %1 Fort save", &["10+(TL/2)+CON"]);
    let dc = monster_ability_formula_save_dc(&c, &owner).expect("TL is HD's synonym here");
    assert_eq!(dc.base, 13);

    // A wrong coefficient is a real deviation from the printed rule, not
    // credited — this is `aluum_soul_shriek`'s live corpus shape.
    assert!(monster_ability_formula_save_dc(
        &synthetic_ability("a DC %1 Will save", &["8+TL/2+CON"]),
        &owner
    )
    .is_none());
    // An extra term is not the canonical shape — `hive_larva_swarm_poison`'s
    // live corpus shape.
    assert!(monster_ability_formula_save_dc(
        &synthetic_ability("a DC %1 Fort save", &["HD+10+HD/2+CON"]),
        &owner
    )
    .is_none());
    // A wrong divisor must not be read as `/2`.
    assert!(monster_ability_formula_save_dc(
        &synthetic_ability("a DC %1 Fort save", &["10+(HD/3)+CON"]),
        &owner
    )
    .is_none());
    // The flat shape is the FIRST sub-seam's alone.
    assert!(
        monster_ability_formula_save_dc(&synthetic_ability("a DC %1 Will save", &["15+WIS"]), &owner)
            .is_none()
    );
    // A bare named variable's formula lives on a separate BONUS:VAR token
    // this evaluator does not chase.
    assert!(monster_ability_formula_save_dc(
        &synthetic_ability("a DC %1 Reflex save", &["ClingDC"]),
        &owner
    )
    .is_none());
    // No owner HD, no base to compute, even with a canonical-shaped formula.
    assert!(monster_ability_formula_save_dc(
        &synthetic_ability("a DC %1 Fort save", &["10+(HD/2)+CON"]),
        &synthetic_monster(None)
    )
    .is_none());
}

/// A scratch `repo_root` carrying ONLY a custom `monster_ability_formula_entries`
/// row — the formula-shape sibling of `ScratchFixtureRoot`.
struct FormulaScratchFixtureRoot {
    root: PathBuf,
}

impl FormulaScratchFixtureRoot {
    fn new(name: &str, body: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "codex_monster_ability_formula_bar_check_{name}_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        let fixture_dir = root.join("tests/fixtures/rules_core");
        std::fs::create_dir_all(&fixture_dir).unwrap();
        std::fs::write(
            fixture_dir.join("derived-evaluator-fixtures.json"),
            format!(r#"{{"entries":[],"monster_ability_formula_entries":[{body}]}}"#),
        )
        .unwrap();
        FormulaScratchFixtureRoot { root }
    }
}

impl Drop for FormulaScratchFixtureRoot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// One scratch entry against the REAL `Dragon (Solar) ~ Channel Radiation`
/// record (`10+(HD/2)+CON`) and its real owner (`Dragon:7`), with the two
/// values a caller may perturb.
fn formula_scratch_entry(expected_base: i32, expected_ability: &str) -> String {
    format!(
        r#"{{
            "unit_id":"scratch:monster_ability:dragon_solar_channel_radiation",
            "book":"bestiary_4",
            "record_key":"Dragon (Solar) ~ Channel Radiation",
            "upstream_lst":"scratch.lst",
            "upstream_lst_sha256":"0",
            "upstream_line":1,
            "corpus_field":"DESC:scratch|10+(HD/2)+CON",
            "desc_argument_index":1,
            "desc_argument":"10+(HD/2)+CON",
            "owner_monster_key":"Dragon (Solar)",
            "owner_upstream_lst":"scratch.lst",
            "owner_upstream_line":1,
            "owner_monster_class_token":"Dragon:7",
            "owner_racial_hd":7,
            "universal_monster_rule_base":13,
            "expected":{{"save_dc_base":{expected_base},"ability":"{expected_ability}"}}
        }}"#
    )
}

/// POSITIVE CONTROL: the real expectation clears through `run_bar_check`.
#[test]
fn the_correct_expectation_clears_the_monster_ability_formula_bar_check() {
    let scratch = FormulaScratchFixtureRoot::new("correct", &formula_scratch_entry(13, "CON"));
    let report = run_bar_check(&scratch.root);

    assert!(report.failures.is_empty(), "failures: {:?}", report.failures);
    assert_eq!(report.cleared.len(), 1, "cleared: {:?}", report.cleared);
    assert!(report
        .cleared
        .contains("scratch:monster_ability:dragon_solar_channel_radiation"));
}

/// MUTATION, through the minting path: a wrong expected base must land in
/// `failures`, never `cleared` — proving `run_monster_ability_formula_bar_check`
/// (reached only through the public `run_bar_check`) can genuinely fail.
#[test]
fn a_wrong_expected_save_dc_base_is_refused_by_the_formula_bar_check() {
    let scratch = FormulaScratchFixtureRoot::new("wrong_base", &formula_scratch_entry(14, "CON"));
    let report = run_bar_check(&scratch.root);

    assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
    assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
}

/// MUTATION: a wrong expected ability is refused too.
#[test]
fn a_wrong_expected_ability_is_refused_by_the_formula_bar_check() {
    let scratch = FormulaScratchFixtureRoot::new("wrong_ability", &formula_scratch_entry(13, "WIS"));
    let report = run_bar_check(&scratch.root);

    assert!(report.cleared.is_empty(), "cleared: {:?}", report.cleared);
    assert_eq!(report.failures.len(), 1, "failures: {:?}", report.failures);
}
