//! Intelligent-item resolver (SD-31, operator ruling 2026-08-19: PF1e's
//! intelligent-item subsystem is IN SCOPE; the 172 `equipment_modifier`
//! units this family carries are to be built, not excluded).
//!
//! PF1's Intelligent Item rules (CRB "Magic Items -- Intelligent Items")
//! give a magic item its own Intelligence, Wisdom, and Charisma scores, an
//! Ego score, and (for items with a strong personality) an alignment. In
//! PCGen's corpus these are represented as a chain of `equipmods`-category
//! records the item's own catalog entry customizes with
//! (`CUSTOMIZATION:EQMOD=`, the same `applied_modifiers` attachment
//! `equipmods.rs`'s `resolve_weapon_to_hit_bonus` already reads --
//! `character_input::EquipmentSelection::applied_modifiers`'s own doc
//! comment), gated behind a mandatory `Intelligent Item ~ Base` prerequisite
//! (`PRETYPE:1,EQMOD=Intelligent Item ~ Base` on every other record in this
//! family, confirmed against `core_rulebook/cr_equipmods.lst` line 377 and
//! `mythic_adventures/ma_equipmods.lst` line 96):
//!
//! - `Intelligent Item ~ Base` (`core_rulebook/cr_equipmods.lst` line 354,
//!   `.COPY=`'d as `IntItemBase`) carries the item's baseline
//!   `BONUS:VAR|IntItemStatINT|10`, `BONUS:VAR|IntItemStatWIS|10`,
//!   `BONUS:VAR|IntItemStatCHA|10` (every intelligent item starts at 10 in
//!   each mental ability) plus a `BONUS:VAR|IntelligentItemEgo|<formula>`
//!   whose value is a `BaseCostTracker`-driven PCGen formula, not a
//!   literal -- this resolver reads only literal `qualifiers[2]` integers
//!   (the same `.parse::<i16>().ok()` discipline `equipmods.rs`/
//!   `magic_items.rs` already use), so that one chain is honestly skipped
//!   rather than fabricated; the item's own literal INT/WIS/CHA=10
//!   baseline still resolves when this record is attached.
//! - `Intelligent Item ~ Ability Score / <Ability> <N>` (30 CRB records,
//!   `WIS`/`INT`/`CHA` x 11..20, `.COPY=`'d under bare `IntItemStat<ABL><N>`
//!   keys at lines 822-864) carries a literal
//!   `BONUS:VAR|IntItemStat<ABL>|<delta>` (the ability score's delta over
//!   the base-10 floor, e.g. `Wisdom 15` carries `IntItemStatWIS|5`,
//!   confirmed `10 + 5 == 15`, the record's own name) plus a literal
//!   `BONUS:VAR|IntelligentItemEgo|<n>` (the same score's own Ego
//!   contribution -- PF1's rule that Ego includes the item's highest
//!   mental-ability *modifier*: Wisdom 15's mod is +2, and its own
//!   `IntelligentItemEgo` chain is literally `2`).
//! - `Legendary Item ~ Intelligent Item ~ Alignment / <Alignment>` (9
//!   Mythic Adventures records, `mythic_adventures/ma_equipmods.lst` lines
//!   96-104) carries a literal `BONUS:VAR|IntItemAlignment|<code>`, a
//!   two-digit base-3 encoding confirmed directly against all 9 records:
//!   tens digit is the Law-Chaos axis (`0`=Lawful, `1`=Neutral, `2`=Chaotic),
//!   ones digit is the Good-Evil axis (`0`=Good, `1`=Neutral, `2`=Evil) --
//!   e.g. `Chaotic Good` carries `20`, `Lawful Evil` carries `02`.
//!
//! **Deliberately NOT resolved here, honestly absent rather than
//! fabricated:** the `Intelligent Item ~ Power / ...` and
//! `Intelligent Item ~ Purpose / ...` families (spell-like-ability grants,
//! movement modes, skill-rank grants, creature-slaying purposes) --
//! real, distinct PF1 mechanics from the item's own ability-score/Ego/
//! alignment stat block this resolver targets, each needing its own
//! evaluator (a granted spell-like ability, a purpose's creature-type
//! target, ...) that is out of this cycle's bounded scope. Every one of
//! those records also carries a literal `BONUS:VAR|IntelligentItemEgo|<n>`
//! (each granted power/purpose contributes its own flat Ego points, PF1's
//! rule) -- this resolver reads that literal Ego contribution wherever it
//! is attached (honest, real, and independently correct even though the
//! record's OWN headline power is not resolved), never the power/purpose
//! itself.
//!
//! No field here is hand-rolled; every value traces back to a real,
//! verbatim corpus token, read the same way `equipmods.rs`/`magic_items.rs`
//! read their own tokens straight off the resolved record.

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;

/// An intelligent item's own alignment (CRB "Intelligent Items"), decoded
/// from the corpus's literal `BONUS:VAR|IntItemAlignment|<code>` two-digit
/// encoding (see module doc comment).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemAlignment {
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    TrueNeutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
}

impl ItemAlignment {
    /// Decodes the corpus's literal two-digit `IntItemAlignment` code.
    /// Returns `None` for any code outside the 9 real values this family's
    /// corpus records carry (`00`, `01`, `02`, `10`, `11`, `12`, `20`,
    /// `21`, `22`) -- an honest refusal, not a guessed alignment.
    pub fn from_code(code: i16) -> Option<Self> {
        let law_chaos = code / 10;
        let good_evil = code % 10;
        match (law_chaos, good_evil) {
            (0, 0) => Some(Self::LawfulGood),
            (0, 1) => Some(Self::LawfulNeutral),
            (0, 2) => Some(Self::LawfulEvil),
            (1, 0) => Some(Self::NeutralGood),
            (1, 1) => Some(Self::TrueNeutral),
            (1, 2) => Some(Self::NeutralEvil),
            (2, 0) => Some(Self::ChaoticGood),
            (2, 1) => Some(Self::ChaoticNeutral),
            (2, 2) => Some(Self::ChaoticEvil),
            _ => None,
        }
    }

    /// The player-facing label, matching PF1's own alignment names.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LawfulGood => "Lawful Good",
            Self::LawfulNeutral => "Lawful Neutral",
            Self::LawfulEvil => "Lawful Evil",
            Self::NeutralGood => "Neutral Good",
            Self::TrueNeutral => "True Neutral",
            Self::NeutralEvil => "Neutral Evil",
            Self::ChaoticGood => "Chaotic Good",
            Self::ChaoticNeutral => "Chaotic Neutral",
            Self::ChaoticEvil => "Chaotic Evil",
        }
    }
}

/// One `equipmods`-category record's literal contribution to an
/// intelligent item's own stat block. Every field is a real, literal
/// corpus value (see module doc comment); a record contributing nothing
/// to this family yields `None` from `compute_intelligent_item_effect`
/// rather than a zeroed struct.
///
/// **`ego_bonus` is a PARTIAL sum, never a resolved total Ego score**
/// (integration-cycle adversarial review, SD-31 wave 18, MEDIUM). The
/// Base record's own `IntelligentItemEgo` chain is a `BaseCostTracker`
/// formula this module deliberately skips rather than fabricates (see
/// module doc comment and `the_base_record_skips_its_own_formula_ego_chain_
/// and_its_conditional_chains`) — so summing every OTHER attached
/// modifier's literal `ego_bonus` (as `resolve_intelligent_item_contribution`
/// in `equipment_effects.rs` does) systematically UNDERSTATES a real
/// item's true Ego. A future consumer must not render this field as "the
/// item's Ego" without also resolving the Base contribution; it is safe
/// only as "this modifier's own literal Ego delta."
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct IntelligentItemContribution {
    pub intelligence_bonus: i16,
    pub wisdom_bonus: i16,
    pub charisma_bonus: i16,
    /// See the struct's own doc comment: a partial sum, not a resolved
    /// total Ego score.
    pub ego_bonus: i16,
    pub alignment: Option<ItemAlignment>,
}

/// Resolve one `equipmods` corpus record's intelligent-item contribution.
///
/// Scans every `BONUS:VAR|...` chain the record carries (unlike
/// `compute_equipmods_effect`/`compute_magic_items_effect`, which each
/// extract a single chain, a real intelligent-item record commonly carries
/// two at once -- e.g. `Intelligent Item ~ Ability Score / Wisdom 15`
/// carries both its own `IntelligentItemEgo` and `IntItemStatWIS` chains,
/// see module doc comment) and accumulates the ones this family names:
/// `IntItemStatINT`/`IntItemStatWIS`/`IntItemStatCHA`, `IntelligentItemEgo`,
/// `IntItemAlignment`. Requires an exact 3-part `[VAR, <name>, <value>]`
/// chain -- a chain with a trailing `PREVARGTEQ:`/`PREVARLTEQ:` condition
/// (e.g. the Base record's own conditional `IntItemNegativeLevel` chains)
/// is not unconditionally true, so it is deliberately excluded rather than
/// asserted. Returns `None` when the record carries none of this family's
/// tokens at all -- that means this record does not belong to the
/// intelligent-item family, not that its contribution is zero.
pub fn compute_intelligent_item_effect(record: &EquipmentRecord) -> Option<IntelligentItemContribution> {
    let mut result = IntelligentItemContribution::default();
    let mut found = false;

    for bonus in &record.bonus_chains {
        let qualifiers = &bonus.qualifiers;
        if qualifiers.len() != 3 || qualifiers[0] != "VAR" {
            continue;
        }
        let Ok(value) = qualifiers[2].parse::<i16>() else {
            continue;
        };
        match qualifiers[1].as_str() {
            "IntItemStatINT" => {
                result.intelligence_bonus += value;
                found = true;
            }
            "IntItemStatWIS" => {
                result.wisdom_bonus += value;
                found = true;
            }
            "IntItemStatCHA" => {
                result.charisma_bonus += value;
                found = true;
            }
            "IntelligentItemEgo" => {
                result.ego_bonus += value;
                found = true;
            }
            "IntItemAlignment" => {
                if let Some(alignment) = ItemAlignment::from_code(value) {
                    result.alignment = Some(alignment);
                    found = true;
                }
            }
            _ => {}
        }
    }

    found.then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;

    /// Real verbatim tokens copied from `KEY:Intelligent Item ~ Base` in
    /// `core_rulebook/cr_equipmods.lst` line 354 (trimmed to the tokens
    /// this resolver reads -- the record's real `PREVARGTEQ`-conditional
    /// `IntItemNegativeLevel` chains and formula-valued `IntelligentItemEgo`
    /// chain are omitted from this fixture on purpose to isolate the
    /// unconditional-literal assertion below; the full-record shape is
    /// covered by `the_base_record_skips_its_own_formula_ego_chain`).
    #[test]
    fn intelligent_item_base_yields_the_literal_ten_point_baseline_in_each_mental_ability() {
        let text = "Intelligent Magic Item Base\tKEY:Intelligent Item ~ Base\tTYPE:Weapon.Armor.Goods\tCOST:500\tBONUS:VAR|IntItemStatINT|10\tBONUS:VAR|IntItemStatWIS|10\tBONUS:VAR|IntItemStatCHA|10\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let record = &result.entries[0];

        let effect = compute_intelligent_item_effect(record).expect("Base record must yield a contribution");
        assert_eq!(effect.intelligence_bonus, 10);
        assert_eq!(effect.wisdom_bonus, 10);
        assert_eq!(effect.charisma_bonus, 10);
        assert_eq!(effect.ego_bonus, 0, "Base record fixture above carries no literal Ego chain");
        assert_eq!(effect.alignment, None);
    }

    /// Real verbatim tokens copied from `KEY:Intelligent Item ~ Base`'s
    /// full line, including its formula-valued
    /// `BONUS:VAR|IntelligentItemEgo|(BaseCostTracker>=1001)+...` chain
    /// and its `PREVARGTEQ:`-conditional `IntItemNegativeLevel` chains --
    /// neither is a literal `[VAR, name, value]` triple this resolver
    /// reads, so both must be honestly skipped rather than fabricated
    /// into a fake Ego/negative-level number.
    #[test]
    fn the_base_record_skips_its_own_formula_ego_chain_and_its_conditional_chains() {
        let text = "Intelligent Magic Item Base\tKEY:Intelligent Item ~ Base\tTYPE:Weapon.Armor.Goods\tCOST:500\tBONUS:VAR|IntItemNegativeLevel|1|PREVARGTEQ:IntelligentItemEgo,20\tBONUS:VAR|IntItemStatINT|10\tBONUS:VAR|IntItemStatWIS|10\tBONUS:VAR|IntItemStatCHA|10\tBONUS:VAR|BaseCostTracker|COST\tBONUS:VAR|IntelligentItemEgo|(BaseCostTracker>=1001)+(BaseCostTracker>=5001)\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let record = &result.entries[0];

        let effect = compute_intelligent_item_effect(record).expect("literal INT/WIS/CHA chains still resolve");
        assert_eq!(effect.intelligence_bonus, 10);
        assert_eq!(effect.wisdom_bonus, 10);
        assert_eq!(effect.charisma_bonus, 10);
        assert_eq!(
            effect.ego_bonus, 0,
            "the formula-valued IntelligentItemEgo chain must not be parsed into a fabricated number"
        );
    }

    /// Real verbatim tokens copied from `KEY:Intelligent Item ~ Ability
    /// Score / Wisdom 15` in `core_rulebook/cr_equipmods.lst` line 377 --
    /// carries BOTH its own Ego contribution and its Wisdom delta in the
    /// same record, proving both chains are read out of one record, not
    /// just the first match (unlike `compute_equipmods_effect`'s
    /// single-chain `find_map`).
    #[test]
    fn wisdom_fifteen_yields_its_real_stat_delta_and_its_real_ego_contribution() {
        let text = "Int Item / Stat Wisdom 15\tKEY:Intelligent Item ~ Ability Score / Wisdom 15\tTYPE:Weapon.Armor.Goods\tCOST:1400\tBONUS:VAR|IntelligentItemEgo|2\tBONUS:VAR|IntItemStatWIS|5\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let record = &result.entries[0];

        let effect = compute_intelligent_item_effect(record).expect("must yield a contribution");
        assert_eq!(effect.wisdom_bonus, 5, "10 (base, not asserted by this record) + 5 == 15, the record's own name");
        assert_eq!(effect.ego_bonus, 2);
        assert_eq!(effect.intelligence_bonus, 0);
        assert_eq!(effect.charisma_bonus, 0);
    }

    /// Real verbatim tokens copied from `KEY:Intelligent Item ~ Ability
    /// Score / Charisma 20` (the top of the CHA ladder,
    /// `core_rulebook/cr_equipmods.lst`) -- a different ability entirely,
    /// proving the ability is read from the token name, not hardcoded to
    /// Wisdom.
    #[test]
    fn charisma_twenty_yields_its_real_cha_delta() {
        let text = "Int Item / Stat Charisma 20\tKEY:Intelligent Item ~ Ability Score / Charisma 20\tTYPE:Weapon.Armor.Goods\tCOST:8000\tBONUS:VAR|IntelligentItemEgo|5\tBONUS:VAR|IntItemStatCHA|10\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let record = &result.entries[0];

        let effect = compute_intelligent_item_effect(record).expect("must yield a contribution");
        assert_eq!(effect.charisma_bonus, 10);
        assert_eq!(effect.ego_bonus, 5);
    }

    /// Real verbatim tokens copied from `KEY:Legendary Item ~ Intelligent
    /// Item ~ Alignment / Chaotic Good` in
    /// `mythic_adventures/ma_equipmods.lst` line 96 -- the alignment
    /// family's own literal `IntItemAlignment|20` code, decoded to
    /// `ChaoticGood`.
    #[test]
    fn chaotic_good_alignment_decodes_from_its_real_two_digit_code() {
        let text = "Legendary Intelligent Item / Align (CG)\tKEY:Legendary Item ~ Intelligent Item ~ Alignment / Chaotic Good\tTYPE:Mythic.Intelligent.Alignment\tCOST:0\tBONUS:VAR|IntItemAlignment|20\n";
        let result = parse_equipment_entries("ma_equipmods.lst", text);
        assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
        let record = &result.entries[0];

        let effect = compute_intelligent_item_effect(record).expect("must yield a contribution");
        assert_eq!(effect.alignment, Some(ItemAlignment::ChaoticGood));
        assert_eq!(effect.ego_bonus, 0, "the alignment chain grants no Ego of its own");
    }

    /// Real verbatim tokens copied from `KEY:Legendary Item ~ Intelligent
    /// Item ~ Alignment / Lawful Evil` -- the opposite corner of the
    /// alignment grid, proving the axis decoding is read from the code,
    /// not hardcoded to one alignment.
    #[test]
    fn lawful_evil_alignment_decodes_from_its_real_two_digit_code() {
        let text = "Legendary Intelligent Item / Align (LE)\tKEY:Legendary Item ~ Intelligent Item ~ Alignment / Lawful Evil\tTYPE:Mythic.Intelligent.Alignment\tCOST:0\tBONUS:VAR|IntItemAlignment|02\n";
        let result = parse_equipment_entries("ma_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_intelligent_item_effect(record).expect("must yield a contribution");
        assert_eq!(effect.alignment, Some(ItemAlignment::LawfulEvil));
    }

    /// Real verbatim tokens copied from `KEY:Intelligent Item ~ Power /
    /// Change Shape` (`core_rulebook/cr_equipmods.lst` line 432) -- a
    /// Power-family record, deliberately NOT resolved by this module (see
    /// its own doc comment): its own headline power is not represented
    /// here, but its real literal Ego contribution still is, since it's
    /// the same `IntelligentItemEgo` chain every family member carries.
    #[test]
    fn a_power_family_record_still_yields_its_literal_ego_contribution_but_no_ability_or_alignment() {
        let text = "Int Item / Power Change shape\tKEY:Intelligent Item ~ Power / Change Shape\tTYPE:Weapon.Armor.Goods\tCOST:10000\tBONUS:VAR|IntelligentItemEgo|2\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_intelligent_item_effect(record).expect("must yield a contribution");
        assert_eq!(effect.ego_bonus, 2);
        assert_eq!(effect.intelligence_bonus, 0);
        assert_eq!(effect.wisdom_bonus, 0);
        assert_eq!(effect.charisma_bonus, 0);
        assert_eq!(effect.alignment, None);
    }

    /// Real verbatim tokens copied from `KEY:Masterwork (Weapon)` -- an
    /// ordinary, non-intelligent-item `equipmods` record carries none of
    /// this family's tokens at all.
    #[test]
    fn an_ordinary_equipmod_has_no_intelligent_item_contribution() {
        let text = "Masterwork (Weapon)\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tCOST:0\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_intelligent_item_effect(record);
        assert_eq!(effect, None);
    }

    /// A chain naming one of this family's variables but carrying a
    /// trailing `PREVARGTEQ:` condition (a 4-part chain, not the family's
    /// real unconditional 3-part shape) must not be asserted -- it is not
    /// unconditionally true. Regression guard for the exact shape the
    /// Base record's own `IntItemNegativeLevel` chains carry.
    #[test]
    fn a_conditional_var_chain_naming_a_family_variable_is_not_asserted() {
        let text = "Conditional Proxy\tKEY:Conditional Proxy\tTYPE:Weapon.Armor.Goods\tCOST:0\tBONUS:VAR|IntItemStatWIS|99|PREVARGTEQ:SomeVar,1\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_intelligent_item_effect(record);
        assert_eq!(effect, None, "a conditional chain must not be asserted as an unconditional +99 Wisdom");
    }

    #[test]
    fn alignment_code_decodes_all_nine_real_corpus_values() {
        assert_eq!(ItemAlignment::from_code(0), Some(ItemAlignment::LawfulGood));
        assert_eq!(ItemAlignment::from_code(1), Some(ItemAlignment::LawfulNeutral));
        assert_eq!(ItemAlignment::from_code(2), Some(ItemAlignment::LawfulEvil));
        assert_eq!(ItemAlignment::from_code(10), Some(ItemAlignment::NeutralGood));
        assert_eq!(ItemAlignment::from_code(11), Some(ItemAlignment::TrueNeutral));
        assert_eq!(ItemAlignment::from_code(12), Some(ItemAlignment::NeutralEvil));
        assert_eq!(ItemAlignment::from_code(20), Some(ItemAlignment::ChaoticGood));
        assert_eq!(ItemAlignment::from_code(21), Some(ItemAlignment::ChaoticNeutral));
        assert_eq!(ItemAlignment::from_code(22), Some(ItemAlignment::ChaoticEvil));
        assert_eq!(ItemAlignment::from_code(99), None, "no real corpus record carries this code");
    }
}
