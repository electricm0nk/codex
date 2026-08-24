//! Generic companion base-ability-score table (SD-32 T12
//! `epic-10-reference-library-residual-reach` row 20, cycle 5).
//!
//! # What this module answers, and why it exists
//!
//! Cycles 3 and 4 asked and settled a question the wolf/horse hand-authored
//! constants in this module's parent (`ground_wolf_companion_stat_block`,
//! `ground_horse_companion_stat_block`) left open: where does a companion's
//! BASE (pre-advancement) ability-score block come from, for every species
//! this engine does not already hand-ground? Cycle 4 read the pinned oracle
//! directly and confirmed the block is **not `.lst` data anywhere in
//! PCGen's own source** -- every companion race record carries only
//! `BONUS:STAT` DELTAS (e.g. Wolf's own `STR|2 DEX|4 CON|4 INT|-8 WIS|2
//! CHA|-4`, `core_rulebook/cr_races_companion.lst:32`), never an absolute
//! score. The base score is fixed, per-species, printed prose the engine
//! must hand-author -- exactly the precedent `WOLF_COMPANION_STRENGTH_
//! SCORE`/`HORSE_COMPANION_STRENGTH_SCORE` (this module's parent, above)
//! already set, verified against two independent primary sources
//! (aonprd.com and d20pfsrd) plus the corpus as a tiebreaker.
//!
//! # This cycle's own correction to cycle 4's sizing (`decisions.md §17a`)
//!
//! Cycle 4 sized the follow-on as "a category table... a handful of
//! `RACESUBTYPE:` rows," inferring from the `RACESUBTYPE:PlantCompanion`-
//! style tags shared by several Ultimate Wilderness companions that a
//! single row could serve every member of one category. Re-derived, not
//! trusted: `grep -rh "RACETYPE:Companion" $PCGEN_REPO_DIR/data | grep -oE
//! "RACESUBTYPE:[A-Za-z]+" | sort | uniq -c` finds only 59 of the corpus's
//! 213 total `RACETYPE:Companion` records carry a `RACESUBTYPE:` tag at
//! all (31 `AnimalCompanionDinosaur`, 13 `Aquatic`, 8 `PlantCompanion`, 4
//! `AnimalCompanionPrimate`, 3 `ConstructCompanion`); the other 154 carry
//! none. Cross-checking two SAME-category Ultimate Wilderness members
//! (both `RACESUBTYPE:PlantCompanion`) against their own published base
//! scores (verified via aonprd.com's Druid Companions pages) REFUTES the
//! shared-category-base hypothesis directly: Gulper Plant's own delta
//! (`STR|2 CON|2 INT|-10 CHA|-8`, no DEX/WIS delta) against its printed
//! Str 12/Dex 11/Con 13/Int 1/Wis 10/Cha 3 backs out a base of STR 10 / DEX
//! 11 / CON 11 / INT 11 / WIS 10 / CHA 11; Hunting Cactus's own delta
//! (`STR|4 DEX|2 CON|6 INT|-8 WIS|2 CHA|-4`) against its printed Str
//! 14/Dex 13/Con 17/Int 2/Wis 13/Cha 6 backs out STR 10 / DEX 11 / CON 11 /
//! INT 10 / WIS 11 / CHA 10 -- the SAME category, two DIFFERENT base
//! vectors. **This is genuinely per-species data, not a per-category
//! table**, and the real population needing it is closer to 213 (or, if
//! scoped to only the "unusual" bucket cycle 4's own finding named, 59)
//! than "a handful."
//!
//! # What this cycle builds, and what it names rather than fabricates
//!
//! Building all 213 (or 59) entries to the same two-independent-source
//! verification bar `WOLF_COMPANION_STRENGTH_SCORE`'s own precedent sets
//! is real, sized, per-species sourcing work this one cycle does not have
//! the room to complete without lowering that bar -- and shipping an
//! under-verified number here is a worse outcome than shipping none: a
//! silently-wrong ability score corrupts a real character's combat math,
//! exactly the failure `decisions.md §1a` and this codebase's own
//! anti-fabrication test suite (`class_feature_grant_consumer.rs`'s
//! thirteen-test gate) exist to refuse. This module therefore:
//!
//! 1. Builds the GENERIC mechanism -- [`CompanionBaseStats`], the lookup
//!    table, and [`ground_companion_stat_block`] -- generalizing
//!    `ground_wolf_companion_stat_block`/`ground_horse_companion_stat_
//!    block`'s own proven math (same universal `MONSTERCLASS:Companion:2`
//!    Hit Dice progression -- confirmed present verbatim on every one of
//!    the 213 records, including Gulper Plant's own -- and the same
//!    universal `floor(MasterLevel/3)` Strength/Dexterity and
//!    `2*floor(MasterLevel/3)` natural-armor advancement, both from the
//!    SHARED `cr_classes_companion.lst`/`cr_abilities_companion.lst` files
//!    every companion species reads, species-specific "Companion
//!    Advancement ~ <Species>" abilities aside -- unmodeled here exactly
//!    as Wolf's own equivalent ability is unmodeled by its existing
//!    grounding function, so this is not a new scope gap, the same one).
//! 2. Populates it with Wolf and Horse, RE-DERIVED (not copied) from this
//!    module's parent's own already-verified constants, as the proof the
//!    generic function reproduces the existing, shipped, tested output
//!    byte-for-byte (`generic_wolf_reproduces_the_existing_hand_authored_
//!    wolf_function` below).
//! 3. Adds ONE new, externally re-verified species -- Gulper Plant
//!    (`RACESUBTYPE:PlantCompanion`, Ultimate Wilderness p.183) -- as
//!    concrete, non-hypothetical proof this generalizes past the two
//!    species it was built to reproduce, verified against aonprd.com's own
//!    Druid Companions page (Str 12, Dex 11, Con 13, Int 1, Wis 10, Cha 3;
//!    natural armor +1, `BONUS:VAR|AC_Natural_Armor|1|TYPE=Base`,
//!    `uw_races_companion.lst`, corpus-confirmed) with the corpus's own
//!    `BONUS:STAT` deltas as the tiebreaker check (both agree).
//! 4. Names the exact residual precisely rather than rounding it away:
//!    212 of 213 `RACETYPE:Companion` corpus records (213 total minus
//!    Wolf, Horse, and Gulper Plant) still have no base-ability-score
//!    entry in [`companion_base_stat_table`] and [`ground_companion_stat_
//!    block`] correctly REFUSES (returns `false`, grounds nothing) for
//!    every one of them -- refuse rather than guess, the same posture
//!    `class_feature_grant_consumer.rs`'s own module doc names throughout.
//!    The next cycle's own concrete first step: pick the next batch of
//!    species (by population weight -- `AnimalCompanionDinosaur`'s 31
//!    records are the largest single bucket) and repeat this cycle's own
//!    two-source-plus-corpus-tiebreaker verification per species.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use super::ComputationExplanation;

/// One companion species' verified base (pre-advancement) statistics --
/// the exact set `WOLF_COMPANION_STRENGTH_SCORE`/`HORSE_COMPANION_
/// STRENGTH_SCORE` and their siblings already hand-author per species in
/// this module's parent, gathered into one row so [`ground_companion_
/// stat_block`] can be table-driven rather than one hand-typed function
/// per species.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CompanionBaseStats {
    pub(crate) strength: i16,
    pub(crate) constitution: i16,
    pub(crate) natural_armor: i16,
    pub(crate) hit_die_size: u8,
}

/// The verified companion base-stat table, keyed by the same lower-case,
/// underscore-joined species slug [`super::pu_feature_slug`]-style
/// convention this module's sibling `class_feature_grant_consumer.rs`
/// already uses for its own id suffixes -- `"wolf"`, `"horse"`,
/// `"gulper_plant"`.
///
/// Hit Die size is 8 for every entry, not per-species: the PF1 Core
/// Rulebook's own "Animal Companion Base Statistics" table fixes the
/// companion's Hit Die at d8 regardless of the companion's own creature
/// type (confirmed by both existing entries already sharing it despite
/// being two different real creature types, Animal in both cases, and
/// unchanged for Gulper Plant's own Plant type below -- the companion
/// mechanic overrides the normal per-type Hit Die a standalone monster of
/// the same species would otherwise use).
fn companion_base_stat_table() -> &'static BTreeMap<&'static str, CompanionBaseStats> {
    static TABLE: OnceLock<BTreeMap<&'static str, CompanionBaseStats>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut out = BTreeMap::new();
        out.insert(
            "wolf",
            CompanionBaseStats {
                strength: super::WOLF_COMPANION_STRENGTH_SCORE,
                constitution: super::WOLF_COMPANION_CONSTITUTION_SCORE,
                natural_armor: super::WOLF_COMPANION_NATURAL_ARMOR,
                hit_die_size: super::WOLF_COMPANION_HIT_DIE_SIZE,
            },
        );
        out.insert(
            "horse",
            CompanionBaseStats {
                strength: super::HORSE_COMPANION_STRENGTH_SCORE,
                constitution: super::HORSE_COMPANION_CONSTITUTION_SCORE,
                natural_armor: super::HORSE_COMPANION_NATURAL_ARMOR,
                hit_die_size: super::HORSE_COMPANION_HIT_DIE_SIZE,
            },
        );
        // Ultimate Wilderness p.183, RACESUBTYPE:PlantCompanion. Str 12,
        // Con 13, +1 natural armor -- verified against aonprd.com's own
        // Druid Companions page (Gulper Plant), with the corpus's own
        // `BONUS:STAT|STR|2 BONUS:STAT|CON|2` deltas
        // (uw_races_companion.lst) and `BONUS:VAR|AC_Natural_Armor|1|
        // TYPE=Base` as the tiebreaker: both agree on a base of Str 10 /
        // Con 11 once the delta is backed out, consistent with the
        // printed totals. Dex/Int/Wis/Cha are not grounded by this module
        // (this table's own consumer, like Wolf's, grounds only the
        // fields with a live downstream reader -- attack bonus, saves,
        // AC, HP).
        out.insert(
            "gulper_plant",
            CompanionBaseStats { strength: 10, constitution: 11, natural_armor: 1, hit_die_size: 8 },
        );
        out
    })
}

/// Grounds `species_slug`'s standalone companion stat block, exactly the
/// way [`super::ground_wolf_companion_stat_block`]/[`super::ground_horse_
/// companion_stat_block`] already do for their own two species, but
/// table-driven via [`companion_base_stat_table`] rather than one
/// hand-typed function per species. Returns `true` when `species_slug`
/// was found and grounded, `false` when it refuses -- never guesses --
/// because no verified base-stat entry exists yet for that species.
///
/// Reuses this module's parent's own universal companion-advancement math
/// (`super::animal_companion_table_index`/`_natural_armor_bonus`/
/// `_stat_bonus`/`_hit_points`), confirmed species-agnostic by every
/// candidate record this cycle checked sharing the identical
/// `MONSTERCLASS:Companion:2` progression tag, so a new species needs
/// only its own base-ability-score row here, never a second copy of the
/// advancement math.
pub(crate) fn ground_companion_stat_block(
    species_slug: &str,
    id_prefix: &str,
    owner_class_label: &str,
    species_display_name: &str,
    companion_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) -> bool {
    let Some(stats) = companion_base_stat_table().get(species_slug) else {
        return false;
    };
    let companion_hd =
        super::ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL[super::animal_companion_table_index(companion_level)];
    let companion_hd_value = i16::from(companion_hd);
    let companion_base_attack_bonus = companion_hd_value * 3 / 4;
    let companion_fort_ref_save = companion_hd_value / 2 + 2;
    let companion_will_save = companion_hd_value / 3;
    let strength_bonus = super::animal_companion_stat_bonus(companion_level);
    let strength_score = stats.strength + strength_bonus;
    let natural_armor = stats.natural_armor + super::animal_companion_natural_armor_bonus(companion_level);
    let companion_armor_class = 10 + natural_armor;
    let strength_modifier = super::ability_modifier(strength_score);
    let constitution_modifier = super::ability_modifier(stats.constitution);
    let companion_attack_bonus = companion_base_attack_bonus + strength_modifier;
    let companion_hp =
        super::animal_companion_hit_points(companion_hd, stats.hit_die_size, constitution_modifier);

    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.{species_slug}_stat_block"),
        value: 0,
        detail: format!(
            "{owner_class_label} level {companion_level} animal companion, {species_display_name}: \
             a wholly separate creature with its own combat statistics -- none of the values below \
             are ever applied to the {owner_class_label}'s own integrated totals. Base ability \
             scores: Str {}, Con {}. This is a bounded recognition record only (+0); the \
             companion's own flat stat values are grounded separately as standalone explanation \
             records below",
            stats.strength, stats.constitution
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_attack_bonus"),
        value: companion_attack_bonus,
        detail: format!(
            "{species_display_name} companion base attack bonus at {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: HD*3/4 = {companion_base_attack_bonus}) + \
             Strength modifier ({strength_modifier:+}, Str {strength_score} = base {} + \
             {strength_bonus} from the companion class's own floor(master level/3) Strength/\
             Dexterity advance) = {companion_attack_bonus}. Standalone record; the companion is a \
             separate creature, not integrated into the {owner_class_label}'s own combat totals",
            stats.strength
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.fortitude"),
        value: companion_fort_ref_save,
        detail: format!(
            "{species_display_name} companion base Fortitude save at {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/2+2 = {companion_fort_ref_save}). \
             Standalone record; not the {owner_class_label}'s own save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.reflex"),
        value: companion_fort_ref_save,
        detail: format!(
            "{species_display_name} companion base Reflex save at {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/2+2 = {companion_fort_ref_save}). \
             Standalone record; not the {owner_class_label}'s own save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.base_save.will"),
        value: companion_will_save,
        detail: format!(
            "{species_display_name} companion base Will save at {companion_hd} HD (PF1 Core \
             Rulebook Animal Companion Base Statistics: classlevel/3 = {companion_will_save}). \
             Standalone record; not the {owner_class_label}'s own save"
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.armor_class"),
        value: companion_armor_class,
        detail: format!(
            "{species_display_name} companion armor class: base 10 + natural armor (+{natural_armor} \
             = the species' own base +{} plus +{} from the companion class's own \
             2*floor(master level/3) natural-armor advance) = {companion_armor_class}. Standalone \
             record; Dexterity's own contribution to the companion's AC is not grounded",
            stats.natural_armor,
            super::animal_companion_natural_armor_bonus(companion_level)
        ),
    });
    explanations.push(ComputationExplanation {
        id: format!("{id_prefix}.hit_points"),
        value: companion_hp,
        detail: format!(
            "{species_display_name} companion hit points at {companion_hd} HD (d{}): maximized \
             first Hit Die plus average for each of the remaining {} (this codebase's own \
             established HP idiom, durability.rs's compute_max_hp), each plus the companion's \
             Constitution modifier ({constitution_modifier:+}, Con {}) = {companion_hp}",
            stats.hit_die_size,
            companion_hd.saturating_sub(1),
            stats.constitution
        ),
    });
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_wolf_reproduces_the_existing_hand_authored_wolf_function() {
        // Same inputs `animal_companion_stat_block_tests` (this module's
        // parent) already exercises for `ground_wolf_companion_stat_
        // block`, at master level 1 -- proving this table-driven function
        // is not a second, independently-drifting implementation.
        let mut generic = Vec::new();
        let grounded =
            ground_companion_stat_block("wolf", "companion", "Druid", "Wolf", 1, &mut generic);
        assert!(grounded, "wolf must be found in the table");

        let mut hand_authored = Vec::new();
        super::super::ground_wolf_companion_stat_block("companion", "Druid", 1, &mut hand_authored);

        let generic_values: BTreeMap<&str, i16> =
            generic.iter().map(|e| (e.id.rsplit('.').next().unwrap(), e.value)).collect();
        let hand_values: BTreeMap<&str, i16> =
            hand_authored.iter().map(|e| (e.id.rsplit('.').next().unwrap(), e.value)).collect();
        for (suffix, hand_value) in &hand_values {
            // `wolf_stat_block`: the recognition-only +0 record, different id
            // shape between the two (only its value, +0 both sides, would
            // match anyway). `bite_attack`: the species' own primary
            // natural-attack damage bonus -- deliberately NOT generalized
            // here (this module's own doc: "grounds only the fields with a
            // live downstream reader -- attack bonus, saves, AC, HP"), since
            // a natural attack's own shape (bite/hoof/vine/claw, and which
            // multiplier applies) is genuinely per-species and would need
            // its own verified data, not a formula this generic function may
            // assume.
            if *suffix == "wolf_stat_block" || *suffix == "bite_attack" {
                continue;
            }
            assert_eq!(
                generic_values.get(suffix),
                Some(hand_value),
                "generic ground_companion_stat_block must reproduce {suffix} byte-for-byte"
            );
        }
    }

    #[test]
    fn generic_horse_reproduces_the_existing_hand_authored_horse_function() {
        let mut generic = Vec::new();
        let grounded =
            ground_companion_stat_block("horse", "mount", "Cavalier", "Horse", 1, &mut generic);
        assert!(grounded, "horse must be found in the table");

        let mut hand_authored = Vec::new();
        super::super::ground_horse_companion_stat_block("mount", "Cavalier", 1, &mut hand_authored);

        let generic_values: BTreeMap<&str, i16> =
            generic.iter().map(|e| (e.id.rsplit('.').next().unwrap(), e.value)).collect();
        let hand_values: BTreeMap<&str, i16> =
            hand_authored.iter().map(|e| (e.id.rsplit('.').next().unwrap(), e.value)).collect();
        for (suffix, hand_value) in &hand_values {
            // See the wolf test's own comment for why the natural-attack
            // damage bonus (here, the Horse's hoof attack) is excluded.
            if *suffix == "horse_stat_block" || *suffix == "hoof_attack" {
                continue;
            }
            assert_eq!(
                generic_values.get(suffix),
                Some(hand_value),
                "generic ground_companion_stat_block must reproduce {suffix} byte-for-byte"
            );
        }
    }

    #[test]
    fn gulper_plant_grounds_a_real_new_species_at_master_level_1() {
        // Str 10 base + 0 advancement (floor(1/3)=0) = Str 10, modifier
        // +0; base attack bonus at 2 HD (master level 1's own HD per the
        // universal table) = 2*3/4 = 1; +0 Str modifier = 1.
        let mut explanations = Vec::new();
        let grounded = ground_companion_stat_block(
            "gulper_plant",
            "companion",
            "Druid",
            "Gulper Plant",
            1,
            &mut explanations,
        );
        assert!(grounded, "gulper_plant must be found in the table");
        let by_id: BTreeMap<&str, i16> = explanations.iter().map(|e| (e.id.as_str(), e.value)).collect();
        assert_eq!(by_id.get("companion.base_attack_bonus"), Some(&1));
    }

    #[test]
    fn gulper_plant_base_saves_and_armor_class_at_master_level_1() {
        let mut explanations = Vec::new();
        assert!(ground_companion_stat_block(
            "gulper_plant",
            "companion",
            "Druid",
            "Gulper Plant",
            1,
            &mut explanations,
        ));
        let by_id: BTreeMap<&str, i16> = explanations.iter().map(|e| (e.id.as_str(), e.value)).collect();
        assert_eq!(by_id.get("companion.base_save.fortitude"), Some(&3));
        assert_eq!(by_id.get("companion.base_save.reflex"), Some(&3));
        assert_eq!(by_id.get("companion.base_save.will"), Some(&0));
        // AC = 10 + natural armor (1 + 0 advancement) = 11.
        assert_eq!(by_id.get("companion.armor_class"), Some(&11));
        // HP at 2 HD, d8, Con modifier +0: maximized first (8) + average
        // second (durability::average_hit_die_value(8) = 5, the PF1
        // round-up convention) = 13.
        assert_eq!(by_id.get("companion.hit_points"), Some(&13));
    }

    #[test]
    fn an_unknown_species_slug_refuses_rather_than_guesses() {
        // `decisions.md §1a`/the same posture `class_feature_grant_
        // consumer.rs`'s own module doc names throughout: no verified
        // base-stat entry, no grounded record -- ever.
        let mut explanations = Vec::new();
        let grounded = ground_companion_stat_block(
            "griffon",
            "companion",
            "Druid",
            "Griffon",
            1,
            &mut explanations,
        );
        assert!(!grounded, "an ungrounded species must refuse, not fabricate a stat block");
        assert!(explanations.is_empty());
    }

    #[test]
    fn only_three_of_the_corpus_s_213_racetype_companion_records_have_a_base_stat_entry() {
        // Named exactly, not rounded away (§16/§17a): the honest residual
        // this cycle leaves for the next one.
        assert_eq!(
            companion_base_stat_table().len(),
            3,
            "wolf, horse, gulper_plant -- 210 of 213 real RACETYPE:Companion corpus records still \
             have no verified base-ability-score entry and must keep refusing until a future cycle \
             adds them, per-species, the same way this one added gulper_plant"
        );
    }
}
