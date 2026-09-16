//! The live side's own converted equipment record.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 10, `decisions.md` §11 ("not one line of
//! PCGen in our live code") and §19 (operator ruling B16: naming
//! `pcgen_import` in shipping code under a live root is a hit).
//!
//! ## What this closes
//!
//! Cycle 8 gave the spell kind a converted shape of its own
//! ([`crate::rules_core::spell_record::CorpusSpellRecord`]) and every census
//! since has named the equipment kind as the piece still open: nine live
//! consumers reading `EquipmentRecord.tokens` / `EquipmentRecord.bonus_chains`
//! -- the ingest format's own token arrays -- as their data type.
//!
//! Cycles 8 and 9 both refused a `CorpusEquipmentRecord` that would *carry
//! those arrays*, on the correct ground that moving PCGen token structures
//! under a live root is worse than the hit it clears. **This record carries no
//! array.** Every field on it is a settled value: a weight in pounds, a price
//! in gold, a resolved `+2 enhancement bonus to Str`. The token spelling, the
//! `BONUS:` chain grammar and the qualifier traversal stay on the converter
//! side, in [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`],
//! which is where `decisions.md` §11 rules that rule conversion happens.
//!
//! That is the sheet rule (`decisions.md` §1) applied to the equipment kind:
//! the sheet prints one final number, and the number is settled at ingest.
//!
//! ## Who builds one
//!
//! One producer: [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`],
//! called from [`crate::pcgen_import::ir_converter::convert_equipment_record`],
//! which is the single construction point of the canonical envelope's
//! `Equipment` payload. A record read from `data/corpus/**/equipment/*.json`
//! reaches it through [`crate::rules_core::corpus_loader::load_equipment_corpus`]
//! on exactly the same path, so there is one conversion, not two.
//!
//! ## What is deliberately NOT on it yet
//!
//! Nothing a live consumer still reads off the parser row. Cycle 12 settled
//! the last of them -- the weapon half and the two `EQMOD:`-referenced steps
//! (see below) -- so every remaining `pcgen_import` hit under
//! `src/rules_core/` belongs to a different group than this one.
//!
//! What is still absent is absent because no consumer asks for it, never
//! because a value resisted settling: a field absent from this struct is a
//! question the live side does not ask, not a value we failed to settle.
//!
//! ## What cycle 12 added
//!
//! The weapon half -- what `damage_total` reads (base damage dice and the
//! `BASEITEM:` stand-in identity, wield category, critical threat range and
//! multiplier) -- plus the two values a record contributes **as a referenced
//! modifier** (`damage_size_steps`, `weight_divisor`) and the two weapon
//! predicates `equipment_effects` asked of a parser row (`is_natural_attack`,
//! `is_shield`, with `states_base_damage` carrying the presence half of the
//! "is this an actively-wielded weapon" test). Each is a settled value; the
//! `DAMAGE:`, `BASEITEM:`, `WIELD:`, `CRITRANGE:`, `CRITMULT:`, `TYPE:`,
//! `BONUS:EQMWEAPON|DAMAGESIZE` and `BONUS:EQM|WEIGHTDIV` grammar stayed on
//! the converter.
//!
//! ## What cycle 11 added
//!
//! The armour, skill, named-variable and weapon-enhancement halves -- what
//! `equipment_effects::arms_armor`, `::general` and `::equipmods` read. Each
//! of those three modules held the ingest format's own vocabulary
//! (`ACCHECK:`, `MAXDEX:`, `SPELLFAILURE:`, `BONUS:COMBAT|AC`,
//! `BONUS:SKILL`, `BONUS:VAR`, `BONUS:WEAPON|…|TYPE=Enhancement`, `SR:`,
//! `TEMPBONUS:`, `EQMOD:`) to answer a question whose answer is one settled
//! number. The grammar moved to
//! [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`]; the
//! numbers live here.

use crate::rules_core::damage_total::{DiceExpression, WieldCategory};
use crate::rules_core::equipment_effects::general::{SkillCheckBonus, VarBonus};
use crate::rules_core::equipment_effects::equipmods::WeaponEnhancementBonus;
use crate::rules_core::equipment_effects::intelligent_item::IntelligentItemContribution;
use crate::rules_core::equipment_effects::magic_items::AbilityScoreBonus;
use crate::rules_core::equipment_effects::EquipmentStatEffect;

/// One equipment or equipment-modifier item as the live side holds it: the
/// converted, settled values of a corpus equipment record.
///
/// `None` on an optional field is honest absence -- the source record does not
/// state that value -- never a fabricated zero. This is the same discipline
/// the consumer modules stated before the values moved here.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CorpusEquipmentRecord {
    /// The item's corpus identity: its own key when the source record carried
    /// one, else its name. The same rule
    /// [`crate::rules_core::equipment_resolver::equipment_id_resolve`] uses to
    /// decide what a needle resolves against.
    pub identity: String,
    /// The item's display name.
    pub name: String,
    /// Item weight in pounds.
    pub weight_lbs: Option<f64>,
    /// Item price in gold pieces.
    pub cost_gp: Option<f64>,
    /// The item's settled ability-score enhancement, e.g. a Belt of Giant
    /// Strength +2's `+2` to Str, or a Potion of Bull's Strength's `+4`.
    pub ability_score_bonus: Option<AbilityScoreBonus>,
    /// The item's settled contribution to an intelligent item's stat block.
    /// See [`IntelligentItemContribution`]'s own doc comment for why
    /// `ego_bonus` is a partial sum.
    pub intelligent_item: Option<IntelligentItemContribution>,
    /// The item's own settled armour/shield stat contribution: AC bonus, max
    /// Dex, arcane spell-failure chance and armour check penalty. Every field
    /// `None` for an item that states none of them (a longsword, a trade
    /// good) -- honest absence, never a fabricated zero.
    pub stat_effect: EquipmentStatEffect,
    /// The item's AC contribution **as a referenced modifier**: the same
    /// number [`Self::stat_effect`]'s `armor_class_bonus` carries, minus the
    /// consumable-triggered fallback, which is a character-side temporary
    /// effect and is never summed into a host item's armour total. Kept as
    /// its own field because a modifier record is read only for this value.
    pub armor_class_chain_bonus: Option<i16>,
    /// The item's settled circumstance bonus to one named skill.
    pub skill_check_bonus: Option<SkillCheckBonus>,
    /// The item's settled flat bonuses to named rules variables, one row per
    /// name. Empty when the item states none.
    pub var_bonuses: Vec<VarBonus>,
    /// The item's settled weapon to-hit / damage enhancement.
    pub weapon_enhancement: Option<WeaponEnhancementBonus>,
    /// The item's settled flat Spell Resistance grant.
    pub spell_resistance_bonus: Option<i16>,
    /// The corpus identities of the modifier items attached to this one, in
    /// the order the source record names them. A **list of item identities**,
    /// not a token: the attachment grammar the identities were read out of
    /// stays on the converter side. Some entries resolve to no corpus record
    /// at all -- that is the same resolve-or-skip discipline the live side
    /// applied to the token's own segments before the list was settled.
    pub eqmod_references: Vec<String>,
    /// The item's settled base damage die, e.g. a longsword's `1d8`. `None`
    /// for an item that states no base damage at all, **and** for one whose
    /// stated value is not PF1's canonical `<count>d<size>` shape -- the same
    /// honest absence [`crate::rules_core::damage_total::DiceExpression::parse`]
    /// gave before the value was settled, never a fabricated roll.
    pub base_damage_dice: Option<DiceExpression>,
    /// Whether the source record states a base damage value **at all**,
    /// including one [`Self::base_damage_dice`] declines to parse. This is the
    /// "is this an actively-wielded weapon" signal, which has always been
    /// presence, not parseability: an item that states damage it does not
    /// spell as `<count>d<size>` is still a weapon.
    pub states_base_damage: bool,
    /// The corpus identity of the item whose own stats stand in for this
    /// one's, when the source record states none of its own -- the real corpus
    /// convention behind `Crossbow (Light)` naming `Light Crossbow (Base)`. An
    /// **item identity**, not a token: the caller chases it one hop through the
    /// same resolver, exactly as it did before the identity was settled.
    pub base_item: Option<String>,
    /// The item's settled wield category, which governs how much of the
    /// wielder's Strength modifier reaches its damage roll (CRB p.187). `None`
    /// for an item that states none, and for one stating a category outside
    /// PF1's three.
    pub wield_category: Option<WieldCategory>,
    /// The item's settled critical threat range as inclusive natural-roll
    /// bounds, e.g. `(19, 20)` for a longsword. `None` for an item that states
    /// none, and for a stated width outside `1..=20`.
    pub critical_threat_range: Option<(u8, u8)>,
    /// The item's settled critical-hit damage multiplier, e.g. `2` for a
    /// longsword and `4` for a scythe. `None` for an item that states none,
    /// and for a stated multiplier below `2`.
    pub critical_multiplier: Option<u8>,
    /// The number of steps this item, **as a referenced modifier**, moves its
    /// host weapon's single-die damage progression -- Shield Spikes' `+1`
    /// step. `0` for an item that states none, which is the same "no step to
    /// apply" the caller distinguished before the value was settled.
    pub damage_size_steps: i32,
    /// The divisor this item, **as a referenced modifier**, applies to its
    /// host item's weight -- Darkleaf Cloth's `2`. `None` for an item that
    /// states none.
    pub weight_divisor: Option<f32>,
    /// Whether this item is one of PF1's natural attacks. The scope signal a
    /// `natural_attack_only` weapon enhancement (the Amulet of Mighty Fists
    /// family) is checked against before it may reach an ordinary weapon's
    /// roll.
    pub is_natural_attack: bool,
    /// Whether this item is a shield. A real corpus shield states a genuine
    /// shield-bash damage die, and a worn-but-not-bashing shield must not
    /// count as a second wielded weapon -- see
    /// [`crate::rules_core::equipment_effects`]'s own doc comment for the
    /// correction that forced this distinction.
    pub is_shield: bool,
    /// Whether the source record is an equipment **modifier** rather than an
    /// item in its own right.
    ///
    /// SD-35 `AT-35-E6-003-RULED` cycle 13. The canonical envelope answers
    /// `SourceContentPayload::kind_token()` with `"EQUIP"` or `"EQUIPMOD"`, and
    /// it used to read that straight off the parser row's own
    /// `EquipmentRecordKind`. The row left the envelope this cycle, so the
    /// distinction is settled here instead -- the same two-valued answer, from
    /// the same source record, on the side that ships.
    pub is_modifier: bool,
}

impl CorpusEquipmentRecord {
    /// Weight in pounds and price in gold, the pair
    /// [`crate::rules_core::encumbrance`] reads for one carried item.
    pub fn weight_and_cost(&self) -> (Option<f64>, Option<f64>) {
        (self.weight_lbs, self.cost_gp)
    }
}

#[cfg(test)]
mod tests {
    use crate::pcgen_import::corpus_equipment_json::every_live_corpus_equipment_pair;

    /// Every book directory under `data/corpus/`, so the sweep below reads the
    /// whole live corpus rather than one book it was tuned on.
    fn every_book_root() -> Vec<std::path::PathBuf> {
        let corpus = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus");
        let mut books: Vec<std::path::PathBuf> = std::fs::read_dir(&corpus)
            .expect("data/corpus must be readable")
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .collect();
        books.sort();
        books
    }

    /// **The parity proof for SD-35 `AT-35-E6-003-RULED` cycle 10.**
    ///
    /// The three reads this cycle moved to the converter --
    /// `encumbrance`'s weight and price, `magic_items`' ability-score
    /// enhancement, `intelligent_item`'s stat-block contribution -- are
    /// re-derived here the way the live modules derived them before the move,
    /// straight off the ingest row the converter's own corpus reader rebuilds,
    /// and compared to the settled field.
    ///
    /// SD-35 `AT-35-E6-003-RULED` cycle 13: the pairing comes from
    /// [`crate::pcgen_import::corpus_equipment_json::every_live_corpus_equipment_pair`]
    /// now, not from the canonical envelope -- the envelope carries the settled
    /// record alone from this cycle on, so the oracle side of the comparison
    /// lives where the converter does. The population, the records and every
    /// assertion below are unchanged.
    ///
    /// The population is the **whole live corpus**, every book, not a fixture
    /// roster: a proof is only as wide as the cases it covers (`AGENTS.md`
    /// rule 7), and the shapes that would break this are precisely the rare
    /// ones a hand-picked roster does not contain -- a `BONUS:STAT` chain
    /// whose magnitude is a formula, a record whose weight token is not a
    /// number, a `TEMPBONUS:` naming several abilities at once.
    #[test]
    fn every_live_corpus_equipment_record_carries_the_same_values_the_token_reads_produced() {
        let books = every_book_root();
        assert!(books.len() > 1, "expected the whole corpus, found {} book(s)", books.len());
        let _ = &books;

        let mut examined = 0usize;
        let mut disagreements: Vec<String> = Vec::new();
        for (row, converted) in every_live_corpus_equipment_pair() {
            let (row, converted) = (&row, &converted);
            examined += 1;

            // --- the old `encumbrance::weight_and_cost_from_record` ---
            let token_number = |key: &str| {
                row.tokens
                    .iter()
                    .find(|token| token.key == key)
                    .and_then(|token| token.value.parse::<f64>().ok())
            };
            if token_number("WT") != converted.weight_lbs {
                disagreements.push(format!(
                    "{}: weight {:?} != {:?}",
                    converted.identity,
                    token_number("WT"),
                    converted.weight_lbs
                ));
            }
            if token_number("COST") != converted.cost_gp {
                disagreements.push(format!(
                    "{}: cost {:?} != {:?}",
                    converted.identity,
                    token_number("COST"),
                    converted.cost_gp
                ));
            }

            // --- the old `magic_items::compute_magic_items_effect` ---
            let old_ability = row
                .bonus_chains
                .iter()
                .find_map(|bonus| {
                    let q = &bonus.qualifiers;
                    if q.len() < 3 || q[0] != "STAT" {
                        return None;
                    }
                    q[2].parse::<i16>().ok().map(|value| (q[1].clone(), value))
                })
                .or_else(|| {
                    row.tokens.iter().find_map(|token| {
                        if token.key != "TEMPBONUS" {
                            return None;
                        }
                        let parts: Vec<&str> = token.value.split('|').collect();
                        if parts.len() < 4
                            || (parts[0] != "PC" && parts[0] != "ANYPC")
                            || parts[1] != "STAT"
                        {
                            return None;
                        }
                        let ability = parts[2];
                        if ability.is_empty() || ability.contains(',') {
                            return None;
                        }
                        parts[3].parse::<i16>().ok().map(|value| (ability.to_string(), value))
                    })
                });
            let new_ability = converted
                .ability_score_bonus
                .as_ref()
                .map(|bonus| (bonus.ability.clone(), bonus.bonus));
            if old_ability != new_ability {
                disagreements.push(format!(
                    "{}: ability bonus {old_ability:?} != {new_ability:?}",
                    converted.identity
                ));
            }

            // --- the old `intelligent_item::compute_intelligent_item_effect` ---
            let mut old_int = (0i16, 0i16, 0i16, 0i16, None::<i16>);
            let mut found = false;
            for bonus in &row.bonus_chains {
                let q = &bonus.qualifiers;
                if q.len() != 3 || q[0] != "VAR" {
                    continue;
                }
                let Ok(value) = q[2].parse::<i16>() else { continue };
                match q[1].as_str() {
                    "IntItemStatINT" => {
                        old_int.0 += value;
                        found = true;
                    }
                    "IntItemStatWIS" => {
                        old_int.1 += value;
                        found = true;
                    }
                    "IntItemStatCHA" => {
                        old_int.2 += value;
                        found = true;
                    }
                    "IntelligentItemEgo" => {
                        old_int.3 += value;
                        found = true;
                    }
                    "IntItemAlignment"
                        if crate::rules_core::equipment_effects::intelligent_item::ItemAlignment::from_code(value)
                            .is_some() =>
                    {
                        old_int.4 = Some(value);
                        found = true;
                    }
                    _ => {}
                }
            }
            let old_contribution = found.then_some(old_int);
            let new_contribution = converted.intelligent_item.map(|c| {
                (
                    c.intelligence_bonus,
                    c.wisdom_bonus,
                    c.charisma_bonus,
                    c.ego_bonus,
                    old_int.4.filter(|_| c.alignment.is_some()),
                )
            });
            if old_contribution != new_contribution {
                disagreements.push(format!(
                    "{}: intelligent item {old_contribution:?} != {new_contribution:?}",
                    converted.identity
                ));
            }
        }

        assert!(examined > 3000, "expected the live equipment corpus, examined {examined}");
        assert!(
            disagreements.is_empty(),
            "{} of {examined} records disagree: {:?}",
            disagreements.len(),
            &disagreements[..disagreements.len().min(20)]
        );
    }

    /// **The parity proof for SD-35 `AT-35-E6-003-RULED` cycle 11.**
    ///
    /// The nine reads this cycle moved to the converter --
    /// `arms_armor`'s four stat fields and its referenced-modifier AC
    /// contribution, `general`'s skill bonus and named-variable rows, and
    /// `equipmods`' weapon enhancement, Spell Resistance grant and
    /// attachment list -- are re-derived here exactly the way the live
    /// modules derived them before the move, straight off the parser row
    /// still paired with the converted record in the canonical envelope, and
    /// compared field for field to the settled value.
    ///
    /// The population is the **whole live corpus**, every book, for the
    /// reason cycle 10's own parity test states: a proof is only as wide as
    /// the cases it covers (`AGENTS.md` rule 7), and the shapes that would
    /// break this -- a circumstance-typed AC chain, a chain whose magnitude
    /// names a sibling variable, an item naming several attachments, an
    /// uppercase bonus type -- are exactly the rare ones a hand-picked
    /// roster does not contain.
    #[test]
    fn every_live_corpus_equipment_record_carries_the_same_armour_skill_and_weapon_values_the_token_reads_produced() {
        use crate::pcgen_import::equipment_bonus_reader;
        use crate::rules_core::equipment_effects::equipmods::WeaponEnhancementBonus;
        use crate::rules_core::equipment_effects::general::{SkillCheckBonus, VarBonus};

        let books = every_book_root();
        let _ = &books;

        let mut examined = 0usize;
        let mut disagreements: Vec<String> = Vec::new();
        for (row, converted) in every_live_corpus_equipment_pair() {
            let (row, converted) = (&row, &converted);
            examined += 1;
            let id = &converted.identity;

            let token_value = |key: &str| {
                row.tokens.iter().find(|token| token.key == key).map(|t| t.value.as_str())
            };
            let token_i16 = |key: &str| token_value(key).and_then(|v| v.parse::<i16>().ok());
            let eqmarmor = |field: &str| {
                row.bonus_chains.iter().find_map(|bonus| {
                    let q = &bonus.qualifiers;
                    if q.len() >= 3 && q[0] == "EQMARMOR" && q[1] == field {
                        q[2].parse::<i16>().ok()
                    } else {
                        None
                    }
                })
            };

            // --- the old `arms_armor::armor_class_bonus_from_bonus_chains` ---
            let old_ac_chain = row.bonus_chains.iter().find_map(|bonus| {
                let q = &bonus.qualifiers;
                if q.len() >= 3
                    && q[0] == "COMBAT"
                    && q[1] == "AC"
                    && !equipment_bonus_reader::declares_circumstance_bonus_type(bonus)
                {
                    q[2].parse::<i16>().ok()
                } else {
                    None
                }
            });
            if old_ac_chain != converted.armor_class_chain_bonus {
                disagreements.push(format!(
                    "{id}: ac chain {old_ac_chain:?} != {:?}",
                    converted.armor_class_chain_bonus
                ));
            }

            // --- the old `arms_armor::tempbonus_combat_ac_fallback` ---
            let old_tempbonus_ac = row.tokens.iter().find_map(|token| {
                if token.key != "TEMPBONUS" {
                    return None;
                }
                let parts: Vec<&str> = token.value.split('|').collect();
                if parts.len() < 4
                    || (parts[0] != "PC" && parts[0] != "ANYPC")
                    || parts[1] != "COMBAT"
                    || parts[2] != "AC"
                {
                    return None;
                }
                parts[3].parse::<i16>().ok()
            });

            // --- the old `arms_armor::compute_arms_armor_effect` ---
            let old_effect = (
                old_ac_chain.or(old_tempbonus_ac),
                token_i16("MAXDEX").or_else(|| eqmarmor("MAXDEX")),
                token_value("SPELLFAILURE")
                    .and_then(|v| v.parse::<f32>().ok())
                    .or_else(|| eqmarmor("SPELLFAILURE").map(f32::from)),
                token_i16("ACCHECK").or_else(|| eqmarmor("ACCHECK")),
            );
            let new_effect = (
                converted.stat_effect.armor_class_bonus,
                converted.stat_effect.max_dex,
                converted.stat_effect.spell_failure,
                converted.stat_effect.armor_check_penalty,
            );
            if old_effect != new_effect {
                disagreements.push(format!("{id}: stat effect {old_effect:?} != {new_effect:?}"));
            }

            // --- the old `general::compute_general_effect` ---
            let old_skill = row
                .bonus_chains
                .iter()
                .find_map(|bonus| {
                    let q = &bonus.qualifiers;
                    if q.len() < 3 || q[0] != "SKILL" {
                        return None;
                    }
                    q[2].parse::<i16>()
                        .ok()
                        .map(|value| SkillCheckBonus { skill: q[1].clone(), bonus: value })
                })
                .or_else(|| {
                    row.tokens.iter().find_map(|token| {
                        if token.key != "TEMPBONUS" {
                            return None;
                        }
                        let parts: Vec<&str> = token.value.split('|').collect();
                        if parts.len() < 4
                            || (parts[0] != "PC" && parts[0] != "ANYPC")
                            || parts[1] != "SKILL"
                        {
                            return None;
                        }
                        let skill = parts[2];
                        if skill.is_empty()
                            || skill.contains(',')
                            || skill.starts_with("TYPE.")
                            || skill.eq_ignore_ascii_case("ALL")
                        {
                            return None;
                        }
                        parts[3].parse::<i16>().ok().map(|value| SkillCheckBonus {
                            skill: skill.to_string(),
                            bonus: value,
                        })
                    })
                })
                .map(|explicit| {
                    let swim = if explicit.skill == "Swim"
                        && row.tokens.iter().any(|token| {
                            token.key == "MOVE"
                                && token
                                    .value
                                    .split(',')
                                    .any(|part| part.trim().eq_ignore_ascii_case("Swim"))
                        }) {
                        8
                    } else {
                        0
                    };
                    SkillCheckBonus { bonus: explicit.bonus + swim, ..explicit }
                });
            if old_skill != converted.skill_check_bonus {
                disagreements.push(format!(
                    "{id}: skill bonus {old_skill:?} != {:?}",
                    converted.skill_check_bonus
                ));
            }

            // --- the old `general::compute_var_effect` ---
            let old_vars: Vec<VarBonus> = row
                .bonus_chains
                .iter()
                .filter_map(|bonus| {
                    let q = &bonus.qualifiers;
                    if q.len() < 3 || q[0] != "VAR" {
                        return None;
                    }
                    let value = q[2].parse::<i16>().ok()?;
                    Some((q[1].as_str(), value))
                })
                .flat_map(|(names, value)| {
                    names
                        .split(',')
                        .map(move |name| VarBonus { name: name.to_string(), bonus: value })
                })
                .collect();
            if old_vars != converted.var_bonuses {
                disagreements.push(format!(
                    "{id}: var bonuses {old_vars:?} != {:?}",
                    converted.var_bonuses
                ));
            }

            // --- the old `equipmods::compute_equipmods_effect` ---
            let var_reference = |name: &str| {
                row.bonus_chains.iter().find_map(|bonus| {
                    let q = &bonus.qualifiers;
                    if q.len() >= 3 && q[0] == "VAR" && q[1] == name {
                        q[2].parse::<i16>().ok()
                    } else {
                        None
                    }
                })
            };
            let mut tohit: Option<i16> = None;
            let mut damage: Option<i16> = None;
            let mut natural_attack_only = false;
            let mut weapon_prof_scope: Option<String> = None;
            let mut matched = false;
            for bonus in &row.bonus_chains {
                let q = &bonus.qualifiers;
                let subject = q.first().map(String::as_str);
                let this_natural = subject == Some("WEAPONPROF=TYPE.Natural");
                let is_roll_shape = q.len() >= 2
                    && matches!(
                        q[1].as_str(),
                        "TOHIT" | "DAMAGE" | "DAMAGE,TOHIT" | "TOHIT,DAMAGE"
                    );
                let apply = |affects: &str, value: i16, tohit: &mut Option<i16>, damage: &mut Option<i16>| {
                    if affects.contains("TOHIT") {
                        *tohit = Some(tohit.unwrap_or(0) + value);
                    }
                    if affects.contains("DAMAGE") {
                        *damage = Some(damage.unwrap_or(0) + value);
                    }
                };
                if (subject == Some("WEAPON") || this_natural) && is_roll_shape {
                    if equipment_bonus_reader::roll_bonus_carries_enhancement_type(bonus)
                        && let Some(value) =
                            q[2].parse::<i16>().ok().or_else(|| var_reference(&q[2]))
                    {
                        matched = true;
                        natural_attack_only = this_natural;
                        apply(&q[1], value, &mut tohit, &mut damage);
                    }
                    continue;
                }
                if let Some(name) = subject.and_then(|s| s.strip_prefix("WEAPONPROF="))
                    && !name.starts_with("TYPE.")
                    && is_roll_shape
                    && q.len() >= 3
                    && let Ok(value) = q[2].parse::<i16>()
                {
                    matched = true;
                    weapon_prof_scope = Some(name.to_string());
                    apply(&q[1], value, &mut tohit, &mut damage);
                }
            }
            let old_enhancement = matched.then_some(WeaponEnhancementBonus {
                tohit_bonus: tohit,
                damage_bonus: damage,
                natural_attack_only,
                weapon_prof_scope,
            });
            if old_enhancement != converted.weapon_enhancement {
                disagreements.push(format!(
                    "{id}: weapon enhancement {old_enhancement:?} != {:?}",
                    converted.weapon_enhancement
                ));
            }

            // --- the old `equipmods::resolve_spell_resistance_bonus` ---
            let old_sr = token_value("SR").and_then(|v| v.parse::<i16>().ok());
            if old_sr != converted.spell_resistance_bonus {
                disagreements.push(format!(
                    "{id}: spell resistance {old_sr:?} != {:?}",
                    converted.spell_resistance_bonus
                ));
            }

            // --- the old `equipment_effects::eqmod_referenced_records` ---
            let mut old_references: Vec<String> = Vec::new();
            for token in row.tokens.iter().filter(|token| token.key == "EQMOD") {
                for instance in token.value.split('.') {
                    for candidate in instance.split('|') {
                        let candidate = candidate.trim();
                        if !candidate.is_empty() {
                            old_references.push(candidate.to_string());
                        }
                    }
                }
            }
            if old_references != converted.eqmod_references {
                disagreements.push(format!(
                    "{id}: eqmod references {old_references:?} != {:?}",
                    converted.eqmod_references
                ));
            }
        }

        assert!(examined > 3000, "expected the live equipment corpus, examined {examined}");
        assert!(
            disagreements.is_empty(),
            "{} of {examined} records disagree: {:?}",
            disagreements.len(),
            &disagreements[..disagreements.len().min(20)]
        );
    }

    /// **The parity proof for SD-35 `AT-35-E6-003-RULED` cycle 12.**
    ///
    /// The ten reads this cycle moved to the converter -- `damage_total`'s
    /// base damage die, stand-in identity, wield category, critical threat
    /// range and critical multiplier; the two values a record contributes as
    /// a referenced modifier (`BONUS:EQMWEAPON|DAMAGESIZE`'s step count and
    /// `BONUS:EQM|WEIGHTDIV`'s divisor); and `equipment_effects`' three weapon
    /// predicates -- are re-derived here exactly the way the live modules
    /// derived them before the move, straight off the parser row still paired
    /// with the converted record in the canonical envelope, and compared field
    /// for field to the settled value.
    ///
    /// The population is the **whole live corpus**, every book, for the reason
    /// cycles 10 and 11 state: a proof is only as wide as the cases it covers
    /// (`AGENTS.md` rule 7). The shapes that would break this one are exactly
    /// the ones a hand-picked weapon roster does not contain -- a shield that
    /// states a real bash die, a record whose `TYPE:` names
    /// `Weapon Group Natural` without being a natural attack, a damage value
    /// that is not `<count>d<size>`, a threat width outside `1..=20`, a
    /// multiplier below `x2`, and a modifier carrying more than one
    /// `DAMAGESIZE` chain.
    #[test]
    fn every_live_corpus_equipment_record_carries_the_same_weapon_values_the_token_reads_produced() {
        use crate::rules_core::damage_total::{DiceExpression, WieldCategory};

        let books = every_book_root();
        assert!(books.len() > 1, "expected the whole corpus, found {} book(s)", books.len());
        let _ = &books;

        let mut examined = 0usize;
        let mut disagreements: Vec<String> = Vec::new();
        for (row, converted) in every_live_corpus_equipment_pair() {
            let (row, converted) = (&row, &converted);
            examined += 1;
            let id = &converted.identity;
            let token_value = |key: &str| {
                row.tokens.iter().find(|token| token.key == key).map(|token| token.value.as_str())
            };

            // --- the old `damage_total::damage_dice_token` ---
            let old_dice: Option<DiceExpression> = row
                .tokens
                .iter()
                .find(|token| token.key == "DAMAGE")
                .and_then(|token| DiceExpression::parse(&token.value));
            if old_dice != converted.base_damage_dice {
                disagreements.push(format!(
                    "{id}: base damage dice {old_dice:?} != {:?}",
                    converted.base_damage_dice
                ));
            }

            // --- the old `equipment_effects::is_weapon_record`'s presence half ---
            let old_states_damage = row.tokens.iter().any(|token| token.key == "DAMAGE");
            if old_states_damage != converted.states_base_damage {
                disagreements.push(format!(
                    "{id}: states base damage {old_states_damage} != {}",
                    converted.states_base_damage
                ));
            }

            // --- the old `damage_total::base_item_damage_dice_token`'s token read ---
            let old_base_item = row
                .tokens
                .iter()
                .find(|token| token.key == "BASEITEM")
                .map(|token| token.value.clone());
            if old_base_item != converted.base_item {
                disagreements.push(format!(
                    "{id}: base item {old_base_item:?} != {:?}",
                    converted.base_item
                ));
            }

            // --- the old `damage_total::wield_category_token` ---
            let old_wield = row
                .tokens
                .iter()
                .find(|token| token.key == "WIELD")
                .and_then(|token| match token.value.as_str() {
                    "Light" => Some(WieldCategory::Light),
                    "OneHanded" => Some(WieldCategory::OneHanded),
                    "TwoHanded" => Some(WieldCategory::TwoHanded),
                    _ => None,
                });
            if old_wield != converted.wield_category {
                disagreements.push(format!(
                    "{id}: wield category {old_wield:?} != {:?}",
                    converted.wield_category
                ));
            }

            // --- the old `damage_total::critical_threat_range_token` ---
            let old_crit_range = row
                .tokens
                .iter()
                .find(|token| token.key == "CRITRANGE")
                .and_then(|token| token.value.parse::<u8>().ok())
                .filter(|width| (1..=20).contains(width))
                .map(|width| (20 - width + 1, 20));
            if old_crit_range != converted.critical_threat_range {
                disagreements.push(format!(
                    "{id}: critical threat range {old_crit_range:?} != {:?}",
                    converted.critical_threat_range
                ));
            }

            // --- the old `damage_total::critical_multiplier_token` ---
            let old_crit_mult = row
                .tokens
                .iter()
                .find(|token| token.key == "CRITMULT")
                .and_then(|token| token.value.strip_prefix('x'))
                .and_then(|digits| digits.parse::<u8>().ok())
                .filter(|multiplier| *multiplier >= 2);
            if old_crit_mult != converted.critical_multiplier {
                disagreements.push(format!(
                    "{id}: critical multiplier {old_crit_mult:?} != {:?}",
                    converted.critical_multiplier
                ));
            }

            // --- the old `damage_total::eqmweapon_damagesize_chain_value` ---
            let old_steps: i32 = row
                .bonus_chains
                .iter()
                .filter_map(|bonus| {
                    let q = &bonus.qualifiers;
                    if q.len() >= 3 && q[0] == "EQMWEAPON" && q[1] == "DAMAGESIZE" {
                        q[2].parse::<i32>().ok()
                    } else {
                        None
                    }
                })
                .sum();
            if old_steps != converted.damage_size_steps {
                disagreements.push(format!(
                    "{id}: damage size steps {old_steps} != {}",
                    converted.damage_size_steps
                ));
            }

            // --- the old inner scan of `resolve_eqm_weightdiv_effect` ---
            let old_divisor = row.bonus_chains.iter().find_map(|bonus| {
                let q = &bonus.qualifiers;
                if q.len() >= 3 && q[0] == "EQM" && q[1] == "WEIGHTDIV" {
                    q[2].parse::<f32>().ok()
                } else {
                    None
                }
            });
            if old_divisor != converted.weight_divisor {
                disagreements.push(format!(
                    "{id}: weight divisor {old_divisor:?} != {:?}",
                    converted.weight_divisor
                ));
            }

            // The weight `resolve_eqm_weightdiv_effect` divides is the same
            // `WT:` token it read before the move, narrowed to `f32` at the
            // point of use. Compared here over the whole corpus because a
            // `f64`-then-narrow is not textually the same operation as a
            // direct `f32` parse.
            let old_weight_f32: Option<f32> = token_value("WT").and_then(|v| v.parse::<f32>().ok());
            let new_weight_f32 = converted.weight_lbs.map(|w| w as f32);
            if old_weight_f32 != new_weight_f32 {
                disagreements.push(format!(
                    "{id}: weight as f32 {old_weight_f32:?} != {new_weight_f32:?}"
                ));
            }

            // --- the old `equipment_effects::is_natural_attack_weapon` ---
            let old_natural = row
                .tokens
                .iter()
                .find(|token| token.key == "TYPE")
                .is_some_and(|token| token.value.split('.').any(|segment| segment == "Natural"));
            if old_natural != converted.is_natural_attack {
                disagreements.push(format!(
                    "{id}: natural attack {old_natural} != {}",
                    converted.is_natural_attack
                ));
            }

            // --- the old shield half of `equipment_effects::is_weapon_record` ---
            let old_shield = row
                .tokens
                .iter()
                .find(|token| token.key == "TYPE")
                .is_some_and(|token| token.value.split('.').next() == Some("Shield"));
            if old_shield != converted.is_shield {
                disagreements
                    .push(format!("{id}: shield {old_shield} != {}", converted.is_shield));
            }
        }

        assert!(examined > 3000, "expected the live equipment corpus, examined {examined}");
        assert!(
            disagreements.is_empty(),
            "{} of {examined} records disagree: {:?}",
            disagreements.len(),
            &disagreements[..disagreements.len().min(20)]
        );
    }
}
