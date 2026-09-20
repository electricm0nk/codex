#[allow(unused_imports)]
pub(crate) use super::*;

/// Surface direct SD13-E4-R3 runtime evidence for the deterministic Human Wizard
/// level-1 prepared arcane spell-bearing baseline, while keeping it explicitly
/// claim-blocked on its two still-missing burdens.
///
/// This deliberately does not compute a supported spell surface. It grounds no
/// spellbook content, no spells prepared, no spell slots per day, no spell save
/// DCs, no bonus spell slots from a high Intelligence, and no school-power or
/// opposed-school preparation-cost math. It only:
/// - leaves one recognition explanation so the `class:wizard:1` identity is
///   acknowledged as a prepared arcane spell-bearing class rather than an
///   undocumented packet placeholder (direct runtime evidence, carrying no
///   fabricated mechanical value),
/// - grounds one universal, specialization-independent class feature for real:
///   Scribe Scroll, the bonus feat every 1st-level Wizard is granted regardless
///   of arcane school specialization (PF1 Core Rulebook Wizard class feature),
///   letting the Wizard create scrolls of spells they know. This is a bounded
///   grant-only recognition, not a numeric formula: it carries no fabricated
///   mechanical value (+0) and computes no scroll-creation cost, crafting time,
///   spellbook content, or spell-slot machinery,
/// - grounds the flat surface of the school specialization choice for real
///   (SD13-E5), gated on the exact canonical deterministic selections: a
///   recognition record of the Evocation specialization with Necromancy and
///   Transmutation opposed (+0), plus the specialist bonus slot as a flat count
///   only — one 1st-level Evocation-only bonus slot at level 1 (+1), with no
///   cantrip-level bonus slot and no slot contents,
/// - grounds two of the Evocation school's own 1st-level school powers as flat
///   numeric magnitudes (a further SD13-E5 slice), gated on the same canonical
///   Evocation selection: Intense Spells' bonus-damage magnitude (half wizard
///   level, minimum 1) and Force Missile's uses-per-day pool (3 + Intelligence
///   modifier). Both were independently verified against the PF1 Core Rulebook
///   Evocation School rule text (the legacy Paizo PRD mirror, cross-checked by a
///   second independent source) before grounding — Force Missile in particular
///   was treated with skepticism (a name that could plausibly have been confused
///   with non-core material) but confirmed as a genuine 1st-level Evocation
///   school power with exactly the "3 + Int-mod" pool the pre-existing blocker
///   text already claimed. Neither grounding applies any bonus to an actual
///   spell-damage roll, casts any force missile, resolves any automatic-hit
///   targeting, or tracks any action economy or per-use consumption,
/// - grounds the foundational base-attack-bonus / base-save progression pillar
///   (a further SD13-E5 slice) that every other class row in this matrix
///   (Fighter, Barbarian, Monk, Rogue, Paladin, Druid, Cleric, Bard, Sorcerer)
///   already has and Wizard never had: base attack bonus (1/2 BAB, `classlevel
///   / 2` — the same shape as Sorcerer, UNLIKE the 3/4 BAB shared by
///   Rogue/Monk/Druid/Cleric/Bard) and base save progression (good Will only,
///   poor Fortitude, poor Reflex). Both were verified against the PF1 Core
///   Rulebook Wizard class table (d20pfsrd and the legacy Paizo PRD mirror),
///   reading the raw level 1-6 rows directly (BAB +0/+1/+1/+2/+2/+3, Fort
///   +0/+0/+1/+1/+1/+2, Ref +0/+0/+1/+1/+1/+2, Will +2/+3/+3/+4/+4/+5) rather
///   than assumed from Sorcerer's matching shape; the level 4/5 BAB values (+2
///   at both) disambiguate the 1/2-vs-3/4 fraction since level 1 alone floors
///   every fraction to +0. Both pillars are grounded as flat, standalone
///   `ComputationExplanation` records mirroring the exact "standalone, not
///   wired into the integrated `PilotBaseChassisComputation`" idiom already
///   used for every other class's own base-attack/base-save grounding: neither
///   is wired into `base_attack_bonus`, `compute_total_saves`, or
///   `compute_combat_baseline`, and
/// - emits two distinct claim-blocking diagnostics naming the school-power
///   execution / opposed-school-preparation-cost burden (the still-unimplemented
///   spell-damage application for Intense Spells, the still-unimplemented
///   casting execution for Force Missile, and the two-prepared-slot cost for
///   opposed-school spells) and the prepared spellbook / spells-prepared /
///   spell-slot posture burden explicitly, rather than hiding behind a generic
///   "unsupported caster" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this
/// seam keeps that blocked posture but makes the Wizard prepared spell-bearing
/// identity, its grounded class-feature surfaces, and its remaining named
/// burdens legible on the runtime path. The matrix file row transition
/// (Unverified/Observed → Blocked/Computed, then Blocked → Partial once Scribe
/// Scroll is grounded) was recorded by this proof surface and applied to the
/// in-source carrier directly (the support-state matrix itself is retired,
/// SD-36 D3; this is now a historical note).
///
/// A further SD13-E5 slice widens the level-1-only gate (`supported_wizard_level`,
/// 1..=2) and extends every one of the formulas above to level 2 via the same
/// formula, without re-derivation, verified independently against the PF1 Core
/// Rulebook Wizard class table (d20pfsrd and legacy.aonprd.com): level 2 base attack
/// bonus is +1, base saves are +0/+0/+3 (Fortitude/Reflex/Will); the specialist bonus
/// slot count stays exactly 1 (a level-2 wizard still only casts 1st-level spells,
/// since 2nd-level wizard spells require caster level 3); Intense Spells' bonus
/// damage stays 1, reached naturally (`max(2/2, 1) = 1`) rather than via the level-1
/// floor; Force Missile's uses-per-day pool is level-independent and unchanged;
/// Scribe Scroll is granted once, at 1st level only, and stays recognized as an
/// already-held grant (its detail text hardcodes "1st level" as the level it was
/// granted, never re-deriving a level-2 grant event). The class table's level-2
/// "Special" column is blank (verified independently against both sources), so no
/// new class feature is gained at 2nd level, unlike Rogue/Monk/Druid's Evasion/
/// Woodland Stride — this slice widens existing pillars only, adds no new one.
///
/// A further SD13-E5 slice widens the gate again (`supported_wizard_level`, 1..=3)
/// and extends the same formulas to level 3, without re-derivation, verified
/// independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and
/// legacy.aonprd.com): level 3 base attack bonus is +1, base saves are +1/+1/+3
/// (Fortitude/Reflex/Will); Intense Spells' bonus damage stays 1
/// (`max(3/2, 1) = 1`); Force Missile's uses-per-day pool is level-independent and
/// unchanged; Scribe Scroll stays recognized as an already-held grant. The
/// specialist bonus slot flat count, in contrast, CHANGES for real at level 3: the
/// PF1 Core Rulebook arcane school class feature grants "an additional spell slot of
/// each spell level he can cast, from 1st on up" (verified against both primary
/// sources' exact rule text), and the raw Wizard spells-per-day table rows (also
/// verified against both sources) show a level-3 wizard casts 2nd-level spells for
/// the first time (level 2: "4/2/—/—"; level 3: "4/2/1/—"), so the flat count
/// becomes 2 (one 1st-level bonus slot plus one 2nd-level bonus slot), up from 1.
/// The class table's level-3 "Special" column is also blank (verified
/// independently against both sources), so no new class feature is gained at 3rd
/// level either, unlike Rogue/Monk/Barbarian's own 3rd-level features — this slice
/// widens existing pillars only (one of them to a new value), adds no new pillar
/// record.
///
/// A further SD13-E5 slice widens the gate again (`supported_wizard_level`, 1..=4)
/// and extends the same formulas to level 4, without re-derivation, verified
/// independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and
/// legacy.aonprd.com): level 4 base attack bonus is +2, base saves are +1/+1/+4
/// (Fortitude/Reflex/Will). The specialist bonus slot flat count, checked rather
/// than assumed to double again, STAYS at 2: the raw Wizard spells-per-day table's
/// level-4 row is still "4/3/2/—/—" — 3rd-level wizard spells do not become
/// available until wizard level 5 (level 5 row: "4/3/2/1/—", the first non-"—"
/// 3rd-level column) — so a level-4 specialist still only casts 1st- and 2nd-level
/// spells and the pre-existing `level >= WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL`
/// gate already produces the correct value with no formula change. Intense Spells'
/// bonus-damage magnitude, in contrast, CHANGES for real at level 4: `max(4 / 2, 1) =
/// 2`, up from 1 at levels 1-3 — the first value change this pillar's formula
/// produces since it was grounded. Force Missile's uses-per-day pool is
/// level-independent and unchanged; Scribe Scroll stays recognized as an
/// already-held grant. The class table's level-4 "Special" column is also blank
/// (verified independently against both sources: the Wizard's own next class
/// feature, a bonus feat, is granted at 5th level, not 4th) — this slice widens
/// existing pillars only (one of them, Intense Spells, to a genuinely new value),
/// adds no new pillar record.
///
/// A further SD13-E5 slice widens the gate again (`supported_wizard_level`, 1..=5)
/// and extends the same formulas to level 5, without re-derivation, verified
/// independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and a
/// second independent Archives of Nethys mirror): level 5 base attack bonus is +2,
/// base saves are +1/+1/+4 (Fortitude/Reflex/Will) — all four values numerically
/// IDENTICAL to level 4, an integer-division coincidence (`5 / 2` and `4 / 2` both
/// floor to `2`; `5 / 3` and `4 / 3` both floor to `1`), not a sign any formula
/// stopped scaling. The specialist bonus slot flat count is the exact question this
/// cycle was briefed to verify: the raw Wizard spells-per-day table's level-5 row is
/// "4/3/2/1/—" — 3rd-level wizard spells become available for the first time at
/// wizard level 5 (level 4 row was "4/3/2/—/—") — so a level-5 specialist now casts
/// 1st-, 2nd-, and 3rd-level spells, and the flat count genuinely becomes 3 (one
/// bonus slot of each spell level 1st through 3rd), up from 2 at levels 3-4. Intense
/// Spells' bonus-damage magnitude, in contrast, STAYS at 2 at level 5: `max(5 / 2, 1)
/// = 2`, another integer-division coincidence, not a formula that stopped scaling.
/// Force Missile's uses-per-day pool is level-independent and unchanged; Scribe
/// Scroll stays recognized as an already-held grant. The class table's level-5
/// "Special" column reads "Bonus feat" (verified independently against both
/// sources) — a genuinely NEW Wizard class feature at 5th level, but checked and
/// confirmed NOT flat: the feat is chosen from an open-ended set of metamagic feats,
/// item creation feats (each its own family with its own prerequisites), or the
/// single named Spell Mastery feature — a general feat-selection/feat-prerequisite
/// engine, not a flat magnitude, mirroring the Monk High Jump precedent exactly
/// (checked rather than assumed, deliberately left named-but-unproven, no record or
/// diagnostic fabricated for it). This slice widens existing pillars only (one of
/// them, the specialist bonus slot count, to a genuinely new value), adds no new
/// pillar record.
///
/// A further SD13-E5 slice widens the gate again (`supported_wizard_level`, 1..=6)
/// and extends the same formulas to level 6, without re-derivation, verified
/// independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and
/// legacy.aonprd.com): level 6 base attack bonus is +3, base saves are +2/+2/+5
/// (Fortitude/Reflex/Will) — all four values genuinely NEW, up from +2/+1/+1/+4 at
/// level 5. The specialist bonus slot flat count, checked rather than assumed to
/// rise again, STAYS at 3: the raw Wizard spells-per-day table's level-6 row is
/// "4/3/3/2/—" — 4th-level wizard spells do not become available until wizard level
/// 7 (level 7 row: "4/4/3/2/1", the first non-"—" 4th-level column) — so a level-6
/// specialist still only casts 1st-, 2nd-, and 3rd-level spells. Intense Spells'
/// bonus-damage magnitude, in contrast, CHANGES for real at level 6: `max(6 / 2, 1) =
/// 3`, up from 2 at level 5, via the same pre-existing formula, not re-derived.
/// Force Missile's uses-per-day pool is level-independent and unchanged; Scribe
/// Scroll stays recognized as an already-held grant. The class table's level-6
/// "Special" column is genuinely BLANK (verified independently against both
/// sources, checked rather than assumed away) — UNLIKE the level-5 "Bonus feat"
/// entry, no new Wizard class feature is gained at 6th level — this slice widens
/// existing pillars only (one of them, Intense Spells, to a genuinely new value),
/// adds no new pillar record.
///
/// A further SD13-E5 slice widens the gate again (`supported_wizard_level`, 1..=7)
/// and extends the same formulas to level 7, without re-derivation, verified
/// independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and
/// legacy.aonprd.com): level 7 base attack bonus and all three base saves are
/// numerically UNCHANGED from level 6 (+3 base attack, +2/+2/+5
/// Fortitude/Reflex/Will) — an integer-division coincidence, re-verified rather
/// than assumed. The specialist bonus slot flat count, checked rather than assumed
/// to stay put, GENUINELY RISES to 4: the raw Wizard spells-per-day table's
/// level-7 row is "4/4/3/2/1" — the first non-"—" 4th-level column — so a level-7
/// specialist now casts 4th-level spells for the first time. Intense Spells'
/// bonus-damage magnitude STAYS at 3 (`max(7/2, 1) = 3`, unchanged from level 6,
/// another integer-division coincidence). Force Missile's uses-per-day pool is
/// level-independent and unchanged; Scribe Scroll stays recognized as an
/// already-held grant. The class table's level-7 "Special" column is genuinely
/// BLANK (verified independently against both sources), so no new Wizard class
/// feature is gained at 7th level — this slice widens existing pillars only (one
/// of them, the specialist bonus slot, to a genuinely new value), adds no new
/// pillar record.
pub(super) fn explain_wizard_level1_prepared_spell_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // SD-24 Epic 5 (criterion 5.1): widened from `supported_wizard_level`
    // (single-class-only) to `wizard_level_in_mix`, so every Wizard-specific
    // explanation this function grounds keeps firing, using Wizard's own
    // sub-level, once a supported second class (Fighter) joins the mix --
    // mirroring `explain_fighter_class_features`'s identical SD-21 E7.30
    // reconciliation for Fighter's own features. Single-class Wizard inputs
    // are unaffected: `wizard_level_in_mix` returns the identical
    // `supported_wizard_level` value for them.
    let Some(level) = wizard_level_in_mix(input) else {
        return;
    };
    // v0.6 alpha swarm (risks-and-open-questions.md item 18, widened
    // 2026-07-24): this function's race gate was removed after tracing every
    // formula/explanation it grounds -- none of them reads `race_id` or any
    // race-specific table. `ability_modifiers` already arrives race-adjusted
    // from `apply_human_ability_bonus` (the same input every other race's
    // chassis computation already uses), the BAB/save/spells-per-day/
    // spell-save-DC formulas are all class-table- and level-driven, and the
    // school-specialization recognition reads `selected_choices`, not race.
    // The two explanation strings below previously said "Human Wizard"
    // specifically; generalized to name the character's actual race.
    let wizard_race_label = race_display_label(&input.chosen.race_id);

    // Direct runtime evidence: recognize the deterministic Wizard level-1
    // prepared arcane spell-bearing identity, for any race. This is a
    // recognition record only; it fabricates no spell math and no
    // school-opposition / specialty school bonus math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.wizard".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic {wizard_race_label} Wizard level {level} prepared arcane \
             spell-bearing baseline: the {WIZARD_CLASS_ID}:{level} class identity is acknowledged \
             as a \
             prepared arcane spell-bearing class on the rules-core seam rather than an \
             undocumented packet placeholder. This is a bounded recognition record only; it \
             grounds no spellbook content, no spells prepared per day, no spell slots per day, \
             no bonus spell slots from a high Intelligence, no school specialization \
             mechanics, no opposed-school bookkeeping, and no specialty school bonus, so it \
             carries no fabricated mechanical value (+0). (v0.6 alpha swarm: the base spell \
             save DC formula is now grounded separately, in its own \
             class_chassis.wizard.spell_save_dc.spell_level_* records below.)"
        ),
    });

    // Grounded (SD13-E5): the foundational base-attack-bonus / base-save progression
    // pillar that every other class row in this matrix (Fighter, Barbarian, Monk,
    // Rogue, Paladin, Druid, Cleric, Bard, Sorcerer) already has and Wizard never had
    // at all. Both formulas were verified against the PF1 Core Rulebook Wizard class
    // table (d20pfsrd and the legacy Paizo PRD mirror) before writing this code,
    // reading the raw level 1-6 table rows directly (BAB +0/+1/+1/+2/+2/+3, Fort
    // +0/+0/+1/+1/+1/+2, Ref +0/+0/+1/+1/+1/+2, Will +2/+3/+3/+4/+4/+5) rather than
    // assuming Wizard's shape merely because it resembles another arcane class: the
    // level 4/5 BAB values (+2 at both) disambiguate the 1/2-vs-3/4 fraction (level 1
    // alone floors every fraction to +0) and confirm Wizard is 1/2 BAB — the SAME
    // shape as Sorcerer, UNLIKE the 3/4 BAB shared by Rogue/Monk/Druid/Cleric/Bard —
    // and the raw Fort/Ref/Will columns independently confirm good Will only, poor
    // Fortitude, poor Reflex (also matching Sorcerer's shape, confirmed rather than
    // assumed).
    let wizard_level_value = i16::from(level);

    // Grounded (1/2): 1/2-BAB base-attack progression (classlevel / 2) — the same
    // shape as Sorcerer, NOT the 3/4-BAB shape shared by Rogue/Monk/Druid/Cleric/Bard.
    let wizard_base_attack_bonus = wizard_level_value / 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.wizard.base_attack_bonus".to_owned(),
        value: wizard_base_attack_bonus,
        detail: format!(
            "Wizard level {level} base attack bonus from the PF1 Core Rulebook Wizard class \
             table's 1/2-BAB progression — the same shape as Sorcerer, UNLIKE the 3/4-BAB shape \
             shared by Rogue/Monk/Druid/Cleric/Bard: classlevel / 2 = {wizard_base_attack_bonus}. \
             This is a standalone explanation record; it is not wired into the integrated \
             base_attack_bonus field or into compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — poor Fortitude, poor Reflex, good Will,
    // verified against the PF1 Core Rulebook Wizard class table (Fortitude +0, Reflex
    // +0, Will +2 at level 1; +0, +0, +3 at level 2 — same formulas, not re-derived).
    let wizard_good_save = wizard_level_value / 2 + 2;
    let wizard_poor_save = wizard_level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.wizard.base_save.fortitude".to_owned(),
        value: wizard_poor_save,
        detail: format!(
            "Wizard level {level} base Fortitude save (poor save) from the PF1 Core Rulebook \
             Wizard class table: classlevel/3 = {wizard_poor_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.wizard.base_save.reflex".to_owned(),
        value: wizard_poor_save,
        detail: format!(
            "Wizard level {level} base Reflex save (poor save) from the PF1 Core Rulebook Wizard \
             class table: classlevel/3 = {wizard_poor_save}. This is a standalone explanation \
             record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.wizard.base_save.will".to_owned(),
        value: wizard_good_save,
        detail: format!(
            "Wizard level {level} base Will save (good save) from the PF1 Core Rulebook Wizard \
             class table: classlevel/2+2 = {wizard_good_save}. This is a standalone explanation \
             record; it is not wired into compute_total_saves"
        ),
    });

    // v0.6 alpha swarm (QA-found gap): the base spell-save-DC arithmetic, one
    // record per ACCESSIBLE spell level, completing the DC family alongside
    // Paladin/Ranger/Sorcerer/Bard (each already grounded via this exact
    // pattern). Verified against PCGen's cr_classes.lst (SPELLSTAT:INT) and
    // both PF1 primary sources, which state the rule identically: "The
    // Difficulty Class for a saving throw against a wizard's spell is 10 +
    // the spell level + the wizard's Intelligence modifier" — Intelligence,
    // unlike the Paladin/Sorcerer/Bard's Charisma or the Ranger's Wisdom.
    // This is unconditional on school specialization: every wizard, not just
    // a specialist, has the same spell-level access ladder and the same DC
    // formula, so this sits outside the
    // `wizard_has_canonical_specialization_selections` gate above, mirroring
    // how the other four classes' DC records are never gated on a class
    // feature choice either. The access ladder reuses the same
    // `WIZARD_<N>TH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` thresholds already
    // verified and grounded above for the specialist bonus slot ladder — the
    // underlying spells-per-day table row is the same for every wizard,
    // specialist or universalist, so no new verification was needed. This
    // grounds only the base DC formula over values already on the seam: no
    // saving-throw resolution, no target, no spell selection, and no feat DC
    // modifiers are computed.
    let wizard_spell_level_access: i16 =
        if level >= WIZARD_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            9
        } else if level >= WIZARD_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            8
        } else if level >= WIZARD_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            7
        } else if level >= WIZARD_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            6
        } else if level >= WIZARD_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            5
        } else if level >= WIZARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            4
        } else if level >= WIZARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            3
        } else if level >= WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            2
        } else {
            1
        };
    for spell_level in 1..=wizard_spell_level_access {
        let spell_save_dc = 10 + spell_level + ability_modifiers.intelligence;
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.wizard.spell_save_dc.spell_level_{spell_level}"),
            value: spell_save_dc,
            detail: format!(
                "Wizard spell save DC at wizard level {level}, spell level {spell_level}: 10 + \
                 {spell_level} + Intelligence modifier {} = {spell_save_dc} (PF1 Core Rulebook, \
                 verified identically on both primary sources and PCGen's cr_classes.lst \
                 SPELLSTAT:INT token: \"The Difficulty Class for a saving throw against a \
                 wizard's spell is 10 + the spell level + the wizard's Intelligence \
                 modifier\" — Intelligence, not the Paladin/Sorcerer/Bard's Charisma or the \
                 Ranger's Wisdom). This grounds the base DC formula only: no saving-throw \
                 resolution, no target, no spell selection, and no feat DC modifiers are \
                 computed",
                ability_modifiers.intelligence
            ),
        });
    }

    // Grounded for real: Scribe Scroll is a universal, specialization-independent
    // Wizard class feature (every 1st-level Wizard is granted it regardless of
    // which school, if any, is later chosen), so it is separable from the
    // school-specialization burden. It is a boolean grant, not a numeric formula.
    // Verified against both PF1 CRB primary sources (d20pfsrd and legacy.aonprd.com):
    // Scribe Scroll is granted exactly once, in the level-1 "Special" column, never
    // re-granted at 2nd level or later. Since the wizard keeps the feat once granted,
    // this record's header cites the character's current level (still recognized at
    // level 2+ within this seam's supported range), but its body text hardcodes "1st
    // level" as the level the feat was actually granted, mirroring the Sorcerer
    // Eschew Materials idiom exactly: no level-2 grant event is re-derived.
    explanations.push(ComputationExplanation {
        id: "class_chassis.wizard.scribe_scroll".to_owned(),
        value: 0,
        detail: format!(
            "Recognized Wizard level {level} Scribe Scroll bonus feat grant: every Wizard, \
             regardless of arcane school specialization, is granted Scribe Scroll as a bonus \
             feat at 1st level (PF1 Core Rulebook Wizard class feature), letting the Wizard \
             create scrolls of spells they know. This is a one-time grant recognized once and \
             kept thereafter, not re-granted at 2nd level or later. This is a bounded grant-only \
             recognition: it carries no fabricated mechanical value (+0) and computes no scroll \
             creation cost, no crafting time, no spellbook content, and no spell-slot machinery"
        ),
    });

    // Grounded for real (SD13-E5): the flat surface of the school specialization
    // choice, gated on the exact canonical deterministic selections ("canonical"
    // versus "absent or anything else"). An input without them (e.g. a
    // universalist-shaped request that never made the choice) gains no
    // specialization recognition and no specialist bonus slot.
    if wizard_has_canonical_specialization_selections(input) {
        explanations.push(ComputationExplanation {
            id: "class_chassis.wizard.specialization_choice".to_owned(),
            value: 0,
            detail: format!(
                "Recognized Wizard level {level} school specialization choice: the canonical \
                 deterministic selections choose Evocation as the specialty arcane school \
                 ({WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID} -> {EVOCATION_SCHOOL_SELECTION}) with \
                 Necromancy and Transmutation as the two opposed schools \
                 ({WIZARD_OPPOSED_SCHOOLS_CHOICE_ID} -> {NECROMANCY_SCHOOL_SELECTION}, \
                 {TRANSMUTATION_SCHOOL_SELECTION}), per the PF1 Core Rulebook arcane school class \
                 feature. A wizard's chosen school does not change by level, so this recognition \
                 is not level-gated. This is a bounded recognition record of the choice identity \
                 only: it carries no fabricated mechanical value (+0) and computes no school \
                 power, no opposed-school preparation cost, and no spell math"
            ),
        });
        // Grounded for real: the specialist bonus slot flat count. Confirmed unchanged
        // at level 2 (SD13-E5): a level-2 wizard still only casts 1st-level wizard
        // spells (2nd-level wizard spells require caster level 3, verified against
        // both primary sources' raw spells-per-day table rows), so "one additional
        // spell slot of each spell level she can cast" is still exactly one 1st-level
        // slot at both levels 1 and 2 this seam supports. A further SD13-E5 slice
        // widens this for real at level 3: a level-3 wizard casts 2nd-level spells
        // for the first time (verified independently against both primary sources'
        // raw spells-per-day table rows), so the specialist now gains one bonus slot
        // of EACH spell level she can cast — one 1st-level bonus slot plus one
        // 2nd-level bonus slot, a flat count of 2.
        let wizard_specialist_bonus_slot_count =
            if level >= WIZARD_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_17
            } else if level >= WIZARD_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_15
            } else if level >= WIZARD_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_13
            } else if level >= WIZARD_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_11
            } else if level >= WIZARD_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_9
            } else if level >= WIZARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_7
            } else if level >= WIZARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_5
            } else if level >= WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_3
            } else {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVELS_1_AND_2
            };
        explanations.push(ComputationExplanation {
            id: "class_chassis.wizard.specialist_bonus_slot".to_owned(),
            value: wizard_specialist_bonus_slot_count,
            detail: format!(
                "Wizard level {level} specialist bonus spell slot: a specialist wizard gains one \
                 additional Evocation-only spell slot of each spell level she can cast, 1st and \
                 up, usable only for spells of the chosen school (PF1 Core Rulebook arcane \
                 school class feature). At levels 1-2 a wizard casts only 1st-level spells, so \
                 the flat count is exactly one 1st-level Evocation-only bonus slot \
                 ({WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVELS_1_AND_2:+}); at level \
                 {WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 2nd-level \
                 spells for the first time (verified against both primary sources' raw \
                 spells-per-day table rows), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_3:+} (one 1st-level Evocation-only bonus \
                 slot plus one 2nd-level Evocation-only bonus slot); at level \
                 {WIZARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 3rd-level \
                 spells for the first time (verified against both primary sources' raw \
                 spells-per-day table rows), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_5:+} (one 1st-level, one 2nd-level, and \
                 one 3rd-level Evocation-only bonus slot); at level \
                 {WIZARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 4th-level \
                 spells for the first time (verified against both primary sources' raw \
                 spells-per-day table rows), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_7:+} (one 1st-level, one 2nd-level, one \
                 3rd-level, and one 4th-level Evocation-only bonus slot); at level \
                 {WIZARD_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 5th-level \
                 spells for the first time (verified against both primary sources' raw \
                 spells-per-day table rows), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_9:+}; at level \
                 {WIZARD_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 6th-level \
                 spells for the first time (verified independently against both primary sources' \
                 raw spells-per-day table rows: the level-10 row is \"4/4/4/3/3/2\" with a still-\
                 \"—\" 6th-level column, the level-11 row is \"4/4/4/4/3/2/1\", the first non-\"—\" \
                 6th-level column), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_11:+} (one bonus slot of each spell \
                 level 1st through 6th); at level \
                 {WIZARD_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 7th-level \
                 spells for the first time (verified independently against both primary sources' \
                 raw spells-per-day table rows: the level-12 row is \"4/4/4/4/3/3/2\" with a \
                 still-\"—\" 7th-level column, the level-13 row is \"4/4/4/4/4/3/2/1\", the first \
                 non-\"—\" 7th-level column), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_13:+} (one bonus slot of each spell \
                 level 1st through 7th); at level \
                 {WIZARD_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 8th-level \
                 spells for the first time (verified independently against both primary sources' \
                 raw spells-per-day table rows: the level-14 row is \"4/4/4/4/4/3/3/2\" with a \
                 still-\"—\" 8th-level column, the level-15 row is \"4/4/4/4/4/4/3/2/1\", the \
                 first non-\"—\" 8th-level column), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_15:+} (one bonus slot of each spell \
                 level 1st through 8th); at level \
                 {WIZARD_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 9th-level \
                 spells for the first time (verified independently against both primary sources' \
                 raw spells-per-day table rows: the level-16 row is \"4/4/4/4/4/4/3/3/2\" with a \
                 still-\"—\" 9th-level column, the level-17 row is \"4/4/4/4/4/4/4/3/2/1\", the \
                 first non-\"—\" 9th-level column), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_17:+} (one bonus slot of each spell \
                 level 1st through 9th). At level {level} this \
                 is {wizard_specialist_bonus_slot_count:+} flat count; there is no cantrip-level \
                 bonus slot. This grounds the flat count only: no slot contents, no spells \
                 prepared per day, no per-day slot totals, and no bonus slots from a high \
                 Intelligence are computed"
            ),
        });

        // Grounded for real (SD13-E5): Intense Spells' flat bonus-damage magnitude.
        // PF1 Core Rulebook Evocation School: whenever an evocation spell that deals
        // hit point damage is cast, add half wizard level (minimum 1) to the damage.
        // Verified against the legacy Paizo PRD mirror rather than trusted from
        // memory or the pre-existing blocker-message claim. This is a flat,
        // non-dice magnitude, so it grounds for real, mirroring the Cleric Touch of
        // Good sacred-bonus idiom exactly. Confirmed at level 2: max(2/2, 1) = 1,
        // reached naturally via the formula rather than via the level-1 floor.
        let intense_spells_bonus_damage = (wizard_level_value / 2).max(1);
        explanations.push(ComputationExplanation {
            id: "class_chassis.wizard.intense_bonus_damage".to_owned(),
            value: intense_spells_bonus_damage,
            detail: format!(
                "Wizard level {level} Evocation school power Intense Spells bonus-damage \
                 magnitude (PF1 Core Rulebook Evocation School): whenever an evocation spell \
                 that deals hit point damage is cast, add half wizard level (minimum 1) to the \
                 damage. At Wizard level {level} this is max({level} / 2, 1) = \
                 {intense_spells_bonus_damage}. This grounds only the flat bonus-damage \
                 magnitude; it applies no bonus to any actual spell-damage roll and implements \
                 no spell-damage-application engine"
            ),
        });

        // Grounded for real (SD13-E5): Force Missile's flat uses-per-day pool. PF1
        // Core Rulebook Evocation School: as a standard action, a specialist
        // Evocation wizard may unleash a force missile (as magic missile, dealing
        // 1d4 points of damage plus the Intense Spells bonus) that automatically
        // strikes a foe, usable 3 + Intelligence modifier times per day. Verified
        // against the legacy Paizo PRD mirror with deliberate skepticism (a name
        // that could plausibly have been confused with non-core material), which
        // confirmed the power is genuinely core and the "3 + Int-mod" pool the
        // pre-existing blocker text already claimed is correct. Only the flat
        // daily-use count is a non-dice formula; the 1d4 damage roll and the
        // automatic-hit casting execution are not flat, so they stay unproven. This
        // pool is level-independent and confirmed unchanged at level 2.
        let force_missile_uses_per_day = (3 + ability_modifiers.intelligence).max(0);
        explanations.push(ComputationExplanation {
            id: "class_chassis.wizard.force_missile_uses_per_day".to_owned(),
            value: force_missile_uses_per_day,
            detail: format!(
                "Wizard level {level} Evocation school power Force Missile uses per day (PF1 \
                 Core Rulebook Evocation School): 3 + Intelligence modifier, floored at 0. At \
                 Intelligence modifier {} this is max(3 + {}, 0) = \
                 {force_missile_uses_per_day}. This grounds only the flat daily-use count; it \
                 casts no force missile, applies no 1d4 damage roll, resolves no automatic-hit \
                 magic-missile-style targeting, and tracks no action economy or per-use \
                 consumption",
                ability_modifiers.intelligence, ability_modifiers.intelligence
            ),
        });
    }

    // Task #66: Wizard's Abjuration arcane school -- the Wizard-fed half only.
    // Verified directly against `cr_abilities_class.lst`'s `KEY:Abjuration
    // School ~ *` records this task: `AbjurationSchoolLVL` <- `ArcaneSchoolLVL`
    // <- `WizardLVL`, and `AbjurationProgressionSchoolLVL` <-
    // `ArcaneSchoolProgressionLVL` <- `WizardLVL` identically (the "Arcane
    // School Tracker" internal record). `ArcaneSchoolLVL` is ALSO fed by
    // `ArcanistLvl` via the Arcanist Exploit "School Understanding" -- that
    // path is NOT built here (School Understanding is a separate,
    // not-yet-built chooser-in-chooser, per task #55's scoping), so for a
    // Wizard-only input both trackers reduce to exactly this Wizard's own
    // class level. Explanation ids live under the shared
    // `class_feature.school.abjuration.*` namespace (not a Wizard-specific
    // id), matching this session's `class_feature.familiar.*` /
    // `class_feature.domain.*` shared-ladder-family precedent, so the
    // namespace is already correctly positioned for when Arcanist's own half
    // is built later.
    if wizard_has_canonical_abjuration_selection(input) {
        let abjuration_school_lvl = wizard_level_value;
        let abjuration_progression_school_lvl = wizard_level_value;

        // Grounded for real: Resistance (`KEY:Abjuration School ~
        // Resistance`). `BONUS:VAR|AbjurationResistanceBonus|5` fires
        // unconditionally once the power itself unlocks
        // (`PREVARGTEQ:AbjurationProgressionSchoolLVL,1` on the Abjuration
        // School record's own `ABILITY:` grant line, i.e. from level 1), and
        // a second `BONUS:VAR|AbjurationResistanceBonus|5|
        // PREVARGTEQ:AbjurationProgressionSchoolLVL,11` stacks another +5 at
        // level 11+, for a flat 10. At level 20 the record's own DESC text
        // (not a BONUS:VAR formula) replaces "resistance" with "immunity to
        // an energy type" -- a non-numeric capstone. The corpus's own
        // `#Immunity` KEY record is commented out (an inactive record, not a
        // live ability), so Immunity stays deferred/named-only: no
        // Resistance magnitude is claimed at level 20+, since there is no
        // longer a "resistance" number to ground.
        if (1..20).contains(&abjuration_progression_school_lvl) {
            let resistance_bonus = if abjuration_progression_school_lvl >= 11 { 10 } else { 5 };
            explanations.push(ComputationExplanation {
                id: "class_feature.school.abjuration.resistance".to_owned(),
                value: resistance_bonus,
                detail: format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   BONUS:VAR|AbjurationResistanceBonus|5 lines in the corpus record
                    "Wizard level {level} Abjuration School power Resistance flat magnitude (PF1 \
                     Core Rulebook Abjuration School, `KEY:Abjuration School ~ Resistance`): \
                     resistance 5 to an energy type of your choice, chosen when you prepare spells; \
                     the resistance increases to 10 at AbjurationProgressionSchoolLVL 11+ (two \
                     stacked). At Wizard level {level} this is {resistance_bonus}. This grounds only \
                     the flat resistance magnitude; it applies no resistance to any actual damage \
                     roll, tracks no daily energy-type reselection, and implements no \
                     energy-type-of-choice bookkeeping. At level 20 the corpus record's DESC text \
                     replaces resistance with immunity to an energy type -- a tokenless, non-numeric \
                     capstone this slice leaves deferred/named-only, so no Resistance explanation \
                     fires at level 20 or above"
                ),
            });
        }

        // Grounded for real: Protective Ward (`KEY:Abjuration School ~
        // Protective Ward`) -- three flat, non-dice `BONUS:VAR` formulas, all
        // unlocked from level 1 (`PREVARGTEQ:AbjurationProgressionSchoolLVL,1`
        // on the Abjuration School record's own `ABILITY:` grant line for
        // Protective Ward).
        if abjuration_progression_school_lvl >= 1 {
            // `AbjurationProtectiveWardTimes|ArcaneSchoolPowerTimes`.
            // `ArcaneSchoolPowerTimes` is itself `DEFINE:
            // ArcaneSchoolPowerTimes|0` `BONUS:VAR|ArcaneSchoolPowerTimes|
            // INT+3` on the shared "Arcane School Tracker" internal record --
            // the SAME "3 + Intelligence modifier" idiom the pre-existing
            // Force Missile grounding above already uses (`INT` resolves to
            // the ability MODIFIER in this codebase's BONUS:VAR convention).
            let protective_ward_uses_per_day = (3 + ability_modifiers.intelligence).max(0);
            explanations.push(ComputationExplanation {
                id: "class_feature.school.abjuration.protective_ward_uses_per_day".to_owned(),
                value: protective_ward_uses_per_day,
                detail: format!(
                    "Wizard level {level} Abjuration School power Protective Ward uses-per-day \
                     pool (PF1 Core Rulebook Abjuration School): \
                     AbjurationProtectiveWardTimes resolves to the shared \
                     ArcaneSchoolPowerTimes counter, 3 + Intelligence modifier, floored at 0 \
                     (the same idiom as the Evocation Force Missile pool above). At \
                     Intelligence modifier {} this is max(3 + {}, 0) = \
                     {protective_ward_uses_per_day}. This grounds only the flat daily-use \
                     count; it creates no 10-foot-radius protective-magic field, and tracks no \
                     action economy or per-use consumption. Task #88 correction: \
                     `defense.baseline_armor_class` IS a real integrated AC total this codebase \
                     computes, but this pool-size record has no bonus of its own to apply to it \
                     anyway (the deflection bonus itself is grounded separately below)",
                    ability_modifiers.intelligence, ability_modifiers.intelligence
                ),
            });

            // `AbjurationProtectiveWardDuration|INT` -- the bare Intelligence
            // modifier. No floor is encoded in this corpus formula itself
            // (unlike the Times formula's own built-in "+3" offset), so this
            // grounds exactly the corpus formula rather than inventing an
            // unencoded minimum.
            let protective_ward_duration = ability_modifiers.intelligence;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.abjuration.protective_ward_duration".to_owned(),
                value: protective_ward_duration,
                detail: format!(
                    "Wizard level {level} Abjuration School power Protective Ward duration in \
                     rounds (PF1 Core Rulebook Abjuration School): \
                     AbjurationProtectiveWardDuration resolves to the bare Intelligence \
                     modifier, {protective_ward_duration}. This grounds only the flat \
                     round-count magnitude; no floor is encoded in the corpus formula itself, \
                     so none is fabricated here. It creates no protective-magic field and \
                     tracks no round-by-round duration"
                ),
            });

            // `AbjurationProtectiveWardBonus|(AbjurationSchoolLVL/5)+1` --
            // integer division, matching the "half wizard level" flooring
            // idiom already used by Intense Spells above.
            let protective_ward_deflection_bonus = (abjuration_school_lvl / 5) + 1;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.abjuration.protective_ward_deflection_bonus"
                    .to_owned(),
                value: protective_ward_deflection_bonus,
                detail: format!(
                    "Wizard level {level} Abjuration School power Protective Ward deflection \
                     bonus to AC (PF1 Core Rulebook Abjuration School): \
                     AbjurationProtectiveWardBonus resolves to \
                     (AbjurationSchoolLVL/5)+1 = ({abjuration_school_lvl}/5)+1 = \
                     {protective_ward_deflection_bonus}. Grounds only the flat deflection- \
                     bonus magnitude -- task #88 correction: `defense.baseline_armor_class` IS \
                     a real integrated AC total this codebase computes (the same total \
                     Brawler's own AC Bonus already integrates into), this magnitude is simply \
                     not wired into it yet, and it grants no real allies-in-area targeting \
                     (unlike the self-only bonuses already wired in, Protective Ward's real \
                     benefit is an area effect on allies, which stays unmodeled regardless)"
                ),
            });
        }

        // Grounded for real: Energy Absorption (`KEY:Abjuration School ~
        // Energy Absorption`). `BONUS:VAR|AbjurationEnergyAbsorption|
        // AbjurationSchoolLVL*3`, gated on the power's own level-6 unlock
        // (`PREVARGTEQ:AbjurationProgressionSchoolLVL,6` on the Abjuration
        // School record's own `ABILITY:` grant line for Energy Absorption).
        if abjuration_progression_school_lvl >= 6 {
            let energy_absorption = abjuration_school_lvl * 3;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.abjuration.energy_absorption".to_owned(),
                value: energy_absorption,
                detail: format!(
                    "Wizard level {level} Abjuration School power Energy Absorption flat daily \
                     pool (PF1 Core Rulebook Abjuration School): AbjurationEnergyAbsorption \
                     resolves to AbjurationSchoolLVL*3 = {abjuration_school_lvl}*3 = \
                     {energy_absorption}. This grounds only the flat daily-absorption-pool \
                     magnitude; it applies no reduction to any actual energy-damage roll, \
                     resolves no resistance-then-immunity-then-absorption ordering against a \
                     real damage instance, and tracks no daily pool depletion"
                ),
            });
        }
    }

    // `AT-34-E3-001` (mechanism 2 continuation, cycle 6): Wizard's
    // Transmutation arcane school, same shape as the Abjuration block above
    // -- `TransmutationSchoolLVL` <- `ArcaneSchoolLVL` <- `WizardLVL`, and
    // `TransmutationProgressionSchoolLVL` <- `ArcaneSchoolProgressionLVL` <-
    // `WizardLVL`, verified directly against `cr_abilities_class.lst`'s
    // `KEY:Transmutation School ~ *` records. Explanation ids live under the
    // shared `class_feature.school.transmutation.*` namespace, matching the
    // Abjuration precedent.
    if wizard_has_canonical_transmutation_selection(input) {
        let transmutation_school_lvl = wizard_level_value;
        let transmutation_progression_school_lvl = wizard_level_value;

        // Grounded for real: Telekinetic Fist (`KEY:Transmutation School ~
        // Telekinetic Fist`) -- two flat, non-dice `BONUS:VAR` formulas,
        // both unlocked from level 1
        // (`PREVARGTEQ:TransmutationProgressionSchoolLVL,1` on the
        // Transmutation School record's own `ABILITY:` grant line for
        // Telekinetic Fist).
        if transmutation_progression_school_lvl >= 1 {
            // `TransmutationTelekineticFistBonus|TransmutationSchoolLVL/2`.
            let telekinetic_fist_bonus = transmutation_school_lvl / 2;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.transmutation.telekinetic_fist_bonus".to_owned(),
                value: telekinetic_fist_bonus,
                detail: format!(
                    "Wizard level {level} Transmutation School power Telekinetic Fist \
                     bonus-damage magnitude (PF1 Core Rulebook Transmutation School): \
                     TransmutationTelekineticFistBonus resolves to \
                     TransmutationSchoolLVL/2 = {transmutation_school_lvl}/2 = \
                     {telekinetic_fist_bonus}, added to the base 1d4 bludgeoning damage. \
                     Grounds only the flat bonus-damage magnitude; it strikes no ranged \
                     touch attack and applies no bonus to any actual damage roll"
                ),
            });

            // `TransmutationTelekineticFistTimes|ArcaneSchoolPowerTimes` --
            // the same shared "3 + Intelligence modifier" idiom the
            // pre-existing Force Missile / Protective Ward groundings above
            // already use.
            let telekinetic_fist_times = (3 + ability_modifiers.intelligence).max(0);
            explanations.push(ComputationExplanation {
                id: "class_feature.school.transmutation.telekinetic_fist_uses_per_day"
                    .to_owned(),
                value: telekinetic_fist_times,
                detail: format!(
                    "Wizard level {level} Transmutation School power Telekinetic Fist \
                     uses-per-day pool (PF1 Core Rulebook Transmutation School): \
                     TransmutationTelekineticFistTimes resolves to the shared \
                     ArcaneSchoolPowerTimes counter, 3 + Intelligence modifier, floored at 0. \
                     At Intelligence modifier {} this is max(3 + {}, 0) = \
                     {telekinetic_fist_times}. Grounds only the flat daily-use count; it \
                     tracks no action economy or per-use consumption",
                    ability_modifiers.intelligence, ability_modifiers.intelligence
                ),
            });
        }

        // Grounded for real: Physical Enhancement (`KEY:Transmutation
        // School ~ Physical Enhancement`) -- `TransmutationPhysicalEnhancementBonus|
        // min(5,(TransmutationSchoolLVL/5)+1)`, unlocked from level 1
        // (`PREVARGTEQ:TransmutationProgressionSchoolLVL,1` on the
        // Transmutation School record's own `ABILITY:` grant line). The
        // three sub-choice records this power auto-grants
        // (`Physical Enhancement ~ Constitution` / `~ Dexterity` /
        // `~ Strength`) each apply this SAME magnitude to whichever one
        // physical ability score the player chose that day
        // (`TEMPBONUS:PC|STAT|<ABL>|TransmutationPhysicalEnhancementBonus|
        // TYPE=Enhancement`) -- the choice of WHICH stat is not modeled
        // (the same "standalone, not wired into ability_modifiers" idiom
        // Abjuration's Protective Ward deflection bonus above already
        // uses), but the shared bonus magnitude genuinely is, so all three
        // sub-choice records and the top-level power record ground on this
        // one explanation id.
        if transmutation_progression_school_lvl >= 1 {
            let physical_enhancement_bonus = (5).min((transmutation_school_lvl / 5) + 1);
            explanations.push(ComputationExplanation {
                id: "class_feature.school.transmutation.physical_enhancement_bonus".to_owned(),
                value: physical_enhancement_bonus,
                detail: format!(
                    "Wizard level {level} Transmutation School power Physical Enhancement \
                     flat magnitude (PF1 Core Rulebook Transmutation School): \
                     TransmutationPhysicalEnhancementBonus resolves to \
                     min(5,(TransmutationSchoolLVL/5)+1) = \
                     min(5,({transmutation_school_lvl}/5)+1) = {physical_enhancement_bonus}. \
                     Grounds only the flat enhancement-bonus magnitude, shared identically by \
                     the three ability-score sub-choice records this power grants; it applies \
                     no bonus to any actual ability score and tracks no daily \
                     ability-score-of-choice reselection"
                ),
            });
        }

        // Grounded for real: Change Shape (`KEY:Transmutation School ~
        // Change Shape`) -- `TransmutationChangeShapeRounds|
        // TransmutationSchoolLVL`, gated on the power's own level-8 unlock
        // (`PREVARLT:Wizard_CF_SchoolPower8,1` on the Change Shape record
        // itself).
        if transmutation_progression_school_lvl >= 8 {
            let change_shape_rounds = transmutation_school_lvl;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.transmutation.change_shape_rounds".to_owned(),
                value: change_shape_rounds,
                detail: format!(
                    "Wizard level {level} Transmutation School power Change Shape \
                     rounds-per-day pool (PF1 Core Rulebook Transmutation School): \
                     TransmutationChangeShapeRounds resolves to TransmutationSchoolLVL = \
                     {change_shape_rounds}. Grounds only the flat rounds-per-day magnitude; \
                     it changes no actual shape and implements no Beast Shape / Elemental \
                     Body effect"
                ),
            });
        }
    }

    // `AT-34-E3-001` (mechanism 2 continuation, cycle 7): Wizard's
    // Conjuration arcane school, same shape as the Transmutation block
    // above -- `ConjurationSchoolLVL` <- `ArcaneSchoolLVL` <- `WizardLVL`,
    // and `ConjurationProgressionSchoolLVL` <- `ArcaneSchoolProgressionLVL`
    // <- `WizardLVL`, verified directly against `cr_abilities_class.lst`'s
    // `KEY:Conjuration School ~ *` records. Explanation ids live under the
    // shared `class_feature.school.conjuration.*` namespace, matching the
    // Transmutation precedent.
    if wizard_has_canonical_conjuration_selection(input) {
        let conjuration_school_lvl = wizard_level_value;
        let conjuration_progression_school_lvl = wizard_level_value;

        // Grounded for real: Summoner's Charm (`KEY:Conjuration School ~
        // Summoner's Charm`) -- `ConjurationSummonersCharmBonus|
        // max(1,ConjurationSchoolLVL/2)`, unlocked from level 1
        // (`PREVARLT:Wizard_CF_SchoolPower1,1` on the Summoner's Charm
        // record itself).
        if conjuration_progression_school_lvl >= 1 {
            let summoners_charm_bonus = (1).max(conjuration_school_lvl / 2);
            explanations.push(ComputationExplanation {
                id: "class_feature.school.conjuration.summoners_charm_bonus".to_owned(),
                value: summoners_charm_bonus,
                detail: format!(
                    "Wizard level {level} Conjuration School power Summoner's Charm \
                     duration-increase magnitude (PF1 Core Rulebook Conjuration School): \
                     ConjurationSummonersCharmBonus resolves to \
                     max(1,ConjurationSchoolLVL/2) = max(1,{conjuration_school_lvl}/2) = \
                     {summoners_charm_bonus} rounds. Grounds only the flat duration-increase \
                     magnitude; it applies no bonus to any actual spell duration and tracks no \
                     permanent-summon-monster designation"
                ),
            });
        }

        // Grounded for real: Acid Dart (`KEY:Conjuration School ~ Acid
        // Dart`) -- two flat, non-dice `BONUS:VAR` formulas, both unlocked
        // from level 1 (`PREVARLT:Wizard_CF_SchoolPower1,1` on the Acid
        // Dart record itself).
        if conjuration_progression_school_lvl >= 1 {
            // `ConjurationAcidDartDamageBonus|ConjurationSchoolLVL/2`.
            let acid_dart_damage_bonus = conjuration_school_lvl / 2;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.conjuration.acid_dart_damage_bonus".to_owned(),
                value: acid_dart_damage_bonus,
                detail: format!(
                    "Wizard level {level} Conjuration School power Acid Dart bonus-damage \
                     magnitude (PF1 Core Rulebook Conjuration School): \
                     ConjurationAcidDartDamageBonus resolves to ConjurationSchoolLVL/2 = \
                     {conjuration_school_lvl}/2 = {acid_dart_damage_bonus}, added to the base \
                     1d6 acid damage. Grounds only the flat bonus-damage magnitude; it strikes \
                     no ranged touch attack and applies no bonus to any actual damage roll"
                ),
            });

            // `ConjurationAcidDartTimes|ArcaneSchoolPowerTimes` -- the
            // same shared "3 + Intelligence modifier" idiom the
            // pre-existing Force Missile / Telekinetic Fist groundings
            // above already use.
            let acid_dart_times = (3 + ability_modifiers.intelligence).max(0);
            explanations.push(ComputationExplanation {
                id: "class_feature.school.conjuration.acid_dart_uses_per_day".to_owned(),
                value: acid_dart_times,
                detail: format!(
                    "Wizard level {level} Conjuration School power Acid Dart uses-per-day \
                     pool (PF1 Core Rulebook Conjuration School): ConjurationAcidDartTimes \
                     resolves to the shared ArcaneSchoolPowerTimes counter, 3 + Intelligence \
                     modifier, floored at 0. At Intelligence modifier {} this is \
                     max(3 + {}, 0) = {acid_dart_times}. Grounds only the flat daily-use \
                     count; it tracks no action economy or per-use consumption",
                    ability_modifiers.intelligence, ability_modifiers.intelligence
                ),
            });
        }

        // Grounded for real: Dimensional Steps (`KEY:Conjuration School ~
        // Dimensional Steps`) -- `ConjurationDimensionalSteps|
        // ConjurationSchoolLVL*30`, gated on the power's own level-8
        // unlock (`PREVARLT:Wizard_CF_SchoolPower8,1` on the Dimensional
        // Steps record itself).
        if conjuration_progression_school_lvl >= 8 {
            let dimensional_steps_feet = conjuration_school_lvl * 30;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.conjuration.dimensional_steps_feet".to_owned(),
                value: dimensional_steps_feet,
                detail: format!(
                    "Wizard level {level} Conjuration School power Dimensional Steps \
                     feet-per-day pool (PF1 Core Rulebook Conjuration School): \
                     ConjurationDimensionalSteps resolves to ConjurationSchoolLVL*30 = \
                     {conjuration_school_lvl}*30 = {dimensional_steps_feet}. Grounds only the \
                     flat feet-per-day magnitude; it teleports no actual character and tracks \
                     no 5-foot-increment consumption"
                ),
            });
        }
    }

    // SD-34 wave 44 (`decisions.md §22`, Piece 2 item 1): Wizard's Necromancy
    // arcane school, same shape as the Abjuration/Transmutation/Conjuration
    // blocks above -- `NecromancySchoolLVL` <- `ArcaneSchoolLVL` <-
    // `WizardLVL`, and `NecromancyProgressionSchoolLVL` <-
    // `ArcaneSchoolProgressionLVL` <- `WizardLVL`, verified directly against
    // `cr_abilities_class.lst`'s `KEY:Necromancy School ~ *` records (and the
    // two further `Power Over Undead ~ *` channeling records the Power Over
    // Undead power itself grants -- see below). Explanation ids live under
    // the shared `class_feature.school.necromancy.*` namespace, matching the
    // other four schools' precedent.
    //
    // **Real audit correction, resolved by direct corpus read (not trusted
    // from the audit's own prose):** the audit that scoped this wave claimed
    // `core_rulebook:class_feature:power_over_undead_turn_undead`'s real
    // owner is Cleric ("Cleric already grounds channel_energy_dice/
    // channel_energy_uses_per_day, only the DC is missing"). Direct read of
    // `cr_abilities_class.lst` line 2681 (the exact corpus source line for
    // this unit) disproves that: `PowerOverUndeadTurnDC`'s own
    // `BONUS:VAR|PowerOverUndeadTurnDC|10+PowerOverUndeadLVL/2+CHA` sits on
    // a record whose `TYPE:WizardClassFeatures.SpecialAttack.Supernatural.
    // NecromancerChanneling` facet and whose `PowerOverUndeadLVL|
    // NecromancySchoolLVL` feed chain are both Wizard/Necromancy-School-only
    // -- there is no `ClericLVL` anywhere in this record's own token
    // closure. Cleric's OWN Channel Positive/Negative Energy DC (a real,
    // separate, still-open gap on the DISTINCT corpus units
    // `core_rulebook:class_feature:cleric_channel_positive_energy`/
    // `cleric_channel_negative_energy`) is left untouched by this wave --
    // conflating the two would have fabricated a Cleric-attributed
    // explanation for a Wizard record. The audit's formula-shape claim
    // (`10+level/2+CHA`, same idiom as `warpriest_channel_energy_dc`) is
    // still correct; only the owner and the level term (`NecromancySchoolLVL`,
    // not `ClericLVL`) were wrong.
    if wizard_has_canonical_necromancy_selection(input) {
        let necromancy_school_lvl = wizard_level_value;
        let necromancy_progression_school_lvl = wizard_level_value;

        // Grounded for real: Power Over Undead (`KEY:Necromancy School ~
        // Power Over Undead`) -- `PowerOverUndeadTimes|3+INT`, unlocked from
        // level 1 (`PREVARGTEQ:NecromancyProgressionSchoolLVL,1` on the
        // Necromancy School record's own `ABILITY:` grant line). The same
        // shared "3 + Intelligence modifier" idiom every other school's
        // uses-per-day pool above already uses.
        if necromancy_progression_school_lvl >= 1 {
            let power_over_undead_times = (3 + ability_modifiers.intelligence).max(0);
            explanations.push(ComputationExplanation {
                id: "class_feature.school.necromancy.power_over_undead_uses_per_day".to_owned(),
                value: power_over_undead_times,
                detail: format!(
                    "Wizard level {level} Necromancy School power Power Over Undead \
                     uses-per-day pool (PF1 Core Rulebook Necromancy School): \
                     PowerOverUndeadTimes resolves to 3 + Intelligence modifier, floored at 0. \
                     At Intelligence modifier {} this is max(3 + {}, 0) = \
                     {power_over_undead_times}. Grounds only the flat daily-use count; it \
                     channels no actual positive or negative energy and tracks no per-use \
                     consumption",
                    ability_modifiers.intelligence, ability_modifiers.intelligence
                ),
            });

            // Grounded for real: the two channeling sub-records Power Over
            // Undead itself grants (`KEY` absent on both -- their own display
            // name IS their key -- `Power Over Undead ~ Turn Undead` and
            // `Power Over Undead ~ Command Undead`, `cr_abilities_class.lst`
            // lines 2680-2681), both auto-granted unconditionally alongside
            // Power Over Undead (each carries its own unconditional
            // `ABILITY:FEAT|AUTOMATIC|<Turn Undead|Command Undead>`, no
            // further PRE-gate of its own), so both are gated on the
            // identical `necromancy_progression_school_lvl >= 1` threshold as
            // their parent.
            //
            // `PowerOverUndeadTurnDC|10+PowerOverUndeadLVL/2+CHA` --
            // `PowerOverUndeadLVL|NecromancySchoolLVL`, so this reduces to
            // the same "10+level/2+CHA" idiom `warpriest_channel_energy_dc`
            // already established, anchored to the Wizard's own Necromancy
            // School level.
            let power_over_undead_turn_dc =
                10 + necromancy_school_lvl / 2 + ability_modifiers.charisma;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.necromancy.power_over_undead_turn_dc".to_owned(),
                value: power_over_undead_turn_dc,
                detail: format!(
                    "Wizard level {level} Necromancy School power Power Over Undead ~ Turn \
                     Undead save DC (PF1 Core Rulebook Necromancy School): PowerOverUndeadTurnDC \
                     resolves to 10+PowerOverUndeadLVL/2+CHA, where PowerOverUndeadLVL = \
                     NecromancySchoolLVL = {necromancy_school_lvl}. At Charisma modifier {} this \
                     is 10+{necromancy_school_lvl}/2+{} = {power_over_undead_turn_dc}. Grounds \
                     only the flat save-DC magnitude; it channels no actual positive energy and \
                     turns no actual undead creature",
                    ability_modifiers.charisma, ability_modifiers.charisma
                ),
            });

            // `PowerOverUndeadCommandDC|10+PowerOverUndeadLVL/2+CHA` -- the
            // identical formula shape to Turn Undead's own DC above, on the
            // sibling `Power Over Undead ~ Command Undead` record.
            let power_over_undead_command_dc = power_over_undead_turn_dc;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.necromancy.power_over_undead_command_dc".to_owned(),
                value: power_over_undead_command_dc,
                detail: format!(
                    "Wizard level {level} Necromancy School power Power Over Undead ~ Command \
                     Undead save DC (PF1 Core Rulebook Necromancy School): \
                     PowerOverUndeadCommandDC resolves to 10+PowerOverUndeadLVL/2+CHA, where \
                     PowerOverUndeadLVL = NecromancySchoolLVL = {necromancy_school_lvl}. At \
                     Charisma modifier {} this is 10+{necromancy_school_lvl}/2+{} = \
                     {power_over_undead_command_dc}. Grounds only the flat save-DC magnitude; \
                     it channels no actual negative energy and commands no actual undead \
                     creature",
                    ability_modifiers.charisma, ability_modifiers.charisma
                ),
            });

            // `PowerOverUndeadCommandHD|PowerOverUndeadLVL` -- the bare
            // school level, no further arithmetic encoded in the corpus
            // formula itself.
            let power_over_undead_command_hd = necromancy_school_lvl;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.necromancy.power_over_undead_command_hd".to_owned(),
                value: power_over_undead_command_hd,
                detail: format!(
                    "Wizard level {level} Necromancy School power Power Over Undead ~ Command \
                     Undead hit-dice cap (PF1 Core Rulebook Necromancy School): \
                     PowerOverUndeadCommandHD resolves to PowerOverUndeadLVL = \
                     NecromancySchoolLVL = {power_over_undead_command_hd}. Grounds only the \
                     flat hit-dice-cap magnitude; it commands no actual undead creature"
                ),
            });
        }

        // Grounded for real: Grave Touch (`KEY:Necromancy School ~ Grave
        // Touch`) -- three flat, non-dice `BONUS:VAR` formulas, all unlocked
        // from level 1 (`PREVARGTEQ:NecromancyProgressionSchoolLVL,1` on the
        // Necromancy School record's own `ABILITY:` grant line for Grave
        // Touch).
        if necromancy_progression_school_lvl >= 1 {
            // `NecromancyGraveTouchDuration|max(1,NecromancySchoolLVL/2)`.
            let grave_touch_duration = (1).max(necromancy_school_lvl / 2);
            explanations.push(ComputationExplanation {
                id: "class_feature.school.necromancy.grave_touch_duration".to_owned(),
                value: grave_touch_duration,
                detail: format!(
                    "Wizard level {level} Necromancy School power Grave Touch shaken-duration \
                     magnitude (PF1 Core Rulebook Necromancy School): \
                     NecromancyGraveTouchDuration resolves to max(1,NecromancySchoolLVL/2) = \
                     max(1,{necromancy_school_lvl}/2) = {grave_touch_duration} rounds. Grounds \
                     only the flat round-count magnitude; it makes no actual melee touch attack \
                     and applies no shaken/frightened condition"
                ),
            });

            // `NecromancyGraveTouchLimit|NecromancySchoolLVL` -- the bare
            // school level, the Hit Dice threshold below which a shaken
            // target becomes frightened instead.
            let grave_touch_limit = necromancy_school_lvl;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.necromancy.grave_touch_limit".to_owned(),
                value: grave_touch_limit,
                detail: format!(
                    "Wizard level {level} Necromancy School power Grave Touch Hit-Dice \
                     threshold (PF1 Core Rulebook Necromancy School): NecromancyGraveTouchLimit \
                     resolves to NecromancySchoolLVL = {grave_touch_limit}. Grounds only the \
                     flat Hit-Dice-threshold magnitude; it applies no frightened condition to \
                     any actual target"
                ),
            });

            // `NecromancyGraveTouchTimes|ArcaneSchoolPowerTimes` -- the same
            // shared "3 + Intelligence modifier" idiom every other school's
            // uses-per-day pool above already uses.
            let grave_touch_times = (3 + ability_modifiers.intelligence).max(0);
            explanations.push(ComputationExplanation {
                id: "class_feature.school.necromancy.grave_touch_uses_per_day".to_owned(),
                value: grave_touch_times,
                detail: format!(
                    "Wizard level {level} Necromancy School power Grave Touch uses-per-day pool \
                     (PF1 Core Rulebook Necromancy School): NecromancyGraveTouchTimes resolves \
                     to the shared ArcaneSchoolPowerTimes counter, 3 + Intelligence modifier, \
                     floored at 0. At Intelligence modifier {} this is max(3 + {}, 0) = \
                     {grave_touch_times}. Grounds only the flat daily-use count; it tracks no \
                     action economy or per-use consumption",
                    ability_modifiers.intelligence, ability_modifiers.intelligence
                ),
            });
        }

        // Grounded for real: Life Sight (`KEY:Necromancy School ~ Life
        // Sight`), gated on the power's own level-8 unlock
        // (`PREVARGTEQ:NecromancyProgressionSchoolLVL,8` on the Necromancy
        // School record's own `ABILITY:` grant line for Life Sight).
        if necromancy_progression_school_lvl >= 8 {
            // `NecromancyLifeSightRange|10+10*((NecromancySchoolLVL-8)/4)`.
            let life_sight_range = 10 + 10 * ((necromancy_school_lvl - 8) / 4);
            explanations.push(ComputationExplanation {
                id: "class_feature.school.necromancy.life_sight_range".to_owned(),
                value: life_sight_range,
                detail: format!(
                    "Wizard level {level} Necromancy School power Life Sight blindsight-range \
                     magnitude in feet (PF1 Core Rulebook Necromancy School): \
                     NecromancyLifeSightRange resolves to \
                     10+10*((NecromancySchoolLVL-8)/4) = \
                     10+10*(({necromancy_school_lvl}-8)/4) = {life_sight_range}. Grounds only \
                     the flat range magnitude; it grants no actual blindsight sense"
                ),
            });

            // `NecromancyLifeSightRounds|NecromancySchoolLVL` -- the bare
            // school level, no further arithmetic encoded in the corpus
            // formula itself.
            let life_sight_rounds = necromancy_school_lvl;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.necromancy.life_sight_rounds".to_owned(),
                value: life_sight_rounds,
                detail: format!(
                    "Wizard level {level} Necromancy School power Life Sight rounds-per-day \
                     pool (PF1 Core Rulebook Necromancy School): NecromancyLifeSightRounds \
                     resolves to NecromancySchoolLVL = {life_sight_rounds}. Grounds only the \
                     flat rounds-per-day magnitude; it grants no actual blindsight sense and \
                     tracks no round-by-round duration"
                ),
            });
        }
    }

    // `AT-34-E3-001` (mechanism 2 continuation, cycle 8): Wizard's Universal
    // School (the "no specialization" arm) -- unlike every specialist block
    // above, `UniversalSchoolLVL` <- `ArcaneSchoolLVL` <- `WizardLVL` still
    // holds (`BONUS:VAR|UniversalSchoolLVL|ArcaneSchoolLVL` on the corpus's
    // own `Universal School` record), so the two power records below share
    // the same `wizard_level_value` chain every specialist school already
    // uses. Verified directly against `cr_abilities_class.lst`'s `KEY:
    // Universal School ~ *` records. Explanation ids live under the shared
    // `class_feature.school.universal.*` namespace.
    if wizard_has_canonical_universal_selection(input) {
        let universal_school_lvl = wizard_level_value;
        let universal_progression_school_lvl = wizard_level_value;

        // Grounded for real: Hand of the Apprentice (`KEY:Universal School
        // ~ Hand of the Apprentice`) -- `UniversalHandOfTheApprenticeTimes|
        // ArcaneSchoolPowerTimes`, the same shared "3 + Intelligence
        // modifier" idiom the pre-existing Force Missile / Telekinetic
        // Fist / Acid Dart groundings above already use, unlocked from
        // level 1 (`PREVARLT:Wizard_CF_SchoolPower1,1` on the Hand of the
        // Apprentice record itself).
        if universal_progression_school_lvl >= 1 {
            let hand_of_the_apprentice_times = (3 + ability_modifiers.intelligence).max(0);
            explanations.push(ComputationExplanation {
                id: "class_feature.school.universal.hand_of_the_apprentice_uses_per_day"
                    .to_owned(),
                value: hand_of_the_apprentice_times,
                detail: format!(
                    "Wizard level {level} Universal School power Hand of the Apprentice \
                     uses-per-day pool (PF1 Core Rulebook Universal School): \
                     UniversalHandOfTheApprenticeTimes resolves to the shared \
                     ArcaneSchoolPowerTimes counter, 3 + Intelligence modifier, floored at 0. \
                     At Intelligence modifier {} this is max(3 + {}, 0) = \
                     {hand_of_the_apprentice_times}. Grounds only the flat daily-use count; \
                     it makes no actual melee-weapon ranged attack and applies no bonus to \
                     any actual attack or damage roll",
                    ability_modifiers.intelligence, ability_modifiers.intelligence
                ),
            });
        }

        // Grounded for real: Metamagic Mastery (`KEY:Universal School ~
        // Metamagic Mastery`) -- `UniversalMetamagicMasteryTimes|
        // (UniversalSchoolLVL-8)/2+1`, gated on the power's own level-8
        // unlock (`PREVARLT:Wizard_CF_SchoolPower8,1` on the Metamagic
        // Mastery record itself).
        if universal_progression_school_lvl >= 8 {
            let metamagic_mastery_times = (universal_school_lvl - 8) / 2 + 1;
            explanations.push(ComputationExplanation {
                id: "class_feature.school.universal.metamagic_mastery_uses_per_day".to_owned(),
                value: metamagic_mastery_times,
                detail: format!(
                    "Wizard level {level} Universal School power Metamagic Mastery \
                     uses-per-day pool (PF1 Core Rulebook Universal School): \
                     UniversalMetamagicMasteryTimes resolves to (UniversalSchoolLVL-8)/2+1 = \
                     ({universal_school_lvl}-8)/2+1 = {metamagic_mastery_times}. Grounds only \
                     the flat daily-use count; it applies no actual metamagic feat to any \
                     spell and tracks no additional-daily-usage cost for a feat that raises \
                     the spell level by more than 1"
                ),
            });
        }
    }

    // SD-21 E6b.2/E6b.3: ground the real prepared-spellbook / daily-preparation
    // posture (bounded to wizard levels 1-3 and the canonical Evocation
    // specialization, `unmet_wizard_spellbook_conditions`'s own supported range),
    // including the opposed-school double-slot-cost rule. When the posture is
    // unmet, both diagnostics below still fire exactly as before (their message
    // now also cites the specific unmet reason); when it is met, this replaces
    // them with real spellbook-contents / daily-preparation / spells-per-day
    // explanation records instead.
    let spellbook_unmet =
        unmet_wizard_spellbook_conditions(input, level, ability_modifiers);
    if spellbook_unmet.is_empty() {
        ground_wizard_prepared_spellbook(input, level, ability_modifiers, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported"
                .to_owned(),
            message: format!(
                "Wizard level {level} remains blocked on its school-power execution and \
                 opposed-school preparation-cost burden: the Evocation intense spells flat \
                 bonus-damage magnitude and the force missile flat 3 + Int-mod uses-per-day pool \
                 are now grounded as flat numbers in dedicated explanation records, and the \
                 opposed-school preparation cost (each opposed-school spell occupies two \
                 prepared slots) is grounded for real once a supported spellbook posture exists, \
                 but no evocation spell-damage application and no force-missile casting \
                 execution (the 1d4 damage roll and automatic-hit targeting) are implemented, so \
                 no full Wizard school-power support is claimed; unmet spellbook posture: {}",
                spellbook_unmet.join("; ")
            ),
            claim_blocking: true,
        });

        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.wizard.prepared_spellbook.unsupported".to_owned(),
            message: format!(
                "Wizard remains blocked on its prepared spellbook / spells prepared / spell slot \
                 posture burden: {}",
                spellbook_unmet.join("; ")
            ),
            claim_blocking: true,
        });
    }
}

/// This grounding's own supported wizard-level ceiling (SD-21 E6b.2),
/// matching `MAX_SUPPORTED_WIZARD_LEVEL` (20) since the v0.6 widening: the
/// base spells-per-day table below originally shipped verified for levels
/// 1-3 only, and deferred levels 4-20 to "a later cycle". This is that
/// cycle -- the full 1-20 table is now transcribed literally from the
/// corpus (see `wizard_base_spells_per_day`), so no level in the legal
/// 1-20 range is refused for lack of a verified row any more.
pub(super) const WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL: u8 = 20;

/// PF1 Core Rulebook Wizard "Spells per Day" table, base counts before any
/// Intelligence bonus spells or the specialist bonus slot, one entry per
/// spell level 0 (cantrip) through 9 (`None` for an inaccessible "--"
/// column).
///
/// Transcribed literally from the PCGen corpus's own `CLASS:Wizard` level-
/// progression `CAST:` rows at
/// `data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst`
/// lines 303-322 (level 1 on line 303 through level 20 on line 322) -- a
/// literal table lookup, not a derived formula. Each `CAST:` row is a
/// comma-separated list starting at spell level 0, so `1 CAST:3,1` is
/// "3 cantrips / 1 first-level", and absent trailing columns are the
/// printed table's "--" (inaccessible) entries. The levels 1-3 rows are
/// byte-for-byte the ones this table already shipped with, previously
/// verified independently against d20pfsrd.com and the Archives of
/// Nethys aonprd.com mirror; the levels 4-20 rows are new in the v0.6
/// widening and come from the same corpus block.
///
/// Returns an all-`None` row outside the legal 1-20 class-level range.
pub(super) fn wizard_base_spells_per_day(level: u8) -> [Option<i16>; 10] {
    match level {
        1 => [Some(3), Some(1), None, None, None, None, None, None, None, None],
        2 => [Some(4), Some(2), None, None, None, None, None, None, None, None],
        3 => [Some(4), Some(2), Some(1), None, None, None, None, None, None, None],
        4 => [Some(4), Some(3), Some(2), None, None, None, None, None, None, None],
        5 => [Some(4), Some(3), Some(2), Some(1), None, None, None, None, None, None],
        6 => [Some(4), Some(3), Some(3), Some(2), None, None, None, None, None, None],
        7 => [Some(4), Some(4), Some(3), Some(2), Some(1), None, None, None, None, None],
        8 => [Some(4), Some(4), Some(3), Some(3), Some(2), None, None, None, None, None],
        9 => [Some(4), Some(4), Some(4), Some(3), Some(2), Some(1), None, None, None, None],
        10 => [Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None, None, None, None],
        11 => [Some(4), Some(4), Some(4), Some(4), Some(3), Some(2), Some(1), None, None, None],
        12 => [Some(4), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None, None, None],
        13 => [Some(4), Some(4), Some(4), Some(4), Some(4), Some(3), Some(2), Some(1), None, None],
        14 => [Some(4), Some(4), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None, None],
        15 => [
            Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(3), Some(2), Some(1), None,
        ],
        16 => [
            Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None,
        ],
        17 => [
            Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(3), Some(2),
            Some(1),
        ],
        18 => [
            Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(3), Some(3),
            Some(2),
        ],
        19 => [
            Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(3),
            Some(3),
        ],
        20 => [
            Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4),
            Some(4),
        ],
        _ => [None, None, None, None, None, None, None, None, None, None],
    }
}

/// Resolves a wizard spellbook entry's `(school, level)` identity.
///
/// v0.6 alpha swarm (real correctness bug, frontend-found): a real spell_id
/// from the actual spell catalog (e.g. `"Magic Missile"`, the literal
/// `SPELL_LIST` key `spell_catalog.rs` hands the frontend picker) has no
/// dots at all, so this used to always return `None` for every real spell
/// -- silently excluding it from `unmet_wizard_spellbook_conditions`'s slot-
/// budget consumption count. A Wizard could add unlimited real spells with
/// zero slot enforcement. Fixed by trying a real `SPELL_LIST` lookup FIRST:
/// unlike equipment (which needs a corpus-resolved `EquipmentRecord` via
/// `SourcePackageContent`), `SPELL_LIST` (`rules_tables::crb::spell_list`)
/// is a `pub const` compiled directly into the binary -- generated from the
/// corpus at build time, not loaded from external fixture files at runtime
/// -- so it is already fully accessible from this headless compute surface
/// with zero corpus threading, exactly like every `spellbook::*` resolver
/// already imports it. This is NOT the same headless-vs-corpus-aware
/// architecture wall that blocked the AC-widening and initially looked like
/// it might block encumbrance (see `risks-and-open-questions.md`) -- it
/// only looks similar on the surface; `SPELL_LIST` was reachable the whole
/// time.
///
/// Falls back to this bounded slice's original corpus-free synthetic
/// convention, `<school>.<level>.<name>` (e.g. `evocation.1.magic_missile`),
/// for any spell_id that isn't a real `SPELL_LIST` key -- zero blast radius
/// on existing fixtures/tests already built against that convention (the
/// same "compound fact encoded in one colon/dot-segmented identifier" idiom
/// used throughout this file, e.g. `feat:weapon_focus:weapon:longsword`).
/// Returns `None` only when neither resolution succeeds.
pub(super) fn parse_wizard_spellbook_spell_id(spell_id: &str) -> Option<(Pf1SchoolId, u8)> {
    if let Some(entry) = SPELL_LIST.iter().find(|entry| entry.key == spell_id) {
        return Some((entry.school, entry.level));
    }
    parse_synthetic_spell_id(spell_id)
}

/// The PF1 slot cost of preparing one spell of the given school for this
/// bounded slice's fixed canonical opposed schools (Necromancy and
/// Transmutation, the same pair `wizard_has_canonical_specialization_selections`
/// already requires): 2 prepared slots for an opposed-school spell, 1
/// otherwise (PF1 Core Rulebook arcane school class feature: "he must use two
/// of his daily spell slots of that level to prepare [an opposed-school]
/// spell").
pub(super) fn wizard_opposed_school_slot_cost(school: Pf1SchoolId) -> i16 {
    if matches!(school, Pf1SchoolId::Necromancy | Pf1SchoolId::Transmutation) {
        2
    } else {
        1
    }
}

/// Return the list of unmet conditions for this grounding's bounded prepared-
/// spellbook posture. An empty list means the posture is fully supported: a
/// canonical-specialization wizard at a supported level, with at least one
/// spell recorded (`AcquisitionMode::Known`) and at least one prepared today
/// (`AcquisitionMode::Prepared`), every prepared spell already recorded, and
/// no spell level's prepared consumption (opposed-school spells costing 2
/// slots each) exceeding that level's total slot budget (base table count +
/// the already-grounded specialist bonus slot + the Intelligence bonus).
///
/// This is a genuine, but deliberately bounded, slot-consumption check: it
/// does not separate the specialist bonus slot's own school restriction
/// (real PF1: that slot can only hold a spell of the specialized school
/// itself) from the generic per-level budget -- both are summed into one
/// total. For this slice's own canonical-Evocation-only reproducer that
/// nuance never bites (every non-opposed prepared spell here is Evocation),
/// and is named here as a known, documented limitation rather than silently
/// assumed away.
pub(super) fn unmet_wizard_spellbook_conditions(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Vec<String> {
    let mut unmet = Vec::new();

    if level > WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL {
        unmet.push(format!(
            "prepared spellbook grounding is only supported for wizard levels \
             1-{WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL}, got {level}"
        ));
        return unmet;
    }
    if !wizard_has_canonical_specialization_selections(input) {
        unmet.push(
            "prepared spellbook grounding requires the canonical Evocation specialization \
             (opposed Necromancy/Transmutation)"
                .to_owned(),
        );
        return unmet;
    }

    let wizard_spells = |mode: AcquisitionMode| {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(move |s| s.source_class_id == WIZARD_CLASS_ID && s.acquisition_mode == mode)
    };
    let recorded: Vec<&str> = wizard_spells(AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();
    let prepared: Vec<&str> = wizard_spells(AcquisitionMode::Prepared)
        .map(|s| s.spell_id.as_str())
        .collect();

    if recorded.is_empty() {
        unmet.push(
            "no wizard spells recorded in the spellbook (AcquisitionMode::Known)".to_owned(),
        );
    }
    if prepared.is_empty() {
        unmet.push("no wizard spells prepared today (AcquisitionMode::Prepared)".to_owned());
    }

    // The Known rule: membership only. See `class_spell_membership_refusal`
    // for why a recorded spell is NOT checked against the wizard's own
    // spell-level access ceiling the way a prepared one is.
    for spell_id in &recorded {
        if let Some(reason) = class_spell_membership_refusal(WIZARD_CLASS_ID, spell_id) {
            unmet.push(reason);
        }
    }

    for spell_id in &prepared {
        if !recorded.contains(spell_id) {
            unmet.push(format!(
                "prepared spell '{spell_id}' is not recorded in the spellbook"
            ));
        }
    }

    // Resolve every prepared spell's WIZARD-specific level up front, and
    // refuse outright any the corpus states no wizard level for — see
    // `resolve_prepared_spell_level`. Resolving here rather than inside the
    // per-spell-level loop is what makes an unresolvable spell a blocker
    // instead of a `filter_map` casualty.
    let mut prepared_levels: Vec<(&str, u8)> = Vec::new();
    for spell_id in &prepared {
        match resolve_prepared_spell_level(WIZARD_CLASS_ID, spell_id) {
            PreparedSpellLevel::Known(spell_level) => prepared_levels.push((spell_id, spell_level)),
            PreparedSpellLevel::Unknown(reason) => unmet.push(reason),
        }
    }

    let base_spells_per_day = wizard_base_spells_per_day(level);
    for (spell_level, base_count) in base_spells_per_day.iter().enumerate() {
        let spell_level = spell_level as u8;
        let Some(base_count) = base_count else {
            if prepared_levels.iter().any(|(_, l)| *l == spell_level) {
                unmet.push(format!(
                    "a prepared spell targets spell level {spell_level}, not yet accessible at \
                     wizard level {level}"
                ));
            }
            continue;
        };
        let specialist_bonus = if spell_level >= 1 { 1 } else { 0 };
        let int_bonus =
            ability_bonus_spells(ability_modifiers.intelligence, i16::from(spell_level));
        let total_slots = base_count + specialist_bonus + int_bonus;
        let consumed: i16 = prepared_levels
            .iter()
            .filter(|(_, l)| *l == spell_level)
            .map(|(spell_id, _)| {
                resolve_prepared_spell_school(spell_id)
                    .map(wizard_opposed_school_slot_cost)
                    .unwrap_or(1)
            })
            .sum();
        if consumed > total_slots {
            unmet.push(format!(
                "spell level {spell_level} over-prepared: {consumed} slots consumed \
                 (opposed-school spells cost 2 each) but only {total_slots} available (base \
                 {base_count} + specialist bonus {specialist_bonus} + Intelligence bonus \
                 {int_bonus})"
            ));
        }
    }

    unmet
}

/// Ground the real prepared-spellbook / daily-preparation state once
/// `unmet_wizard_spellbook_conditions` reports an empty unmet list: the
/// recorded spellbook contents, the daily preparation selection, and the
/// base/Intelligence-bonus/total spells-per-day counts per accessible spell
/// level (mirroring the Paladin partial-caster base/bonus/total precedent
/// already grounded elsewhere in this file).
pub(super) fn ground_wizard_prepared_spellbook(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let wizard_spells = |mode: AcquisitionMode| {
        input
            .chosen
            .spells_selected
            .iter()
            .filter(move |s| s.source_class_id == WIZARD_CLASS_ID && s.acquisition_mode == mode)
    };
    let recorded: Vec<&str> = wizard_spells(AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();
    let prepared: Vec<&str> = wizard_spells(AcquisitionMode::Prepared)
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.wizard.spellbook_contents".to_owned(),
        value: recorded.len() as i16,
        detail: format!(
            "Wizard level {level} recorded spellbook contents ({} spells, \
             AcquisitionMode::Known): {}. This grounds which spells are recorded as real, \
             chosen input; it does not verify against any corpus that a named spell genuinely \
             exists or genuinely belongs to the school/level its own identifier claims",
            recorded.len(),
            recorded.join(", ")
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_spell.wizard.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Wizard level {level} daily preparation selection ({} spells, \
             AcquisitionMode::Prepared, each already verified recorded in the spellbook above): \
             {}. This grounds the prepared-vs-known distinction for real: every prepared spell \
             is drawn from the recorded spellbook, consuming its spell level's slot budget \
             (opposed-school spells costing 2 slots instead of 1). It computes no spell save DC \
             and no casting execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let base_spells_per_day = wizard_base_spells_per_day(level);
    for (spell_level, base_count) in base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = spell_level as u8;
        let specialist_bonus = if spell_level >= 1 { 1 } else { 0 };
        let int_bonus =
            ability_bonus_spells(ability_modifiers.intelligence, i16::from(spell_level));
        let total = base_count + specialist_bonus + int_bonus;

        explanations.push(ComputationExplanation {
            id: format!("class_spell.wizard.base_spells_per_day.spell_level_{spell_level}"),
            value: *base_count,
            detail: format!(
                "Wizard level {level} base spells per day at spell level {spell_level}: \
                 {base_count}, read directly from the PF1 Core Rulebook Wizard class table's \
                 spells-per-day row (verified against the raw table rows of both primary \
                 sources; a literal table lookup, not a derived formula)"
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!(
                "class_spell.wizard.intelligence_bonus_spells_per_day.spell_level_{spell_level}"
            ),
            value: int_bonus,
            detail: format!(
                "Wizard level {level} Intelligence bonus spells per day at spell level \
                 {spell_level}: {int_bonus} from Intelligence modifier \
                 {} (PF1 Core Rulebook Table: Ability Modifiers and Bonus Spells)",
                ability_modifiers.intelligence
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!("class_spell.wizard.total_spells_per_day.spell_level_{spell_level}"),
            value: total,
            detail: format!(
                "Wizard level {level} total spells per day at spell level {spell_level}: base \
                 {base_count} + specialist bonus slot {specialist_bonus} (Evocation-only, \
                 already grounded above as `class_chassis.wizard.specialist_bonus_slot`'s own \
                 flat count, decomposed here per spell level) + Intelligence bonus {int_bonus} \
                 = {total}"
            ),
        });
    }
}

/// v0.6 alpha swarm: QA found no `wizard_spell_save_dc` computation anywhere
/// in this file, unlike Paladin/Ranger/Sorcerer/Bard which all have the
/// standard `10 + spell_level + ability_modifier` formula grounded with
/// their own `tests/sd13_*_spell_save_dcs.rs` catalogue files. Inline here
/// (rather than a new `tests/sd13_wizard_spell_save_dcs.rs`) for the same
/// reason as `multiclass_bab_save_stacking_generalization_tests` above:
/// `tests/**` is the QA teammate's owned surface for this swarm; this uses
/// the exact same public entry point (`compute_pilot_base_chassis`) so it is
/// trivially portable into that file's convention once QA reviews and
/// adopts it into the catalogue.
#[cfg(test)]
mod wizard_spell_save_dc_tests {
    use super::{compute_pilot_base_chassis, WIZARD_CLASS_ID};
    use crate::rules_core::character_input::{load_character_input_fixture, CharacterInput};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// Reuses the Human Fighter level-1 fixture's ability scores (Intelligence
    /// 10, no human ability bonus applied to it -- the fixture's own
    /// `choice:human_ability_bonus` targets Strength) but swaps the single
    /// class entry to Wizard at the requested level, mirroring
    /// `multiclass_bab_save_stacking_generalization_tests::multiclass`'s
    /// override-in-place shape for the single-class case.
    fn wizard_at_level(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(
            result.diagnostics.is_empty(),
            "fixture should load cleanly: {:?}",
            result.diagnostics
        );
        let mut input = result
            .character_input
            .expect("valid fixture should produce a character input record");
        input.chosen.class_levels[0].class_id = WIZARD_CLASS_ID.to_owned();
        input.chosen.class_levels[0].level = level;
        input
    }

    /// Intelligence 10 (fixture default, human bonus goes to Strength) means
    /// modifier 0, so the level-1 DC is the bare `10 + spell_level` formula
    /// with nothing added -- a wizard at level 1 only has spell-level-access
    /// 1, so exactly one record should exist.
    #[test]
    fn wizard_level1_spell_save_dc_is_ten_plus_spell_level_plus_zero_intelligence_modifier() {
        let input = wizard_at_level(1);
        let computation = compute_pilot_base_chassis(&input);

        let record = computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.wizard.spell_save_dc.spell_level_1")
            .expect("level-1 wizard should ground a spell_level_1 save DC record");
        assert_eq!(record.value, 11, "10 + 1 + 0 = 11: {computation:?}");

        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.wizard.spell_save_dc.spell_level_2"),
            "a level-1 wizard has no 2nd-level spell access, so no spell_level_2 DC record \
             should exist: {computation:?}"
        );
    }

    /// At wizard level 3 (`WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL`),
    /// 2nd-level spell access opens up, so both spell_level_1 and
    /// spell_level_2 DC records should exist, still with a +0 Intelligence
    /// modifier from the shared fixture.
    #[test]
    fn wizard_level3_grounds_both_spell_level_1_and_2_save_dcs() {
        let input = wizard_at_level(3);
        let computation = compute_pilot_base_chassis(&input);

        let level_1 = computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.wizard.spell_save_dc.spell_level_1")
            .expect("level-3 wizard should still ground a spell_level_1 save DC record");
        assert_eq!(level_1.value, 11, "10 + 1 + 0 = 11: {computation:?}");

        let level_2 = computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.wizard.spell_save_dc.spell_level_2")
            .expect("level-3 wizard should ground a spell_level_2 save DC record");
        assert_eq!(level_2.value, 12, "10 + 2 + 0 = 12: {computation:?}");

        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.wizard.spell_save_dc.spell_level_3"),
            "a level-3 wizard has no 3rd-level spell access yet \
             (WIZARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL is 5), so no spell_level_3 DC \
             record should exist: {computation:?}"
        );
    }

    /// The DC formula is unconditional on school specialization: this
    /// fixture never sets the canonical `choice:wizard_school_specialization`
    /// selections, so `wizard_has_canonical_specialization_selections` is
    /// false for it -- yet the save DC record must still be grounded,
    /// proving it sits outside that gate (unlike the specialist bonus slot,
    /// Intense Spells, and Force Missile records, which are correctly absent
    /// here).
    #[test]
    fn wizard_spell_save_dc_is_grounded_even_without_the_canonical_specialization_choice() {
        let input = wizard_at_level(1);
        let computation = compute_pilot_base_chassis(&input);

        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.wizard.specialist_bonus_slot"),
            "this fixture never sets the canonical specialization choice, so the \
             specialization-gated specialist bonus slot record must be absent: {computation:?}"
        );
        assert!(
            computation
                .explanations
                .iter()
                .any(|e| e.id == "class_chassis.wizard.spell_save_dc.spell_level_1"),
            "the spell save DC record must be grounded regardless of school specialization: \
             {computation:?}"
        );
    }
}

/// task #88 correction: Protective Ward's own detail strings used to claim
/// "applies no [deflection] bonus to any actual AC total" (authored
/// 2026-07-28, task #66) -- false: Wizard is one of the two original
/// dispatch-supported chassis (alongside Fighter), so a GE-06-posture
/// Wizard has always reached the real `defense.baseline_armor_class`
/// pillar via `compute_combat_baseline`. Protective Ward's deflection
/// bonus simply isn't wired into it (correctly still true -- it also
/// requires area-of-effect ally targeting this codebase doesn't model).
#[cfg(test)]
mod wizard_abjuration_protective_ward_ac_claim_tests {
    use super::{compute_pilot_base_chassis, WIZARD_CLASS_ID};
    use crate::rules_core::character_input::{
        load_character_input_fixture, CharacterInput, SelectedChoice,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// A GE-06-posture (Longsword/Chain Shirt/Dodge/Weapon Focus/no shield)
    /// Human Wizard at level 6 (past Protective Ward's level-1 unlock,
    /// giving a real nonzero deflection bonus of (6/5)+1=2), with the
    /// canonical Abjuration school + opposed-schools selection.
    fn abjuration_wizard_with_pilot_posture(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels[0].class_id = WIZARD_CLASS_ID.to_owned();
        input.chosen.class_levels[0].level = level;
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:wizard_school_specialization".to_owned(),
            selection_id: "school:abjuration".to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:wizard_opposed_schools".to_owned(),
            selection_id: "school:necromancy".to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:wizard_opposed_schools".to_owned(),
            selection_id: "school:transmutation".to_owned(),
        });
        input
    }

    #[test]
    fn protective_ward_detail_no_longer_falsely_claims_no_ac_total_exists() {
        let computation =
            compute_pilot_base_chassis(&abjuration_wizard_with_pilot_posture(6));

        let deflection = computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.school.abjuration.protective_ward_deflection_bonus")
            .expect("Protective Ward's deflection bonus must ground at level 6");
        assert_eq!(deflection.value, 2, "level 6 Protective Ward deflection: (6/5)+1=2: {:?}", deflection);
        assert!(
            !deflection.detail.contains("applies no bonus to any actual AC total"),
            "the corrected detail must not repeat the false no-total-exists claim: {:?}",
            deflection
        );
        assert!(
            deflection.detail.contains("defense.baseline_armor_class"),
            "the corrected detail must name the real AC total it isn't wired into: {:?}",
            deflection
        );

        let baseline_ac = computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .expect(
                "a GE-06-posture Wizard is one of the two original supported chassis, so \
                 baseline AC must be real, not absent",
            );
        assert!(
            !baseline_ac.detail.contains("Protective Ward"),
            "Protective Ward's +2 must NOT be folded into baseline AC yet -- the corrected \
             claim says 'not wired in', not 'wired in': {:?}",
            baseline_ac
        );
    }
}

/// SD-34 wave 44 (`decisions.md §22`, Piece 2 item 1): direct formula tests
/// for Wizard's Necromancy arcane school, the fifth school this bundle adds
/// (mirroring Abjuration/Transmutation/Conjuration/Universal above), plus
/// the two `Power Over Undead ~ *` channeling records the school's own
/// Power Over Undead power grants. Same pipeline-level test shape as
/// `wizard_abjuration_protective_ward_ac_claim_tests` immediately above:
/// every one of the five schools' own formulas is inlined directly in
/// `compute_pilot_base_chassis` rather than extracted into a standalone pure
/// function, so "direct formula test" here means through the real
/// `compute_pilot_base_chassis` entry point at specific levels, not a
/// standalone function call -- exactly the existing precedent this file
/// already established for the other four schools.
#[cfg(test)]
mod wave44_necromancy_school_new_compute_tests {
    use super::{compute_pilot_base_chassis, WIZARD_CLASS_ID};
    use crate::rules_core::character_input::{
        load_character_input_fixture, CharacterInput, SelectedChoice,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// The fixture's own ability scores give an Intelligence modifier of 0
    /// (score 10, the Human bonus goes to Strength) and a Charisma modifier
    /// of -1 (score 8) -- both real, non-hardcoded inputs, so asserting
    /// against them proves the ability-modifier terms below are read from
    /// the character, not constants baked into the formula.
    const FIXTURE_INTELLIGENCE_MODIFIER: i16 = 0;
    const FIXTURE_CHARISMA_MODIFIER: i16 = -1;

    /// A GE-06-posture Human Wizard at `level`, with the canonical
    /// Necromancy school + Abjuration/Conjuration opposed-schools selection
    /// -- `wizard_has_canonical_necromancy_selection`'s own precondition.
    fn necromancy_wizard(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels[0].class_id = WIZARD_CLASS_ID.to_owned();
        input.chosen.class_levels[0].level = level;
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:wizard_school_specialization".to_owned(),
            selection_id: "school:necromancy".to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:wizard_opposed_schools".to_owned(),
            selection_id: "school:abjuration".to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:wizard_opposed_schools".to_owned(),
            selection_id: "school:conjuration".to_owned(),
        });
        input
    }

    fn value_of(computation: &super::PilotBaseChassisComputation, id: &str) -> Option<i16> {
        computation.explanations.iter().find(|e| e.id == id).map(|e| e.value)
    }

    /// Power Over Undead's own uses-per-day pool: `3+INT`, unlocked from
    /// level 1 (`PREVARGTEQ:NecromancyProgressionSchoolLVL,1`). At the
    /// fixture's Intelligence modifier 0 this is exactly 3.
    #[test]
    fn power_over_undead_uses_per_day_matches_the_corpus_token() {
        let computation = compute_pilot_base_chassis(&necromancy_wizard(1));
        assert_eq!(
            value_of(&computation, "class_feature.school.necromancy.power_over_undead_uses_per_day"),
            Some(3 + FIXTURE_INTELLIGENCE_MODIFIER),
            "{:?}",
            computation
        );
    }

    /// Turn Undead's / Command Undead's save DC: `10+PowerOverUndeadLVL/2+CHA`
    /// where `PowerOverUndeadLVL = NecromancySchoolLVL` (the Wizard's own
    /// level, verified directly against `cr_abilities_class.lst` lines
    /// 2680-2681 -- NOT Cleric's `ClericLVL`, the audit's own mistaken
    /// premise for this unit). At level 7, Charisma modifier -1: 10+7/2-1 =
    /// 10+3-1 = 12.
    #[test]
    fn power_over_undead_turn_and_command_dc_match_the_corpus_token() {
        let computation = compute_pilot_base_chassis(&necromancy_wizard(7));
        let expected = 10 + 7 / 2 + FIXTURE_CHARISMA_MODIFIER;
        assert_eq!(
            value_of(&computation, "class_feature.school.necromancy.power_over_undead_turn_dc"),
            Some(expected),
            "{:?}",
            computation
        );
        assert_eq!(
            value_of(&computation, "class_feature.school.necromancy.power_over_undead_command_dc"),
            Some(expected),
            "Command Undead's DC must equal Turn Undead's -- the identical corpus formula \
             shape on the sibling record: {:?}",
            computation
        );
    }

    /// Command Undead's HD cap: the bare `PowerOverUndeadLVL`
    /// (= `NecromancySchoolLVL` = Wizard level), no further arithmetic.
    #[test]
    fn power_over_undead_command_hd_is_the_bare_school_level() {
        let computation = compute_pilot_base_chassis(&necromancy_wizard(9));
        assert_eq!(
            value_of(&computation, "class_feature.school.necromancy.power_over_undead_command_hd"),
            Some(9),
            "{:?}",
            computation
        );
    }

    /// Grave Touch: `max(1,NecromancySchoolLVL/2)` duration,
    /// `NecromancySchoolLVL` HD limit, shared `3+INT` uses-per-day pool. At
    /// level 1 the duration formula's own floor (`max(1, ...)`) is exercised
    /// (1/2 floors to 0, raised to 1 by the max).
    #[test]
    fn grave_touch_formulas_match_the_corpus_tokens() {
        let level1 = compute_pilot_base_chassis(&necromancy_wizard(1));
        assert_eq!(
            value_of(&level1, "class_feature.school.necromancy.grave_touch_duration"),
            Some(1),
            "level 1: max(1, 1/2) = max(1,0) = 1: {:?}",
            level1
        );
        assert_eq!(
            value_of(&level1, "class_feature.school.necromancy.grave_touch_limit"),
            Some(1),
            "{:?}",
            level1
        );
        assert_eq!(
            value_of(&level1, "class_feature.school.necromancy.grave_touch_uses_per_day"),
            Some(3 + FIXTURE_INTELLIGENCE_MODIFIER),
            "{:?}",
            level1
        );

        let level6 = compute_pilot_base_chassis(&necromancy_wizard(6));
        assert_eq!(
            value_of(&level6, "class_feature.school.necromancy.grave_touch_duration"),
            Some(3),
            "level 6: max(1, 6/2) = 3: {:?}",
            level6
        );
        assert_eq!(
            value_of(&level6, "class_feature.school.necromancy.grave_touch_limit"),
            Some(6),
            "{:?}",
            level6
        );
    }

    /// Life Sight is gated at `NecromancyProgressionSchoolLVL,8` -- absent
    /// below level 8, present from level 8 on with
    /// `10+10*((NecromancySchoolLVL-8)/4)` range and the bare level as its
    /// rounds-per-day pool.
    #[test]
    fn life_sight_is_gated_at_level_8_and_matches_the_corpus_tokens() {
        let level7 = compute_pilot_base_chassis(&necromancy_wizard(7));
        assert_eq!(
            value_of(&level7, "class_feature.school.necromancy.life_sight_range"),
            None,
            "Life Sight must not ground below level 8: {:?}",
            level7
        );

        let level8 = compute_pilot_base_chassis(&necromancy_wizard(8));
        assert_eq!(
            value_of(&level8, "class_feature.school.necromancy.life_sight_range"),
            Some(10),
            "level 8: 10+10*((8-8)/4) = 10+10*0 = 10: {:?}",
            level8
        );
        assert_eq!(
            value_of(&level8, "class_feature.school.necromancy.life_sight_rounds"),
            Some(8),
            "{:?}",
            level8
        );

        let level12 = compute_pilot_base_chassis(&necromancy_wizard(12));
        assert_eq!(
            value_of(&level12, "class_feature.school.necromancy.life_sight_range"),
            Some(20),
            "level 12: 10+10*((12-8)/4) = 10+10*1 = 20: {:?}",
            level12
        );
    }

    /// None of the five new explanation ids fires for a Wizard who selected
    /// a DIFFERENT specialty school -- the canonical-selection gate must
    /// actually gate, not fire unconditionally for any Wizard.
    #[test]
    fn necromancy_explanations_are_absent_for_a_different_specialist() {
        let mut input = necromancy_wizard(12);
        input.chosen.selected_choices.retain(|c| {
            c.choice_set_id != "choice:wizard_school_specialization"
                && c.choice_set_id != "choice:wizard_opposed_schools"
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:wizard_school_specialization".to_owned(),
            selection_id: "school:evocation".to_owned(),
        });
        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.school.necromancy.")),
            "an Evocation specialist must ground no Necromancy school explanation: {:?}",
            computation
        );
    }
}

/// v0.6 alpha swarm item 18 widening (2026-07-24, operator-directed):
/// `explain_wizard_level1_prepared_spell_baseline`'s race gate was removed
/// after tracing every formula it grounds -- none reads `race_id` directly,
/// `ability_modifiers` already arrives race-adjusted from
/// `apply_human_ability_bonus`. Proves this empirically end to end for a
/// non-Human (Elf) Wizard, through the real `build_pilot_headless_receipt`
/// entry point, not just a direct call on the narrower
/// `compute_pilot_base_chassis`: a correct, racially-adjusted spell-save-DC
/// at level 1, and the real `WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL` ceiling
/// correctly firing past level 3 -- both previously entirely absent for any
/// non-Human Wizard (risks-and-open-questions.md item 18).
#[cfg(test)]
mod wizard_non_human_widening_tests {
    use super::{
        build_pilot_headless_receipt, EVOCATION_SCHOOL_SELECTION, NECROMANCY_SCHOOL_SELECTION,
        TRANSMUTATION_SCHOOL_SELECTION, WIZARD_CLASS_ID, WIZARD_OPPOSED_SCHOOLS_CHOICE_ID,
        WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, AcquisitionMode, CharacterInput, SelectedChoice,
        SpellSelection,
    };
    use crate::rules_core::pilot_compute::HeadlessReceiptStatus;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// Builds an Elf Wizard at `level`, with `intelligence` already reflecting
    /// Elf's real +2 Intelligence racial adjustment (base 10 + 2 = 12) --
    /// `apply_human_ability_bonus` returns a non-Human race's submitted score
    /// unchanged, so a non-Human fixture must submit the already-adjusted
    /// value (the same contract risks-and-open-questions.md item 19's fix
    /// established for the create-character submission path). Seeds the
    /// canonical school-specialization choices and one real spell (both
    /// Known and Prepared) exactly like a real Wizard's create/level-up path
    /// does, so the full pipeline can reach genuine `Computed`, not just
    /// exercise the one function in isolation.
    fn elf_wizard_at_level(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly: {:?}", result.diagnostics);
        let mut input = result
            .character_input
            .expect("valid fixture should produce a character input record");

        input.chosen.race_id = "race:elf".to_owned();
        input.chosen.ability_scores.intelligence = 12;
        input.chosen.class_levels[0].class_id = WIZARD_CLASS_ID.to_owned();
        input.chosen.class_levels[0].level = level;

        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID.to_owned(),
            selection_id: EVOCATION_SCHOOL_SELECTION.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WIZARD_OPPOSED_SCHOOLS_CHOICE_ID.to_owned(),
            selection_id: NECROMANCY_SCHOOL_SELECTION.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: WIZARD_OPPOSED_SCHOOLS_CHOICE_ID.to_owned(),
            selection_id: TRANSMUTATION_SCHOOL_SELECTION.to_owned(),
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: WIZARD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: WIZARD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        input
    }

    #[test]
    fn elf_wizard_level1_reaches_computed_with_the_real_racially_adjusted_spell_save_dc() {
        let input = elf_wizard_at_level(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "an Elf Wizard level 1 with the canonical specialization and a real prepared \
             spell must reach Computed, got: {:?}",
            receipt.computation.diagnostics
        );

        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.wizard.spell_save_dc.spell_level_1")
            .expect("an Elf Wizard must ground a spell_level_1 save DC record, not skip it entirely");
        assert_eq!(
            record.value, 12,
            "10 + spell level 1 + Intelligence modifier +1 (score 12, Elf's real +2 INT \
             already applied) = 12, not the Human-only value of 11: {record:?}"
        );
    }

    /// Negative control: a Human Wizard built the identical way (same
    /// spells/choices, `race_id` and `intelligence` reverted) must still get
    /// the pre-existing, unmodified value -- proves the widening didn't
    /// accidentally change the Human-path formula or double-apply a racial
    /// bonus.
    #[test]
    fn human_wizard_level1_still_gets_the_unmodified_spell_save_dc() {
        let mut input = elf_wizard_at_level(1);
        input.chosen.race_id = "race:human".to_owned();
        input.chosen.ability_scores.intelligence = 10;

        let receipt = build_pilot_headless_receipt(&input);
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.wizard.spell_save_dc.spell_level_1")
            .expect("Human Wizard must still ground the record");
        assert_eq!(record.value, 11, "10 + 1 + 0 = 11, unchanged from before this widening");
    }

    /// The real `WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL`-gated spellbook
    /// grounding must run for a non-Human Wizard too, not just Human -- this
    /// is the exact gap item 18 identified (the whole gated function never
    /// ran at all for non-Human before that widening).
    ///
    /// This test originally asserted that an Elf Wizard at level 4 was
    /// honestly `Blocked` on the old bounded 1-3 ceiling. The v0.6
    /// spellcasting widening transcribed the real levels 4-20 rows from the
    /// corpus and lifted that ceiling to 20, so level 4 now legitimately
    /// reaches `Computed`. The non-Human coverage the test exists to protect
    /// is preserved and strengthened: rather than merely asserting "no longer
    /// blocked", it pins the real corpus spells-per-day values that the
    /// non-Human path must actually produce, so a non-Human Wizard silently
    /// grounding nothing (the original item-18 bug shape) still fails here.
    #[test]
    fn elf_wizard_level4_grounds_the_real_widened_spellbook_just_like_human() {
        let input = elf_wizard_at_level(4);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "an Elf Wizard at level 4 is now inside the widened spellbook ceiling and must \
             reach Computed: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported"),
            "the spellbook-ceiling diagnostic must no longer fire at level 4: {:?}",
            receipt.computation.diagnostics
        );

        // cr_classes.lst:306 -- `4 CAST:4,3,2`. Fixture Intelligence 12 (+1),
        // so only the base row is asserted here; the Intelligence bonus and
        // the specialist slot are separate records.
        for (spell_level, expected) in [(0i16, 4i16), (1, 3), (2, 2)] {
            let id = format!("class_spell.wizard.base_spells_per_day.spell_level_{spell_level}");
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("a non-Human Wizard must ground {id}"));
            assert_eq!(
                record.value, expected,
                "Wizard level 4 base spells per day at spell level {spell_level} \
                 (cr_classes.lst:306): {record:?}"
            );
        }
    }

    /// The same non-Human path must also carry all the way to the top of the
    /// widened range: `cr_classes.lst:322` -- `20 CAST:4,4,4,4,4,4,4,4,4,4`,
    /// including the 9th-level column that only exists at level 20.
    #[test]
    fn elf_wizard_level20_grounds_the_full_corpus_spells_per_day_row() {
        let input = elf_wizard_at_level(20);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "an Elf Wizard at level 20 must reach Computed after the widening: {:?}",
            receipt.computation.diagnostics
        );
        for spell_level in 0..=9i16 {
            let id = format!("class_spell.wizard.base_spells_per_day.spell_level_{spell_level}");
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("Wizard level 20 must ground {id}"));
            assert_eq!(
                record.value, 4,
                "Wizard level 20 base spells per day is 4 at every spell level 0-9 \
                 (cr_classes.lst:322): {record:?}"
            );
        }
    }

    /// The ceiling must NOT fire for an Elf Wizard at level 3 (still within
    /// the supported range) -- confirms the gate is a genuine level
    /// comparison, not something that was accidentally always-Blocked once
    /// widened.
    #[test]
    fn elf_wizard_level3_stays_within_the_real_spellbook_ceiling() {
        let input = elf_wizard_at_level(3);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "an Elf Wizard level 3 is still within the real supported ceiling and must reach \
             Computed: {:?}",
            receipt.computation.diagnostics
        );
    }
}

/// v0.6 alpha swarm: QA/frontend found `parse_wizard_spellbook_spell_id`
/// only ever recognized the synthetic `<school>.<level>.<name>` convention,
/// so every REAL spell_id from the actual catalog (e.g. `"Magic Missile"`,
/// no dots) silently failed to resolve and was dropped from
/// `unmet_wizard_spellbook_conditions`'s slot-budget consumption count -- a
/// Wizard could add unlimited real spells with zero slot enforcement. This
/// covers the resolver fix in isolation (real SPELL_LIST keys resolve, the
/// synthetic fallback still works); the live-verified reproduction of the
/// actual bug through the real command surface lives in
/// `pf1_adapter.rs`'s inline tests instead, per the same
/// `tests/**`-is-QA's-owned-surface convention as this file's other inline
/// modules.
#[cfg(test)]
mod wizard_spellbook_spell_id_resolution_tests {
    use super::{parse_wizard_spellbook_spell_id, Pf1SchoolId};

    #[test]
    fn resolves_real_spell_list_keys_directly() {
        assert_eq!(
            parse_wizard_spellbook_spell_id("Magic Missile"),
            Some((Pf1SchoolId::Evocation, 1))
        );
        assert_eq!(
            parse_wizard_spellbook_spell_id("Alarm"),
            Some((Pf1SchoolId::Abjuration, 1))
        );
        assert_eq!(
            parse_wizard_spellbook_spell_id("Grease"),
            Some((Pf1SchoolId::Conjuration, 1))
        );
        assert_eq!(
            parse_wizard_spellbook_spell_id("Light"),
            Some((Pf1SchoolId::Evocation, 0))
        );
    }

    #[test]
    fn falls_back_to_the_synthetic_dotted_convention_for_non_catalog_ids() {
        assert_eq!(
            parse_wizard_spellbook_spell_id("evocation.1.magic_missile"),
            Some((Pf1SchoolId::Evocation, 1)),
            "existing fixtures built against the pre-fix synthetic convention must keep \
             resolving unchanged"
        );
        assert_eq!(parse_wizard_spellbook_spell_id("not.a.real.spell.id.at.all"), None);
        assert_eq!(parse_wizard_spellbook_spell_id("no dots and not a catalog key"), None);
    }
}

/// v0.6 alpha swarm, risks item 8, fifth slice (2026-07-25): Sorcerer's
/// real known-spell posture, mirroring `ranger_dispatch_widening_safety_tests`/
/// `paladin_dispatch_widening_safety_tests` exactly (same gate-ordering
/// structural risk existed here too and was fixed proactively as part of
/// this same slice). Unlike Ranger/Paladin, Sorcerer's bloodline-power
/// burden stays permanently unconditional (mirrors APG/ACG's own
/// `class_feature.<book>.<class>.unsupported` shape), so a single-class
/// Sorcerer never reaches `Computed` even with a fully valid known-spell
/// posture -- only the spell-specific diagnostic is conditional.
#[cfg(test)]
mod sorcerer_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, CharacterClassLevel, HeadlessReceiptStatus,
        ARCANE_BLOODLINE_SELECTION_ID, ARCANE_BOND_BONDED_OBJECT_SELECTION_ID,
        ARCANE_BOND_FAMILIAR_SELECTION_ID, FIGHTER_CLASS_ID, SORCERER_ARCANE_BOND_CHOICE_ID,
        SORCERER_BLOODLINE_CHOICE_ID, SORCERER_CLASS_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// A single-class Sorcerer with no bloodline chosen at all (this test's
    /// fixture never selects Arcane bloodline) still stays `Blocked` on the
    /// bloodline-power diagnostic -- unlike the Arcane-bloodline-recognized
    /// case (see `single_class_sorcerer_with_arcane_bond_recognized_reaches_computed`
    /// below), an unrecognized bloodline has no vacuous pieces to resolve,
    /// so it stays unconditionally blocking. The spell-specific diagnostic
    /// must NOT fire when the posture is genuinely valid, regardless.
    #[test]
    fn single_class_sorcerer_with_no_known_spells_stays_blocked_only_on_bloodline() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 5 }];

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Sorcerer's bloodline-power burden is permanently unconditional: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
                    && d.claim_blocking),
            "expected the permanent bloodline diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported"),
            "the spell-posture diagnostic must not fire when the known-spell posture is valid: \
             {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Sorcerer preparing a known spell beyond their
    /// spell-level access ceiling must carry the real spell-posture
    /// diagnostic too (in addition to the permanent bloodline one).
    #[test]
    fn single_class_sorcerer_with_an_inaccessible_known_spell_carries_both_diagnostics() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Acid Arrow".to_owned(),
            source_class_id: SORCERER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported"
                    && d.claim_blocking),
            "a 2nd-level sorcerer spell is not accessible at sorcerer level 1: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Sorcerer preparing a spell not on the real PF1 Core
    /// Rulebook sorcerer spell list at all must also carry the diagnostic.
    #[test]
    fn single_class_sorcerer_with_an_off_list_known_spell_stays_blocked_on_spell_posture() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Not A Real Spell".to_owned(),
            source_class_id: SORCERER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported"
                    && d.claim_blocking),
            "an off-list spell must trip the spell-posture diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Sorcerer at level 1 (cap 4 cantrips, 2 first-level
    /// spells known) knowing 3 distinct first-level spells over-knows its
    /// real cap and must carry the spell-posture diagnostic.
    #[test]
    fn single_class_sorcerer_over_known_spells_stays_blocked_on_spell_posture() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 1 }];
        for spell_id in ["Burning Hands", "Charm Person", "Cause Fear"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: SORCERER_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported"
                    && d.claim_blocking),
            "sorcerer level 1 only knows 2 first-level spells; 3 distinct known spells \
             over-knows the real cap: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Sorcerer knowing 2 real, valid, in-budget first-level
    /// spells (within the real cap) must NOT trip the spell-posture
    /// diagnostic -- proving real validation, not a vacuous check.
    #[test]
    fn single_class_sorcerer_with_valid_known_spells_does_not_trip_the_spell_posture() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 1 }];
        for spell_id in ["Burning Hands", "Charm Person"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: SORCERER_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported"),
            "2 distinct first-level spells is within sorcerer level 1's real cap of 2: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Multiclass safety, verified directly. A Sorcerer-containing
    /// multiclass mix with a genuine posture violation must still stay
    /// Blocked, since `SORCERER_CLASS_ID` is deliberately not registered
    /// with `multiclass_class_level_supported` beyond `table_class_id`
    /// itself (the same construction Ranger/Paladin already proved safe).
    #[test]
    fn sorcerer_fighter_multiclass_with_an_invalid_known_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 1 },
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
        ];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Acid Arrow".to_owned(),
            source_class_id: SORCERER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "a Sorcerer+Fighter multiclass must not reach Computed while Sorcerer's posture is \
             genuinely violated: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported"
                    && d.claim_blocking),
            "expected the real spell-posture diagnostic to fire in the multiclass mix too: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// v0.6 alpha swarm, risks item 8 (Sorcerer Arcane bloodline closure):
    /// a single-class Human Sorcerer at level 1 with the Arcane bloodline
    /// chosen, Arcane Bond recognized (either bond type), and a genuinely
    /// valid (empty) known-spell posture reaches `Computed` -- the
    /// bloodline-power burden is no longer permanently unconditional once
    /// its two vacuous pieces (bloodline arcana, Arcane Bond's
    /// spell-casting half) and its one real, recognizable piece (which
    /// bond type) are all resolved, and bonus spells/feats are correctly
    /// absent below level 3.
    #[test]
    fn single_class_sorcerer_with_arcane_bond_recognized_reaches_computed() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SORCERER_BLOODLINE_CHOICE_ID.to_owned(),
            selection_id: ARCANE_BLOODLINE_SELECTION_ID.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SORCERER_ARCANE_BOND_CHOICE_ID.to_owned(),
            selection_id: ARCANE_BOND_FAMILIAR_SELECTION_ID.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Sorcerer with Arcane bloodline + Arcane Bond recognized + a valid known-spell \
             posture should reach Computed: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.sorcerer.arcane_bloodline.arcane_bond_choice"),
            "expected the real Arcane Bond choice recognition record: {:?}",
            receipt.computation.explanations
        );
    }

    /// A bonded-object Arcane Bond choice is recognized identically to a
    /// familiar (both are real, representable choices), and also reaches
    /// `Computed`.
    #[test]
    fn single_class_sorcerer_with_bonded_object_arcane_bond_reaches_computed() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SORCERER_BLOODLINE_CHOICE_ID.to_owned(),
            selection_id: ARCANE_BLOODLINE_SELECTION_ID.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SORCERER_ARCANE_BOND_CHOICE_ID.to_owned(),
            selection_id: ARCANE_BOND_BONDED_OBJECT_SELECTION_ID.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);
    }

    /// Arcane bloodline chosen but NO Arcane Bond selection recognized
    /// still blocks -- a genuine unmet precondition (the choice was never
    /// made), not a silent pass.
    #[test]
    fn single_class_sorcerer_with_arcane_bloodline_but_no_bond_choice_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SORCERER_BLOODLINE_CHOICE_ID.to_owned(),
            selection_id: ARCANE_BLOODLINE_SELECTION_ID.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
                    && d.claim_blocking),
            "no Arcane Bond choice recognized is a genuine unmet precondition: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Superseded assertion, corrected 2026-07-29. This test previously
    /// asserted that a 3rd-level Arcane-bloodline Sorcerer stays `Blocked`
    /// because "bonus spells/feats at 3rd level are real, unimplemented
    /// grants". That was true when it was written and is no longer: the
    /// Arcane bloodline's whole 3rd-and-above progression is now grounded from
    /// the corpus (see `sorcerer_arcane_bloodline_progression_tests`), so the
    /// gap that justified the block has genuinely closed. The test is kept,
    /// inverted, at the same level so the exact boundary the old blocker fired
    /// at stays pinned rather than silently deleted.
    #[test]
    fn single_class_sorcerer_with_arcane_bond_at_level_3_now_reaches_computed() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 3 }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SORCERER_BLOODLINE_CHOICE_ID.to_owned(),
            selection_id: ARCANE_BLOODLINE_SELECTION_ID.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SORCERER_ARCANE_BOND_CHOICE_ID.to_owned(),
            selection_id: ARCANE_BOND_FAMILIAR_SELECTION_ID.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "the 3rd-level bonus spell (Identify) and bloodline power (Metamagic Adept) are now \
             grounded from the corpus, so the old block no longer names a real gap: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"),
            "expected the bloodline blocker to be retired for a recognized Arcane bloodline at \
             level 3: {:?}",
            receipt.computation.diagnostics
        );
    }
}

/// v0.6 alpha swarm (Sorcerer levels 3-20 closure, 2026-07-29): the Arcane
/// bloodline's whole 3rd-and-above progression -- bonus spells, bonus feats,
/// and the 3rd/9th/15th/20th-level bloodline powers.
///
/// Every value asserted below is transcribed from the PF1 Core Rulebook corpus
/// (`pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst`)
/// and NOT from memory:
///
/// * `KEY:Arcane Bloodline ~ Bonus Spells` -- nine `SPELLKNOWN:CLASS|Sorcerer=N|<spell>`
///   tokens, each gated `PREVARGTEQ:BloodlineCasterLVL,<grant level>`.
/// * `Arcane Bloodline` (`CATEGORY:Sorcerer Bloodline`) --
///   `BONUS:ABILITYPOOL|Sorcerer Bloodline Feat|BloodlineFeatCount`, where
///   `BONUS:VAR|BloodlineFeatCount|(BloodlineFeatProgression-1)/6|TYPE=Base` and
///   `BONUS:VAR|BloodlineFeatProgression|BloodlineProgressionLVL|TYPE=Base` and
///   `BONUS:VAR|BloodlineProgressionLVL|SorcererLVL|TYPE=Base`.
/// * `KEY:Arcane Bloodline ~ Metamagic Adept` --
///   `BONUS:VAR|Sorcerer_ArcaneMetamagicAdept_Times|floor((Sorcerer_Arcane_BloodlinePower3LVL+1)/4)`.
/// * `KEY:Arcane Bloodline ~ New Arcana` --
///   `BONUS:VAR|Sorcerer_NewArcana_Number|floor((Sorcerer_Arcane_BloodlinePower9LVL-5)/4)`.
/// * `KEY:Arcane Bloodline ~ School Power Choice` -- `BONUS:DC|SCHOOL.%LIST|2`.
/// * `KEY:Arcane Bloodline ~ Arcane Apotheosis` --
///   `BONUS:VAR|Sorcerer_Arcane_BloodlinePower3|-1` (the 20th-level supersession of
///   Metamagic Adept).
#[cfg(test)]
mod sorcerer_arcane_bloodline_progression_tests {
    use super::{
        arcane_bloodline_bonus_feat_count, arcane_bloodline_bonus_spells_known,
        arcane_bloodline_metamagic_adept_uses_per_day, arcane_bloodline_new_arcana_spell_count,
        build_pilot_headless_receipt, ground_sorcerer_bloodline_feat_pool, CharacterClassLevel, ComputationExplanation, HeadlessReceiptStatus,
        ARCANE_BLOODLINE_BONUS_SPELLS, ARCANE_BLOODLINE_ELIGIBLE_BONUS_FEATS,
        ARCANE_BLOODLINE_SELECTION_ID, SORCERER_BLOODLINE_FEAT_POOL_DIAGNOSTIC_EXCLUSIONS,
        SORCERER_BLOODLINE_FEAT_POOL_ELIGIBLE_FEATS,
        ARCANE_BOND_FAMILIAR_SELECTION_ID, SORCERER_ARCANE_BOND_CHOICE_ID,
        SORCERER_ARCANE_BLOODLINE_ARCANE_APOTHEOSIS_EXPLANATION_ID,
        SORCERER_ARCANE_BLOODLINE_BONUS_FEAT_COUNT_EXPLANATION_ID,
        SORCERER_ARCANE_BLOODLINE_BONUS_SPELLS_EXPLANATION_ID,
        SORCERER_ARCANE_BLOODLINE_METAMAGIC_ADEPT_EXPLANATION_ID,
        SORCERER_ARCANE_BLOODLINE_NEW_ARCANA_EXPLANATION_ID,
        SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_EXPLANATION_ID,
        SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_BONUS, SORCERER_BLOODLINE_CHOICE_ID,
        SORCERER_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};
    use crate::rules_core::rules_tables::crb::sorcerer_spell_list;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// The exact production posture the shipped app composes for a Sorcerer
    /// (`pf1_adapter.rs`'s Sorcerer block, mirrored by
    /// `src/bin/v06_class_state_dump.rs`'s `canonical_seeds_for`): Arcane
    /// bloodline plus a familiar as the Arcane Bond.
    fn arcane_sorcerer_receipt(level: u8) -> crate::rules_core::pilot_compute::PilotHeadlessReceipt {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SORCERER_BLOODLINE_CHOICE_ID.to_owned(),
            selection_id: ARCANE_BLOODLINE_SELECTION_ID.to_owned(),
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SORCERER_ARCANE_BOND_CHOICE_ID.to_owned(),
            selection_id: ARCANE_BOND_FAMILIAR_SELECTION_ID.to_owned(),
        });
        build_pilot_headless_receipt(&input)
    }

    fn explanation<'a>(
        explanations: &'a [ComputationExplanation],
        id: &str,
    ) -> &'a ComputationExplanation {
        explanations
            .iter()
            .find(|e| e.id == id)
            .unwrap_or_else(|| panic!("expected explanation {id}"))
    }

    /// The nine bonus-spell rows, pinned verbatim against the corpus
    /// `SPELLKNOWN:CLASS|Sorcerer=N|<spell>|...|PREVARGTEQ:BloodlineCasterLVL,<L>`
    /// tokens of `KEY:Arcane Bloodline ~ Bonus Spells`.
    #[test]
    fn arcane_bloodline_bonus_spell_table_matches_the_corpus_verbatim() {
        assert_eq!(
            ARCANE_BLOODLINE_BONUS_SPELLS,
            &[
                (3u8, 1u8, "Identify"),
                (5, 2, "Invisibility"),
                (7, 3, "Dispel Magic"),
                (9, 4, "Dimension Door"),
                (11, 5, "Overland Flight"),
                (13, 6, "True Seeing"),
                (15, 7, "Teleport (Greater)"),
                (17, 8, "Power Word Stun"),
                (19, 9, "Wish"),
            ]
        );
    }

    /// Cross-check, not a restatement: every bonus spell must resolve on the
    /// engine's own independently-ingested sorcerer spell list at exactly the
    /// spell level the bloodline record's `Sorcerer=N` token declares. A
    /// transcription slip in either table breaks this.
    #[test]
    fn every_arcane_bonus_spell_resolves_on_the_real_sorcerer_spell_list() {
        for (grant_level, spell_level, spell_id) in ARCANE_BLOODLINE_BONUS_SPELLS {
            assert_eq!(
                sorcerer_spell_list::sorcerer_spell_level(spell_id),
                Some(*spell_level),
                "{spell_id} (granted at sorcerer level {grant_level}) must be a level-\
                 {spell_level} sorcerer spell on the real list"
            );
        }
    }

    /// One new bonus spell at every odd level from 3rd through 19th, and none
    /// before 3rd.
    #[test]
    fn arcane_bloodline_bonus_spells_known_steps_at_every_odd_level_from_3() {
        let expected: [i16; 21] = [
            0, // index 0, unused
            0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9,
        ];
        for level in 1u8..=20 {
            assert_eq!(
                arcane_bloodline_bonus_spells_known(level),
                expected[usize::from(level)],
                "bonus spells known at sorcerer level {level}"
            );
        }
    }

    /// `(BloodlineFeatProgression - 1) / 6` with `BloodlineFeatProgression ==
    /// SorcererLVL`: one bloodline feat at 7th, 13th, and 19th.
    #[test]
    fn arcane_bloodline_bonus_feat_count_matches_the_corpus_formula() {
        for level in 1u8..=6 {
            assert_eq!(arcane_bloodline_bonus_feat_count(level), 0, "level {level}");
        }
        for level in 7u8..=12 {
            assert_eq!(arcane_bloodline_bonus_feat_count(level), 1, "level {level}");
        }
        for level in 13u8..=18 {
            assert_eq!(arcane_bloodline_bonus_feat_count(level), 2, "level {level}");
        }
        for level in 19u8..=20 {
            assert_eq!(arcane_bloodline_bonus_feat_count(level), 3, "level {level}");
        }
    }

    /// The eight feats the corpus `Arcane Bloodline ~ Feat Tracker` record
    /// enables, verbatim from its `BONUS:VAR|Sorcerer_BloodlineFeat_<X>|1`
    /// tokens.
    #[test]
    fn arcane_bloodline_eligible_bonus_feats_match_the_corpus_feat_tracker() {
        assert_eq!(
            ARCANE_BLOODLINE_ELIGIBLE_BONUS_FEATS,
            &[
                "Combat Casting",
                "Improved Counterspell",
                "Improved Initiative",
                "Iron Will",
                "Scribe Scroll",
                "Skill Focus (Knowledge [arcana])",
                "Spell Focus",
                "Still Spell",
            ]
        );
    }

    /// SD-34 AT-34-E3-001 (`decisions.md §16`): the bloodline feat pool's
    /// slot count is bloodline-invariant, so it grounds a magnitude EVEN
    /// when this seam has not recognized any bloodline choice at all.
    #[test]
    fn sorcerer_bloodline_feat_pool_slot_count_grounds_regardless_of_recognized_bloodline() {
        let mut explanations = Vec::new();
        let mut diagnostics = Vec::new();
        ground_sorcerer_bloodline_feat_pool(7, &mut explanations, &mut diagnostics);
        let count_explanation = explanations
            .iter()
            .find(|e| e.id == "class_feature.sorcerer.bloodline_feat_pool.slot_count")
            .expect("the slot count must ground at level 7");
        assert_eq!(count_explanation.value, 1);
    }

    #[test]
    fn sorcerer_bloodline_feat_pool_slot_count_is_correctly_absent_below_the_grant_level() {
        let mut explanations = Vec::new();
        let mut diagnostics = Vec::new();
        ground_sorcerer_bloodline_feat_pool(6, &mut explanations, &mut diagnostics);
        let count_explanation = explanations
            .iter()
            .find(|e| e.id == "class_feature.sorcerer.bloodline_feat_pool.slot_count")
            .expect("the slot count record must still be present, valued at 0");
        assert_eq!(count_explanation.value, 0);
        assert!(
            diagnostics.is_empty(),
            "no per-option diagnostic should fire before any slot is granted"
        );
    }

    #[test]
    fn sorcerer_bloodline_feat_pool_names_every_eligible_option_once_a_slot_is_granted() {
        let mut explanations = Vec::new();
        let mut diagnostics = Vec::new();
        ground_sorcerer_bloodline_feat_pool(7, &mut explanations, &mut diagnostics);
        assert_eq!(
            diagnostics.len(),
            SORCERER_BLOODLINE_FEAT_POOL_ELIGIBLE_FEATS.len()
                - SORCERER_BLOODLINE_FEAT_POOL_DIAGNOSTIC_EXCLUSIONS.len(),
            "every eligible feat gets its own diagnostic except the verified-collision \
             exclusions"
        );
        let acrobatic_steps_slug = "acrobatic_steps";
        assert!(
            diagnostics
                .iter()
                .any(|d| d.id.contains(".sorcerer.") && d.id.contains(acrobatic_steps_slug)),
            "Acrobatic Steps must have its own matching diagnostic id"
        );
        assert!(diagnostics.iter().all(|d| !d.claim_blocking), "non-claim-blocking only");
    }

    /// `floor((level + 1) / 4)`: 1/day at 3rd, rising by one at 7th, 11th,
    /// 15th, and 19th.
    #[test]
    fn arcane_bloodline_metamagic_adept_uses_match_the_corpus_formula() {
        for (level, expected) in
            [(3u8, 1i16), (4, 1), (5, 1), (6, 1), (7, 2), (10, 2), (11, 3), (14, 3), (15, 4),
             (18, 4), (19, 5), (20, 5)]
        {
            assert_eq!(
                arcane_bloodline_metamagic_adept_uses_per_day(level),
                expected,
                "Metamagic Adept uses/day at sorcerer level {level}"
            );
        }
    }

    /// `floor((level - 5) / 4)`: one added spell at 9th, a second at 13th, a
    /// third at 17th, and no fourth by 20th.
    #[test]
    fn arcane_bloodline_new_arcana_count_matches_the_corpus_formula() {
        for (level, expected) in
            [(9u8, 1i16), (12, 1), (13, 2), (16, 2), (17, 3), (20, 3)]
        {
            assert_eq!(
                arcane_bloodline_new_arcana_spell_count(level),
                expected,
                "New Arcana spells at sorcerer level {level}"
            );
        }
    }

    /// Level 3 -- the exact level the old blocker fired at. Identify is the
    /// only bonus spell yet, Metamagic Adept is 1/day, and nothing from the
    /// 9th/15th/20th tiers has arrived.
    #[test]
    fn arcane_sorcerer_reaches_computed_at_level_3_with_the_real_corpus_values() {
        let receipt = arcane_sorcerer_receipt(3);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "{:?}",
            receipt.computation.diagnostics
        );
        let explanations = &receipt.computation.explanations;

        let bonus_spells =
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_BONUS_SPELLS_EXPLANATION_ID);
        assert_eq!(bonus_spells.value, 1);
        assert!(
            bonus_spells.detail.contains("Identify"),
            "{}",
            bonus_spells.detail
        );

        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_METAMAGIC_ADEPT_EXPLANATION_ID)
                .value,
            1
        );
        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_BONUS_FEAT_COUNT_EXPLANATION_ID)
                .value,
            0
        );
        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_NEW_ARCANA_EXPLANATION_ID).value,
            0
        );
        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_EXPLANATION_ID)
                .value,
            0
        );
    }

    /// Level 9 -- four bonus spells, the New Arcana tier opens, Metamagic
    /// Adept is 2/day, one bloodline feat has been granted.
    #[test]
    fn arcane_sorcerer_reaches_computed_at_level_9_with_the_real_corpus_values() {
        let receipt = arcane_sorcerer_receipt(9);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "{:?}",
            receipt.computation.diagnostics
        );
        let explanations = &receipt.computation.explanations;

        let bonus_spells =
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_BONUS_SPELLS_EXPLANATION_ID);
        assert_eq!(bonus_spells.value, 4);
        for spell in ["Identify", "Invisibility", "Dispel Magic", "Dimension Door"] {
            assert!(bonus_spells.detail.contains(spell), "{}", bonus_spells.detail);
        }
        assert!(
            !bonus_spells.detail.contains("Overland Flight"),
            "the 11th-level bonus spell must not appear at 9th: {}",
            bonus_spells.detail
        );

        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_NEW_ARCANA_EXPLANATION_ID).value,
            1
        );
        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_METAMAGIC_ADEPT_EXPLANATION_ID)
                .value,
            2
        );
        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_BONUS_FEAT_COUNT_EXPLANATION_ID)
                .value,
            1
        );
    }

    /// Level 20 -- the full progression, plus the corpus's own
    /// `BONUS:VAR|Sorcerer_Arcane_BloodlinePower3|-1` supersession: Arcane
    /// Apotheosis replaces Metamagic Adept's per-day budget entirely, so that
    /// record must NOT report 5/day at 20th.
    #[test]
    fn arcane_sorcerer_reaches_computed_at_level_20_with_apotheosis_superseding_metamagic_adept() {
        let receipt = arcane_sorcerer_receipt(20);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "{:?}",
            receipt.computation.diagnostics
        );
        let explanations = &receipt.computation.explanations;

        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_BONUS_SPELLS_EXPLANATION_ID).value,
            9
        );
        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_BONUS_FEAT_COUNT_EXPLANATION_ID)
                .value,
            3
        );
        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_NEW_ARCANA_EXPLANATION_ID).value,
            3
        );
        assert_eq!(
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_EXPLANATION_ID)
                .value,
            SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_BONUS
        );

        let metamagic =
            explanation(explanations, SORCERER_ARCANE_BLOODLINE_METAMAGIC_ADEPT_EXPLANATION_ID);
        assert_eq!(
            metamagic.value, 0,
            "Arcane Apotheosis removes the per-day limit; reporting 5/day here would be wrong"
        );
        assert!(
            metamagic.detail.contains("Arcane Apotheosis"),
            "the 0 must be explained as supersession, not absence: {}",
            metamagic.detail
        );

        explanation(explanations, SORCERER_ARCANE_BLOODLINE_ARCANE_APOTHEOSIS_EXPLANATION_ID);
    }

    /// Every level 1-20 reaches `Computed` for the production Sorcerer
    /// posture -- the whole point of this slice.
    #[test]
    fn arcane_sorcerer_reaches_computed_at_every_level_1_through_20() {
        for level in 1u8..=20 {
            let receipt = arcane_sorcerer_receipt(level);
            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Computed,
                "sorcerer level {level}: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// The guard that must survive this widening: a Sorcerer whose bloodline
    /// this seam does not recognize gains no Arcane-specific grant and stays
    /// blocked. Only the Arcane bloodline is grounded; the other 19 CRB
    /// bloodlines are deferred, not silently claimed.
    #[test]
    fn sorcerer_without_a_recognized_bloodline_still_blocks_at_level_20() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level: 20 }];

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt.computation.diagnostics.iter().any(|d| d.id
                == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
                && d.claim_blocking),
            "{:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == SORCERER_ARCANE_BLOODLINE_BONUS_SPELLS_EXPLANATION_ID),
            "no Arcane-specific grant may be fabricated for an unrecognized bloodline"
        );
    }
}

/// task #61, 2026-07-28: Sorcerer Draconic Bloodline Dragon Resistances' natural
/// armor bonus wiring into `compute_combat_baseline`'s shared Armor Class total.
/// Mirrors `alchemist_dispatch_widening_safety_tests`'s `human_alchemist_input`-style
/// fixture-swap helper and Brawler AC Bonus's own "class ownership + level, no
/// activation" shape (task #61's Dragon Resistances additionally requires a
/// recognized Draconic bloodline choice, since -- unlike Brawler's AC Bonus -- this
/// power belongs to one specific bloodline, not the whole class).
#[cfg(test)]
mod sorcerer_draconic_bloodline_dragon_resistances_ac_wiring_tests {
    use super::{
        build_pilot_headless_receipt, ARCANE_BLOODLINE_SELECTION_ID, CharacterClassLevel,
        CharacterInput, DRACONIC_BLOODLINE_SELECTION_ID, SORCERER_BLOODLINE_CHOICE_ID,
        SORCERER_CLASS_ID,
    };
    use crate::rules_core::character_input::{SelectedChoice, load_character_input_fixture};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn sorcerer_input(level: u8, bloodline_selection: &str) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SORCERER_CLASS_ID.to_owned(), level }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SORCERER_BLOODLINE_CHOICE_ID.to_owned(),
            selection_id: bloodline_selection.to_owned(),
        });
        input
    }

    fn baseline_armor_class(input: &CharacterInput) -> i16 {
        let receipt = build_pilot_headless_receipt(input);
        receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .unwrap_or_else(|| {
                panic!(
                    "baseline Armor Class must be grounded: {:?}",
                    receipt.computation.diagnostics
                )
            })
            .value
    }

    /// Base AC (10 + Chain Shirt 4 + DEX +2 + Dodge 1 = 17) + Dragon Resistances'
    /// natural armor bonus at level 3 (+1) = 18 -- proves the magnitude is really
    /// summed into `defense.baseline_armor_class`, not just named in a standalone
    /// record.
    #[test]
    fn draconic_bloodline_sorcerer_level_3_adds_1_to_baseline_armor_class() {
        let input = sorcerer_input(3, DRACONIC_BLOODLINE_SELECTION_ID);
        assert_eq!(
            baseline_armor_class(&input),
            18,
            "Dragon Resistances' natural armor bonus must be applied"
        );
    }

    /// Level 9: Base AC 17 + Dragon Resistances' natural armor bonus (+2) = 19.
    #[test]
    fn draconic_bloodline_sorcerer_level_9_adds_2_to_baseline_armor_class() {
        let input = sorcerer_input(9, DRACONIC_BLOODLINE_SELECTION_ID);
        assert_eq!(
            baseline_armor_class(&input),
            19,
            "Dragon Resistances' natural armor bonus must be applied"
        );
    }

    /// Level 15 is the case that would be WRONG (17 + 3 = 20) if only the first of
    /// the two additively-stacking corpus BONUS:VAR lines were wired in: Base AC 17 +
    /// Dragon Resistances' natural armor bonus at level 15 (+4) = 21.
    #[test]
    fn draconic_bloodline_sorcerer_level_15_adds_4_to_baseline_armor_class() {
        let input = sorcerer_input(15, DRACONIC_BLOODLINE_SELECTION_ID);
        assert_eq!(
            baseline_armor_class(&input),
            21,
            "Dragon Resistances' natural armor bonus must be applied, including the second, \
             additively-stacking level-15+ BONUS:VAR line"
        );
    }

    /// Below the level-3 gate, no natural armor bonus is added: Base AC stays 17.
    #[test]
    fn draconic_bloodline_sorcerer_level_1_adds_nothing_to_baseline_armor_class() {
        let input = sorcerer_input(1, DRACONIC_BLOODLINE_SELECTION_ID);
        assert_eq!(
            baseline_armor_class(&input),
            17,
            "below the level-3 gate no natural armor bonus applies"
        );
    }

    /// The Arcane bloodline's own Sorcerer never gains Dragon Resistances' natural
    /// armor bonus -- proves the class-ownership-by-construction gate is genuinely
    /// bloodline-specific, not merely class-specific.
    #[test]
    fn arcane_bloodline_sorcerer_adds_nothing_to_baseline_armor_class() {
        let input = sorcerer_input(9, ARCANE_BLOODLINE_SELECTION_ID);
        assert_eq!(
            baseline_armor_class(&input),
            17,
            "the Arcane bloodline must never gain Dragon Resistances' natural armor bonus"
        );
    }
}

/// v0.6 alpha swarm, risks item 8 (Arcanist full-build closure, first
/// non-CRB class attempting real `Computed` status): tests the real
/// prepared-spellbook grounding directly, mirroring the Wizard
/// dispatch-widening test module's own shape.
#[cfg(test)]
mod arcanist_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, CharacterClassLevel, CharacterInput,
        HeadlessReceiptStatus, ARCANIST_CLASS_ID, ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID,
        EMPOWER_SPELL_METAMAGIC_SELECTION, FIGHTER_CLASS_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_arcanist_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: ARCANIST_CLASS_ID.to_owned(), level }];
        input
    }

    /// A single-class Human Arcanist with a real recorded and prepared
    /// spell reaches a genuinely grounded spellbook posture -- stays
    /// `Blocked` only on `exploits_deferred` (never the retired generic
    /// diagnostic, never `prepared_spellbook.unsupported`), with the real
    /// base/Intelligence-bonus/total spells-per-day counts grounded.
    ///
    /// Fixture Intelligence 10 (+0 modifier, no bonus spells). Level 1
    /// base: cantrips 4, 1st-level 2 (verified against the raw corpus
    /// formula and legacy.aonprd.com's own printed table).
    #[test]
    fn single_class_arcanist_with_a_real_prepared_spell_grounds_the_spellbook_and_stays_blocked_only_on_exploits()
    {
        let mut input = human_arcanist_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Arcanist stays Blocked on exploits_deferred alone, even with a real, valid \
             spellbook posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.arcanist.prepared_spellbook.unsupported"),
            "the prepared_spellbook diagnostic must not fire once a real, valid posture is \
             recorded: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.arcanist.exploits_deferred.unsupported"
                    && d.claim_blocking),
            "expected the exploits_deferred diagnostic even with a valid spellbook: {:?}",
            receipt.computation.diagnostics
        );

        let base_cantrips = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.arcanist.base_spells_per_day.spell_level_0")
            .expect("base cantrips per day must be grounded");
        assert_eq!(base_cantrips.value, 4, "Arcanist level 1 base cantrips: 4: {:?}", base_cantrips);

        let base_first_level = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.arcanist.base_spells_per_day.spell_level_1")
            .expect("base 1st-level spells per day must be grounded");
        assert_eq!(
            base_first_level.value, 2,
            "Arcanist level 1 base 1st-level spells: 2 (not Wizard's own 1): {:?}",
            base_first_level
        );
    }

    /// **The milestone test** (v0.6 alpha swarm, risks item 8, Arcanist
    /// Metamagic Knowledge Exploit closure): a real, valid spellbook
    /// posture PLUS a recognized Metamagic Knowledge choice naming
    /// `Empower Spell` clears the last remaining claim-blocking
    /// diagnostic (`exploits_deferred` becomes non-blocking) -- Arcanist
    /// reaches genuine `HeadlessReceiptStatus::Computed` for the first
    /// time among all ACG/APG classes this session. Confirmed before
    /// building (per team-lead review) that this is the real last gap,
    /// not another standalone fact.
    #[test]
    fn single_class_arcanist_with_a_valid_spellbook_and_recognized_metamagic_knowledge_reaches_computed()
    {
        let mut input = human_arcanist_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID.to_owned(),
            selection_id: EMPOWER_SPELL_METAMAGIC_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Arcanist with a valid spellbook and recognized Metamagic Knowledge must reach \
             Computed -- the last claim-blocking diagnostic should be cleared: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.claim_blocking),
            "zero claim-blocking diagnostics expected for this posture: {:?}",
            receipt.computation.diagnostics
        );

        let feat_grant = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.arcanist.metamagic_knowledge.feat_granted")
            .expect("the Metamagic Knowledge feat grant must be grounded");
        assert!(
            feat_grant.detail.contains("Empower Spell"),
            "expected the granted feat to be named: {:?}",
            feat_grant
        );
    }

    /// Proves the recognition is genuinely general, not hardcoded to
    /// `Empower Spell` alone: a different real, catalog-verified
    /// Metamagic feat (`Silent Spell`, seeded via the same
    /// `metamagic:<slug>` namespacing as `EMPOWER_SPELL_METAMAGIC_SELECTION`)
    /// is equally recognized and clears the same diagnostic.
    #[test]
    fn single_class_arcanist_metamagic_knowledge_recognizes_other_real_metamagic_feats_too() {
        let mut input = human_arcanist_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID.to_owned(),
            selection_id: "metamagic:silent_spell".to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a different real metamagic feat must be recognized equally: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// An Arcanist naming an unrecognized (non-Metamagic, or nonexistent)
    /// feat via the Metamagic Knowledge choice is a genuine posture
    /// violation and must claim-block, mirroring every other "active but
    /// unrecognized" shape this session.
    #[test]
    fn single_class_arcanist_with_an_ineligible_metamagic_knowledge_feat_stays_blocked() {
        let mut input = human_arcanist_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID.to_owned(),
            selection_id: "metamagic:toughness".to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.arcanist.metamagic_knowledge.feat_ineligible"
                    && d.claim_blocking),
            "expected the feat_ineligible claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.arcanist.exploits_deferred.unsupported"
                    && d.claim_blocking),
            "the original claim-blocking exploits_deferred diagnostic must remain: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A selection with no `metamagic:` namespace prefix at all (the same
    /// shape as the pre-fix `"Empower Spell"` literal that broke
    /// `local_store.rs`'s save-layer colon-segment requirement) must be
    /// treated as unrecognized, not accidentally translated -- proves
    /// `arcanist_metamagic_knowledge_feat_name`'s own `None`-on-missing-
    /// prefix branch is exercised, not just its happy path.
    #[test]
    fn single_class_arcanist_with_a_non_namespaced_metamagic_knowledge_selection_stays_blocked() {
        let mut input = human_arcanist_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID.to_owned(),
            selection_id: "Empower Spell".to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.arcanist.metamagic_knowledge.feat_ineligible"
                    && d.claim_blocking),
            "a non-namespaced selection must claim-block via feat_ineligible, not be silently \
             translated: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// An Arcanist with a valid spellbook but NO Metamagic Knowledge
    /// choice at all stays Blocked on exploits_deferred alone -- the
    /// pre-existing shape, unchanged by this closure.
    #[test]
    fn single_class_arcanist_without_a_metamagic_knowledge_choice_stays_blocked_on_exploits() {
        let mut input = human_arcanist_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "no Metamagic Knowledge choice means exploits_deferred still claim-blocks: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.arcanist.exploits_deferred.unsupported"
                    && d.claim_blocking),
            "expected the original claim-blocking exploits_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// An Arcanist with a recognized Metamagic Knowledge choice, but an
    /// INVALID spellbook posture (no spells recorded at all), must still
    /// stay Blocked -- Metamagic Knowledge alone is not sufficient; the
    /// spellbook posture is a genuinely separate, still-required gate.
    #[test]
    fn single_class_arcanist_with_metamagic_knowledge_but_no_spellbook_stays_blocked() {
        let mut input = human_arcanist_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID.to_owned(),
            selection_id: EMPOWER_SPELL_METAMAGIC_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Metamagic Knowledge alone, without a valid spellbook, must still stay Blocked: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.arcanist.prepared_spellbook.unsupported"
                    && d.claim_blocking),
            "expected the prepared_spellbook diagnostic since no spells are recorded: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.arcanist.exploits_deferred.unsupported"
                    && d.claim_blocking),
            "exploits_deferred should be non-blocking now that Metamagic Knowledge is \
             recognized, even though the spellbook gate still blocks overall: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// An Arcanist with a spoofed Metamagic Knowledge choice granted to
    /// a NON-Arcanist character must have it silently ignored. Also
    /// proves Fighter's own golden path is unaffected.
    #[test]
    fn non_arcanist_characters_spoofed_metamagic_knowledge_choice_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID.to_owned(),
            selection_id: EMPOWER_SPELL_METAMAGIC_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Arcanist choice: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.acg.arcanist.")),
            "a non-Arcanist character must never ground any Arcanist Metamagic Knowledge \
             explanation: {:?}",
            receipt.computation.explanations
        );
    }

    /// An Arcanist with a prepared spell that was never recorded in the
    /// spellbook is a genuine posture violation and must claim-block,
    /// mirroring `unmet_wizard_spellbook_conditions`'s own shape.
    #[test]
    fn single_class_arcanist_with_an_unrecorded_prepared_spell_stays_blocked() {
        let mut input = human_arcanist_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.arcanist.prepared_spellbook.unsupported"
                    && d.claim_blocking),
            "expected the prepared_spellbook diagnostic for an unrecorded prepared spell: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// An Arcanist who over-prepares a spell level (more spells prepared
    /// than the real slot budget allows) is a genuine posture violation
    /// and must claim-block, mirroring the same over-budget discipline
    /// used throughout this session (Rage/Bloodrage/Judgment's own
    /// rounds/uses-per-day checks).
    ///
    /// Level 1 base 1st-level slots: 2 (+0 Intelligence bonus). Preparing
    /// 3 distinct 1st-level spells exceeds this budget.
    #[test]
    fn single_class_arcanist_over_prepared_at_a_spell_level_stays_blocked() {
        let mut input = human_arcanist_input(1);
        for spell_id in ["Magic Missile", "Light", "Mage Armor"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: ARCANIST_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }
        for spell_id in ["Magic Missile", "Mage Armor"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: ARCANIST_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Prepared,
            });
        }
        // A third 1st-level prepared spell (Light is 0-level, so use a
        // second real 1st-level spell instead to trigger the over-budget
        // check at spell level 1 specifically).
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Comprehend Languages".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Comprehend Languages".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.arcanist.prepared_spellbook.unsupported"
                    && d.claim_blocking),
            "expected the prepared_spellbook diagnostic for an over-prepared spell level: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A non-Arcanist character carrying spoofed Arcanist spell
    /// selections must have them silently ignored, not applied -- the
    /// class-ownership gate is by construction
    /// (`unmet_arcanist_spellbook_conditions`/`ground_arcanist_prepared_spellbook`
    /// only ever read `spells_selected` entries whose `source_class_id`
    /// matches `ARCANIST_CLASS_ID`), not a bolt-on rejection. Also proves
    /// Fighter's own golden path is unaffected.
    #[test]
    fn non_arcanist_characters_spoofed_arcanist_spells_are_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Light".to_owned(),
            source_class_id: ARCANIST_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by stray Arcanist spell selections: \
             {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Arcane Reservoir progression at higher levels, verified against
    /// the raw corpus `BONUS:VAR` formulas directly.
    #[test]
    fn arcane_reservoir_progression_matches_the_corpus_formula_at_higher_levels() {
        for (level, expected_max, expected_fill) in [(1, 4, 3), (2, 5, 4), (4, 7, 5), (6, 9, 6)] {
            let input = human_arcanist_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let max = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.arcanist.arcane_reservoir_max")
                .expect("Arcane Reservoir max must be grounded");
            assert_eq!(max.value, expected_max, "level {level} Reservoir max: {:?}", max);

            let fill = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.arcanist.arcane_reservoir_daily_fill")
                .expect("Arcane Reservoir daily fill must be grounded");
            assert_eq!(fill.value, expected_fill, "level {level} Reservoir daily fill: {:?}", fill);
        }
    }
}

