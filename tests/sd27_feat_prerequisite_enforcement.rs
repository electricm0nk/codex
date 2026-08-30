//! SD-27: feat prerequisite enforcement across all five ingested books.
//!
//! # The defect this closes
//!
//! There was no feat prerequisite enforcement anywhere in the product.
//! Proven in the running app: a **Fighter 1 with a +1 base attack bonus was
//! allowed to take Improved Two-Weapon Fighting**, which requires BAB +6,
//! Dex 17 and the Two-Weapon Fighting feat. All 690 offered feats were
//! accepted by every character.
//!
//! Two separate things were missing, and both had to land:
//!
//! 1. **The data.** `rules_tables::feats_all::FeatCatalogRecord` carried no
//!    `prerequisites` field at all, and ARG's and PU's own tables never
//!    gathered the tokens -- all 187 ARG feat rows carry at least one
//!    `PRE`-family token in the corpus and the engine held none of them.
//! 2. **The evaluation.** `feat_prereqs` answered only "is this id in the
//!    CRB catalog under this category".
//!
//! # What this file proves
//!
//! * The token-kind census is complete: every kind present in the live
//!   catalog is either modelled or on the explicit unmodelled list, so a
//!   newly-ingested book cannot introduce a kind that silently passes.
//! * The verdicts match the **published Core Rulebook** for 25 well-known
//!   feats, checked by hand against the printed prerequisites rather than
//!   against the engine's own output.
//! * Nothing is offered-then-refused and nothing is silently refused: every
//!   ineligible feat states a reason.
//! * `PCGEN_CORPUS_ROOT`-gated: the gathered ARG/PU tokens, the race
//!   subtype table, and the `FeatDexRequirement`-is-never-raised claim are
//!   each re-derived from the real on-disk corpus.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SkillAllocation,
};
use codex::rules_core::feat_prereqs::pre_tokens::{
    token_kind, CharacterPrereqFacts, MODELLED_KINDS, UNMODELLED_KINDS,
};
use codex::rules_core::feat_prereqs::{
    character_prereq_facts, evaluate_every_catalog_feat, evaluate_feat_key_prerequisites,
};
use codex::rules_core::rules_tables::feats_all::all_feat_tables;

fn build(
    race_id: &str,
    class_id: &str,
    level: u8,
    scores: AbilityScores,
    feats: &[&str],
    skills: &[(&str, u8)],
) -> CharacterInput {
    CharacterInput {
        case_id: None,
        source_package_id: "sd27-feat-prereqs".to_owned(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: race_id.to_owned(),
            class_levels: vec![CharacterClassLevel { class_id: class_id.to_owned(), level }],
            ability_scores: scores,
            selected_feats: feats.iter().map(|feat| (*feat).to_owned()).collect(),
            skill_allocations: skills
                .iter()
                .map(|(skill_id, ranks)| SkillAllocation {
                    skill_id: (*skill_id).to_owned(),
                    ranks: *ranks,
                })
                .collect(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// Str 14 / Dex 13 / Con 12 / Int 10 / Wis 10 / Cha 8 -- an ordinary
/// starting fighter's array.
fn starting_scores() -> AbilityScores {
    AbilityScores {
        strength: 14,
        dexterity: 13,
        constitution: 12,
        intelligence: 10,
        wisdom: 10,
        charisma: 8,
    }
}

fn fighter_1() -> CharacterPrereqFacts {
    let input = build("race:human", "class:fighter", 1, starting_scores(), &[], &[]);
    character_prereq_facts(&input, 1)
}

// ---------------------------------------------------------------------------
// 1. The census: no kind can slip through unnoticed
// ---------------------------------------------------------------------------

/// Every top-level and nested `PRE`-family kind present in the live 690-record
/// catalog, with its occurrence count.
fn catalog_kind_census() -> BTreeMap<String, usize> {
    let mut census: BTreeMap<String, usize> = BTreeMap::new();
    let visit = |token: &str, census: &mut BTreeMap<String, usize>| {
        if let Some(kind) = token_kind(token) {
            *census.entry(kind.to_owned()).or_default() += 1;
        }
    };

    for book in all_feat_tables() {
        for entry in book.entries {
            for token in entry.prerequisites.unwrap_or(&[]) {
                visit(token, &mut census);
                // Sub-clauses inside a `PREMULT:`'s brackets are real
                // prerequisites too; a kind that only ever appears nested
                // (`PREPROFWITHSHIELD`, `PREVARGT`) must still be accounted
                // for.
                for nested in nested_clauses(token) {
                    visit(&nested, &mut census);
                }
            }
        }
    }
    census
}

/// Pulls `PRE...` kinds out of `[...]` groups, at any nesting depth.
fn nested_clauses(token: &str) -> Vec<String> {
    let mut found = Vec::new();
    let bytes: Vec<char> = token.chars().collect();
    for (index, character) in bytes.iter().enumerate() {
        if *character != '[' {
            continue;
        }
        let tail: String = bytes[index + 1..].iter().collect();
        if let Some(kind_end) = tail.find(':') {
            found.push(tail[..=kind_end].to_owned());
        }
    }
    found
}

/// **The completeness guard.** Every kind the corpus actually uses is either
/// evaluated or explicitly listed as unmodelled with a reason. A book
/// ingested later that introduces a new kind fails here rather than having
/// its prerequisites silently ignored.
#[test]
fn every_pre_kind_in_the_catalog_is_either_modelled_or_declared_unmodelled() {
    let census = catalog_kind_census();
    assert!(!census.is_empty(), "the catalog must carry prerequisite tokens at all");

    let declared: BTreeSet<&str> = MODELLED_KINDS
        .iter()
        .copied()
        .chain(UNMODELLED_KINDS.iter().map(|(kind, _)| *kind))
        // `PRETEXT:` is PCGen display prose, handled as informational.
        .chain(std::iter::once("PRETEXT"))
        .collect();

    let mut unaccounted = Vec::new();
    for (kind, count) in &census {
        let bare = kind.trim_start_matches('!');
        if !declared.contains(bare) {
            unaccounted.push(format!("{kind} ({count} occurrences)"));
        }
    }
    assert!(
        unaccounted.is_empty(),
        "these PRE kinds occur in the catalog but no arm names them, so they would be \
         silently ignored: {unaccounted:?}"
    );
}

/// The census itself, pinned in full. Asserted as one whole map rather than
/// a handful of spot checks so a kind *appearing* is a failure too, not
/// only a count changing. These are the numbers the delivery report quotes.
#[test]
fn the_pre_kind_census_is_the_real_one() {
    let expected: BTreeMap<String, usize> = [
        // Full census, re-derived after `SD31-E6-F8-002`'s five-book feat
        // gap lane (2026-08-16) -- 3 new PRE kinds arrived
        // (`PRELEVELMAX`/`PRESIZEEQ`/`PRESPELLSCHOOL`, all declared
        // unmodelled in `pre_tokens.rs`), and every existing kind's count
        // moved with the 242 new gap rows' own `PRE`-family tokens.
        // `SD31-E6-F8-003` adds 7 more gap rows (inner_sea_intrigue 6 +
        // book_of_the_damned_volume_2 1), moving 6 kinds: PREABILITY +4
        // (2 top-level + 2 nested inside a `PREMULT` bracket clause),
        // PREDEITY +1, PREMULT +2, PRESKILL +7 (5 top-level + 2 nested),
        // PRETEXT +6, PREVARGTEQ +1 -- re-derived directly against each of
        // the 7 raw `.lst` rows, not guessed from the delta alone.
        // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
        // onboarding-precondition`, AT-32-G0-003) adds 9 more gap rows
        // (inner_sea_taverns), moving 4 kinds: PREABILITY +2 (`Hardy
        // Liver`, `Read the Room`'s own PREABILITY clause), PREDEITY +1
        // (`Drunken God's Blessings`, whose deity name was PI-redacted
        // in the record's own text but the `PREDEITY:` token itself
        // survives), PRESKILL +2 (`Drunken Sing-Along`'s and `Read the
        // Room`'s own PRESKILL token, one occurrence each regardless of
        // how many skills each token names), PREVARGTEQ +1 (`Tavern
        // Regular`'s `PreStatScore_CHA` clause) --
        // re-derived directly against each of the 9 raw `.lst` rows.
        // SD-32 T9 onboarding (card 11, `decisions.md §19` PI sign-off) adds
        // 109 more gap rows (inner_sea_combat 23 + inner_sea_gods 86, the
        // latter a deities book whose feats carry `PREDEITY:` heavily),
        // re-derived from this test's own RED-run assertion output against
        // the pinned oracle rather than hand-counted: PREABILITY +59,
        // PREALIGN +2, PRECLASS +5, PREDEITY +77, PREMULT +45, PREPCLEVEL
        // +2, PRESKILL +7, PRESPELLTYPE +6, PRESTAT +13, PRETEXT +108,
        // PRETOTALAB +22, PREVARGTEQ +103, PREWEAPONPROF +2.
        ("!PREABILITY", 35),
        ("!PREALIGN", 6),
        ("PREABILITY", 1663),
        ("PREALIGN", 28),
        ("PRECHECKBASE", 2),
        ("PRECLASS", 135),
        ("PREDEITY", 81),
        ("PREDEITYALIGN", 2),
        ("PREDOMAIN", 5),
        ("PREDR", 1),
        ("PREFACT", 243),
        ("PREHANDSGTEQ", 1),
        ("PREHD", 9),
        ("PRELEVEL", 40),
        ("PRELEVELMAX", 2),
        ("PREMOVE", 5),
        ("PREMULT", 432),
        ("PREPCLEVEL", 14),
        ("PREPROFWITHARMOR", 5),
        ("PREPROFWITHSHIELD", 9),
        ("PRERACE", 41),
        ("PRERULE", 4),
        ("PRESIZEEQ", 2),
        ("PRESIZEGTEQ", 3),
        ("PRESIZELTEQ", 7),
        ("PRESKILL", 353),
        ("PRESPELL", 38),
        ("PRESPELLCAST", 13),
        ("PRESPELLDESCRIPTOR", 2),
        ("PRESPELLSCHOOL", 1),
        ("PRESPELLSCHOOLSUB", 2),
        ("PRESPELLTYPE", 18),
        ("PRESTAT", 257),
        ("PRETEMPLATE", 34),
        ("PRETEXT", 260),
        ("PRETOTALAB", 390),
        ("PREVAREQ", 12),
        ("PREVARGT", 9),
        ("PREVARGTEQ", 1066),
        ("PREVARLT", 4),
        ("PREVARLTEQ", 2),
        ("PREVISION", 2),
        ("PREWEAPONPROF", 21),
    ]
    .into_iter()
    .map(|(kind, count)| (kind.to_owned(), count))
    .collect();

    assert_eq!(catalog_kind_census(), expected);

    // Prerequisite clauses across 41 distinct kinds (was 40/4,425 with
    // 4,103 modelled before `SD31-E6-F2-007`'s Mythic Adventures feat gap
    // lane; that cycle in turn was 37/3,914 with 3,713 modelled before
    // `SD31-E6-F8-002`'s five-book feat gap lane; that cycle in turn was
    // 35/3,805 before the original 83-row gap lane). `PRETEMPLATE` is the
    // one kind this cycle introduced (all 34 occurrences are Mythic's own
    // "Racial Heritage" feats' `PRETEMPLATE:1,Racial Heritage ~ <Race>`
    // clause), declared unmodelled with a stated reason rather than guessed
    // at (`pre_tokens.rs`'s `UNMODELLED_KINDS`) -- this engine has no
    // template system. The kind-by-kind values above are pinned
    // as-observed; see `MODELLED_KINDS`/`UNMODELLED_KINDS` for which of
    // them carry a real evaluation arm versus a named reason for staying
    // unmodelled, and `pre_tokens.rs`'s own `ClauseOutcome::Informational`
    // arm for `PRETEXT:`'s own third category. Total computed from the map
    // itself, not hand-summed (decisions.md §43's own lesson: a hand-summed
    // total was wrong once already this session).
    // `SD31-E6-F8-003`'s 7 new gap rows add 21 total clauses (PREABILITY
    // +4, PREDEITY +1, PREMULT +2, PRESKILL +7, PRETEXT +6, PREVARGTEQ +1),
    // 14 of which are modelled (all but PREDEITY's +1 and PRETEXT's +6,
    // neither in `MODELLED_KINDS`).
    // SD-32 Gate 0 book-onboarding precondition's 9 new gap rows
    // (inner_sea_taverns) add 6 total clauses (PREABILITY +2, PREDEITY +1,
    // PRESKILL +2, PREVARGTEQ +1), 5 of which are modelled (all but
    // PREDEITY's +1, not in `MODELLED_KINDS`).
    let total: usize = expected.values().sum();
    assert_eq!(total, 5259);
    let modelled: usize = expected
        .iter()
        .filter(|(kind, _)| MODELLED_KINDS.contains(&kind.trim_start_matches('!')))
        .map(|(_, count)| *count)
        .sum();
    assert_eq!(modelled, 4697);
}

/// 599 of the catalog's 690 records carry at least one prerequisite -- the
/// denominator the whole feature operates over.
#[test]
fn the_number_of_records_carrying_any_prerequisite_is_the_real_one() {
    let with_any: usize = all_feat_tables()
        .iter()
        .flat_map(|book| book.entries.iter())
        .filter(|entry| entry.prerequisites.is_some())
        .count();
    // 599 of the original 690 + all 23 UCA records (every one carries a
    // `PRETEXT:` prerequisite entry -- see `feats_all::UCA_FEAT_PREREQUISITES`)
    // + 98 of UI's 104 records (real `PRE`-family tokens, gathered directly
    // at ingest -- see `ultimate_intrigue::feat_tables`'s own doc comment).
    // + 63 of the original 83 corpus gap rows + 223 of `SD31-E6-F8-002`'s 242
    // more + 195 of `SD31-E6-F2-007`'s 199 Mythic Adventures rows, whose
    // `PRE`-family tokens the gap generator carries verbatim off the corpus
    // record. Unchanged by SD31-W10-INTEGRATE-001's exclusion of 159
    // VISIBLE:EXPORT twins -- every one of them carried zero `PRE` tokens,
    // so none was ever in this numerator; only the denominator moved.
    // + 7 of `SD31-E6-F8-003`'s 7 more gap rows (inner_sea_intrigue 6 +
    // book_of_the_damned_volume_2 1) -- all 7 carry a real `PRE`-family
    // token.
    // + 5 of SD-32 Gate 0 book-onboarding precondition's 9 inner_sea_taverns
    // rows (`Drunken God's Blessings`, `Drunken Sing-Along`, `Hardy Liver`,
    // `Read the Room`, `Tavern Regular`) carry a real `PRE`-family token.
    // + 108 of SD-32 T9 onboarding's (card 11) 109 new rows (inner_sea_combat
    // 23 + inner_sea_gods 86) carry a real `PRE`-family token -- re-derived
    // via `feats_all::tests::the_per_book_prerequisite_coverage_is_the_real_one`.
    assert_eq!(with_any, 2030, "of 2227");
}

// ---------------------------------------------------------------------------
// 2. The published-rulebook oracle
// ---------------------------------------------------------------------------

/// Verdicts checked by hand against the **printed** Pathfinder 1e Core
/// Rulebook prerequisites, not against this engine's own output, for a
/// Human Fighter 1 with Str 14 / Dex 13 / Int 10, no feats, no skill ranks.
///
/// `true` = the rulebook says this character qualifies.
const PUBLISHED_CRB_VERDICTS: &[(&str, bool, &str)] = &[
    ("Toughness", true, "no prerequisites"),
    ("Improved Initiative", true, "no prerequisites"),
    ("Alertness", true, "no prerequisites"),
    ("Acrobatic", true, "no prerequisites"),
    ("Stealthy", true, "no prerequisites"),
    ("Run", true, "no prerequisites"),
    ("Iron Will", true, "no prerequisites"),
    ("Blind-Fight", true, "no prerequisites"),
    ("Endurance", true, "no prerequisites"),
    ("Skill Focus", true, "no prerequisites"),
    ("Power Attack", true, "Str 13, BAB +1 -- has Str 14, BAB +1"),
    ("Deadly Aim", true, "Dex 13, BAB +1 -- has Dex 13, BAB +1"),
    ("Dodge", true, "Dex 13 -- has Dex 13"),
    ("Point-Blank Shot", true, "no prerequisites"),
    ("Weapon Focus", true, "proficiency + BAB +1 -- a Fighter 1 has both"),
    ("Cleave", false, "requires Power Attack, which this character has not taken"),
    ("Great Cleave", false, "requires Cleave, Power Attack and BAB +4"),
    ("Mobility", false, "requires Dodge"),
    ("Spring Attack", false, "requires Dodge, Mobility and BAB +4"),
    ("Precise Shot", false, "requires Point-Blank Shot"),
    ("Rapid Shot", false, "requires Point-Blank Shot"),
    ("Diehard", false, "requires Endurance"),
    ("Two-Weapon Fighting", false, "requires Dex 15; this character has Dex 13"),
    (
        "Improved Two-Weapon Fighting",
        false,
        "requires Two-Weapon Fighting, Dex 17 and BAB +6 -- the operator's reported defect",
    ),
    ("Improved Critical", false, "requires BAB +8"),
];

#[test]
fn the_verdicts_match_the_published_core_rulebook_for_a_starting_fighter() {
    let facts = fighter_1();
    let mut wrong = Vec::new();
    for (feat_key, published_eligible, published_reason) in PUBLISHED_CRB_VERDICTS {
        let report = evaluate_feat_key_prerequisites(feat_key, &facts)
            .unwrap_or_else(|| panic!("'{feat_key}' must be in the catalog"));
        if report.is_eligible != *published_eligible {
            wrong.push(format!(
                "'{feat_key}': engine says eligible={}, rulebook says {published_eligible} \
                 ({published_reason}); engine's unmet = {:?}",
                report.is_eligible,
                report.unmet.iter().map(|u| u.reason.clone()).collect::<Vec<_>>()
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// The operator's exact scenario, both directions, with the reason text a
/// player actually sees.
#[test]
fn the_reported_defect_is_closed_in_both_directions() {
    let denied = evaluate_feat_key_prerequisites("Improved Two-Weapon Fighting", &fighter_1())
        .expect("catalog record");
    assert!(!denied.is_eligible);
    let reason = denied.unavailable_reason().expect("a denial must state why");
    assert!(reason.contains("base attack bonus +6"), "{reason}");
    assert!(reason.contains("Two-Weapon Fighting"), "{reason}");
    assert!(reason.contains("DEX 17"), "{reason}");

    let qualified = build(
        "race:human",
        "class:fighter",
        6,
        AbilityScores { dexterity: 17, ..starting_scores() },
        &["Two-Weapon Fighting"],
        &[],
    );
    let allowed = evaluate_feat_key_prerequisites(
        "Improved Two-Weapon Fighting",
        &character_prereq_facts(&qualified, 6),
    )
    .expect("catalog record");
    assert!(allowed.is_eligible, "unmet: {:?}", allowed.unmet);
    assert_eq!(allowed.unavailable_reason(), None);
}

/// Both real `selected_feats` id shapes must satisfy a prerequisite. A
/// character created by the app is seeded with `feat:snake_case` tokens
/// while the picker appends catalog keys, and a checker that only
/// understood one shape would deny feats a player has legitimately earned.
#[test]
fn both_selected_feat_id_shapes_satisfy_a_feat_prerequisite() {
    for shape in ["Power Attack", "feat:power_attack"] {
        let input = build(
            "race:human",
            "class:fighter",
            1,
            starting_scores(),
            &[shape],
            &[],
        );
        let report = evaluate_feat_key_prerequisites("Cleave", &character_prereq_facts(&input, 1))
            .expect("catalog record");
        assert!(report.is_eligible, "'{shape}' should satisfy Cleave's Power Attack requirement");
    }
}

/// Skill-rank prerequisites read the character's real allocations.
#[test]
fn allocating_the_required_skill_ranks_unlocks_a_skill_gated_feat() {
    let without = build("race:human", "class:fighter", 3, starting_scores(), &[], &[]);
    let blocked = evaluate_feat_key_prerequisites(
        "Acrobatic Steps",
        &character_prereq_facts(&without, 3),
    );
    // Only assert the mechanism on a feat that really is skill-gated; find
    // one from the live catalog rather than trusting a hand-picked name.
    let facts = character_prereq_facts(&without, 3);
    let skill_gated: Vec<String> = evaluate_every_catalog_feat(&facts)
        .into_iter()
        .filter(|report| {
            report.unmet.iter().any(|unmet| unmet.reason.contains("rank(s) in"))
        })
        .map(|report| report.feat_key)
        .collect();
    assert!(
        !skill_gated.is_empty(),
        "the catalog really does contain skill-gated feats; if this is empty the \
         PRESKILL arm is not running at all"
    );
    let _ = blocked;

    // Nimble Moves: 'Dex 13' only -- but Acrobatic Steps needs Nimble Moves.
    // Use a genuinely rank-gated CRB feat instead: Stunning Fist is class
    // gated, so take the first live skill-gated record and satisfy it.
    let target = &skill_gated[0];
    let record = all_feat_tables()
        .iter()
        .flat_map(|book| book.entries.iter())
        .find(|entry| entry.key == target)
        .expect("the key came from the catalog");
    let requirement = record
        .prerequisites
        .unwrap_or(&[])
        .iter()
        .find(|token| token.starts_with("PRESKILL:"))
        .expect("it was blocked on a PRESKILL clause");

    // `PRESKILL:1,Acrobatics=5` -> allocate 5 ranks in Acrobatics.
    let body = requirement.split_once(':').unwrap().1;
    let entry = body.split(',').nth(1).expect("a skill entry");
    let (skill_name, ranks) = entry.rsplit_once('=').expect("<Skill>=<ranks>");
    let skill_id = format!("skill:{}", skill_name.to_lowercase().replace(' ', "_"));
    let ranks: u8 = ranks.parse().expect("a rank count");

    let with = build(
        "race:human",
        "class:fighter",
        3,
        starting_scores(),
        &[],
        &[(skill_id.as_str(), ranks)],
    );
    let report =
        evaluate_feat_key_prerequisites(target, &character_prereq_facts(&with, 3)).unwrap();
    assert!(
        !report.unmet.iter().any(|unmet| unmet.reason.contains("rank(s) in")),
        "allocating {ranks} ranks in {skill_name} must clear '{target}'s skill clause, \
         still unmet: {:?}",
        report.unmet
    );
}

// ---------------------------------------------------------------------------
// 3. Honesty properties
// ---------------------------------------------------------------------------

/// No silent denials, anywhere, for any of five very different builds.
#[test]
fn every_ineligible_feat_states_a_reason_for_every_build() {
    let builds = [
        build("race:human", "class:fighter", 1, starting_scores(), &[], &[]),
        build("race:tiefling", "class:wizard", 5, starting_scores(), &[], &[]),
        build("race:halfling", "class:rogue", 11, starting_scores(), &[], &[]),
        build("race:duergar", "class:barbarian", 20, starting_scores(), &[], &[]),
        build("race:half-orc", "class:unchained_rogue", 8, starting_scores(), &[], &[]),
    ];
    for input in &builds {
        let level = input.chosen.class_levels[0].level;
        let facts = character_prereq_facts(input, i16::from(level));
        let reports = evaluate_every_catalog_feat(&facts);
        assert_eq!(reports.len(), 2227);
        for report in &reports {
            if report.is_eligible {
                assert_eq!(report.unavailable_reason(), None);
            } else {
                let reason = report.unavailable_reason().unwrap_or_default();
                assert!(
                    !reason.trim().is_empty(),
                    "{} / '{}' is unavailable with no reason",
                    input.chosen.race_id,
                    report.feat_key
                );
            }
        }
        // ...and no build is left with a dead catalog.
        let eligible = reports.iter().filter(|report| report.is_eligible).count();
        assert!(
            eligible > 100,
            "{} {} L{level} has only {eligible} feats available -- that is a dead catalog, \
             not enforcement",
            input.chosen.race_id,
            input.chosen.class_levels[0].class_id,
        );
    }
}

/// A clause this engine cannot evaluate must never block. Checked over the
/// whole catalog: if any record's *only* problem were an unmodelled clause,
/// it would still be offered.
#[test]
fn unverifiable_clauses_never_block_and_are_always_reported() {
    let facts = fighter_1();
    let reports = evaluate_every_catalog_feat(&facts);

    let with_notes = reports.iter().filter(|report| !report.unverified.is_empty()).count();
    assert!(
        with_notes > 0,
        "the catalog really does contain clauses this engine cannot evaluate; zero here \
         would mean the unmodelled path is dead code"
    );

    for report in &reports {
        for note in &report.unverified {
            assert!(
                note.message.starts_with("not verified: "),
                "an unverified note must say so plainly: {}",
                note.message
            );
            assert!(
                note.message.contains("PRE"),
                "an unverified note must name the token it could not evaluate: {}",
                note.message
            );
        }
        if report.unmet.is_empty() {
            assert!(
                report.is_eligible,
                "'{}' has no unmet clause but was denied -- an unverifiable clause must \
                 not block",
                report.feat_key
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 4. Corpus-gated re-derivations
// ---------------------------------------------------------------------------

fn corpus_root() -> Option<PathBuf> {
    let path = PathBuf::from(std::env::var("PCGEN_CORPUS_ROOT").ok()?);
    path.is_dir().then_some(path)
}

fn book_dir(root: &std::path::Path, book: &str) -> PathBuf {
    root.join("pathfinder/paizo/roleplaying_game").join(book)
}

/// Re-parses a `*_feats.lst` into `record identity -> top-level PRE tokens`,
/// deliberately not sharing code with the offline generator so a drift
/// between this file's table and the corpus actually fails.
fn parse_feat_lst(path: &std::path::Path) -> BTreeMap<String, Vec<String>> {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()));
    let mut rows: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in text.lines() {
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t');
        let Some(name) = fields.next().map(str::trim) else { continue };
        if name.is_empty() {
            continue;
        }
        let fields: Vec<&str> = fields.collect();
        let key = fields
            .iter()
            .find_map(|field| field.strip_prefix("KEY:"))
            .map(str::trim)
            .unwrap_or(name);
        let tokens: Vec<String> = fields
            .iter()
            .map(|field| field.trim())
            .filter(|field| {
                let bare = field.trim_start_matches('!');
                bare.starts_with("PRE") && bare.contains(':')
            })
            .map(|field| field.to_string())
            .collect();
        rows.entry(key.to_owned()).or_insert(tokens);
    }
    rows
}

/// The gathered ARG and PU tokens must be exactly what the corpus says.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn the_gathered_arg_and_pu_prerequisites_match_the_live_corpus() {
    let root = corpus_root().expect("PCGEN_CORPUS_ROOT must point at a pcgen/data checkout");
    let books = [
        ("Arg", book_dir(&root, "advanced_race_guide").join("arg_feats.lst")),
        ("Pu", book_dir(&root, "pathfinder_unchained").join("pu_feats.lst")),
    ];

    let mut mismatches = Vec::new();
    for (book_name, path) in books {
        let corpus = parse_feat_lst(&path);
        let table = all_feat_tables()
            .iter()
            .find(|book| format!("{:?}", book.rule_set) == book_name)
            .expect("book is in the aggregate");
        for entry in table.entries {
            let expected: &[String] = corpus
                .get(entry.key)
                .unwrap_or_else(|| panic!("{book_name} '{}' is not in {}", entry.key, path.display()));
            let actual: Vec<String> = entry
                .prerequisites
                .unwrap_or(&[])
                .iter()
                .map(|token| (*token).to_owned())
                .collect();
            if actual != expected {
                mismatches.push(format!(
                    "{book_name} '{}':\n  table:  {actual:?}\n  corpus: {expected:?}",
                    entry.key
                ));
            }
        }
    }
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

/// `FeatDexRequirement` is modelled as 0 for every character this product
/// can build. That is only honest while no ingested book raises it.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn feat_dex_requirement_is_never_raised_by_any_ingested_book() {
    let root = corpus_root().expect("PCGEN_CORPUS_ROOT must point at a pcgen/data checkout");
    let mut offenders = Vec::new();
    for book in [
        "core_rulebook",
        "advanced_players_guide",
        "advanced_class_guide",
        "advanced_race_guide",
        "pathfinder_unchained",
        "bestiary",
    ] {
        let directory = book_dir(&root, book);
        for path in walk_lst_files(&directory) {
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            if text.contains("BONUS:VAR|FeatDexRequirement") {
                offenders.push(path.display().to_string());
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "pre_tokens models FeatDexRequirement as 0 because nothing ingested raises it; \
         these files now do, so the model is wrong: {offenders:?}"
    );
}

/// The hand-modelled race subtype table must be what the corpus's race
/// templates say. Only checks the subtypes the catalog's `PRERACE:` tokens
/// actually ask about -- those are the ones a wrong value would misjudge.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn race_subtypes_match_the_corpus_race_templates() {
    let root = corpus_root().expect("PCGEN_CORPUS_ROOT must point at a pcgen/data checkout");
    let essentials = book_dir(&root, "core_essentials").join("races");

    // (race token, corpus template directory, expected subtypes)
    let expectations: &[(&str, &str, &[&str])] = &[
        ("race:dwarf", "dwarf", &["Dwarf"]),
        ("race:elf", "elf", &["Elf"]),
        ("race:gnome", "gnome", &["Gnome"]),
        ("race:halfling", "halfling", &["Halfling"]),
        ("race:human", "human", &["Human"]),
        ("race:drow", "drow", &["Elf"]),
        ("race:duergar", "duergar", &["Dwarf"]),
        ("race:svirfneblin", "svirfneblin", &["Gnome"]),
        ("race:orc", "orc", &["Orc"]),
    ];

    // Which template rows anywhere under core_essentials grant which
    // `RACESUBTYPE:`. The subtype is never on the race row itself; PCGen
    // applies a named template that carries it, which is exactly why a
    // Duergar (`TEMPLATE:Humanoid|Dwarf`) has the Dwarf subtype without the
    // word appearing on its own race record.
    let mut subtype_granting_templates: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for path in walk_lst_files(&essentials) {
        if !path.to_string_lossy().contains("_templates") {
            continue;
        }
        for line in std::fs::read_to_string(&path).unwrap_or_default().lines() {
            if line.trim().is_empty() || line.trim_start().starts_with('#') {
                continue;
            }
            let mut fields = line.split('\t');
            let Some(row_name) = fields.next().map(str::trim) else { continue };
            for field in fields {
                if let Some(value) = field.trim().strip_prefix("RACESUBTYPE:") {
                    subtype_granting_templates
                        .entry(row_name.to_owned())
                        .or_default()
                        .insert(value.trim().to_owned());
                }
            }
        }
    }

    for (race_token, directory, expected) in expectations {
        // The engine's own answer, via a real prerequisite evaluation.
        let input = build(race_token, "class:fighter", 1, starting_scores(), &[], &[]);
        let facts = character_prereq_facts(&input, 1);

        // Every template name this race's own corpus files apply.
        let race_dir = essentials.join(directory);
        let mut applied_templates = BTreeSet::new();
        for path in walk_lst_files(&race_dir) {
            for line in std::fs::read_to_string(&path).unwrap_or_default().lines() {
                for field in line.split('\t') {
                    if let Some(value) = field.trim().strip_prefix("TEMPLATE:") {
                        for name in value.split('|') {
                            applied_templates.insert(name.trim().to_owned());
                        }
                    }
                }
            }
        }

        for subtype in *expected {
            // Step 1: the corpus really has a template granting this
            // subtype...
            let granted = subtype_granting_templates
                .get(*subtype)
                .unwrap_or_else(|| panic!("no core_essentials template row named {subtype}"));
            assert!(
                granted.contains(*subtype),
                "the {subtype} template must carry RACESUBTYPE:{subtype}, carries {granted:?}"
            );
            // Step 2: ...and this race applies it.
            assert!(
                applied_templates.contains(*subtype),
                "{race_token}: nothing under {} applies TEMPLATE:{subtype} \
                 (applies {applied_templates:?})",
                race_dir.display()
            );
            // Step 3: ...and the engine agrees, checked through the real
            // evaluation path rather than by reading the table.
            let token = format!("PRERACE:1,RACESUBTYPE={subtype}");
            let outcome =
                codex::rules_core::feat_prereqs::pre_tokens::evaluate_prerequisite_token(
                    &token, &facts,
                );
            assert!(
                matches!(
                    outcome,
                    codex::rules_core::feat_prereqs::pre_tokens::ClauseOutcome::Met { .. }
                ),
                "{race_token} must satisfy {token}, got {outcome:?}"
            );
        }
    }
}

fn walk_lst_files(directory: &std::path::Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(directory) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(walk_lst_files(&path));
        } else if path.extension().is_some_and(|extension| extension == "lst") {
            found.push(path);
        }
    }
    found
}
