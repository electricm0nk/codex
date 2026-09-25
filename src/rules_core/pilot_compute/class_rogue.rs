#[allow(unused_imports)]
pub(crate) use super::*;

pub(super) fn ground_unchained_rogue_class_features(
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if let Some(dice) = rogue_features::sneak_attack_dice(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.sneak_attack_dice".to_owned(),
            value: dice,
            detail: format!(
                "Unchained Rogue level {level} Sneak Attack: {dice}d{} ((level + 1) / 2). A \
                 standalone magnitude -- this engine has no attack-resolution or damage total to \
                 add the dice to",
                rogue_features::SNEAK_ATTACK_DIE_SIZE
            ),
        });
    }
    if let Some(bonus) = rogue_features::trapfinding_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.trapfinding_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Unchained Rogue level {level} Trapfinding: +{bonus} (max(level / 2, 1)) on \
                 Perception checks to locate traps and on Disable Device checks"
            ),
        });
    }
    if let Some(bonus) = rogue_features::danger_sense_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.danger_sense_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Unchained Rogue level {level} Danger Sense: +{bonus} (level / 3) on Reflex saves \
                 to avoid traps, as a dodge bonus to Armor Class against traps, AND on Perception \
                 checks to avoid being surprised. That third clause is what makes it Danger Sense \
                 rather than the Core Rulebook Rogue's Trap Sense"
            ),
        });
    }
    if let Some(talents) = rogue_features::rogue_talents_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.rogue_talents_known".to_owned(),
            value: i16::from(talents),
            detail: format!(
                "Unchained Rogue level {level} Rogue Talents: a pool of {talents} (level / 2). \
                 The talent catalogue is not ingested for this book, so this is the size of the \
                 pool and not a claim that the options exist"
            ),
        });
    }
    if let Some(choices) = rogue_features::finesse_training_weapon_choices(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.finesse_training_weapon_choices".to_owned(),
            value: i16::from(choices),
            detail: format!(
                "Unchained Rogue level {level} Finesse Training: {choices} weapon choices \
                 ((level + 5) / 8) that add Dexterity instead of Strength to damage. Absent \
                 entirely from the Core Rulebook Rogue -- one of the features that makes this a \
                 different class rather than a re-skin"
            ),
        });
    }
    if let Some(unlocks) = rogue_features::rogues_edge_skill_unlocks(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.rogues_edge_skill_unlocks".to_owned(),
            value: i16::from(unlocks),
            detail: format!(
                "Unchained Rogue level {level} Rogue's Edge: {unlocks} skill unlocks (level / 5). \
                 Absent entirely from the Core Rulebook Rogue. The skill-unlock content itself is \
                 not ingested"
            ),
        });
    }
    if let Some(dc) = rogue_features::master_strike_dc(level, ability_modifiers.intelligence) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.master_strike_dc".to_owned(),
            value: dc,
            detail: format!(
                "Unchained Rogue level {level} Master Strike: save DC {dc} \
                 (10 + level / 2 + Intelligence modifier {}). A standalone magnitude -- there is \
                 no saving-throw resolution here for it to be rolled against",
                ability_modifiers.intelligence
            ),
        });
    }
    if let Some(flanking_level) = rogue_features::uncanny_dodge_flanking_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.uncanny_dodge_flanking_level".to_owned(),
            value: i16::from(flanking_level),
            detail: format!(
                "Unchained Rogue level {level} Uncanny Dodge: counts as a level-{flanking_level} \
                 defender for the flanking comparison"
            ),
        });
    }
    if let Some(steps) = rogue_features::uncanny_dodge_tracker_steps(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.uncanny_dodge_tracker_steps".to_owned(),
            value: i16::from(steps),
            detail: format!(
                "Unchained Rogue level {level} Uncanny Dodge Tracker: {steps} satisfied step(s) \
                 -- Uncanny Dodge, then Improved Uncanny Dodge"
            ),
        });
    }
    // Debilitating Injury: the Unchained Rogue's headline feature, and the
    // one that made "23 of 64 compute nothing" a player-visible problem. Its
    // corpus row (`:583`) is a bare declaration plus a DESC: -- no BONUS:, no
    // DEFINE:, nothing PCGen itself computes -- so both numbers below are
    // read out of its own sentences, quoted verbatim in
    // `rogue_features::prose_derived` and re-checked against the ingested
    // record by that module's tests.
    if let Some(penalty) = rogue_features::prose_derived::general_penalty(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.debilitating_injury_penalty".to_owned(),
            value: penalty,
            detail: format!(
                "Unchained Rogue level {level} Debilitating Injury: sneak attack damage also \
                 imposes one chosen penalty for {} round -- Bewildered ({penalty} to Armor \
                 Class), Disoriented ({penalty} on attack rolls) or Hampered (all speeds halved, \
                 minimum 5 feet, and no 5-foot step). Only one may afflict a target at a time; \
                 further sneak attacks extend it a round each. A standalone magnitude: this \
                 engine resolves no attacks, so nothing consumes it",
                rogue_features::prose_derived::DURATION_ROUNDS
            ),
        });
    }
    if let Some(penalty) = rogue_features::prose_derived::penalty_vs_the_rogue(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_rogue.debilitating_injury_penalty_against_this_rogue"
                .to_owned(),
            value: penalty,
            detail: format!(
                "Unchained Rogue level {level} Debilitating Injury: against the rogue's own \
                 attacks the Bewildered and Disoriented penalties total {penalty}, escalating by \
                 -2 at level 10 and again at level 16 to the row's stated maximum of -8. The \
                 sentence is ambiguous about whether the escalation applies to the additional \
                 penalty or the combined one, but both readings converge on -4 / -6 / -8 and the \
                 stated cap confirms them, so nothing is being guessed between competing answers"
            ),
        });
    }
    let class_skills = rogue_features::class_skills();
    explanations.push(ComputationExplanation {
        id: "class_feature.pu.unchained_rogue.class_skill_count".to_owned(),
        value: class_skills.len() as i16,
        detail: format!(
            "Unchained Rogue class skills, as the book's own class-skill list states them \
             ({} entries): {}",
            class_skills.len(),
            render_class_skill_list(class_skills)
        ),
    });

    push_deferred_class_features(
        "class_feature.pu.unchained_rogue.other_features_deferred.unsupported",
            "class:unchained_rogue grounds every Unchained Rogue magnitude this book states as a \
             formula token: the chassis (borrowed unchanged from the Core Rulebook Rogue, which \
             the corpus record confirms it does not override), Sneak Attack dice, Trapfinding, \
             Danger Sense, the Rogue Talent pool, Finesse Training's weapon choices, Rogue's Edge \
             skill unlocks, Master Strike's save DC, the Uncanny Dodge flanking level and tracker \
             steps, and the class-skill list -- plus Debilitating Injury, whose penalties are \
             stated only in its own prose and are now derived from it. This diagnostic is NOT \
             claim-blocking; it carries the honest remainder. What is missing: (1) the \
             rogue-talent and skill-unlock catalogues are not ingested -- the pool sizes are \
             real, the option lists are not; (2) APPLICATION rather than magnitude -- Sneak \
             Attack dice have no damage total, Master Strike's DC no save resolution, Danger \
             Sense's dodge bonus no trap encounter to apply against, and Debilitating Injury no \
             attack resolution to be imposed by; (3) Debilitating Injury's Hampered option \
             (speeds halved to a minimum of 5 feet, no 5-foot step) is a movement effect on a \
             TARGET, and this engine models no targets, so only the two penalty magnitudes are \
             grounded; (4) Unchained Rogue's own Evasion row is empty -- no DESC, no token -- and \
             SERVESAS the shared Core Rulebook `Rogue ~ Evasion` record, so any magnitude belongs \
             to that record and not to this book; Improved Uncanny Dodge's qualitative clause and \
             the weapon/armor proficiency rows likewise carry no numeric token"
            .to_owned(),
        explanations,
        diagnostics,
    );
}

pub(super) const ROGUE_CLASS_ID: &str = "class:rogue";

/// PF1 Core Rulebook level gate of the Rogue's first talent, verified
/// identically on both primary sources: "Starting at 2nd level, a rogue
/// gains one rogue talent. She gains an additional rogue talent for every 2
/// levels of rogue attained after 2nd level. A rogue cannot select an
/// individual talent more than once." Only the 2nd-level slot is recognized
/// here; the 4th/6th/8th/10th additional talents stay named-but-unproven as
/// future NUMBERED slots per the proven monk-second-bonus-feat idiom.
pub(super) const ROGUE_TALENT_GRANT_LEVEL: u8 = 2;

pub(super) const ROGUE_TALENT_CHOICE_ID: &str = "choice:rogue_talent";

/// PF1 Core Rulebook level gate of the Rogue's SECOND talent — the first
/// "additional rogue talent for every 2 levels of rogue attained after 2nd
/// level" lands at rogue level 4. A numbered choice slot per the proven
/// monk-second-bonus-feat repeat-grant idiom; the 6th/8th/10th additional
/// talents stay named-but-unproven as further numbered slots.
pub(super) const ROGUE_SECOND_TALENT_GRANT_LEVEL: u8 = 4;

pub(super) const ROGUE_SECOND_TALENT_CHOICE_ID: &str = "choice:rogue_talent_2";

// A further SD13-E5 slice widens the gate to level 9 — the first level-9 slice
// in the tranche (verified independently against d20pfsrd and
// legacy.aonprd.com): level 9 base attack bonus stays +6 (9 * 3 / 4, an
// integer-division coincidence with level 8) while poor Fortitude/Will both
// genuinely rise to +3 (9 / 3) and good Reflex stays +6 (9 / 2 + 2, another
// coincidence); the level-9 "Special" column reads "Sneak attack +5d6, trap
// sense +3" — BOTH entries are tier-rises on already-grounded formula pillars,
// not new class features: the sneak attack die count genuinely rises to 5 via
// the pre-existing (level + 1) / 2 formula and Trap Sense genuinely rises to
// +3 via the pre-existing level / 3 formula; Trapfinding stays 4
// (max(9/2, 1), a coincidence); level 9 is NOT a rogue-talent level (talents
// land at 2/4/6/8/10...), so no new pillar is grounded and nothing new is
// left unproven for the talent tree either. A further SD13-E5 slice widens
// the gate to level 10 — the first level-10 slice, opening the tranche's
// final level band (verified independently against d20pfsrd and
// legacy.aonprd.com): level 10 base attack genuinely rises to +7
// (10 * 3 / 4) and good Reflex genuinely rises to +7 (10 / 2 + 2), while
// poor Fortitude/Will both stay +3 (10 / 3, integer-division coincidences);
// sneak attack stays 5d6 ((10 + 1) / 2, the odd-level cadence — next rise
// at 11th) and Trap Sense stays +3 (10 / 3, next rise at 12th), while
// Trapfinding genuinely rises to +5 (max(10/2, 1)); the level-10 "Special"
// column reads "Advanced talents, rogue talent" — BOTH parts of the same
// genuinely open-ended choice-list feature already left named-but-unproven
// at levels 2/4/6/8 (the advanced-talent unlock is a list expansion of that
// feature, not a new pillar), so no new pillar is grounded at level 10
// either.
// SD18 widening (cycle-2026-07-14T2000, tests/sd18_rogue_level11_sneak_attack.rs):
// widens the gate to level 11, verified independently against d20pfsrd and
// the Archives of Nethys aonprd.com mirror (both agree byte-for-byte): base
// attack genuinely rises to +8 (11 * 3 / 4) while all three base saves stay
// numerically unchanged (Fortitude/Will 11/3 = 3, Reflex 11/2+2 = 7,
// integer-division coincidences with level 10); the level-11 "Special"
// column reads only "Sneak attack +6d6" — the pre-existing sneak-attack
// die-count formula ((level + 1) / 2) genuinely rises to 6 (i.e. 6d6), up
// from 5 (5d6) at level 10, via the same formula, not a new record; Trap
// Sense stays +3 (11/3, next rise at 12th) and Trapfinding stays 5
// (max(11/2, 1), a coincidence); Evasion, Uncanny Dodge, and Improved
// Uncanny Dodge all stay granted, not re-derived; level 11 is NOT a
// rogue-talent level (talents land at 2/4/6/8/10/12), so no new talent
// pillar is grounded or fabricated either.
// SD18 widening (cycle-2026-07-15T0800, tests/sd18_rogue_level12_widening.rs):
// widens the gate to level 12, verified independently against d20pfsrd and
// the Archives of Nethys aonprd.com mirror (both agree byte-for-byte): base
// attack genuinely rises to +9 (12 * 3 / 4) and all three base saves
// genuinely rise too (Fortitude/Will 12/3 = 4, Reflex 12/2+2 = 8, up from
// 3/3/7 at level 11); the level-12 "Special" column reads "Rogue talent,
// trap sense +4" — Trap Sense genuinely rises to +4 (12/3) via the
// pre-existing formula, not a new record, and Rogue Talent is the SIXTH
// numbered choice slot (talents land at 2/4/6/8/10/12), surfaced via the
// same open-ended, non-validated raw-string idiom already proven at slots
// 1-5 (see ROGUE_SIXTH_TALENT_GRANT_LEVEL / ROGUE_SIXTH_TALENT_CHOICE_ID
// below); sneak attack stays 6d6 ((12 + 1) / 2, an integer-division
// coincidence with level 11, next rise at level 13) but Trapfinding
// genuinely rises to 6 (max(12/2, 1), up from 5 at level 11, via the
// pre-existing formula — this rise is not named in the level-12 "Special"
// column, since Trapfinding's own formula is independent of it); Evasion,
// Uncanny Dodge, and Improved Uncanny Dodge all stay granted, not
// re-derived.
// SD18 widening (cycle-2026-07-15T1100, tests/sd18_rogue_level13_widening.rs):
// widens the gate to level 13, the first §3.2 level-13 widening attempted
// across any of the 11 core classes (all 11 landed level 12 as of
// cycle-2026-07-14T2244), verified independently against d20pfsrd and the
// Archives of Nethys aonprd.com mirror (both agree byte-for-byte): base
// attack STAYS +9 (13 * 3 / 4, an integer-division coincidence with level
// 12) and all three base saves also STAY unchanged (Fortitude/Will 13/3 =
// 4, Reflex 13/2+2 = 8); the level-13 "Special" column reads only "Sneak
// attack +7d6" — a tier-rise on the already-grounded sneak-attack die-count
// formula ((level + 1) / 2), which genuinely rises to 7 (7d6), up from 6
// (6d6) at level 12, via the same formula, not a new record; Trap Sense
// stays +4 (13/3, next rise at level 15) and Trapfinding stays 6
// (max(13/2, 1), unchanged from level 12); Evasion, Uncanny Dodge, and
// Improved Uncanny Dodge all stay granted, not re-derived; level 13 is NOT
// a rogue-talent level (talents land at 2/4/6/8/10/12/14...), so no seventh
// talent choice-slot record is grounded or fabricated either. This is the
// cleanest possible widening shape: the ONLY value that genuinely changes
// is the sneak-attack die count, entirely through the pre-existing formula
// — zero new record types, zero new named pillars, zero new choice slots.
// SD18 widening (cycle-2026-07-15T2000, tests/sd18_rogue_level14_widening.rs):
// widens the gate to level 14, the loop's Rogue level-14 sweep landing,
// verified independently against both primary sources (d20pfsrd and the
// Archives of Nethys aonprd.com mirror, which agree byte-for-byte): base
// attack genuinely rises to +10 (14 * 3 / 4) and good Reflex genuinely
// rises to +9 (14 / 2 + 2), up from 9/8 at level 13, while poor
// Fortitude/Will both stay +4 (14 / 3, integer-division coincidences); the
// level-14 "Special" column reads only "Rogue talent" — level 14 IS a
// rogue-talent cadence level (talents land at 2/4/6/8/10/12/14), so a
// SEVENTH numbered choice-recognition slot (choice:rogue_talent_7) is
// added, mirroring the proven open-ended raw-string idiom used at slots 1-6
// exactly; sneak attack stays 7d6 ((14 + 1) / 2, an integer-division
// coincidence with level 13, next rise at level 15) but Trapfinding
// genuinely rises to 7 (max(14/2, 1), up from 6 at level 13, via the
// pre-existing formula — this rise is not named in the level-14 "Special"
// column); Trap Sense stays +4 (14/3, next rise at level 15); Evasion,
// Uncanny Dodge, and Improved Uncanny Dodge all stay granted, not
// re-derived.
// SD18 widening (cycle-2026-07-15T2900, tests/sd18_rogue_level15_widening.rs):
// widens the gate to level 15, the loop's Rogue level-15 sweep landing,
// verified independently against both primary sources (d20pfsrd and the
// Archives of Nethys aonprd.com mirror, which agree byte-for-byte): base
// attack genuinely rises to +11 (15 * 3 / 4) and poor Fortitude/Will both
// genuinely rise to +5 (15 / 3), while good Reflex STAYS +9 (15 / 2 + 2, an
// integer-division coincidence with level 14); the level-15 "Special"
// column reads only "Sneak attack +8d6, trap sense +5" — both entries are
// tier-rises on already-grounded formula pillars, not new class features:
// the sneak-attack die-count formula ((level + 1) / 2) genuinely rises to 8
// (8d6), up from 7d6 at level 14, and the Trap Sense flat-magnitude formula
// (level / 3) genuinely rises to +5, up from +4 at level 14; Trapfinding
// stays 7 (max(15/2, 1), an integer-division coincidence with level 14);
// level 15 is NOT a rogue-talent level (talents land at
// 2/4/6/8/10/12/14/16...), so no eighth talent choice-slot record is
// grounded or fabricated either; Evasion, Uncanny Dodge, and Improved
// Uncanny Dodge all stay granted, not re-derived. This is the cleanest
// possible widening shape, mirroring the Barbarian level-15 landing: zero
// new record types, zero new named pillars, zero new choice slots — the
// ONLY production-code change is this ceiling raise.
// SD18 widening (cycle-2026-07-15T5200, tests/sd18_rogue_level16_widening.rs):
// widens the gate to level 16, the loop's Rogue level-16 sweep landing (the
// FOURTH §3.2 level-16 landing, after Barbarian, Fighter, and Wizard),
// verified independently against both primary sources (d20pfsrd and the
// Archives of Nethys aonprd.com mirror, which agree byte-for-byte): base
// attack genuinely rises to +12 (16 * 3 / 4) and good Reflex genuinely rises
// to +10 (16 / 2 + 2), up from 11/9 at level 15, while poor Fortitude/Will
// both stay +5 (16 / 3, integer-division coincidences); the level-16
// "Special" column reads only "Rogue talent" — level 16 IS a rogue-talent
// cadence level (talents land at 2/4/6/8/10/12/14/16), so an EIGHTH
// numbered choice-recognition slot (choice:rogue_talent_8) is added,
// mirroring the proven open-ended raw-string idiom used at slots 1-7
// exactly; sneak attack stays 8d6 ((16 + 1) / 2, an integer-division
// coincidence with level 15, next rise at level 17) but Trapfinding
// genuinely rises to 8 (max(16/2, 1), up from 7 at level 15, via the
// pre-existing formula — this rise is not named in the level-16 "Special"
// column); Trap Sense stays +5 (16/3, next rise at level 18); Evasion,
// Uncanny Dodge, and Improved Uncanny Dodge all stay granted, not
// re-derived.
// SD18 widening (cycle-2026-07-15T8100, tests/sd18_rogue_level17_widening.rs):
// widens the gate to level 17, the loop's THIRD §3.2 level-17 sweep landing
// (after Ranger and Bard), verified independently against both primary
// sources (d20pfsrd and the Archives of Nethys aonprd.com mirror, which
// agree byte-for-byte, fetched across the full levels-14-18 block to guard
// against level-misattribution): base attack stays +12 (17 * 3 / 4) and all
// three base saves stay unchanged (Fortitude/Will 17/3=5, Reflex 17/2+2=10),
// all integer-division coincidences with level 16; the level-17 "Special"
// column reads only "Sneak attack +9d6" — level 17 is NOT a rogue-talent
// cadence level (talents land at 2/4/6/8/10/12/14/16, next at 18), so no
// ninth talent slot is grounded; the sneak-attack die-count formula
// ((level + 1) / 2) genuinely rises to 9d6, up from 8d6 at level 16, via
// the same pre-existing formula, not a new record; Trap Sense stays +5
// (17/3, next rise at level 18) and Trapfinding stays 8 (max(17/2, 1), an
// integer-division coincidence with level 16), neither named in the
// level-17 "Special" column; Evasion, Uncanny Dodge, and Improved Uncanny
// Dodge all stay granted, not re-derived. This needs ZERO new tier
// constants and ZERO new choice slots — the ONLY production-code change is
// this ceiling raise.
// SD18 widening (cycle-2026-07-16T0212, tests/sd18_rogue_level18_widening.rs):
// widens the gate to level 18, the loop's SIXTH §3.2 level-18 sweep landing
// (after Wizard, Cleric, Paladin, Fighter, and Barbarian), verified
// independently against both primary sources (d20pfsrd and the Archives of
// Nethys aonprd.com mirror, which agree byte-for-byte, fetched across the
// full levels-16-19 block to guard against level-misattribution): base
// attack genuinely rises to +13 (18 * 3 / 4) and all three base saves
// genuinely rise too (Fortitude/Will 18/3=6, Reflex 18/2+2=11, up from
// 12/5/5/10 at level 17); the level-18 "Special" column reads "Rogue
// talent, trap sense +6" — level 18 IS a rogue-talent cadence level
// (talents land at 2/4/6/8/10/12/14/16/18), so a NINTH numbered
// choice-recognition slot (choice:rogue_talent_9) is added, mirroring the
// proven open-ended raw-string idiom used at slots 1-8 exactly; and the
// pre-existing Trap Sense flat-magnitude formula (level / 3) genuinely
// rises to +6, up from +5 at level 17, via the same formula, not a new
// record; the sneak-attack die-count formula ((level + 1) / 2) stays at 9
// (9d6, an integer-division coincidence with level 17, next rise at level
// 19), not named in the level-18 "Special" column; Trapfinding genuinely
// rises to 9 (max(18/2, 1), up from 8 at level 17, via its own independent
// pre-existing formula), also not named in the level-18 "Special" column;
// Evasion, Uncanny Dodge, and Improved Uncanny Dodge all stay granted, not
// re-derived. This needs ZERO new tier constants for base-attack/save/
// trap-sense/trapfinding/sneak-attack (all already level-generic
// formulas) — the ONLY production-code changes are this ceiling raise and
// a ninth numbered talent slot appended to the existing tuple-array idiom.
// SD18 widening (cycle-2026-07-16T3600, tests/sd18_rogue_level19_widening.rs):
// widens the gate to level 19, the loop's SEVENTH §3.2 level-19 sweep
// landing (after Barbarian, Cleric, Fighter, Bard, Paladin, and Ranger),
// verified independently against both primary sources (d20pfsrd and the
// Archives of Nethys aonprd.com mirror, which agree byte-for-byte, fetched
// across the full levels-15-20 block to guard against
// level-misattribution): base attack genuinely rises to +14 (19 * 3 / 4),
// while all three base saves stay put (Fortitude/Will 19/3=6, Reflex
// 19/2+2=11, integer-division coincidences with level 18); the level-19
// "Special" column reads only "Sneak attack +10d6" — 19 is NOT a
// rogue-talent cadence level (talents land at 2/4/6/8/10/12/14/16/18, next
// at 20), so no tenth numbered talent slot is added; the sneak-attack
// die-count formula ((level + 1) / 2) genuinely rises to 10 (10d6, up from
// 9d6 at level 18, its own final PF1 CRB tier); Trap Sense stays +6 (19/3,
// an integer-division coincidence with level 18) and Trapfinding stays 9
// (max(19/2, 1), also a coincidence), neither named in the level-19
// "Special" column; Evasion, Uncanny Dodge, and Improved Uncanny Dodge all
// stay granted, not re-derived.
// SD18 widening (cycle-2026-07-16T1431, tests/sd18_rogue_level20_widening.rs):
// widened again to level 20 (verified independently against both d20pfsrd
// and the Archives of Nethys aonprd.com mirror, byte-for-byte agreement).
// Level 20's "Special" column reads "Master strike, rogue talent" — base
// attack bonus genuinely rises to +15 (20*3/4) and good Reflex genuinely
// rises to +12 (20/2+2), while poor Fortitude/Will both stay +6 (20/3,
// integer-division coincidences with level 19); sneak attack stays 10d6
// ((20+1)/2, its own final PF1 CRB tier) and Trap Sense stays +6 (20/3,
// its own final PF1 CRB tier), neither named in the level-20 "Special"
// column; Trapfinding genuinely rises to 10 (max(20/2,1)), also not named.
// 20 IS a rogue-talent cadence level (talents land at
// 2/4/6/8/10/12/14/16/18/20), so a TENTH numbered talent slot is appended
// to the existing tuple-array idiom; Master Strike, the rogue's 20th-level
// capstone, is newly granted as a bounded grant-only identity record
// (value 0, non-fabricated) mirroring exactly the already-proven Paladin
// Holy Champion / Ranger Master Hunter capstone idiom — no
// action-economy, attack-resolution, or saving-throw-resolution engine
// exists anywhere in this codebase, so this grounds no actual mechanic.
// This closes Rogue's own per-level arithmetic-widening frontier: level 20
// is the final level within PF1's 1-20 character-level cap.
pub(super) const MAX_SUPPORTED_ROGUE_LEVEL: u8 = 20;

/// PF1 Core Rulebook level gate at which Rogue gains Master Strike (the
/// 20th-level capstone, verified independently against d20pfsrd and the
/// Archives of Nethys aonprd.com mirror).
pub(super) const ROGUE_MASTER_STRIKE_LEVEL: u8 = 20;

/// PF1 Core Rulebook level gate at which Rogue gains Evasion.
pub(super) const ROGUE_EVASION_LEVEL: u8 = 2;

/// PF1 Core Rulebook level gate at which Rogue gains Trap Sense.
pub(super) const ROGUE_TRAP_SENSE_LEVEL: u8 = 3;

/// PF1 Core Rulebook level gate at which Rogue gains Uncanny Dodge (4th
/// level, verified independently against d20pfsrd and legacy.aonprd.com —
/// the Rogue class table's level-4 "Special" column reads "Rogue talent,
/// uncanny dodge" — NOT the same level as Barbarian's own 2nd-level Uncanny
/// Dodge grant).
pub(super) const ROGUE_UNCANNY_DODGE_LEVEL: u8 = 4;

/// PF1 Core Rulebook level gate at which Rogue gains Improved Uncanny Dodge
/// (8th level, verified independently against d20pfsrd and
/// legacy.aonprd.com — the Rogue class table's level-8 "Special" column
/// reads "Improved uncanny dodge, rogue talent." This is a DIFFERENT gate
/// level than Barbarian's own Improved Uncanny Dodge grant, which is at
/// barbarian level 5, not rogue's level 8; verified rather than assumed).
pub(super) const ROGUE_IMPROVED_UNCANNY_DODGE_LEVEL: u8 = 8;

/// The bounded Rogue milestone level this decomposition surface grounds, if
/// any. Returns the single Rogue level when the chosen input is exactly a
/// single-class Rogue at one of the supported milestone levels (1 through
/// 10). Returns `None` for no Rogue, a non-Rogue class, a multiclass
/// mix, or any level-11+ Rogue this slice deliberately does not recognize —
/// each of which stays claim-blocked exactly as before. Mirrors the Fighter
/// `supported_fighter_level` / Paladin `supported_paladin_level` level-range
/// gate idiom.
pub(super) fn supported_rogue_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == ROGUE_CLASS_ID
                && (1..=MAX_SUPPORTED_ROGUE_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

pub(super) fn explain_rogue_level1_chassis(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = supported_rogue_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Rogue
    // chassis identity at the supported level. This is a recognition record
    // only; it fabricates no mechanical value.
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.bounded_progression".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Rogue level {level} chassis: the \
             {ROGUE_CLASS_ID}:{level} class identity is acknowledged on the \
             rules-core seam rather than an undocumented packet placeholder. This is a bounded \
             chassis-recognition record only; the base-attack, base-save, sneak-attack \
             die-count, trapfinding, Evasion, Trap Sense, Uncanny Dodge, and Improved Uncanny \
             Dodge pillars are grounded separately below, but this record still grounds no \
             rogue talent and no level-9+ progression, so it carries no fabricated mechanical \
             value (+0)"
        ),
    });

    // Grounded (1/8): base-attack progression (3/4 BAB).
    let level_value = i16::from(level);
    let base_attack_bonus = level_value * 3 / 4;
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Rogue level {level} base attack bonus from the PF1 Core Rulebook \
             Rogue class table's 3/4-BAB progression: level * 3 / 4 = {base_attack_bonus}"
        ),
    });

    // Grounded (2/8): base-save progression (good Reflex, poor Fortitude, poor Will).
    let base_save_fortitude = level_value / 3;
    let base_save_reflex = level_value / 2 + 2;
    let base_save_will = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.base_save.fortitude".to_owned(),
        value: base_save_fortitude,
        detail: format!(
            "Rogue level {level} base Fortitude save (poor) from the PF1 Core \
             Rulebook Rogue class table: level / 3 = {base_save_fortitude}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.base_save.reflex".to_owned(),
        value: base_save_reflex,
        detail: format!(
            "Rogue level {level} base Reflex save (good) from the PF1 Core \
             Rulebook Rogue class table: level / 2 + 2 = {base_save_reflex}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.base_save.will".to_owned(),
        value: base_save_will,
        detail: format!(
            "Rogue level {level} base Will save (poor) from the PF1 Core \
             Rulebook Rogue class table: level / 3 = {base_save_will}"
        ),
    });

    // Grounded (3/8): sneak attack damage-die count only. PF1 Core Rulebook:
    // the sneak attack die count increases by 1d6 every two rogue levels
    // (1d6 at levels 1-2, 2d6 at level 3+): (level + 1) / 2.
    let sneak_attack_die_count = (level_value + 1) / 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.sneak_attack".to_owned(),
        value: sneak_attack_die_count,
        detail: format!(
            "Rogue level {level} sneak attack from the PF1 Core Rulebook Rogue \
             class table: the sneak attack die count increases by 1 every two rogue levels \
             (1d6 at levels 1-2, 2d6 at level 3+): (level + 1) / 2 = ({level_value} + 1) / 2 = \
             {sneak_attack_die_count}, i.e. {sneak_attack_die_count}d6 sneak attack damage die, \
             against a flanked or Dexterity-denied target. Only the die-count facet is grounded \
             here; damage-roll execution and the flanking / Dexterity-denial trigger-condition \
             engine are not implemented"
        ),
    });

    // Grounded (4/8): trapfinding — the flat numeric bonus and the
    // magic-trap-disarm statement only, mirroring the grounded Ranger Track
    // record (no check-execution engine behind it).
    let trapfinding_bonus = (level_value / 2).max(1);
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.trapfinding".to_owned(),
        value: trapfinding_bonus,
        detail: format!(
            "Rogue Trapfinding class feature: adds a bonus equal to max(rogue level / 2, 1) \
             (PF1 Core Rulebook Trapfinding: +1/2 rogue level, minimum +1) on Perception checks \
             made to locate traps and on Disable Device checks, and lets the rogue use Disable \
             Device to disarm magic traps. At Rogue level {level} this bonus is \
             max({level_value} / 2, 1) = {trapfinding_bonus}. This grounds only the \
             flat numeric Trapfinding bonus and the magic-trap-disarm statement; it is not a \
             check-execution engine and computes no full Perception or Disable Device check, no \
             trap DC resolution, and no magic-trap disarm engine"
        ),
    });

    // Grounded (5/8): Evasion, a 2nd-level Rogue class feature. Below the
    // level-2 gate this is a correct PF1 Core Rulebook level-gate absence
    // (value 0); at or above it, it is a bounded identity/recognition record
    // only (value 0, non-fabricated) naming the rule text — mirroring how
    // Divine Grace and Bravery were grounded as flat rules-text records
    // without folding into an actual saving-throw-resolution or
    // damage-resolution engine, neither of which exists in this codebase.
    if level < ROGUE_EVASION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.evasion".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Evasion at rogue level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant rule is named but not computed. Evasion \
                 is a 2nd-level rogue class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.evasion".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Evasion granted at rogue level {level} (PF1 Core Rulebook, 2nd-level \
                 rogue class feature): if the rogue makes a successful Reflex saving throw \
                 against an attack that normally deals half damage on a successful save, she \
                 instead takes no damage; Evasion has no effect if the rogue fails the saving \
                 throw, and it has no effect at all against attacks that do not allow a saving \
                 throw for half damage. This is a bounded identity/recognition record only \
                 (value 0, non-fabricated): no saving-throw-resolution engine and no \
                 damage-resolution engine exists anywhere in this codebase to apply it, so this \
                 grounds no actual damage reduction on any save outcome"
            ),
        });
    }

    // Grounded (6/8): Trap Sense, a 3rd-level Rogue class feature (verified
    // independently against d20pfsrd and legacy.aonprd.com). Below the
    // level-3 gate this is a correct PF1 Core Rulebook level-gate absence
    // (value 0); at or above it, it is a bounded flat-magnitude record only
    // (level / 3, floor) naming the rule text — mirroring how Fighter's
    // Bravery and Paladin's Divine Grace were grounded as flat rules-text
    // magnitudes without folding into an actual saving-throw-resolution or
    // armor-class-resolution engine, neither of which exists in this
    // codebase.
    if level < ROGUE_TRAP_SENSE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.trap_sense".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Trap Sense at rogue level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant magnitude is named but not computed. \
                 Trap Sense is a 3rd-level rogue class feature."
            ),
        });
    } else {
        let trap_sense_bonus = level_value / 3;
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.trap_sense".to_owned(),
            value: trap_sense_bonus,
            detail: format!(
                "Rogue Trap Sense granted at rogue level {level} (PF1 Core Rulebook, 3rd-level \
                 rogue class feature): a +{trap_sense_bonus} bonus on Reflex saves made to avoid \
                 traps and a +{trap_sense_bonus} dodge bonus to AC against attacks made by traps \
                 (rogue level / 3 = {trap_sense_bonus}; this bonus rises further at 9th/12th/\
                 15th/18th rogue level, beyond this bounded slice). This is a bounded \
                 flat-magnitude record only, non-fabricated: it is never applied to any actual \
                 Reflex-save total or AC total, since no saving-throw-resolution or \
                 armor-class-resolution engine exists anywhere in this codebase to apply it, and \
                 no trap-detection or trap-triggering engine exists to decide when it would \
                 apply"
            ),
        });
    }

    // Grounded (7/8): Uncanny Dodge, a 4th-level Rogue class feature (verified
    // independently against d20pfsrd and legacy.aonprd.com: the Rogue class table's
    // level-4 "Special" column reads "Rogue talent, uncanny dodge" — NOT the same
    // level as Barbarian's own 2nd-level Uncanny Dodge grant, verified rather than
    // assumed). Below the level-4 gate this is a correct PF1 Core Rulebook
    // level-gate absence (value 0); at or above it, it is a bounded
    // identity/recognition record only (value 0, non-fabricated) naming the rule
    // text — mirroring exactly how Barbarian's own Uncanny Dodge was grounded,
    // without folding into any actual flat-footed-state tracking, Armor Class
    // computation, or invisibility-detection engine, none of which exists in this
    // codebase. The level-4 row's OTHER named entry, a Rogue Talent (a genuinely
    // open-ended choice-list feature, a new-subsystem-shaped burden), is
    // deliberately left named-but-unproven this slice, mirroring the Monk level-2
    // bonus feat / Barbarian Rage Power precedent: no new choice-slot and no new
    // diagnostic was added for it.
    if level < ROGUE_UNCANNY_DODGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Uncanny Dodge at rogue level {level}: correctly absent at level {level} \
                 by PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Uncanny Dodge is a 4th-level rogue class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Uncanny Dodge granted at rogue level {level} (PF1 Core Rulebook, \
                 4th-level rogue class feature, part of the \"Rogue talent, uncanny dodge\" \
                 table entry): she cannot be caught flat-footed, and she retains her Dexterity \
                 bonus to Armor Class even if the attacker is invisible; she still loses her \
                 Dexterity bonus to Armor Class if immobilized, and a successful feint action \
                 can still strip it away. This is a bounded identity/recognition record only \
                 (value 0, non-fabricated): no flat-footed-state tracking, no Armor Class \
                 computation, and no invisibility-detection engine exists anywhere in this \
                 codebase to apply it, so this grounds no actual flat-footed immunity or \
                 Dexterity-to-AC retention"
            ),
        });
    }

    // Grounded (8/8): Improved Uncanny Dodge, an 8th-level Rogue class feature
    // (verified independently against d20pfsrd and legacy.aonprd.com: both name
    // "Improved uncanny dodge, rogue talent" as the Rogue 8th-level "Special"
    // class table entry). Below the level-8 gate this is a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it, it is a bounded
    // identity/recognition record only (value 0, non-fabricated) naming the rule
    // text — mirroring exactly how Barbarian's own Improved Uncanny Dodge was
    // grounded at barbarian level 5. The rule's own CONDITIONAL piece (comparing
    // the attacking rogue's own levels against this rogue's own levels to decide
    // whether the immunity is actually pierced) is never applied: no
    // flanking-resolution engine, no attacker-level-comparison engine, and no
    // sneak-attack-trigger engine exists anywhere in this codebase, so this
    // grounds no actual flanking immunity or sneak-attack denial. The level-8
    // row's OTHER named entry, a third Rogue Talent (a genuinely open-ended
    // choice-list feature, a new-subsystem-shaped burden), is deliberately left
    // named-but-unproven this slice, mirroring the level-2/level-4/level-6
    // rogue-talent precedent: no new choice-slot and no new diagnostic was added
    // for it.
    if level < ROGUE_IMPROVED_UNCANNY_DODGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.improved_uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Improved Uncanny Dodge at rogue level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Improved Uncanny Dodge is an 8th-level rogue class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.improved_uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Improved Uncanny Dodge granted at rogue level {level} (PF1 Core \
                 Rulebook, 8th-level rogue class feature, part of the \"Improved uncanny \
                 dodge, rogue talent\" table entry): a rogue of 8th level or higher can no \
                 longer be flanked, denying another rogue the ability to sneak attack her by \
                 flanking unless the attacker has at least four more rogue levels than she \
                 has. This is a bounded identity/recognition record only (value 0, \
                 non-fabricated): no flanking-resolution engine, no \
                 attacker-level-comparison engine, and no sneak-attack-trigger engine exists \
                 anywhere in this codebase to apply it, so this grounds no actual flanking \
                 immunity or sneak-attack denial"
            ),
        });
    }

    // SD13-E5: the 2nd-level rogue talent choice slot, recognized via the
    // open-ended (non-restricted-list) idiom of choice:ranger_favored_enemy /
    // choice:ranger_favored_terrain / choice:paladin_mercy — deliberately NOT
    // an encoding of the talent list, and deliberately NOT the talent tree's
    // own effects (the new-subsystem-shaped burden stays unproven; no
    // talent-effect engine exists in this codebase). The 4th/6th/8th/10th
    // additional talents stay named-but-unproven as future numbered slots per
    // the proven monk-second-bonus-feat repeat-grant idiom.
    if level >= ROGUE_TALENT_GRANT_LEVEL
        && let Some(talent_selection) = choice_selection(input, ROGUE_TALENT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_chassis.rogue.talent_choice".to_owned(),
            value: 0,
            detail: format!(
                "Rogue talent selection ({ROGUE_TALENT_CHOICE_ID} -> {talent_selection}): \
                 PF1 Core Rulebook, verified identically on both primary sources — \
                 \"Starting at 2nd level, a rogue gains one rogue talent. She gains an \
                 additional rogue talent for every 2 levels of rogue attained after 2nd \
                 level. A rogue cannot select an individual talent more than once.\" The \
                 level-{level} talent chosen for this character is {talent_selection}, \
                 recognized as a bounded +0 record of the choice slot only (open-ended raw \
                 string, no talent-list validation): the selected talent's own effect is \
                 not computed — no talent-effect engine exists in this codebase — and the \
                 4th/6th/8th/10th additional talents stay named-but-unproven as future \
                 numbered slots"
            ),
        });
    }

    // SD13-E5: the SECOND talent, the level-4 repeat grant, recognized as
    // its own numbered choice slot per the proven monk-second-bonus-feat
    // idiom — same open-ended recognition, same non-fabrication posture.
    if level >= ROGUE_SECOND_TALENT_GRANT_LEVEL
        && let Some(second_talent) = choice_selection(input, ROGUE_SECOND_TALENT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_chassis.rogue.talent_2_choice".to_owned(),
            value: 0,
            detail: format!(
                "Rogue SECOND talent selection ({ROGUE_SECOND_TALENT_CHOICE_ID} -> \
                 {second_talent}): PF1 Core Rulebook, verified identically on both primary \
                 sources — \"She gains an additional rogue talent for every 2 levels of \
                 rogue attained after 2nd level. A rogue cannot select an individual talent \
                 more than once.\" — the first additional talent lands at rogue level \
                 {ROGUE_SECOND_TALENT_GRANT_LEVEL}. The level-{level} second talent chosen \
                 for this character is {second_talent}, recognized as a bounded +0 record of \
                 the numbered choice slot only (open-ended raw string, no talent-list \
                 validation): the selected talent's own effect is not computed — no \
                 talent-effect engine exists in this codebase — and the 6th/8th/10th \
                 additional talents stay named-but-unproven as further numbered slots"
            ),
        });
    }

    // SD13-E5: talents 3-5, the remaining numbered slots of the rogue's
    // level-10 talent family (gates 6/8/10 per the same verified rule
    // text), each the same open-ended +0 recognition. With these the full
    // five-slot count at the tranche ceiling is recognized; the talent
    // tree's effects stay the named new-subsystem burden.
    // SD18 widening (cycle-2026-07-15T0800, tests/sd18_rogue_level12_widening.rs):
    // slot 6, gated to rogue level 12 (the level-12 "Special" column's
    // "Rogue talent" entry, verified independently against both primary
    // sources), the same open-ended +0 recognition idiom — no talent-list
    // validation, no talent-effect engine.
    // SD18 widening (cycle-2026-07-15T2000, tests/sd18_rogue_level14_widening.rs):
    // slot 7, gated to rogue level 14 (the level-14 "Special" column's
    // "Rogue talent" entry, verified independently against both primary
    // sources), the same open-ended +0 recognition idiom — no talent-list
    // validation, no talent-effect engine.
    // SD18 widening (cycle-2026-07-15T5200, tests/sd18_rogue_level16_widening.rs):
    // slot 8, gated to rogue level 16 (the level-16 "Special" column's
    // "Rogue talent" entry, verified independently against both primary
    // sources), the same open-ended +0 recognition idiom — no talent-list
    // validation, no talent-effect engine.
    // SD18 widening (cycle-2026-07-16T0212, tests/sd18_rogue_level18_widening.rs):
    // slot 9, gated to rogue level 18 (the level-18 "Special" column's
    // "Rogue talent, trap sense +6" entry, verified independently against
    // both primary sources), the same open-ended +0 recognition idiom — no
    // talent-list validation, no talent-effect engine.
    // SD18 widening (cycle-2026-07-16T1431, tests/sd18_rogue_level20_widening.rs):
    // slot 10, gated to rogue level 20 (the level-20 "Special" column's
    // "Master strike, rogue talent" entry, verified independently against
    // both primary sources), the same open-ended +0 recognition idiom — no
    // talent-list validation, no talent-effect engine. This is the FINAL
    // numbered talent slot within PF1's 1-20 character-level cap.
    let additional_talent_slots: [(u8, u8, &str); 8] = [
        (3, 6, "choice:rogue_talent_3"),
        (4, 8, "choice:rogue_talent_4"),
        (5, 10, "choice:rogue_talent_5"),
        (6, 12, "choice:rogue_talent_6"),
        (7, 14, "choice:rogue_talent_7"),
        (8, 16, "choice:rogue_talent_8"),
        (9, 18, "choice:rogue_talent_9"),
        (10, 20, "choice:rogue_talent_10"),
    ];
    for (slot_number, grant_level, choice_id) in additional_talent_slots {
        if level < grant_level {
            continue;
        }
        let Some(talent) = choice_selection(input, choice_id) else {
            continue;
        };
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.rogue.talent_{slot_number}_choice"),
            value: 0,
            detail: format!(
                "Rogue talent slot {slot_number} selection ({choice_id} -> {talent}): PF1 \
                 Core Rulebook, verified identically on both primary sources — \"She gains \
                 an additional rogue talent for every 2 levels of rogue attained after 2nd \
                 level. A rogue cannot select an individual talent more than once.\" — \
                 this slot's grant lands at rogue level {grant_level}. The level-{level} \
                 selection for this slot is {talent}, recognized as a bounded +0 record of \
                 the numbered choice slot only (open-ended raw string, no talent-list \
                 validation): the selected talent's own effect is not computed — no \
                 talent-effect engine exists in this codebase"
            ),
        });
    }

    // Resiliency (task #58, v0.6 alpha swarm): the one tokened talent this
    // codebase actually computes a magnitude for, recognized across EVERY
    // numbered slot above (not just the first) -- a rogue could take it at
    // any of her ten possible talent picks. `cr_abilities_class.lst`'s own
    // `KEY:Rogue Talent ~ Resiliency` record: `BONUS:VAR|ResiliencyHitPoints|
    // RogueTalentLVL`, and `RogueTalentLVL` resolves to the rogue's own
    // class level with no `PRE` gate at all. Each slot's own grant_level is
    // honored (a selection sitting in a not-yet-unlocked slot does not
    // ground), mirroring the open-ended recognition's own per-slot gating
    // immediately above rather than trusting the selection independent of
    // level.
    let resiliency_slot = [
        (ROGUE_TALENT_GRANT_LEVEL, ROGUE_TALENT_CHOICE_ID),
        (ROGUE_SECOND_TALENT_GRANT_LEVEL, ROGUE_SECOND_TALENT_CHOICE_ID),
    ]
    .into_iter()
    .chain(additional_talent_slots.iter().map(|(_, grant_level, choice_id)| (*grant_level, *choice_id)))
    .find(|(grant_level, choice_id)| {
        level >= *grant_level && choice_selection(input, choice_id) == Some(RESILIENCY_TALENT_SELECTION)
    });

    if resiliency_slot.is_some() {
        let temp_hp = crate::rules_core::durability::rogue_resiliency_temp_hp(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.resiliency_temp_hp".to_owned(),
            value: temp_hp,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   , `BONUS:VAR|ResiliencyHitPoints|RogueTalentLVL`, no PRE gate
                "Rogue level {level} selected the Resiliency talent: once per day, when brought \
                 below 0 hit points, she can gain {temp_hp} temporary hit points (equal to her rogue \
                 level) as an immediate action, lasting 1 minute. This grounds the magnitude only: \
                 the once-per-day budget and the \"brought below 0 hit points\" trigger are named \
                 but not enforced -- no once-per-day activation tracker and no HP-threshold trigger \
                 engine exists anywhere in this codebase (the same honest-gap idiom already used for \
                 Skald's Raging Song rounds-per-day). No temporary-hit-point total exists anywhere \
                 in this codebase or its downstream apps/desktop/src-tauri character_hub.rs consumer \
                 (which tracks only max_hp/current_hp), so this grounds as a standalone flat \
                 magnitude rather than a false integration claim, the same honest-gap idiom already \
                 used for Witch's Ward hex"
            ),
        });
    }

    // Master Strike: below the level-20 gate, this stays a correct PF1
    // Core Rulebook level-gate absence (value 0); at or above it (SD18
    // level-20 widening, the class capstone), it transitions to a bounded
    // GRANT-only identity record (mirroring the Paladin Holy Champion /
    // Ranger Master Hunter idiom exactly). No action-economy engine, no
    // attack-resolution engine, and no saving-throw-resolution engine
    // exists anywhere in this codebase to apply this to.
    if level < ROGUE_MASTER_STRIKE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.master_strike".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Master Strike at rogue level {level}: correctly absent at level {level} \
                 by PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Master Strike is the 20th-level rogue capstone."
            ),
        });
    } else if let Some(dc) = resolve_class_feature_bonus_var(
        "Rogue ~ Master Strike",
        "RogueLVL",
        "MasterStrikeDC",
        level,
        ability_modifiers,
    ) {
        // SD-32 Epic 1 (compute-library wiring, F3): the corpus's own
        // `BONUS:VAR|MasterStrikeDC|10+(MasterStrikeLVL/2)+INT`
        // (`cr_abilities_class.lst:1619`), resolved through
        // `resolve_class_feature_bonus_var` -- no longer a fabricated 0.
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.master_strike".to_owned(),
            value: dc,
            detail: format!(
                "Rogue Master Strike granted at rogue level {level} (PF1 Core Rulebook, \
                 20th-level rogue capstone): \"Upon reaching 20th level, a rogue becomes \
                 incredibly deadly when dealing sneak attack damage. Each time the rogue deals \
                 sneak attack damage, she can choose one of the following three effects: the \
                 target can be put to sleep for 1d4 hours, paralyzed for 2d6 rounds, or slain. \
                 Regardless of the effect chosen, the target receives a Fortitude save to \
                 negate the additional effect. The DC of this save is {dc} (10 + 1/2 the \
                 rogue's level + the rogue's Intelligence modifier). Once a creature has been \
                 the target of a master strike, regardless of whether or not the save is made, \
                 that creature is immune to that rogue's master strike for 24 hours.\" The save \
                 DC is a genuinely computed magnitude (corpus formula, resolved against this \
                 character's real Intelligence modifier); no action-economy engine and no \
                 attack-resolution engine exists anywhere in this codebase to apply this to, so \
                 this still grounds no actual sleep, paralysis, or death effect -- only the \
                 save DC itself is computed"
            ),
        });
    } else {
        // The interpreter's own formula chain did not resolve (e.g. the
        // corpus record's token shape changed underneath this call site) --
        // refuse rather than guess, falling back to the same bounded
        // grant-only identity record the pre-wiring behaviour always used.
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.master_strike".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Master Strike granted at rogue level {level} (PF1 Core Rulebook, \
                 20th-level rogue capstone). This is a bounded grant-only identity record \
                 (value 0, non-fabricated): the corpus formula for the save DC did not resolve, \
                 so no save-DC computation is claimed"
            ),
        });
    }
}

