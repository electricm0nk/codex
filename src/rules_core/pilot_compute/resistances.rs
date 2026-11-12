//! The resistances block's data (SD-40 E2.21, `decisions.md §24.3.6`): energy resistances, damage
//! reduction, spell resistance and immunities, from what the character holds. Every number is one
//! the records state or the engine already evaluated; nothing is simulated (`§17`).
//!
//! # Sources
//!
//! - **Energy:** a held racial trait's `BONUS:VAR <Energy>ResistanceBonus` (or
//!   `Resistance<Energy>`) chain, read off the trait the race resolver applied (so an alternate
//!   that replaced it holds none); plus whatever the caller adds ([`Resistances::add_energy`]:
//!   the engine's existing energy-resistance rows). One energy from several sources keeps the
//!   highest amount (PF1 resistances of one type do not stack) and names every source.
//! - **Damage reduction / spell resistance:** a rendered sheet line's `DR: <n>/<bypass>` and
//!   `SR: <n>` entries (the converted `DR` / `SR` tokens, slots already evaluated); an entry with
//!   no leading number keeps the record's words and no number. Plus what the caller adds (the
//!   snapshot's class DR, worn items' SR). One bypass from several sources keeps the highest.
//! - **Immunities:** a rendered line whose label names an immunity, by name.
//!
//! Not gathered (named remainder): active spells, conditions and templates; a held rule with
//! `print: false` (it renders no line, `decisions.md` E2.28); energy chains on kinds other than
//! racial traits (the engine's own rows cover the ARG feats).

use crate::rules_core::declared_bonuses::VarContribution;

/// The five energy types, as the sheet's boxes name them.
pub const ENERGIES: [&str; 5] = ["Acid", "Cold", "Electricity", "Fire", "Sonic"];

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyResistance {
    /// One of [`ENERGIES`].
    pub energy: String,
    pub amount: i16,
    /// Every source granting it, `"; "`-joined, in the order found.
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageReduction {
    /// The number, when the record states or the engine evaluated one.
    pub amount: Option<i16>,
    /// What overcomes it (`"—"` for nothing, `"silver"`); empty when `words` carry it.
    pub bypass: String,
    /// The record's words when there is no number.
    pub words: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellResistance {
    pub value: Option<i16>,
    pub words: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Immunity {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resistances {
    pub energy: Vec<EnergyResistance>,
    pub damage_reduction: Vec<DamageReduction>,
    pub spell_resistance: Vec<SpellResistance>,
    pub immunities: Vec<Immunity>,
}

/// `source` joined onto `into` once.
fn name_source(into: &mut String, source: &str) {
    if !into.split("; ").any(|s| s == source) {
        into.push_str("; ");
        into.push_str(source);
    }
}

impl Resistances {
    /// Resistance `amount` to `energy` (any case) from `source`: the highest per energy, every
    /// source named. An energy outside [`ENERGIES`] is ignored.
    pub fn add_energy(&mut self, energy: &str, amount: i16, source: &str) {
        let Some(energy) = ENERGIES.iter().find(|e| e.eq_ignore_ascii_case(energy)) else { return };
        match self.energy.iter_mut().find(|e| e.energy == *energy) {
            Some(held) => {
                held.amount = held.amount.max(amount);
                name_source(&mut held.source, source);
            }
            None => self.energy.push(EnergyResistance { energy: (*energy).to_owned(), amount, source: source.to_owned() }),
        }
    }

    /// Damage reduction `amount`/`bypass` from `source`: the highest per bypass.
    pub fn add_damage_reduction(&mut self, amount: i16, bypass: &str, source: &str) {
        let bypass = normalize_bypass(bypass);
        match self.damage_reduction.iter_mut().find(|dr| dr.amount.is_some() && dr.bypass == bypass) {
            Some(held) => {
                held.amount = held.amount.max(Some(amount));
                name_source(&mut held.source, source);
            }
            None => self.damage_reduction.push(DamageReduction { amount: Some(amount), bypass, words: None, source: source.to_owned() }),
        }
    }

    /// Spell resistance `value` from `source` (several sources print one line each).
    pub fn add_spell_resistance(&mut self, value: i16, source: &str) {
        self.spell_resistance.push(SpellResistance { value: Some(value), words: None, source: source.to_owned() });
    }
}

/// `"-"`, `"--"` and `"—"` (DR that nothing overcomes) all print `"—"`.
fn normalize_bypass(bypass: &str) -> String {
    let bypass = bypass.trim();
    if bypass.chars().all(|c| c == '-' || c == '—') { "—".to_owned() } else { bypass.to_owned() }
}

/// The energy a race-trait variable names (`FireResistanceBonus`, `ResistanceFire`).
fn energy_of_var(name: &str) -> Option<&'static str> {
    ENERGIES.iter().copied().find(|e| name == format!("{e}ResistanceBonus") || name == format!("Resistance{e}"))
}

/// The resistances over a character's rendered lines (`(rule id, label, prose)`) and the racial
/// traits it holds (`(trait key, its variable contributions)`); see the module doc.
pub fn resistances<'a>(
    lines: impl IntoIterator<Item = (&'a str, &'a str, &'a str)>,
    race_traits: impl IntoIterator<Item = (&'a str, &'a [VarContribution])>,
) -> Resistances {
    let mut out = Resistances::default();
    for (key, vars) in race_traits {
        for var in vars {
            if let (Some(energy), Some(amount)) = (energy_of_var(&var.name), var.amount.and_then(|a| i16::try_from(a).ok())) {
                out.add_energy(energy, amount, key);
            }
        }
    }
    for (id, label, prose) in lines {
        for entry in prose.lines() {
            if let Some(dr) = entry.strip_prefix("DR: ") {
                let parsed = dr.split_once('/').and_then(|(n, bypass)| Some((n.trim().parse::<i16>().ok()?, bypass)));
                match parsed {
                    Some((amount, bypass)) => out.add_damage_reduction(amount, bypass, label),
                    None => out.damage_reduction.push(DamageReduction {
                        amount: None,
                        bypass: String::new(),
                        words: Some(dr.to_owned()),
                        source: label.to_owned(),
                    }),
                }
            } else if let Some(sr) = entry.strip_prefix("SR: ") {
                match sr.trim().parse::<i16>() {
                    Ok(value) => out.add_spell_resistance(value, label),
                    Err(_) => out.spell_resistance.push(SpellResistance { value: None, words: Some(sr.to_owned()), source: label.to_owned() }),
                }
            }
        }
        let lower = label.to_lowercase();
        // A record named "... (Replaces X Immunities)" names what it takes away.
        if lower.contains("immunit") && !lower.contains("replaces") && !out.immunities.iter().any(|i| i.name == label) {
            out.immunities.push(Immunity { name: label.to_owned(), source: id.to_owned() });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn var(name: &str, amount: i64) -> VarContribution {
        VarContribution { name: name.to_owned(), amount: Some(amount) }
    }

    /// Ifrit (Bestiary 2) fire 5; a second fire source keeps the higher and names both; DR and SR
    /// lines parse to numbers or keep their words; an immunity is gathered by name.
    #[test]
    fn resistances_gather_energy_dr_sr_and_immunities() {
        let ifrit = [var("FireResistanceBonus", 5)];
        let suli = [var("ResistanceFire", 10), var("SomethingElse", 3)];
        let lines = [
            ("a:class_feature:dr", "Damage Reduction", "You gain damage reduction.\nDR: 3/-"),
            ("a:class_feature:dr2", "Ectoplasmic Body", "DR: 5/slashing"),
            ("a:class_feature:dr3", "Greater Rage", "DR: 2/—"),
            ("a:class_feature:dr4", "Odd", "DR: (choice not yet made)"),
            ("a:race_trait:sr", "Spell Resistance", "SR: 11"),
            ("a:race_trait:sr2", "Magic Resistant", "SR: (choice not yet made)"),
            ("a:race_trait:imm", "Elven Immunities", "Elves are immune to magic sleep effects."),
            ("a:race_trait:rep", "Dwarf Traits (Replaces Duergar Immunities)", ""),
        ];
        let out = resistances(lines, [("Ifrit ~ Energy Resistance", &ifrit[..]), ("Suli ~ Fire", &suli[..])]);
        assert_eq!(
            out.energy,
            [EnergyResistance { energy: "Fire".into(), amount: 10, source: "Ifrit ~ Energy Resistance; Suli ~ Fire".into() }]
        );
        let drs: Vec<(Option<i16>, &str, Option<&str>)> =
            out.damage_reduction.iter().map(|d| (d.amount, d.bypass.as_str(), d.words.as_deref())).collect();
        assert_eq!(drs, [(Some(3), "—", None), (Some(5), "slashing", None), (None, "", Some("(choice not yet made)"))]);
        assert_eq!(out.damage_reduction[0].source, "Damage Reduction; Greater Rage");
        let srs: Vec<(Option<i16>, Option<&str>)> = out.spell_resistance.iter().map(|s| (s.value, s.words.as_deref())).collect();
        assert_eq!(srs, [(Some(11), None), (None, Some("(choice not yet made)"))]);
        assert_eq!(out.immunities, [Immunity { name: "Elven Immunities".into(), source: "a:race_trait:imm".into() }]);
    }

    #[test]
    fn resistances_of_nothing_are_four_empty_lists() {
        assert_eq!(resistances([], []), Resistances::default());
        let mut out = Resistances::default();
        out.add_energy("plasma", 5, "x");
        out.add_energy("fire", 5, "ARG");
        assert_eq!(out.energy[0].energy, "Fire");
        assert_eq!(out.energy.len(), 1);
    }
}
