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

/// A weapon to-hit/damage enhancement bonus granted by an
/// `equipmods`-category item's
/// `BONUS:WEAPON|<TOHIT|DAMAGE|DAMAGE,TOHIT|TOHIT,DAMAGE>|<n>|
/// TYPE=Enhancement` corpus token.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

/// One `equipmods` corpus record's settled weapon to-hit / damage
/// enhancement.
///
/// The item's magnitude for each roll, summed across every enhancement it
/// states -- a record may state two, with DIFFERENT per-roll numbers
/// (`ultimate_equipment:equipment:heavy_hammer`, real oracle
/// `MAGICHIT=-2` / `MAGICDAMAGE=+4`; it is the only such record corpus-wide).
/// `natural_attack_only` and `weapon_prof_scope` carry the item's stated
/// scope: the Amulet of Mighty Fists family applies only to a real natural
/// attack, and a proficiency-scoped grant (the Cursed Sword and Horseshoes of
/// a Zealous Warhorse families) applies only to the weapon whose own
/// proficiency name matches -- confirmed against PCGen's own
/// `WeaponToken.getMagicHitToken`/`getMagicDamageToken`. A wield-category
/// offset and an untyped size offset are real but are not a magic
/// enhancement, and are not reported here.
///
/// `None` means the item states no enhancement (the majority of records):
/// honest absence, never a fabricated zero.
///
/// SD-35 `AT-35-E6-003-RULED` cycle 11: the reading moved to ingest
/// ([`crate::pcgen_import::ir_converter::equipment_record_to_corpus`],
/// `decisions.md` §11); the rule and every witness above are unchanged,
/// including the one real family whose magnitude is stated as the name of a
/// variable the SAME record defines (`ultimate_psionics`' dissonance
/// modifiers), which is resolved from that one record and never any other.
pub fn compute_equipmods_effect(
    record: &CorpusEquipmentRecord,
) -> Option<WeaponEnhancementBonus> {
    record.weapon_enhancement.clone()
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
    eqmod_records: &[&CorpusEquipmentRecord],
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

/// One `equipmods` corpus record's settled flat Spell Resistance grant.
///
/// The armour-slot "Spell Resistance" special-ability family
/// (`core_rulebook/cr_equipmods.lst:343-346`) states a literal value that
/// applies unconditionally whenever the wearer's SR is checked -- Decision 7
/// REFINED (`SD31-D7-PROSE-004`) names it the paradigm UNIVERSAL case, so
/// text alone does not satisfy its done-bar; it must be a number.
///
/// The sibling "Bonus Spell Resistance" record states a player CHOICE with a
/// range, not a flat grant, and yields `None` -- no fabricated number, the
/// same discipline every resolver in this module follows.
///
/// SD-35 `AT-35-E6-003-RULED` cycle 11: the reading moved to ingest
/// ([`crate::pcgen_import::ir_converter::equipment_record_to_corpus`],
/// `decisions.md` §11); the rule is unchanged.
pub fn resolve_spell_resistance_bonus(record: &CorpusEquipmentRecord) -> Option<i16> {
    record.spell_resistance_bonus
}

