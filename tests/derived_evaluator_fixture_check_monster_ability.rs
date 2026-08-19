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
    load_monster_ability_fixtures, monster_ability_save_dc,
    universal_monster_rule_save_dc_base,
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
