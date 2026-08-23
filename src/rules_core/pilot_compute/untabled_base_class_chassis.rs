//! Base-attack-bonus / base-save chassis for the **18-real-base-classes-
//! without-tables** half of SD-32 Epic 3 (`epic-3-class-reachability`,
//! `acceptance-and-verification.md` AT-32-E3-001): "The 18 real base
//! classes without tables ... feed this epic from Epic 4." The first
//! AT-32-E3-001 cycle closed the prestige-entry-gating half only
//! (`prestige_class_entry_gate.rs`) and explicitly deferred this half
//! (`docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/003_cycle_receipt.md`,
//! "Deferred, not silently dropped"); `decisions.md §10` reopened it.
//!
//! # Population: 20, not 18 -- corrected, not silently substituted
//!
//! `epic-breakdown.md` and `acceptance-and-verification.md` both quote
//! **18**. `scripts/census_untabled_base_classes.py` -- this module's own
//! re-derive command, run against the pinned oracle -- finds **20** real
//! (`TYPE:Base.PC`, not `NPC`, not `Prestige`, not `Ex-*`) base classes
//! whose source book this repo has actually ingested and that no existing
//! `compute_class_chassis` dispatch arm (CRB's `table_class_id`, APG, ACG,
//! Unchained, Ultimate Combat) recognizes: Aegis, Antipaladin, Cryptic,
//! Dread, Kineticist, Magus, Marksman, Medium, Mesmerist, Occultist, Psion,
//! Psychic, Psychic Warrior, Shifter, Soulknife, Spiritualist, Tactician,
//! Vigilante, Vitalist, Wilder. Logged as `scripts/retro.py correction`
//! against the 18 figure; see this card's cycle receipt for the id.
//!
//! # Method: reuse the CRB table's own formulas, source new metadata
//!
//! `rules_tables::crb::class_tables` already carries the two formulas every
//! PF1e base class chassis uses (`base_attack_bonus`/`save_bonus` for
//! Full/ThreeQuarter/Half BAB and good/poor saves) -- verified by SD-18's
//! test suite. Nothing about those formulas is CRB-specific; what differs
//! per class is only which BAB progression and which three save
//! classifications apply, plus hit die and level ceiling. This module reads
//! that per-class metadata from a corpus-derived fixture
//! (`tests/fixtures/rules_core/untabled-base-class-chassis.json`,
//! `include_str!`-embedded, same shape as `prestige_class_entry_gate.rs`'s
//! own fixture) and re-runs the CRB table's own two formula functions
//! against it, rather than re-deriving or hand-transcribing a second copy
//! of either formula.

use std::sync::OnceLock;

use super::super::rules_tables::crb::class_tables::{base_attack_bonus, save_bonus, BabProgression};

/// One untabled base class's corpus-derived chassis metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntabledBaseClassMeta {
    pub class_id: String,
    pub display_name: String,
    pub source_book: String,
    pub source_file: String,
    pub hit_die: u8,
    pub max_level: u8,
    pub(crate) bab_progression: BabProgression,
    pub good_saves: (bool, bool, bool), // (fortitude, reflex, will)
}

#[derive(serde::Deserialize)]
struct FixtureFile {
    entries: Vec<FixtureEntry>,
}

#[derive(serde::Deserialize)]
struct FixtureGoodSaves {
    fortitude: bool,
    reflex: bool,
    will: bool,
}

#[derive(serde::Deserialize)]
struct FixtureEntry {
    class_id: String,
    display_name: String,
    source_book: String,
    source_file: String,
    hit_die: u8,
    max_level: u8,
    bab_progression: String,
    good_saves: FixtureGoodSaves,
}

const FIXTURE_JSON: &str =
    include_str!("../../../tests/fixtures/rules_core/untabled-base-class-chassis.json");

static REGISTRY: OnceLock<Vec<UntabledBaseClassMeta>> = OnceLock::new();

/// The full registry, parsed once from the committed fixture
/// (`scripts/census_untabled_base_classes.py`'s output). Panics on a
/// malformed fixture or an unrecognized `bab_progression` string -- a
/// corrupt committed fixture is a build defect, not a runtime condition any
/// caller can recover from.
pub fn untabled_base_class_registry() -> &'static [UntabledBaseClassMeta] {
    REGISTRY
        .get_or_init(|| {
            let parsed: FixtureFile = serde_json::from_str(FIXTURE_JSON)
                .expect("tests/fixtures/rules_core/untabled-base-class-chassis.json must parse");
            parsed
                .entries
                .into_iter()
                .map(|e| UntabledBaseClassMeta {
                    class_id: e.class_id.clone(),
                    display_name: e.display_name,
                    source_book: e.source_book,
                    source_file: e.source_file,
                    hit_die: e.hit_die,
                    max_level: e.max_level,
                    bab_progression: match e.bab_progression.as_str() {
                        "Full" => BabProgression::Full,
                        "ThreeQuarter" => BabProgression::ThreeQuarter,
                        "Half" => BabProgression::Half,
                        other => panic!(
                            "untabled-base-class-chassis.json: unrecognized bab_progression \
                             {other:?} for {}",
                            e.class_id
                        ),
                    },
                    good_saves: (e.good_saves.fortitude, e.good_saves.reflex, e.good_saves.will),
                })
                .collect()
        })
        .as_slice()
}

fn find_by_class_id(class_id_str: &str) -> Option<&'static UntabledBaseClassMeta> {
    untabled_base_class_registry()
        .iter()
        .find(|meta| meta.class_id == class_id_str)
}

/// One row: `class_id_str`'s base attack bonus and three base saves at
/// `level`, computed from the registry's corpus-derived metadata using the
/// same `base_attack_bonus`/`save_bonus` formulas
/// `rules_tables::crb::class_tables` already carries. `None` when
/// `class_id_str` names no class this registry covers, or when `level`
/// exceeds that class's own corpus `MAXLEVEL` ceiling.
pub struct UntabledBaseClassRow {
    pub display_name: String,
    pub hit_die: u8,
    pub base_attack_bonus: i16,
    pub fort_save: i16,
    pub ref_save: i16,
    pub will_save: i16,
}

pub fn resolve(class_id_str: &str, level: u8) -> Option<UntabledBaseClassRow> {
    let meta = find_by_class_id(class_id_str)?;
    if level == 0 || level > meta.max_level {
        return None;
    }
    let (good_fort, good_ref, good_will) = meta.good_saves;
    Some(UntabledBaseClassRow {
        display_name: meta.display_name.clone(),
        hit_die: meta.hit_die,
        base_attack_bonus: base_attack_bonus(meta.bab_progression, level),
        fort_save: save_bonus(level, good_fort),
        ref_save: save_bonus(level, good_ref),
        will_save: save_bonus(level, good_will),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_loads_all_20_corpus_derived_entries() {
        let registry = untabled_base_class_registry();
        assert_eq!(
            registry.len(),
            20,
            "scripts/census_untabled_base_classes.py's own pinned-oracle run found 20 real \
             untabled base classes; a different count here means the fixture drifted from the \
             script's output without re-running it"
        );
    }

    #[test]
    fn registry_has_no_overlap_with_any_already_dispatched_class_id() {
        // Mirrors `compute_class_chassis`'s own dispatch chain: none of
        // this registry's class ids may collide with a CRB/APG/ACG/PU/UC
        // class id, or this module's dispatch arm would shadow (or be
        // shadowed by) an existing one.
        let already_dispatched = [
            "class:fighter", "class:wizard", "class:rogue", "class:ranger",
            "class:paladin", "class:sorcerer", "class:cleric", "class:druid",
            "class:barbarian", "class:bard", "class:monk",
            "class:alchemist", "class:cavalier", "class:inquisitor", "class:oracle",
            "class:summoner", "class:witch",
            "class:arcanist", "class:bloodrager", "class:brawler", "class:hunter",
            "class:investigator", "class:shaman", "class:skald", "class:slayer",
            "class:swashbuckler", "class:warpriest",
            "class:gunslinger", "class:ninja", "class:samurai",
            "class:unchained_barbarian", "class:unchained_monk",
            "class:unchained_rogue", "class:unchained_summoner",
        ];
        for meta in untabled_base_class_registry() {
            assert!(
                !already_dispatched.contains(&meta.class_id.as_str()),
                "{} collides with an already-dispatched class id",
                meta.class_id
            );
        }
    }

    #[test]
    fn unknown_class_id_resolves_to_none() {
        assert!(resolve("class:not_a_real_class", 5).is_none());
    }

    #[test]
    fn level_zero_resolves_to_none() {
        assert!(resolve("class:kineticist", 0).is_none());
    }

    #[test]
    fn level_beyond_max_level_resolves_to_none() {
        assert!(resolve("class:kineticist", 21).is_none());
    }

    /// Fixture-checked against bytes `resolve` never reads: the oracle's
    /// own `up_classes.lst` `CLASS:Psion` record (`HD:6`,
    /// `BONUS:COMBAT|BASEAB|classlevel(...)/2`, `BONUS:SAVE|BASE.Will|.../2+2`,
    /// `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|.../3`) hand-transcribed here
    /// from the corpus text this cycle's own probe read, not from this
    /// module's own fixture file.
    #[test]
    fn psion_level_10_matches_the_oracle_s_half_bab_good_will_poor_fort_reflex() {
        let row = resolve("class:psion", 10).expect("class:psion must resolve at level 10");
        assert_eq!(row.hit_die, 6);
        assert_eq!(row.base_attack_bonus, 5); // Half BAB: 10/2
        assert_eq!(row.fort_save, 3); // poor: 10/3
        assert_eq!(row.ref_save, 3); // poor: 10/3
        assert_eq!(row.will_save, 7); // good: 10/2+2
    }

    /// Fixture-checked against the oracle's `CLASS:Kineticist` record
    /// (`HD:8`, `*3/4` BAB, Fortitude and Reflex good, Will poor).
    #[test]
    fn kineticist_level_20_matches_the_oracle_s_three_quarter_bab_fort_reflex_good_will_poor() {
        let row = resolve("class:kineticist", 20).expect("class:kineticist must resolve at level 20");
        assert_eq!(row.hit_die, 8);
        assert_eq!(row.base_attack_bonus, 15); // ThreeQuarter BAB: (20*3)/4
        assert_eq!(row.fort_save, 12); // good: 20/2+2
        assert_eq!(row.ref_save, 12); // good: 20/2+2
        assert_eq!(row.will_save, 6); // poor: 20/3
    }

    /// Fixture-checked against the oracle's `CLASS:Antipaladin` record
    /// (`HD:10`, full BAB, Fortitude and Will good, Reflex poor) -- proves
    /// the registry does NOT silently reuse Paladin's CRB row even though
    /// the numeric progression happens to match it.
    #[test]
    fn antipaladin_level_1_matches_the_oracle_s_full_bab_fort_will_good_reflex_poor() {
        let row = resolve("class:antipaladin", 1).expect("class:antipaladin must resolve at level 1");
        assert_eq!(row.hit_die, 10);
        assert_eq!(row.base_attack_bonus, 1); // Full BAB: level 1
        assert_eq!(row.fort_save, 2); // good: 1/2+2 = 2 (integer division)
        assert_eq!(row.ref_save, 0); // poor: 1/3 = 0
        assert_eq!(row.will_save, 2); // good: 1/2+2 = 2
    }
}
