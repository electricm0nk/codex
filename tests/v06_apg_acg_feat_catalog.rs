//! v0.6: APG and ACG feat catalogs.
//!
//! Before this cycle the engine's only feat catalog was CRB's 185 records
//! (`rules_tables::crb::feats::feat_tables()`), and
//! `rules_tables::{apg,acg}` carried no feat table at all — so a player
//! building an APG or ACG class could not take a single feat from that
//! class's own book. This test is the structural proof for the two new
//! catalogs, mirroring `sd19_feat_catalog.rs`'s pattern for CRB: real
//! per-book counts, real pinned records, no fabricated fields, and a
//! `CORPUS_ROOT`-gated cross-check against the live PCGen corpus so drift
//! is caught rather than silently trusted.
//!
//! It also pins the two properties that only appear once more than one
//! book is in play: which feat keys repeat across books (so no book's
//! record shadows another's unnoticed), and that
//! `feats_all::all_feat_tables()` — the single aggregate the Tauri feat
//! picker reads — carries every record from every ingested book tagged
//! with the book it came from. The aggregate has since widened past the
//! three books this file was written for, to Advanced Race Guide and
//! Pathfinder Unchained; the aggregate-level assertions below cover all
//! five, while the per-category and corpus-cross-check tests stay APG/ACG
//! -specific, which is what this file is for.

use std::path::PathBuf;

use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::rules_tables::acg::feats as acg_feats;
use codex::rules_core::rules_tables::apg::feats as apg_feats;
use codex::rules_core::rules_tables::crb::feats::{FeatCategory, FeatEffectBonus};
use codex::rules_core::rules_tables::feats_all::all_feat_tables;

#[test]
fn apg_feat_catalog_has_the_real_per_category_corpus_counts() {
    // `apg_feats.lst` carries 221 non-comment records. 37 are `.MOD` lines
    // (they modify a base record rather than declaring one) leaving 184
    // real declarations; of those, 172 carry a `TYPE:` facet this catalog
    // can honestly classify. See `apg::feats`'s own doc comment for the
    // 12 excluded records and why each is excluded.
    let all = apg_feats::feat_tables();
    assert_eq!(all.len(), 172, "expected 172 classifiable APG feat records");

    let count_of = |category: FeatCategory| all.iter().filter(|f| f.category == category).count();
    assert_eq!(count_of(FeatCategory::General), 69);
    assert_eq!(count_of(FeatCategory::Combat), 81);
    assert_eq!(count_of(FeatCategory::Metamagic), 19);
    assert_eq!(count_of(FeatCategory::Teamwork), 3);
    // The APG has no Item Creation or Panache feat records at all — an
    // honest zero, not a gap in this ingest.
    assert_eq!(count_of(FeatCategory::ItemCreation), 0);
    assert_eq!(count_of(FeatCategory::Panache), 0);
}

#[test]
fn acg_feat_catalog_has_the_real_per_category_corpus_counts() {
    // `acg_feats.lst` carries 173 non-comment records, 39 of them `.MOD`
    // lines, leaving 134 real declarations of which 129 are classifiable.
    // See `acg::feats`'s own doc comment for the 5 exclusions.
    let all = acg_feats::feat_tables();
    assert_eq!(all.len(), 129, "expected 129 classifiable ACG feat records");

    let count_of = |category: FeatCategory| all.iter().filter(|f| f.category == category).count();
    assert_eq!(count_of(FeatCategory::General), 62);
    assert_eq!(count_of(FeatCategory::Combat), 59);
    assert_eq!(count_of(FeatCategory::Teamwork), 4);
    assert_eq!(count_of(FeatCategory::Panache), 4);
    assert_eq!(count_of(FeatCategory::Metamagic), 0);
    assert_eq!(count_of(FeatCategory::ItemCreation), 0);
}

#[test]
fn pinned_apg_feats_carry_their_real_corpus_fields() {
    let all = apg_feats::feat_tables();
    let find = |key: &str| {
        all.iter()
            .find(|f| f.key == key)
            .unwrap_or_else(|| panic!("'{key}' must be in the APG catalog"))
    };

    // A Teamwork feat: the whole reason `FeatCategory::Teamwork` exists —
    // `TYPE:Teamwork` with no Combat/General facet, so under CRB's
    // four-category rule it would have been silently dropped.
    let allied = find("Allied Spellcaster");
    assert_eq!(allied.category, FeatCategory::Teamwork);
    assert_eq!(
        allied.description,
        Some(
            "With the aid of an ally, you are skilled at piercing the protections \
             of other creatures with your spells."
        )
    );
    assert_eq!(
        allied.prerequisites,
        Some(&["PREMULT:1,[PRECLASS:1,SPELLCASTER=1],[PREVARGTEQ:CasterLevel_Highest,1]"] as &[&str])
    );

    // A Metamagic feat with no prerequisite token at all — recorded as
    // `None`, never as an empty list that would read as "not gathered yet".
    let bouncing = find("Bouncing Spell");
    assert_eq!(bouncing.category, FeatCategory::Metamagic);
    assert_eq!(
        bouncing.description,
        Some("You can direct a failed spell against a different target.")
    );
    assert_eq!(bouncing.prerequisites, None);

    // A General feat carrying both a real `BONUS:` token and a real
    // prerequisite chain.
    let extra_hex = find("Extra Hex");
    assert_eq!(extra_hex.category, FeatCategory::General);
    assert_eq!(
        extra_hex.effect,
        Some(&[FeatEffectBonus {
            qualifiers: &["ABILITYPOOL", "Witch Hex", "1"]
        }] as &[FeatEffectBonus])
    );

    // The base `Elemental Focus` feat (`TYPE:General`) is present; the four
    // `TYPE:ElementalFocus` per-element DC-support records are not feats
    // and are excluded.
    let elemental_focus = find("Elemental Focus");
    assert_eq!(elemental_focus.category, FeatCategory::General);
    assert!(
        !all.iter().any(|f| f.key == "Elemental Focus (Acid)"),
        "the per-element DC-support helper records are not player-facing feats"
    );
}

#[test]
fn pinned_acg_feats_carry_their_real_corpus_fields() {
    let all = acg_feats::feat_tables();
    let find = |key: &str| {
        all.iter()
            .find(|f| f.key == key)
            .unwrap_or_else(|| panic!("'{key}' must be in the ACG catalog"))
    };

    let extra_panache = find("Extra Panache");
    assert_eq!(extra_panache.category, FeatCategory::Panache);
    assert_eq!(
        extra_panache.description,
        Some("You have more panache than the ordinary swashbuckler.")
    );
    assert_eq!(
        extra_panache.effect,
        Some(&[
            FeatEffectBonus {
                qualifiers: &["VAR", "PanachePoints", "2"]
            },
            FeatEffectBonus {
                qualifiers: &["VAR", "Panache_Cap", "2"]
            },
        ] as &[FeatEffectBonus])
    );

    let pack_flanking = find("Pack Flanking");
    assert_eq!(pack_flanking.category, FeatCategory::Teamwork);
    assert_eq!(
        pack_flanking.prerequisites,
        Some(&[
            "PREABILITY:1,CATEGORY=FEAT,Combat Expertise",
            "PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion",
            "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]",
        ] as &[&str])
    );

    let extra_exploit = find("Extra Arcanist Exploit");
    assert_eq!(extra_exploit.category, FeatCategory::General);
    assert_eq!(
        extra_exploit.description,
        Some("Your repertoire of arcanist exploits expands.")
    );

    // `Witch Hex` / `Shaman Hex` (`TYPE:Hex Selection`) and the three
    // `TYPE:Evolved Companion` records are selection/companion-source
    // plumbing, not feats, and carry no player-facing description.
    for plumbing in ["Witch Hex", "Shaman Hex", "Animal Companion of Nature Bond Class Feature"] {
        assert!(
            !all.iter().any(|f| f.key == plumbing),
            "'{plumbing}' is corpus plumbing, not a feat"
        );
    }
}

#[test]
fn every_apg_and_acg_record_has_a_real_key_name_and_description() {
    // No fabricated fields: every ingested record must carry a non-empty
    // key and name, and its `DESC:` text when the corpus record has one.
    //
    // Exactly one record across both books genuinely has no `DESC:`
    // token: APG's `VISIBLE:DISPLAY` "Elemental Fist" base variant (its
    // two `VISIBLE:EXPORT` siblings, distinct `KEY:`s, do carry the
    // text). That absence is recorded as `None` rather than filled in
    // from a sibling record, the same way CRB records the 8
    // "Heighten Spell +N" records' missing `DESC:`. An earlier version of
    // this test asserted no such record existed and failed — the data was
    // right and the assertion was wrong.
    let mut missing_description: Vec<&str> = Vec::new();
    for (book, table) in [("APG", apg_feats::feat_tables()), ("ACG", acg_feats::feat_tables())] {
        for entry in table {
            assert!(!entry.key.is_empty(), "{book} entry with empty key");
            assert!(!entry.name.is_empty(), "{book} entry '{}' has empty name", entry.key);
            match entry.description {
                Some(text) => assert!(
                    !text.is_empty(),
                    "{book} entry '{}' has an empty description string; absence must be None",
                    entry.key
                ),
                None => missing_description.push(entry.key),
            }
            // An empty effect/prereq slice would be indistinguishable from
            // "no data gathered yet"; absence is always None.
            assert!(entry.effect.is_none_or(|e| !e.is_empty()));
            assert!(entry.prerequisites.is_none_or(|p| !p.is_empty()));
        }
    }
    assert_eq!(missing_description, vec!["Elemental Fist"]);
}

#[test]
fn cross_book_feat_key_repeats_are_exactly_the_known_set() {
    // A recent bug had 18 spell keys collide across books, one book's
    // record silently shadowing another's. Feats had no cross-book repeat
    // at all while the aggregate was CRB/APG/ACG. Pathfinder Unchained
    // introduces exactly one — `Endurance`, which it re-lists from the
    // Core Rulebook under its Wound Threshold rules rather than defining
    // anew (see `rules_tables::feats_all`'s own "Key collisions" section
    // for the two corpus rows). This test pins that set exactly, so a
    // genuinely different feat arriving under an existing key fails here
    // instead of shadowing one silently.
    let mut seen: Vec<(&str, RuleSetId)> = Vec::new();
    let mut cross_book: Vec<(&str, RuleSetId, RuleSetId)> = Vec::new();
    for book in all_feat_tables() {
        for entry in book.entries {
            if let Some((_, other)) = seen.iter().find(|(key, _)| *key == entry.key) {
                // CRB's two real "Combat Expertise" corpus variants are a
                // known within-book duplicate preserved verbatim (see
                // `sd19_feat_catalog.rs`); only cross-book repeats are
                // collected here.
                if *other != book.rule_set {
                    cross_book.push((entry.key, *other, book.rule_set));
                }
            }
            seen.push((entry.key, book.rule_set));
        }
    }

    // Widened by the corpus feat gap lane (`epic-4-proven-feat-race-class`,
    // commit `dde9dfc4`), whose 83 rows brought two more genuine upstream
    // duplicates into the aggregate. Both were checked against the PCGen
    // source before this pin moved — a shared key never implies a shared
    // record, so each was confirmed to be a first-class definition in BOTH
    // books rather than the gap generator mis-attributing one book's row to
    // another:
    //   grep -n '^Extended Animal Focus' \
    //     .../advanced_class_guide/acg_feats.lst .../ultimate_wilderness/uw_feats.lst
    //     -> acg_feats.lst:58 AND uw_feats.lst:46
    //   grep -n '^Feral Combat Training' \
    //     .../ultimate_combat/uc_feats.lst .../ultimate_psionics/up_feats.lst
    //     -> uc_feats.lst:117 AND up_feats.lst:128
    // A genuinely different feat arriving under an existing key still fails
    // here; this pin is widened to the verified set, not relaxed.
    //
    // `SD31-E6-F8-002` widened this further with three more pairs, and NONE
    // is a reprint -- each is two genuinely DIFFERENT feats sharing a
    // display name, checked against both corpus records' own `DESC:`/
    // `BENEFIT:` text (`rules_tables::feats_all`'s own
    // `cross_book_key_collisions_are_exactly_the_known_set` carries the full
    // per-pair citation; not duplicated here):
    //   `Returning Throw` (Upsi marksman feat vs Isr goblinoid teamwork feat)
    //   `Desert Dweller` (Uw desert-terrain feat vs Iswg heat-resistance feat)
    //   `Strangler` (Uc grapple/sneak-attack feat vs MonsterCodex lasso feat)
    let (mythic_cross_book, other_cross_book): (Vec<_>, Vec<_>) =
        cross_book.into_iter().partition(|(_, _, second)| *second == RuleSetId::Mythic);
    assert_eq!(
        other_cross_book,
        vec![
            ("Endurance", RuleSetId::Crb, RuleSetId::Pu),
            ("Extended Animal Focus", RuleSetId::Acg, RuleSetId::Uw),
            ("Feral Combat Training", RuleSetId::Uc, RuleSetId::Upsi),
            ("Returning Throw", RuleSetId::Upsi, RuleSetId::Isr),
            ("Desert Dweller", RuleSetId::Uw, RuleSetId::Iswg),
            ("Strangler", RuleSetId::Uc, RuleSetId::MonsterCodex),
        ]
    );

    // `SD31-E6-F2-007` -- `RuleSetId::Mythic`'s 142 collisions are the same
    // mechanically-proven population `feats_all::tests::
    // cross_book_key_collisions_are_exactly_the_known_set` checks record-by-
    // record (every colliding Mythic row's own `PREABILITY:` prerequisite
    // names that exact key under `CATEGORY=FEAT`); not re-verified a second
    // time here, only counted, so this pin still fails if a future book adds
    // or removes one.
    assert_eq!(mythic_cross_book.len(), 142, "re-derive if a book's feat gap rows change");
}

#[test]
fn the_aggregate_catalog_spans_every_ingested_book() {
    let books = all_feat_tables();
    assert_eq!(books.len(), 23);

    let entries_for = |rule_set: RuleSetId| {
        books
            .iter()
            .find(|b| b.rule_set == rule_set)
            .unwrap_or_else(|| panic!("{rule_set:?} must be in the aggregate catalog"))
            .entries
            .len()
    };
    // Each book is now `hand-authored + corpus gap rows`, since
    // `feats_all::all_feat_tables` chains `feat_gap_tables`' rows after each
    // book's own. The gap addends are the generated table's own per-book
    // counts, tallied two independent ways so a single miscount cannot move
    // a pin (`AGENTS.md` §"Concurrency and Measurement"):
    //   grep -E '^/// [a-z_]+ - [0-9]+ record' src/rules_core/rules_tables/feat_gap_tables.rs
    //   awk '/^pub static /{n=$3} /FeatCatalogRecord \{/{c[n]++} END{for(k in c) print c[k],k}' \
    //     src/rules_core/rules_tables/feat_gap_tables.rs
    // Both agree: CRB 1, core_essentials 15, ARG 48, UC 2, UI 3, UM 12, UPsi 1,
    // UW 1 = 83 rows, matching that file's own stated total (`SD31-E6-F8-001`
    // re-bucketed 15 of the prior CRB-filed 16 rows to `RuleSetId::Ce`, the
    // rule set `classify()`'s feat arm actually resolves a `core_essentials`
    // -directory record's `source_book` to). APG/ACG/PU/UCA have no gap rows
    // and are unmoved.
    assert_eq!(entries_for(RuleSetId::Crb), 186); // 185 + 1
    assert_eq!(entries_for(RuleSetId::Apg), 172); // unmoved
    assert_eq!(entries_for(RuleSetId::Acg), 129); // unmoved
    assert_eq!(entries_for(RuleSetId::Arg), 235); // 187 + 48
    assert_eq!(entries_for(RuleSetId::Pu), 17); // unmoved
    assert_eq!(entries_for(RuleSetId::Uca), 23); // unmoved
    assert_eq!(entries_for(RuleSetId::Ui), 107); // 104 + 3
    assert_eq!(entries_for(RuleSetId::Uw), 136); // 135 + 1
    assert_eq!(entries_for(RuleSetId::Uc), 263); // 261 + 2
    assert_eq!(entries_for(RuleSetId::Um), 156); // 144 + 12
    assert_eq!(entries_for(RuleSetId::Upsi), 222); // 221 + 1
    assert_eq!(entries_for(RuleSetId::Ce), 15); // 0 + 15
    // `SD31-E6-F8-002` -- five more books already compiled for another kind
    // that had no feat table at all; every served entry is a gap row.
    assert_eq!(entries_for(RuleSetId::Ha), 61); // 0 + 61
    assert_eq!(entries_for(RuleSetId::Isr), 50); // 0 + 50
    assert_eq!(entries_for(RuleSetId::Oa), 68); // 0 + 68
    assert_eq!(entries_for(RuleSetId::Iswg), 31); // 0 + 31
    assert_eq!(entries_for(RuleSetId::MonsterCodex), 32); // 0 + 32
    // `SD31-E6-F2-007` -- Mythic Adventures' first compiled rule set of any
    // kind; every served entry is a gap row (`ma_feats.lst`'s 358 non-`.MOD`
    // declarations). `SD31-W10-INTEGRATE-001` excluded 159 `VISIBLE:EXPORT`
    // display-plumbing twins (PCGen's own export-only duplicate of an
    // auto-granted feat, never independently selectable): 358 -> 199.
    assert_eq!(entries_for(RuleSetId::Mythic), 199); // 0 + 199
    // `SD31-E6-F8-003` -- two more books already compiled for another kind
    // that had no feat table at all; every served entry is a gap row.
    assert_eq!(entries_for(RuleSetId::Isi), 6); // 0 + 6
    assert_eq!(entries_for(RuleSetId::Botd2), 1); // 0 + 1
    // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-onboarding-
    // precondition`, AT-32-G0-003) -- Inner Sea Taverns' first compiled
    // rule set of any kind; every served entry is a gap row
    // (`istav_feats.lst`'s 9 non-`.MOD` declarations).
    assert_eq!(entries_for(RuleSetId::InnerSeaTaverns), 9); // 0 + 9
    // SD-32 T9 onboarding (card 11), `decisions.md §19` PI sign-off --
    // `Isc`/`Isg` already compiled for equipment/monster content, each
    // served entry a gap row (`isc_abilities_feat.lst`'s 23 non-`.MOD`,
    // non-`NAMEISPI:YES` declarations; `isg_abilities_feat.lst`'s 86).
    assert_eq!(entries_for(RuleSetId::Isc), 23); // 0 + 23
    assert_eq!(entries_for(RuleSetId::Isg), 86); // 0 + 86

    let total: usize = books.iter().map(|b| b.entries.len()).sum();
    assert_eq!(total, 2227, "186 CRB + 172 APG + 129 ACG + 235 ARG + 17 PU + 23 UCA + 107 UI + 136 UW + 263 UC + 156 UM + 222 UPsi + 15 Ce + 61 Ha + 50 Isr + 68 Oa + 31 Iswg + 32 MonsterCodex + 199 Mythic + 6 Isi + 1 Botd2 + 9 InnerSeaTaverns + 23 Isc + 86 Isg = 1578 hand-authored + 649 corpus gap rows (SD31-E6-F8-001's 83 + SD31-E6-F8-002's 242 + SD31-E6-F2-007's 199, after SD31-W10-INTEGRATE-001 excluded 159 VISIBLE:EXPORT twins from the raw 358 + SD31-E6-F8-003's 7 + SD-32 Gate 0 book-onboarding precondition's 9 + SD-32 T9 onboarding's (card 11) 109)");
}

#[test]
fn crb_records_gained_their_real_prerequisite_tokens() {
    // `feat_prereqs/general.rs`'s doc comment named the missing
    // `PRE*`-family tokens on the table store as the blocker for a real
    // per-feat prerequisite chain, citing three CRB records by name. Those
    // tokens are now transcribed verbatim.
    use codex::rules_core::rules_tables::crb::feats::feat_tables;
    let all = feat_tables();
    let find = |key: &str| all.iter().find(|f| f.key == key).expect(key);

    assert_eq!(
        find("Greater Spell Focus").prerequisites,
        Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus"] as &[&str])
    );
    assert_eq!(
        find("Improved Great Fortitude").prerequisites,
        Some(&["PREABILITY:1,CATEGORY=FEAT,Great Fortitude"] as &[&str])
    );
    assert!(
        find("Leadership")
            .prerequisites
            .is_some_and(|p| p.contains(&"PRELEVEL:MIN=7")),
        "Leadership's real PRELEVEL:MIN=7 token must be transcribed"
    );

    // Adding the field must not have disturbed the existing 185.
    assert_eq!(all.len(), 185);
}

fn corpus_root() -> Option<PathBuf> {
    match std::env::var("CORPUS_ROOT") {
        Ok(value) => {
            let path = PathBuf::from(value);
            if path.is_dir() { Some(path) } else { None }
        }
        Err(_) => None,
    }
}

/// Minimal inline re-derivation of the `TYPE:`-facet classification rule
/// `apg::feats` / `acg::feats` document, applied straight to the raw
/// corpus text — deliberately not sharing code with the offline
/// generator, so this test actually fails if the transcribed data drifts
/// from the live corpus. Mirrors `sd19_feat_catalog.rs`'s own
/// `facet_category`, widened to the two categories only the APG/ACG
/// corpus uses.
fn facet_category(type_value: &str) -> Option<&'static str> {
    for facet in type_value.split('.') {
        match facet {
            "General" => return Some("General"),
            "Combat" => return Some("Combat"),
            "ItemCreation" => return Some("ItemCreation"),
            "Metamagic" => return Some("Metamagic"),
            "Teamwork" => return Some("Teamwork"),
            "Panache" => return Some("Panache"),
            _ => {}
        }
    }
    None
}

/// One classifiable corpus record, re-derived straight from the raw
/// `.lst` line.
struct CorpusRecord {
    key: String,
    name: String,
    category: &'static str,
    description: Option<String>,
    has_bonus: bool,
    prerequisite_count: usize,
}

/// Every real (non-comment, non-`.MOD`) classifiable record in a feats
/// `.lst`, in file order.
fn classifiable_corpus_records(text: &str) -> Vec<CorpusRecord> {
    let mut records = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').filter(|f| !f.is_empty()).collect();
        let Some(name) = fields.first().map(|f| f.trim()) else {
            continue;
        };
        if name.is_empty() || name.starts_with("SOURCE") || name.contains(".MOD") {
            continue;
        }
        let Some(type_value) = fields.iter().skip(1).find_map(|f| f.trim().strip_prefix("TYPE:"))
        else {
            continue;
        };
        let Some(category) = facet_category(type_value) else {
            continue;
        };
        records.push(CorpusRecord {
            key: fields
                .iter()
                .skip(1)
                .find_map(|f| f.strip_prefix("KEY:"))
                .unwrap_or(name)
                .to_string(),
            name: name.to_string(),
            category,
            description: fields
                .iter()
                .skip(1)
                .find_map(|f| f.strip_prefix("DESC:"))
                .map(|d| d.to_string()),
            has_bonus: fields.iter().skip(1).any(|f| f.starts_with("BONUS:")),
            prerequisite_count: fields
                .iter()
                .skip(1)
                .filter(|f| f.starts_with("PRE") || f.starts_with("!PRE"))
                .count(),
        });
    }
    records
}

#[test]
fn apg_and_acg_catalogs_match_the_live_corpus() {
    let Some(root) = corpus_root() else {
        eprintln!(
            "CORPUS_ROOT not set or not a directory; skipping (set \
             CORPUS_ROOT=$HOME/workspace/repos/pcgen/data to enable)"
        );
        return;
    };

    let cases: [(&str, &[codex::rules_core::rules_tables::crb::feats::FeatTableEntry], usize); 2] = [
        (
            "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_feats.lst",
            apg_feats::feat_tables(),
            172,
        ),
        (
            "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_feats.lst",
            acg_feats::feat_tables(),
            129,
        ),
    ];

    for (relative, catalog, expected) in cases {
        let path = root.join(relative);
        if !path.is_file() {
            eprintln!("corpus file not present at {}; skipping", path.display());
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("feats .lst must be readable");
        let corpus = classifiable_corpus_records(&text);
        assert_eq!(
            corpus.len(),
            expected,
            "live corpus classifiable-record count for {relative} drifted from the \
             {expected} this catalog was generated from; regenerate feat_data/"
        );

        // Field-level, per record, keyed on the corpus `KEY:` (which is
        // globally unique within each of these two books). This is the
        // check that would actually catch a transcription slip: a
        // description belonging to the wrong row, a dropped `BONUS:`
        // token, or a lost prerequisite.
        for record in &corpus {
            let entry = catalog
                .iter()
                .find(|entry| entry.key == record.key)
                .unwrap_or_else(|| {
                    panic!("corpus record '{}' is classifiable but missing from {relative}'s catalog", record.key)
                });
            assert_eq!(entry.name, record.name, "'{}' display name drifted", record.key);
            assert_eq!(
                format!("{:?}", entry.category),
                record.category,
                "'{}' category drifted",
                record.key
            );
            assert_eq!(
                entry.description.map(str::to_owned),
                record.description,
                "'{}' DESC: text drifted from the live corpus",
                record.key
            );
            assert_eq!(
                entry.effect.is_some(),
                record.has_bonus,
                "'{}' BONUS: presence disagrees with the live corpus",
                record.key
            );
            assert_eq!(
                entry.prerequisites.map_or(0, <[&str]>::len),
                record.prerequisite_count,
                "'{}' PRE-token count disagrees with the live corpus",
                record.key
            );
        }
    }
}
