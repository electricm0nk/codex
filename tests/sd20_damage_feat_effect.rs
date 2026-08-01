//! SD-20 Epic 6 (damage-total engine): fourth damage-class criterion per
//! `scope-draft.md` §1.6's work-unit order — feat-effect modifier
//! (sequential after base-dice `208f326`, STR-modifier `f1188fe`,
//! weapon-enhancement `1eb2eec`).
//!
//! **Bounded scope, per this cycle's brief:** feats whose `BONUS:` token
//! is a directly-usable constant, not a PCGen formula over runtime state
//! (e.g. Power Attack's damage bonus depends on the wielder's base attack
//! bonus and is out of scope here). See
//! `damage_total::resolve_feat_damage_effect`'s own doc comment for the
//! full scoping rationale — in scope vs. still out of scope.
//!
//! This cycle's concrete slice: `KEY:Weapon Specialization` and
//! `KEY:Greater Weapon Specialization` (`core_rulebook/cr_feats.lst`
//! lines 185 and 89 respectively), both carrying a real, verbatim
//! `BONUS:WEAPONPROF=%LIST|DAMAGE|2` token — confirmed directly against
//! the live corpus at
//! `$HOME/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst`.
//! CRB's own benefit text confirms the constant: "You gain a +2 bonus on
//! all damage rolls you make using the selected weapon" (p.137 / p.126).
//!
//! Reads `rules_tables::crb::feats::feat_tables()` directly (no
//! `RulesTables` parameter, `technical-design.md` §2.0) — this table's
//! `effect: Option<&'static [FeatEffectBonus]>` field landed at `3d962c2`,
//! resolving the prior blocked attempt's (`cycle-2026-07-17T1738`) "no
//! numeric effect data" blocker.

use codex::rules_core::damage_total::resolve_feat_damage_effect;

#[test]
fn weapon_specialization_yields_its_real_plus_two_damage_bonus() {
    let resolved = resolve_feat_damage_effect("Weapon Specialization")
        .expect("Weapon Specialization is a real Combat feat with a constant BONUS: token");
    assert_eq!(resolved.feat_key, "Weapon Specialization");
    assert_eq!(
        resolved.damage_bonus, 2,
        "CRB p.137: a +2 bonus on all damage rolls you make using the selected weapon"
    );
    assert_eq!(resolved.table_cell.table, "feats");
    assert_eq!(resolved.table_cell.row_key, "Weapon Specialization");
}

#[test]
fn greater_weapon_specialization_yields_its_real_plus_two_damage_bonus() {
    let resolved = resolve_feat_damage_effect("Greater Weapon Specialization").expect(
        "Greater Weapon Specialization is a real Combat feat with a constant BONUS: token",
    );
    assert_eq!(
        resolved.damage_bonus, 2,
        "CRB p.126: a +2 bonus on all damage rolls you make using the selected weapon"
    );
}

#[test]
fn power_attack_is_formula_based_and_stays_out_of_scope() {
    // Power Attack's BONUS: tokens are all `VAR`-category PCGen formula
    // expressions over BAB (e.g.
    // `PowerAttackDamageModifier|PowerAttackDamageBase*floor(PowerAttackModifier)`),
    // not static literals -- resolving them requires a full PCGen formula
    // evaluator over runtime character state, out of this cycle's bounded
    // scope. Honest `None`, not a fabricated resolved integer -- exactly
    // the fabrication this exact work-unit's prior blocked attempt
    // (`cycle-2026-07-17T1738`) rejected.
    assert!(resolve_feat_damage_effect("Power Attack").is_none());
}

#[test]
fn toughness_has_a_real_effect_but_no_damage_bonus() {
    // Toughness carries a real BONUS: token (HP CURRENTMAX), but no
    // DAMAGE-targeted bonus at all -- honest None, not a fabricated value.
    assert!(resolve_feat_damage_effect("Toughness").is_none());
}

#[test]
fn unrecognized_feat_key_yields_none_not_fabricated() {
    assert!(resolve_feat_damage_effect("Not A Real Feat In The Catalog").is_none());
}
