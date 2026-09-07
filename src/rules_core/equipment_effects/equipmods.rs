//! Epic 5, fourth and final equipment category (SD-20 §1.5 work-unit
//! order): CRB `equipmods` per-item effect resolution. Landing this
//! closes Epic 5 — all four CRB equipment categories (`arms_armor`,
//! `general`, `magic_items`, `equipmods`) are done.
//!
//! Unlike `arms_armor` (AC/max-dex/spell-failure), `general`
//! (`BONUS:SKILL|...`), and `magic_items` (`BONUS:STAT|...`), the CRB
//! `equipmods` block (`core_rulebook/cr_equipmods.lst`) is a mixed bag of
//! materials, masterwork qualities, and weapon special abilities. Its
//! single most common per-item mechanical bonus that is genuinely
//! player-facing (not an internal cost/formula token like `BONUS:VAR` or
//! `BONUS:ITEMCOST`, which dominate this file's `BONUS:` token count) is
//! a weapon to-hit/damage enhancement bonus carried by
//! `BONUS:WEAPON|<TOHIT|DAMAGE|DAMAGE,TOHIT|TOHIT,DAMAGE>|<n>|
//! TYPE=Enhancement` — confirmed directly against the real corpus on the
//! canonical "+1 (Enhancement to Weapon)" through "+5 (Enhancement to
//! Weapon)" records (`KEY:Special Ability ~ +1 ~ Weapon` ... `~ +5 ~
//! Weapon`, each carrying `BONUS:WEAPON|DAMAGE,TOHIT|<n>|
//! TYPE=Enhancement`), on the `Masterwork`/`Adamantine`/`Mithral`
//! weapon-material records (each carrying `BONUS:WEAPON|TOHIT|1|
//! TYPE=Enhancement`), and on `Maul of the Titans`/`Mattock of the
//! Titans` (`BONUS:WEAPON|TOHIT,DAMAGE|3|TYPE=Enhancement` — the reverse
//! pipe order of the canonical records, proving the affected-roll set
//! must accept both orders, not just `DAMAGE,TOHIT`).
//!
//! **Re-landed correctly (`SD31-W17-INTEGRATE-001` OPEN-ISSUES row 309,
//! SD-31 wave 18):** the Amulet of Mighty Fists family's own
//! `BONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|<n>|TYPE=Enhancement`
//! chain (`KEY:Special Ability ~ +1 ~ Amulet of Mighty Fists` through
//! `~ +5 ~`) was widened into the same match as a bare `WEAPON` chain in
//! wave 17 and reverted after review: `WEAPONPROF=TYPE.Natural` scopes the
//! bonus to NATURAL attacks only, but `WeaponEnhancementBonus` carried no
//! field able to represent that scope, and the consumer
//! (`damage_total::resolve_weapon_enhancement_modifier`) summed every
//! equipped item's `weapon_enhancement_bonus` into EVERY weapon a
//! character wields — so treating this chain the same as a bare `WEAPON`
//! chain gave an equipped Amulet of Mighty Fists +5 a wrongful +5
//! attack/+5 damage on an ordinary longsword, reachable in the shipped
//! desktop app (proven live: `apps/desktop/src-tauri/src/
//! character_hub.rs`'s `attach_equipment_modifier_at_root` gates only on
//! catalog recognition, target-equipped and funds, no legality check).
//! Wave 18 adds the piece both review findings named: this module now
//! sets `WeaponEnhancementBonus::natural_attack_only` (real, distinct
//! from a bare `WEAPON` chain, never guessed).
//!
//! **`SD31-W18-INTEGRATE-001` correction (integration-cycle adversarial
//! review, `OPEN-ISSUES.md` row 309 re-opened a second time):** the
//! wave-18 lane guarded only `damage_total::
//! resolve_weapon_enhancement_modifier` (the `weapon_enhancement_bonus`
//! top-level-selection consumer). It left `equipment_effects::
//! resolve_weapon_to_hit_bonus` — the function actually called for
//! `to_hit_bonus`/`attack_bonus_delta` via `selection.applied_modifiers`,
//! the SAME attachment shape `attach_equipment_modifier_at_root` uses —
//! unguarded, so an equipped Amulet of Mighty Fists still leaked its
//! bonus onto an ordinary longsword's attack roll even after that fix.
//! BOTH consumers now check `equipment_effects::is_natural_attack_weapon`
//! on the specific weapon being resolved before applying a
//! `natural_attack_only` bonus to it — an ordinary longsword receives
//! neither the Amulet's attack nor damage bonus; only a real
//! natural-attack weapon (e.g. CRB's `Unarmed Strike`) does.
//!
//! Deliberately requires the trailing `TYPE=Enhancement` qualifier — this
//! excludes the `BONUS:WEAPON|WIELDCATEGORY|...` chains (Wield Size
//! records, which shift a weapon's effective wield category, not its
//! attack/damage rolls) and the bare `BONUS:WEAPON|TOHIT|<n>` chain some
//! Wield-Size "No Penalty" records carry with no `TYPE=` qualifier at all
//! (a size-handling to-hit offset, not a magic enhancement bonus).
//! Folding either into the same field would misrepresent a
//! wielding-mechanic delta as an enhancement bonus. The affected-roll
//! requirement (`TOHIT`/`DAMAGE`/`DAMAGE,TOHIT`/`TOHIT,DAMAGE`) is a
//! second, independent guard — every real corpus chain reaching this
//! function today already carries `TYPE=Enhancement` AND one of these
//! four roll shapes together, so the two checks are redundant on the
//! CURRENT corpus, but each is real and independently testable (see
//! `weapon_chain_with_unrecognized_affected_roll_has_no_weapon_
//! enhancement_bonus` below, which defeats the `TYPE=Enhancement` guard
//! on purpose to exercise the roll check on its own). Many other
//! `equipmods` records (charge trackers, spell-effect triggers,
//! artisan's tools with only a skill bonus, plain materials like Cloth,
//! ...) carry no matching chain at all, so `None` for those is an honest
//! absence, not a fabricated zero. No field here is hand-rolled; every
//! value traces back to a real, verbatim corpus token, read the same way
//! `arms_armor.rs`, `general.rs`, and `magic_items.rs` read their own
//! tokens straight off the resolved record.

use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;

/// A weapon to-hit/damage enhancement bonus granted by an
/// `equipmods`-category item's
/// `BONUS:WEAPON|<TOHIT|DAMAGE|DAMAGE,TOHIT|TOHIT,DAMAGE>|<n>|
/// TYPE=Enhancement` corpus token.
#[derive(Debug, Clone, PartialEq)]
pub struct WeaponEnhancementBonus {
    /// The record's TOHIT-affecting magnitude, summed across every
    /// qualifying chain on this record that affects `TOHIT` (`None` if no
    /// qualifying chain affects it). Before SD-33 remediation wave 5 this
    /// was a single `bonus: i16` shared by whichever roll(s) a lone
    /// `affects: String` named — insufficient for a record carrying TWO
    /// separate qualifying chains with DIFFERENT per-roll magnitudes
    /// (`ultimate_equipment:equipment:heavy_hammer`'s real
    /// `BONUS:WEAPONPROF=Warhammer|TOHIT|-2` +
    /// `BONUS:WEAPONPROF=Warhammer|DAMAGE|4`, confirmed against the pinned
    /// oracle: `WEAPON.n.MAGICHIT=-2`, `WEAPON.n.MAGICDAMAGE=+4` — two
    /// genuinely different numbers, not one scalar). A corpus-wide scan
    /// (`data/corpus/**/equipment*/*.json`, 579 records with any bonus
    /// chain) confirms `heavy_hammer` is the ONLY record with 2+
    /// qualifying chains, so this split is a pure widening: every other
    /// record's resolved value is byte-identical before and after.
    pub tohit_bonus: Option<i16>,
    /// The record's DAMAGE-affecting magnitude, summed the same way. See
    /// `tohit_bonus`'s doc comment.
    pub damage_bonus: Option<i16>,
    /// `true` when the source chain's qualifier[0] subject is
    /// `WEAPONPROF=TYPE.Natural` (the Amulet of Mighty Fists family) —
    /// real, verbatim from the token, not inferred. `damage_total::
    /// resolve_weapon_enhancement_modifier` (`SD31-W17-INTEGRATE-001`
    /// OPEN-ISSUES row 309) must only apply a `true` bonus to a weapon
    /// `equipment_effects::is_natural_attack_weapon` confirms is a real
    /// natural attack. `false` for a bare `WEAPON` chain, which applies to
    /// any weapon per PF1's ordinary enhancement rule.
    pub natural_attack_only: bool,
    /// SD-33 Epic 5 combat/weapon lane: the specific weapon-proficiency
    /// name this bonus is scoped to, when the source chain's subject is a
    /// bare `WEAPONPROF=<name>` other than `TYPE.Natural` — e.g.
    /// `Some("Longsword")` for `BONUS:WEAPONPROF=Longsword|TOHIT,DAMAGE|
    /// <n>` (`ultimate_equipment`'s "Cursed Sword" family), `Some("Hoof")`
    /// for the Horseshoes of a Zealous Warhorse family. Real, verbatim
    /// from the token — never inferred. `None` for a bare `WEAPON` chain
    /// (applies broadly) and for `WEAPONPROF=TYPE.Natural`
    /// (`natural_attack_only` already carries that distinct scope, kept
    /// as its own field for every existing consumer's back-compat).
    /// Confirmed against real PCGen source
    /// (`pcgen.io.exporttoken.WeaponToken.getMagicHitToken`/
    /// `getMagicDamageToken`): PCGen sums a `WEAPONPROF=<name>` bonus onto
    /// a specific equipped weapon only when that weapon's own resolved
    /// proficiency name matches `<name>` exactly.
    pub weapon_prof_scope: Option<String>,
}

/// Resolve one `equipmods` corpus record's weapon-enhancement-bonus
/// contribution.
///
/// Reads EVERY `BONUS:<WEAPON|WEAPONPROF=TYPE.Natural|WEAPONPROF=<name>>|
/// <TOHIT|DAMAGE|DAMAGE,TOHIT|TOHIT,DAMAGE>|<n>|TYPE=Enhancement` chain on
/// the record (SD-33 remediation wave 5: was the FIRST such chain only,
/// via `find_map` — silently dropped a second, separately-scoped chain on
/// the same record; see `WeaponEnhancementBonus::tohit_bonus`'s doc
/// comment) and sums each roll's magnitude across every qualifying chain.
/// A record with no such chain (the majority of `equipmods` records)
/// yields `None`: that means this record's raw tokens do not carry the
/// field, not that its value is zero. `BONUS:WEAPON|WIELDCATEGORY|...`
/// chains and `TYPE=Enhancement`-less `BONUS:WEAPON|...` chains are
/// deliberately not matched (see module doc comment).
/// SD-33 remediation wave 6 (`AT-33-E5-last39-skill-combat`): a record's
/// own `BONUS:VAR|<name>|<n>` chain, when `name` matches exactly. Used
/// only as a narrow substitution for a sibling `WEAPON|...` chain's own
/// magnitude segment when that segment is not a literal integer -- never
/// consulted for any other purpose, and never looks outside this ONE
/// record (no cross-record variable resolution, no character context).
/// `n` is required to be a literal integer itself; a non-literal `VAR`
/// value (none observed in the pinned corpus) yields `None` rather than a
/// fabricated number, same discipline as every other resolver in this
/// module.
fn resolve_var_reference(record: &EquipmentRecord, name: &str) -> Option<i16> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        if qualifiers.len() >= 3 && qualifiers[0] == "VAR" && qualifiers[1] == name {
            qualifiers[2].parse::<i16>().ok()
        } else {
            None
        }
    })
}

/// A `WEAPON`-chain magnitude segment: a literal signed integer, or (SD-33
/// remediation wave 6) the name of a variable this same record defines via
/// its own `BONUS:VAR|<name>|<n>` chain (see [`resolve_var_reference`]).
/// Real corpus example: `ultimate_psionics`'s dissonance-modifier family
/// carries both `BONUS:VAR|DissonanceEnhancementBonusMain|1` and
/// `BONUS:WEAPON|DAMAGE,TOHIT|DissonanceEnhancementBonusMain|
/// TYPE=ENHANCEMENT` on the SAME record -- the second chain's magnitude
/// segment names the first chain's own variable, whose value (`1`) is a
/// real, verbatim corpus literal, not computed by any formula evaluator.
fn resolve_bonus_magnitude(record: &EquipmentRecord, raw: &str) -> Option<i16> {
    raw.parse::<i16>().ok().or_else(|| resolve_var_reference(record, raw))
}

pub fn compute_equipmods_effect(record: &EquipmentRecord) -> Option<WeaponEnhancementBonus> {
    let mut tohit_bonus: Option<i16> = None;
    let mut damage_bonus: Option<i16> = None;
    let mut natural_attack_only = false;
    let mut weapon_prof_scope: Option<String> = None;
    let mut matched = false;

    let mut apply = |affects: &str, bonus_value: i16| {
        if affects.contains("TOHIT") {
            tohit_bonus = Some(tohit_bonus.unwrap_or(0) + bonus_value);
        }
        if affects.contains("DAMAGE") {
            damage_bonus = Some(damage_bonus.unwrap_or(0) + bonus_value);
        }
    };

    for bonus in &record.bonus_chains {
        let qualifiers = &bonus.qualifiers;
        let subject = qualifiers.first().map(String::as_str);
        let this_natural_attack_only = subject == Some("WEAPONPROF=TYPE.Natural");
        let is_roll_shape = qualifiers.len() >= 2
            && matches!(qualifiers[1].as_str(), "TOHIT" | "DAMAGE" | "DAMAGE,TOHIT" | "TOHIT,DAMAGE");

        if (subject == Some("WEAPON") || this_natural_attack_only) && is_roll_shape {
            // Unchanged from before this cycle: a bare `WEAPON` chain or
            // `WEAPONPROF=TYPE.Natural` chain still requires the trailing
            // `TYPE=Enhancement` qualifier (see module doc comment for
            // why: it excludes `WIELDCATEGORY` and untyped Wield-Size
            // to-hit-offset chains, which are real but are not a magic
            // enhancement bonus).
            //
            // SD-33 remediation wave 6 (`AT-33-E5-last39-skill-combat`):
            // matched case-insensitively -- the `ultimate_psionics`
            // dissonance-modifier family (`up_equipmods.lst:141-142`)
            // carries the SAME shape with `TYPE=ENHANCEMENT` (uppercase),
            // which the prior exact-string match never matched. Named,
            // not fixed, by `AT-33-E5-last75_cycle_receipt.md` Finding 4
            // and `AT-33-E5-last67-skill-combat_cycle_receipt.md`. No real
            // corpus record's `TYPE=` qualifier for this shape is
            // observed in any casing other than these two, and
            // case-insensitive comparison cannot turn an unrelated
            // qualifier into a false match (it only widens this exact
            // string, never a substring).
            if qualifiers.len() >= 4 && qualifiers[3].eq_ignore_ascii_case("TYPE=Enhancement") {
                // SD-33 remediation wave 6: the magnitude segment is
                // either a literal signed integer (the common case) or
                // the NAME of a variable this SAME record itself defines
                // via a sibling `BONUS:VAR|<name>|<n>` chain (the
                // dissonance-modifier family's `WEAPON|DAMAGE,TOHIT|
                // DissonanceEnhancementBonus{Alt,Main}|TYPE=ENHANCEMENT`
                // shape) -- resolved via `resolve_var_reference`, never a
                // blind/general formula evaluator, and never a value from
                // any OTHER record.
                if let Some(bonus_value) = resolve_bonus_magnitude(record, &qualifiers[2]) {
                    matched = true;
                    natural_attack_only = this_natural_attack_only;
                    apply(&qualifiers[1], bonus_value);
                }
            }
            continue;
        }

        // SD-33 Epic 5 combat/weapon lane: a bare `WEAPONPROF=<name>|
        // <TOHIT|DAMAGE|...>|<n>` chain scoped to one SPECIFIC named
        // proficiency (e.g. Longsword, Hoof, Bite) -- distinct from both
        // the broadly-applying bare `WEAPON` chain and the natural-
        // attack-only `TYPE.Natural` chain. Every real corpus record of
        // this shape (`ultimate_equipment`'s "Cursed <Weapon>" and
        // "Horseshoes of a Zealous Warhorse" families) carries this
        // WITHOUT a trailing `TYPE=Enhancement` qualifier at all -- real
        // PCGen source (`pcgen.io.exporttoken.WeaponToken.
        // getMagicHitToken`/`getMagicDamageToken`) sums a
        // `WEAPONPROF=<name>` bonus unconditionally, with no `TYPE=`
        // filter either, so no such gate applies here.
        if let Some(name) = subject.and_then(|s| s.strip_prefix("WEAPONPROF=")) {
            // Excludes every `TYPE.`-prefixed subject, not just the
            // literal `TYPE.Natural` string: a `WEAPONPROF=TYPE.<x>`
            // chain names a whole weapon-TYPE category (a hypothetical
            // shape this module's own pre-existing negative-control test,
            // `a_different_weaponprof_subject_has_no_weapon_enhancement_
            // bonus`, deliberately keeps unrecognized), never a single
            // literal proficiency name PCGen's own `getProfName(eq)`
            // would compare a specific weapon's proficiency against.
            if !name.starts_with("TYPE.") && is_roll_shape && qualifiers.len() >= 3
                && let Ok(bonus_value) = qualifiers[2].parse::<i16>() {
                    matched = true;
                    weapon_prof_scope = Some(name.to_string());
                    apply(&qualifiers[1], bonus_value);
                }
        }
    }

    matched.then_some(WeaponEnhancementBonus {
        tohit_bonus,
        damage_bonus,
        natural_attack_only,
        weapon_prof_scope,
    })
}

/// Folds each `EQMOD:`-referenced modifier record's own weapon-enhancement
/// contribution (via [`compute_equipmods_effect`] applied recursively to
/// each modifier) into `effect`, mirroring
/// `arms_armor::apply_eqmod_armor_class_bonus`'s pattern for the AC
/// dimension (SD-33 remediation wave 4) with one deliberate difference:
/// **per-dimension MAX, not sum.**
///
/// A magic weapon's real enhancement bonus frequently lives on a SEPARATE
/// `equipment_modifier` record the base weapon's `EQMOD:` token names,
/// never on the base record's own chain -- confirmed by the real corpus
/// example this fix closes, `advanced_race_guide:equipment:rending_claw_
/// blades`: the base record's own chain is `BONUS:WEAPON|TOHIT|1|
/// TYPE=Enhancement`; its `.MOD`-attached `EQMOD:`-referenced `Special
/// Ability ~ +1 ~ Weapon` modifier carries the separate
/// `BONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement` chain. Both chains carry
/// the identical `TYPE=Enhancement` qualifier -- every chain
/// [`compute_equipmods_effect`]'s `WEAPON`/`WEAPONPROF=TYPE.Natural`
/// match arms requires it (see that function's own doc comment) -- and
/// Pathfinder's core stacking rule is that bonuses of the SAME type never
/// stack; only the highest applies. Live-oracle-confirmed on this exact
/// record: `MAGICHIT=+1` (not `+2` -- base `TOHIT|1` and the modifier's
/// own `TOHIT|1` are the SAME Enhancement type, `max(1, 1) = 1`, matching)
/// and `MAGICDAMAGE=+1` (base contributes no `DAMAGE` chain at all; the
/// modifier's own `DAMAGE|1` is the only contributor, so it is simply the
/// result -- absence is `None`, never `0`, so it never wins a `max`
/// against a real contributed value it should not suppress). This is
/// unlike the AC dimension's `apply_eqmod_armor_class_bonus`, which
/// correctly SUMS: a base armor's own `TYPE=Armor` value and an
/// enhancement modifier's `TYPE=ArmorEnhancement` bonus are two DIFFERENT
/// bonus types by Pathfinder rule (armor's base AC value plus its
/// enhancement bonus, always additive -- that pattern's own real corpus
/// witness, `inner_sea_races:equipment:armor_of_grim_triumph`, is
/// `6 + 1 = 7`, proven against the oracle).
///
/// **Scope this proof does NOT cover** (`AGENTS.md` Non-Negotiable Rule
/// 7): the `WEAPONPROF=<name>` bare-chain shape `compute_equipmods_effect`
/// also matches carries NO `TYPE=` qualifier at all (a different,
/// untyped bonus family -- see that function's own doc comment), so
/// treating it identically to a same-typed `Enhancement` chain here (via
/// `max`) is a simplification, not a proven rule; no real corpus record
/// combines a `WEAPONPROF=<name>`-shaped base chain with an `EQMOD:`-
/// referenced modifier in the population this cycle examined, so the
/// question is unexercised, not answered.
///
/// `effect` starts `None` when the base record's own chain matched
/// nothing at all -- an eqmod-only contribution (no real corpus example
/// observed yet, but not excluded) still surfaces correctly, the same way
/// `apply_eqmod_armor_class_bonus` allows a base item with no armor chain
/// of its own to still gain one from a referenced modifier.
pub fn apply_eqmod_weapon_enhancement_bonus(
    effect: &mut Option<WeaponEnhancementBonus>,
    eqmod_records: &[&EquipmentRecord],
) {
    for modifier in eqmod_records {
        let Some(modifier_bonus) = compute_equipmods_effect(modifier) else {
            continue;
        };
        let target = effect.get_or_insert(WeaponEnhancementBonus {
            tohit_bonus: None,
            damage_bonus: None,
            natural_attack_only: false,
            weapon_prof_scope: None,
        });
        if let Some(tohit) = modifier_bonus.tohit_bonus {
            target.tohit_bonus = Some(target.tohit_bonus.map_or(tohit, |current| current.max(tohit)));
        }
        if let Some(damage) = modifier_bonus.damage_bonus {
            target.damage_bonus = Some(target.damage_bonus.map_or(damage, |current| current.max(damage)));
        }
    }
}

/// Resolve one `equipmods` corpus record's flat Spell Resistance
/// contribution.
///
/// Reads the record's own `SR:<n>` token, when present and a literal
/// integer -- the armor-slot "Spell Resistance" special ability family
/// (`KEY:Special Ability ~ Spell Resistance / 13 ~ Armor` through `/ 19 ~
/// Armor`, `core_rulebook/cr_equipmods.lst:343-346`). Decision 7 REFINED
/// (`SD31-D7-PROSE-004`) names this exact shape as the paradigm UNIVERSAL
/// case: it applies unconditionally whenever the wearer's Spell
/// Resistance is checked, so text alone ("grants spell resistance 13")
/// does not satisfy the done-bar -- it must be COMPUTED.
///
/// Deliberately does NOT match `BNS_SPL_RST` ("Bonus Spell Resistance",
/// `KEY:Special Ability ~ Bonus Spell Resistance`), whose own `SR:%CHOICE`
/// token carries a PCGen chooser placeholder rather than a literal
/// integer -- `str::parse` fails on `"%CHOICE"` and correctly yields
/// `None`, the same "no fabricated number" discipline every other
/// resolver in this module follows. That record is a genuine player
/// CHOICE (`CHOOSE:NUMBER|MIN=13|MAX=32`), not a flat grant, and stays out
/// of this function's scope until a chosen-value resolution mechanism
/// exists.
pub fn resolve_spell_resistance_bonus(record: &EquipmentRecord) -> Option<i16> {
    record.tokens.iter().find(|token| token.key == "SR").and_then(|token| token.value.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::lst_parser::equipment::parse_equipment_entries;

    /// Real verbatim tokens copied from `KEY:Special Ability ~ +1 ~
    /// Weapon` in `core_rulebook/cr_equipmods.lst`.
    #[test]
    fn plus_one_weapon_enhancement_yields_a_real_damage_tohit_bonus() {
        let text = "+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(1),
                damage_bonus: Some(1),
                natural_attack_only: false,
                weapon_prof_scope: None,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Material ~ Adamantine ~
    /// Weapon` — a different affected-roll shape (`TOHIT` alone), proving
    /// the affected-roll set is read from the token, not hardcoded.
    #[test]
    fn adamantine_weapon_yields_a_real_tohit_only_bonus() {
        let text = "Adamantine\tKEY:Material ~ Adamantine ~ Weapon\tTYPE:BaseMaterial.MasterworkQuality.Weapon\tCOST:3000\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(1),
                damage_bonus: None,
                natural_attack_only: false,
                weapon_prof_scope: None,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Special Ability ~ +3 ~
    /// Weapon` on `Maul of the Titans`/`Mattock of the Titans`
    /// (`core_rulebook/cr_equip_arms_armor.lst`) — the SAME mechanic as
    /// the canonical `+1..+5` records above, but with the affected-roll
    /// segment in the opposite pipe order (`TOHIT,DAMAGE`, not
    /// `DAMAGE,TOHIT`), proving both orders are read, not just one.
    #[test]
    fn reversed_roll_order_weapon_enhancement_yields_a_real_bonus() {
        let text = "+3 (Enhancement to Weapon)\tKEY:Special Ability ~ +3 ~ Weapon Reversed\tTYPE:Weapon\tPLUS:3\tBONUS:WEAPON|TOHIT,DAMAGE|3|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(3),
                damage_bonus: Some(3),
                natural_attack_only: false,
                weapon_prof_scope: None,
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Special Quality ~ Wield Size
    /// / 1 Step Greater` — carries a `BONUS:WEAPON|WIELDCATEGORY|...`
    /// chain, which is deliberately not a weapon-enhancement bonus (see
    /// module doc comment).
    #[test]
    fn wield_size_shift_has_no_weapon_enhancement_bonus() {
        let text = "Wield One Step Greater\tKEY:Special Quality ~ Wield Size / 1 Step Greater\tTYPE:Weapon.Melee\tBONUS:WEAPON|WIELDCATEGORY|-1\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(effect, None);
    }

    /// Real verbatim tokens copied from `KEY:Special Quality ~ Wield Size
    /// / 1 Step Greater / No Penalty` — carries both a `WIELDCATEGORY`
    /// chain and a bare `BONUS:WEAPON|TOHIT|2` chain with no `TYPE=`
    /// qualifier at all (a size-handling to-hit offset, not a magic
    /// enhancement bonus); neither chain matches.
    #[test]
    fn wield_size_no_penalty_has_no_weapon_enhancement_bonus() {
        let text = "Wield One Step Greater No Penalty\tKEY:Special Quality ~ Wield Size / 1 Step Greater / No Penalty\tTYPE:Weapon.Melee\tBONUS:WEAPON|WIELDCATEGORY|-1\tBONUS:WEAPON|TOHIT|2\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(effect, None);
    }

    /// SD-33 Epic 5 combat/weapon lane: real verbatim tokens copied from
    /// `data/corpus/ultimate_equipment/equipment/cursed_sword_2.json`
    /// (`ue_equip_magic_items.lst`) — a bare `WEAPONPROF=Longsword|
    /// TOHIT,DAMAGE|<n>` chain, no trailing `TYPE=Enhancement` qualifier
    /// at all (confirmed against the real record: arity 3, not 4). Before
    /// this cycle `compute_equipmods_effect` only recognized `WEAPON` and
    /// `WEAPONPROF=TYPE.Natural` subjects, so this real, comparable,
    /// negative (`-2`, a cursed item) magnitude silently resolved to
    /// `None`.
    #[test]
    fn named_weaponprof_scope_yields_a_real_bonus_with_no_type_enhancement_gate() {
        let text = "Cursed Sword\tKEY:Cursed Sword\tTYPE:Weapon.Melee.Martial\tCOST:1\tWT:4\tCRITMULT:x2\tDAMAGE:1d8\tBONUS:WEAPONPROF=Longsword|TOHIT,DAMAGE|-2\n";
        let result = parse_equipment_entries("ue_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(-2),
                damage_bonus: Some(-2),
                natural_attack_only: false,
                weapon_prof_scope: Some("Longsword".to_string()),
            })
        );
    }

    /// The same named-`WEAPONPROF=` shape with a single affected roll
    /// (`TOHIT` alone, not the combined `TOHIT,DAMAGE` pair) — real
    /// verbatim tokens copied from
    /// `data/corpus/ultimate_equipment/equipment/belt_of_teeth.json`.
    #[test]
    fn named_weaponprof_scope_single_roll_yields_a_real_bonus() {
        let text = "Belt of Teeth\tKEY:Belt of Teeth\tTYPE:Magic.Belt\tCOST:1\tWT:1\tBONUS:WEAPONPROF=Bite|TOHIT|4\n";
        let result = parse_equipment_entries("ue_equip_magic_items.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(4),
                damage_bonus: None,
                natural_attack_only: false,
                weapon_prof_scope: Some("Bite".to_string()),
            })
        );
    }

    /// Real verbatim tokens copied from `KEY:Material ~ Cloth` — a plain
    /// material carries no `BONUS:` token at all.
    #[test]
    fn cloth_material_has_no_weapon_enhancement_bonus() {
        let text = "Cloth\tKEY:Material ~ Cloth\tTYPE:BaseMaterial.Mundane.Ammunition.Armor.Shield.Weapon.Instruments.Tools.Goods\tCOST:0\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(effect, None);
    }

    /// `SD31-W17-INTEGRATE-001` (OPEN-ISSUES row 309), re-landed wave 18:
    /// the Amulet of Mighty Fists family's own `WEAPONPROF=TYPE.Natural`
    /// chain now resolves to a real bonus, correctly tagged
    /// `natural_attack_only: true` — real verbatim tokens copied from
    /// `KEY:Special Ability ~ +1 ~ Amulet of Mighty Fists`. Wave 17
    /// recognized this chain without the tag and without a scope-aware
    /// consumer, which wrongly bonused every equipped weapon; the scope
    /// itself (this test) is now real, and
    /// `damage_total::resolve_weapon_enhancement_modifier`'s own tests
    /// prove the consumer honours it.
    #[test]
    fn amulet_of_mighty_fists_weaponprof_chain_yields_a_natural_attack_only_bonus() {
        let text = "+1 to Hit and Damage\tKEY:Special Ability ~ +1 ~ Amulet of Mighty Fists\tTYPE:Amulet of Mighty Fists\tPLUS:1\tBONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(1),
                damage_bonus: Some(1),
                natural_attack_only: true,
                weapon_prof_scope: None,
            })
        );
    }

    /// SD-33 remediation wave 5 (`AT-33-E5-002`/`003`, weapon-token-family
    /// lane): real verbatim tokens copied from
    /// `data/corpus/ultimate_equipment/equipment/heavy_hammer.json`'s
    /// `raw_bonus_chains` — a TOHIT-only `WEAPONPROF=Warhammer|TOHIT|-2`
    /// chain and a SEPARATE DAMAGE-only `WEAPONPROF=Warhammer|DAMAGE|4`
    /// chain on the SAME record (plus an unrelated `MOVEADD` chain, which
    /// this test also carries to prove it's correctly skipped). Before
    /// this cycle `compute_equipmods_effect` used `find_map` and stopped
    /// at the first qualifying chain, so only `-2`/`TOHIT` was ever seen —
    /// the real, player-facing `+4` damage bonus never reached
    /// `WeaponEnhancementBonus` at all. Confirmed against the pinned
    /// oracle (direct-java runner, Heavy Hammer worn as its own weapon,
    /// `PROFICIENCY WEAPON|Warhammer`): `WEAPON.n.MAGICHIT=-2`,
    /// `WEAPON.n.MAGICDAMAGE=+4` — both magnitudes real, and different.
    #[test]
    fn record_with_two_separately_scoped_chains_sums_both_rolls_independently() {
        let text = "Heavy Hammer\tKEY:Heavy Hammer\tTYPE:Magic.Cursed.Weapon\tPROFICIENCY:WEAPON|Warhammer\tCOST:0\tWT:20\tBONUS:MOVEADD|TYPE.All|-10\tBONUS:WEAPONPROF=Warhammer|TOHIT|-2\tBONUS:WEAPONPROF=Warhammer|DAMAGE|4\n";
        let result = parse_equipment_entries("ue_equip_magic_items.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(-2),
                damage_bonus: Some(4),
                natural_attack_only: false,
                weapon_prof_scope: Some("Warhammer".to_string()),
            })
        );
    }

    /// A `WEAPONPROF=` subject other than `TYPE.Natural` (e.g. a
    /// hypothetical class-specific proficiency scope) must not be treated
    /// as the natural-attack family — only the exact literal
    /// `WEAPONPROF=TYPE.Natural` string this family's real corpus records
    /// carry is recognized, never a substring or prefix match.
    #[test]
    fn a_different_weaponprof_subject_has_no_weapon_enhancement_bonus() {
        let text = "Proxy\tKEY:Special Quality ~ WeaponProf Proxy\tTYPE:Weapon\tBONUS:WEAPONPROF=TYPE.Bow|TOHIT,DAMAGE|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(effect, None);
    }

    /// Defeats the `TYPE=Enhancement` guard on purpose (an otherwise
    /// well-formed bare-`WEAPON` chain with 4 qualifiers) so the
    /// affected-roll check is exercised on its own rather than always
    /// being shadowed by arity or the qualifier[0]/TYPE=Enhancement
    /// checks — `SD31-W17-INTEGRATE-001` (review) found the PRIOR
    /// negative-control test for this shape could never fail because its
    /// fixture was excluded earlier by arity alone.
    #[test]
    fn weapon_chain_with_unrecognized_affected_roll_has_no_weapon_enhancement_bonus() {
        let text = "Weapon Focus Proxy\tKEY:Special Quality ~ Weapon Focus Proxy\tTYPE:Weapon\tBONUS:WEAPON|CRITMULT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(effect, None);
    }

    /// `SD31-W21-EQUIPMOD-001`: real verbatim tokens copied from
    /// `KEY:Special Ability ~ Spell Resistance / 13 ~ Armor`
    /// (`core_rulebook/cr_equipmods.lst:343`) -- a flat, unconditional
    /// `SR:13` token, the paradigm UNIVERSAL magnitude Decision 7 REFINED
    /// names (`SD31-D7-PROSE-004`: "a modifier to a value the character
    /// sheet computes, that applies UNCONDITIONALLY... Must be COMPUTED").
    #[test]
    fn spell_resistance_13_armor_yields_a_real_spell_resistance_bonus() {
        let text = "Spell Resistance 13\tFORMATCAT:FRONT\tNAMEOPT:NORMAL\tKEY:Special Ability ~ Spell Resistance / 13 ~ Armor\tTYPE:Armor.Bracer.ArmorLike\tPLUS:2\tVISIBLE:QUALIFY\tPREMULT:2,[PRETYPE:1,ArmorEnhancement],[PRETYPE:1,Armor,Bracer]\tSR:13\tSPROP:grants spell resistance 13\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        assert_eq!(resolve_spell_resistance_bonus(record), Some(13));
    }

    /// The same family's `/ 19 ~ Armor` tier, proving the value is read
    /// from the token rather than hardcoded to `13`. Real verbatim tokens
    /// copied from `cr_equipmods.lst:346`.
    #[test]
    fn spell_resistance_19_armor_yields_a_real_spell_resistance_bonus() {
        let text = "Spell Resistance 19\tFORMATCAT:FRONT\tNAMEOPT:NORMAL\tKEY:Special Ability ~ Spell Resistance / 19 ~ Armor\tTYPE:Armor.Bracer.ArmorLike\tPLUS:8\tVISIBLE:QUALIFY\tPREMULT:2,[PRETYPE:1,ArmorEnhancement],[PRETYPE:1,Armor,Bracer]\tSR:19\tSPROP:grants spell resistance 19\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        assert_eq!(resolve_spell_resistance_bonus(record), Some(19));
    }

    /// `KEY:Special Ability ~ Bonus Spell Resistance` (`BNS_SPL_RST`,
    /// `cr_equipmods.lst:617`) carries `SR:%CHOICE`, not a literal
    /// integer -- a real player CHOICE (`CHOOSE:NUMBER|MIN=13|MAX=32`),
    /// not a flat grant. `str::parse` fails on `"%CHOICE"` and this
    /// resolver must yield `None`, never a fabricated number, the same
    /// "no invented value" discipline every other resolver in this module
    /// follows for a chain it does not recognize.
    #[test]
    fn bonus_spell_resistance_choice_token_has_no_flat_spell_resistance_bonus() {
        let text = "BNS_SPL_RST\tVISIBLE:NO\tKEY:Special Ability ~ Bonus Spell Resistance\tTYPE:Weapon.Belt.Body\tCOST:10000*(%CHOICE-12)\tSR:%CHOICE\tSPROP:base spell resistance of %CHOICE\tCHOOSE:NUMBER|MIN=13|MAX=32|NOSIGN|TITLE=Spell Resistance\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        assert_eq!(resolve_spell_resistance_bonus(record), None);
    }

    /// A record with no `SR:` token anywhere (the canonical `+1`
    /// weapon-enhancement record already used above) yields `None`, not a
    /// fabricated zero.
    #[test]
    fn weapon_enhancement_record_has_no_spell_resistance_bonus() {
        let text = "+1 (Enhancement to Weapon)\tKEY:Special Ability ~ +1 ~ Weapon\tTYPE:Weapon\tPLUS:1\tCOST:0\tBONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        assert_eq!(resolve_spell_resistance_bonus(record), None);
    }

    /// SD-33 remediation wave 6 (`AT-33-E5-last39-skill-combat`): real
    /// verbatim tokens copied from `ultimate_psionics/up_equipmods.lst:141`
    /// (`KEY:Special Quality ~ Dissonance / Enhancement Bonus / Main`,
    /// `data/corpus/ultimate_psionics/equipment/equipmods/
    /// special_quality_dissonance_enhancement_bonus_main.json`). Two real
    /// gaps this record exposes together, both closed by this cycle:
    /// (1) the chain's affected-roll magnitude is the NAME of a variable
    /// (`DissonanceEnhancementBonusMain`), not a literal integer --
    /// `qualifiers[2].parse::<i16>()` fails closed on a bare name. The
    /// record's OWN sibling `BONUS:VAR|DissonanceEnhancementBonusMain|1`
    /// chain states that variable's own contribution as a real literal
    /// `1`, and the record's `DEFINE:DissonanceEnhancementBonusMain|0`
    /// token (base 0, no other source in an isolated single-item read)
    /// means the variable's total, knowable value is `0 + 1 = 1` -- not a
    /// guess, cross-referenced against the record's own chain.
    /// (2) the chain's `TYPE=` qualifier is `TYPE=ENHANCEMENT`
    /// (uppercase), which `qualifiers[3] == "TYPE=Enhancement"`'s exact
    /// string match never matches -- named but not fixed by
    /// `AT-33-E5-last75_cycle_receipt.md` Finding 4 and reconfirmed still
    /// open by `AT-33-E5-last67-skill-combat_cycle_receipt.md`. Before this
    /// cycle: `compute_equipmods_effect(record)` returns `None` (matches
    /// neither gate). After: real `tohit_bonus`/`damage_bonus` of `Some(1)`
    /// each, traced to the record's own two chains, no formula evaluator
    /// and no fabricated number.
    #[test]
    fn dissonance_enhancement_bonus_var_referenced_chain_resolves_via_its_own_sibling_var_chain() {
        let text = "Enhancement Bonus for Dissonance Main\tKEY:Special Quality ~ Dissonance / Enhancement Bonus / Main\tTYPE:Weapon\tVISIBLE:NO\tBONUS:VAR|DissonanceEnhancementBonusMain|1\tBONUS:WEAPON|DAMAGE,TOHIT|DissonanceEnhancementBonusMain|TYPE=ENHANCEMENT\tDEFINE:DissonanceEnhancementBonusMain|0\n";
        let result = parse_equipment_entries("up_equipmods.lst", text);
        assert!(result.entries.len() == 1, "expected exactly one parsed record");
        let record = &result.entries[0];

        let effect = compute_equipmods_effect(record);
        assert_eq!(
            effect,
            Some(WeaponEnhancementBonus {
                tohit_bonus: Some(1),
                damage_bonus: Some(1),
                natural_attack_only: false,
                weapon_prof_scope: None,
            })
        );
    }

    /// Negative control: a bare `WEAPON` chain whose magnitude names a
    /// variable with NO sibling `VAR` chain on the same record must still
    /// yield `None` -- the substitution is scoped to the record's own
    /// verified `VAR` chain, never a blind lookup that could fabricate a
    /// value for an unrelated or undefined variable name.
    #[test]
    fn weapon_chain_referencing_an_undefined_variable_name_has_no_weapon_enhancement_bonus() {
        let text = "Fabricated\tKEY:Fabricated Test Record\tTYPE:Weapon\tBONUS:WEAPON|TOHIT|SomeUndefinedVariable|TYPE=Enhancement\n";
        let result = parse_equipment_entries("cr_equipmods.lst", text);
        let record = &result.entries[0];

        assert_eq!(compute_equipmods_effect(record), None);
    }
}
