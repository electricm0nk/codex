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

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;

/// A skill-check circumstance bonus granted by a `general`-category
/// item's `BONUS:SKILL|<skill>|<n>|TYPE=Circumstance` corpus token.
#[derive(Debug, Clone, PartialEq)]
pub struct SkillCheckBonus {
    pub skill: String,
    pub bonus: i16,
}

/// Resolve one `general` corpus record's skill-check-bonus contribution.
///
/// Reads the record's first `BONUS:SKILL|<skill>|<n>|...` chain, if any.
/// A record with no such chain (the overwhelming majority of `general`
/// records — trade goods, containers, tattoos, ...) yields `None`: that
/// means this record's raw tokens do not carry the field, not that its
/// value is zero.
pub fn compute_general_effect(record: &EquipmentRecord) -> Option<SkillCheckBonus> {
    let explicit = record
        .bonus_chains
        .iter()
        .find_map(|bonus| {
            let qualifiers = &bonus.qualifiers;
            let is_skill_bonus = qualifiers.len() >= 3 && qualifiers[0] == "SKILL";
            if !is_skill_bonus {
                return None;
            }
            qualifiers[2].parse::<i16>().ok().map(|bonus_value| SkillCheckBonus {
                skill: qualifiers[1].clone(),
                bonus: bonus_value,
            })
        })
        .or_else(|| tempbonus_skill_fallback(record))?;
    Some(SkillCheckBonus {
        bonus: explicit.bonus + swim_speed_racial_bonus(record, &explicit.skill),
        ..explicit
    })
}

/// `AT-34-E3-003` (bucket `M`, equipment sub-causes, cycle 3): a
/// `TEMPBONUS:<target>|SKILL|<skill>|<n>|...` corpus token is PCGen's
/// temporary/consumable-triggered sibling of `BONUS:SKILL|<skill>|<n>|...`
/// — the real, load-bearing mechanical effect on every potion/elixir in
/// this population (`Elixir of Swimming`, `Elixir of Vision`, `Dust of
/// Appearance`, ...), none of which carry a `BONUS:` chain at all
/// (confirmed against the live corpus: `raw_bonus_chains` is empty on
/// every one). Only fires when no explicit `BONUS:SKILL` chain exists
/// (checked by the caller's `.or_else`), and only for a `<target>` of
/// `PC`/`ANYPC` (a character-side skill bonus) — a `TEMPBONUS:EQ|...`
/// (the `Lead Blades` shape: an equipment-side weapon-damage buff) is a
/// structurally different effect this function must never read as a skill
/// bonus. Only a literal, single-skill, integer-valued token is read: a
/// comma-joined skill list or a `TYPE.<Group>` wildcard is a different,
/// wider shape this fallback deliberately does not attempt (an honest
/// `None`, not a guessed value).
fn tempbonus_skill_fallback(record: &EquipmentRecord) -> Option<SkillCheckBonus> {
    record.tokens.iter().find_map(|token| {
        if token.key != "TEMPBONUS" {
            return None;
        }
        let parts: Vec<&str> = token.value.split('|').collect();
        if parts.len() < 4 || (parts[0] != "PC" && parts[0] != "ANYPC") || parts[1] != "SKILL" {
            return None;
        }
        let skill = parts[2];
        // `ALL` is PCGen's real wildcard meaning "every skill" (confirmed
        // live: `Setting Stone (Invigoration)`, `TEMPBONUS:PC|SKILL|ALL|2|
        // TYPE=Morale`) -- a blanket bonus, not a bonus to one skill
        // literally named "ALL". Reading it as a single-skill `SkillCheck
        // Bonus{skill:"ALL"}` would be a fabricated, wrong value (this
        // struct has no field for "every skill"), so it is excluded here
        // the same way a comma-joined list and a `TYPE.<Group>` wildcard
        // are: an honest `None`, not a guessed shape.
        if skill.is_empty() || skill.contains(',') || skill.starts_with("TYPE.") || skill.eq_ignore_ascii_case("ALL") {
            return None;
        }
        parts[3].parse::<i16>().ok().map(|bonus_value| SkillCheckBonus {
            skill: skill.to_string(),
            bonus: bonus_value,
        })
    })
}

/// PF1 core rule (`Core Rulebook` Swim skill entry): "A swim speed of at
/// least 5 feet gives a creature a +8 racial bonus on Swim checks." This is
/// an automatic bonus triggered by the item itself granting a swim speed
/// (a `MOVE:...Swim,<n>...` token), independent of and additive with any
/// explicit `BONUS:SKILL|Swim|...` token the same item also carries —
/// confirmed against the real pinned PCGen oracle, which sums both
/// (`ultimate_equipment:equipment:ring_of_the_sea_strider`: explicit `+8`
/// racial token + this auto-rule's `+8` = real oracle export `16`, not the
/// explicit token's bare `8`; `AT-33-E5-remainder-equipment_cycle_receipt.md`).
fn swim_speed_racial_bonus(record: &EquipmentRecord, skill: &str) -> i16 {
    if skill != "Swim" {
        return 0;
    }
    let grants_swim_speed = record.tokens.iter().any(|token| {
        token.key == "MOVE"
            && token
                .value
                .split(',')
                .any(|part| part.trim().eq_ignore_ascii_case("Swim"))
    });
    if grants_swim_speed {
        8
    } else {
        0
    }
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
#[derive(Debug, Clone, PartialEq)]
pub struct VarBonus {
    pub name: String,
    pub bonus: i16,
}

/// Resolve every `BONUS:VAR|<name(s)>|<value>` chain on one equipment
/// record into its flat per-name literal bonus rows.
///
/// A chain's own name field may itself be a comma-joined list (e.g.
/// `CMD_Disarm,CMD_Sunder`, real verbatim from `Gloves of Dueling`) — each
/// named variable gets its own row carrying the SAME literal value, the
/// same convention `compute_general_effect`'s own doc comment already
/// named for the sibling multi-skill/`ALL` `SKILL`-shape gap. Any trailing
/// qualifier past the value (`TYPE=...`) is stacking metadata, not part of
/// the magnitude, and is not read here. Returns an empty vec when the
/// record carries no `VAR` chain at all — an honest absence, not a
/// fabricated zero (matching `compute_general_effect`'s own `None`
/// convention for its own shape).
pub fn compute_var_effect(record: &EquipmentRecord) -> Vec<VarBonus> {
    record
        .bonus_chains
        .iter()
        .filter_map(|bonus| {
            let qualifiers = &bonus.qualifiers;
            if qualifiers.len() < 3 || qualifiers[0] != "VAR" {
                return None;
            }
            let value = qualifiers[2].parse::<i16>().ok()?;
            Some((qualifiers[1].as_str(), value))
        })
        .flat_map(|(names, value)| {
            names
                .split(',')
                .map(move |name| VarBonus { name: name.to_string(), bonus: value })
        })
        .collect()
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
pub fn apply_eqmod_var_bonus(base: &mut Vec<VarBonus>, eqmod_records: &[&EquipmentRecord]) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;

    /// Real verbatim tokens copied from `KEY:Thieves' Tools` in
    /// `core_rulebook/cr_equip_general.lst`.
    #[test]
    fn thieves_tools_yields_a_real_disable_device_skill_bonus() {
        let text = "Thieves' Tools\tKEY:Thieves' Tools\tTYPE:Goods.Tools.Thief.ThiefTools\tCOST:30\tWT:1\tBONUS:SKILL|Disable Device|2|TYPE=Circumstance|PRETYPE:1,Masterwork\n";
        let result = parse_equipment_entries("cr_equip_general.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Disable Device".to_string(),
                bonus: 2,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Climber's Kit` — a different
    /// skill entirely, proving the skill name is read from the token, not
    /// hardcoded.
    #[test]
    fn climbers_kit_yields_a_real_climb_skill_bonus() {
        let text = "Climber's Kit\tKEY:Climber's Kit\tTYPE:Goods.Tools.Masterwork.Resizable\tCOST:80\tWT:5\tBONUS:SKILL|Climb|2|TYPE=Circumstance\n";
        let result = parse_equipment_entries("cr_equip_general.lst", text);
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Climb".to_string(),
                bonus: 2,
            })
        );
    }

    /// `AT-34-E3-003` (bucket `M`, equipment sub-causes, cycle 3): real
    /// verbatim tokens copied from `KEY:Elixir of Swimming` in
    /// `core_rulebook/cr_equip_magic_items.lst` — a `magic_items`-category
    /// consumable whose real, load-bearing mechanical effect is a
    /// `TEMPBONUS:ANYPC|SKILL|Swim|10|TYPE=Competence` token, never a
    /// `BONUS:SKILL` chain (`raw_bonus_chains` is empty on this record —
    /// confirmed against the live corpus). `TEMPBONUS` is PCGen's
    /// temporary/consumable-triggered sibling of `BONUS` and carries the
    /// identical `SKILL|<skill>|<n>|TYPE=...` shape one segment further in
    /// (`ANYPC|SKILL|...` vs `SKILL|...`) — the same widening shape
    /// `damage_total::resolve_base_damage_dice`'s `BASEITEM:` chase used
    /// for the sibling equipment-M cycle: consulting a real, already-typed
    /// token this resolver did not yet read, not a new resolution
    /// mechanism.
    #[test]
    fn elixir_of_swimming_yields_a_real_swim_skill_bonus_from_tempbonus() {
        let text = "Elixir of Swimming\tKEY:Elixir of Swimming\tTYPE:Magic.Wondrous.Elixir.Consumable\tCOST:250\tWT:0\tTEMPBONUS:ANYPC|SKILL|Swim|10|TYPE=Competence\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Swim".to_string(),
                bonus: 10,
            })
        );
    }

    /// `Dust of Appearance` — real verbatim token, a NEGATIVE `TEMPBONUS`
    /// (a Stealth penalty, not a bonus): proves the fallback reads the
    /// literal signed integer rather than assuming a positive value.
    #[test]
    fn dust_of_appearance_yields_a_real_negative_stealth_tempbonus() {
        let text = "Dust of Appearance\tKEY:Dust of Appearance\tTYPE:Magic.Wondrous.Consumable\tCOST:1500\tWT:0\tTEMPBONUS:ANYPC|SKILL|Stealth|-30\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Stealth".to_string(),
                bonus: -30,
            })
        );
    }

    /// A record's own explicit `BONUS:SKILL` chain always wins over a
    /// `TEMPBONUS` fallback — negative control proving the fallback only
    /// fires when the explicit chain is absent, mirroring
    /// `damage_total.rs`'s `a_records_own_damage_token_wins_over_its_
    /// baseitem` negative control.
    #[test]
    fn explicit_bonus_skill_wins_over_a_tempbonus_on_the_same_record() {
        let text = "Hybrid\tKEY:Hybrid\tTYPE:Goods.Tools\tCOST:1\tWT:1\tBONUS:SKILL|Climb|2|TYPE=Circumstance\tTEMPBONUS:ANYPC|SKILL|Swim|99|TYPE=Competence\n";
        let result = parse_equipment_entries("cr_equip_general.lst", text);
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Climb".to_string(),
                bonus: 2,
            })
        );
    }

    /// `Setting Stone (Invigoration)` (`ultimate_psionics`) — real verbatim
    /// token: `TEMPBONUS:PC|SKILL|ALL|2|TYPE=Morale`. `ALL` is PCGen's
    /// wildcard for "every skill", never a skill literally named `ALL` —
    /// negative control proving the fallback refuses this shape rather
    /// than fabricating a `SkillCheckBonus{skill:"ALL"}` this struct has
    /// no way to represent correctly.
    #[test]
    fn tempbonus_skill_all_wildcard_is_never_read_as_a_single_skill() {
        let text = "Setting Stone (Invigoration)\tKEY:Setting Stone (Invigoration)\tTYPE:Wondrous\tCOST:1\tWT:0\tTEMPBONUS:PC|SKILL|ALL|2|TYPE=Morale\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        let record = &result.entries[0];

        assert_eq!(compute_general_effect(record), None);
    }

    /// A `TEMPBONUS` targeting `EQ` (an equipment-side effect, e.g. a
    /// weapon-damage-size buff — the real `Lead Blades` shape) rather than
    /// `PC`/`ANYPC` (a character-side skill bonus) is never read as a skill
    /// bonus — negative control proving the fallback is anchored on the
    /// real PC/ANYPC selector, not any `TEMPBONUS:...SKILL...` substring.
    #[test]
    fn tempbonus_targeting_eq_not_pc_is_never_read_as_a_skill_bonus() {
        let text = "Lead Blades\tKEY:Lead Blades\tTYPE:Magic.Wondrous\tCOST:1\tWT:0\tTEMPBONUS:EQ|Weapon,Melee|SKILL|Fake|5|TYPE=Temporary\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        let record = &result.entries[0];

        assert_eq!(compute_general_effect(record), None);
    }

    /// Real verbatim line copied from `ue_equip_magic_items.lst:200`
    /// (`ultimate_equipment:equipment:ring_of_the_sea_strider`). The item
    /// grants a swim speed (`MOVE:Swim,30`) *and* carries its own explicit
    /// `BONUS:SKILL|Swim|8|TYPE=Racial` token. PF1's core rule ("a swim
    /// speed of at least 5 feet gives a creature a +8 racial bonus on Swim
    /// checks") is an automatic, item-triggered bonus additive with the
    /// item's own explicit token — confirmed against the real pinned PCGen
    /// oracle (`AT-33-E5-remainder-equipment_cycle_receipt.md`'s
    /// `ring_of_the_sea_strider` disagreement: real oracle export `16`, not
    /// the explicit token's bare `8`).
    #[test]
    fn ring_of_the_sea_strider_sums_the_explicit_token_with_the_auto_swim_speed_bonus() {
        let text = "Ring of the Sea Strider\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tTYPE:Magic.Ring.LesserMedium\tCOST:14000\tWT:0\t\tMOVE:Swim,30\tSOURCEPAGE:p.176\tSPELLS:Magic Item|TIMES=1|CASTERLEVEL=7|Dimension Door\tBONUS:SKILL|Swim|8|TYPE=Racial\n";
        let result = parse_equipment_entries("ue_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Swim".to_string(),
                bonus: 16,
            })
        );
    }

    /// A plain masterwork tool's `BONUS:SKILL` token is untouched by the
    /// swim-speed rule when the item grants no swim speed at all — proves
    /// the addition is gated on the item's own `MOVE:...Swim...` token, not
    /// applied unconditionally to every `Swim` bonus.
    #[test]
    fn climbers_kit_swim_unaffected_when_no_swim_speed_granted() {
        let text = "Climber's Kit\tKEY:Climber's Kit\tTYPE:Goods.Tools.Masterwork.Resizable\tCOST:80\tWT:5\tBONUS:SKILL|Climb|2|TYPE=Circumstance\n";
        let result = parse_equipment_entries("cr_equip_general.lst", text);
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(
            effect,
            Some(SkillCheckBonus {
                skill: "Climb".to_string(),
                bonus: 2,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Backpack` — a plain
    /// container carries no `BONUS:` token at all.
    #[test]
    fn backpack_has_no_skill_bonus() {
        let text = "Backpack\tKEY:Backpack\tTYPE:Goods.Container.General.Resizable\tCONTAINS:UNLIM|Any\tCOST:2\tWT:2\n";
        let result = parse_equipment_entries("cr_equip_general.lst", text);
        let record = &result.entries[0];

        let effect = compute_general_effect(record);
        assert_eq!(effect, None);
    }

    /// SD-33 remediation wave 3 (`var-bonus-shape` lane): real verbatim
    /// tokens from `advanced_players_guide:equipment:muleback_cords`
    /// (`apg_equip_magic_items.lst`) — a single-name `BONUS:VAR|LOADSCORE|8`
    /// chain, no `TYPE=` qualifier. `compute_var_effect` genuinely did not
    /// exist before this cycle (confirmed: `general.rs`'s own
    /// `compute_general_effect` only ever matched `qualifiers[0] == "SKILL"`)
    /// — every `BONUS:VAR|...` equipment chain was silently unread by any
    /// resolver.
    #[test]
    fn muleback_cords_yields_a_single_loadscore_var_bonus() {
        let text = "Muleback Cords\tKEY:Muleback Cords\tTYPE:Magic.Wondrous.Shoulders\tCOST:1000\tWT:0.25\tBONUS:VAR|LOADSCORE|8\n";
        let result = parse_equipment_entries("apg_equip_magic_items.lst", text);
        assert_eq!(result.entries.len(), 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_var_effect(record);
        assert_eq!(effect, vec![VarBonus { name: "LOADSCORE".to_string(), bonus: 8 }]);
    }

    /// Real verbatim tokens from
    /// `advanced_players_guide:equipment:gloves_of_dueling` — TWO separate
    /// `BONUS:VAR` chains on one record, and the first chain's own name
    /// field is itself a comma-joined list (`CMD_Disarm,CMD_Sunder`) that
    /// must expand into two rows sharing the same literal value, matching
    /// the exact convention `compute_general_effect`'s own multi-skill gap
    /// (`AT-33-E5-remainder-equipment_cycle_receipt.md`) already named for
    /// the sibling `SKILL` shape.
    #[test]
    fn gloves_of_dueling_expands_comma_joined_names_and_keeps_both_chains() {
        let text = "Gloves of Dueling\tKEY:Gloves of Dueling\tTYPE:Magic.Wondrous.Glove\tCOST:15000\tWT:0.01\tBONUS:VAR|CMD_Disarm,CMD_Sunder|4\tBONUS:VAR|WeaponTrainingBase|2\n";
        let result = parse_equipment_entries("apg_equip_magic_items.lst", text);
        let record = &result.entries[0];

        let effect = compute_var_effect(record);
        assert_eq!(
            effect,
            vec![
                VarBonus { name: "CMD_Disarm".to_string(), bonus: 4 },
                VarBonus { name: "CMD_Sunder".to_string(), bonus: 4 },
                VarBonus { name: "WeaponTrainingBase".to_string(), bonus: 2 },
            ]
        );
    }

    /// A `TYPE=` qualifier (stacking metadata, not part of the magnitude)
    /// must not change the extracted value — real verbatim tokens from
    /// `core_rulebook:equipment:cloak_of_resistance_1`.
    #[test]
    fn cloak_of_resistance_reads_the_literal_value_past_a_type_qualifier() {
        let text = "Cloak of Resistance +1\tKEY:Cloak of Resistance +1\tTYPE:Magic.Wondrous.Shoulders\tCOST:1000\tWT:0.5\tBONUS:VAR|FortitudeSave_ResistanceBonus,ReflexSave_ResistanceBonus,WillSave_ResistanceBonus|1|TYPE=Resistance\n";
        let result = parse_equipment_entries("cr_equip_magic_items.lst", text);
        let record = &result.entries[0];

        let effect = compute_var_effect(record);
        assert_eq!(
            effect,
            vec![
                VarBonus { name: "FortitudeSave_ResistanceBonus".to_string(), bonus: 1 },
                VarBonus { name: "ReflexSave_ResistanceBonus".to_string(), bonus: 1 },
                VarBonus { name: "WillSave_ResistanceBonus".to_string(), bonus: 1 },
            ]
        );
    }

    /// A record with no `BONUS:VAR` chain at all yields an empty vec, not a
    /// fabricated entry — real verbatim tokens from `KEY:Climber's Kit`
    /// (carries only a `BONUS:SKILL` chain, `compute_general_effect`'s own
    /// fixture above).
    #[test]
    fn climbers_kit_has_no_var_bonus() {
        let text = "Climber's Kit\tKEY:Climber's Kit\tTYPE:Goods.Tools.Masterwork.Resizable\tCOST:80\tWT:5\tBONUS:SKILL|Climb|2|TYPE=Circumstance\n";
        let result = parse_equipment_entries("cr_equip_general.lst", text);
        let record = &result.entries[0];

        let effect = compute_var_effect(record);
        assert_eq!(effect, Vec::<VarBonus>::new());
    }

    /// SD-33 remediation wave 4 (`AT-33-E5-003`): real verbatim tokens
    /// copied from `inner_sea_races/isr_equip_arms_armor.lst:20`
    /// (`Panoply of the Fierani Knight`) plus its own `EQMOD:`-referenced
    /// Mithral Heavy Armor modifier record
    /// (`core_rulebook/cr_equipmods.lst:104`). The base item's own
    /// `ArmorCheckPenalty` chain alone (`6`) is Full Plate's base ACP;
    /// the pinned oracle's real total is `3`
    /// (`AT-33-E5-003.combined-oracle-results.json`) — Mithral's own
    /// separate, already-signed-negative `-3` chain, summed by
    /// `apply_eqmod_var_bonus`.
    #[test]
    fn eqmod_referenced_material_var_chain_sums_into_the_base_items_var_bonus() {
        let base_text = "Panoply of the Fierani Knight\tKEY:Panoply of the Fierani Knight\tTYPE:Armor.Magic.Heavy.ArmorProfHeavy.Suit.Specific\tCOST:1500\tWT:50\tACCHECK:-6\tEQMOD:Special Ability ~ Enhancement Cost|12000.Special Ability ~ +2 ~ Armor.Material ~ Mithril ~ Armor / Heavy\tMAXDEX:1\tSPELLFAILURE:35\tBONUS:COMBAT|AC|9|TYPE=Armor\tBONUS:VAR|ArmorCheckPenalty|6\n";
        let result = parse_equipment_entries("isr_equip_arms_armor.lst", base_text);
        let base_record = &result.entries[0];

        let mithral_text = "Mithral\tKEY:Material ~ Mithril ~ Armor / Heavy\tTYPE:BaseMaterial.MasterworkQuality.Armor\tCOST:9000\tBONUS:VAR|ArmorCheckPenalty|-3|TYPE=Enhancement\tBONUS:EQMARMOR|ACCHECK|3|TYPE=Enhancement.REPLACE\tBONUS:EQMARMOR|MAXDEX|2\tBONUS:EQMARMOR|SPELLFAILURE|-10|TYPE=Enhancement\n";
        let mithral_result = parse_equipment_entries("cr_equipmods.lst", mithral_text);
        let mithral_record = &mithral_result.entries[0];

        let mut effect = compute_var_effect(base_record);
        assert_eq!(
            effect,
            vec![VarBonus { name: "ArmorCheckPenalty".to_string(), bonus: 6 }],
            "the base item's own chain alone is Full Plate's base ACP"
        );

        apply_eqmod_var_bonus(&mut effect, &[mithral_record]);
        assert_eq!(
            effect,
            vec![VarBonus { name: "ArmorCheckPenalty".to_string(), bonus: 3 }],
            "Mithral's own separate, already-negative-signed VAR chain must sum in: 6 + (-3) = 3"
        );
    }
}
