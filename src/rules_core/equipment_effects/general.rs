//! Epic 5, second equipment category (SD-20 §1.5 work-unit order): CRB
//! `general` per-item effect resolution.
//!
//! Unlike `arms_armor` (which carries `ACCHECK:`/`MAXDEX:`/`SPELLFAILURE:`
//! and `BONUS:COMBAT|AC|...` tokens), the CRB `general` equipment block
//! (`core_rulebook/cr_equip_general.lst`) is dominated by masterwork
//! tools and kits whose real, load-bearing mechanical effect is a
//! `BONUS:SKILL|<skill>|<n>|TYPE=Circumstance` token — e.g. `KEY:Thieves'
//! Tools` carries `BONUS:SKILL|Disable Device|2|TYPE=Circumstance|
//! PRETYPE:1,Masterwork` and `KEY:Climber's Kit` carries
//! `BONUS:SKILL|Climb|2|TYPE=Circumstance`, both confirmed directly
//! against the real corpus. Most other `general` records (trade goods,
//! plain containers, tattoos, ...) carry no `BONUS:` token at all, so
//! `None` for those is an honest absence, not a fabricated zero. No field
//! here is hand-rolled; every value traces back to a real, verbatim
//! corpus token, read the same way `arms_armor.rs` reads its own tokens
//! straight off the resolved record.
//!
//! **SD-35 `AT-35-E6-003-RULED` cycle 11.** Every rule and every real-corpus
//! witness named above is unchanged and still real. What moved is WHERE the
//! reading happens: once, at ingest, in
//! [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`], which is
//! where `decisions.md` §11 rules that rule conversion belongs. The functions
//! below report the settled value off
//! [`crate::rules_core::equipment_record::CorpusEquipmentRecord`] and name no
//! ingest-format token at all.

use crate::rules_core::equipment_record::CorpusEquipmentRecord;

/// A skill-check circumstance bonus granted by a `general`-category
/// item's `BONUS:SKILL|<skill>|<n>|TYPE=Circumstance` corpus token.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SkillCheckBonus {
    pub skill: String,
    pub bonus: i16,
}

/// One `general` corpus record's settled skill-check-bonus contribution.
///
/// The item's own stated circumstance bonus to one named skill, plus the
/// automatic `+8` PF1 grants on Swim checks to anything that grants a swim
/// speed (real witness: `ultimate_equipment:equipment:ring_of_the_sea_strider`,
/// whose oracle total is `16`, not its stated `8`). The overwhelming majority
/// of `general` records -- trade goods, containers, tattoos -- state no such
/// bonus and yield `None`: honest absence, never a fabricated zero. A
/// multi-skill or wildcard grant is a wider shape this single-skill value
/// cannot state, and is likewise absent rather than guessed.
///
/// SD-35 `AT-35-E6-003-RULED` cycle 11: the reading moved to ingest
/// ([`crate::pcgen_import::ir_converter::equipment_record_to_corpus`],
/// `decisions.md` §11); the rule and every witness above are unchanged.
pub fn compute_general_effect(record: &CorpusEquipmentRecord) -> Option<SkillCheckBonus> {
    record.skill_check_bonus.clone()
}

/// One named-variable bonus granted by a `BONUS:VAR|<name(s)>|<value>`
/// equipment chain.
///
/// SD-33 remediation wave 3 (`var-bonus-shape` lane, `AT-33-E5-002`
/// remainder): unlike `BONUS:SKILL`/`BONUS:STAT`, a `VAR` chain targets an
/// arbitrary named PCGen variable (`LOADSCORE`, `CMD_Disarm`,
/// `WeaponTrainingBase`, ...) rather than a fixed stat/skill slot. PCGen's
/// own variable engine (`pc.getVariable`, confirmed empirically this cycle
/// against the pinned oracle) resolves an unset/undefined variable name to
/// `0`, and every named variable this cycle's 108-unit population reads is
/// itself `DEFINE:<name>|0`'d inside a specific race/class/feat ability
/// record (e.g. `LOADSCORE` inside the `STR` stat's own always-granted
/// definition, `WeaponTrainingBase` inside the Fighter `Weapon Training`
/// class feature) — the equipment item's own `BONUS:VAR` contribution is a
/// flat, unconditional, character-independent literal in every one of
/// this population's 108 real corpus records (confirmed this cycle: zero
/// formula-valued or `PRE`-gated `VAR` chains among them).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VarBonus {
    pub name: String,
    pub bonus: i16,
}

/// One equipment record's settled flat bonuses to named rules variables,
/// one row per name.
///
/// An item may state the same magnitude for several variables at once (real
/// verbatim: `Gloves of Dueling`'s `CMD_Disarm` and `CMD_Sunder`); each gets
/// its own row. Stacking metadata is not part of the magnitude and is not
/// carried here. An empty vec means the item states no such bonus -- honest
/// absence, not a fabricated zero.
///
/// SD-35 `AT-35-E6-003-RULED` cycle 11: the reading moved to ingest
/// ([`crate::pcgen_import::ir_converter::equipment_record_to_corpus`],
/// `decisions.md` §11); the rule and every witness above are unchanged.
pub fn compute_var_effect(record: &CorpusEquipmentRecord) -> Vec<VarBonus> {
    record.var_bonuses.clone()
}

/// Sums every EQMOD-referenced modifier record's own `VAR` chains into
/// `base`, matched by name — the `VAR`-shape sibling of
/// [`arms_armor::apply_eqmod_armor_class_bonus`].
///
/// SD-33 remediation wave 4 (`AT-33-E5-003`): the same
/// base-item-plus-attached-EQMOD summation gap that
/// `apply_eqmod_armor_class_bonus` closes for `COMBAT|AC` chains recurs
/// here for `VAR` chains — confirmed by
/// `inner_sea_races:equipment:panoply_of_the_fierani_knight`'s real
/// disagreement (`ours=6`, oracle=`3`): the base item's own
/// `BONUS:VAR|ArmorCheckPenalty|6` chain is its Full Plate base ACP; its
/// `EQMOD:...Material ~ Mithril ~ Armor / Heavy` token names a real
/// corpus record whose OWN `BONUS:VAR|ArmorCheckPenalty|-3|
/// TYPE=Enhancement` chain is Mithral's real ACP improvement (already
/// *signed negative* in the corpus data, so summing — not subtracting —
/// reaches the real total: `6 + (-3) = 3`, matching the oracle exactly).
/// A modifier record with no `VAR` chain at all (materials without an
/// ACP effect, cosmetic special qualities) contributes nothing, same
/// resolve-or-skip discipline as every other `equipment_effects`
/// resolver.
pub fn apply_eqmod_var_bonus(
    base: &mut Vec<VarBonus>,
    eqmod_records: &[&CorpusEquipmentRecord],
) {
    for modifier in eqmod_records {
        for extra in compute_var_effect(modifier) {
            if let Some(existing) = base.iter_mut().find(|v| v.name == extra.name) {
                existing.bonus += extra.bonus;
            } else {
                base.push(extra);
            }
        }
    }
}

