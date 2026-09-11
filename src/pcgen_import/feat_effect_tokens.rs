//! The `BONUS:`/`DEFINE:` effect tokens the Ultimate Magic feat catalog
//! records carry, relocated off the live side — SD-35
//! `AT-35-E6-003-SWEEP` cycle 6, enforcing `decisions.md` §11 (nothing on
//! the live side reads a PCGen token).
//!
//! # Why this moved, and why it was not deleted
//!
//! `rules_tables::ultimate_magic::feat_tables::UmFeatEntry` carried an
//! `effect: Option<&'static [&'static str]>` field holding every `BONUS:`
//! and `DEFINE:` token of the corpus row, verbatim, in source order. It was
//! the largest single non-test PCGen residue left on the live side — 75 of
//! the 798 code hits `scripts/pcgen_residue_gate.py --check` counted at
//! cycle 6's start.
//!
//! **No live engine ever read it.** The only readers were three assertions
//! in the catalog file's own `#[cfg(test)] mod tests`, which read it as a
//! *presence* flag ("this record carries real content"), never as a token.
//! Those three now read [`um_feat_effect_tokens`]. Re-derive the absence of
//! any other reader:
//!
//! ```text
//! grep -rn "\.effect\b" src apps/desktop/src-tauri/src tests --include=*.rs \
//!   | grep -v "effect: " | grep -v effect_text | grep -v "\.effect\.spell_id"
//! ```
//!
//! Every remaining hit is `rules_tables::crb::feats::FeatTableEntry.effect`,
//! a typed `FeatEffectBonus` slice — a different field of a different type,
//! owned by a different lane, and untouched here.
//!
//! So this is a **move, not a removal** (`decisions.md` §11: the converter
//! side is KEPT and reused for Starfinder). The relocation is generated and
//! checked by
//! `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle6_relocate_um_effect_tokens.py`,
//! whose `--check` mode re-reads both sides and fails unless every token
//! string survives byte for byte.
//!
//! # How a row is addressed
//!
//! By `(rule_set, index)` — the record's own position in
//! `ultimate_magic::feat_tables::feat_tables()` — and **not** by
//! `(rule_set, key)`, for the reason
//! [`crate::pcgen_import::feat_prereq_tokens`] gives at length: a key lookup
//! can collide, an index+key pair fails loudly when the live table is
//! reordered. Each row also carries the key it was taken from, and the
//! lookup asserts it against the caller's record.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::rules_core::rules_tables::RuleSetId;

/// One relocated record's effect tokens: `(rule_set, index in that book's
/// table, the record's `key`, the tokens in corpus source order)`.
pub type FeatEffectRow = (RuleSetId, usize, &'static str, &'static [&'static str]);

/// The `BONUS:`/`DEFINE:` tokens the Ultimate Magic feat table carried,
/// addressed by each record's index in
/// `ultimate_magic::feat_tables::feat_tables()`.
///
/// 43 row(s) of the table's 144 record(s) carry at least one token.
pub static UM_FEAT_EFFECT_TOKENS: &[FeatEffectRow] = &[
    (RuleSetId::Um, 19, "Eldritch Heritage", &["BONUS:VAR|EldritchHeritageBloodlineLVL|TL-2", "BONUS:VAR|Sorcerer_Spells_StatBonus|CHA|TYPE=Base"]),
    (RuleSetId::Um, 21, "Evolved Familiar", &["BONUS:VAR|FamiliarEP|1"]),
    (RuleSetId::Um, 23, "Extra Arcana", &["BONUS:ABILITYPOOL|Magus Arcana|1"]),
    (RuleSetId::Um, 24, "Extra Arcane Pool", &["BONUS:VAR|MagusArcanePool|2"]),
    (RuleSetId::Um, 25, "Extended Bane", &["BONUS:VAR|InquisitorBanePool|MAX(0,WIS)"]),
    (RuleSetId::Um, 26, "Extra Cantrips or Orisons", &["BONUS:SPELLKNOWN|CLASS=%LIST;LEVEL=0|2"]),
    (RuleSetId::Um, 27, "Extra Evolution", &["BONUS:VAR|Feat_Extra_Evolution_Count|1", "BONUS:VAR|EidolonEvolution|1"]),
    (RuleSetId::Um, 28, "Extra Ranger Trap", &["BONUS:VAR|TrapTimes|2"]),
    (RuleSetId::Um, 29, "Extra Summons", &["BONUS:VAR|Feat_Extra_Summons_Taken|1", "BONUS:VAR|SummonMonsterTimes|1"]),
    (RuleSetId::Um, 40, "Greater Eldritch Heritage", &["BONUS:ABILITYPOOL|Eldritch Heritage Selection|1", "BONUS:VAR|EldritchHeritageBloodlineLVL|2"]),
    (RuleSetId::Um, 44, "Implant Bomb", &["BONUS:VAR|ImplantedBombDisableDC|11+classlevel(\"Alchemist\")"]),
    (RuleSetId::Um, 45, "Improved Eldritch Heritage", &["BONUS:ABILITYPOOL|Eldritch Heritage Selection|1"]),
    (RuleSetId::Um, 46, "Improved Monster Lore", &["BONUS:VAR|ImprovedMonsterLoreBonus|classlevel(\"Inquisitor\")/2"]),
    (RuleSetId::Um, 51, "Learn Ranger Trap", &["BONUS:ABILITYPOOL|Ranger Trap|1", "BONUS:VAR|RangerTrapLVL|TL", "BONUS:VAR|TrapTimes|max(1,WIS+classlevel(\"Ranger\")/2)", "BONUS:VAR|TrapDC|10+CL/2+WIS", "BONUS:VAR|TrapDuration|CL/2"]),
    (RuleSetId::Um, 52, "Life Lure", &["BONUS:VAR|LifeLureDuration|max(1,CHA)"]),
    (RuleSetId::Um, 55, "Oracular Intuition", &["BONUS:SKILL|Sense Motive|if(skillinfo(\"TOTALRANK\",\"Sense Motive\")>=10,4,2)", "BONUS:SKILL|Spellcraft|if(skillinfo(\"TOTALRANK\",\"Spellcraft\")>=10,4,2)"]),
    (RuleSetId::Um, 56, "Painful Anchor", &["BONUS:VAR|PainfulAnchorBonusDamage|CHA"]),
    (RuleSetId::Um, 60, "Prodigy", &["BONUS:SKILL|%LIST|if(skillinfo(\"TOTALRANK\",\"%LIST\")>=10,4,2)|TYPE=Prodigy"]),
    (RuleSetId::Um, 63, "Quarterstaff Master", &["BONUS:VAR|WeapSpecQualify|1"]),
    (RuleSetId::Um, 66, "Radiant Charge", &["BONUS:VAR|RadiantChargeBonusDamage|CHA"]),
    (RuleSetId::Um, 68, "Resilient Eidolon", &["BONUS:VAR|ResilientEidolonDuration|classlevel(\"Summoner\")"]),
    (RuleSetId::Um, 70, "Reward of Life", &["BONUS:VAR|RewardOfLifeHealing|CHA"]),
    (RuleSetId::Um, 75, "Shaping Focus", &["BONUS:VAR|DruidWildShape|MIN(4,classlevel(\"TYPE=PC\")+classlevel(\"TYPE=NPC\")-classlevel(\"Druid\"))"]),
    (RuleSetId::Um, 81, "Spell Specialization (Abjuration)", &["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
    (RuleSetId::Um, 82, "Spell Specialization (Conjuration)", &["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
    (RuleSetId::Um, 83, "Spell Specialization (Divination)", &["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
    (RuleSetId::Um, 84, "Spell Specialization (Enchantment)", &["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
    (RuleSetId::Um, 85, "Spell Specialization (Evocation)", &["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
    (RuleSetId::Um, 86, "Spell Specialization (Illusion)", &["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
    (RuleSetId::Um, 87, "Spell Specialization (Necromancy)", &["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
    (RuleSetId::Um, 88, "Spell Specialization (Transmutation)", &["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
    (RuleSetId::Um, 104, "Ultimate Resolve", &["BONUS:VAR|AuraOfResolveRadius|10"]),
    (RuleSetId::Um, 105, "Uncanny Alertness", &["BONUS:SKILL|Perception|1", "BONUS:SKILL|Sense Motive|1"]),
    (RuleSetId::Um, 106, "Uncanny Concentration", &["BONUS:CONCENTRATION|ALLSPELLS|2"]),
    (RuleSetId::Um, 107, "Undead Master", &["BONUS:VAR|CommandUndeadHD|4"]),
    (RuleSetId::Um, 109, "Unsanctioned Knowledge", &["BONUS:ABILITYPOOL|Unsanctioned Knowledge ~ Level 1|1", "BONUS:ABILITYPOOL|Unsanctioned Knowledge ~ Level 2|1", "BONUS:ABILITYPOOL|Unsanctioned Knowledge ~ Level 3|1", "BONUS:ABILITYPOOL|Unsanctioned Knowledge ~ Level 4|1"]),
    (RuleSetId::Um, 110, "Versatile Channeler", &["BONUS:ABILITYPOOL|Versatile Channeler|1"]),
    (RuleSetId::Um, 112, "Voice of the Sibyl", &["BONUS:SITUATION|Bluff=Using voice|if(skillinfo(\"TOTALRANK\",\"Bluff\")>=10,3,1)", "BONUS:SITUATION|Diplomacy=Using voice|if(skillinfo(\"TOTALRANK\",\"Diplomacy\")>=10,3,1)", "BONUS:SKILL|Perform (Oratory)|if(skillinfo(\"TOTALRANK\",\"Perform (Oratory)\")>=10,3,1)"]),
    (RuleSetId::Um, 113, "Warrior Priest", &["BONUS:COMBAT|INITIATIVE|1"]),
    (RuleSetId::Um, 114, "Wild Speech", &["BONUS:VAR|WildSpeechCasterLevel|DruidLVL", "BONUS:VAR|WildSpeechDuration|DruidLVL"]),
    (RuleSetId::Um, 132, "Transfer Feat to Familiar", &["BONUS:VAR|BeastBondedFeatTransfer|1"]),
    (RuleSetId::Um, 139, "Discovery (Opposition Research)", &["BONUS:VAR|Arcane Opposition School|-1"]),
    (RuleSetId::Um, 140, "Discovery (Split Slot)", &["BONUS:ABILITYPOOL|Split Slot|1", "BONUS:VAR|SplitSlotTimes|1"]),
];

/// Every record in the live Ultimate Magic feat table, in table order —
/// the denominator the relocated rows are a subset of.
pub const UM_FEAT_TABLE_LEN: usize = 144;

/// The number of relocated rows; the live table's own
/// `the_desc_benefit_effect_split_is_the_real_one` re-derives it.
pub const UM_FEAT_EFFECT_ROW_COUNT: usize = 43;

fn index_of() -> &'static HashMap<(RuleSetId, usize), FeatEffectRow> {
    static CELL: OnceLock<HashMap<(RuleSetId, usize), FeatEffectRow>> = OnceLock::new();
    CELL.get_or_init(|| UM_FEAT_EFFECT_TOKENS.iter().map(|row| ((row.0, row.1), *row)).collect())
}

/// The `BONUS:`/`DEFINE:` tokens the record at `index` in `rule_set`'s feat
/// table carried, or `None` when its corpus row carried none.
///
/// `key` is the caller's own record key and is asserted against the one the
/// row was taken from — see this module's doc comment.
pub fn um_feat_effect_tokens(
    rule_set: RuleSetId,
    index: usize,
    key: &str,
) -> Option<&'static [&'static str]> {
    let row = index_of().get(&(rule_set, index))?;
    assert_eq!(
        row.2, key,
        "{rule_set:?} index {index} holds the effect tokens taken from {:?}, but the caller \
         passed {key:?}; the live table this was relocated from has been reordered",
        row.2
    );
    Some(row.3)
}

/// `true` when the record at `index` carried at least one effect token. The
/// presence flag the live catalog's own tests used to read off the field.
pub fn um_feat_carries_effect(rule_set: RuleSetId, index: usize, key: &str) -> bool {
    um_feat_effect_tokens(rule_set, index, key).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::ultimate_magic::feat_tables::feat_tables;

    #[test]
    fn every_relocated_row_still_names_its_own_live_record() {
        for &(rule_set, index, key, tokens) in UM_FEAT_EFFECT_TOKENS {
            assert_eq!(rule_set, RuleSetId::Um);
            let entry = feat_tables()
                .get(index)
                .unwrap_or_else(|| panic!("index {index} is past the end of the live table"));
            assert_eq!(entry.key, key, "index {index} no longer names {key}");
            assert!(!tokens.is_empty(), "{key} was relocated with an empty token array");
        }
    }

    #[test]
    fn the_live_table_is_the_length_the_relocation_was_taken_from() {
        assert_eq!(feat_tables().len(), UM_FEAT_TABLE_LEN);
        assert_eq!(UM_FEAT_EFFECT_TOKENS.len(), UM_FEAT_EFFECT_ROW_COUNT);
    }

    #[test]
    fn a_key_that_does_not_match_its_index_panics() {
        let (_, index, key, _) = UM_FEAT_EFFECT_TOKENS[0];
        assert!(um_feat_effect_tokens(RuleSetId::Um, index, key).is_some());
        assert!(
            std::panic::catch_unwind(|| um_feat_effect_tokens(
                RuleSetId::Um,
                index,
                "Not This Record"
            ))
            .is_err(),
            "a mismatched key must fail loudly, not return another record's tokens"
        );
    }
}
