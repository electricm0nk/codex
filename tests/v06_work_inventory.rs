//! Standing regression coverage for `src/bin/v06_work_inventory.rs`.
//!
//! **What these tests are for.** The work inventory's whole value is that it is
//! trustworthy *unread*: the dashboard renders it and a multi-day run is
//! monitored from it, so nobody is eyeballing a diff to catch a bad entry. Two
//! classes of failure would destroy that, and both are pinned here:
//!
//! 1. **A corpus trap re-opening.** Every rule in the generator's `TRAP_RULES`
//!    was validated against a figure this codebase independently documents and
//!    defends (`crb::feats`' 185, `crb::spell_list`'s 652,
//!    `crb::equipment_tables`' 310/453/1556). Those figures are asserted here
//!    against the *same corpus reading the generator does*, so a change that
//!    silently reintroduces `.MOD` records, `#`-disabled duplicates or
//!    per-level class rows fails a test instead of inflating a number.
//! 2. **Non-idempotence.** Two consecutive runs must differ only in
//!    `generated_at`.
//!
//! These tests read the real corpus when it is reachable and **skip cleanly
//! when it is not** — the corpus is a sibling checkout, not a repo asset, so a
//! CI box without it must not go red. The skip prints, so a silently-skipping
//! suite is visible in the output.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The corpus books directory, or `None` when this machine has no PCGen
/// checkout to read.
fn corpus_books_dir() -> Option<PathBuf> {
    let root = match std::env::var_os("PCGEN_CORPUS_ROOT") {
        Some(configured) => PathBuf::from(configured),
        // HOME-relative default: the operator keeps `workspace/` in the home
        // directory and syncs it between machines. Rust does not expand `~`.
        None => PathBuf::from(std::env::var_os("HOME").filter(|home| !home.is_empty())?)
            .join("workspace/repos/pcgen/data"),
    };
    let dir = root.join("pathfinder/paizo/roleplaying_game");
    dir.is_dir().then_some(dir)
}

macro_rules! corpus_or_skip {
    () => {
        match corpus_books_dir() {
            Some(dir) => dir,
            None => {
                println!("SKIP: no PCGen corpus at PCGEN_CORPUS_ROOT");
                return;
            }
        }
    };
}

/// The generator's own record-shape rules, re-implemented here at the exact
/// granularity each assertion needs. Deliberately a *second* implementation
/// rather than a call into the binary: a test that reuses the code under test
/// cannot catch that code drifting away from the corpus figures it claims to
/// reproduce.
struct Row<'a> {
    fields: Vec<&'a str>,
}

impl<'a> Row<'a> {
    fn first(&self) -> &'a str {
        self.fields.first().copied().unwrap_or("").trim()
    }
    fn has(&self, prefix: &str) -> bool {
        self.fields.iter().any(|f| f.starts_with(prefix))
    }
    fn value(&self, prefix: &str) -> Option<&'a str> {
        self.fields
            .iter()
            .find(|f| f.starts_with(prefix))
            .map(|f| &f[prefix.len()..])
    }
}

fn rows(text: &str) -> Vec<Row<'_>> {
    text.lines()
        .map(|l| Row { fields: l.trim_end_matches(['\n', '\r']).split('\t').collect() })
        .collect()
}

/// Every row that survives the generator's universal filters: not a comment or
/// `#`-disabled duplicate, not an ALL-CAPS directive line, not a `.MOD`
/// modifier, not the `CATEGORY=Internal|` namespace.
fn base_rows<'a>(text: &'a str) -> Vec<Row<'a>> {
    rows(text)
        .into_iter()
        .filter(|r| {
            let f = r.first();
            if f.is_empty() || f.starts_with('#') || f.starts_with("CATEGORY=Internal|") {
                return false;
            }
            if f.contains(".MOD") {
                return false;
            }
            let is_directive = f
                .split_once(':')
                .map(|(head, _)| {
                    !head.is_empty()
                        && head.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
                })
                .unwrap_or(false);
            !is_directive || f.starts_with("CLASS:")
        })
        .collect()
}

/// A row's identity: its `KEY:` token when present, else its display name with
/// any `<Base>.COPY=` prefix resolved to the variant it declares.
fn identity<'a>(row: &Row<'a>) -> String {
    if let Some(key) = row.value("KEY:") {
        return key.to_string();
    }
    let f = row.first();
    match f.split_once(".COPY=") {
        Some((_, variant)) => variant.to_string(),
        None => f.to_string(),
    }
}

fn read(dir: &Path, book: &str, file: &str) -> String {
    std::fs::read_to_string(dir.join(book).join(file))
        .unwrap_or_else(|e| panic!("corpus file {book}/{file} must be readable: {e}"))
}

fn distinct(mut ids: Vec<String>) -> usize {
    ids.sort();
    ids.dedup();
    ids.len()
}

// ---------------------------------------------------------------------------
// Trap rules
// ---------------------------------------------------------------------------

/// `missing_classifying_token` + `mod_record` + `comment_or_disabled`.
///
/// `crb::feats`' own module doc defends 185 as the full, real CRB feat count:
/// every `cr_feats.lst` record carrying an explicit `TYPE:` facet, with 12
/// records deliberately excluded (9 `VISIBLE:NO` PCGen helpers and a
/// item-granted variant that carry no `TYPE:` token at all, plus 2 `.MOD`
/// lines). If the generator's rules drift, this number moves.
#[test]
fn crb_feat_rules_reproduce_the_documented_185() {
    let dir = corpus_or_skip!();
    let text = read(&dir, "core_rulebook", "cr_feats.lst");
    let ids: Vec<String> = base_rows(&text)
        .iter()
        .filter(|r| !r.first().contains(".COPY=") && r.has("TYPE:"))
        .map(identity)
        .collect();
    assert_eq!(
        distinct(ids),
        185,
        "CRB feat enumeration must reproduce the count `crb::feats` documents \
         and `feats_all::all_feat_tables` ships"
    );
    assert_eq!(
        codex::rules_core::rules_tables::crb::feats::feat_tables().len(),
        185,
        "the engine's own CRB feat table must still hold the same 185 records"
    );
}

/// A `TYPE:`-less row is a sub-choice helper, not a feat. Pinned as its own
/// assertion because it is most of what separates a naive 195 from the real
/// 185: 9 `TYPE:`-less rows, then one duplicate identity.
#[test]
fn feat_rows_without_a_type_facet_are_not_feats() {
    let dir = corpus_or_skip!();
    let text = read(&dir, "core_rulebook", "cr_feats.lst");
    let all = base_rows(&text).len();
    let untyped: Vec<String> = base_rows(&text)
        .iter()
        .filter(|r| !r.has("TYPE:"))
        .map(|r| r.first().to_string())
        .collect();
    assert_eq!(
        untyped.len(),
        9,
        "exactly nine CRB rows carry no `TYPE:` facet -- the 8 `Power Attack \
         (...)` PCGen helpers and the item-granted `Cleave (Granted by Sylvan \
         Scimitar)` variant. Saw: {untyped:?}"
    );
    assert!(
        untyped.iter().all(|n| n.starts_with("Power Attack (") || n.starts_with("Cleave (")),
        "and every one of them is one of those two shapes: {untyped:?}"
    );
    assert_eq!(
        all, 195,
        "195 rows survive the universal filters; 9 lose the TYPE: gate and one \
         more collapses under duplicate_identity, landing on the documented 185"
    );
}

/// `missing_classifying_token` for spells, and the `.COPY=` merge that takes
/// `cr_spells.lst` from 674 raw rows to `crb::spell_list`'s documented 652.
#[test]
fn crb_spell_rules_reproduce_the_documented_652() {
    let dir = corpus_or_skip!();
    let text = read(&dir, "core_rulebook", "cr_spells.lst");
    let ids: Vec<String> = base_rows(&text)
        .iter()
        .filter(|r| {
            !r.first().contains(".COPY=") && (r.has("SCHOOL:") || r.has("CLASSES:"))
        })
        .map(identity)
        .collect();
    assert_eq!(
        distinct(ids),
        652,
        "CRB spell enumeration must reproduce `crb::spell_list`'s documented count"
    );
    assert_eq!(
        codex::rules_core::rules_tables::crb::spell_list::SPELL_LIST.len(),
        652
    );
}

/// `copy_record`: a `.COPY=` row declares a NEW named record. Keeping them is
/// what makes the equipment counts land exactly on the engine's own per-file
/// figures; dropping them is what makes the spell count land on 652. Both
/// behaviours are load-bearing, so both are pinned.
#[test]
fn copy_rows_are_kept_for_equipment_and_reproduce_the_engine_per_file_counts() {
    let dir = corpus_or_skip!();
    for (file, expected) in [
        ("cr_equip_arms_armor.lst", 310usize),
        ("cr_equip_general.lst", 453),
        ("cr_equip_magic_items.lst", 1556),
    ] {
        let text = read(&dir, "core_rulebook", file);
        let ids: Vec<String> = base_rows(&text).iter().map(identity).collect();
        assert_eq!(
            distinct(ids),
            expected,
            "{file} must reproduce the per-file record count \
             `crb::equipment_tables::EquipmentFieldCoverage` documents"
        );
    }
}

/// `mod_only_rescue`. All seven playable CRB races are declared ONLY as `.MOD`
/// rows in `cr_races.lst`; their bases live in the shared `core_essentials`
/// library that `core_rulebook.pcc` pulls in. A generator that drops every
/// `.MOD` reports the Core Rulebook as having zero races.
#[test]
fn crb_races_exist_only_as_mod_rows_over_a_shared_library_base() {
    let dir = corpus_or_skip!();
    let text = read(&dir, "core_rulebook", "cr_races.lst");
    assert_eq!(
        base_rows(&text).len(),
        0,
        "`cr_races.lst` declares no base race record at all -- if this ever \
         becomes non-zero the mod_only_rescue rule needs revisiting"
    );
    let mod_rows = rows(&text)
        .iter()
        .filter(|r| !r.first().starts_with('#') && r.first().contains(".MOD"))
        .count();
    assert_eq!(mod_rows, 7, "the seven CRB races are seven `.MOD` rows");
    assert_eq!(
        codex::rules_core::rules_tables::crb::race_tables::RaceId::ALL.len(),
        7,
        "and the engine models exactly those seven"
    );
}

/// The include edge that makes the rescue attributable rather than a guess:
/// `core_rulebook.pcc` really does pull `core_essentials` in.
#[test]
fn core_rulebook_pcc_really_includes_the_shared_core_essentials_library() {
    let dir = corpus_or_skip!();
    let pcc = read(&dir, "core_rulebook", "core_rulebook.pcc");
    let includes_library = pcc.lines().any(|l| {
        l.trim().starts_with("PCC:")
            && l.replace('\\', "/").contains("roleplaying_game/core_essentials/")
    });
    assert!(
        includes_library,
        "the shared-library attribution rests on this `PCC:` edge existing"
    );
}

/// `class_level_line`. Only `CLASS:`-prefixed rows declare a class; the rest of
/// a `*_classes.lst` is per-level progression. Counting every row reports 368
/// CRB classes where 28 exist.
#[test]
fn only_class_prefixed_rows_declare_a_class() {
    let dir = corpus_or_skip!();
    let text = read(&dir, "core_rulebook", "cr_classes.lst");
    let all = base_rows(&text).len();
    let declared: Vec<String> = base_rows(&text)
        .iter()
        .filter(|r| r.first().starts_with("CLASS:"))
        .map(|r| r.first().trim_start_matches("CLASS:").to_string())
        .collect();
    assert_eq!(
        distinct(declared),
        28,
        "CRB declares 28 classes (11 core + 5 NPC + 10 prestige + 2 Ex- variants)"
    );
    assert!(
        all > 300,
        "and it does so across {all} rows -- the level-line trap is real"
    );
}

/// `duplicate_identity`. Archetype-qualified `KEY:` tokens differ from their
/// display name; keying on the display name collided 18 spells across books.
#[test]
fn archetype_qualified_keys_are_distinct_identities() {
    let dir = corpus_or_skip!();
    let apg = read(&dir, "advanced_players_guide", "apg_spells.lst");
    let qualified: Vec<&str> = base_rows(&apg)
        .iter()
        .filter_map(|r| r.value("KEY:"))
        .filter(|k| k.starts_with("Summoner "))
        .collect();
    assert_eq!(
        qualified.len(),
        9,
        "the nine Summoner-archetype summon spells carry an archetype-qualified KEY"
    );
    for key in &qualified {
        assert!(
            !key.is_empty() && key.contains("Summon"),
            "and each is a real, differently-named record: {key}"
        );
    }
    let acg = read(&dir, "advanced_class_guide", "acg_spells.lst");
    let acg_qualified = base_rows(&acg)
        .iter()
        .filter_map(|r| r.value("KEY:"))
        .filter(|k| k.starts_with("Naturalist "))
        .count();
    assert_eq!(acg_qualified, 9, "and ACG has its own nine Naturalist rows");
}

/// `token_not_record`. One record can carry many tokens; the inventory counts
/// records. Pinned against a real corpus row rather than a remembered example.
#[test]
fn a_single_record_can_carry_many_bonus_tokens() {
    let dir = corpus_or_skip!();
    let text = read(&dir, "core_rulebook", "cr_feats.lst");
    let max_bonus_tokens = base_rows(&text)
        .iter()
        .map(|r| r.fields.iter().filter(|f| f.starts_with("BONUS:")).count())
        .max()
        .unwrap_or(0);
    assert!(
        max_bonus_tokens > 1,
        "at least one CRB feat record carries multiple BONUS tokens, so a \
         token-counting enumerator would over-report"
    );
}

/// `book_scoped_count`. The hexes figure that was reported as 27 is APG-only;
/// the corpus carries far more across the other books. Asserted structurally:
/// more than one book declares hex records, so any single-book figure must not
/// be presented as corpus-wide.
#[test]
fn hex_records_span_more_than_one_book() {
    let dir = corpus_or_skip!();
    let mut books_with_hexes = 0;
    for book in ["advanced_players_guide", "advanced_class_guide", "ultimate_magic"] {
        let path = dir.join(book);
        let Ok(entries) = std::fs::read_dir(&path) else { continue };
        let hit = entries.flatten().any(|e| {
            let p = e.path();
            p.extension().and_then(|x| x.to_str()) == Some("lst")
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.contains("_abilities_class"))
                    .unwrap_or(false)
                && std::fs::read_to_string(&p)
                    .map(|t| t.contains("Hex ~") || t.contains("CATEGORY:Hex"))
                    .unwrap_or(false)
        });
        if hit {
            books_with_hexes += 1;
        }
    }
    assert!(
        books_with_hexes > 1,
        "hexes are declared in more than one book, so a per-book count is never \
         the corpus-wide count"
    );
}

/// `race_favored_class_bonus_row` + `race_choice_suboption_row` (SD28-E16,
/// `decisions.md §35`). `_abilities_race.lst` is classified whole-file as
/// `Kind::RaceTrait`, but carries Favored Class Bonus rows (a different
/// mechanic; `race_trait_ids` can never hold an FCB identity) and
/// `CATEGORY:Choice` sub-option rows (menu options of an already-counted
/// parent trait) that must not be double-counted as independent racial
/// traits. Pinned against `arg_abilities_race.lst` directly, independent of
/// the generator under test, per this file's own re-implementation
/// philosophy.
#[test]
fn arg_race_file_carries_favored_class_bonus_and_choice_suboption_rows_not_traits() {
    let dir = corpus_or_skip!();
    let text = read(&dir, "advanced_race_guide", "arg_abilities_race.lst");
    let rows = base_rows(&text);

    let is_fcb = |r: &&Row| {
        r.fields
            .iter()
            .any(|f| f.starts_with("TYPE:") && f[5..].split('.').any(|c| c == "FavoredClassBonus"))
    };
    let fcb_count = rows.iter().filter(is_fcb).count();
    assert_eq!(
        fcb_count, 291,
        "arg_abilities_race.lst's Favored Class Bonus rows (TYPE:...FavoredClassBonus...) -- \
         a different mechanic from a racial trait, one row per race x class"
    );

    let choice_count = rows.iter().filter(|r| !is_fcb(r) && r.fields.contains(&"CATEGORY:Choice")).count();
    assert_eq!(
        choice_count, 82,
        "arg_abilities_race.lst's CATEGORY:Choice sub-option rows -- CHOOSE: menu options of an \
         already-counted parent trait (e.g. Elf ~ Elemental Resistance's Acid/Cold/Electricity/Fire \
         choices), never independent racial traits"
    );

    // The already-shipped ARG corpus never uses CATEGORY:Choice for a real
    // trait -- the discriminator's other side, so a future trait added with
    // CATEGORY:Choice would be a real regression this test would catch.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let trait_root = manifest.join("data/corpus/advanced_race_guide/race_trait");
    let mut checked = 0usize;
    let mut race_dirs: Vec<PathBuf> =
        std::fs::read_dir(&trait_root).expect("race_trait dir must exist").filter_map(Result::ok).map(|e| e.path()).collect();
    race_dirs.sort();
    for race_dir in race_dirs {
        let mut files: Vec<PathBuf> =
            std::fs::read_dir(&race_dir).unwrap().filter_map(Result::ok).map(|e| e.path()).collect();
        files.sort();
        for path in files {
            let text = std::fs::read_to_string(&path).unwrap();
            let value: serde_json::Value = serde_json::from_str(&text).unwrap();
            checked += 1;
            assert_eq!(
                value["data"]["category"], "Special Ability",
                "{path:?}: every already-ingested ARG alternate racial trait carries \
                 CATEGORY:Special Ability, never CATEGORY:Choice"
            );
        }
    }
    assert_eq!(checked, 156, "156 already-ingested ARG alternate racial trait records");
}

// ---------------------------------------------------------------------------
// Generator contract
// ---------------------------------------------------------------------------

fn run_generator() -> Option<String> {
    corpus_books_dir()?;
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output = Command::new(env!("CARGO"))
        .args(["run", "--quiet", "--bin", "v06_work_inventory", "--", "--stdout-only"])
        .current_dir(&manifest)
        .output()
        .expect("cargo must be able to run the generator");
    assert!(
        output.status.success(),
        "generator exited {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    Some(String::from_utf8(output.stdout).expect("generator emits UTF-8"))
}

/// Strip the one field that is *supposed* to change between runs.
fn without_timestamp(json: &str) -> String {
    json.lines()
        .filter(|l| !l.trim_start().starts_with("\"generated_at\""))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Idempotence: two consecutive runs produce byte-identical output apart from
/// the timestamp. This is the property that lets the file be committed and
/// diffed — without it every regeneration would be noise and a real change
/// would be invisible inside it.
#[test]
#[ignore = "runs the generator twice; ~2 minutes. Run explicitly: cargo test --test v06_work_inventory -- --ignored"]
fn two_consecutive_runs_differ_only_in_the_timestamp() {
    let Some(first) = run_generator() else {
        println!("SKIP: no PCGen corpus at PCGEN_CORPUS_ROOT");
        return;
    };
    let second = run_generator().expect("corpus was present a moment ago");
    assert_eq!(
        without_timestamp(&first),
        without_timestamp(&second),
        "the inventory must be idempotent over an unchanged corpus and engine"
    );
    assert!(
        first.contains("\"generated_at\""),
        "and it must still stamp when it ran"
    );
}

/// The committed artefact must parse, carry the documented shape, and never
/// contain a status outside the declared vocabulary — a fabricated status word
/// would be exactly the kind of confident-wrong entry this inventory exists to
/// prevent.
#[test]
fn the_committed_inventory_is_well_formed_and_uses_only_declared_statuses() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/work-inventory.json");
    let Ok(text) = std::fs::read_to_string(&path) else {
        println!("SKIP: docs/work-inventory.json has not been generated yet");
        return;
    };
    let doc: serde_json::Value = serde_json::from_str(&text).expect("inventory is valid JSON");

    let vocabulary: Vec<String> = doc["status_vocabulary"]
        .as_object()
        .expect("status_vocabulary is an object")
        .keys()
        .cloned()
        .collect();
    assert!(
        ["grounded", "text-complete", "deferred-with-reason", "not-started"]
            .iter()
            .all(|s| vocabulary.iter().any(|v| v == s)),
        "the four statuses the brief requires must all be declared"
    );

    let units = doc["units"].as_array().expect("units is an array");
    assert!(!units.is_empty(), "the inventory must enumerate real units");

    let mut ids = std::collections::HashSet::new();
    for unit in units {
        let status = unit["status"].as_str().expect("every unit has a status");
        assert!(
            vocabulary.iter().any(|v| v == status),
            "unit {} carries undeclared status {status}",
            unit["id"]
        );
        // An honest unknown always says why.
        if status == "unknown" {
            assert!(
                unit["reason"].is_string(),
                "unit {} is unknown with no reason",
                unit["id"]
            );
        }
        // A deferral always quotes the engine rather than re-narrating it.
        if status == "deferred-with-reason" {
            assert!(
                unit["reason"].is_string() && !unit["reason"].as_str().unwrap().is_empty(),
                "unit {} is deferred with no verbatim engine text",
                unit["id"]
            );
            assert!(
                unit["evidence"]
                    .as_str()
                    .unwrap_or_default()
                    .starts_with("engine_diagnostic:"),
                "unit {} defers without naming the diagnostic it quotes",
                unit["id"]
            );
        }
        assert!(
            unit["evidence"].as_str().map(|e| !e.is_empty()).unwrap_or(false),
            "unit {} has no evidence for its status",
            unit["id"]
        );
        ids.insert(unit["id"].as_str().unwrap_or_default().to_string());
    }

    // Totals must be derivable from the units, not asserted independently of
    // them -- a totals block that disagrees with its own rows is precisely the
    // drift this document exists to make impossible.
    assert_eq!(
        doc["totals"]["units"].as_u64().unwrap_or(0) as usize,
        units.len(),
        "totals.units must equal the number of emitted units"
    );
    let by_status = doc["totals"]["by_status"].as_object().expect("by_status object");
    let summed: u64 = by_status.values().filter_map(|v| v.as_u64()).sum();
    assert_eq!(
        summed,
        units.len() as u64,
        "totals.by_status must partition the units exactly"
    );
    let by_kind = doc["totals"]["by_kind"].as_object().expect("by_kind object");
    let summed_kinds: u64 = by_kind.values().filter_map(|v| v.as_u64()).sum();
    assert_eq!(
        summed_kinds,
        units.len() as u64,
        "totals.by_kind must partition the units exactly"
    );
}

/// SD28-E15: a `class_feature` record from an unowned option pool (Rage
/// Power, Discovery, Domain Power, ...) that carries no magnitude token AND
/// is confirmed absent from every engine table, corpus JSON cache, and
/// picker this program has (verified by direct search, not inferred from
/// the magnitude check alone) must land `not-ingested` -- a real, honestly
/// reported gap -- never `unknown`, which implies a mystery rather than a
/// confirmed absence. `text-complete` was tried and rejected here: it
/// requires the engine to HOLD the record per `status_vocabulary`'s own
/// definition, and it does not (decisions.md §40's amendment). Before this
/// fix `classify()`'s `Kind::ClassFeature` "group names no class" branch
/// returned `unknown` unconditionally, misclassifying 2,275 of the epic's
/// 4,172 `unknown` units as an open mystery rather than a confirmed gap.
#[test]
fn zero_magnitude_option_pool_class_features_are_not_ingested_not_unknown() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/work-inventory.json");
    let Ok(text) = std::fs::read_to_string(&path) else {
        println!("SKIP: docs/work-inventory.json has not been generated yet");
        return;
    };
    let doc: serde_json::Value = serde_json::from_str(&text).expect("inventory is valid JSON");
    let units = doc["units"].as_array().expect("units is an array");

    // Real, verified-by-hand-trace units: zero-magnitude Rage Power records
    // from acg_abilities_class.lst:2658,2660. Confirmed absent from every
    // engine table this program has (rules_core, apps/desktop) -- only a
    // slot-COUNT mechanism (barbarian_features::rage_powers_known) is
    // wired, never a catalog of the individual named options.
    for id in [
        "advanced_class_guide:class_feature:rage_power_abyssal_blood",
        "advanced_class_guide:class_feature:rage_power_abyssal_blood_lesser",
    ] {
        let unit = units.iter().find(|u| u["id"] == id).unwrap_or_else(|| panic!("{id} missing from inventory"));
        assert_eq!(unit["magnitude_token_count"], 0, "{id}: fixture assumption changed, re-check");
        assert_eq!(
            unit["status"], "not-ingested",
            "{id}: zero-magnitude option-pool class_feature confirmed unheld by the engine must be not-ingested, not unknown or text-complete"
        );
        assert_eq!(
            unit["evidence"],
            "class_feature_option_pool_record_not_held_by_engine"
        );
    }

    // No unknown class_feature may carry magnitude_token_count == 0 -- that
    // combination is now unreachable by construction (it either finds an
    // owner and grounds/defers through the existing paths, or falls to the
    // text_only check above and lands not-ingested).
    let mut violations = 0usize;
    for unit in units {
        if unit["status"] == "unknown"
            && unit["kind"] == "class_feature"
            && unit["magnitude_token_count"] == 0
        {
            violations += 1;
            eprintln!("VIOLATION: {} is unknown with magnitude_token_count 0", unit["id"]);
        }
    }
    assert_eq!(violations, 0, "found {violations} unknown class_feature units with no magnitude token");
}

/// The inventory must cover EVERY corpus book, including the ones no code has
/// read — that is the completeness guarantee, and it is what makes a
/// full-corpus run viable. A book that vanishes from the roster is a silent
/// hole, so this asserts against the corpus directory listing itself.
#[test]
fn every_corpus_book_appears_in_the_inventory() {
    let dir = corpus_or_skip!();
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/work-inventory.json");
    let Ok(text) = std::fs::read_to_string(&path) else {
        println!("SKIP: docs/work-inventory.json has not been generated yet");
        return;
    };
    let doc: serde_json::Value = serde_json::from_str(&text).expect("inventory is valid JSON");
    let listed: std::collections::HashSet<String> = doc["books"]
        .as_array()
        .expect("books array")
        .iter()
        .filter_map(|b| b["id"].as_str().map(|s| s.to_string()))
        .collect();

    let on_disk: Vec<String> = std::fs::read_dir(&dir)
        .expect("corpus books directory is readable")
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    for book in &on_disk {
        assert!(
            listed.contains(book),
            "corpus book `{book}` is missing from the inventory -- a book that \
             is not listed is a book whose work is invisible"
        );
    }
    // Every on-disk roleplaying_game book must be listed, and the only
    // entries the inventory may carry beyond that set are the books
    // enumerated from their own corpus roots via `EXTRA_BOOK_DIRS`:
    // `ultimate_psionics` (SD-28, dreamscarred_press) and the twelve SD-30
    // campaign_setting books.
    let on_disk_set: std::collections::HashSet<String> = on_disk.iter().cloned().collect();
    let extra: std::collections::HashSet<String> =
        listed.difference(&on_disk_set).cloned().collect();
    let expected: std::collections::HashSet<String> = ["ultimate_psionics"]
        .into_iter()
        .chain(SD30_CAMPAIGN_SETTING_BOOKS.iter().copied())
        .map(str::to_string)
        .collect();
    assert_eq!(
        extra, expected,
        "the inventory must list exactly the on-disk roleplaying_game books plus \
         the EXTRA_BOOK_DIRS roster (ultimate_psionics + the twelve SD-30 \
         campaign_setting books)"
    );

    // And every un-ingested book must contribute real, named units rather than
    // being skipped: that is what makes `not-started` a measurement.
    let unstarted_books: std::collections::HashSet<String> = doc["units"]
        .as_array()
        .expect("units array")
        .iter()
        .filter(|u| u["status"].as_str() == Some("not-started"))
        .filter_map(|u| u["book"].as_str().map(|s| s.to_string()))
        .collect();
    assert!(
        unstarted_books.len() >= 15,
        "expected the large majority of un-ingested books to contribute \
         not-started units, saw {}",
        unstarted_books.len()
    );
}

/// SD-30: the twelve campaign_setting books (Book of the Damned ×2, Inner Sea
/// World Guide + nine Inner Sea modules). Kept in one place so the roster
/// assertions above and the per-book assertions below can never disagree
/// about which books SD-30 added.
/// The subset of [`SD30_CAMPAIGN_SETTING_BOOKS`] that SD-29's lanes have since
/// ingested, so the inventory registers them `in_scope` rather than
/// `future_state`. Named individually rather than derived from the inventory,
/// because deriving the expectation from the artifact under test would make the
/// assertion vacuous.
const SD29_INGESTED_CAMPAIGN_SETTING_BOOKS: &[&str] = &[
    // SD-29 race-trait lane round 2 (`decisions.md §45`): 72 race-trait records.
    "inner_sea_races",
    // SD-29 monster lane round 2 (`decisions.md §46`): 5 + 36 and 4 + 17
    // `monster`/`monster_ability` records. The first `campaign_setting/` books
    // to carry the monster chassis. Added when the two lanes' branches merged
    // -- this entry and the one above were each written by the lane that
    // ingested the book, and the merge is the first point at which the list
    // had to be complete. A third lane ingesting a thirteenth book adds a line.
    "book_of_the_damned_volume_1",
    "book_of_the_damned_volume_2",
];

const SD30_CAMPAIGN_SETTING_BOOKS: &[&str] = &[
    "book_of_the_damned_volume_1",
    "book_of_the_damned_volume_2",
    "inner_sea_world_guide",
    "inner_sea_combat",
    "inner_sea_faiths",
    "inner_sea_gods",
    "inner_sea_magic",
    "inner_sea_races",
    "inner_sea_temples",
    "inner_sea_taverns",
    "inner_sea_bestiary",
    "inner_sea_intrigue",
];

/// SD-30: every campaign_setting book on the pinned sixteen-book list must
/// appear in the inventory like any other un-ingested book -- a real books[]
/// entry registered `future_state`, real corpus files seen, and every unit it
/// contributes at `not-started` (no compiled rule set claims any of them).
/// Without this, cycle step 0 and definition-of-done item 4 are unexecutable
/// for twelve of SD-30's sixteen books.
#[test]
fn sd30_campaign_setting_books_appear_in_the_inventory_as_not_started_books() {
    let _dir = corpus_or_skip!();
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/work-inventory.json");
    let Ok(text) = std::fs::read_to_string(&path) else {
        println!("SKIP: docs/work-inventory.json has not been generated yet");
        return;
    };
    let doc: serde_json::Value = serde_json::from_str(&text).expect("inventory is valid JSON");
    let books = doc["books"].as_array().expect("books array");
    let units = doc["units"].as_array().expect("units array");

    for id in SD30_CAMPAIGN_SETTING_BOOKS {
        let book = books
            .iter()
            .find(|b| b["id"].as_str() == Some(id))
            .unwrap_or_else(|| panic!("{id} must appear in the inventory's books list"));
        // A book SD-29 has since ingested is `in_scope`, not `future_state`, and
        // that is the correct state rather than a regression -- the roster
        // assertion above is about SD-30's sixteen books existing, not about
        // them staying un-ingested forever. SD-29's race-trait lane round 2
        // ingested `inner_sea_races` on 2026-08-11 and left this assertion RED
        // on the branch; round 3 states the exemption as its own claim rather
        // than dropping the book from the roster or relaxing the check
        // (`decisions.md §47.3`). The `else` branch is deliberately a hard
        // assertion too, so a book flipping scope silently still fails here.
        let expected_scope =
            if SD29_INGESTED_CAMPAIGN_SETTING_BOOKS.contains(id) { "in_scope" } else { "future_state" };
        assert_eq!(
            book["scope"].as_str(),
            Some(expected_scope),
            "{id} must be registered {expected_scope} (data/stubs/{id}.json for future_state), \
             got {:?}",
            book["scope"]
        );
        if expected_scope == "in_scope" {
            continue;
        }
        let enumerated = book["files_enumerated"].as_u64().unwrap_or(0);
        let not_enumerated = book["files_not_enumerated"]
            .as_array()
            .map(|a| a.len() as u64)
            .unwrap_or(0);
        assert!(
            enumerated + not_enumerated > 0,
            "{id} must see real corpus files (enumerated or explicitly not), got none"
        );
        for unit in units.iter().filter(|u| u["book"].as_str() == Some(id)) {
            assert_eq!(
                unit["status"].as_str(),
                Some("not-started"),
                "unit {} of {id} must be not-started (no compiled rule set claims \
                 this book), was {:?}",
                unit["id"],
                unit["status"]
            );
        }
    }
}

/// SD-28: `ultimate_psionics` (Dreamscarred Press, a flat corpus directory
/// outside `roleplaying_game`) must appear in the inventory like any other
/// book -- real enumerated files, real kinds -- even though its `.pcc` has
/// no `PCC:` includes.
///
/// **This test's premise changed since it was written, and the assertion
/// below reflects that verified change, not a convenience relaxation
/// (`decisions.md §32`).** UPsi genuinely had no compiled rule set when this
/// test was authored, so every one of its units was blanket `not-started`.
/// UPsi has since landed a real feat catalog (221 records,
/// `decisions.md §50`) and an archetype-swap table (15 records,
/// `decisions.md §51`), so `RuleSetId::Upsi` has been in `COMPILED_RULE_SETS`
/// for multiple cycles now (unrelated to SD28-E15's equipment-classifier
/// fix, which never touches this const -- confirmed by `git log -p` on this
/// file finding no local diff near it). Once a book has ANY compiled rule
/// set, `classify()` stops returning blanket `not-started` and evaluates
/// each unit against real per-kind engine facts instead -- so UPsi's units
/// now carry a real, granular mix of statuses (`not-ingested`/`unknown` for
/// the un-landed majority, `text-complete`/`unknown` for the landed feat
/// catalog, `grounded` for 4 race traits), never `not-started`. The old
/// blanket assertion was asserting a fact about the world that stopped being
/// true partway through this program, not a fact this fix broke.
#[test]
fn ultimate_psionics_appears_in_the_inventory_with_real_per_kind_status() {
    let _dir = corpus_or_skip!();
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/work-inventory.json");
    let Ok(text) = std::fs::read_to_string(&path) else {
        println!("SKIP: docs/work-inventory.json has not been generated yet");
        return;
    };
    let doc: serde_json::Value = serde_json::from_str(&text).expect("inventory is valid JSON");

    let books = doc["books"].as_array().expect("books array");
    let book = books
        .iter()
        .find(|b| b["id"].as_str() == Some("ultimate_psionics"))
        .expect("ultimate_psionics must appear in the inventory's books list");

    assert!(
        book["files_enumerated"].as_u64().unwrap_or(0) > 0,
        "ultimate_psionics must enumerate at least one real .lst file, got {:?}",
        book["files_enumerated"]
    );

    let not_enumerated: Vec<String> = book["files_not_enumerated"]
        .as_array()
        .expect("files_not_enumerated array")
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();
    assert!(
        not_enumerated.iter().any(|f| f == "up_powers.lst"),
        "up_powers.lst must land in files_not_enumerated -- mapping it to Spell \
         is deliberately deferred to Epic 9. Saw: {not_enumerated:?}"
    );

    let kinds = book["kinds"].as_object().expect("kinds object");
    for expected in ["class", "race", "feat", "equipment"] {
        assert!(
            kinds.contains_key(expected),
            "ultimate_psionics must contribute at least a {expected} unit; kinds \
             seen were {:?}",
            kinds.keys().collect::<Vec<_>>()
        );
    }

    let units = doc["units"].as_array().expect("units array");
    let up_units: Vec<&serde_json::Value> =
        units.iter().filter(|u| u["book"].as_str() == Some("ultimate_psionics")).collect();
    assert!(!up_units.is_empty(), "ultimate_psionics must contribute real, named units");

    // No unit may EVER be `not-started` once a compiled rule set claims the
    // book -- `not-started` means "no engine table for this book was even
    // consulted", which is no longer true for any UPsi unit. This is the
    // one assertion carried forward unchanged from the original test's
    // intent (verifying the not-started/compiled-book status boundary),
    // just inverted to match which side of that boundary UPsi is now on.
    for unit in &up_units {
        assert_ne!(
            unit["status"].as_str(),
            Some("not-started"),
            "unit {} of ultimate_psionics must NOT be not-started -- RuleSetId::Upsi has been \
             in COMPILED_RULE_SETS since the feat catalog landed, so this book's units must \
             receive a real per-kind verdict, was {:?}",
            unit["id"],
            unit["status"]
        );
    }

    // The landed feat catalog (decisions.md §50, 221 records) must show up
    // as real progress, not merely "not not-started" -- pins the positive
    // half of the boundary this test now checks.
    let feat_statuses: std::collections::BTreeSet<&str> = up_units
        .iter()
        .filter(|u| u["kind"].as_str() == Some("feat"))
        .filter_map(|u| u["status"].as_str())
        .collect();
    assert!(
        feat_statuses.contains("text-complete") || feat_statuses.contains("grounded"),
        "ultimate_psionics' landed feat catalog must produce at least one text-complete or \
         grounded feat unit, statuses seen were {feat_statuses:?}"
    );
}
