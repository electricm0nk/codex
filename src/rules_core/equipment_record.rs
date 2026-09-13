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
//! The armour/weapon/var/enhancement halves -- what `arms_armor`,
//! `damage_total`, `equipmods` and `general` read. Those are the remainder
//! this criterion still carries; each is a settled value too, and each lands
//! on this struct as its consumer moves. Nothing here stands in for work not
//! done: a field absent from this struct is a consumer that has not moved yet,
//! not a value we failed to settle.

use crate::rules_core::equipment_effects::intelligent_item::IntelligentItemContribution;
use crate::rules_core::equipment_effects::magic_items::AbilityScoreBonus;

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
}
