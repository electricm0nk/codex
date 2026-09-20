#[allow(unused_imports)]
pub(crate) use super::*;

// ---------------------------------------------------------------------------
// SD-31 wave 27 -- prestige-class investigation (CRB shape), NOT a
// Monk-shaped gap. Dispatched to establish whether the 10 CRB prestige
// classes (Arcane Archer, Assassin, Duelist, Shadowdancer, Eldritch Knight,
// Dragon Disciple, Loremaster, Mystic Theurge, Pathfinder Chronicler,
// Arcane Trickster) could reach `Computed` the same way Monk did above --
// data already present, only a `table_class_id` string mapping missing.
// They cannot, for two independent reasons, each confirmed against the
// pinned corpus rather than assumed. No functional code change follows
// this note: the finding itself is this lane's deliverable, reported to
// the wave 27 orchestrator directly (this lane's write scope did not cover
// `progress.md`).
//
// 1. MONK-SHAPED CENSUS: ZERO OF TEN. Monk's fix worked because
//    `class_tables()` (`rules_tables/crb/class_tables.rs`) already carried
//    a real, corpus-backed row for Monk and `table_class_id` was the only
//    missing link. No such row exists for any prestige class in ANY of the
//    five class-id enums this dispatch chain reads (`ClassId::ALL`,
//    `ApgClassId::ALL`, `AcgClassId::ALL`, `UcClassId::ALL`,
//    `PuClassId::ALL` -- none names a single prestige class; confirmed by
//    direct enum-body read, not inference). Worse, the `class` kind's own
//    doneness gate (`src/bin/v06_work_inventory.rs`'s
//    `modelled_class_books()`, consulted by `Kind::Class`'s verdict arm)
//    enumerates precisely those five enums -- so even a fully-correct
//    chassis dispatch added inside `pilot_compute` cannot move a prestige
//    class off `engine-does-not-hold`, because the instrument asks "does some
//    registered `ClassId`-family enum name this class" BEFORE it ever asks
//    whether a real character reaches a delta. That enum registration
//    lives in `rules_tables/crb/class_tables.rs` (a new sibling enum,
//    following the ACG/APG/PU/UC precedent) and the
//    `modelled_class_books()` loop in `src/bin/v06_work_inventory.rs`, both
//    OUTSIDE this lane's granted write scope (`src/rules_core/
//    pilot_compute/` and its tests only). Confirmed directly:
//    `data/corpus/core_rulebook/class/` holds exactly the 11 base-class
//    JSON records; grepping the pinned `cr_classes.lst` for `^CLASS:`
//    returns those 11 plus 10 prestige classes, 2 `Ex-`variants, and 5 NPC
//    classes (Adept/Aristocrat/Commoner/Expert/Warrior) -- all 17 non-base
//    entries show `engine-does-not-hold` in `docs/work-inventory.json` today,
//    matching the wave brief's figure exactly, and none of them can be
//    moved from inside this file alone.
//
// 2. A REAL ARCHITECTURAL GAP, INDEPENDENT OF (1): six of the ten grant
//    spellcasting that advances an EXISTING class the character already
//    has, and nothing in this codebase can express that. `CharacterClassLevel`
//    (`character_input.rs`) is `{class_id: String, level: u8}` -- a flat,
//    independent entry with no field linking one class's levels to
//    another class's spell progression. Every spells-per-day function in
//    this file (`wizard_base_spells_per_day`, `cleric_total_spells_per_day`,
//    and every sibling) takes only that one class's own `level`;
//    `compute_multiclass_base_chassis` sums BAB/saves per class
//    independently but never touches spellcasting at all. Confirmed
//    against the corpus, not memory: `data/corpus/core_rulebook/
//    class_feature/dragon_disciple/spells_per_day.json`'s `DESC:` reads
//    verbatim "a dragon disciple gains new spells per day as if he had
//    also gained a level in an arcane spellcasting class he belonged to
//    before adding the prestige class" -- exactly the shape this codebase
//    cannot represent. Mystic Theurge's `combined_spells.json`/
//    `spell_synthesis.json` confirm the same family (dual-class variant).
//    PCGen's own encoding of this (`<Class>_CFP_Level` `DEFINE`/
//    `BONUS:VAR` tokens, present on EVERY prestige class record checked,
//    Duelist included) is a generic class-feature-point counter, NOT
//    itself the stacking mechanism.
//
//    CORRECTION (wave 27 integration cycle, after adversarial review): the
//    original version of this note classified the stacking/non-stacking
//    split by probing `data/corpus/core_rulebook/class_feature/<class>/`
//    for a "spell-shaped" JSON record. That probe is narrower than the
//    data -- in PCGen, four of the six stacking classes encode their
//    advancement as `ADD:SPELLCASTER|<type>` tokens directly on the
//    `CLASS:` line itself, not as a separate class_feature record, so a
//    directory-listing probe is structurally blind to them. Re-checked
//    directly against the pinned oracle (`cr_classes.lst`, PCGEN_ORACLE_SHA
//    7f818006e371188e5717fd18d74d18a420747fc6) with a per-class-block scan
//    for `ADD:SPELLCASTER`. The corrected, oracle-verified split:
//      STACKING (grants `ADD:SPELLCASTER|<type>` at one or more levels,
//      advancing an existing class's caster level/spells-per-day/spells-
//      known -- needs the missing cross-class-level-link mechanism):
//      Arcane Archer (`cr_classes.lst:334-340`, `ADD:SPELLCASTER|Arcane` at
//      levels 2/3/4/6/7/8/10), Arcane Trickster, Dragon Disciple, Eldritch
//      Knight, Loremaster, Mystic Theurge.
//      NON-STACKING (no `ADD:SPELLCASTER` token anywhere in the class
//      block; whatever else blocks them, it is not this mechanism):
//      Assassin, Duelist, Pathfinder Chronicler (`cr_classes.lst:467-485`:
//      the entire level progression is a single
//      `1 ABILITY:Class|AUTOMATIC|Pathfinder Chronicler` line, HD:8, 3/4
//      BAB, good Will, MAXLEVEL:10, no spellcasting in PF1 at all),
//      Shadowdancer.
//    The original note had this backwards for two of ten: it nominated
//    Arcane Archer as non-stacking (wrong -- seven of its ten levels
//    advance an existing arcane caster) and filed Pathfinder Chronicler as
//    needing the stacking mechanism (wrong -- it has no spellcasting
//    whatsoever). Building Arcane Archer as a plain chassis per the
//    original note's recommendation would have produced exactly the
//    wave's named signature defect: a class that dispatches and silently
//    drops seven levels of caster advancement.
//
// CONCLUSION, corrected: do not force a prestige class through this
// chassis. Six of ten (Arcane Archer, Arcane Trickster, Dragon Disciple,
// Eldritch Knight, Loremaster, Mystic Theurge) need a caster-level-stacking
// mechanism that does not exist in any form in this codebase, and building
// it correctly is new evaluator work, not a dispatch fix -- exactly the
// "genuinely cannot represent it" case the brief pre-authorized flagging
// over hacking. The other four (Assassin, Duelist, Pathfinder Chronicler,
// Shadowdancer) are architecturally closer to buildable (no caster-stacking
// blocker), but even they cannot bank a single `class`-kind unit from a
// `pilot_compute`-only write scope, because the doneness instrument's
// class-membership gate is registered entirely outside this directory.
// NEEDS AN OPERATOR/ORCHESTRATOR RULING: either widen a future
// prestige-class lane's write scope to include
// `rules_tables/crb/class_tables.rs` (or a new sibling enum) and
// `src/bin/v06_work_inventory.rs`'s `modelled_class_books()`, or route
// class-enum registration through the census lane / an integration cycle
// once this lane's (or a future one's) chassis logic is ready to consume
// it.
// ---------------------------------------------------------------------------

/// Table-driven base-attack-bonus / base-save chassis pillar for any class
/// `table_class_id` recognizes besides Fighter (v0.6 alpha swarm, task 4).
/// Mirrors `compute_wizard_chassis`'s exact shape: reads
/// `class_tables()`'s row for `class_id` at `level` rather than re-deriving
/// any formula, and pushes the same generic `class_chassis.base_attack_bonus`
/// / `class_chassis.base_save.*` explanation ids every chassis function
/// pushes. Returns `None` (with a claim-blocking `class_chassis.unsupported`
/// diagnostic) when no table row exists for `class_id` at `level` -- i.e.
/// `level` exceeds that class's own `class_tables()`-declared ceiling, so a
/// level beyond what this table can verify stays honestly blocked rather
/// than silently computed. Read the ceiling from `CLASS_META` rather than
/// from this sentence: every CRB class now declares 20 (Druid was the last
/// to widen, v0.6 2026-07-29), but that is a fact about the table's current
/// contents, not a guarantee this function relies on.
///
/// Corrected 2026-07-29 (Monk chassis-recognition closure): this comment
/// previously cited concrete ceiling examples -- "Druid caps at 15" and
/// "Monk at 12" -- both of which went stale. Druid widened to 20 the same
/// day; Monk's row had already declared `max_supported_level: 20` while
/// Monk was not reachable through this function at all, since
/// `table_class_id` did not map `class:monk` until this cycle. That is
/// exactly the shipped-prose drift `AGENTS.md` warns about -- a doc comment
/// naming a number no live function agrees with -- which is why the text
/// above now points at `CLASS_META` instead of quoting values.
pub(super) fn compute_generic_table_chassis(
    class_id: ClassId,
    class_id_str: &str,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> Option<(i16, BaseSaves)> {
    let Some(row) = class_tables()
        .into_iter()
        .find(|row| row.class_id == class_id && row.level == level)
    else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis has no {class_id_str} class_tables() row at level \
                 {level}, so no chassis values were computed"
            ),
            claim_blocking: true,
        });
        return None;
    };

    let base_attack_bonus = row.base_attack_bonus;
    let base_saves = BaseSaves {
        fortitude: row.fort_save,
        reflex: row.ref_save,
        will: row.will_save,
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "{class_id_str} level {level} base attack bonus from \
             rules_tables::crb::class_tables::class_tables()'s row for this class: \
             {base_attack_bonus}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: base_saves.fortitude,
        detail: format!(
            "{class_id_str} level {level} base Fortitude save from \
             rules_tables::crb::class_tables::class_tables()'s row for this class: {}",
            base_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: base_saves.reflex,
        detail: format!(
            "{class_id_str} level {level} base Reflex save from \
             rules_tables::crb::class_tables::class_tables()'s row for this class: {}",
            base_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: base_saves.will,
        detail: format!(
            "{class_id_str} level {level} base Will save from \
             rules_tables::crb::class_tables::class_tables()'s row for this class: {}",
            base_saves.will
        ),
    });

    Some((base_attack_bonus, base_saves))
}

/// v0.6 alpha swarm, risks item 8 -- the class/multiclass breadth scoping
/// plan's second slice: the same generic table-driven BAB/save pillar as
/// `compute_generic_table_chassis`, sourced from `rules_tables::apg`'s own
/// already-built `ApgClassId`/`class_chassis_resolve` (built in a past
/// SD-22 cycle, deliberately left unwired -- see
/// `class-multiclass-breadth-scoping.md`'s "central finding"). Deliberately
/// single-class-only by construction: this is only ever reached from
/// `compute_class_chassis`'s single-class branch, and is never registered
/// with `table_class_id`/`multiclass_class_level_supported`, so an
/// APG-class-containing multiclass mix cannot reach this path at all --
/// avoiding the exact multiclass loophole the Ranger slice's adversarial
/// review found and fixed (see `class_spell.ranger.partial_caster.unsupported`'s
/// own doc comment).
///
/// Unlike `compute_generic_table_chassis` (CRB classes, which each have
/// their own `explain_<class>_...` function separately deciding what else
/// is missing), an APG class has zero existing per-class investment beyond
/// this BAB/save table -- no class-skill list, no named class features, no
/// spellcasting. This function pushes a real, unconditional claim-blocking
/// diagnostic naming that whole remaining bucket, every time it's called,
/// so recognizing the class for BAB/save purposes can never silently
/// produce a false `Computed` status while that bucket stays genuinely
/// unbuilt.
pub(super) fn compute_apg_class_chassis(
    class_id: ApgClassId,
    class_id_str: &str,
    level: u8,
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> Option<(i16, BaseSaves)> {
    let Some(row) = apg::class_chassis_resolve(class_id, level, RuleSetId::Apg) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis has no {class_id_str} APG class_chassis_resolve row at \
                 level {level} (exceeds this class's real MAXLEVEL ceiling), so no chassis \
                 values were computed"
            ),
            claim_blocking: true,
        });
        return None;
    };

    let base_attack_bonus = row.base_attack_bonus;
    let base_saves = BaseSaves {
        fortitude: row.fort_save,
        reflex: row.ref_save,
        will: row.will_save,
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "{class_id_str} level {level} base attack bonus from \
             rules_tables::apg::class_chassis_resolve's row for this class: {base_attack_bonus}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: base_saves.fortitude,
        detail: format!(
            "{class_id_str} level {level} base Fortitude save from \
             rules_tables::apg::class_chassis_resolve's row for this class: {}",
            base_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: base_saves.reflex,
        detail: format!(
            "{class_id_str} level {level} base Reflex save from \
             rules_tables::apg::class_chassis_resolve's row for this class: {}",
            base_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: base_saves.will,
        detail: format!(
            "{class_id_str} level {level} base Will save from \
             rules_tables::apg::class_chassis_resolve's row for this class: {}",
            base_saves.will
        ),
    });

    // v0.6 alpha swarm, risks item 8 (Cavalier Mount / Alchemist Mutagen /
    // Inquisitor Judgment / Oracle / Witch full-build closures, first
    // through fifth APG class-specific closures, spot-checked
    // 2026-07-26): Cavalier, Alchemist, Inquisitor, Oracle, and Witch are
    // the five APG classes with a genuinely real class feature now (the
    // Mount, unconditional on class ownership and level alone; Mutagen
    // and Judgment, both choice- and activation-gated; Oracle's own
    // known-spell posture plus Mystery/Curse choices; Witch's Ward hex --
    // see each one's own doc comment). The other 1 APG class keeps the
    // exact original unconditional diagnostic unchanged. These branches
    // are reached only for single-class Cavalier/Alchemist/Inquisitor/
    // Oracle/Witch (this function is only ever called from
    // `compute_class_chassis`'s single-class-only section;
    // `ApgClassId::from_class_id_str` is deliberately not registered with
    // `multiclass_class_level_supported`, so a Cavalier-, Alchemist-,
    // Inquisitor-, Oracle-, or Witch-containing multiclass mix never
    // reaches this function at all).
    if class_id == ApgClassId::Cavalier {
        ground_cavalier_mount_and_defer_the_rest(
            input,
            level,
            ability_modifiers,
            explanations,
            diagnostics,
        );
    } else if class_id == ApgClassId::Alchemist {
        ground_or_block_alchemist_mutagen(input, level, explanations, diagnostics);
        let alchemist_intelligence_modifier =
            ability_modifier(input.chosen.ability_scores.intelligence);
        ground_alchemist_bomb_and_poison_resistance(
            input,
            level,
            alchemist_intelligence_modifier,
            &input.chosen.selected_feats,
            explanations,
        );
        ground_alchemist_feral_mutagen_discovery(input, level, explanations);
        // SD-32 T12 Epic 8: generic pass over the OTHER 34 Discoveries
        // (and Grand Discovery) this file has never hand-modelled by name.
        push_generic_pool_choice_magnitude(
            input,
            level,
            &ability_modifiers_from_scores(&input.chosen.ability_scores),
            ALCHEMIST_DISCOVERY_CHOICE_ID,
            "Discovery",
            "discovery:",
            "class_feature.apg.alchemist.discovery.generic",
            ALCHEMIST_DISCOVERY_GRANT_LEVEL,
            explanations,
        );
        let extract_unmet =
            unmet_alchemist_extract_conditions(input, level, alchemist_intelligence_modifier);
        if extract_unmet.is_empty() {
            ground_alchemist_prepared_extracts(
                input,
                level,
                alchemist_intelligence_modifier,
                explanations,
            );
        } else {
            diagnostics.push(ComputationDiagnostic {
                id: "class_spell.apg.alchemist.prepared_extracts.unsupported".to_owned(),
                message: format!(
                    "Alchemist remains blocked on its prepared extract / daily preparation / \
                     extract slot posture burden: {}",
                    extract_unmet.join("; ")
                ),
                claim_blocking: true,
            });
        }
    } else if class_id == ApgClassId::Inquisitor {
        ground_or_block_inquisitor_judgment(input, level, explanations, diagnostics);
        ground_inquisitor_flat_named_facts(input, level, explanations);
        ground_or_block_inquisitor_domain_power(input, level, explanations, diagnostics);
        let inquisitor_known_spell_unmet = unmet_inquisitor_known_spell_conditions(input, level);
        if inquisitor_known_spell_unmet.is_empty() {
            ground_inquisitor_known_spells(input, level, explanations);
        } else {
            diagnostics.push(ComputationDiagnostic {
                id: "class_spell.apg.inquisitor.known_spells.unsupported".to_owned(),
                message: format!(
                    "Inquisitor remains blocked on its known-spell posture burden: {}",
                    inquisitor_known_spell_unmet.join("; ")
                ),
                claim_blocking: true,
            });
        }
    } else if class_id == ApgClassId::Oracle {
        ground_or_block_oracle_class_features(input, level, explanations, diagnostics);
    } else if class_id == ApgClassId::Witch {
        ground_or_block_witch_class_features(input, level, explanations, diagnostics);
    } else if class_id == ApgClassId::Summoner {
        ground_summoner_eidolon(input, level, explanations, diagnostics);
        ground_summoner_slice_a_features(input, level, explanations);
    } else {
        // The real, unconditional blocker: nothing beyond BAB/save/HP is
        // grounded for any other APG class yet -- no class-skill list, no
        // named class features, no spellcasting (even for the casters
        // among the 1 remaining).
        diagnostics.push(ComputationDiagnostic {
            id: format!("class_feature.apg.{}.unsupported", class_id.name()),
            message: format!(
                "{class_id_str} remains blocked beyond its base-attack-bonus/base-save chassis \
                 pillar: this APG class has no class-skill list, no named class-feature \
                 computation, and no spellcasting posture grounded anywhere in this codebase yet \
                 (only the BAB/save table and hit die are transcribed); no class-feature or spell \
                 execution is fabricated in this bounded chassis baseline"
            ),
            claim_blocking: true,
        });
    }

    Some((base_attack_bonus, base_saves))
}

/// The character's base land speed in feet, read from the authoritative
/// CRB race table's own `Speed` trait row (`GAIT:WALK|N`) rather than
/// re-transcribed here. `None` for an unrecognized race id.
pub(super) fn base_land_speed_feet(race_id: &str) -> Option<i16> {
    use crate::rules_core::rules_tables::crb::race_tables::{race_traits, RaceId};
    let race = match race_id {
        "race:human" => RaceId::Human,
        "race:dwarf" => RaceId::Dwarf,
        "race:elf" => RaceId::Elf,
        "race:gnome" => RaceId::Gnome,
        "race:half-elf" => RaceId::HalfElf,
        "race:half-orc" => RaceId::HalfOrc,
        "race:halfling" => RaceId::Halfling,
        _ => return None,
    };
    race_traits()
        .iter()
        .find(|t| t.race_id == race && t.trait_name == "Speed")
        .map(|t| t.value)
}

pub(super) const QUADRUPED_LEGS_EVOLUTIONS: i16 = 2;

/// Quadruped's own ability-score bonuses
/// (`BONUS:STAT|STR|4|TYPE=Race`, `BONUS:STAT|DEX|4|TYPE=Race`).
pub(super) const QUADRUPED_STRENGTH_BONUS: i16 = 4;

pub(super) const QUADRUPED_DEXTERITY_BONUS: i16 = 4;

/// The one evolution this slice genuinely builds, namespaced
/// `evolution:<snake_case_slug>` to match the `metamagic:`/`domain:`/
/// `bloodline:` convention every other chooser in this file uses.
///
/// Improved Natural Armor was chosen over the other 103 for three
/// reasons, in order of weight:
///
/// 1. It is the only cheap evolution whose magnitude lands on a total
///    this engine ALREADY computes -- `eidolon_natural_armor_bonus` and
///    the Eidolon race's own `EIDOLON_RACIAL_NATURAL_ARMOR`. That makes
///    it a genuine integration rather than another standalone record
///    parked beside the stat block.
/// 2. Its corpus `TYPE:EvolutionChoice.Extraordinary` carries no base-form
///    restriction, so it is legal on the canonical Quadruped without
///    smuggling in a second base form. (Contrast `Evolution ~ Bite`,
///    whose TYPE names Quadruped and Serpentine only.)
/// 3. Its real prerequisite is checkable from state this engine already
///    has -- see `IMPROVED_NATURAL_ARMOR_LEVELS_PER_EXTRA`.
pub(super) const IMPROVED_NATURAL_ARMOR_EVOLUTION_SELECTION: &str = "evolution:improved_natural_armor";

/// `Evolution ~ Improved Natural Armor`'s cost in evolution points.
///
/// The record carries NO `COST:` field. That is not a missing value: in
/// this corpus `COST:` is the override and its absence means the PCGen
/// default of 1, which is why 41 of the 104 evolution records omit it
/// while 37 carry `COST:2`, 9 `COST:3` and 14 `COST:4`. Verified by
/// enumerating the whole `KEY:Evolution ~ *` family rather than by
/// reading this one record in isolation.
pub(super) const IMPROVED_NATURAL_ARMOR_COST: i16 = 1;

/// Improved Natural Armor's own magnitude:
/// `BONUS:VAR|AC_Natural_Armor|2|TYPE=Base.STACK`, corroborated by the
/// record's own `DESC` ("giving it a +2 bonus to its natural armor").
/// `STACK:YES`/`MULT:YES` are why it stacks with the level-driven
/// progression instead of overlapping it.
pub(super) const IMPROVED_NATURAL_ARMOR_BONUS: i16 = 2;

/// The record's real prerequisite,
/// `PREVARLTEQ:EvoImpNatArmCount,MasterLevel/5`: the number of instances
/// already taken must not exceed master level / 5 (integer division).
///
/// This slice buys exactly ONE instance, and the check for the first
/// instance is `0 <= level/5`, which holds at every level from 1 to 20.
/// So the purchase is genuinely legal across the whole sweep -- it is
/// evaluated below rather than assumed, because a SECOND instance would
/// require level 5 and this constant is what makes that boundary
/// explicit for whoever widens the slice next.
pub(super) const IMPROVED_NATURAL_ARMOR_LEVELS_PER_EXTRA: i16 = 5;

/// Folds a class feature's display name into the snake_case tail of its
/// explanation id (`"Opportune Parry and Riposte"` ->
/// `"opportune_parry_and_riposte"`, `"Kip-Up"` -> `"kip_up"`).
///
/// Deliberately class-neutral: Swashbuckler's deeds and Skald's
/// zero-magnitude features both build ids this way, and the ids they
/// produce are already namespaced by their own
/// `class_feature.acg.<class>.` prefix at the call site, so sharing this
/// fold cannot collide two classes' records. It carried a
/// `swashbuckler_`-prefixed name when only that class used it; the name
/// was corrected rather than a second identical copy added.
pub(super) fn class_feature_id_slug(display_name: &str) -> String {
    display_name
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c == ' ' || c == '-' {
                Some('_')
            } else {
                None
            }
        })
        .collect()
}

/// Title-cases a lowercase spirit key for prose use in explanation text.
pub(super) fn key_titlecase(key: &str) -> String {
    let mut chars = key.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// The ACG counterpart of `compute_apg_class_chassis` (v0.6 alpha swarm,
/// risks item 8, fourth slice) -- identical shape, sourcing from
/// `rules_tables::acg::class_chassis_resolve` instead of
/// `rules_tables::apg::class_chassis_resolve`. Deliberately NOT registered
/// with `table_class_id`/`multiclass_class_level_supported` for the same
/// reason: only reachable from `compute_class_chassis`'s already-single-
/// class-only section, so an ACG-class-containing multiclass mix cannot
/// reach this path at all.
pub(super) fn compute_acg_class_chassis(
    class_id: AcgClassId,
    class_id_str: &str,
    level: u8,
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> Option<(i16, BaseSaves)> {
    let Some(row) = acg::class_chassis_resolve(class_id, level, RuleSetId::Acg) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis has no {class_id_str} ACG class_chassis_resolve row at \
                 level {level} (exceeds this class's real MAXLEVEL ceiling), so no chassis \
                 values were computed"
            ),
            claim_blocking: true,
        });
        return None;
    };

    let base_attack_bonus = row.base_attack_bonus;
    let base_saves = BaseSaves {
        fortitude: row.fort_save,
        reflex: row.ref_save,
        will: row.will_save,
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "{class_id_str} level {level} base attack bonus from \
             rules_tables::acg::class_chassis_resolve's row for this class: {base_attack_bonus}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: base_saves.fortitude,
        detail: format!(
            "{class_id_str} level {level} base Fortitude save from \
             rules_tables::acg::class_chassis_resolve's row for this class: {}",
            base_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: base_saves.reflex,
        detail: format!(
            "{class_id_str} level {level} base Reflex save from \
             rules_tables::acg::class_chassis_resolve's row for this class: {}",
            base_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: base_saves.will,
        detail: format!(
            "{class_id_str} level {level} base Will save from \
             rules_tables::acg::class_chassis_resolve's row for this class: {}",
            base_saves.will
        ),
    });

    // v0.6 alpha swarm, risks item 8 (first through tenth APG/ACG
    // closures, adversarially reviewed 2026-07-25 for the gate-widening
    // piece): Skald, Bloodrager, Brawler, Hunter, Arcanist, Warpriest,
    // Slayer, Swashbuckler, Investigator, and Shaman are the ten ACG
    // classes with a genuinely real class feature now (Inspired Rage /
    // Bloodrage / AC Bonus / Animal Companion / real prepared
    // spellcasting + Arcane Reservoir / real prepared spellcasting +
    // Blessings + Sacred Weapon / Sneak Attack dice + Trap Sense +
    // Trapfinding + Track / Panache + Charmed Life + Nimble /
    // Trapfinding + Trap Sense + Inspiration pool-size / Life Spirit's
    // Channel) -- every APG class except Cavalier/Alchemist/Inquisitor/
    // Oracle/Witch keeps the exact original unconditional diagnostic
    // unchanged. These branches are reached only for single-class
    // Skald/Bloodrager/Brawler/Hunter/Arcanist/Warpriest/Slayer/
    // Swashbuckler/Investigator/Shaman (this function is only ever
    // called from `compute_class_chassis`'s single-class-only section;
    // `AcgClassId::from_class_id_str` is deliberately not registered
    // with `multiclass_class_level_supported`, so any of these ten
    // classes in a multiclass mix never reaches this function at all),
    // so no separate gate-ordering/hoisting fix is needed the way CRB
    // classes required once `table_class_id` recognized them
    // generically.
    if class_id == AcgClassId::Skald {
        ground_or_block_skald_inspired_rage(input, level, ability_modifiers, explanations, diagnostics);
        ground_or_block_skald_spellcasting(input, level, ability_modifiers, explanations, diagnostics);
        ground_skald_damage_reduction(level, explanations);
        ground_skald_bardic_knowledge(level, explanations);
        ground_skald_well_versed(level, explanations);
        ground_skald_spell_kenning(level, explanations);
        ground_skald_lore_master(level, explanations);
        ground_skald_versatile_performance(level, explanations);
        ground_skald_rage_powers_pool_size(level, explanations);
        ground_skald_remaining_named_features(
            level,
            ability_modifiers.charisma,
            &input.chosen.selected_feats,
            explanations,
        );
        push_skald_other_features_deferred_diagnostic(diagnostics);
    } else if class_id == AcgClassId::Bloodrager {
        ground_or_block_bloodrager_bloodrage(input, level, ability_modifiers, explanations, diagnostics);
    } else if class_id == AcgClassId::Brawler {
        ground_brawler_ac_bonus_and_defer_the_rest(input, level, explanations, diagnostics);
    } else if class_id == AcgClassId::Hunter {
        ground_hunter_animal_companion_and_defer_the_rest(
            input,
            level,
            ability_modifiers,
            explanations,
            diagnostics,
        );
    } else if class_id == AcgClassId::Arcanist {
        ground_or_block_arcanist_class_features(
            input,
            level,
            ability_modifiers,
            explanations,
            diagnostics,
        );
    } else if class_id == AcgClassId::Warpriest {
        ground_or_block_warpriest_class_features(
            input,
            level,
            ability_modifiers,
            explanations,
            diagnostics,
        );
    } else if class_id == AcgClassId::Slayer {
        ground_or_block_slayer_class_features(input, level, explanations, diagnostics);
    } else if class_id == AcgClassId::Swashbuckler {
        ground_or_block_swashbuckler_class_features(
            input,
            level,
            ability_modifiers,
            explanations,
            diagnostics,
        );
    } else if class_id == AcgClassId::Investigator {
        ground_or_block_investigator_class_features(
            input,
            level,
            ability_modifiers,
            explanations,
            diagnostics,
        );
    } else if class_id == AcgClassId::Shaman {
        ground_or_block_shaman_class_features(input, level, ability_modifiers, explanations, diagnostics);
    } else {
        // The real, unconditional blocker: nothing beyond BAB/save/HP is
        // grounded for any other ACG class yet -- no class-skill list, no
        // named class features, no spellcasting (even for the casters
        // among the 10).
        diagnostics.push(ComputationDiagnostic {
            id: format!("class_feature.acg.{}.unsupported", class_id.name()),
            message: format!(
                "{class_id_str} remains blocked beyond its base-attack-bonus/base-save chassis \
                 pillar: this ACG class has no class-skill list, no named class-feature \
                 computation, and no spellcasting posture grounded anywhere in this codebase yet \
                 (only the BAB/save table and hit die are transcribed); no class-feature or spell \
                 execution is fabricated in this bounded chassis baseline"
            ),
            claim_blocking: true,
        });
    }

    Some((base_attack_bonus, base_saves))
}

/// The shared `CombatFeatIntRequirement` idiom: the effective
/// Intelligence score used when checking combat-feat prerequisites
/// (task #14, 2026-07-27).
///
/// Two ACG classes write this same corpus variable with different
/// operands, which is why this takes the substitute as a parameter
/// rather than hardcoding either:
/// - Brawler's Cunning: `max(13,INTSCORE)` -- a constant floor.
/// - Swashbuckler Finesse: `max(CHASCORE,INTSCORE)` -- the higher of two
///   real ability SCORES.
///
/// Genuine reuse at the variable and consumer level, but NOT a drop-in
/// call: Brawler's existing single-operand signature could not express
/// Finesse, so it was generalized rather than described as already
/// satisfying it. Takes raw SCORES, not derived modifiers, because the
/// real rules substitute the score itself.
pub(super) fn effective_combat_feat_intelligence_score(
    substitute_score: i16,
    intelligence_score: i16,
) -> i16 {
    intelligence_score.max(substitute_score)
}

/// Whether `class_level` is a class this dispatch grounds a base-chassis
/// computation for (a bespoke `compute_<class>_chassis` for Fighter/Wizard,
/// or the generic table-driven path for every other class `table_class_id`
/// recognizes), at a level within that class's own `class_tables()`-declared
/// ceiling.
///
/// **Doc-accuracy correction (v0.6 alpha swarm, risks item 8, 2026-07-24)**:
/// this comment previously claimed the widening covered "every core class"
/// `class_tables()` carries data for (naming all 11). That was never true
/// of the code -- this function has always bottomed out in
/// `table_class_id`, which recognizes only the classes named on that
/// function's own doc comment (Fighter, Wizard, Rogue, and now Ranger --
/// see `table_class_id`'s doc comment for the real, current allowlist and
/// why it isn't wider yet). Corrected here rather than left to mislead a
/// future reader into believing multiclass already supports all 11.
pub(super) fn multiclass_class_level_supported(class_level: &CharacterClassLevel) -> bool {
    let Some(class_id) = table_class_id(&class_level.class_id) else {
        return false;
    };
    class_tables()
        .iter()
        .any(|row| row.class_id == class_id && row.level == class_level.level)
}

/// Whether `input` is a length-2+ `class_levels` mix every entry of which is
/// individually a supported class/level (SD-21 E7.28; widened v0.6 alpha
/// swarm task 4 from Fighter-or-Wizard-only to any class/level pair
/// `multiclass_class_level_supported` recognizes).
pub(super) fn is_supported_multiclass_mix(input: &CharacterInput) -> bool {
    input.chosen.class_levels.len() >= 2
        && input
            .chosen
            .class_levels
            .iter()
            .all(multiclass_class_level_supported)
}

pub(super) fn compute_class_chassis(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> Option<(i16, BaseSaves)> {
    if input.chosen.class_levels.len() >= 2 {
        return compute_multiclass_base_chassis(input, ability_modifiers, explanations, diagnostics);
    }
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return None;
    };
    if class_level.class_id == FIGHTER_CLASS_ID {
        Some(compute_fighter_chassis(input, explanations, diagnostics))
    } else if class_level.class_id == WIZARD_CLASS_ID {
        Some(compute_wizard_chassis(input, explanations, diagnostics))
    } else if let Some(class_id) = table_class_id(&class_level.class_id) {
        compute_generic_table_chassis(
            class_id,
            &class_level.class_id,
            class_level.level,
            explanations,
            diagnostics,
        )
    } else if let Some(apg_class_id) = ApgClassId::from_class_id_str(&class_level.class_id) {
        // Deliberately NOT registered with `table_class_id` -- this branch
        // is only ever reached here, from the single-class-only section of
        // `compute_class_chassis` (multiclass returns early above), so an
        // APG-class-containing multiclass mix cannot reach this path at
        // all. See `compute_apg_class_chassis`'s own doc comment.
        compute_apg_class_chassis(
            apg_class_id,
            &class_level.class_id,
            class_level.level,
            input,
            ability_modifiers,
            explanations,
            diagnostics,
        )
    } else if let Some(acg_class_id) = AcgClassId::from_class_id_str(&class_level.class_id) {
        // Deliberately NOT registered with `table_class_id` -- same reasoning
        // as the APG branch above. See `compute_acg_class_chassis`'s own doc
        // comment.
        compute_acg_class_chassis(
            acg_class_id,
            &class_level.class_id,
            class_level.level,
            input,
            ability_modifiers,
            explanations,
            diagnostics,
        )
    } else if let Some(pu_class_id) = PuClassId::from_class_id_str(&class_level.class_id) {
        // Deliberately NOT registered with `table_class_id` -- same reasoning
        // as the APG and ACG branches above, and it matters more here: the
        // four Unchained classes are replacements for CRB/APG classes, and
        // registering them with the generic CRB table would be the one edit
        // that could make `class:unchained_rogue` resolve CRB Rogue rows.
        // See `compute_pu_class_chassis`'s own doc comment.
        compute_pu_class_chassis(
            pu_class_id,
            &class_level.class_id,
            class_level.level,
            input,
            ability_modifiers,
            explanations,
            diagnostics,
        )
    } else if let Some(uc_class_id) = UcClassId::from_class_id_str(&class_level.class_id) {
        // Deliberately NOT registered with `table_class_id` -- same
        // reasoning as the APG/ACG/PU branches above. See
        // `compute_uc_class_chassis`'s own doc comment.
        compute_uc_class_chassis(
            uc_class_id,
            &class_level.class_id,
            class_level.level,
            input,
            ability_modifiers,
            explanations,
            diagnostics,
        )
    } else if let Some(row) =
        untabled_base_class_chassis::resolve(&class_level.class_id, class_level.level)
    {
        // SD-32 Epic 3 (`epic-3-class-reachability`, AT-32-E3-001), second
        // half: a real base class (Aegis, Antipaladin, Cryptic, Dread,
        // Kineticist, Magus, Marksman, Medium, Mesmerist, Occultist, Psion,
        // Psychic, Psychic Warrior, Shifter, Soulknife, Spiritualist,
        // Tactician, Vigilante, Vitalist, or Wilder) that had no dispatch
        // arm anywhere in this function until this cycle. Unlike the
        // prestige-entry-gate arm below, this one DOES produce a real
        // chassis magnitude -- base attack bonus and all three base saves,
        // computed from `untabled_base_class_chassis::resolve`'s
        // corpus-derived registry via the same
        // `rules_tables::crb::class_tables` formulas the CRB table itself
        // uses. See `untabled_base_class_chassis`'s own doc comment for the
        // 18-vs-20 population correction and the re-derive command.
        let base_attack_bonus = row.base_attack_bonus;
        let base_saves = BaseSaves {
            fortitude: row.fort_save,
            reflex: row.ref_save,
            will: row.will_save,
        };

        explanations.push(ComputationExplanation {
            id: "class_chassis.base_attack_bonus".to_owned(),
            value: base_attack_bonus,
            detail: format!(
                "{} ({}) level {} base attack bonus from \
                 pilot_compute::untabled_base_class_chassis::resolve's corpus-derived row for \
                 this class: {base_attack_bonus}",
                row.display_name, class_level.class_id, class_level.level
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_save.fortitude".to_owned(),
            value: base_saves.fortitude,
            detail: format!(
                "{} ({}) level {} base Fortitude save from \
                 pilot_compute::untabled_base_class_chassis::resolve's corpus-derived row for \
                 this class: {}",
                row.display_name, class_level.class_id, class_level.level, base_saves.fortitude
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_save.reflex".to_owned(),
            value: base_saves.reflex,
            detail: format!(
                "{} ({}) level {} base Reflex save from \
                 pilot_compute::untabled_base_class_chassis::resolve's corpus-derived row for \
                 this class: {}",
                row.display_name, class_level.class_id, class_level.level, base_saves.reflex
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_save.will".to_owned(),
            value: base_saves.will,
            detail: format!(
                "{} ({}) level {} base Will save from \
                 pilot_compute::untabled_base_class_chassis::resolve's corpus-derived row for \
                 this class: {}",
                row.display_name, class_level.class_id, class_level.level, base_saves.will
            ),
        });

        // SD-32 card 11 (T12, `epic-2-t12-modelled-class-books` cycle-1's
        // own named next lever): the generic corpus-derived roster,
        // reused across every class the fixture covers. See
        // `push_untabled_base_class_feature_records`'s own doc comment.
        push_untabled_base_class_feature_records(
            &class_level.class_id,
            class_level.level,
            explanations,
        );
        // SD-32 card 11 (T12): the first magnitude-bearing group of the
        // roster given real per-feature compute functions rather than
        // left "named, not attempted" -- see
        // `ground_antipaladin_class_features`'s own doc comment.
        if class_level.class_id == "class:antipaladin" {
            ground_antipaladin_class_features(class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:cryptic" {
            ground_cryptic_class_features(input, class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:dread" {
            ground_dread_class_features(input, class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:marksman" {
            ground_marksman_class_features(input, class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:psychic_warrior" {
            ground_psychic_warrior_class_features(input, class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:soulknife" {
            ground_soulknife_class_features(class_level.level, explanations);
        } else if class_level.class_id == "class:aegis" {
            ground_aegis_class_features(class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:tactician" {
            ground_tactician_class_features(input, class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:vitalist" {
            ground_vitalist_class_features(input, class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:wilder" {
            ground_wilder_class_features(input, class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:kineticist" {
            ground_kineticist_class_features(class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:medium" {
            ground_medium_class_features(class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:mesmerist" {
            ground_mesmerist_class_features(class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:occultist" {
            ground_occultist_class_features(class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:psychic" {
            ground_psychic_class_features(input, class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:spiritualist" {
            ground_spiritualist_class_features(class_level.level, explanations);
            // SD-34 wave 44 (`decisions.md §22`, Piece 2 item 4): Shared
            // Consciousness's own `BONUS:ABILITYPOOL|Phantom Emotional
            // Focus|1` (`oa_abilities_class.lst:1276`) is a genuine
            // one-pick pool over the seven `"Phantom Emotional Focus ~
            // <Name>"` records (Anger/Dedication/Despair/Fear/Hatred/
            // Jealousy/Zeal), each a bare literal `BONUS:VAR|
            // PhantomEmotionalFocus_<Name>|1` -- the identical
            // choose-one-flat-literal shape `push_generic_pool_choice_
            // magnitude` already resolves for Alchemist Discovery/Rogue
            // Talent/etc, verified directly against the real corpus tokens
            // before assuming the audit's "misrouted, not unmodelled"
            // framing (it was half right: the CLASS routing was the only
            // real bug, but no existing function names WHICH focus was
            // picked, so this one small generic-pool call is a genuine,
            // if cheap, addition).
            push_generic_pool_choice_magnitude(
                input,
                class_level.level,
                ability_modifiers,
                "choice:spiritualist_emotional_focus",
                "Phantom Emotional Focus",
                "focus:",
                "class_feature.occult_adventures.spiritualist.phantom_emotional_focus.generic",
                1,
                explanations,
            );
        } else if class_level.class_id == "class:magus" {
            ground_magus_class_features(class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:shifter" {
            ground_shifter_class_features(input, class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:vigilante" {
            ground_vigilante_class_features(class_level.level, ability_modifiers, explanations);
        } else if class_level.class_id == "class:psion" {
            ground_psion_class_features(input, class_level.level, ability_modifiers, explanations);
        }

        Some((base_attack_bonus, base_saves))
    } else if let Some(row) = generic_class_chassis::resolve(&class_level.class_id, class_level.level) {
        // SD-32 T12 `epic-10-reference-library-residual-reach` row 20 cycle 5: the
        // character-creation-time dispatch arm for all 61 conventional PC classes cycle 4
        // already re-derived a reference-catalog BAB/save TABLE for but never wired a chassis
        // dispatch arm for (Demoniac's own bare-`classlevel()` gap closed on this cycle's own
        // rebase, row 18 cycle 9 — see `generic_class_chassis`'s own module doc). Same shape
        // as the `untabled_base_class_chassis::resolve` arm above: real
        // base attack bonus and all three base saves, computed from the class's own corpus
        // the class record's own converted base-attack/save arithmetic, not a
        // hand-typed table.
        let base_attack_bonus = row.base_attack_bonus;
        let base_saves =
            BaseSaves { fortitude: row.fort_save, reflex: row.ref_save, will: row.will_save };
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_attack_bonus".to_owned(),
            value: base_attack_bonus,
            detail: format!(
                "{} ({}) level {} base attack bonus from \
                 pilot_compute::generic_class_chassis::resolve's corpus-derived formula for this \
                 class: {base_attack_bonus}",
                row.display_name, class_level.class_id, class_level.level
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_save.fortitude".to_owned(),
            value: base_saves.fortitude,
            detail: format!(
                "{} ({}) level {} base Fortitude save from \
                 pilot_compute::generic_class_chassis::resolve's corpus-derived formula for this \
                 class: {}",
                row.display_name, class_level.class_id, class_level.level, base_saves.fortitude
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_save.reflex".to_owned(),
            value: base_saves.reflex,
            detail: format!(
                "{} ({}) level {} base Reflex save from \
                 pilot_compute::generic_class_chassis::resolve's corpus-derived formula for this \
                 class: {}",
                row.display_name, class_level.class_id, class_level.level, base_saves.reflex
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_save.will".to_owned(),
            value: base_saves.will,
            detail: format!(
                "{} ({}) level {} base Will save from \
                 pilot_compute::generic_class_chassis::resolve's corpus-derived formula for this \
                 class: {}",
                row.display_name, class_level.class_id, class_level.level, base_saves.will
            ),
        });
        Some((base_attack_bonus, base_saves))
    } else if let Some(row) =
        crb_untabled_class_chassis::resolve(&class_level.class_id, class_level.level)
    {
        // SD-34 `AT-34-E3-001` (`decisions.md §14`): CRB's five NPC classes
        // and two `Ex-*` variant states (Adept, Aristocrat, Commoner,
        // Expert, Warrior, Ex-Barbarian, Ex-Paladin) -- real corpus records
        // `ClassId::ALL` never carried, evaluated via the class's own
        // corpus `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` formula strings, same
        // shape as the `generic_class_chassis`/`untabled_base_class_
        // chassis` arms above. See `crb_untabled_class_chassis`'s own
        // module doc for why CRB's ten prestige classes are deliberately
        // NOT resolved here.
        let base_attack_bonus = row.base_attack_bonus;
        let base_saves =
            BaseSaves { fortitude: row.fort_save, reflex: row.ref_save, will: row.will_save };
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_attack_bonus".to_owned(),
            value: base_attack_bonus,
            detail: format!(
                "{} ({}) level {} base attack bonus from \
                 pilot_compute::crb_untabled_class_chassis::resolve's corpus-derived formula for \
                 this class: {base_attack_bonus}",
                row.display_name, class_level.class_id, class_level.level
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_save.fortitude".to_owned(),
            value: base_saves.fortitude,
            detail: format!(
                "{} ({}) level {} base Fortitude save from \
                 pilot_compute::crb_untabled_class_chassis::resolve's corpus-derived formula for \
                 this class: {}",
                row.display_name, class_level.class_id, class_level.level, base_saves.fortitude
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_save.reflex".to_owned(),
            value: base_saves.reflex,
            detail: format!(
                "{} ({}) level {} base Reflex save from \
                 pilot_compute::crb_untabled_class_chassis::resolve's corpus-derived formula for \
                 this class: {}",
                row.display_name, class_level.class_id, class_level.level, base_saves.reflex
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.base_save.will".to_owned(),
            value: base_saves.will,
            detail: format!(
                "{} ({}) level {} base Will save from \
                 pilot_compute::crb_untabled_class_chassis::resolve's corpus-derived formula for \
                 this class: {}",
                row.display_name, class_level.class_id, class_level.level, base_saves.will
            ),
        });
        Some((base_attack_bonus, base_saves))
    } else if let Some(gate) =
        prestige_class_entry_gate::evaluate_prestige_class_entry(&class_level.class_id, input)
    {
        // SD-32 Epic 3 (`epic-3-class-reachability`, AT-32-E3-001):
        // real entry-requirement gating for a prestige class this dispatch
        // does not (yet) compute a chassis for. No chassis magnitude is
        // produced -- the caller's existing `class_chassis.unsupported`
        // diagnostic still fires for `None`, same as any other unsupported
        // class id -- but the gate genuinely runs and reports whether the
        // character's real chosen feats/skills/etc. satisfy the class's
        // real corpus PRE-token entry requirements, fixture-checked in
        // `prestige_class_entry_gate`'s own tests.
        diagnostics.push(ComputationDiagnostic {
            id: if gate.qualifies {
                "class_chassis.prestige_entry_gate.met".to_owned()
            } else {
                "class_chassis.prestige_entry_gate.unmet".to_owned()
            },
            message: if gate.qualifies {
                format!(
                    "{} ({}): entry requirements met ({} clause(s) satisfied, {} unmodelled); \
                     chassis magnitude still unsupported (see class_chassis.unsupported)",
                    gate.display_name,
                    class_level.class_id,
                    gate.met.len(),
                    gate.unmodelled.len()
                )
            } else {
                format!(
                    "{} ({}): entry requirements NOT met -- {}",
                    gate.display_name,
                    class_level.class_id,
                    gate.unmet.join("; ")
                )
            },
            claim_blocking: true,
        });
        None
    } else {
        None
    }
}

/// SD-27 (Pathfinder Unchained class wiring, 2026-07-31): compute the
/// base-attack-bonus / base-save chassis pillar for one of the four
/// Unchained classes, then ground that class's own named features.
///
/// Structurally identical to `compute_apg_class_chassis` /
/// `compute_acg_class_chassis` — same explanation ids, same
/// `class_chassis.unsupported` diagnostic shape, same "chassis first, then
/// per-class grounding" order — so nothing downstream has to learn a new
/// path. It is only ever called from `compute_class_chassis`'s
/// single-class-only section (multiclass returns early there), and
/// `PuClassId::from_class_id_str` is deliberately NOT registered with
/// `table_class_id`, so an Unchained class inside a multiclass mix never
/// reaches here and never resolves a CRB row by accident.
///
/// **Three of the four chassis rows are the base class's own row, byte for
/// byte, and that is a corpus fact rather than a shortcut**: the ingested
/// `class` record for the Unchained Barbarian, Rogue and Summoner carries
/// `null` for `hit_die`, `bab` and all three save columns, i.e. the
/// selection ability overrides no chassis field. The Unchained Monk is the
/// one that does override (d10 / full BAB / poor Will). All of that lives
/// in `rules_tables::pathfinder_unchained::class_chassis`, which owns the
/// sourcing and pins it; this function only reads the row it returns.
pub(super) fn compute_pu_class_chassis(
    class_id: PuClassId,
    class_id_str: &str,
    level: u8,
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> Option<(i16, BaseSaves)> {
    let Some(row) = pu_class_chassis::class_chassis_resolve(class_id, level, RuleSetId::Pu) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis has no {class_id_str} Pathfinder Unchained \
                 class_chassis_resolve row at level {level} (exceeds this class's real MAXLEVEL \
                 ceiling), so no chassis values were computed"
            ),
            claim_blocking: true,
        });
        return None;
    };

    let base_attack_bonus = row.base_attack_bonus;
    let base_saves = BaseSaves {
        fortitude: row.fort_save,
        reflex: row.ref_save,
        will: row.will_save,
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "{class_id_str} level {level} base attack bonus from \
             rules_tables::pathfinder_unchained::class_chassis::class_chassis_resolve's row for \
             this class: {base_attack_bonus}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: base_saves.fortitude,
        detail: format!(
            "{class_id_str} level {level} base Fortitude save from \
             rules_tables::pathfinder_unchained::class_chassis::class_chassis_resolve's row for \
             this class: {}",
            base_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: base_saves.reflex,
        detail: format!(
            "{class_id_str} level {level} base Reflex save from \
             rules_tables::pathfinder_unchained::class_chassis::class_chassis_resolve's row for \
             this class: {}",
            base_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: base_saves.will,
        detail: format!(
            "{class_id_str} level {level} base Will save from \
             rules_tables::pathfinder_unchained::class_chassis::class_chassis_resolve's row for \
             this class: {}",
            base_saves.will
        ),
    });

    // The replacement relationship, recorded on every Unchained character's
    // own receipt rather than only in a doc comment: a reader of the sheet
    // can see which class this one stands in for. Value is the class's
    // level, matching the "standalone flat fact" idiom the ACG/APG feature
    // groundings already use for non-numeric statements.
    explanations.push(ComputationExplanation {
        id: format!("class_chassis.pu.{}.replaces", class_id.name()),
        value: i16::from(level),
        detail: format!(
            "{class_id_str} ({}) is Pathfinder Unchained's replacement for {}, not an addition \
             to it: PCGen declares it as the `{}` selection ability in that class's single-slot \
             selection pool, so a character holds one or the other and never both. Both remain \
             separately selectable in this engine under distinct class ids",
            class_id.display_name(),
            class_id.replaces_class_id(),
            class_id.corpus_key(),
        ),
    });

    // The corpus-record roster, emitted before the magnitude groundings so a
    // reader of the sheet meets the class's own feature list first and the
    // derived numbers second. See `push_pu_class_feature_records`.
    push_pu_class_feature_records(class_id, level, ability_modifiers, explanations, diagnostics);

    match class_id {
        PuClassId::UnchainedBarbarian => {
            ground_unchained_barbarian_class_features(
                level,
                input,
                ability_modifiers,
                explanations,
                diagnostics,
            );
        }
        PuClassId::UnchainedMonk => {
            ground_unchained_monk_class_features(
                level,
                base_attack_bonus,
                ability_modifiers,
                &input.chosen.race_id,
                explanations,
                diagnostics,
            );
        }
        PuClassId::UnchainedRogue => {
            ground_unchained_rogue_class_features(level, ability_modifiers, explanations, diagnostics);
        }
        PuClassId::UnchainedSummoner => {
            ground_unchained_summoner_class_features(level, ability_modifiers, explanations, diagnostics);
        }
    }

    // `input` carries the character's race to the Unchained Monk branch above,
    // whose unarmed strike damage die is a column of the shared Core Rulebook
    // record chosen by creature size. Nothing else here reads it: no Unchained
    // feature is choice-gated yet, and this assertion says so out loud rather
    // than the fact being inferable only from the absence of a call.
    debug_assert_eq!(input.chosen.class_levels.len(), 1);

    Some((base_attack_bonus, base_saves))
}

// ---------------------------------------------------------------------------
// Pathfinder Unchained class-feature records, carried by corpus key
// ---------------------------------------------------------------------------

/// The citation this engine appends to every Pathfinder Unchained
/// class-feature receipt row, naming the ingested corpus record the row is
/// about.
///
/// # Why this exists
///
/// `reach_gate.rs` recorded the gap verbatim: PU's 64 ingested `class_feature`
/// records demonstrably influence the character sheet, but *which* of them
/// could not be claimed, "because `pilot_compute` names its receipt rows
/// semantically (`class_feature.pu.unchained_rogue.sneak_attack_dice`) while
/// the corpus record is keyed `Unchained Rogue ~ Sneak Attack`, so nothing can
/// join the two without a hand-written mapping". A mapping written in the gate
/// would be an unexecuted claim — the thing that file exists to refuse. So the
/// key travels **on the receipt**, authored where the row is emitted, and the
/// gate reads it back off the live response.
///
/// Deliberately a function rather than a format string at each call site: the
/// same wording is what [`pu_class_feature_cited_key`] parses, and a citation
/// only one of the two knows how to spell would silently stop being findable.
pub fn pu_class_feature_citation(key: &str, corpus_line: u32) -> String {
    format!(" Corpus record `{key}` (pu_abilities_class.lst:{corpus_line}).")
}

/// Reads the corpus `KEY:` token back out of a receipt row's `detail`, or
/// `None` when the row carries no citation.
///
/// The backtick delimiters are load-bearing rather than decorative: without
/// them `Unchained Barbarian ~ Rage` is a prefix of
/// `Unchained Barbarian ~ Rage Powers`, and a substring match would report the
/// wrong record as reached.
pub fn pu_class_feature_cited_key(detail: &str) -> Option<&str> {
    let rest = detail.split_once("Corpus record `")?.1;
    rest.split_once("` (pu_abilities_class.lst:")
        .map(|(key, _)| key)
}

// ---------------------------------------------------------------------------
// Pathfinder Unchained class-feature descriptions, resolved to this
// character's own numbers
// ---------------------------------------------------------------------------

/// The clause that introduces a resolved corpus description on a Pathfinder
/// Unchained class-feature receipt row.
///
/// Written by [`push_pu_class_feature_records`] and read back by
/// [`pu_resolved_description_from_detail`]. The pair exists for the same reason
/// [`pu_class_feature_citation`] and [`pu_class_feature_cited_key`] do: a marker
/// only one of the two knows how to spell silently stops being findable.
///
/// It names the text as the book's rather than the engine's, because the rest of
/// a `detail` is this engine's own derivation prose and the two must not read as
/// one voice.
pub const PU_RESOLVED_DESCRIPTION_MARKER: &str =
    " Rules text, with this character's own numbers resolved into it: ";

/// Reads the resolved rules text back out of a receipt row's `detail`, or `None`
/// when the row carries none.
pub fn pu_resolved_description_from_detail(detail: &str) -> Option<&str> {
    detail.split_once(PU_RESOLVED_DESCRIPTION_MARKER).map(|(_, text)| text)
}

/// The converted rule id for one Pathfinder Unchained class-feature record key.
///
/// SD-35 `AT-35-E6-003` (`decisions.md` §11, §1). This file used to hold every one of these
/// records' `DESC:` tokens **verbatim** in a `const`, and hand them to the PCGen renderer at run
/// time — the last PCGen token text in this crate's executable code. It holds none now: the
/// converted package at `data/sheet_rules/pathfinder_unchained/class_feature/` already carries
/// the same records' prose as plain-English pieces with typed slots over our own `Expr`, gates
/// and all, and `resolved_prose` renders that. The join is the schema's own
/// [`slug`](crate::rules_core::sheet_rule::slug) of the corpus `KEY:`, which is exactly how the
/// converter named the file — no transcription, so nothing to drift.
///
/// `tests/sd27_pu_class_feature_descriptions_carry_the_characters_numbers.rs` re-derives the
/// population off disk (the PU records carrying a `%N`, 7 of 64), renders each one **both ways**
/// over a level/ability matrix — this path, and the record's own corpus `DESC:` tokens through
/// the tool-side PCGen renderer — and asserts byte-identical text. The oracle stayed where the
/// oracle belongs; only the live reader changed.
pub(super) fn pu_rule_id(record_key: &str) -> String {
    format!(
        "pathfinder_unchained:class_feature:{}",
        crate::rules_core::sheet_rule::slug(record_key)
    )
}


/// The display values one Unchained character has for the PCGen variables its
/// own class-feature descriptions reference.
///
/// **Every value is read from the hand-modelled function that already owns it**
/// — `barbarian_features::rage_rounds_per_day`, `monk_features::ki_points`,
/// `rogue_features::master_strike_dc` and their siblings, the same functions
/// whose results are already pushed as standalone magnitude explanations by
/// `ground_unchained_*_class_features`. Nothing is recomputed here, so a
/// description can never disagree with the magnitude record beside it.
///
/// A feature the character has not reached yet contributes no entry, which leaves
/// its `%N` dropped and reported rather than rendered as a misleading `0`.
///
/// # No feat contribution, deliberately, and what that costs
///
/// `Extra Rage` (`crb/feat_data/general.rs:32`,
/// `BONUS:VAR|RageDuration|6`) and `Extra Ki` (`:28`,
/// `BONUS:VAR|KiPoints|2`) are live catalog feats that move two of these
/// variables. Neither is applied here, because neither is applied to the
/// standalone magnitude row either — `class_feature.pu.unchained_barbarian.
/// rage_rounds_per_day` is `rage_rounds_per_day(level, con)` and nothing else.
/// Adding the feat to the sentence alone would put two different rage-round
/// counts on one sheet, which is `decisions.md §29.2`'s exact defect. Closing it
/// properly needs two new fields on `feat_effects::FeatDisplayValueDeltas` (the
/// §29.1 shared seam for this kind of contribution) consumed by both the sentence
/// and the magnitude row; that file is outside this change's write scope and the
/// gap is reported rather than half-closed.
pub(super) fn pu_display_values(
    class_id: PuClassId,
    level: u8,
    ability_modifiers: &AbilityModifiers,
) -> DisplayValues {
    let mut values = DisplayValues::new();
    let mut set = |name: &str, value: Option<i16>| {
        if let Some(value) = value {
            values.set(name, i64::from(value));
        }
    };

    match class_id {
        PuClassId::UnchainedBarbarian => {
            // `:306` `BONUS:VAR|RageDuration|2+var("STAT.2.MOD.NOTEMP")+(2*RageLVL)`
            // with `:290` `BONUS:VAR|RageLVL|BarbarianLVL`.
            set(
                "RageDuration",
                barbarian_features::rage_rounds_per_day(level, ability_modifiers.constitution),
            );
            // `:306` `BONUS:VAR|RageBonus|2`, plus `:294` and `:296`'s `|1` each.
            set("RageBonus", barbarian_features::rage_morale_bonus(level));
            // `:306` `BONUS:VAR|RageACPenalty|-2`.
            set("RageACPenalty", barbarian_features::rage_armor_class_penalty(level));
            // `:306` `BONUS:VAR|RageBonusHP|TL*2`, plus `:294` and `:296`'s
            // `|TL` each. Single-class only on this path, so character level ==
            // class level — the identical note
            // `ground_unchained_barbarian_class_features` already carries for
            // the same call.
            set("RageBonusHP", barbarian_features::rage_temporary_hit_points(level, level));
            // `:293` `BONUS:VAR|BarbarianDR|(BarbarianDRLVL-4)/3` with the same
            // row's `BONUS:VAR|BarbarianDRLVL|BarbarianLVL`.
            set("BarbarianDR", barbarian_features::damage_reduction(level));
        }
        PuClassId::UnchainedMonk => {
            // `:467` `BONUS:VAR|KiPoolLVL|MonkLVL` feeding
            // `core_rulebook/cr_abilities_class.lst:1175`'s
            // `BONUS:VAR|KiPoints|KiPoolLVL/2` and `:1179`'s
            // `BONUS:VAR|KiPoints|WIS`. Wisdom is passed for the same
            // corpus-declared-choice reason `ground_unchained_monk_class_features`
            // records at its own `ki_points` call.
            set("KiPoints", monk_features::ki_points(level, ability_modifiers.wisdom));
        }
        PuClassId::UnchainedRogue => {
            // `:590` `BONUS:VAR|TrapfindingBonus|max(TrapfindingLVL/2,1)`.
            set("TrapfindingBonus", rogue_features::trapfinding_bonus(level));
            // `:586` `BONUS:VAR|MasterStrikeDC|10+(MasterStrikeLVL/2)+INT`.
            set(
                "MasterStrikeDC",
                rogue_features::master_strike_dc(level, ability_modifiers.intelligence),
            );
            // `:588` `BONUS:VAR|RoguesEdgeLVL|RogueLVL/5`. Also the gate
            // variable for that record's two mutually exclusive prose branches.
            set(
                "RoguesEdgeLVL",
                rogue_features::rogues_edge_skill_unlocks(level).map(i16::from),
            );
            // SD-35 `AT-35-E6-003`. The converter did not leave `RoguesEdgeLVL` as an opaque
            // name: it folded that row's own one-step chain and wrote the record's slot and
            // both its gates as `Rogue LVL / 5` over the class level itself
            // (`unchained_rogue_rogues_edge.json`). So the class level is the value the
            // converted prose actually asks for, and it is seeded beside — not instead of —
            // the hand-modelled unlock count above, which the standalone magnitude row still
            // reads. Single-class on this path, so character level == rogue level, the same
            // note `rage_temporary_hit_points` already carries for the same reason.
            set("RogueLVL", Some(i16::from(level)));
        }
        // The Unchained Summoner's own records carry no `%N` at all
        // (`data/corpus/pathfinder_unchained/class_feature/summoner_unchained_class/`),
        // so it contributes no display values rather than an empty-looking
        // special case.
        PuClassId::UnchainedSummoner => {}
    }

    values
}

/// This record's corpus description with **this character's numbers in it**, or
/// `None` when the record carries no resolvable `%N` description.
///
/// Returns `None` rather than a partially-resolved sentence when any argument is
/// still unresolved: a description that has lost a number reads as a defect
/// (*"You can rage for rounds per day"*), and the roster line without it is
/// honest where the mangled sentence is not. The un-rendered case is exactly what
/// a character below the feature's grant level hits.
pub(super) fn pu_resolved_description(record_key: &str, values: &DisplayValues) -> Option<String> {
    // SD-35 `AT-35-E6-003`. Was: find the record's verbatim `DESC:` tokens in a live `const` and
    // call `render_pcgen_desc_tokens` on them. Now: the converted rule's own prose, rendered by
    // `resolved_prose`, which reads no ingest format at all. The "`None` rather than a
    // partially-resolved sentence" contract this function's doc states is unchanged — it is
    // `resolved_description`'s own contract now, stated as a return value rather than as a
    // `dropped_args` report.
    //
    // The PCGen-syntax `debug_assert!` that used to stand here is gone with the renderer, and is
    // not weakened by its absence: `data/sheet_rules/` is swept corpus-wide for ingest syntax by
    // this bundle's own release gate (`grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' →  0`),
    // which is a wider check than one assertion on one rendered string.
    resolved_description(&pu_rule_id(record_key), values)
}

/// One ingested Pathfinder Unchained `class_feature` record, normalised across
/// the four per-class table shapes.
///
/// The four tables are genuinely different shapes — Barbarian and Monk expose a
/// `pub struct` roster, Rogue and Summoner a `pub enum` with accessor methods,
/// and `min_level` is `Option<u8>` on two of them and `u8` on the other two.
/// Normalising here rather than reshaping four corpus-pinned tables keeps this
/// change inside the seam it belongs in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PuClassFeatureRecord {
    /// The corpus `KEY:` token, verbatim.
    key: &'static str,
    /// The corpus row's display name (its first column).
    name: &'static str,
    /// The class level a progression row first grants this record at. `None`
    /// means the row states no `PREVARGTEQ:` level gate — for a granted record
    /// that means "from level 1", and for an ungranted one there is no level to
    /// state.
    min_level: Option<u8>,
    /// Whether any progression row grants this record at all. Five of the 64
    /// are declared and never granted; that is a real corpus fact, not an
    /// ingest gap.
    is_granted: bool,
    /// 1-based line in `pu_abilities_class.lst`.
    corpus_line: u32,
}

/// Every ingested `class_feature` record for one Unchained class, read live off
/// that class's own corpus-pinned table.
pub(super) fn pu_class_feature_records(class_id: PuClassId) -> Vec<PuClassFeatureRecord> {
    match class_id {
        PuClassId::UnchainedBarbarian => barbarian_features::features()
            .iter()
            .map(|feature| PuClassFeatureRecord {
                key: feature.key,
                name: feature.name,
                min_level: feature.min_level,
                is_granted: feature.is_granted,
                corpus_line: feature.corpus_line,
            })
            .collect(),
        // Every one of the Monk's 18 records is granted by the single
        // `Monk ~ Unchained Class.MOD` progression block, so the table carries
        // no `is_granted` flag to read — there is no ungranted case to model.
        PuClassId::UnchainedMonk => monk_features::features()
            .iter()
            .map(|feature| PuClassFeatureRecord {
                key: feature.key,
                name: feature.name,
                min_level: Some(feature.min_level),
                is_granted: true,
                corpus_line: feature.corpus_line,
            })
            .collect(),
        PuClassId::UnchainedRogue => rogue_features::UnchainedRogueFeature::ALL
            .iter()
            .map(|feature| PuClassFeatureRecord {
                key: feature.key(),
                name: feature.name(),
                min_level: feature.min_level(),
                // The Rogue table states the same fact as the Barbarian's
                // `is_granted` flag through `min_level`: `None` there is
                // documented as "no progression row grants either one".
                is_granted: feature.min_level().is_some(),
                corpus_line: feature.declaring_line(),
            })
            .collect(),
        PuClassId::UnchainedSummoner => summoner_features::UnchainedSummonerFeature::ALL
            .iter()
            .map(|feature| PuClassFeatureRecord {
                key: feature.key(),
                name: feature.name(),
                min_level: Some(feature.min_level()),
                is_granted: true,
                corpus_line: feature.declaring_line(),
            })
            .collect(),
    }
}

/// The receipt-id segment for one record: its corpus key with the
/// `<Class> ~ ` prefix dropped and the rest slugged.
///
/// Dropping the prefix is what keeps the id readable — the id already names the
/// class two segments earlier, and `classFeaturesModel.ts` humanises the
/// remaining segments into the row's on-screen label. Uniqueness within a class
/// is not assumed: `pu_class_feature_receipt_ids_are_unique_within_each_class`
/// asserts it over all four live rosters.
pub(super) fn pu_feature_slug(key: &str) -> String {
    let tail = key.rsplit(" ~ ").next().unwrap_or(key);
    let mut slug = String::with_capacity(tail.len());
    let mut last_was_separator = false;
    for character in tail.chars() {
        if character.is_ascii_alphanumeric() {
            slug.extend(character.to_lowercase());
            last_was_separator = false;
        } else if is_intraword_punctuation(character) {
            // Swallowed, not separated. See `is_intraword_punctuation`.
        } else if !last_was_separator && !slug.is_empty() {
            slug.push('_');
            last_was_separator = true;
        }
    }
    slug.trim_end_matches('_').to_owned()
}

/// Punctuation an id slug **swallows** instead of turning into a `_`.
///
/// # The defect this closes
///
/// An apostrophe sits inside a word, so promoting it to a separator splits the
/// word in two. `classFeaturesModel.ts::humanise` then splits the id on
/// `[\s._]+` and title-cases every part, so the orphaned letter is capitalised
/// on its own: `Unchained Summoner ~ Maker's Call` became
/// `…corpus_record.maker_s_call` and rendered on the character sheet as
/// **"Maker S Call"**. `Scavenger's Eye` did the same through
/// [`slugify_id_segment`].
///
/// Dropping the character instead yields `makers_call` -> "Makers Call", which
/// is the convention [`class_feature_id_slug`] already used for the Advanced
/// Class Guide's deeds (`Swashbuckler's Edge` -> `swashbucklers_edge`). This is
/// that rule, shared, rather than a third spelling of it.
///
/// Both the ASCII apostrophe and the Unicode right single quotation mark are
/// swallowed: PCGen writes ASCII in every row this repo has ingested, and a
/// future row arriving with the typographic form would otherwise reopen the
/// defect silently.
///
/// Deliberately *not* generalised to all punctuation. A parenthesis or a
/// hyphen separates words (`Craft (Alchemy)`, `Kip-Up`) and must stay a
/// separator; only the apostrophe family sits mid-word.
pub(super) fn is_intraword_punctuation(character: char) -> bool {
    matches!(character, '\'' | '\u{2019}')
}

/// Emits one receipt row per ingested Pathfinder Unchained `class_feature`
/// record this character actually holds, each carrying the record's own corpus
/// key.
///
/// # What a player gets that they did not have before
///
/// The magnitude groundings below this function each explain one *number* —
/// Rage's rounds per day, the Rogue's sneak-attack dice. Several of them derive
/// from a single corpus record (four rows come out of
/// `Unchained Barbarian ~ Rage` alone), and many corpus records state no number
/// at all and so produced no row whatsoever. The sheet's Class Features section
/// was therefore a list of derived magnitudes rather than a list of the
/// character's class features: an Unchained Monk 20 saw nothing named
/// "Timeless Body" or "Tongue of the Sun and Moon" anywhere.
///
/// These rows are that roster, one per record, with the grant level the corpus
/// states and a citation of the record itself.
///
/// # Which rows are emitted, and which deliberately are not
///
/// * **Granted, and this character has reached its level** — a roster row.
/// * **Granted, but above this character's level** — no row. A level-1 Rogue
///   does not have Master Strike, and listing a feature the character has not
///   got would be the same overstatement the rest of this file avoids.
/// * **Never granted by any progression row** (5 of the 64) — an
///   `.unsupported` row, which `classFeaturesModel.ts` routes to the sheet's
///   "Not computed" lane. These are declared machinery records whose effect the
///   book reaches through a sibling record the class *does* grant (the Uncanny
///   Dodge Tracker drives both Uncanny Dodge rows;
///   `Unchained Barbarian ~ Rage`'s own
///   `ABILITY:Special Ability|AUTOMATIC|Unchained Rage` reaches
///   `Unchained Rage`). Saying that on the sheet is honest; silently dropping
///   them would leave five ingested records reaching nothing.
pub(super) fn push_pu_class_feature_records(
    class_id: PuClassId,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let class_name = class_id.name();
    let display_name = class_id.display_name();
    // Resolved once per class rather than per record: every entry comes from a
    // hand-modelled function of this character's level and ability modifiers, so
    // it is the same table for all 64 rows.
    let display_values = pu_display_values(class_id, level, ability_modifiers);

    for record in pu_class_feature_records(class_id) {
        let citation = pu_class_feature_citation(record.key, record.corpus_line);
        let slug = pu_feature_slug(record.key);
        // Empty for the 57 records stating no `%N`, and for any record whose
        // variable this character has not reached. See `pu_resolved_description`
        // for why a partial sentence is never emitted.
        let rules_text = match pu_resolved_description(record.key, &display_values) {
            Some(text) => format!("{PU_RESOLVED_DESCRIPTION_MARKER}{text}"),
            None => String::new(),
        };

        if !record.is_granted {
            push_deferred_class_features(
                &format!("class_feature.pu.{class_name}.corpus_record.{slug}.unsupported"),
                format!(
                    "{display_name}: `{}` is declared by Pathfinder Unchained but no progression \
                     row grants it a level of its own, so nothing is computed for the record \
                     directly. The book reaches its effect through a sibling record this class \
                     does grant, whose own rows carry the magnitude.{citation}{rules_text}",
                    record.name
                ),
                explanations,
                diagnostics,
            );
            continue;
        }

        // `None` on a granted record means the progression row states no level
        // gate, which PCGen applies from the class's first level.
        let granted_at = record.min_level.unwrap_or(1);
        if level < granted_at {
            continue;
        }

        explanations.push(ComputationExplanation {
            id: format!("class_feature.pu.{class_name}.corpus_record.{slug}"),
            value: i16::from(granted_at),
            detail: format!(
                "{display_name} level {level}: `{}` is a class feature of this character, granted \
                 from class level {granted_at}.{citation}{rules_text}",
                record.name
            ),
        });
    }
}

/// Emits one receipt row per granted, level-reached record in
/// [`untabled_base_class_feature_roster::roster_for`] for `class_id_str`
/// (the `"class:<name>"` form `compute_class_chassis` dispatches with).
///
/// SD-32 card 11 (T12): the generic form of [`push_pu_class_feature_records`]
/// above. That function's own four Rust tables are hand-curated per class;
/// this one reads a single corpus-derived fixture keyed by class id and
/// pushes the SAME id shape (`class_feature.untabled.<class>.corpus_record.
/// <slug>`), so a class costs nothing here beyond adding its own rows to the
/// fixture — no new Rust match arm, no new push function. Reuses
/// [`pu_feature_slug`] for the id's slug segment: the transform is not
/// Pathfinder-Unchained-specific despite the name (`class_feature_engine_
/// join_slug` in `v06_work_inventory.rs` is the identical rule, duplicated
/// there because that file is a `bin`, not a library, and cannot import
/// this one).
///
/// A class absent from the fixture (18 of the 20 registry entries, as of
/// this cycle — see the fixture's own doc comment) simply gets no rows here,
/// same as before this function existed; no fabricated row is ever emitted.
pub(super) fn push_untabled_base_class_feature_records(
    class_id_str: &str,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let bare_class_id = class_id_str.strip_prefix("class:").unwrap_or(class_id_str);
    for record in untabled_base_class_feature_roster::roster_for(bare_class_id) {
        if level < record.min_level {
            continue;
        }
        let slug = pu_feature_slug(&record.key);
        explanations.push(ComputationExplanation {
            id: format!("class_feature.untabled.{bare_class_id}.corpus_record.{slug}"),
            value: i16::from(record.min_level),
            detail: format!(
                "{class_id_str} level {level}: `{}` is a class feature of this character, \
                 granted from class level {} (source: {}:{}).",
                record.name, record.min_level, record.source_file, record.source_line
            ),
        });
    }
}

/// Grounds the seven magnitude-bearing Antipaladin features
/// (`rules_tables::apg::antipaladin_features`) — the first `untabled_base_
/// class_feature_roster` group given a real per-feature compute function
/// rather than left "named, not attempted" (SD-32 card 11, T12).
///
/// Every magnitude below is that module's own pure function; nothing is
/// recomputed here, and a feature the character has not yet reached simply
/// emits no record (`None` below its grant level), the same contract
/// `ground_unchained_barbarian_class_features` uses.
pub(super) fn ground_antipaladin_class_features(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::apg::antipaladin_features as af;
    let cha = ability_modifiers.charisma;

    if let Some(uses) = af::touch_of_corruption_uses_per_day(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.touch_of_corruption.uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Antipaladin level {level} Touch of Corruption: {uses} uses per day \
                 (level/2 + Charisma modifier {cha}), identical formula to Paladin's Lay on Hands"
            ),
        });
    }
    if let Some(dice) = af::touch_of_corruption_damage_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.touch_of_corruption.damage_dice".to_owned(),
            value: dice,
            detail: format!(
                "Antipaladin level {level} Touch of Corruption: {dice}d6 damage (to a living \
                 target) or healing (to an undead target), level/2"
            ),
        });
    }
    if let Some(bonus) = af::unholy_resilience_save_bonus(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.unholy_resilience.save_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Antipaladin level {level} Unholy Resilience: +{bonus} on all saving throws \
                 (Charisma modifier {cha}, floored at 0)"
            ),
        });
    }
    if let Some(dc) = af::cruelty_dc(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.cruelty.dc".to_owned(),
            value: dc,
            detail: format!(
                "Antipaladin level {level} Cruelty: DC {dc} Fortitude save to resist an applied \
                 cruelty's effect (10 + Charisma modifier {cha} + level/2)"
            ),
        });
    }
    if let Some(known) = af::cruelties_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.cruelty.known".to_owned(),
            value: known,
            detail: format!(
                "Antipaladin level {level} Cruelty: {known} cruelties known (one every 3 levels \
                 from 3rd, capped at 6)"
            ),
        });
    }
    if let Some(dice) = af::channel_negative_energy_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.channel_negative_energy.dice".to_owned(),
            value: dice,
            detail: format!(
                "Antipaladin level {level} Channel Negative Energy: {dice}d{} damage/healing \
                 ((level+1)/2 dice)",
                af::CHANNEL_NEGATIVE_ENERGY_DIE_SIZE
            ),
        });
    }
    if let Some(dc) = af::channel_negative_energy_dc(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.channel_negative_energy.dc".to_owned(),
            value: dc,
            detail: format!(
                "Antipaladin level {level} Channel Negative Energy: DC {dc} Will save to halve \
                 (10 + level/2 + Charisma modifier {cha})"
            ),
        });
    }
    if let Some(selections) = af::fiendish_boon_selections(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.fiendish_boon.selections".to_owned(),
            value: selections,
            detail: format!(
                "Antipaladin level {level} Fiendish Boon: {selections} selection(s) (one at 5th \
                 level, one more every 4 levels thereafter, capped at 4)"
            ),
        });
    }
    if let Some(dr) = af::aura_of_depravity_damage_reduction(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.aura_of_depravity.damage_reduction"
                .to_owned(),
            value: dr,
            detail: format!("Antipaladin level {level} Aura of Depravity: DR {dr}/good"),
        });
    }
    if let Some(cl) = af::unholy_champion_banishment_caster_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.unholy_champion.banishment_caster_level"
                .to_owned(),
            value: cl,
            detail: format!(
                "Antipaladin level {level} Unholy Champion: Banishment caster level {cl} on a \
                 successful Smite Good against a good outsider (grounds the RAW-correct \
                 antipaladin level; the upstream PCGen token reads a Paladin-only variable that \
                 is never set on an Antipaladin -- decisions.md §22, inherited but not \
                 perpetuated)"
            ),
        });
    }

    // SD-32 card 11 (T12) follow-up: three magnitude-bearing records the
    // `psion` cycle's widened census surfaced on this class -- see
    // `antipaladin_features`'s own module doc comment.
    if let Some(tier) = af::aura_of_evil_strength_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.aura_of_evil.strength_level".to_owned(),
            value: tier,
            detail: format!(
                "Antipaladin level {level} Aura of Evil: aura strength level {tier} \
                 (a pure class-level pass-through selecting one of four DESC-prose tiers: \
                 faint at 1, moderate at 2-4, strong at 5-10, overwhelming at 11+)"
            ),
        });
    }
    if let Some(cl) = af::detect_good_caster_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.detect_good.caster_level".to_owned(),
            value: cl,
            detail: format!(
                "Antipaladin level {level} Detect Good: at-will spell-like ability, caster \
                 level {cl} (a pure class-level pass-through)"
            ),
        });
    }
    if let Some(uses) = af::smite_good_uses_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.smite_good.uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Antipaladin level {level} Smite Good: {uses} uses per day \
                 (min((level+2)/3, 7))"
            ),
        });
    }
    if let Some(bonus) = af::smite_good_attack_and_ac_bonus(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.smite_good.attack_and_ac_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Antipaladin level {level} Smite Good: +{bonus} on attack rolls against the \
                 smite's target and +{bonus} deflection bonus to AC against it (Charisma \
                 modifier {cha}, floored at 0; one token sets both)"
            ),
        });
    }
    if let Some(dmg) = af::smite_good_damage_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.antipaladin.smite_good.damage_bonus".to_owned(),
            value: dmg,
            detail: format!(
                "Antipaladin level {level} Smite Good: +{dmg} damage on the smite's target \
                 (+{} against a good outsider, good-aligned dragon, or good cleric/paladin -- \
                 the record's own %4 = SmiteGoodDamageBonus*2)",
                dmg * 2
            ),
        });
    }
}

/// Grounds Cryptic's six magnitude-bearing features
/// (`rules_tables::ultimate_psionics::cryptic_features`) — SD-32 card 11
/// (T12), the second class attempted end-to-end after Antipaladin.
pub(super) fn ground_cryptic_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_psionics::cryptic_features as cf;
    let int_mod = ability_modifiers.intelligence;
    let int_score = input.chosen.ability_scores.intelligence;

    // SD-32 card 11 (T12) follow-up: `Cryptic Manifesting`'s shape-3
    // magnitudes, surfaced by the `psion` cycle's widened census.
    if let Some(v) = cf::cryptic_power_points_total(level, int_mod) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.cryptic.cryptic_manifesting.power_points".to_owned(),
            value: v,
            detail: format!(
                "Cryptic level {level} Cryptic Manifesting: {v} power points (base ladder + \
                 (Intelligence modifier {int_mod} * level)/2)"
            ),
        });
    }
    if let Some(v) = cf::cryptic_powers_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.cryptic.cryptic_manifesting.powers_known".to_owned(),
            value: v,
            detail: format!(
                "Cryptic level {level} Cryptic Manifesting: {v} powers known (equal to level)"
            ),
        });
    }
    if let Some(v) = cf::cryptic_max_power_level(level, int_score) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.cryptic.cryptic_manifesting.max_power_level".to_owned(),
            value: v,
            detail: format!(
                "Cryptic level {level} Cryptic Manifesting: maximum power level known {v} \
                 (min(6, floor((level+2)/3), Intelligence score {int_score} - 10))"
            ),
        });
    }

    if let Some(dr) = cf::altered_defense_damage_reduction(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.cryptic.altered_defense.damage_reduction".to_owned(),
            value: dr,
            detail: format!(
                "Cryptic level {level} Altered Defense (Absorb): DR {dr}/- \
                 (floor((level+3)/4))"
            ),
        });
    }
    if let Some(range) = cf::disrupt_pattern_range_feet(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.cryptic.disrupt_pattern.range_feet".to_owned(),
            value: range,
            detail: format!("Cryptic level {level} Disrupt Pattern: {range}-foot range (flat)"),
        });
    }
    if let Some(dice) = cf::enhanced_disruption_bonus_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.cryptic.enhanced_disruption.bonus_dice".to_owned(),
            value: dice,
            detail: format!(
                "Cryptic level {level} Enhanced Disruption: +{dice} bonus disrupt pattern \
                 dice (floor((level-1)/2))"
            ),
        });
    }
    if let Some(bonus) = cf::hidden_pattern_stealth_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.cryptic.hidden_pattern.stealth_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Cryptic level {level} Hidden Pattern: +{bonus} competence bonus on Stealth \
                 (2*min(3,floor((level+1)/3)))"
            ),
        });
    }
    if let Some(bonus) = cf::trapmaker_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.cryptic.trapmaker.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Cryptic level {level} Trapmaker: +{bonus} competence bonus on Craft (traps) \
                 (equal to level)"
            ),
        });
    }
    if let Some(pr) = cf::unchanging_pattern_power_resistance(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.cryptic.unchanging_pattern.power_resistance".to_owned(),
            value: pr,
            detail: format!(
                "Cryptic level {level} Unchanging Pattern: power resistance {pr} (12+level)"
            ),
        });
    }
}

/// Grounds Dread's six magnitude-bearing features
/// (`rules_tables::ultimate_psionics::dread_features`) — SD-32 card 11
/// (T12), the third class attempted end-to-end.
pub(super) fn ground_dread_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_psionics::dread_features as df;
    let cha = ability_modifiers.charisma;
    let cha_score = input.chosen.ability_scores.charisma;

    // SD-32 card 11 (T12) follow-up: `Dread Manifesting`'s shape-3
    // magnitudes, surfaced by the `psion` cycle's widened census.
    if let Some(v) = df::dread_power_points_total(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.dread.dread_manifesting.power_points".to_owned(),
            value: v,
            detail: format!(
                "Dread level {level} Dread Manifesting: {v} power points (base ladder + \
                 (Charisma modifier {cha} * level)/2)"
            ),
        });
    }
    if let Some(v) = df::dread_powers_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.dread.dread_manifesting.powers_known".to_owned(),
            value: v,
            detail: format!(
                "Dread level {level} Dread Manifesting: {v} powers known (equal to level)"
            ),
        });
    }
    if let Some(v) = df::dread_max_power_level(level, cha_score) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.dread.dread_manifesting.max_power_level".to_owned(),
            value: v,
            detail: format!(
                "Dread level {level} Dread Manifesting: maximum power level known {v} \
                 (min(6, floor((level+2)/3), Charisma score {cha_score} - 10))"
            ),
        });
    }

    if let Some(dmg) = df::devastating_touch_bonus_damage(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.dread.devastating_touch.bonus_damage".to_owned(),
            value: dmg,
            detail: format!(
                "Dread level {level} Devastating Touch: +{dmg} bonus damage on the melee \
                 touch attack (equal to level, added to a flat 1d6)"
            ),
        });
    }
    if let Some(bonus) = df::fearsome_insight_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.dread.fearsome_insight.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Dread level {level} Fearsome Insight: +{bonus} insight bonus on Intimidate \
                 (max(1,floor(level/2)))"
            ),
        });
    }
    if let Some(uses) = df::terror_uses_per_day(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.dread.terror.uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Dread level {level} Terror: {uses} uses per day (level + Charisma modifier \
                 {cha})"
            ),
        });
    }
    if let Some(penalty) = df::aura_of_fear_penalty(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.dread.aura_of_fear.penalty".to_owned(),
            value: penalty,
            detail: format!(
                "Dread level {level} Aura of Fear: {penalty} penalty on nearby enemies' saves \
                 against fear (flat)"
            ),
        });
    }
    if let Some(uses) = df::shadow_twin_uses_per_day(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.dread.shadow_twin.uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Dread level {level} Shadow Twin: {uses} uses per day (Charisma modifier {cha} \
                 only, no level term)"
            ),
        });
    }
    if let Some(dr) = df::fear_incarnate_damage_reduction(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.dread.fear_incarnate.damage_reduction".to_owned(),
            value: dr,
            detail: format!("Dread level {level} Fear Incarnate: DR {dr}/psionic (flat)"),
        });
    }
}

/// Grounds Marksman's five magnitude-bearing features
/// (`rules_tables::ultimate_psionics::marksman_features`) — SD-32 card 11
/// (T12), the fourth class attempted end-to-end.
pub(super) fn ground_marksman_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_psionics::marksman_features as mf;
    let dex = ability_modifiers.dexterity;
    let wis = ability_modifiers.wisdom;
    let wis_score = input.chosen.ability_scores.wisdom;

    // SD-32 card 11 (T12) follow-up: `Marksman Manifesting`'s shape-3
    // magnitudes, surfaced by the `psion` cycle's widened census.
    if let Some(v) = mf::marksman_power_points_total(level, wis) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.marksman.marksman_manifesting.power_points".to_owned(),
            value: v,
            detail: format!(
                "Marksman level {level} Marksman Manifesting: {v} power points (base ladder + \
                 (Wisdom modifier {wis} * level)/2)"
            ),
        });
    }
    if let Some(v) = mf::marksman_powers_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.marksman.marksman_manifesting.powers_known".to_owned(),
            value: v,
            detail: format!(
                "Marksman level {level} Marksman Manifesting: {v} powers known \
                 (min(9,floor((3*level-1)/4)), plus floor((level-13)/2) summed in once \
                 level >= 15 -- the two same-target bonus rows sum, the converter-side \
                 bonus-stack reader's documented semantics)"
            ),
        });
    }
    if let Some(v) = mf::marksman_max_power_level(level, wis_score) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.marksman.marksman_manifesting.max_power_level"
                .to_owned(),
            value: v,
            detail: format!(
                "Marksman level {level} Marksman Manifesting: maximum power level known {v} \
                 (0 below level 2; min(4, floor((level+3)/4), Wisdom score {wis_score} - 10) \
                 from level 2)"
            ),
        });
    }

    if let Some(uses) = mf::wind_reader_uses_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.marksman.wind_reader.uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Marksman level {level} Wind Reader: {uses} uses per day (3+level)"
            ),
        });
    }
    if let Some(bonus) = mf::evade_arrows_ac_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.marksman.evade_arrows.ac_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Marksman level {level} Evade Arrows: +{bonus} AC vs. ranged attacks \
                 ((level+2)/4)"
            ),
        });
    }
    if let Some(bonus) = mf::favored_weapon_base_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.marksman.favored_weapon.base_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Marksman level {level} Favored Weapon: +{bonus} base competence bonus \
                 ((level+2)/4)"
            ),
        });
    }
    if let Some(dc) = mf::cover_fire_dc(level, dex) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.marksman.cover_fire.dc".to_owned(),
            value: dc,
            detail: format!(
                "Marksman level {level} Cover Fire: DC {dc} (10 + Dexterity modifier {dex} + \
                 level/2)"
            ),
        });
    }
    if let Some(bonus) = mf::ranged_specialist_critical_multiplier_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.marksman.ranged_specialist.critical_multiplier_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Marksman level {level} Ranged Specialist: +{bonus} critical multiplier on \
                 ranged/thrown weapons (flat)"
            ),
        });
    }
}

/// Grounds Psychic Warrior's three magnitude-bearing features
/// (`rules_tables::ultimate_psionics::psychic_warrior_features`) — SD-32
/// card 11 (T12), the fifth class attempted end-to-end.
pub(super) fn ground_psychic_warrior_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_psionics::psychic_warrior_features as pwf;
    let wis = ability_modifiers.wisdom;
    let wis_score = input.chosen.ability_scores.wisdom;

    // SD-32 card 11 (T12) follow-up: `Psychic Warrior Manifesting`'s
    // shape-3 magnitudes, surfaced by the `psion` cycle's widened census.
    if let Some(v) = pwf::psychic_warrior_power_points_total(level, wis) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psychic_warrior.psychic_warrior_manifesting.\
power_points"
                .to_owned(),
            value: v,
            detail: format!(
                "Psychic Warrior level {level} Psychic Warrior Manifesting: {v} power \
                 points (base ladder + (Wisdom modifier {wis} * level)/2)"
            ),
        });
    }
    if let Some(v) = pwf::psychic_warrior_powers_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psychic_warrior.psychic_warrior_manifesting.\
powers_known"
                .to_owned(),
            value: v,
            detail: format!(
                "Psychic Warrior level {level} Psychic Warrior Manifesting: {v} powers \
                 known (equal to level)"
            ),
        });
    }
    if let Some(v) = pwf::psychic_warrior_max_power_level(level, wis_score) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psychic_warrior.psychic_warrior_manifesting.\
max_power_level"
                .to_owned(),
            value: v,
            detail: format!(
                "Psychic Warrior level {level} Psychic Warrior Manifesting: maximum power \
                 level known {v} (min(6, floor((level+2)/3), Wisdom score {wis_score} - 10))"
            ),
        });
    }

    if let Some(lvl) = pwf::warriors_path_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psychic_warrior.warriors_path.level".to_owned(),
            value: lvl,
            detail: format!(
                "Psychic Warrior level {level} Warrior's Path: tracked path level {lvl} \
                 (equal to class level)"
            ),
        });
    }
    if let Some(uses) = pwf::pathweaving_uses_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psychic_warrior.pathweaving.uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Psychic Warrior level {level} Pathweaving: {uses} uses per day \
                 ((level-12)/3)"
            ),
        });
    }
    if let Some(uses) = pwf::eternal_warrior_uses_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psychic_warrior.eternal_warrior.uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Psychic Warrior level {level} Eternal Warrior: {uses} use per day (flat)"
            ),
        });
    }
}

/// Grounds Soulknife's four magnitude-bearing features
/// (`rules_tables::ultimate_psionics::soulknife_features`) — SD-32 card 11
/// (T12), the sixth class attempted end-to-end.
pub(super) fn ground_soulknife_class_features(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    use crate::rules_core::rules_tables::ultimate_psionics::soulknife_features as skf;

    if let Some(lvl) = skf::form_mind_blade_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.soulknife.form_mind_blade.level".to_owned(),
            value: lvl,
            detail: format!(
                "Soulknife level {level} Form Mind Blade: tracked mind blade level {lvl} \
                 (equal to class level)"
            ),
        });
    }
    if let Some(bonus) = skf::enhanced_mind_blade_max_enhancement_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.soulknife.enhanced_mind_blade.max_enhancement_bonus"
                .to_owned(),
            value: bonus,
            detail: format!(
                "Soulknife level {level} Enhanced Mind Blade: max enhancement bonus +{bonus} \
                 (min(level/3,5))"
            ),
        });
    }
    if let Some(size) = skf::psychic_strike_die_size(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.soulknife.psychic_strike.die_size".to_owned(),
            value: size,
            detail: format!(
                "Soulknife level {level} Psychic Strike: d{size} damage die size (flat)"
            ),
        });
    }
    if let Some(uses) = skf::quick_draw_uses_per_round(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.soulknife.quick_draw.uses_per_round".to_owned(),
            value: uses,
            detail: format!(
                "Soulknife level {level} Quick Draw: {uses} free-action manifestation per \
                 round (flat)"
            ),
        });
    }
}

/// Grounds Aegis's seven magnitude-bearing features
/// (`rules_tables::ultimate_psionics::aegis_features`) — SD-32 card 11
/// (T12), cycle 3, the sixth class of `ultimate_psionics` attempted.
pub(super) fn ground_aegis_class_features(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_psionics::aegis_features as agf;
    let int = ability_modifiers.intelligence;

    if let Some(hp) = agf::astral_repair_hp(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.aegis.astral_repair.hp".to_owned(),
            value: hp,
            detail: format!("Aegis level {level} Astral Repair: {hp} hit points restored (flat)"),
        });
    }
    if let Some(dr) = agf::damage_reduction(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.aegis.damage_reduction.value".to_owned(),
            value: dr,
            detail: format!(
                "Aegis level {level} Damage Reduction: DR {dr}/- while wearing the astral suit \
                 (floor((level+4)/3))"
            ),
        });
    }
    if let Some(points) = agf::form_astral_suit_custom_points(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.aegis.form_astral_suit.custom_points".to_owned(),
            value: points,
            detail: format!(
                "Aegis level {level} Form Astral Suit: {points} customization points \
                 (2+level+floor((level+1)/5))"
            ),
        });
    }
    if let Some(bonus) = agf::craftsman_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.aegis.craftsman.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Aegis level {level} Craftsman: +{bonus} bonus on a chosen Craft skill \
                 (floor((level+2)/4))"
            ),
        });
    }
    if let Some(times) = agf::reconfigure_times_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.aegis.reconfigure.times_per_day".to_owned(),
            value: times,
            detail: format!(
                "Aegis level {level} Reconfigure: {times} uses per day (floor((level-1)/2))"
            ),
        });
    }
    if let Some(duration) = agf::augment_suit_duration_rounds(level, int) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.aegis.augment_suit.duration_rounds".to_owned(),
            value: duration,
            detail: format!(
                "Aegis level {level} Augment Suit: {duration} rounds duration (Intelligence \
                 modifier {int} only, no level term)"
            ),
        });
    }
    if let Some(times) = agf::cannibalize_suit_times_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.aegis.cannibalize_suit.times_per_day".to_owned(),
            value: times,
            detail: format!(
                "Aegis level {level} Cannibalize Suit: {times} uses per day \
                 (floor((level-10)/2))"
            ),
        });
    }
}

/// Grounds Tactician's six magnitude-bearing features
/// (`rules_tables::ultimate_psionics::tactician_features`) — SD-32 card 11
/// (T12), cycle 3, the seventh class of `ultimate_psionics` attempted.
pub(super) fn ground_tactician_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_psionics::tactician_features as tf;
    let int = ability_modifiers.intelligence;
    let cha = ability_modifiers.charisma;
    let int_score = input.chosen.ability_scores.intelligence;

    // SD-32 card 11 (T12) follow-up: `Tactician Manifesting`'s shape-3
    // magnitudes, surfaced by the `psion` cycle's widened census. Same
    // full-manifester `BasePowerPoints` ladder as `psion` itself.
    if let Some(v) = tf::tactician_power_points_total(level, int) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.tactician.tactician_manifesting.power_points"
                .to_owned(),
            value: v,
            detail: format!(
                "Tactician level {level} Tactician Manifesting: {v} power points (base \
                 ladder + (Intelligence modifier {int} * level)/2)"
            ),
        });
    }
    if let Some(v) = tf::tactician_powers_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.tactician.tactician_manifesting.powers_known"
                .to_owned(),
            value: v,
            detail: format!(
                "Tactician level {level} Tactician Manifesting: {v} powers known \
                 (equal to level)"
            ),
        });
    }
    if let Some(v) = tf::tactician_max_power_level(level, int_score) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.tactician.tactician_manifesting.max_power_level"
                .to_owned(),
            value: v,
            detail: format!(
                "Tactician level {level} Tactician Manifesting: maximum power level known \
                 {v} (min(9, floor((level+1)/2), Intelligence score {int_score} - 10))"
            ),
        });
    }

    if let Some(minds) = tf::collective_minds(level, int) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.tactician.collective.minds".to_owned(),
            value: minds,
            detail: format!(
                "Tactician level {level} Collective: {minds} minds joined \
                 (max(Intelligence modifier {int}, level/2))"
            ),
        });
    }
    if let Some(times) = tf::coordinated_strike_times_per_day(level, int) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.tactician.coordinated_strike.times_per_day".to_owned(),
            value: times,
            detail: format!(
                "Tactician level {level} Coordinated Strike: {times} uses per day (3 + \
                 Intelligence modifier {int})"
            ),
        });
    }
    if let Some(times) = tf::strategy_times_per_day(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.tactician.strategy.times_per_day".to_owned(),
            value: times,
            detail: format!(
                "Tactician level {level} Strategy: {times} uses per day (3 + Charisma \
                 modifier {cha})"
            ),
        });
    }
    if let Some(powers) = tf::improved_share_powers(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.tactician.improved_share.powers".to_owned(),
            value: powers,
            detail: format!(
                "Tactician level {level} Improved Share: {powers} Shared powers maintained \
                 (1+floor((level+1)/6))"
            ),
        });
    }
    if let Some(pool) = tf::teamwork_feats_bonus_pool(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.tactician.teamwork_feats.bonus_pool".to_owned(),
            value: pool,
            detail: format!(
                "Tactician level {level} Teamwork Feats: {pool} bonus teamwork feats \
                 (floor(level/6))"
            ),
        });
    }
    if let Some(bonus) = tf::master_strategist_bonus(level, int) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.tactician.master_strategist.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Tactician level {level} Master Strategist: +{bonus} insight bonus \
                 (Intelligence modifier {int} only, no level term)"
            ),
        });
    }
}

/// Grounds Vitalist's six magnitude-bearing features
/// (`rules_tables::ultimate_psionics::vitalist_features`) — SD-32 card 11
/// (T12), cycle 3, the eighth class of `ultimate_psionics` attempted.
pub(super) fn ground_vitalist_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_psionics::vitalist_features as vf;
    let wis = ability_modifiers.wisdom;
    let wis_score = input.chosen.ability_scores.wisdom;

    // SD-32 card 11 (T12) follow-up: `Vitalist Manifesting`'s shape-3
    // magnitudes, surfaced by the `psion` cycle's widened census. Same
    // full-manifester `BasePowerPoints` ladder as `psion`/Tactician.
    if let Some(v) = vf::vitalist_power_points_total(level, wis) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vitalist.vitalist_manifesting.power_points".to_owned(),
            value: v,
            detail: format!(
                "Vitalist level {level} Vitalist Manifesting: {v} power points (base ladder \
                 + (Wisdom modifier {wis} * level)/2)"
            ),
        });
    }
    if let Some(v) = vf::vitalist_powers_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vitalist.vitalist_manifesting.powers_known".to_owned(),
            value: v,
            detail: format!(
                "Vitalist level {level} Vitalist Manifesting: {v} powers known \
                 (1 + floor((level+1)/2))"
            ),
        });
    }
    if let Some(v) = vf::vitalist_max_power_level(level, wis_score) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vitalist.vitalist_manifesting.max_power_level"
                .to_owned(),
            value: v,
            detail: format!(
                "Vitalist level {level} Vitalist Manifesting: maximum power level known {v} \
                 (min(9, floor((level+1)/2), Wisdom score {wis_score} - 10))"
            ),
        });
    }

    if let Some(minds) = vf::collective_minds(level, wis) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vitalist.collective.minds".to_owned(),
            value: minds,
            detail: format!(
                "Vitalist level {level} Collective: {minds} minds joined \
                 (max(level/2, Wisdom modifier {wis}))"
            ),
        });
    }
    if let Some(times) = vf::transfer_wounds_times_per_day(level, wis) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vitalist.transfer_wounds.times_per_day".to_owned(),
            value: times,
            detail: format!(
                "Vitalist level {level} Transfer Wounds: {times} uses per day (3 + Wisdom \
                 modifier {wis})"
            ),
        });
    }
    if let Some(lvl) = vf::health_sense_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vitalist.health_sense.level".to_owned(),
            value: lvl,
            detail: format!(
                "Vitalist level {level} Health Sense: tracked level {lvl} (equal to class \
                 level)"
            ),
        });
    }
    if let Some(dmg) = vf::steal_health_damage(level, wis) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vitalist.steal_health.damage".to_owned(),
            value: dmg,
            detail: format!(
                "Vitalist level {level} Steal Health: {dmg} hit points dealt and healed \
                 (level + Wisdom modifier {wis})"
            ),
        });
    }
    if let Some(times) = vf::request_aid_times_per_day(level, wis) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vitalist.request_aid.times_per_day".to_owned(),
            value: times,
            detail: format!(
                "Vitalist level {level} Request Aid: {times} uses per day (3 + Wisdom \
                 modifier {wis})"
            ),
        });
    }
    if let Some(dc) = vf::steal_life_dc(level, wis) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vitalist.steal_life.dc".to_owned(),
            value: dc,
            detail: format!(
                "Vitalist level {level} Steal Life: DC {dc} (10 + Wisdom modifier {wis} + \
                 level/2)"
            ),
        });
    }
}

/// Grounds Wilder's five magnitude-bearing features
/// (`rules_tables::ultimate_psionics::wilder_features`) — SD-32 card 11
/// (T12), cycle 3, the ninth and last class of `ultimate_psionics`
/// attempted, closing the whole source book.
pub(super) fn ground_wilder_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_psionics::wilder_features as wf;
    let cha = ability_modifiers.charisma;
    let cha_score = input.chosen.ability_scores.charisma;

    // SD-32 card 11 (T12) follow-up: `Wilder Manifesting`'s shape-3
    // magnitudes, surfaced by the `psion` cycle's widened census. Same
    // full-manifester `BasePowerPoints` ladder as `psion`/Tactician/Vitalist.
    if let Some(v) = wf::wilder_power_points_total(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.wilder.wilder_manifesting.power_points".to_owned(),
            value: v,
            detail: format!(
                "Wilder level {level} Wilder Manifesting: {v} power points (base ladder + \
                 (Charisma modifier {cha} * level)/2)"
            ),
        });
    }
    if let Some(v) = wf::wilder_powers_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.wilder.wilder_manifesting.powers_known".to_owned(),
            value: v,
            detail: format!(
                "Wilder level {level} Wilder Manifesting: {v} powers known \
                 (1 + floor(level/2))"
            ),
        });
    }
    if let Some(v) = wf::wilder_max_power_level(level, cha_score) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.wilder.wilder_manifesting.max_power_level".to_owned(),
            value: v,
            detail: format!(
                "Wilder level {level} Wilder Manifesting: maximum power level known {v} \
                 (min(9, floor((level+1)/2), Charisma score {cha_score} - 10))"
            ),
        });
    }

    if let Some(pct) = wf::psychic_enervation_percent(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.wilder.psychic_enervation.percent".to_owned(),
            value: pct,
            detail: format!(
                "Wilder level {level} Psychic Enervation: {pct}% chance of enervation (flat)"
            ),
        });
    }
    if let Some(range) = wf::surge_blast_range_feet(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.wilder.surge_blast.range_feet".to_owned(),
            value: range,
            detail: format!("Wilder level {level} Surge Blast: {range}-foot range (flat)"),
        });
    }
    if let Some(bonus) = wf::wild_surge_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.wilder.wild_surge.bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Wilder level {level} Wild Surge: +{bonus} manifester level and bonus power \
                 points (1+floor((level+1)/4))"
            ),
        });
    }
    if let Some(bonus) = wf::elude_attack_ac_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.wilder.elude_attack.ac_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Wilder level {level} Elude Attack: +{bonus} dodge bonus to AC \
                 (floor((level+2)/4))"
            ),
        });
    }
    if let Some(duration) = wf::surging_euphoria_duration_rounds(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.wilder.surging_euphoria.duration_rounds".to_owned(),
            value: duration,
            detail: format!(
                "Wilder level {level} Surging Euphoria: {duration} rounds duration (equal to \
                 the current Wild Surge bonus)"
            ),
        });
    }
}

/// Grounds Kineticist's six magnitude-bearing features
/// (`rules_tables::occult_adventures::kineticist_features`) — SD-32 card 11
/// (T12), cycle 4, the first of six `occult_adventures` classes sharing
/// `oa_abilities_class.lst`.
pub(super) fn ground_kineticist_class_features(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::occult_adventures::kineticist_features as kf;
    let con = ability_modifiers.constitution;

    if let Some(v) = kf::burn_max_points(level, con) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.kineticist.burn.max_points".to_owned(),
            value: v,
            detail: format!(
                "Kineticist level {level} Burn: {v} maximum points of burn (3 + Constitution \
                 modifier {con})"
            ),
        });
    }
    if let Some(v) = kf::elemental_focus_level_base(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.kineticist.elemental_focus.level_base".to_owned(),
            value: v,
            detail: format!(
                "Kineticist level {level} Elemental Focus: effective level {v} \
                 (max(1,floor(level/2)))"
            ),
        });
    }
    if let Some(v) = kf::infusion_pool(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.kineticist.infusion.pool".to_owned(),
            value: v,
            detail: format!("Kineticist level {level} Infusion: {v} infusions known"),
        });
    }
    if let Some(v) = kf::kinetic_blast_range_feet(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.kineticist.kinetic_blast.range_feet".to_owned(),
            value: v,
            detail: format!("Kineticist level {level} Kinetic Blast: {v}-foot range (flat)"),
        });
    }
    if let Some(v) = kf::wild_talents_dc(level, con) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.kineticist.wild_talents.dc".to_owned(),
            value: v,
            detail: format!(
                "Kineticist level {level} Wild Talents: DC {v} (10 + level/2 + Constitution \
                 modifier {con})"
            ),
        });
    }
    if let Some(v) = kf::expanded_element_pool(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.kineticist.expanded_element.pool".to_owned(),
            value: v,
            detail: format!(
                "Kineticist level {level} Expanded Element: {v} expanded elements (1 + \
                 (level>=15))"
            ),
        });
    }
}

/// Grounds Medium's magnitude-bearing features
/// (`rules_tables::occult_adventures::medium_features`) — SD-32 card 11
/// (T12), cycle 4, the second of six `occult_adventures` classes sharing
/// `oa_abilities_class.lst`.
pub(super) fn ground_medium_class_features(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::occult_adventures::medium_features as mf;
    let cha = ability_modifiers.charisma;

    if let Some(v) = mf::spirit_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.medium.spirit.bonus".to_owned(),
            value: v,
            detail: format!("Medium level {level} Spirit: +{v} spirit bonus (1+level/4)"),
        });
    }
    if let Some(v) = mf::spirit_surge_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.medium.spirit_surge.dice".to_owned(),
            value: v,
            detail: format!(
                "Medium level {level} Spirit Surge: {v}d6 (6+2*floor(level/10))"
            ),
        });
    }
    if let Some(v) = mf::haunt_channeler_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.medium.haunt_channeler.dice".to_owned(),
            value: v,
            detail: format!("Medium level {level} Haunt Channeler: {v}d6 (level/2)"),
        });
    }
    if let Some(v) = mf::haunt_channeler_dc(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.medium.haunt_channeler.dc".to_owned(),
            value: v,
            detail: format!("Medium level {level} Haunt Channeler: DC {v} (20+level/2)"),
        });
    }
    if let Some(v) = mf::location_channel_duration_rounds(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.medium.location_channel.duration_rounds".to_owned(),
            value: v,
            detail: format!("Medium level {level} Location Channel: {v} rounds (equal to level)"),
        });
    }
    if let Some(v) = mf::location_channel_dc(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.medium.location_channel.dc".to_owned(),
            value: v,
            detail: format!("Medium level {level} Location Channel: DC {v} (20+level/2)"),
        });
    }
    if let Some(v) = mf::ask_the_spirits_dc(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.medium.ask_the_spirits.dc".to_owned(),
            value: v,
            detail: format!(
                "Medium level {level} Ask the Spirits: DC {v} (15 + Charisma modifier {cha})"
            ),
        });
    }
    if let Some(v) = mf::astral_journey_dc(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.medium.astral_journey.dc".to_owned(),
            value: v,
            detail: format!(
                "Medium level {level} Astral Journey: DC {v} (19 + Charisma modifier {cha})"
            ),
        });
    }
    if let Some(v) = mf::trance_of_three_duration_rounds(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.medium.trance_of_three.duration_rounds".to_owned(),
            value: v,
            detail: format!(
                "Medium level {level} Trance of Three: {v} rounds (equal to level)"
            ),
        });
    }
}

/// Grounds Mesmerist's ten magnitude-bearing features
/// (`rules_tables::occult_adventures::mesmerist_features`) — SD-32 card 11
/// (T12), cycle 4, the third of six `occult_adventures` classes sharing
/// `oa_abilities_class.lst`.
pub(super) fn ground_mesmerist_class_features(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::occult_adventures::mesmerist_features as mf;
    let cha = ability_modifiers.charisma;

    if let Some(v) = mf::consummate_liar_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.consummate_liar.bonus".to_owned(),
            value: v,
            detail: format!("Mesmerist level {level} Consummate Liar: +{v} Bluff (max(level/2,1))"),
        });
    }
    if let Some(v) = mf::hypnotic_stare_penalty(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.hypnotic_stare.penalty".to_owned(),
            value: v,
            detail: format!("Mesmerist level {level} Hypnotic Stare: -{v} penalty"),
        });
    }
    if let Some(v) = mf::mesmerist_tricks_uses(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.mesmerist_tricks.uses".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Mesmerist Tricks: {v} uses per day (max(level/2,1) + \
                 Charisma modifier {cha})"
            ),
        });
    }
    if let Some(v) = mf::mesmerist_trick_range_feet(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.mesmerist_tricks.range_feet".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Mesmerist Tricks: {v}-foot range (100+level*10)"
            ),
        });
    }
    if let Some(v) = mf::mesmerist_trick_dc(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.mesmerist_tricks.dc".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Mesmerist Tricks: DC {v} (10+level/2+Charisma modifier \
                 {cha})"
            ),
        });
    }
    if let Some(v) = mf::mesmerist_tricks_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.mesmerist_tricks.known".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Mesmerist Tricks: {v} tricks known (level/2+1)"
            ),
        });
    }
    if let Some(v) = mf::painful_stare_damage(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.painful_stare.damage".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Painful Stare: +{v} damage (max(level/2,1))"
            ),
        });
    }
    if let Some(v) = mf::painful_stare_bonus_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.painful_stare.bonus_dice".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Painful Stare: {v} bonus dice (level/3)"
            ),
        });
    }
    if let Some(v) = mf::towering_ego_bonus(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.towering_ego.bonus".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Towering Ego: +{v} bonus (Charisma modifier {cha})"
            ),
        });
    }
    if let Some(v) = mf::bold_stares_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.bold_stare.known".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Bold Stare: {v} known ((level+1)/4)"
            ),
        });
    }
    if let Some(v) = mf::touch_treatment_uses(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.touch_treatment.uses".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Touch Treatment: {v} uses per day (3 + Charisma \
                 modifier {cha})"
            ),
        });
    }
    if let Some(v) = mf::manifold_tricks_count(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.manifold_tricks.count".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Manifold Tricks: {v} (2+(level-5)/4)"
            ),
        });
    }
    if let Some(v) = mf::mental_potency_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.mental_potency.bonus".to_owned(),
            value: v,
            detail: format!(
                "Mesmerist level {level} Mental Potency: +{v} (min(level/5,4))"
            ),
        });
    }
    if let Some(v) = mf::glib_lie_dc(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.mesmerist.glib_lie.dc".to_owned(),
            value: v,
            detail: format!("Mesmerist level {level} Glib Lie: DC {v} (15+level)"),
        });
    }
}

/// Grounds Occultist's magnitude-bearing features
/// (`rules_tables::occult_adventures::occultist_features`) — SD-32 card 11
/// (T12), cycle 4, the fourth of six `occult_adventures` classes sharing
/// `oa_abilities_class.lst`.
pub(super) fn ground_occultist_class_features(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::occult_adventures::occultist_features as of;
    let int = ability_modifiers.intelligence;

    if let Some(v) = of::focus_powers_count(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.occultist.focus_powers.count".to_owned(),
            value: v,
            detail: format!("Occultist level {level} Focus Powers: {v} known ((level+1)/2)"),
        });
    }
    if let Some(v) = of::focus_powers_dc(level, int) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.occultist.focus_powers.dc".to_owned(),
            value: v,
            detail: format!(
                "Occultist level {level} Focus Powers: DC {v} (10+level/2+Intelligence modifier \
                 {int})"
            ),
        });
    }
    if let Some(v) = of::implements_school_count(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.occultist.implements.school_count".to_owned(),
            value: v,
            detail: format!(
                "Occultist level {level} Implements: {v} (2+((level+2)/4))"
            ),
        });
    }
    if let Some(v) = of::mental_focus(level, int) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.occultist.mental_focus.value".to_owned(),
            value: v,
            detail: format!(
                "Occultist level {level} Mental Focus: {v} points (level + Intelligence \
                 modifier {int})"
            ),
        });
    }
    if let Some(v) = of::magic_item_skill_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.occultist.magic_item_skill.bonus".to_owned(),
            value: v,
            detail: format!(
                "Occultist level {level} Magic Item Skill: +{v} Use Magic Device (level/2)"
            ),
        });
    }
    if let Some(v) = of::outside_contact_count(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.occultist.outside_contact.count".to_owned(),
            value: v,
            detail: format!(
                "Occultist level {level} Outside Contact: {v} (1+(level-8)/4)"
            ),
        });
    }
    if let Some(v) = of::binding_circles_dc(level, int) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.occultist.binding_circles.dc".to_owned(),
            value: v,
            detail: format!(
                "Occultist level {level} Binding Circles: DC {v} (10+level/2+Intelligence \
                 modifier {int})"
            ),
        });
    }
}

// SD-32 row 17 (§27/§27a/§27b) residual closure: Psychic Discipline choice
// seam. `oa_abilities_class.lst:1188`-1196 (block "Psychic Disciplines")
// declares nine `KEY:Psychic Discipline ~ <Name>` records, each with its own
// `BONUS:VAR|PhrenicPoolAbility|<CHA|WIS>` token -- the source itself, not
// domain recall, fixes the split: Abomination/Dream/Pain/Rapport = CHA (4),
// Faith/Lore/Psychedelia/Self-Perfection/Tranquility = WIS (5). Recognition
// only, same idiom as `SORCERER_BLOODLINE_CHOICE_ID`: no ability score is
// ever fabricated for a choice the character input does not make.
pub(super) const PSYCHIC_DISCIPLINE_CHOICE_ID: &str = "choice:psychic_discipline";

pub(super) const PSYCHIC_DISCIPLINE_CHA_SELECTION_IDS: [&str; 4] = [
    "discipline:abomination",
    "discipline:dream",
    "discipline:pain",
    "discipline:rapport",
];

pub(super) const PSYCHIC_DISCIPLINE_WIS_SELECTION_IDS: [&str; 5] = [
    "discipline:faith",
    "discipline:lore",
    "discipline:psychedelia",
    "discipline:self_perfection",
    "discipline:tranquility",
];

/// Resolves `oa_abilities_class.lst`'s `PhrenicPoolAbility` term
/// (`Psychic ~ Phrenic Pool`'s own `BONUS:VAR|PhrenicPool|(PsychicLVL/2)+
/// PhrenicPoolAbility`) to the real modifier and display name for whichever
/// Psychic Discipline the character actually chose, via
/// `PSYCHIC_DISCIPLINE_CHOICE_ID`. Returns `None` for no selection or an
/// unrecognized one -- the caller must not guess an ability score for an
/// unmade choice (`decisions.md §1a`: a relabelled shape is not a closed
/// shape, and neither is a fabricated one).
pub(super) fn psychic_discipline_pool_ability(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
) -> Option<(i16, &'static str)> {
    let selection = choice_selection(input, PSYCHIC_DISCIPLINE_CHOICE_ID)?;
    if PSYCHIC_DISCIPLINE_CHA_SELECTION_IDS.contains(&selection) {
        Some((ability_modifiers.charisma, "Charisma"))
    } else if PSYCHIC_DISCIPLINE_WIS_SELECTION_IDS.contains(&selection) {
        Some((ability_modifiers.wisdom, "Wisdom"))
    } else {
        None
    }
}

/// Grounds Psychic's four magnitude-bearing features
/// (`rules_tables::occult_adventures::psychic_features`) — SD-32 card 11
/// (T12), cycle 4, the fifth of six `occult_adventures` classes sharing
/// `oa_abilities_class.lst`; row 17 residual closure (cycle 2) then wired
/// `Phrenic Pool`'s discipline-dependent ability term to the character's
/// actual chosen Psychic Discipline (`psychic_discipline_pool_ability`)
/// instead of hard-coding Charisma.
pub(super) fn ground_psychic_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::occult_adventures::psychic_features as pf;

    if let Some(v) = pf::phrenic_amplifications_count(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psychic.phrenic_amplifications.count".to_owned(),
            value: v,
            detail: format!(
                "Psychic level {level} Phrenic Amplifications: {v} known (1+((level-1)/4))"
            ),
        });
    }
    if let Some((discipline_ability_modifier, discipline_ability_name)) =
        psychic_discipline_pool_ability(input, ability_modifiers)
        && let Some(v) = pf::phrenic_pool(level, discipline_ability_modifier) {
            explanations.push(ComputationExplanation {
                id: "class_feature.untabled.psychic.phrenic_pool.value".to_owned(),
                value: v,
                detail: format!(
                    "Psychic level {level} Phrenic Pool: {v} points (level/2 + discipline \
                     ability modifier, {discipline_ability_name} {discipline_ability_modifier} \
                     for the chosen Psychic Discipline)"
                ),
            });
        }
    if let Some(v) = pf::psychic_discipline_pool(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psychic.psychic_discipline.pool".to_owned(),
            value: v,
            detail: format!("Psychic level {level} Psychic Discipline: {v} chosen (flat)"),
        });
    }
    if let Some(v) = pf::major_amplifications_count(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psychic.major_amplifications.count".to_owned(),
            value: v,
            detail: format!(
                "Psychic level {level} Major Amplifications: {v} known (1+((level-11)/4))"
            ),
        });
    }
}

/// Grounds Spiritualist's three magnitude-bearing features
/// (`rules_tables::occult_adventures::spiritualist_features`) — SD-32 card
/// 11 (T12), cycle 4, the sixth and last of the six `occult_adventures`
/// classes sharing `oa_abilities_class.lst`, closing the whole source file.
pub(super) fn ground_spiritualist_class_features(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    use crate::rules_core::rules_tables::occult_adventures::spiritualist_features as sf;

    if let Some(v) = sf::phantom_master_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.spiritualist.phantom.master_level".to_owned(),
            value: v,
            detail: format!(
                "Spiritualist level {level} Phantom: master level {v} (equal to class level)"
            ),
        });
    }
    if let Some(v) = sf::shared_consciousness_focus_pool(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.spiritualist.shared_consciousness.focus_pool".to_owned(),
            value: v,
            detail: format!(
                "Spiritualist level {level} Shared Consciousness: {v} emotional focus chosen \
                 (flat)"
            ),
        });
    }
    if let Some(v) = sf::calm_spirit_uses_per_day(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.spiritualist.calm_spirit.uses_per_day".to_owned(),
            value: v,
            detail: format!(
                "Spiritualist level {level} Calm Spirit: {v} uses per day (1+(level-7)/4)"
            ),
        });
    }
}

/// Grounds Magus's six magnitude-bearing features
/// (`rules_tables::ultimate_magic::magus_features`) — SD-32 card 11 (T12),
/// cycle 4, `ultimate_magic`'s single magnitude-bearing class.
pub(super) fn ground_magus_class_features(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_magic::magus_features as mf;
    let int = ability_modifiers.intelligence;

    if let Some(v) = mf::arcane_pool(level, int) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.magus.arcane_pool.value".to_owned(),
            value: v,
            detail: format!(
                "Magus level {level} Arcane Pool: {v} points (max(floor(level/2),1) + \
                 Intelligence modifier {int})"
            ),
        });
    }
    if let Some(v) = mf::arcane_pool_enhancement_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.magus.arcane_pool.enhancement_bonus".to_owned(),
            value: v,
            detail: format!(
                "Magus level {level} Arcane Pool: +{v} maximum enhancement bonus \
                 (min(1+((level-1)/4),5))"
            ),
        });
    }
    if let Some(v) = mf::armor_proficiency_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.magus.armor_proficiency.level".to_owned(),
            value: v,
            detail: format!(
                "Magus level {level} Armor Proficiency: tracked level {v} (equal to class \
                 level)"
            ),
        });
    }
    if let Some(v) = mf::magus_arcana_pool(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.magus.magus_arcana.pool".to_owned(),
            value: v,
            detail: format!("Magus level {level} Magus Arcana: {v} known (level/3)"),
        });
    }
    if let Some(v) = mf::bonus_feats_pool(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.magus.bonus_feats.pool".to_owned(),
            value: v,
            detail: format!("Magus level {level} Bonus Feats: {v} ((level+1)/6)"),
        });
    }
    if let Some(v) = mf::fighter_training_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.magus.fighter_training.level".to_owned(),
            value: v,
            detail: format!(
                "Magus level {level} Fighter Training: effective fighter level {v} (level/2)"
            ),
        });
    }
}

/// Grounds Shifter's magnitude-bearing features
/// (`rules_tables::ultimate_wilderness::shifter_features`) — SD-32 card 11
/// (T12), cycle 4, `ultimate_wilderness`'s single magnitude-bearing class.
/// `Shifter Claws`' base value is keyed on the character's resolved size
/// (`race_size_for_race_token`), the same size-resolution mechanism the
/// combat baseline already uses.
pub(super) fn ground_shifter_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_wilderness::shifter_features as sf;
    let wis = ability_modifiers.wisdom;
    let size = race_size_for_race_token(&input.chosen.race_id);

    if let Some(v) = sf::shifter_aspect_minutes(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.shifter.shifter_aspect.minutes".to_owned(),
            value: v,
            detail: format!("Shifter level {level} Shifter Aspect: {v} minutes (level+3)"),
        });
    }
    if let Some(v) = sf::shifter_aspect_count(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.shifter.shifter_aspect.count".to_owned(),
            value: v,
            detail: format!(
                "Shifter level {level} Shifter Aspect: {v} aspects (1+min(3,level/5))"
            ),
        });
    }
    if let Some(v) = sf::shifter_claw_damage(level, size) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.shifter.shifter_claws.damage".to_owned(),
            value: v,
            detail: format!(
                "Shifter level {level} Shifter Claws: {v} tracked claw damage value (size- and \
                 level-keyed)"
            ),
        });
    }
    if let Some(v) = sf::defensive_instinct_ac_bonus(level, wis) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.shifter.defensive_instinct.ac_bonus".to_owned(),
            value: v,
            detail: format!(
                "Shifter level {level} Defensive Instinct: +{v} AC (min(level/4,5) + Wisdom \
                 modifier {wis}/2)"
            ),
        });
    }
    if let Some(v) = sf::shifter_track_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.shifter.track.level".to_owned(),
            value: v,
            detail: format!("Shifter level {level} Track: tracked level {v} (equal to level)"),
        });
    }
    if let Some(v) = sf::wild_shape_count(level, wis) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.shifter.wild_shape.count".to_owned(),
            value: v,
            detail: format!(
                "Shifter level {level} Wild Shape: {v} uses per day (level + Wisdom modifier \
                 {wis})"
            ),
        });
    }
}

/// Grounds Vigilante's seven magnitude-bearing features
/// (`rules_tables::ultimate_intrigue::vigilante_features`) — SD-32 card 11
/// (T12), cycle 4, `ultimate_intrigue`'s single magnitude-bearing class.
pub(super) fn ground_vigilante_class_features(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_intrigue::vigilante_features as vf;
    let cha = ability_modifiers.charisma;

    if let Some(v) = vf::seamless_guise_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vigilante.seamless_guise.bonus".to_owned(),
            value: v,
            detail: format!("Vigilante level {level} Seamless Guise: +{v} (flat)"),
        });
    }
    if let Some(v) = vf::social_talent_count(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vigilante.social_talent.count".to_owned(),
            value: v,
            detail: format!(
                "Vigilante level {level} Social Talent: {v} known ((level+1)/2)"
            ),
        });
    }
    if let Some(v) = vf::vigilante_specialization_pool(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vigilante.vigilante_specialization.pool".to_owned(),
            value: v,
            detail: format!(
                "Vigilante level {level} Vigilante Specialization: {v} chosen (flat)"
            ),
        });
    }
    if let Some(v) = vf::vigilante_talent_count(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vigilante.vigilante_talent.count".to_owned(),
            value: v,
            detail: format!(
                "Vigilante level {level} Vigilante Talent: {v} known (level/2)"
            ),
        });
    }
    if let Some(v) = vf::vigilante_talent_dc(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vigilante.vigilante_talent.dc".to_owned(),
            value: v,
            detail: format!(
                "Vigilante level {level} Vigilante Talent: DC {v} (10+level/2+Charisma modifier \
                 {cha})"
            ),
        });
    }
    if let Some(v) = vf::unshakable_dc_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vigilante.unshakable.dc_bonus".to_owned(),
            value: v,
            detail: format!(
                "Vigilante level {level} Unshakable: +{v} DC bonus (equal to level)"
            ),
        });
    }
    if let Some(v) = vf::frightening_appearance_dc(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vigilante.frightening_appearance.dc".to_owned(),
            value: v,
            detail: format!(
                "Vigilante level {level} Frightening Appearance: DC {v} \
                 (10+level/2+Charisma modifier {cha})"
            ),
        });
    }
    if let Some(v) = vf::stunning_appearance_dc(level, cha) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vigilante.stunning_appearance.dc".to_owned(),
            value: v,
            detail: format!(
                "Vigilante level {level} Stunning Appearance: DC {v} \
                 (10+level/2+Charisma modifier {cha})"
            ),
        });
    }
    if let Some(v) = vf::stunning_appearance_hd_threshold(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.vigilante.stunning_appearance.hd_threshold".to_owned(),
            value: v,
            detail: format!(
                "Vigilante level {level} Stunning Appearance: {v} HD threshold (equal to level)"
            ),
        });
    }
}

/// Grounds Psion's own shape-3 magnitude-bearing feature (`rules_tables::
/// ultimate_psionics::psion_features`) — SD-32 card 11 (T12), the tenth
/// `ultimate_psionics` class attempted end-to-end, and the last named T12
/// item. See `psion_features`'s own module doc comment for why `psion`
/// uses a genuinely-third grant convention (confirmed against the earlier
/// "7 classes need a third shape" false lead, not a repeat of that
/// case-sensitivity bug) and for the discipline-choice pool population
/// (32 magnitude leaves) this function deliberately does not attempt.
pub(super) fn ground_psion_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::rules_tables::ultimate_psionics::psion_features as pf;
    let int_mod = ability_modifiers.intelligence;
    let int_score = input.chosen.ability_scores.intelligence;

    if let Some(v) = pf::psion_power_points_total(level, int_mod) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psion.psion_manifesting.power_points".to_owned(),
            value: v,
            detail: format!(
                "Psion level {level} Psion Manifesting: {v} power points (base ladder + \
                 (Intelligence modifier {int_mod} * level)/2)"
            ),
        });
    }
    // SD-32 T12 follow-up: closes the previously-escalated PsionPowersKnown/
    // PsionMaxPowerLevel ambiguity -- see `psion_features`'s own doc comment.
    if let Some(v) = pf::psion_powers_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psion.psion_manifesting.powers_known".to_owned(),
            value: v,
            detail: format!(
                "Psion level {level} Psion Manifesting: {v} powers known \
                 (min(21,2*level+1), plus floor((level-10)*3/2) summed in once level >= 11 -- \
                 the two same-target bonus rows sum, the converter-side bonus-stack \
                 reader's documented semantics)"
            ),
        });
    }
    if let Some(v) = pf::psion_max_power_level(level, int_score) {
        explanations.push(ComputationExplanation {
            id: "class_feature.untabled.psion.psion_manifesting.max_power_level".to_owned(),
            value: v,
            detail: format!(
                "Psion level {level} Psion Manifesting: maximum power level known {v} \
                 (min(9, floor((level+1)/2), Intelligence score {int_score} - 10))"
            ),
        });
    }
}

/// Emits one class's "these features are named but this engine computes
/// nothing for them" record on **both** receipt channels.
///
/// Until this existed, the four Pathfinder Unchained classes pushed that
/// record on the diagnostic channel only, and it reached no player. The
/// Character Sheet's "Class Features & Special Abilities" section
/// (`CharacterSheet.tsx` -> `classFeaturesModel.ts`) has always had a
/// dedicated "Not computed" lane for exactly this — a record whose id ends in
/// `.unsupported`, rendered with its detail text and deliberately without its
/// filler-zero value, so the sheet never flattens "not computed" into "0" —
/// but it reads `LoadSavedCharacterResponse.explanations`, and nothing in this
/// engine had ever emitted an `.unsupported` *explanation*. The lane was
/// therefore dead code and the deferral invisible: 23 of Unchained Rogue's,
/// Barbarian's, Monk's and Summoner's 64 ingested class features compute
/// nothing (including the Rogue's headline Debilitating Injury) and the sheet
/// said nothing at all.
///
/// The diagnostic is kept as well as, not replaced by, the explanation: it is
/// what `pf1_adapter`'s creation path and the headless receipt consumers read,
/// and dropping it would silently change the mutation-blocking surface. Both
/// carry the identical text, from one source, so the two channels cannot
/// drift into telling different stories.
///
/// `value: 0` is the filler zero `classFeaturesModel.ts` documents and
/// discards; nothing downstream renders it as a magnitude.
pub(super) fn push_deferred_class_features(
    id: &str,
    message: String,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    diagnostics.push(ComputationDiagnostic {
        id: id.to_owned(),
        message: message.clone(),
        claim_blocking: false,
    });
    explanations.push(ComputationExplanation { id: id.to_owned(), value: 0, detail: message });
}

/// Grounds the Unchained Rogue's named features
/// (`rules_tables::pathfinder_unchained::rogue_features`).
/// Render a class-skill list as a sheet line reads it.
///
/// A whole-family entry becomes the words the rule uses ("every Craft skill"),
/// never the ingest format's `TYPE=<Family>` selector. SD-35
/// `AT-35-E6-003-SWEEP` cycle 9: this string is shipped `explanation` text a
/// player reads, and it was printing ingest vocabulary verbatim. The family is
/// deliberately still not expanded into its member skills -- that roster lives
/// with `skill_allocation`, and inventing it here would be a different claim
/// than the rule makes.
pub(super) fn render_class_skill_list(
    skills: &'static [crate::rules_core::rules_tables::crb::class_skill_tables::ClassSkillEntry],
) -> String {
    use crate::rules_core::rules_tables::crb::class_skill_tables::ClassSkillEntry;
    skills
        .iter()
        .map(|entry| match entry {
            ClassSkillEntry::Named(name) => (*name).to_string(),
            ClassSkillEntry::Family(family) => format!("every {family} skill"),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Compute the base-attack-bonus / base-save chassis pillar for a length-2+
/// multiclass `class_levels` mix (SD-21 E7.28), or return `None` when any class
/// in the mix is not one Epic 6 grounds a `compute_<class>_chassis` for.
///
/// Each class level is run through its own `compute_<class>_chassis` in
/// isolation — a synthetic single-class `CharacterInput` clone carrying only
/// that one `CharacterClassLevel` — so Fighter's and Wizard's existing,
/// independently-verified chassis functions run completely unmodified. The
/// per-class explanations/diagnostics from each isolated sub-computation are
/// deliberately discarded (not merged into the outer `explanations` /
/// `diagnostics`): merging them verbatim would push the same generic
/// `class_chassis.base_attack_bonus` / `class_chassis.base_save.*` ids twice —
/// once per class — silently clobbering one class's explanation record with
/// the other's under a `Vec` lookup-by-id. Instead, this function pushes its
/// own single combined explanation per field, naming both classes in the
/// detail text.
///
/// Base attack bonus is a plain sum of the per-class results. Base saves use
/// the same per-class-round-then-sum shape for now; E7.29 replaces the save
/// combination with PF1's fractional-progression stacking rule (summing each
/// class's un-rounded fractional save contribution before rounding down once),
/// which is not a naive sum and diverges from this shape at some level pairs.
pub(super) fn compute_multiclass_base_chassis(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    // Every class in a supported multiclass mix (per `is_supported_multiclass_mix`)
    // resolves its own isolated `compute_class_chassis` to `Some`, so no
    // `class_chassis.unsupported`-style diagnostic is ever pushed here; the
    // parameter is kept only for signature symmetry with `compute_class_chassis`.
    _diagnostics: &mut Vec<ComputationDiagnostic>,
) -> Option<(i16, BaseSaves)> {
    if !is_supported_multiclass_mix(input) {
        return None;
    }

    let mut total_bab: i16 = 0;
    let mut fort_fraction = 0.0_f64;
    let mut ref_fraction = 0.0_f64;
    let mut will_fraction = 0.0_f64;
    let mut class_summaries: Vec<String> = Vec::new();

    for class_level in &input.chosen.class_levels {
        let mut isolated = input.clone();
        isolated.chosen.class_levels = vec![class_level.clone()];

        let mut isolated_explanations = Vec::new();
        let mut isolated_diagnostics = Vec::new();
        let (bab, _isolated_saves) = compute_class_chassis(
            &isolated,
            ability_modifiers,
            &mut isolated_explanations,
            &mut isolated_diagnostics,
        )?;
        total_bab += bab;

        // SD-21 E7.29: PF1's multiclass base-save rule sums each class's
        // *un-rounded* fractional save contribution before rounding down once
        // for the total -- it is not a naive per-class-round-then-sum (which
        // would round each class's contribution down separately first, losing
        // any fractional remainder that would otherwise carry into the next
        // integer once combined with another class's remainder). The
        // good/poor classification per save mirrors `class_tables.rs`'s
        // `GoodSaves` row for whatever class this loop iteration is on
        // (widened v0.6 alpha swarm task 4 from a Fighter/Wizard-only pair
        // to every class `table_class_id` recognizes).
        let (fort_good, ref_good, will_good) = multiclass_good_saves(&class_level.class_id)?;
        fort_fraction += fractional_save_value(class_level.level, fort_good);
        ref_fraction += fractional_save_value(class_level.level, ref_good);
        will_fraction += fractional_save_value(class_level.level, will_good);

        class_summaries.push(format!(
            "{} {}: base attack bonus {bab}",
            class_level.class_id, class_level.level
        ));
    }

    let total_saves = BaseSaves {
        fortitude: fort_fraction.floor() as i16,
        reflex: ref_fraction.floor() as i16,
        will: will_fraction.floor() as i16,
    };

    let class_summary = class_summaries.join("; ");

    explanations.push(ComputationExplanation {
        id: "class_chassis.base_attack_bonus".to_owned(),
        value: total_bab,
        detail: format!(
            "Multiclass base attack bonus {total_bab}: the sum of each class's own \
             independently-computed base attack bonus ({class_summary})"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: total_saves.fortitude,
        detail: format!(
            "Multiclass base Fortitude save {}: PF1's sum-fractions-then-round-down-once rule, \
             fractional total {fort_fraction:.3} across ({class_summary})",
            total_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: total_saves.reflex,
        detail: format!(
            "Multiclass base Reflex save {}: PF1's sum-fractions-then-round-down-once rule, \
             fractional total {ref_fraction:.3} across ({class_summary})",
            total_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: total_saves.will,
        detail: format!(
            "Multiclass base Will save {}: PF1's sum-fractions-then-round-down-once rule, \
             fractional total {will_fraction:.3} across ({class_summary})",
            total_saves.will
        ),
    });

    Some((total_bab, total_saves))
}

/// PF1's per-class base-save fractional value at a class level, before
/// rounding (SD-21 E7.29). A "good" save's fractional formula is
/// `level/2 + 2`; a "poor" save's is `level/3` -- the same formulas
/// `compute_fighter_chassis` and `class_tables.rs`'s `save_bonus` already
/// apply, just evaluated as a real number instead of floored per-class. The
/// canonical multiclass rule sums these fractional values across every class
/// in the mix and rounds down only once for the total.
pub(super) fn fractional_save_value(level: u8, good: bool) -> f64 {
    let level = f64::from(level);
    if good { level / 2.0 + 2.0 } else { level / 3.0 }
}

/// Whether `class_id` grounds a "good" progression for Fortitude, Reflex, and
/// Will respectively (SD-21 E7.29; widened v0.6 alpha swarm task 4 to every
/// class `table_class_id` recognizes, not just Fighter/Wizard). Reads
/// `class_tables.rs`'s own ingested `good_saves_for` classification directly
/// (SD-24 Epic 5 criterion 5.3) rather than re-declaring a second,
/// independently-maintained copy that could silently drift from it. Returns
/// `None` for any other class id.
pub(super) fn multiclass_good_saves(class_id: &str) -> Option<(bool, bool, bool)> {
    good_saves_for(table_class_id(class_id)?)
}

/// PF1's own class-level ceiling, and the `MAXLEVEL:20` every CRB/APG/ACG
/// class record in the corpus declares. There is no 21st character level, so
/// nothing above this grounds a caster level.
pub(super) const PF1_MAX_CLASS_LEVEL: u8 = 20;

/// One casting class's corpus-transcribed caster-level rule
/// (v0.6 Receipt-to-Sheet slice 1 item 4; see
/// `docs/release/v0.6/execution-engine-scoping.md` §3 and §7).
///
/// Every field is a verbatim transcription, never a summary. The two
/// `BONUS:*` strings are what make this record auditable without leaving the
/// file: a reader can grep the named corpus line and compare byte for byte.
pub(super) struct CasterLevelRule {
    /// The `class:<name>` suffix this rule answers for.
    class_name: &'static str,
    /// The corpus file and line the `BONUS:CASTERLEVEL` token lives on.
    token_source: &'static str,
    /// `true` for the four ACG classes whose corpus rule names the class
    /// level directly, with no intermediate variable to chase.
    ///
    /// The two `&'static str` fields that stood here -- `token` and
    /// `resolution` -- held the class's `BONUS:CASTERLEVEL` token and the
    /// `BONUS:VAR` chain it names, verbatim, and the renderer interpolated
    /// BOTH into the `ComputationExplanation.detail` a player's sheet
    /// prints. They moved to the `//` provenance lines above each row of
    /// `CASTER_LEVEL_RULES` -- SD-35 `AT-35-E6-003-SWEEP` cycle 6,
    /// `decisions.md` §1 (no ingest vocabulary on a sheet line) and §17
    /// (provenance belongs in a comment beside the number it explains).
    /// Nothing was lost: every token string is still in the file, byte for
    /// byte, four lines up from the row it belongs to.
    names_class_level_directly: bool,
    /// The class level at which the corpus's own `PRECLASS:` gate on the
    /// token opens. `1` where the token carries no gate at all.
    first_casting_class_level: u8,
    /// The corpus's declared `SPELLSTAT:` for the class. Carried purely so
    /// the record can name it; no ability-score arithmetic is done with it
    /// here (bonus spell slots and save DCs are explicitly out of scope).
    spell_stat: &'static str,
}

/// Every class in this codebase's 27-class roster that casts, EXCEPT the two
/// partial casters (see below), paired with the corpus rule that fixes its
/// caster level.
///
/// **All seventeen resolve to "caster level = class level."** That uniformity
/// is a finding, not an assumption — each entry was transcribed one at a time
/// from its own `BONUS:CASTERLEVEL` token and the `BONUS:VAR` chain that token
/// names, and the shared terms were checked rather than waved through:
///
///   * `Caster_Level_Bonus` is `DEFINE`d to 0 in
///     `core_essentials/ce_abilities.lst:11`. It is raised only by opt-in
///     content — the Orange Prism Ioun Stone (`cr_abilities.lst:555`) and the
///     APG Magical Knack trait, whose own
///     `BONUS:CASTERLEVEL|<Class>|min(2,(TL-(var("CL=<Class>")+var("BL=<Class>"))))`
///     family (`apg_abilities.lst:82-103`) is trait content, not base-class
///     content. Neither is base-class content, so both contribute 0 here.
///     This is exactly the "`DEFINE`s to 0, real value arrives elsewhere"
///     trap, checked rather than assumed.
///   * Each `CasterLevelBL<Class>` term is likewise `DEFINE`d to 0 on the
///     class line itself and raised only by bloodline/archetype records.
///   * The `BONUS:CASTERLEVEL|<Class>.RESET|0` records
///     (`apg_abilities_class.lst:3435-3437`, `acg_abilities_class.lst:1272`)
///     that would zero a caster level outright are `TYPE:Internal.<Class>
///     ClassFeatures` "No Spellcasting" abilities, granted only by
///     spellcasting-removing archetypes this repo does not ingest. They are
///     archetype-only records posing as base-class content, and are
///     deliberately not applied.
///   * No `.MOD` record anywhere in the corpus alters any of these tokens; the
///     only `#`-disabled `BONUS:CASTERLEVEL` lines
///     (`apg_abilities.lst:46`, `ce_abilities.lst:48`,
///     `uca_abilities_traits.lst:111,113`) are commented-out duplicates and
///     carry no effect.
///
/// **The Bloodrager trap.** Bloodrager's spell progression has precisely the
/// Paladin/Ranger shape — first spells at class level 4 — which invites
/// reading its caster level as `level - 3`. The corpus says otherwise: its
/// token resolves to the *full* class level and only the gate is delayed. The
/// literal `-3` appears on Paladin and Ranger alone (and on APG's Antipaladin,
/// which is outside this codebase's roster). Deriving Bloodrager's caster level
/// from its spell-progression table shape instead of from its
/// `BONUS:CASTERLEVEL` token would have produced a number that is wrong by 3 at
/// every one of its seventeen casting levels, 4 through 20.
///
/// **Why Paladin and Ranger are absent.** Both already ground this exact
/// arithmetic under `class_chassis.<class>.partial_caster.effective_caster_level`
/// (`explain_paladin_level1_chassis_and_spell_burden_separation` and
/// `explain_ranger_level1_chassis_and_class_feature_separation`). Emitting a
/// second record carrying the same number under a second id would be a
/// duplicate free to drift from the original; a consumer wanting a uniform
/// caster-level lookup should map those two ids, not read a copy.
pub(super) const CASTER_LEVEL_RULES: &[CasterLevelRule] = &[
    // ----- PF1 Core Rulebook (`core_rulebook/cr_classes.lst`) -----
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Bard|Caster_Level_BL_Stripped_Bard
    //   cr_classes.lst:24 BONUS:VAR|Caster_Level_BL_Stripped_Bard|Caster_Level_Bard-CasterLevelBLBard and BONUS:VAR|Caster_Level_Bard|CL+Caster_Level_Bonus+CasterLevelBLBard
    CasterLevelRule {
        class_name: "bard",
        token_source: "core_rulebook/cr_classes.lst:28",
        first_casting_class_level: 1,
        spell_stat: "CHA",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Cleric|Caster_Level_BL_Stripped_Cleric
    //   cr_classes.lst:55 BONUS:VAR|Caster_Level_BL_Stripped_Cleric|Caster_Level_Cleric-CasterLevelBLCleric and BONUS:VAR|Caster_Level_Cleric|CL+Caster_Level_Bonus+CasterLevelBLCleric
    CasterLevelRule {
        class_name: "cleric",
        token_source: "core_rulebook/cr_classes.lst:59",
        first_casting_class_level: 1,
        spell_stat: "WIS",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Druid|Caster_Level_BL_Stripped_Druid
    //   cr_classes.lst:93 BONUS:VAR|Caster_Level_BL_Stripped_Druid|Caster_Level_Druid-CasterLevelBLDruid and BONUS:VAR|Caster_Level_Druid|CL+Caster_Level_Bonus+CasterLevelBLDruid
    CasterLevelRule {
        class_name: "druid",
        token_source: "core_rulebook/cr_classes.lst:99",
        first_casting_class_level: 1,
        spell_stat: "WIS",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Sorcerer|Caster_Level_BL_Stripped_Sorcerer
    //   cr_classes.lst:246 BONUS:VAR|Caster_Level_BL_Stripped_Sorcerer|Caster_Level_Sorcerer-CasterLevelBLSorcerer and BONUS:VAR|Caster_Level_Sorcerer|CL+Caster_Level_Bonus+CasterLevelBLSorcerer
    CasterLevelRule {
        class_name: "sorcerer",
        token_source: "core_rulebook/cr_classes.lst:250",
        first_casting_class_level: 1,
        spell_stat: "CHA",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Wizard|Caster_Level_BL_Stripped_Wizard
    //   cr_classes.lst:277 BONUS:VAR|Caster_Level_BL_Stripped_Wizard|Caster_Level_Wizard-CasterLevelBLWizard, BONUS:VAR|Caster_Level_Wizard|WizardLVL+Caster_Level_Bonus+CasterLevelBLWizard and BONUS:VAR|WizardLVL|CL
    CasterLevelRule {
        class_name: "wizard",
        token_source: "core_rulebook/cr_classes.lst:281",
        first_casting_class_level: 1,
        spell_stat: "INT",
        names_class_level_directly: false,
    },
    // ----- PF1 Advanced Player's Guide (`advanced_players_guide/apg_classes.lst`) -----
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Alchemist|Caster_Level_BL_Stripped_Alchemist
    //   apg_classes.lst:11 BONUS:VAR|Caster_Level_BL_Stripped_Alchemist|Caster_Level_Alchemist-CasterLevelBLAlchemist and BONUS:VAR|Caster_Level_Alchemist|CL+Caster_Level_Bonus+CasterLevelBLAlchemist
    CasterLevelRule {
        class_name: "alchemist",
        token_source: "advanced_players_guide/apg_classes.lst:15",
        first_casting_class_level: 1,
        spell_stat: "INT",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Inquisitor|Caster_Level_BL_Stripped_Inquisitor
    //   apg_classes.lst:50 BONUS:VAR|Caster_Level_BL_Stripped_Inquisitor|Caster_Level_Inquisitor-CasterLevelBLInquisitor and BONUS:VAR|Caster_Level_Inquisitor|CL+Caster_Level_Bonus+CasterLevelBLInquisitor
    CasterLevelRule {
        class_name: "inquisitor",
        token_source: "advanced_players_guide/apg_classes.lst:56",
        first_casting_class_level: 1,
        spell_stat: "WIS",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Oracle|Caster_Level_BL_Stripped_Oracle
    //   apg_classes.lst:107 BONUS:VAR|Caster_Level_BL_Stripped_Oracle|Caster_Level_Oracle-CasterLevelBLOracle and BONUS:VAR|Caster_Level_Oracle|CL+Caster_Level_Bonus+CasterLevelBLOracle
    CasterLevelRule {
        class_name: "oracle",
        token_source: "advanced_players_guide/apg_classes.lst:111",
        first_casting_class_level: 1,
        spell_stat: "CHA",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Summoner|Caster_Level_BL_Stripped_Summoner
    //   apg_classes.lst:139 BONUS:VAR|Caster_Level_BL_Stripped_Summoner|Caster_Level_Summoner-CasterLevelBLSummoner and BONUS:VAR|Caster_Level_Summoner|CL+Caster_Level_Bonus+CasterLevelBLSummoner
    CasterLevelRule {
        class_name: "summoner",
        token_source: "advanced_players_guide/apg_classes.lst:145",
        first_casting_class_level: 1,
        spell_stat: "CHA",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Witch|Caster_Level_BL_Stripped_Witch
    //   apg_classes.lst:172 BONUS:VAR|Caster_Level_BL_Stripped_Witch|Caster_Level_Witch-CasterLevelBLWitch and BONUS:VAR|Caster_Level_Witch|CL+Caster_Level_Bonus+CasterLevelBLWitch
    CasterLevelRule {
        class_name: "witch",
        token_source: "advanced_players_guide/apg_classes.lst:176",
        first_casting_class_level: 1,
        spell_stat: "INT",
        names_class_level_directly: false,
    },
    // ----- PF1 Advanced Class Guide (`advanced_class_guide/acg_classes.lst`) -----
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Arcanist|CL
    //   (the token names the class level `CL` directly -- no chain)
    CasterLevelRule {
        class_name: "arcanist",
        token_source: "advanced_class_guide/acg_classes.lst:15",
        first_casting_class_level: 1,
        spell_stat: "INT",
        names_class_level_directly: true,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Bloodrager|Caster_Level_Bloodrager|PRECLASS:1,Bloodrager=4
    //   acg_classes.lst:40 BONUS:VAR|Caster_Level_Bloodrager|BloodragerLVL+Caster_Level_Bonus+CasterLevelBLBloodrager and BONUS:VAR|BloodragerLVL|CL
    CasterLevelRule {
        class_name: "bloodrager",
        token_source: "advanced_class_guide/acg_classes.lst:44",
        first_casting_class_level: 4,
        spell_stat: "CHA",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Hunter|CL
    //   (the token names the class level `CL` directly -- no chain)
    CasterLevelRule {
        class_name: "hunter",
        token_source: "advanced_class_guide/acg_classes.lst:114",
        first_casting_class_level: 1,
        spell_stat: "WIS",
        names_class_level_directly: true,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Investigator|Caster_Level_Investigator
    //   acg_classes.lst:168 BONUS:VAR|Caster_Level_Investigator|InvestigatorLVL+Caster_Level_Bonus+CasterLevelBLInvestigator and BONUS:VAR|InvestigatorLVL|classlevel(\"APPLIEDAS=NONEPIC\"), which is the class level for every level this codebase supports (1-20, all non-epic)
    CasterLevelRule {
        class_name: "investigator",
        token_source: "advanced_class_guide/acg_classes.lst:172",
        first_casting_class_level: 1,
        spell_stat: "INT",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Shaman|CL
    //   (the token names the class level `CL` directly -- no chain)
    CasterLevelRule {
        class_name: "shaman",
        token_source: "advanced_class_guide/acg_classes.lst:225",
        first_casting_class_level: 1,
        spell_stat: "WIS",
        names_class_level_directly: true,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Skald|Caster_Level_BL_Stripped_Skald
    //   acg_classes.lst:274 BONUS:VAR|Caster_Level_BL_Stripped_Skald|Caster_Level_Skald-CasterLevelBLSkald, BONUS:VAR|Caster_Level_Skald|SkaldLVL+Caster_Level_Bonus+CasterLevelBLSkald and BONUS:VAR|SkaldLVL|CL
    CasterLevelRule {
        class_name: "skald",
        token_source: "advanced_class_guide/acg_classes.lst:278",
        first_casting_class_level: 1,
        spell_stat: "CHA",
        names_class_level_directly: false,
    },
    // Provenance (ingest tokens, demoted out of the rendered sheet line
    // -- SD-35 AT-35-E6-003-SWEEP cycle 6). Verbatim, byte for byte:
    //   BONUS:CASTERLEVEL|Warpriest|CL
    //   (the token names the class level `CL` directly -- no chain)
    CasterLevelRule {
        class_name: "warpriest",
        token_source: "advanced_class_guide/acg_classes.lst:368",
        // The same corpus line also carries `BONUS:CASTERLEVEL|Cleric|CL`,
        // because Warpriest casts off `SPELLLIST:1|Cleric`. Both are the same
        // number (the Warpriest's own class level), so the borrowed list
        // changes nothing about the value here.
        first_casting_class_level: 1,
        spell_stat: "WIS",
        names_class_level_directly: true,
    },
];

/// The caster-level rule for a `class:<name>` id, or `None` for a class that
/// casts nothing (and for Paladin/Ranger, which ground their own).
pub(super) fn caster_level_rule(class_id: &str) -> Option<&'static CasterLevelRule> {
    let name = class_id.strip_prefix("class:")?;
    CASTER_LEVEL_RULES.iter().find(|r| r.class_name == name)
}

/// `"wizard"` -> `"Wizard"`, for the prose in a caster-level record. Every
/// `CASTER_LEVEL_RULES` class name is lowercase ASCII, so uppercasing the
/// first character is sufficient and cannot split a grapheme.
pub(super) fn capitalized_class_name(class_name: &str) -> String {
    let mut chars = class_name.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Ground `class_chassis.<class>.caster_level` for every casting class in the
/// character's mix (v0.6 Receipt-to-Sheet slice 1 item 4).
///
/// **What this unlocks.** The Spells tab already renders each spell's full
/// description, so a scaling clause like *"1d6 points of damage per caster
/// level (maximum 10d6)"* is already on the player's screen. It is unusable
/// without the one number it multiplies, and before this record no full caster
/// in the codebase had one — the only `caster_level` ids that existed were
/// Monk's Abundant Step and the two partial casters'. Per
/// `docs/release/v0.6/execution-engine-scoping.md` §4, computing the dice
/// themselves is a corpus-authoring project (PCGen's spell schema carries no
/// damage token at all); this is the ~1% of that effort that captures most of
/// its value.
///
/// **Single-class only, levels 1-20**, matching the shape every existing
/// `class_chassis.<class>.*` record already has. Both halves of that gate are
/// load-bearing, and both were caught by the existing negative controls rather
/// than assumed:
///
///   * **Multiclass is deliberately not promoted.** Every per-class slice in
///     this file pins a `multiclass_<class>_level<N>_is_not_promoted_by_this_slice`
///     control asserting that a mix surfaces NO `class_chassis.<class>.*` id at
///     all; `compute_multiclass_base_chassis` likewise discards its per-class
///     sub-computations' explanations on purpose. Emitting a per-class caster
///     level into a mix would have broken 71 of those controls. A multiclass
///     caster level is a real thing a player wants, but it belongs to whatever
///     slice promotes the multiclass class-chassis surface as a whole, not to
///     this one.
///   * **Level 21+ grounds nothing.** PF1 has no 21st character level and every
///     CRB/APG/ACG class record declares `MAXLEVEL:20`; each class's
///     `<class>_level_21_is_not_promoted_by_this_slice` control pins that the
///     range gate does not overshoot. An ungated loop broke 27 of those.
///
/// Within that gate it is unconditional on chassis support and on
/// spell-posture validity: the caster level is a fact about the class and level
/// alone, and a character blocked on some unrelated burden still has one.
///
/// **Bounded strictly to the caster level.** No spells known, no spells per
/// day, no bonus spell slots from a high `SPELLSTAT`, no spell save DCs, and no
/// per-spell dice are computed here. Every one of those remains exactly as
/// deferred as it was before this record existed.
pub(super) fn ground_caster_level_records(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    // Single-class only -- see this function's doc comment. `let [x] = ...`
    // mirrors `compute_class_chassis`'s own single-class dispatch exactly.
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return;
    };
    let Some(rule) = caster_level_rule(&class_level.class_id) else {
        return;
    };
    let level = class_level.level;
    if level == 0 || level > PF1_MAX_CLASS_LEVEL {
        return;
    }
    let gated_off = level < rule.first_casting_class_level;
    let caster_level = if gated_off { 0 } else { i16::from(level) };
    let class_display = capitalized_class_name(rule.class_name);

    // SD-35 AT-35-E6-003-SWEEP cycle 6: this prose used to interpolate
    // `rule.token` and `rule.resolution` verbatim, printing the corpus's own
    // ingest tokens on the player's sheet. Both now live in the `//`
    // provenance lines above each `CASTER_LEVEL_RULES` row; the sheet gets
    // the rule's words and the source citation, which is what it is for.
    let resolution_text = if rule.names_class_level_directly {
        "The book's rule names the class level directly, so the caster level is the class \
         level with no intermediate step."
            .to_owned()
    } else {
        "The book's rule routes the caster level through an intermediate variable that \
         resolves to the class level: the two terms that could raise it are both 0 for a \
         base-class character — one is raised only by opt-in item and trait content, the \
         other only by bloodline and archetype records."
            .to_owned()
    };

    let gate_text = if rule.first_casting_class_level > 1 {
        if gated_off {
            format!(
                "The rule does not open until {class} level {gate}, and this character is \
                 {class} {level}, below it, so the caster level is a correct absence (0) \
                 rather than a fabricated number. Only the opening level is delayed: from \
                 {class} level {gate} the caster level is the FULL class level, not \
                 level - 3 — that reduction applies to Paladin and Ranger alone.",
                class = class_display,
                gate = rule.first_casting_class_level,
            )
        } else {
            format!(
                "The rule opens at {class} level {gate} and this character is at or above \
                 it. Only the opening level is delayed: the caster level is the FULL class \
                 level, not level - 3, despite the spell progression having the \
                 Paladin/Ranger shape — that reduction applies to Paladin and Ranger alone.",
                class = class_display,
                gate = rule.first_casting_class_level,
            )
        }
    } else {
        "The rule carries no level gate, so it applies from class level 1.".to_owned()
    };

    explanations.push(ComputationExplanation {
        id: format!("class_chassis.{}.caster_level", rule.class_name),
        value: caster_level,
        detail: format!(
            "{class} caster level at {class} level {level}: {caster_level}, transcribed from \
             the book's own caster-level rule ({source}). {resolution_text} {gate_text} The \
             class's declared spellcasting ability ({stat}) is named for reference only. This \
             grounds the caster level and nothing else: no spells known, no spells per day, no \
             bonus spell slots from a high {stat}, no spell save DC, and no per-spell dice \
             counts are computed from it. The Spells tab already renders each spell's own \
             description text, which is where a \"per caster level\" scaling clause and its cap \
             live",
            class = class_display,
            source = rule.token_source,
            stat = rule.spell_stat,
        ),
    });
}

/// Choice-slot id for the PF1 Core Rulebook Favored Class rule (Core Rulebook
/// pg. 31, verified against the Archives of Nethys primary source: "Whenever a
/// character gains a level in his favored class, he receives either +1 hit
/// point or +1 skill rank"). A Human's favored class is Any (PF1 Core Rulebook
/// Human racial traits), which trivially includes Fighter, so this choice
/// applies at Fighter level 1 on this codebase's Human-only Fighter chassis
/// seam without needing to resolve the later "any race, any class" errata.
pub(super) const FAVORED_CLASS_BONUS_CHOICE_ID: &str = "choice:favored_class_bonus";

pub(super) const FAVORED_CLASS_BONUS_HP_SELECTION: &str = "bonus:hp";

pub(super) const FAVORED_CLASS_BONUS_SKILL_RANK_SELECTION: &str = "bonus:skill_rank";

/// `AT-34-E3-002` (bucket C, cycle 8): the SAME Favored Class Bonus choice
/// rule [`explain_fighter_favored_class_bonus_choice`] already grounds for
/// Fighter, generalized to the five OTHER PF1 Core Rulebook base classes
/// whose own bounded level-1 chassis recognition seam already exists
/// (`supported_barbarian_level`, `supported_monk_level`,
/// `supported_paladin_level`, `supported_rogue_level`,
/// `supported_wizard_level`). The rule itself (PF1 Core Rulebook pg. 31:
/// "whenever a character gains a level in his favored class, he receives
/// either +1 hit point or +1 skill rank") is genuinely class-agnostic, and a
/// Human's favored class is Any (PF1 Core Rulebook Human racial traits),
/// which trivially includes every one of these five classes at level 1 --
/// the identical reasoning Fighter's own doc comment gives. Written as one
/// generalized function reusing the SAME `FAVORED_CLASS_BONUS_CHOICE_ID`
/// choice id and the SAME flat, rule-verified +1 magnitude, rather than
/// five near-duplicates of Fighter's own function: only the class's own
/// bounded-seam gate and the explanation id's class-key segment differ. Each
/// class's own explanation is standalone (never applied to any hit-point or
/// skill-rank total), mirroring Fighter's own explicitly-scoped, non-
/// fabricated ceiling.
pub(super) fn explain_other_classes_favored_class_bonus_choice(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    // (class_key, class_display, level-support gate) per class this favored-class-bonus
    // choice applies to.
    type ClassGate = (&'static str, &'static str, fn(&CharacterInput) -> Option<u8>);
    const CLASS_GATES: &[ClassGate] = &[
        ("barbarian", "Barbarian", supported_barbarian_level),
        ("monk", "Monk", supported_monk_level),
        ("paladin", "Paladin", supported_paladin_level),
        ("rogue", "Rogue", supported_rogue_level),
        ("wizard", "Wizard", supported_wizard_level),
    ];

    for &(class_key, class_display, supported_level) in CLASS_GATES {
        if supported_level(input) != Some(1) {
            continue;
        }

        let Some(selection) = choice_selection(input, FAVORED_CLASS_BONUS_CHOICE_ID) else {
            continue;
        };

        let (value, detail) = if selection == FAVORED_CLASS_BONUS_HP_SELECTION {
            (
                1,
                format!(
                    "Favored Class bonus choice ({FAVORED_CLASS_BONUS_CHOICE_ID} -> {selection}): \
                     PF1 Core Rulebook pg. 31 grants a character +1 hit point or +1 skill rank for \
                     each level taken in his favored class; a Human's favored class is Any, which \
                     trivially includes {class_display}, so this level-1 {class_display} class level \
                     qualifies. This selection chooses the +1 hit point option. This is a flat, \
                     non-fabricated bonus magnitude only (+1) -- it is standalone and not applied to \
                     any hit-point or skill-rank total, since that would require wiring into the \
                     integrated computation, never attempted in this codebase"
                ),
            )
        } else if selection == FAVORED_CLASS_BONUS_SKILL_RANK_SELECTION {
            (
                1,
                format!(
                    "Favored Class bonus choice ({FAVORED_CLASS_BONUS_CHOICE_ID} -> {selection}): \
                     PF1 Core Rulebook pg. 31 grants a character +1 hit point or +1 skill rank for \
                     each level taken in his favored class; a Human's favored class is Any, which \
                     trivially includes {class_display}, so this level-1 {class_display} class level \
                     qualifies. This selection chooses the +1 skill rank option. This is a flat, \
                     non-fabricated bonus magnitude only (+1) -- it is standalone and not applied to \
                     any selected-skill-rank total, since that would require wiring into a general \
                     class-skill-rank allocation engine, never attempted in this codebase"
                ),
            )
        } else {
            (
                0,
                format!(
                    "Favored Class bonus choice slot is present ({FAVORED_CLASS_BONUS_CHOICE_ID} -> \
                     {selection}), but only the PF1 Core Rulebook's two legal options \
                     ({FAVORED_CLASS_BONUS_HP_SELECTION} or \
                     {FAVORED_CLASS_BONUS_SKILL_RANK_SELECTION}) are recognized on this bounded seam; \
                     no hp/skill-rank identity is resolved and no mechanical value is fabricated (+0)"
                ),
            )
        };

        explanations.push(ComputationExplanation {
            id: format!("class_chassis.{class_key}.favored_class_bonus_choice"),
            value,
            detail,
        });
    }
}

/// A hybrid (martial + later spellcasting) class this slice recognizes at its bounded
/// single-class level-1 chassis boundary only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum HybridClass {
    Paladin,
    Ranger,
}

/// Return the hybrid class when the chosen input is exactly a single-class Paladin or
/// Ranger at the bounded hybrid baseline level (1). Returns `None` for any other class,
/// a multiclass mix, or a level-2+ hybrid this slice deliberately does not recognize —
/// each of which stays blocked exactly as before.
pub(super) fn hybrid_level1_class(input: &CharacterInput) -> Option<HybridClass> {
    match input.chosen.class_levels.as_slice() {
        [class_level] if class_level.level == HYBRID_BASELINE_LEVEL => {
            match class_level.class_id.as_str() {
                PALADIN_CLASS_ID => Some(HybridClass::Paladin),
                RANGER_CLASS_ID => Some(HybridClass::Ranger),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Surface direct SD13-E3-F6 runtime evidence for the deterministic Human Paladin
/// level-1 and Human Ranger level-1 hybrid chassis.
///
/// This deliberately does not compute a supported hybrid chassis on its own. It only
/// leaves one chassis-recognition explanation so the `class:paladin:1` /
/// `class:ranger:1` identity is acknowledged as a hybrid martial baseline rather than
/// an undocumented packet placeholder (direct runtime evidence, carrying no fabricated
/// mechanical value).
///
/// **This function used to also emit a second claim-blocking diagnostic**
/// (`class_feature.hybrid.<class>.unsupported`) flatly asserting the non-spell
/// class-feature burden -- Smite Evil / lay on hands / divine grace / mercy for
/// Paladin, favored enemy / combat style / tracking for Ranger -- "are not
/// implemented in this bounded hybrid chassis baseline". That diagnostic was a
/// baseline placeholder meant to be superseded once real per-class work landed on
/// top of it. It has been: `explain_paladin_level1_chassis_and_spell_burden_separation`
/// and `explain_ranger_level1_chassis_and_class_feature_separation` are ALSO
/// dispatched unconditionally for the exact same deterministic Human level-1 input
/// (see the call sites immediately below this function's own call site) and ground
/// REAL, non-fabricated values for exactly the burden this diagnostic claimed was
/// unimplemented: `class_chassis.paladin.smite_evil_attack_bonus` /
/// `_damage_bonus`, and `class_chassis.ranger.track` /
/// `favored_enemy_skill_bonus` / `favored_enemy_attack_damage_bonus`. Emitting both
/// at once was a genuine self-contradiction visible in a real character's computed
/// output (Paladin and Ranger are both `Computed`-status classes a user can build
/// and save) -- not internal bookkeeping on an unreachable path. The per-class
/// feature-burden diagnostic is therefore retired here; the per-class functions'
/// own remaining named-but-unproven burdens (Divine Bond, a second mercy slot, the
/// favored-enemy conditional-application engine, etc.) are already tracked by
/// their own doc comments and matrix rows, not by this now-superseded blanket
/// claim.
///
/// **The later hybrid SPELL burden diagnostic (`class_spell.hybrid.<class>.unsupported`)
/// has now been retired too** (v0.6 alpha swarm, 2026-07-28), for exactly the same
/// reason, and the sentence that used to sit here justifying its survival --
/// "Paladin/Ranger spellcasting genuinely remains unimplemented at level 1, and
/// nothing grounds a spell posture that would contradict it" -- was simply out of
/// date. Two things falsify it:
///
/// 1. **The rules.** In `cr_classes.lst` the `CLASS:Paladin` and `CLASS:Ranger`
///    blocks carry no `CAST:` row whatsoever for class levels 1-3. The first row
///    either class has is at class level 4 (`CAST:0,0`), their `BONUS:CASTERLEVEL`
///    rows are gated `PRECLASS:1,<class>=4` with a `CL-3` effective caster level,
///    and the first nonzero BASE 1st-level slot lands at class level 5
///    (`CAST:0,1`). A level-1 Paladin/Ranger having no slots, no caster level and
///    no prepared-spell posture is therefore the CORRECT computed answer, not a
///    missing one. Treating "this class has no spellcasting at this level" as an
///    unimplemented gap rather than a satisfied condition is the actual bug.
///
/// 2. **The code.** `explain_paladin_level1_chassis_and_spell_burden_separation`
///    and `explain_ranger_level1_chassis_and_class_feature_separation` are
///    dispatched for this exact same input and, since the 2026-07-24/25 slices,
///    validate and ground the real spell posture UNCONDITIONALLY at every level
///    (the `unmet_*_prepared_spell_conditions` / `ground_*_prepared_spells` pair
///    sits above their own single-class/Human gate). At level 1 they ground
///    `class_chassis.<class>.partial_caster.effective_caster_level` = 0,
///    `...spell_level_access` = 0, `class_spell.<class>.daily_preparation` = 0,
///    and correctly emit no per-day slot record at all (because
///    `<class>_base_spells_per_day_table(1)` is `[None; 4]`). So a spell posture
///    IS grounded here, and the blanket "out of scope" claim contradicted it --
///    the identical self-contradiction that retired the feature-burden sibling.
///
/// Note the level-1-only asymmetry this resolves was never rules-driven: this
/// function is reached only via `hybrid_level1_class`, which hard-matches
/// `level == HYBRID_BASELINE_LEVEL`. Levels 2-20 never "passed" a spell check --
/// they were structurally never subject to this diagnostic. Levels 2 and 3 have
/// exactly as little spellcasting as level 1 and reached `Computed` throughout,
/// which is itself the evidence that correct-absence is the right treatment at
/// level 1 too.
///
/// The honest blockers are untouched: `class_spell.<class>.partial_caster.unsupported`
/// still claim-blocks a genuinely invalid prepared-spell posture (off-list spell,
/// spell level above the access ceiling, over-prepared slot) at any level,
/// including a Paladin/Ranger inside a multiclass mix. Pinned by
/// `tests/v06_hybrid_level1_no_spellcasting_is_computed.rs`.
///
/// This seam therefore now only makes the hybrid class identity legible on the
/// runtime path; it no longer imposes a blocked posture of its own.
pub(super) fn explain_hybrid_level1_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(hybrid) = hybrid_level1_class(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    let (class_id, class_name, chassis_id) = match hybrid {
        HybridClass::Paladin => (
            PALADIN_CLASS_ID,
            "Paladin",
            "class_chassis.hybrid_baseline.paladin",
        ),
        HybridClass::Ranger => (
            RANGER_CLASS_ID,
            "Ranger",
            "class_chassis.hybrid_baseline.ranger",
        ),
    };

    // Direct runtime evidence: recognize the deterministic Human hybrid level-1 chassis
    // identity. This is a recognition record only; it fabricates no mechanical value.
    explanations.push(ComputationExplanation {
        id: chassis_id.to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human {class_name} level {HYBRID_BASELINE_LEVEL} hybrid chassis: \
             the {class_id}:{HYBRID_BASELINE_LEVEL} class identity is acknowledged as a hybrid martial \
             baseline on the rules-core seam rather than an undocumented packet placeholder. This is a \
             bounded chassis-recognition record only; it grounds no {class_name} class-feature math and \
             no spell posture, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Nothing further is emitted here. Both blanket burden diagnostics this
    // function used to push -- the non-spell `class_feature.hybrid.<class>.unsupported`
    // and the later-spell `class_spell.hybrid.<class>.unsupported` -- have now been
    // retired for the same reason, so this function structurally cannot self-block
    // (its signature no longer takes a `diagnostics` parameter at all, matching
    // `explain_ranger_level1_chassis_and_class_feature_separation`'s original shape).
    // See this function's own doc comment.
}

