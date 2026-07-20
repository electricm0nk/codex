//! SD-22 Epic 6 acceptance test — DM-toolkit happy-path integration
//! (criterion 21): `epic-breakdown.md` criterion 21, "DM-toolkit consumes
//! ingested content in a happy-path integration test: a campaign-shaped
//! fixture (PartySnapshot) + a monster-block fixture (MonsterRef) →
//! EncounterResult → assertion against the canonical Paizo encounter-table
//! result." This is Epic 6's fourth and last cycle, and the last remaining
//! SD-22 criterion in Epic 6 (criteria 18-20 already landed: `Encounter::new`,
//! `party_challenge_rating`, and the deterministic tests).
//!
//! Per `corpus-source-inventory.md` §4 ("DM Toolkit — Epic 6"), the
//! happy-path integration test's required corpus input is "One ingested
//! `PartySnapshot` + one ingested `MonsterRef` from Epic 3+4+5's first
//! cycles." This bundle's actual party-side type is `CharacterSnapshot`
//! (`src/rules_core/encounters.rs`, criterion 18) — there is no separate
//! `PartySnapshot` type anywhere in the codebase; `epic-breakdown.md`'s
//! prose name and the shipped type name differ, same as `MonsterRef` is
//! the shipped consumer-side type name, not the phrase "monster-block
//! fixture" itself. This test builds the party with `CharacterSnapshot`,
//! per the same established precedent criteria 18-20 already used.
//!
//! ## What "ingested content" means for this test
//!
//! The monster half must be pulled from Epic 5's already-landed Bestiary 1
//! output (`src/rules_core/rules_tables/beastiary1/`), not a synthetic
//! `MonsterRef::new(cr)` literal like criteria 18-20's own unit/acceptance
//! tests use — those tests exercise `encounters.rs` in isolation; this test
//! is specifically the integration point proving Epic 6 can consume real
//! Epic 5 output. This test resolves the real Ghoul monster stat block via
//! `beastiary1::monster_resolve(MonsterId::Ghoul, RuleSetId::Bestiary1)`
//! (`src/rules_core/rules_tables/beastiary1/monster_subset_01.rs`'s
//! `ghoul()` function, transcribed from the real corpus row
//! `b1_races.lst:200`, `CR:1`, cited in that function's own doc comment)
//! rather than fabricating a monster.
//!
//! ## Type-shape check (per `loop-instruction.md`'s anticipated integration
//! gap)
//!
//! `beastiary1::MonsterStatBlock` (Epic 5's richer per-monster type: name,
//! CR, size, speed, race type/subtype, source page, natural attacks) and
//! `encounters::MonsterRef` (Epic 6's minimal encounter-math input: just
//! `challenge_rating: f32`) are, as expected, two distinct types — Epic 5
//! ships the full Bestiary 1 stat-block shape; Epic 6 (per its own module
//! doc comment, "a later cycle... is where the two get reconciled") only
//! ever needed the one field the grounded encounter-difficulty formula
//! consumes. Investigated whether this is a genuine integration gap
//! (`loop-instruction.md`'s "type mismatch" concern) requiring a production
//! code change: it is not — `MonsterStatBlock::challenge_rating` is a
//! public `f32` field and `MonsterRef::new` is a public `f32`-argument
//! constructor, so extracting the one field Epic 6 needs from the richer
//! Epic 5 type is a direct, lossless field read
//! (`MonsterRef::new(stat_block.challenge_rating)`), not a conversion that
//! needs new glue code, a `From` impl, or a schema change. No production
//! code changes ship with this cycle.
//!
//! ## The canonical Paizo encounter-table result asserted
//!
//! Party: 1 level-1 PC (`CharacterSnapshot::new(1)`). Monster: the real
//! Ghoul (`challenge_rating: 1.0`, `b1_races.lst:200`). This is the exact
//! same party/monster shape as `corpus-source-inventory.md` §4.1 case 5
//! ("1 level-1 PC vs 1 CR-1 monster") and `encounters.rs`'s own
//! `encounter_1_level_1_pc_vs_1_cr_1_monster_returns_valid_difficulty` /
//! `tests/sd22_dm_toolkit_deterministic.rs`'s
//! `encounters_1_level_1_pc_vs_1_cr_1_monster_returns_valid_difficulty` —
//! both already independently verified against the PF1 Core Rulebook's
//! "Gamemastering" chapter (Table: Encounter Design, Table: CR
//! Equivalencies): APL 1, group EL 1 (1 creature = its own CR per Table:
//! CR Equivalencies), EL − APL = 0 ⇒ `Difficulty::Medium` per Table:
//! Encounter Design's `Average = APL` row (this bundle's 4-tier collapse
//! maps the rulebook's `Average` tier to `Difficulty::Medium` — see
//! `encounters.rs`'s module doc comment, "Difficulty tier collapse"
//! section). This test re-derives the identical canonical math using the
//! real ingested Ghoul's CR instead of a synthetic `MonsterRef::new(1.0)`
//! literal, confirming the two are the same value end to end.

use codex::rules_core::encounters::{CharacterSnapshot, Difficulty, Encounter, MonsterRef};
use codex::rules_core::rules_tables::beastiary1::{monster_resolve, MonsterId};
use codex::rules_core::rules_tables::RuleSetId;

/// The happy-path integration test itself: build a party fixture, resolve
/// a real ingested monster-block fixture, feed both into `Encounter::new`,
/// and assert the result against the canonical Paizo encounter-table
/// derivation (see module doc comment for the full grounded math).
#[test]
fn happy_path_1_level_1_pc_vs_ingested_ghoul_is_medium_per_grounded_pf1_math() {
    // 1. Campaign-shaped party fixture.
    let party = vec![CharacterSnapshot::new(1)];

    // 2. Real ingested monster-block fixture, pulled from Epic 5's
    //    already-landed Bestiary 1 output via its book-scoped resolver —
    //    not a synthetic literal.
    let ghoul_stat_block = monster_resolve(MonsterId::Ghoul, RuleSetId::Bestiary1)
        .expect("Ghoul must resolve via RuleSetId::Bestiary1 — Epic 5 subset 01 already landed it");
    assert_eq!(ghoul_stat_block.name, "Ghoul");
    assert_eq!(ghoul_stat_block.challenge_rating, 1.0);

    // Cross-book invariant sanity check on the ingested fixture itself
    // (`corpus-source-inventory.md` §3.2): a Bestiary 1 monster must not
    // resolve under a different book's rule set.
    assert!(monster_resolve(MonsterId::Ghoul, RuleSetId::Crb).is_none());
    assert!(monster_resolve(MonsterId::Ghoul, RuleSetId::Apg).is_none());
    assert!(monster_resolve(MonsterId::Ghoul, RuleSetId::Acg).is_none());

    // 3. Reconcile the two types by direct field extraction (see module
    //    doc comment's "Type-shape check" section for why no production
    //    code change is needed here).
    let monsters = vec![MonsterRef::new(ghoul_stat_block.challenge_rating)];

    // 4. Feed both into the Epic 6 DM-toolkit encounter-math surface.
    let result = Encounter::new(&party, &monsters);

    // 5. Assert against the canonical Paizo encounter-table result.
    assert_eq!(result.average_party_level, 1);
    assert_eq!(result.encounter_level, 1);
    assert_eq!(result.difficulty, Difficulty::Medium);
}

/// Sibling-preservation / breadth check: the same happy-path shape holds
/// for a second real ingested monster from a different Epic 5 subset
/// (Darkmantle, subset 02, `challenge_rating: 1.0`, `b1_races.lst:91`),
/// against a larger canonical party (`corpus-source-inventory.md` §4.1
/// case 1's party shape: 4 level-3 PCs). A single CR-1 monster is group EL
/// 1 (Table: CR Equivalencies, 1 creature = its own CR); APL 3; EL − APL =
/// 1 − 3 = −2, which is `<= -1` ⇒ `Difficulty::Easy` per Table: Encounter
/// Design's `Easy = APL-1` row (same grounded formula `encounters.rs`
/// already implements and `sd22_dm_toolkit_deterministic.rs`'s case 1
/// exercises, here fed a real ingested monster instead of a synthetic
/// `MonsterRef::new` literal).
#[test]
fn happy_path_4_level_3_pcs_vs_ingested_darkmantle_is_easy_per_grounded_pf1_math() {
    let party: Vec<CharacterSnapshot> = (0..4).map(|_| CharacterSnapshot::new(3)).collect();

    let darkmantle_stat_block = monster_resolve(MonsterId::Darkmantle, RuleSetId::Bestiary1)
        .expect("Darkmantle must resolve via RuleSetId::Bestiary1 — Epic 5 subset 02 already landed it");
    assert_eq!(darkmantle_stat_block.name, "Darkmantle");
    assert_eq!(darkmantle_stat_block.challenge_rating, 1.0);

    let monsters = vec![MonsterRef::new(darkmantle_stat_block.challenge_rating)];
    let result = Encounter::new(&party, &monsters);

    assert_eq!(result.average_party_level, 3);
    assert_eq!(result.encounter_level, 1);
    assert_eq!(result.difficulty, Difficulty::Easy);
}
