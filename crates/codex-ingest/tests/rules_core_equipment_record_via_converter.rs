// -- split from `tests` in src/rules_core/equipment_record.rs (pcgen-touching items only) --
mod tests {
    use codex_ingest::pcgen_import::corpus_equipment_json::every_live_corpus_equipment_pair;

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
    /// [`codex_ingest::pcgen_import::corpus_equipment_json::every_live_corpus_equipment_pair`]
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
                        if codex::rules_core::equipment_effects::intelligent_item::ItemAlignment::from_code(value)
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
        use codex_ingest::pcgen_import::equipment_bonus_reader;
        use codex::rules_core::equipment_effects::equipmods::WeaponEnhancementBonus;
        use codex::rules_core::equipment_effects::general::{SkillCheckBonus, VarBonus};

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
        use codex::rules_core::damage_total::{DiceExpression, WieldCategory};

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

    /// Every book directory under `data/corpus/`, so the sweep below reads the
    /// whole live corpus rather than one book it was tuned on.
    fn every_book_root() -> Vec<std::path::PathBuf> {
        let corpus = codex_ingest::repo_root().join("data/corpus");
        let mut books: Vec<std::path::PathBuf> = std::fs::read_dir(&corpus)
            .expect("data/corpus must be readable")
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .collect();
        books.sort();
        books
    }


}
