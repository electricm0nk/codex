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
//! The weapon half -- what `damage_total` reads (base damage dice, critical
//! range and multiplier, wield category) -- and the `EQMOD:`-referenced
//! weight-division and damage-size steps that ride on the same records.
//! Those are the remainder this criterion still carries; each is a settled
//! value too, and each lands on this struct as its consumer moves. Nothing
//! here stands in for work not done: a field absent from this struct is a
//! consumer that has not moved yet, not a value we failed to settle.
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
#[derive(Debug, Clone, Default, PartialEq)]
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
    use crate::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
    use crate::rules_core::source_content::{SourceContentKind, SourceContentPayload};

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
    /// straight off the parser row that is still paired with the converted
    /// record in the canonical envelope, and compared to the settled field.
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
        let roots: Vec<BookCorpusRoot<'_>> = books
            .iter()
            .map(|dir| BookCorpusRoot {
                book_id: dir.file_name().and_then(|n| n.to_str()).unwrap_or(""),
                dir: dir.as_path(),
            })
            .collect();
        let package = load_equipment_corpus(&roots);

        let mut examined = 0usize;
        let mut disagreements: Vec<String> = Vec::new();
        for record in package.records_by_kind(SourceContentKind::Equipment) {
            let SourceContentPayload::Equipment(row, converted) = record.payload else {
                continue;
            };
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
        let roots: Vec<BookCorpusRoot<'_>> = books
            .iter()
            .map(|dir| BookCorpusRoot {
                book_id: dir.file_name().and_then(|n| n.to_str()).unwrap_or(""),
                dir: dir.as_path(),
            })
            .collect();
        let package = load_equipment_corpus(&roots);

        let mut examined = 0usize;
        let mut disagreements: Vec<String> = Vec::new();
        for record in package.records_by_kind(SourceContentKind::Equipment) {
            let SourceContentPayload::Equipment(row, converted) = record.payload else {
                continue;
            };
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
}
