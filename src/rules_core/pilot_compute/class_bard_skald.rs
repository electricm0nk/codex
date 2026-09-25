#[allow(unused_imports)]
pub(crate) use super::*;

// SD13-E4-F7/SD13-E5 spell-bearing baseline identity. Bard is a spontaneous arcane
// caster with a distinct chassis-class-feature burden (Bardic Knowledge and Bardic
// Music); this slice recognizes its bounded single-class level-1/level-2 identity as
// direct runtime evidence and grounds no performance-state engine (no
// start/maintain action economy, no round tracking or consumption), no Countersong,
// Distraction, or Versatile Performance execution, and no spell math (spells known,
// spells per day, spell DCs, bonus spells, school choice, or prepared posture) for
// it.
pub(super) const BARD_CLASS_ID: &str = "class:bard";

/// v0.6 alpha swarm, risks item 8 (first APG/ACG closure): ACG Skald, a
/// Bard+Barbarian hybrid whose Raging Song is explicitly "the bard's
/// bardic performance special ability for any effect that affects bardic
/// performances" (verified against the PCGen corpus
/// `acg_abilities_class.lst`), and whose 1st-level song, Inspired Rage,
/// grants the same four-value shape as Barbarian's own Rage (STR/CON
/// morale bonus, Will-save morale bonus, AC penalty).
pub(super) const SKALD_CLASS_ID: &str = "class:skald";

/// PF1 Advanced Class Guide Damage Reduction (`KEY:Skald ~ Damage
/// Reduction`, deepening 2026-07-26, task #7): "At 9th level, a skald
/// gains damage reduction... At 14th and 19th level this damage
/// reduction rises by 1 point" -- verified directly against the raw
/// corpus DESC text, the same real, standalone, level-gated flat-
/// magnitude shape Barbarian's own `class_feature.barbarian.
/// damage_reduction` already established (see
/// `BARBARIAN_DAMAGE_REDUCTION_LEVEL`'s own doc comment): grounded as a
/// standalone explanation record, never applied to any incoming-damage
/// total, since no damage-resolution engine or incoming-damage total
/// exists anywhere in this codebase. Unlike Barbarian's own DR, Skald's
/// version also extends to all allies affected by his Raging Song ("The
/// skald grants this DR to all allies affected by his raging song") --
/// that ally-extension stays explicitly deferred, since this engine
/// models neither allies nor any ally-targeting mechanism at all; only
/// the skald's own self-DR is grounded here.
pub(super) const SKALD_DAMAGE_REDUCTION_LEVEL: u8 = 9;

pub(super) const SKALD_DAMAGE_REDUCTION_TWO_LEVEL: u8 = 14;

pub(super) const SKALD_DAMAGE_REDUCTION_THREE_LEVEL: u8 = 19;

/// `ClassAbilityActivation.ability_id` for Skald Inspired Rage.
pub(super) const SKALD_INSPIRED_RAGE_ABILITY_ID: &str = "inspired_rage";

/// PF1 Advanced Class Guide Inspired Rage: a flat -1 penalty to AC,
/// unconditional at every tier (verified against the PCGen corpus DESC
/// text: "but also take a -1 penalty to AC" -- not tiered, unlike the
/// STR/CON/Will values).
pub(super) const SKALD_INSPIRED_RAGE_ARMOR_CLASS_PENALTY: i16 = -1;

/// PF1 Advanced Class Guide Raging Song rounds-per-day: 3 + Charisma
/// modifier at 1st level, +2 additional rounds per level thereafter
/// (verified against the PCGen corpus DESC text, the same shape as Bard's
/// own Bardic Performance formula with a different base -- 3, not 4).
pub(super) const SKALD_RAGING_SONG_BASE_ROUNDS_PER_DAY: i16 = 3;

/// PF1 Advanced Class Guide Well-Versed (`KEY:Skald ~ Well-Versed`, task
/// #50): granted at 2nd level -- confirmed directly against the real
/// PCGen corpus's own per-level grant row (`2\tABILITY:Skald Class
/// Feature|AUTOMATIC|Skald ~ Well-Versed|...` in `acg_classes.lst`, the
/// `\t` a real tab column separator, not a doc-formatting artifact),
/// since the feature's own `BONUS:VAR|SkaldWellVersedBonus|4` token
/// carries no level term to self-gate on (unlike Spell Kenning/Lore
/// Master/Versatile Performance/Rage Powers below, whose formulas floor
/// to 0 below their own real grant level). A flat +4 bonus on saving
/// throws against bardic performance, sonic, and language-dependent
/// effects, not level-scaled -- byte-identical to Bard's own already-
/// shipped `BARD_WELL_VERSED_BONUS` (Bard's own grant level, 2, happens
/// to match Skald's).
pub(super) const SKALD_WELL_VERSED_LEVEL: u8 = 2;

pub(super) const SKALD_WELL_VERSED_BONUS: i16 = 4;

/// PF1 Advanced Class Guide Spell Kenning (`KEY:Skald ~ Spell Kenning`,
/// task #50): granted at 5th level, confirmed against the real corpus's
/// own per-level grant row. `BONUS:VAR|SkaldSpellKenningUsesPerDay|
/// (1+SkaldLVL)/6` self-gates to 0 below level 5 by its own floor
/// division, matching the real grant level exactly.
pub(super) const SKALD_SPELL_KENNING_LEVEL: u8 = 5;

/// PF1 Advanced Class Guide Lore Master (`KEY:Skald ~ Lore Master`, task
/// #50): granted at 7th level, confirmed against the real corpus's own
/// per-level grant row. `BONUS:VAR|SkaldLoreMasterUsesPerDay|
/// min((SkaldLVL-1)/6,3)` self-gates to 0 below level 7 by its own floor
/// division; the cap (3) is the corpus token's own second `min()`
/// operand, reached at level 19.
pub(super) const SKALD_LORE_MASTER_LEVEL: u8 = 7;

pub(super) const SKALD_LORE_MASTER_MAX_USES_PER_DAY: i16 = 3;

/// PF1 Advanced Class Guide Versatile Performance (`KEY:Skald ~
/// Versatile Performance`, task #50): granted at 2nd level, confirmed
/// against the real corpus's own per-level grant row.
/// `BONUS:ABILITYPOOL|Skald Versatile Performance|min((SkaldLVL+3)/5)`
/// is a genuinely single-argument `min()` in the raw corpus (confirmed
/// directly, not a transcription defect) -- min of one argument is just
/// that argument, so this grounds `(SkaldLVL+3)/5` as its own count,
/// self-gating to 0 below level 2 by its own floor division.
pub(super) const SKALD_VERSATILE_PERFORMANCE_LEVEL: u8 = 2;

/// PF1 Advanced Class Guide Rage Powers (`KEY:Skald ~ Rage Powers`, task
/// #50): Skald's own pool-SIZE only (distinct from selecting/executing
/// any individual rage power, out of scope here -- that is a separate
/// build). Granted at 3rd level, confirmed against the real corpus's own
/// per-level grant row. `BONUS:VAR|RagePowersLVL|SkaldLVL` sets the
/// shared `RagePowersLVL` variable unconditionally from `SkaldLVL` (no
/// borrowed-variable transcription gap the way Swashbuckler's deed gate
/// had), and `BONUS:ABILITYPOOL|Rage Power|RagePowersLVL/3` self-gates
/// to 0 below level 3 by its own floor division.
pub(super) const SKALD_RAGE_POWERS_LEVEL: u8 = 3;

/// The bard level at which 2nd-level bard spells first become available,
/// verified against the raw PF1 Core Rulebook Bard spells-per-day table rows
/// (d20pfsrd and legacy.aonprd.com, identical): level 3 shows "3/—/…",
/// level 4 shows "3/1/—/…" — the first non-"—" 2nd-level column. Unlike the
/// Paladin table, the Bard table has NO "0" spells-per-day entries at levels
/// 1-10; every non-"—" entry is a positive count. 1st-level bard spells are
/// available from level 1 ("1/—/…"), so the ladder has no zero step and no
/// 1st-level threshold const is needed.
pub(super) const BARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 4;

/// The bard level at which 3rd-level bard spells first become available,
/// verified against the raw table rows (both sources): level 6 shows
/// "4/3/—/…", level 7 shows "4/3/1/—/…" — the first non-"—" 3rd-level column.
pub(super) const BARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 7;

/// The bard level at which 4th-level bard spells first become available,
/// verified against the raw table rows (both sources): level 9 shows
/// "5/4/3/—/…", level 10 shows "5/4/3/1/—/—" — the first non-"—" 4th-level
/// column. The 5th-level column stays "—" through level 10 (5th-level bard
/// spells begin at 13, outside the tranche ceiling), so no 5th-level
/// threshold const is grounded.
pub(super) const BARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 10;

/// PF1 Core Rulebook Versatile Performance slots, verified identically on
/// both primary sources: "At 2nd level, a bard can choose one type of
/// Perform skill... At 6th level, and every 4 levels thereafter, the bard
/// can select an additional type of Perform to substitute." — gates 2/6/10
/// within the tranche ceiling. Numbered slots per the proven repeat-grant
/// idiom; restricted-list recognition over the nine verified Perform types
/// with their fixed associated-skill pairs.
pub(super) const BARD_VERSATILE_PERFORMANCE_SLOTS: [(u8, u8, &str); 3] = [
    (1, 2, "choice:bard_versatile_performance"),
    (2, 6, "choice:bard_versatile_performance_2"),
    (3, 10, "choice:bard_versatile_performance_3"),
];

/// The nine verified Perform types as (selection, display name, associated
/// skill pair) — identical on d20pfsrd and legacy.aonprd.com.
pub(super) const BARD_VERSATILE_PERFORMANCE_TYPES: [(&str, &str, &str); 9] = [
    ("perform:act", "Act", "Bluff and Disguise"),
    ("perform:comedy", "Comedy", "Bluff and Intimidate"),
    ("perform:dance", "Dance", "Acrobatics and Fly"),
    (
        "perform:keyboard_instruments",
        "Keyboard Instruments",
        "Diplomacy and Intimidate",
    ),
    ("perform:oratory", "Oratory", "Diplomacy and Sense Motive"),
    ("perform:percussion", "Percussion", "Handle Animal and Intimidate"),
    ("perform:sing", "Sing", "Bluff and Sense Motive"),
    ("perform:string", "String", "Bluff and Diplomacy"),
    ("perform:wind", "Wind", "Diplomacy and Handle Animal"),
];

/// SD13-E5 Bard level-range gate, mirroring the Fighter `supported_fighter_level` /
/// Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` idiom. Verified against the PF1 Core Rulebook Bard class
/// table (d20pfsrd and legacy.aonprd.com) before widening: level 2 base attack +1,
/// base saves +0/+3/+3 (Fortitude/Reflex/Will), Bardic Performance rounds per day
/// gains 2 additional rounds after 1st level, Inspire Courage's flat magnitude does
/// not increase until level 5, and the level-2 "Special" column reads "Versatile
/// performance, well-versed" — Well-Versed (a flat, non-level-scaled +4 save bonus)
/// is grounded this slice; Versatile Performance (a choice-gated skill-substitution
/// engine) is deliberately left named-but-unproven. Widened again to level 3 by a
/// further SD13-E5 slice: level 3 base attack +2, base saves +1/+3/+3
/// (Fortitude/Reflex/Will), every other flat formula (Bardic Knowledge, Bardic
/// Performance rounds/day, Inspire Courage, Fascinate DC/affected-creature-count,
/// Well-Versed) extends via the same formula with no re-derivation, and the
/// level-3 "Special" column reads "Inspire competence +2" — a flat, identity-shaped
/// class feature grounded this slice. Unlike Wizard's specialist-bonus-slot or
/// Cleric's domain-slot doubling at level 3, Bard has no grounded spell-slot-count
/// pillar at all (the Bard spells-per-day table's own 2nd-level spell column does
/// not turn non-blank until 4th level, verified independently, and this row's
/// still-unproven list already names the entire spontaneous spell posture as
/// ungrounded), so no analogous slot-count doubling applies here. A later SD13-E5
/// slice widens this gate to level 4, verified independently against the PF1 Core
/// Rulebook Bard class table (d20pfsrd and legacy.aonprd.com): the level-4
/// "Special" column is BLANK (no new class feature is gained at 4th level; the
/// next new feature, Lore Master, comes at 5th level), so that widening extends
/// every already-grounded formula (base attack, base saves, Bardic Knowledge,
/// Bardic Performance rounds/day, Fascinate DC/count) and keeps Well-Versed and
/// Inspire Competence granted, without introducing any new pillar. A further
/// SD13-E5 slice widens this gate to level 5, re-verified independently
/// against both primary sources rather than trusting an earlier cycle's "stays
/// +1 through level 5" note at face value: the level-5 "Special" column reads
/// "Inspire courage +2, lore master 1/day", so the Inspire Courage flat
/// magnitude genuinely increases to +2 exactly at level 5 (the earlier note
/// turns out to have been precise, not imprecise — level 4 stays +1, and level
/// 5 is exactly the level the rule's own "at 5th level... increases by +1"
/// text describes), and Lore Master is newly grounded as a bounded grant-only
/// flat 1/day usage-count record for its take-20 half only (the take-10 half
/// has no flat magnitude to ground and neither mechanic is executed against
/// any actual Knowledge check). A still further SD13-E5 slice widens this
/// gate to level 6, verified independently against the PF1 Core Rulebook
/// Bard class table (d20pfsrd and the legacy.aonprd.com mirror): the
/// level-6 row is BAB +4, Fort +2, Ref +5, Will +5 — all extended via the
/// same pre-existing formulas, no re-derivation. Bardic Knowledge genuinely
/// rises to 3 (`max(6/2, 1)`), Bardic Performance rounds per day continues
/// scaling, and the Fascinate DC genuinely rises to 15 (`10 + 6/2 + CHA`);
/// the Fascinate affected-creature count stays 2 (an integer-division
/// coincidence with level 5), and Inspire Courage stays +2 (the next
/// increase does not land until bard level 11). The level-6 "Special"
/// column reads "Suggestion, Versatile performance" (verified independently
/// against both primary sources). Both entries were checked and confirmed
/// NOT flat: Suggestion is a spell-like ability requiring a
/// fascinated-target prerequisite and the "suggestion" spell's own
/// effect-resolution engine (neither exists in this codebase), and the
/// 6th-level Versatile Performance grant is merely an additional instance
/// of the same choice-gated skill-substitution engine already deliberately
/// left named-but-unproven at 2nd level, not a new type of class feature —
/// so no new pillar record is grounded at level 6. A still further SD13-E5
/// slice widens this gate to level 7, verified independently against the
/// PF1 Core Rulebook Bard class table (d20pfsrd and the legacy.aonprd.com
/// mirror): the level-7 row is BAB +5, Fort +2, Ref +5, Will +5 — base
/// attack genuinely rises to 5 (`7 * 3 / 4`) while all three base saves
/// stay numerically unchanged from level 6 (an integer-division
/// coincidence, re-verified against the raw table row rather than
/// assumed). Bardic Knowledge stays 3 (`max(7/2, 1)`, unchanged), Bardic
/// Performance rounds per day continues scaling, the Fascinate DC stays 15
/// (`10 + 7/2 + CHA`, an integer-division coincidence with level 6), and
/// the Fascinate affected-creature count genuinely rises to 3
/// (`1 + (7-1)/3`), up from 2 at level 6. The level-7 "Special" column
/// reads "Inspire competence +3" (verified independently against both
/// primary sources): the Inspire Competence rule text itself confirms this
/// is a flat magnitude increase on an already-grounded pillar ("This bonus
/// increases by +1 for every four levels the bard has attained beyond 3rd
/// (+3 at 7th, +4 at 11th, +5 at 15th, and +6 at 19th)"), the same kind of
/// arithmetic tier-widening as Inspire Courage's own second tier at level
/// 5, not a new class feature — grounded as a genuine rise to +3.
/// A still further SD13-E5 slice widens this gate to level 8, verified
/// independently against the PF1 Core Rulebook Bard class table (d20pfsrd
/// and the legacy.aonprd.com mirror): the level-8 row is BAB +6/+1, Fort +2,
/// Ref +6, Will +6 -- base attack genuinely rises to 6 (`8 * 3 / 4`, the
/// class table's own iterative-attack notation "+6/+1" not modeled anywhere
/// in this codebase, only the flat base value, mirroring the Cleric
/// level-8 precedent), base Fortitude stays 2 (`8/3`, an integer-division
/// coincidence with level 7, re-verified rather than assumed), and base
/// Reflex/Will both genuinely rise to 6 (`8/2+2`), up from 5 at level 7.
/// Bardic Knowledge genuinely rises to 4 (`max(8/2, 1)`), up from 3 at
/// level 7. Bardic Performance rounds per day continues scaling. The
/// Fascinate DC genuinely rises to 16 (`10 + 8/2 + CHA`), up from 15 at
/// level 7, while the Fascinate affected-creature count stays 3
/// (`1 + (8-1)/3 = 1 + 7/3 = 3`), an integer-division coincidence with
/// level 7, confirmed by direct arithmetic against the primary source rule
/// text rather than trusted from the formula alone. Inspire Courage stays
/// +2 and Inspire Competence stays +3 (neither's next tier lands until
/// bard level 11). The level-8 "Special" column reads "Dirge of doom"
/// (verified independently against both primary sources): a genuinely NEW
/// bardic-performance type, checked and confirmed NOT flat/identity-shaped
/// -- it requires both the same performance-state engine already left
/// ungrounded (start/maintain action economy, round tracking/consumption)
/// and a fear/shaken-condition resolution engine, neither of which exists
/// in this codebase, so it is deliberately left named-but-unproven,
/// mirroring the Suggestion / Countersong / Distraction precedent exactly
/// -- no explanation record is fabricated for it.
///
/// A further SD13-E5 slice widens the gate to level 9 (verified independently
/// against d20pfsrd and legacy.aonprd.com): level 9 base attack stays +6
/// (9 * 3 / 4) and good Reflex/Will both stay +6 (9 / 2 + 2),
/// integer-division coincidences, while poor Fortitude genuinely rises to +3
/// (9 / 3); the Bardic Performance rounds-per-day pool genuinely rises to 22
/// (4 + Cha mod + 2 per level after 1st); Bardic Knowledge, the Fascinate
/// DC/count, Inspire Courage, Inspire Competence, Well-Versed, and Lore
/// Master all carry over unchanged (the next Fascinate-count rise lands at
/// 10th and the next Inspire/Lore tiers at 11th, checked rather than
/// assumed); the level-9 "Special" column reads "Inspire greatness" -- a
/// genuinely NEW bardic-performance type checked and confirmed NOT flat (it
/// grants 2 bonus Hit Dice with commensurate temporary hit points, a +2
/// competence attack bonus, and a +1 competence Fortitude bonus to a willing
/// ally, requiring the performance-state engine plus
/// temporary-Hit-Dice/temporary-hit-point mechanics, none of which exist in
/// this codebase), so it is deliberately left named-but-unproven, mirroring
/// the Suggestion / Countersong / Distraction / Dirge-of-Doom precedent
/// exactly -- no explanation record is fabricated for it.
///
/// A further SD13-E5 slice widens the gate to level 10 — the tranche ceiling
/// (verified independently against d20pfsrd and legacy.aonprd.com): level 10
/// base attack genuinely rises to +7 (10 * 3 / 4) and good Reflex/Will both
/// genuinely rise to +7 (10 / 2 + 2), while poor Fortitude stays +3 (10 / 3,
/// a coincidence); the performance rounds pool genuinely rises to 24, Bardic
/// Knowledge genuinely rises to 5 (max(10/2, 1)), the Fascinate DC genuinely
/// rises to 17 and its affected-creature count genuinely rises to 4
/// (1 + (10-1)/3); Inspire Courage/Competence and Lore Master stay at their
/// tiers (next rises at 11th); the level-10 "Special" column reads
/// "Jack-of-all-trades, versatile performance": Jack-of-All-Trades' 10th-level
/// piece (use any skill untrained) is grounded as a +0 identity/recognition
/// record (BARD_JACK_OF_ALL_TRADES_LEVEL), mirroring the Woodland Stride /
/// Purity of Body idiom, while the repeat Versatile Performance grant stays
/// named-but-unproven exactly like the 2nd/6th-level grants before it.
///
/// A still further SD18 slice widens the gate to level 11 (verified
/// independently against d20pfsrd and legacy.aonprd.com, mirroring exactly
/// how `MAX_SUPPORTED_BARBARIAN_LEVEL` was widened from 10 to 11): base
/// attack (classlevel * 3 / 4) genuinely rises to +8, base saves stay
/// Fortitude +3 / Reflex +7 / Will +7 (11/3 and 11/2+2, both
/// integer-division coincidences unchanged from level 10), Bardic Knowledge
/// stays 5 and the Fascinate DC/count stay 17/4 (all integer-division
/// coincidences), the Bardic Performance rounds-per-day pool genuinely
/// rises to 26 (4 + Charisma modifier + 2 per level after 1st), and the
/// level-11 "Special" column reads "Inspire competence +4, inspire courage
/// +3, lore master 2/day" only — three magnitude-rises on the
/// already-grounded flat-constant pillars (Inspire Competence 2 +
/// (level-3)/4, Inspire Courage's every-sixth-level-after-5th tier, and
/// Lore Master's own every-sixth-level-after-5th take-20 usage-count tier),
/// mirroring exactly how the Barbarian Greater Rage magnitude-rise was
/// grounded. No new class feature (no new choice slot) is granted at 11th
/// level, so no new engine is invented; Jack-of-All-Trades and the repeat
/// Versatile Performance grant both carry over unchanged.
///
/// A still further SD18 slice widens the gate to level 12 (verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror, mirroring exactly how `MAX_SUPPORTED_BARBARIAN_LEVEL` was
/// widened from 11 to 12): base attack (classlevel * 3 / 4) genuinely
/// rises to +9, base saves genuinely rise to Fortitude +4 (12/3) / Reflex
/// +8 / Will +8 (both 12/2+2), Bardic Knowledge genuinely rises to 6
/// (max(12/2, 1)), the Bardic Performance rounds-per-day pool genuinely
/// rises to 28 (4 + Charisma modifier + 2 per level after 1st), and the
/// Fascinate DC genuinely rises to 18 (10 + 12/2 + Charisma modifier)
/// while the Fascinate affected-creature count stays 4 (1 + (12-1)/3, an
/// integer-division coincidence with level 11). The level-12 "Special"
/// column reads "Soothing performance" only — a wholly new 12th-level
/// class feature, grounded ONLY as a bounded grant-only identity record
/// (`BARD_SOOTHING_PERFORMANCE_LEVEL`), mirroring the Monk Diamond Body /
/// Paladin Aura of Justice idiom exactly: no healing-application engine
/// and no condition-removal engine exist anywhere in this codebase, so
/// neither is fabricated. Inspire Courage, Inspire Competence, and Lore
/// Master's flat magnitudes all stay unchanged at their level-11 third
/// tier (their next tiers land at bard level 15 or 17, out of scope);
/// Jack-of-All-Trades and the repeat Versatile Performance grant both
/// carry over unchanged.
///
/// SD18 (cycle-2026-07-15T1700) widens the gate again to level 13, the
/// loop's seventh §3.2 level-13 landing (after Rogue, Barbarian, Fighter,
/// Ranger, Cleric, and Druid) and the first on a spontaneous
/// (non-9-level) caster. All three primary sources (d20pfsrd, the
/// Archives of Nethys aonprd.com mirror, and legacy.aonprd.com) agree
/// byte-for-byte that the level-13 "Special" column is BLANK: base
/// attack bonus and all three base saves stay numerically unchanged from
/// level 12 (13*3/4=9, 13/3=4, 13/2+2=8, all integer-division
/// coincidences), Bardic Knowledge stays 6 (max(13/2,1), also a
/// coincidence), and the Fascinate DC stays 18 (10+13/2+CHA, since
/// 13/2==12/2==6). The Bardic Performance rounds-per-day pool and the
/// Fascinate affected-creature count both genuinely rise via their
/// already-generic level-valued formulas. No new named class feature is
/// granted, so this is a pure arithmetic-pillar widening: no new record
/// is added, and the spontaneous spell-level-access ladder / base
/// spells-per-day / spells-known table lookups stay at their
/// pre-existing level-10 ceiling exactly as left by the level-11 and
/// level-12 cycles (no 5th-level spell-access threshold is grounded).
///
/// SD18 (cycle-2026-07-15T2200) widens the gate again to level 14, the
/// loop's FIFTH §3.2 level-14 landing (after Barbarian, Fighter, Rogue,
/// and Ranger). Both primary sources (d20pfsrd and the Archives of Nethys
/// aonprd.com mirror) agree byte-for-byte: base attack bonus genuinely
/// rises to +10 (14*3/4) and both good saves genuinely rise to +9
/// (14/2+2, Reflex and Will) while poor Fortitude stays +4 (14/3, an
/// integer-division coincidence with level 13); Bardic Knowledge
/// genuinely rises to 7 (max(14/2,1)); the Bardic Performance
/// rounds-per-day pool genuinely rises to 32 (4+CHA+2*(14-1)); the
/// Fascinate DC genuinely rises to 19 (10+14/2+CHA) while the Fascinate
/// affected-creature count stays 5 (1+(14-1)/3, an integer-division
/// coincidence with level 13). The level-14 "Special" column reads
/// "Frightening tune, Versatile performance": Frightening Tune is a
/// wholly new 14th-level class feature whose rule text gives the exact
/// same Will-save DC formula shape as the already-grounded Fascinate DC
/// (10 + 1/2 bard level + Charisma modifier), so it is grounded ONLY as a
/// flat standalone DC magnitude (`BARD_FRIGHTENING_TUNE_LEVEL`), mirroring
/// the Fascinate DC idiom; unlike Fascinate, its affected scope is
/// range-based ("each enemy within 30 feet who can hear the
/// performance"), not a numeric-count formula, so no affected-creature
/// count record is added for it. The repeat Versatile Performance grant
/// (also at levels 2, 6, and 10) stays named-but-unproven unchanged.
///
/// SD18 (cycle-2026-07-15T4500) widens the gate again to level 15, the loop's
/// TENTH §3.2 level-15 landing (after Barbarian, Rogue, Fighter, Cleric, Druid,
/// Ranger, Wizard, Paladin, and Sorcerer) and the FINAL class needed to close
/// the §3.2 level-15 sweep at 10 of 10 non-Monk classes. Both primary sources
/// (d20pfsrd and the Archives of Nethys aonprd.com mirror) agree byte-for-byte:
/// base attack bonus genuinely rises to +11 (15*3/4) and poor Fortitude
/// genuinely rises to +5 (15/3), while both good saves (Reflex, Will) stay +9
/// (15/2+2, an integer-division coincidence with level 14); Bardic Knowledge
/// stays 7 (max(15/2,1), a coincidence); the Bardic Performance rounds-per-day
/// pool genuinely rises to 34 (4+CHA+2*(15-1)); the Fascinate DC and
/// affected-creature count both stay unchanged (19, 5 — both integer-division
/// coincidences with level 14); Frightening Tune's DC (the same formula shape
/// as the Fascinate DC) likewise stays 19 for the same reason. The level-15
/// "Special" column reads "Inspire competence +5, inspire heroics"
/// (resolving the level-13 cycle's own open question about whether the
/// Inspire Courage/Lore Master tier thresholds land at level 15 or 17: they
/// do NOT — both stay at their level-11 third tier, since their own next
/// tier is at level 17, verified directly against the rule text "every six
/// bard levels thereafter"). Inspire Competence's flat magnitude genuinely
/// rises from +4 to +5 — a fourth tier on the already-generalized tiered
/// if/else chain, the same arithmetic-widening idiom as the third-tier
/// addition at level 11, needing no new grounding machinery. Inspire
/// Heroics is a wholly new 15th-level class feature ("A bard of 15th level
/// or higher can inspire tremendous heroism in himself or a single ally
/// within 30 feet... Inspired creatures gain a +4 morale bonus on saving
/// throws and a +4 dodge bonus to AC."); both magnitude numbers are flat
/// and non-level-scaled at the level they are gained, so they are grounded
/// as flat standalone magnitudes mirroring the Well-Versed idiom exactly,
/// and the base target count (a single creature at 15th level, before the
/// "+1 creature per three bard levels beyond 15th" scaling, which lands
/// beyond this bounded slice's ceiling) is grounded as a flat count
/// mirroring the Fascinate affected-creature-count idiom. No targeting,
/// save resolution, AC application, or performance-state execution is
/// grounded for Inspire Heroics — it remains named-but-unproven for
/// execution, exactly like Frightening Tune and Soothing Performance
/// before it.
///
/// Widened to level 16 by an SD18 slice (the loop's EIGHTH §3.2 level-16
/// landing): verified independently against THREE primary sources
/// (d20pfsrd, the Archives of Nethys aonprd.com mirror, and
/// legacy.aonprd.com's corerulebook mirror, all three byte-for-byte
/// identical: "+12/+7/+2 | +5 | +10 | +10 | —"). The level-16 "Special"
/// column is genuinely BLANK, resolving a prior cycle's carried-forward
/// risk-map note that had claimed a source disagreement (aonprd.com
/// allegedly reading "Versatile performance" at level 16) — that text in
/// fact belongs to level 14's own already-grounded Special column
/// ("Frightening tune, Versatile performance"), misattributed to level 16
/// by an earlier cycle's transcription. A pure ceiling raise: every
/// formula below is already level-generic, so no new tier constant,
/// record type, or choice slot is added.
///
/// Widened to level 17 by an SD18 slice (cycle-2026-07-15T7100, the loop's
/// SECOND §3.2 level-17 landing, after Ranger): verified independently
/// against TWO primary sources (d20pfsrd and the Archives of Nethys
/// aonprd.com mirror, byte-for-byte identical: "+12/+7/+2 | +5 | +10 | +10
/// | Inspire courage +4, lore master 3/day"), with neighboring levels 16
/// ("—") and 18 ("Mass suggestion, versatile performance") re-fetched in
/// the same pass to rule out misattribution. Base attack bonus (`17*3/4`),
/// both good saves (`17/2+2`), poor Fortitude (`17/3`), Bardic Knowledge
/// (`max(17/2,1)`), the Fascinate DC (`10+17/2+CHA`), and the Fascinate
/// affected-creature count (`1+(17-1)/3`) are all numerically UNCHANGED
/// from level 16 — every one an integer-division coincidence re-verified
/// against the raw class table row rather than assumed — while the Bardic
/// Performance rounds-per-day pool genuinely rises (`4+CHA+2*(17-1)`).
/// Inspire Courage's flat magnitude GENUINELY RISES from +3 to +4 (a
/// fourth tier on the already-generalized tiered if/else chain, the same
/// arithmetic-widening idiom as Inspire Competence's own third/fourth
/// tier additions) and Lore Master's flat take-20 usage-count magnitude
/// GENUINELY RISES from 2/day to 3/day (a third tier on its own
/// already-generalized tiered if/else chain), both the same
/// every-six-bard-levels-after-5th cadence that produced their own
/// level-11 third/second tier respectively. Inspire Competence stays at
/// its level-15 fourth tier (next tier at level 19, out of scope);
/// Inspire Heroics' flat magnitudes and base target count carry over
/// unchanged (the "+1 creature per three bard levels beyond 15th" scaling
/// lands at level 18, out of scope). Only two new tier constant pairs are
/// added (on already-generalized tiered if/else chains); no new record
/// type or choice slot is added, and no Bard level 18+ is proven.
///
/// SD18 (cycle-2026-07-16T0900) widens the gate again to level 18 — the
/// loop's NINTH §3.2 level-18 landing (after Wizard, Cleric, Paladin,
/// Fighter, Barbarian, Rogue, Ranger, and Sorcerer) and the CLOSE of the
/// §3.2 level-18 sweep at 9 of 9 eligible classes (Druid capped at 15,
/// Monk capped at 12, both documented structural exceptions) — verified
/// independently against TWO primary sources fetched fresh this cycle: a
/// raw HTML parse of d20pfsrd.com's own class table (bypassing
/// AI-summarization, following the lesson from the Sorcerer level-18
/// cycle) and the Archives of Nethys aonprd.com mirror via
/// `ClassDisplay.aspx`, both byte-for-byte identical on the level-18 row
/// ("+13/+8/+3 | +6 | +11 | +11 | Mass suggestion, versatile
/// performance"), with neighboring levels 16 ("—"), 17 ("Inspire courage
/// +4, lore master 3/day"), and 19 ("Inspire competence +6") re-fetched in
/// the same pass to rule out misattribution. Base attack bonus (`18*3/4`),
/// both good saves (`18/2+2`), and poor Fortitude (`18/3`) all GENUINELY
/// RISE from level 17; Bardic Knowledge (`max(18/2,1)`) GENUINELY RISES;
/// the Bardic Performance rounds-per-day pool GENUINELY RISES
/// (`4+CHA+2*(18-1)`); the Fascinate DC (`10+18/2+CHA`) GENUINELY RISES
/// while the Fascinate affected-creature count (`1+(18-1)/3`) STAYS
/// unchanged, an integer-division coincidence; Frightening Tune's DC (the
/// same formula shape) likewise GENUINELY RISES. Inspire Courage, Inspire
/// Competence, and Lore Master all stay at their level-17/level-15 tiers
/// (no further tier is defined within this bounded slice's ceiling, or
/// lands at level 19, out of scope). Inspire Heroics' flat save-bonus
/// (+4) and AC-bonus (+4) magnitudes stay unchanged, but its base target
/// count GENUINELY RISES from 1 to 2 — the PF1 Core Rulebook's own text
/// ("for every three bard levels the character attains beyond 15th, he
/// can inspire heroics in one additional creature") places this exactly
/// at bard level 18, a genuine arithmetic-pillar widening on an
/// already-generalized tiered if/else chain, the same idiom as Inspire
/// Courage's/Inspire Competence's/Lore Master's own tier additions.
///
/// The level-18 "Special" column's two named entries were checked and
/// confirmed to require the SAME already-declined machinery as their own
/// precedents, so NEITHER is grounded as a new record. "Mass suggestion"
/// (PF1 Core Rulebook: "This ability functions just like suggestion, but
/// allows a bard of 18th level or higher to make a suggestion
/// simultaneously to any number of creatures that he has already
/// fascinated") is a strict widening of the 6th-level Suggestion
/// spell-like ability, which was already deliberately left
/// named-but-unproven at level 6 (it requires a fascinated-target
/// prerequisite and the "suggestion" spell's own effect-resolution
/// engine, neither of which exists in this codebase) — Mass Suggestion
/// inherits the identical blocker and adds a multi-target dimension on
/// top of it, so it stays named-but-unproven, with no record fabricated
/// for it, exactly mirroring the level-6 Suggestion precedent (this is
/// NOT a new spell-like-ability-casting engine declined for the first
/// time; it is the SAME already-declined engine, re-confirmed).
/// "Versatile performance" is a REPEAT of the Bard's own 2nd-level grant
/// (also seen at levels 6, 10, and 14): already deliberately left
/// named-but-unproven at level 2 (requires a choice-gated
/// skill-substitution engine that does not exist in this codebase), so
/// this cycle adds no new record for its level-18 reappearance either.
/// Only one new tier constant pair is added (on an already-generalized
/// tiered if/else chain, Inspire Heroics' target count); no new record
/// type or choice slot is added, and no Bard level 19+ is proven. A
/// further SD18 slice (`cycle-2026-07-16T1400`, the loop's FOURTH §3.2
/// level-19 landing, after Barbarian, Cleric, and Fighter) widens the gate
/// again to 1..=19 (`MAX_SUPPORTED_BARD_LEVEL = 19`): the class table's
/// level-19 "Special" column reads "Inspire competence +6" (verified
/// independently against two primary sources — a raw HTML parse of
/// d20pfsrd.com's own class table and the Archives of Nethys aonprd.com
/// mirror via `ClassDisplay.aspx`, both covering the full
/// levels-17-through-20 block, byte-for-byte agreement, so a third source
/// was not required) — Inspire Competence's flat magnitude genuinely rises
/// to +6 via a FIFTH tier constant
/// (`BARD_INSPIRE_COMPETENCE_FIFTH_TIER_LEVEL = 19`), mirroring exactly
/// the already-generalized tiered if/else chain idiom used for its own
/// second/third/fourth tiers and for Inspire Courage's/Lore Master's own
/// tier additions; this is the ONLY named feature at level 19, so no other
/// new pillar is grounded from the Special column. Base attack bonus
/// genuinely rises to +14 (`19 * 3 / 4 = 14`); poor Fortitude stays put at
/// +6 (`19 / 3 = 6`) and both good saves stay put at +11 (`19 / 2 + 2 =
/// 11`), integer-division coincidences with level 18, checked not assumed;
/// Bardic Knowledge stays put at 9 (`max(19 / 2, 1) = 9`, also a
/// coincidence); the Bardic Performance rounds-per-day pool genuinely
/// rises (`4 + CHA + 2 * (19 - 1)`); the Fascinate DC stays put at
/// `10 + 19 / 2 + CHA` (a coincidence with level 18) while the Fascinate
/// affected-creature count genuinely rises to `1 + (19 - 1) / 3`;
/// Frightening Tune's DC (the same formula shape) likewise stays put.
/// Inspire Courage stays at its level-17 fourth tier (next tier at level
/// 23, out of scope); Lore Master stays at its level-17 third tier (no
/// further tier defined); Inspire Heroics' flat save-bonus/AC-bonus
/// magnitudes and base target count (set at level 18) all carry over
/// unchanged (the next target-count rise lands at level 21, out of
/// scope). This needed ZERO new record types and ZERO new choice slots —
/// only one new tier constant pair on an already-generalized tiered
/// if/else chain, and no Bard level 20 is proven. A further SD18 slice
/// (alphabetically the first of the six remaining §3.2 level-20
/// candidates after Cleric, Wizard, and Barbarian) widens the gate again
/// to 1..=20 (`MAX_SUPPORTED_BARD_LEVEL = 20`) — the final remaining
/// level within PF1's 1-20 character-level cap for this class row.
/// Verified independently against two primary sources (a raw HTML parse
/// of d20pfsrd.com's own class table, bypassing AI-summarization, and
/// the Archives of Nethys aonprd.com mirror via `ClassDisplay.aspx`,
/// both covering the full levels-17-through-20 block, byte-for-byte
/// agreement, so a third source was not required): the level-20 row
/// reads "+15/+10/+5 | +6 | +12 | +12 | Deadly performance |
/// 5/5/5/5/5/5" (spells per day) and "6/6/6/6/6/5/5" (spells known).
/// Base attack bonus genuinely rises to +15 (`20 * 3 / 4 = 15`); poor
/// Fortitude stays put at +6 (`20 / 3 = 6`, an integer-division
/// coincidence with level 19) while both good saves (Reflex, Will)
/// genuinely rise to +12 (`20 / 2 + 2 = 12`, up from +11); Bardic
/// Knowledge genuinely rises to 10 (`max(20 / 2, 1) = 10`, up from 9);
/// the Bardic Performance rounds-per-day pool genuinely rises to 44
/// (`4 + CHA + 2 * (20 - 1)`, up from 42); the Fascinate DC genuinely
/// rises to 22 (`10 + 20 / 2 + CHA`, up from 21) while the Fascinate
/// affected-creature count stays put at 7 (`1 + (20 - 1) / 3 = 7`, an
/// integer-division coincidence with level 19); Frightening Tune's DC
/// (the same formula shape) likewise genuinely rises to 22. Inspire
/// Courage stays at its level-17 fourth tier (next tier at level 23,
/// out of scope); Inspire Competence stays at its level-19 fifth tier
/// (no further tier is defined within PF1's Core Rulebook); Lore Master
/// stays at its level-17 third tier (no further tier defined); Inspire
/// Heroics' flat save-bonus/AC-bonus magnitudes and base target count
/// (set at level 18) all carry over unchanged (the next target-count
/// rise lands at level 21, out of scope); Soothing Performance carries
/// over unchanged as a bounded grant-only identity record. The
/// level-20 "Special" column's sole entry, Deadly Performance (the
/// class capstone — PF1 Core Rulebook: "A bard of 20th level or higher
/// can use his performance to cause one enemy to die from joy or
/// sorrow... The target receives a Will save (DC 10 + 1/2 the bard's
/// level + the bard's Cha modifier) to negate the effect... If a
/// creature's saving throw fails, it dies"), is a genuinely NEW class
/// feature whose named Will-save DC is the EXACT SAME formula shape as
/// the already-grounded Fascinate DC and Frightening Tune DC, so only
/// that flat DC magnitude is grounded here (`BARD_DEADLY_PERFORMANCE_LEVEL`),
/// mirroring the Frightening Tune idiom exactly; no
/// death-effect-resolution engine, no audible/visual-performance-
/// requirement checking, and no range/targeting engine exists anywhere
/// in this codebase, so none of that is fabricated. This needed ZERO
/// new record types beyond the one new DC magnitude and ZERO new choice
/// slots.
pub(super) const MAX_SUPPORTED_BARD_LEVEL: u8 = 20;

/// PF1 Core Rulebook level gate at which Bard gains Frightening Tune
/// (14th level, verified independently against two primary sources:
/// d20pfsrd and the Archives of Nethys aonprd.com mirror both list
/// "Frightening tune, Versatile performance" as the Bard 14th-level
/// "Special" column entry). The rule text: "Each enemy within range
/// receives a Will save (DC 10 + 1/2 the bard's level + the bard's Cha
/// modifier) to negate the effect" — the exact same DC formula shape as
/// the already-grounded Fascinate DC, so this grounds ONLY that flat DC
/// magnitude (mirroring the Fascinate DC idiom exactly); the
/// range-based affected-creature scope, the fear/frightened-condition
/// resolution, and the audible-performance-execution prerequisite are
/// not computed because no targeting/range or condition-resolution
/// engine exists anywhere in this codebase.
pub(super) const BARD_FRIGHTENING_TUNE_LEVEL: u8 = 14;

/// PF1 Core Rulebook level gate at which Bard gains Deadly Performance,
/// the class capstone (20th level, verified independently against two
/// primary sources: d20pfsrd and the Archives of Nethys aonprd.com
/// mirror both list "Deadly performance" as the sole Bard 20th-level
/// "Special" column entry). The rule text: "The target receives a Will
/// save (DC 10 + 1/2 the bard's level + the bard's Cha modifier) to
/// negate the effect" — the exact same DC formula shape as the
/// already-grounded Fascinate DC and Frightening Tune DC, so this
/// grounds ONLY that flat DC magnitude (mirroring the Frightening Tune
/// idiom exactly); the audible/visual-performance-requirement checking,
/// the Will-save resolution, and the death-effect application itself
/// are not computed because no targeting/range, save-resolution, or
/// death-effect-resolution engine exists anywhere in this codebase.
pub(super) const BARD_DEADLY_PERFORMANCE_LEVEL: u8 = 20;

/// PF1 Core Rulebook level gate at which Bard gains Soothing Performance
/// (12th level, verified independently against two primary sources:
/// d20pfsrd and the Archives of Nethys aonprd.com mirror both list
/// "Soothing performance" as the sole Bard 12th-level "Special" column
/// entry). The rule text: "a bard of 12th level or higher can use his
/// performance to help heal the wounds of his allies... this ability
/// functions as mass cure serious wounds... this use of bardic
/// performance also removes the fatigued, sickened, and shaken
/// conditions." This is grounded ONLY as a bounded grant-only identity
/// record (value 0, non-fabricated): no healing-application engine and no
/// condition-removal engine exist anywhere in this codebase.
pub(super) const BARD_SOOTHING_PERFORMANCE_LEVEL: u8 = 12;

/// PF1 Core Rulebook level gate at which Bard gains Jack-of-All-Trades (10th
/// level, verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Jack-of-all-trades, versatile performance" as
/// the Bard 10th-level "Special" column entry).
pub(super) const BARD_JACK_OF_ALL_TRADES_LEVEL: u8 = 10;

/// PF1 Core Rulebook level gate at which Bard gains Well-Versed (2nd level, verified
/// independently against two primary sources: d20pfsrd and legacy.aonprd.com both
/// list "Versatile performance, well-versed" as the Bard 2nd-level special feature
/// entry).
pub(super) const BARD_WELL_VERSED_LEVEL: u8 = 2;

/// PF1 Core Rulebook Well-Versed magnitude: a flat +4 bonus on saving throws against
/// bardic performance, sonic, and language-dependent effects. Unlike Bardic
/// Knowledge or Fascinate, this magnitude is NOT level-scaled (it stays +4 for the
/// class feature's entire existence), verified against both primary sources rather
/// than assumed to follow the "half level" idiom used elsewhere on this seam.
pub(super) const BARD_WELL_VERSED_BONUS: i16 = 4;

/// PF1 Core Rulebook Bardic Performance additional-rounds-per-level constant: "At
/// each level after 1st a bard can use bardic performance for 2 additional rounds
/// per day" (verified against d20pfsrd and legacy.aonprd.com, not assumed from
/// Barbarian's superficially similar Rage-rounds progression).
pub(super) const BARD_PERFORMANCE_ADDITIONAL_ROUNDS_PER_LEVEL: i16 = 2;

/// `ClassAbilityActivation.ability_id` for Bard Bardic Performance (v0.6
/// alpha swarm, risks item 8) -- the flat compound-string idiom
/// `character_input.rs`'s `ClassAbilityActivation` doc comment specifies,
/// not a per-performance-type enum. The schema has no separate field
/// distinguishing WHICH performance is active (Inspire Courage,
/// Countersong, etc.), so an active `"bardic_performance"` entry is
/// interpreted as Inspire Courage specifically -- the only performance
/// this bounded slice grounds real mechanics for (mirrors this file's own
/// pre-existing `inspire_courage_bonus` explanation, which already
/// assumed the fixture's chosen performance is Inspire Courage without a
/// separate choice gate).
pub(super) const BARD_BARDIC_PERFORMANCE_ABILITY_ID: &str = "bardic_performance";

/// PF1 Core Rulebook level gate at which Bard gains Inspire Competence (3rd level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Inspire competence +2" as the Bard 3rd-level
/// special feature entry).
pub(super) const BARD_INSPIRE_COMPETENCE_LEVEL: u8 = 3;

/// PF1 Core Rulebook level at which the Inspire Competence flat magnitude first
/// increases from +2 to +3 (7th level, verified independently against two
/// primary sources: d20pfsrd and legacy.aonprd.com both list "Inspire
/// competence +3" as the Bard 7th-level special feature entry, and both state
/// the rule text "This bonus increases by +1 for every four levels the bard
/// has attained beyond 3rd (+3 at 7th, +4 at 11th, +5 at 15th, and +6 at
/// 19th)"). The next increase (to +4) lands at bard level 11 (grounded below
/// by `BARD_INSPIRE_COMPETENCE_THIRD_TIER_LEVEL`).
pub(super) const BARD_INSPIRE_COMPETENCE_SECOND_TIER_LEVEL: u8 = 7;

/// PF1 Core Rulebook Inspire Competence magnitude at the level it is first gained: a
/// flat +2 competence bonus on skill checks with a particular skill. Verified
/// against both primary sources.
pub(super) const BARD_INSPIRE_COMPETENCE_BONUS_FIRST_TIER: i16 = 2;

/// PF1 Core Rulebook Inspire Competence flat magnitude at or above the
/// second-tier level gate (7th level and beyond, until the third tier at
/// 11th level).
pub(super) const BARD_INSPIRE_COMPETENCE_BONUS_SECOND_TIER: i16 = 3;

/// PF1 Core Rulebook level at which the Inspire Competence flat magnitude
/// increases again from +3 to +4 (11th level, verified independently
/// against two primary sources: d20pfsrd and legacy.aonprd.com both list
/// "Inspire competence +4, inspire courage +3, lore master 2/day" as the
/// Bard 11th-level special feature entry). The next increase (to +5) lands
/// at bard level 15 (grounded below by
/// `BARD_INSPIRE_COMPETENCE_FOURTH_TIER_LEVEL`).
pub(super) const BARD_INSPIRE_COMPETENCE_THIRD_TIER_LEVEL: u8 = 11;

/// PF1 Core Rulebook Inspire Competence flat magnitude at or above the
/// third-tier level gate (11th level and beyond, until the fourth tier at
/// 15th level).
pub(super) const BARD_INSPIRE_COMPETENCE_BONUS_THIRD_TIER: i16 = 4;

/// PF1 Core Rulebook level at which the Inspire Competence flat magnitude
/// increases again from +4 to +5 (15th level, verified independently
/// against two primary sources: d20pfsrd and the Archives of Nethys
/// aonprd.com mirror, both byte-for-byte identical: "Inspire competence
/// +5, inspire heroics" is the Bard 15th-level special feature entry, and
/// both state the rule text "This bonus increases by +1 for every four
/// levels the bard has attained beyond 3rd"). The next increase (to +6)
/// lands at bard level 19 (grounded below by
/// `BARD_INSPIRE_COMPETENCE_FIFTH_TIER_LEVEL`).
pub(super) const BARD_INSPIRE_COMPETENCE_FOURTH_TIER_LEVEL: u8 = 15;

/// PF1 Core Rulebook Inspire Competence flat magnitude at or above the
/// fourth-tier level gate (15th level and beyond, until the fifth tier at
/// 19th level).
pub(super) const BARD_INSPIRE_COMPETENCE_BONUS_FOURTH_TIER: i16 = 5;

/// PF1 Core Rulebook level at which the Inspire Competence flat magnitude
/// increases again from +5 to +6 (19th level, verified independently
/// against two primary sources: d20pfsrd and the Archives of Nethys
/// aonprd.com mirror, both byte-for-byte identical: "Inspire competence
/// +6" is the Bard 19th-level "Special" column entry — the sole entry at
/// that level — and both state the rule text "This bonus increases by +1
/// for every four levels the bard has attained beyond 3rd"). The next
/// increase (to +7) lands at bard level 23, out of scope since only Bard
/// levels 1-19 are supported.
pub(super) const BARD_INSPIRE_COMPETENCE_FIFTH_TIER_LEVEL: u8 = 19;

/// PF1 Core Rulebook Inspire Competence flat magnitude at or above the
/// fifth-tier level gate (19th level and beyond, until the next tier at
/// 23rd level, out of scope here).
pub(super) const BARD_INSPIRE_COMPETENCE_BONUS_FIFTH_TIER: i16 = 6;

/// PF1 Core Rulebook level at which the Inspire Courage flat magnitude first
/// increases from +1 to +2 (5th level, verified independently against two
/// primary sources: d20pfsrd and legacy.aonprd.com both list "Inspire courage
/// +2, lore master 1/day" as the Bard 5th-level special feature entry, and
/// both state the rule text "At 5th level, and every six bard levels
/// thereafter, this bonus increases by +1"). The next increase (to +3) lands
/// at bard level 11 (grounded below by `BARD_INSPIRE_COURAGE_THIRD_TIER_LEVEL`).
pub(super) const BARD_INSPIRE_COURAGE_SECOND_TIER_LEVEL: u8 = 5;

/// PF1 Core Rulebook Inspire Courage flat magnitude below the second-tier
/// level gate.
pub(super) const BARD_INSPIRE_COURAGE_BONUS_FIRST_TIER: i16 = 1;

/// PF1 Core Rulebook Inspire Courage flat magnitude at or above the
/// second-tier level gate (5th level and beyond, until the third tier at
/// 11th level).
pub(super) const BARD_INSPIRE_COURAGE_BONUS_SECOND_TIER: i16 = 2;

/// PF1 Core Rulebook level at which the Inspire Courage flat magnitude
/// increases again from +2 to +3 (11th level, verified independently
/// against two primary sources: d20pfsrd and legacy.aonprd.com both list
/// "Inspire competence +4, inspire courage +3, lore master 2/day" as the
/// Bard 11th-level special feature entry — "every six bard levels
/// thereafter" after the 5th-level tier lands exactly on 11th). The next
/// increase (to +4) lands at bard level 17 (grounded below by
/// `BARD_INSPIRE_COURAGE_FOURTH_TIER_LEVEL`).
pub(super) const BARD_INSPIRE_COURAGE_THIRD_TIER_LEVEL: u8 = 11;

/// PF1 Core Rulebook Inspire Courage flat magnitude at or above the
/// third-tier level gate (11th level and beyond, until the fourth tier at
/// 17th level).
pub(super) const BARD_INSPIRE_COURAGE_BONUS_THIRD_TIER: i16 = 3;

/// PF1 Core Rulebook level at which the Inspire Courage flat magnitude
/// increases again from +3 to +4 (17th level, verified independently
/// against two primary sources: d20pfsrd and the Archives of Nethys
/// aonprd.com mirror, both byte-for-byte identical: "Inspire courage +4,
/// lore master 3/day" as the Bard 17th-level special feature entry — the
/// same "every six bard levels thereafter" cadence after the 11th-level
/// tier lands exactly on 17th). The next increase (to +5) lands at bard
/// level 23, out of scope since only Bard levels 1-17 are supported.
pub(super) const BARD_INSPIRE_COURAGE_FOURTH_TIER_LEVEL: u8 = 17;

/// PF1 Core Rulebook Inspire Courage flat magnitude at or above the
/// fourth-tier level gate (17th level and beyond, until the next tier at
/// 23rd level, out of scope here).
pub(super) const BARD_INSPIRE_COURAGE_BONUS_FOURTH_TIER: i16 = 4;

/// PF1 Core Rulebook level gate at which Bard gains Lore Master (5th level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Inspire courage +2, lore master 1/day" as the
/// Bard 5th-level special feature entry). The rule grants two capabilities:
/// an at-will "take 10 on any Knowledge skill check he has ranks in"
/// capability (no flat magnitude to ground — a resolution-mode toggle, not a
/// countable resource) and a flat "once per day... take 20 on any Knowledge
/// skill check" capability. Only the latter's flat usage count is grounded,
/// mirroring the Paladin Smite Evil / Wizard Force Missile uses-per-day
/// idiom; neither mechanic is executed against any actual Knowledge check.
pub(super) const BARD_LORE_MASTER_LEVEL: u8 = 5;

/// PF1 Core Rulebook Lore Master take-20 usage-count magnitude at the level
/// it is first gained: a flat 1/day count (verified against both primary
/// sources).
pub(super) const BARD_LORE_MASTER_TAKE_20_USES_PER_DAY: i16 = 1;

/// PF1 Core Rulebook level at which the Lore Master take-20 usage-count
/// magnitude increases from 1/day to 2/day (11th level, verified
/// independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Inspire competence +4, inspire courage +3,
/// lore master 2/day" as the Bard 11th-level special feature entry — the
/// same every-sixth-level-after-5th cadence as Inspire Courage). The next
/// increase (to 3/day) lands at bard level 17 (grounded below by
/// `BARD_LORE_MASTER_THIRD_TIER_LEVEL`).
pub(super) const BARD_LORE_MASTER_SECOND_TIER_LEVEL: u8 = 11;

/// PF1 Core Rulebook Lore Master take-20 usage-count magnitude at or above
/// the second-tier level gate (11th level and beyond, until the third tier
/// at 17th level).
pub(super) const BARD_LORE_MASTER_TAKE_20_USES_PER_DAY_SECOND_TIER: i16 = 2;

/// PF1 Core Rulebook level at which the Lore Master take-20 usage-count
/// magnitude increases again from 2/day to 3/day (17th level, verified
/// independently against two primary sources: d20pfsrd and the Archives of
/// Nethys aonprd.com mirror, both byte-for-byte identical: "Inspire courage
/// +4, lore master 3/day" as the Bard 17th-level special feature entry —
/// the same every-sixth-level-after-5th cadence as Inspire Courage). The
/// next increase lands beyond bard level 17, out of scope since only Bard
/// levels 1-17 are supported.
pub(super) const BARD_LORE_MASTER_THIRD_TIER_LEVEL: u8 = 17;

/// PF1 Core Rulebook Lore Master take-20 usage-count magnitude at or above
/// the third-tier level gate (17th level and beyond, out of scope beyond
/// here).
pub(super) const BARD_LORE_MASTER_TAKE_20_USES_PER_DAY_THIRD_TIER: i16 = 3;

/// PF1 Core Rulebook level gate at which Bard gains Inspire Heroics (15th
/// level, verified independently against two primary sources: d20pfsrd and
/// the Archives of Nethys aonprd.com mirror both list "Inspire competence
/// +5, inspire heroics" as the Bard 15th-level "Special" column entry). The
/// rule text: "A bard of 15th level or higher can inspire tremendous
/// heroism in himself or a single ally within 30 feet... Inspired
/// creatures gain a +4 morale bonus on saving throws and a +4 dodge bonus
/// to AC." This grounds only the two flat, non-level-scaled magnitude
/// numbers (the save bonus and the AC bonus) and the flat base target
/// count, mirroring the Well-Versed flat-magnitude idiom and the Fascinate
/// affected-creature-count idiom respectively; no targeting, save
/// resolution, or AC application is computed because no such engine exists
/// anywhere in this codebase.
pub(super) const BARD_INSPIRE_HEROICS_LEVEL: u8 = 15;

/// PF1 Core Rulebook Inspire Heroics flat morale bonus on saving throws (a
/// flat, non-level-scaled +4, verified against both primary sources).
pub(super) const BARD_INSPIRE_HEROICS_SAVE_BONUS: i16 = 4;

/// PF1 Core Rulebook Inspire Heroics flat dodge bonus to AC (a flat,
/// non-level-scaled +4, verified against both primary sources).
pub(super) const BARD_INSPIRE_HEROICS_AC_BONUS: i16 = 4;

/// PF1 Core Rulebook Inspire Heroics base target count at the level it is
/// first gained: "himself or a single ally" is a flat 1 target (verified
/// against both primary sources).
pub(super) const BARD_INSPIRE_HEROICS_BASE_TARGET_COUNT: i16 = 1;

/// SD18 (cycle-2026-07-16T0900) PF1 Core Rulebook level gate at which
/// Inspire Heroics' target count first rises: "For every three bard levels
/// the character attains beyond 15th, he can inspire heroics in an
/// additional creature" lands exactly at bard level 18 (15 + 3), verified
/// against the rule text directly rather than assumed, mirroring the
/// already-generalized tiered if/else chain idiom used for Inspire
/// Courage / Inspire Competence / Lore Master's own tier constants.
pub(super) const BARD_INSPIRE_HEROICS_TARGET_COUNT_SECOND_TIER_LEVEL: u8 = 18;

/// PF1 Core Rulebook Inspire Heroics target count at its second tier (bard
/// level 18+): the base 1 target plus the rule's own "+1 creature per three
/// bard levels beyond 15th" scaling, i.e. 2 at level 18.
pub(super) const BARD_INSPIRE_HEROICS_BASE_TARGET_COUNT_SECOND_TIER: i16 = 2;

/// `AT-34-E3-001` (`class_feature_option_pool_record_with_magnitude_not_
/// held_by_engine` mechanism, cycle 5): PF1 Core Rulebook level gate at
/// which the Bard gains Suggestion, verified directly against this repo's
/// own ingested corpus record's `PREVARGTEQ:BardicPerformanceLVL,6` token
/// (`data/corpus/core_rulebook/class_feature/bard/bardic_performance.json`),
/// not from memory.
pub(super) const BARD_SUGGESTION_LEVEL: u8 = 6;

/// PF1 Core Rulebook level gate at which the Bard gains Inspire Greatness,
/// verified the same way against `PREVARGTEQ:BardicPerformanceLVL,9`.
pub(super) const BARD_INSPIRE_GREATNESS_LEVEL: u8 = 9;

/// PF1 Core Rulebook level gate at which the Bard gains Mass Suggestion,
/// verified the same way against `PREVARGTEQ:BardicPerformanceLVL,18`.
pub(super) const BARD_MASS_SUGGESTION_LEVEL: u8 = 18;

/// PF1 Core Rulebook Inspire Greatness maximum allies-affected count (the
/// corpus's own `BONUS:VAR|InspireGreatnessAllies|min((BardicPerformanceLVL-6)/3,4)`
/// token caps at 4 within the level range this codebase ever reaches).
pub(super) const BARD_INSPIRE_GREATNESS_MAX_ALLIES: i16 = 4;

/// v0.6 alpha swarm, risks item 8 (first APG/ACG closure): whether `input`
/// is a single-class Skald at a level within `acg::class_chassis_resolve`'s
/// declared ceiling for Skald. This is the first time this gate admits any
/// APG/ACG class -- deliberately an EXACT match on `AcgClassId::Skald`, not
/// a broad `AcgClassId::from_class_id_str(...).is_some()` check (which
/// would resolve `Some` for any of the 10 ACG classes, silently admitting
/// all of them into real save/combat/skill-modifier computation --
/// verified directly against `AcgClassId::from_class_id_str`'s own
/// `Self::ALL.iter()` search before writing this). The other 9 ACG classes
/// (and every APG class) remain genuinely unsupported by this gate; only
/// Skald's own chassis dispatch (`compute_acg_class_chassis`) grounds
/// anything beyond BAB/save for now.
pub(super) fn is_supported_skald_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if AcgClassId::from_class_id_str(&class_level.class_id) != Some(AcgClassId::Skald) {
        return false;
    }
    acg::class_chassis_resolve(AcgClassId::Skald, class_level.level, RuleSetId::Acg).is_some()
}

/// Skald's Raging Song rounds-per-day budget: 3 + Charisma modifier + 2 *
/// (level - 1) (PF1 Advanced Class Guide, verified against the PCGen corpus
/// DESC text -- the same shape as `bard_bardic_performance_rounds_per_day`
/// with a different base, 3 rather than 4).
///
/// **Deliberate, ruled deviation from the corpus's literal `BONUS:VAR`
/// token** (task #50, 2026-07-28; lead ruling recorded as
/// `risks-and-open-questions.md` item 69, the same precedent class as
/// item 50's Swashbuckler deed gate -- see
/// `swashbuckler_deed_tier_reached`'s own doc comment for the sibling
/// case). The raw corpus token, `BONUS:VAR|SkaldRagingSongRoundsPerDay|
/// 3+CHA+(2*SkaldLVL)` under `KEY:Skald ~ Raging Song` in
/// `acg_abilities_class.lst`, disagrees with the formula below by a
/// uniform `-2` at every level: expanding this formula gives
/// `3+CHA+2*(level-1) = 1+CHA+2*level`, against the token's own
/// `3+CHA+2*level`.
///
/// Resolved without any RAW recollection, using a cross-check already in
/// the corpus: Skald's own DESC text states explicitly that Raging Song
/// "counts as the bard's bardic performance special ability for any
/// effect that affects bardic performances" -- naming Bard's own Bardic
/// Performance as a structurally identical sibling record, already
/// shipped and lead-verified this session. Bard's own corpus token,
/// `BONUS:VAR|BardicPerformanceDuration|2+CHA+(2*BardicPerformanceLVL)`,
/// is algebraically identical to Bard's own shipped
/// `bard_bardic_performance_rounds_per_day` formula (`4+CHA+2*(level-1)`,
/// confirmed by direct computation at levels 1/2/3/20). That reveals the
/// corpus's own authoring convention for this exact shape: the token's
/// flat addend equals the DESC-prose base MINUS 2, because the token
/// multiplies by `level` directly while the prose's "+2 per level after
/// 1st" implies `level-1`. Applying that same convention to Skald: the
/// DESC-prose base is 3 ("3 + his Charisma modifier" at 1st level,
/// explicit in the DESC text), so a correctly-authored token would read
/// `1+CHA+(2*SkaldLVL)` -- not the `3+CHA+(2*SkaldLVL)` actually shipped
/// in the corpus. The shipped Rust formula below already matches both
/// the DESC prose and Bard's own correctly-authored sibling token
/// exactly (levels 1/2/3/20 give 3/5/7/41 either way); it is the
/// corpus's raw `BONUS:VAR` token for Skald specifically that carries
/// the transcription defect (missing the `-2` offset applied correctly
/// everywhere else in this same family).
///
/// **Ruling: no code change.** `SKALD_RAGING_SONG_BASE_ROUNDS_PER_DAY`
/// and this formula are already correct and stay exactly as they are --
/// this is a known, knowingly-overridden corpus defect, not an
/// uncorrected bug. Pure function (v0.6 alpha swarm, risks item 8, first
/// APG/ACG closure) so the real rage-execution validation
/// (`ground_or_block_skald_inspired_rage`,
/// `active_skald_inspired_rage_bonus`) shares one source of truth.
///
/// **`Extra Performance` reaches this pool too** (v0.6 alpha swarm,
/// 2026-07-29). The feat is a Core Rulebook record whose own
/// `cr_feats.lst` line names only `BardicPerformanceDuration` and gates
/// on `PREABILITY:1,CATEGORY=Special Ability,TYPE.Bardic Performance` --
/// read there and nowhere else, it is unambiguously Bard-only, which is
/// how this engine treated it. `acg_feats.lst` then carries THREE
/// `CATEGORY=FEAT|Extra Performance.MOD` records (lines 232-234) which
/// together `PRE:.CLEAR` that gate, add
/// `BONUS:VAR|SkaldRagingSongRoundsPerDay|6`, and re-impose a widened
/// `PREMULT:1,[...TYPE.SkaldRagingSong],[...TYPE.Bardic Performance]`.
/// The real value for Skald therefore lives entirely on `.MOD` records
/// in a different book from the feat itself -- the same trap shape that
/// hid `WeaponFocusToHit`, here in its additive direction: not a `0`
/// masquerading as a magnitude, but a whole second consumer invisible
/// from the base record.
///
/// The magnitude is `6`, identical to the Bard half, and the prose
/// ("You can use bardic performance for 6 additional rounds per day")
/// governs both because it is one feat with one benefit line. Skald's
/// Raging Song is one shared pool that Inspired Rage and every other
/// song spends from, so widening it here widens it for all of them.
pub(super) fn skald_inspired_rage_rounds_per_day(
    charisma_modifier: i16,
    level: u8,
    selected_feats: &[String],
) -> i16 {
    SKALD_RAGING_SONG_BASE_ROUNDS_PER_DAY
        + charisma_modifier
        + 2 * (i16::from(level) - 1)
        + extra_resource_feat_bonus(
            selected_feats,
            EXTRA_PERFORMANCE_FEAT_KEY,
            EXTRA_ROUNDS_PER_DAY,
        )
}

/// Skald's Inspired Rage magnitude tier: (Strength/Constitution morale
/// bonus, Will-save morale bonus), verified against the PCGen corpus DESC
/// text and `BONUS:VAR` formulas: STR/CON = 2 + floor(level/8)*2 (+2 at
/// 1-7, +4 at 8-15, +6 at 16+), Will = 1 + floor(level/4) (+1 at 1-3, rising
/// every 4 levels). The Armor Class penalty is a flat -1 at every tier
/// (unlike Barbarian's Rage, which stays -2 at every tier for a different
/// reason -- Inspired Rage's penalty simply never scales at all), so it is
/// not part of this tuple -- callers needing it use
/// `SKALD_INSPIRED_RAGE_ARMOR_CLASS_PENALTY` directly. Pure function so the
/// real rage-execution engine (ability modifiers, total saves) shares one
/// source of truth with any future informational explanation record.
pub(super) fn skald_inspired_rage_tier(level: u8) -> (i16, i16) {
    let strength_constitution_bonus = 2 + (i16::from(level) / 8) * 2;
    let will_save_bonus = 1 + i16::from(level) / 4;
    (strength_constitution_bonus, will_save_bonus)
}

/// Whether `input` is a Skald validly, actively singing Inspired Rage right
/// now, and if so, the magnitude tier to apply (v0.6 alpha swarm, risks
/// item 8, first APG/ACG closure). Class-ownership-gated by construction:
/// only returns `Some` when `class_levels` actually contains Skald, so a
/// non-Skald character's stray `class_ability_activations` entry for
/// `SKALD_INSPIRED_RAGE_ABILITY_ID` is never read at all, mirroring
/// `active_barbarian_rage_bonus` exactly. An activation present but not
/// `ActiveState::EquippedActive`, or one that exceeds the grounded
/// rounds-per-day budget, is treated the same as "not singing" here --
/// pushes no diagnostic itself (`ground_or_block_skald_inspired_rage` is
/// the single place that pushes the over-budget claim-blocking diagnostic
/// and the informational recognition records).
///
/// Self-application: the PCGen corpus text carves out an explicit
/// exception "allies other than the skald cannot use any Charisma-,
/// Dexterity-, or Intelligence-based skills..." while raging -- an
/// exception naming the skald specifically only makes sense if she is
/// among the affected targets by default, so self-application is modeled
/// here as Inspired Rage's ordinary behavior, not a narrowed alternate
/// use. This inference is weaker evidence than Bard's own explicit
/// "including yourself" phrasing (confirmed absent from the Raging Song/
/// Inspired Rage corpus text entirely), so the grounded explanation text
/// in `ground_or_block_skald_inspired_rage` states this evidentiary gap
/// honestly rather than implying it is as settled as Bard's or Cleric's
/// Touch of Good self-targeting.
pub(super) fn active_skald_inspired_rage_bonus(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
) -> Option<(u8, i16, i16)> {
    let skald_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == SKALD_CLASS_ID)
        .map(|class_level| class_level.level)?;

    let activation = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == SKALD_INSPIRED_RAGE_ABILITY_ID)?;

    if activation.active_state != ActiveState::EquippedActive {
        return None;
    }

    if let Some(rounds_consumed) = activation.rounds_consumed_today {
        let charisma_modifier = ability_modifier_for(ability_modifiers, "charisma");
        let rounds_per_day = skald_inspired_rage_rounds_per_day(
            charisma_modifier,
            skald_level,
            &input.chosen.selected_feats,
        );
        if i32::from(rounds_consumed) > i32::from(rounds_per_day) {
            return None;
        }
    }

    let (strength_constitution_bonus, will_save_bonus) = skald_inspired_rage_tier(skald_level);
    Some((skald_level, strength_constitution_bonus, will_save_bonus))
}

/// Grounds or claim-blocks Skald's Inspired Rage execution engine for
/// `skald_level` (v0.6 alpha swarm, risks item 8, first APG/ACG closure).
/// Called from `compute_acg_class_chassis`'s Skald branch, gated only on
/// Skald class-ownership -- mirrors `ground_or_block_barbarian_rage`'s
/// structure exactly: a character who simply isn't singing is a genuinely
/// valid PF1 posture (grounds a real "not singing" recognition record,
/// not a claim-blocking one), and an activation that IS active but exceeds
/// the grounded rounds-per-day budget is a genuine posture violation and
/// claim-blocks.
///
/// This function no longer pushes Skald's own "other features deferred"
/// diagnostic itself (v0.6 alpha swarm, risks item 8, Skald spellcasting
/// closure) -- that diagnostic is now independent of Inspired Rage's own
/// state and is pushed exactly once from the top-level Skald dispatch
/// branch, alongside `ground_or_block_skald_spellcasting`'s own call. See
/// `push_skald_other_features_deferred_diagnostic`'s own doc comment.
///
/// Also grounds Raging Climber/Swimmer's shared RagePowersLVL magnitude
/// (task #54) at each of the three branch points below, via
/// `ground_raging_climber_and_swimmer` -- see that function's own doc
/// comment.
pub(super) fn ground_or_block_skald_inspired_rage(
    input: &CharacterInput,
    skald_level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(activation) = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == SKALD_INSPIRED_RAGE_ABILITY_ID)
    else {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.inspired_rage_execution.not_singing".to_owned(),
            value: 0,
            detail: format!(
                "Skald level {skald_level} is not currently singing Raging Song / Inspired Rage \
                 (no class_ability_activations entry for \
                 \"{SKALD_INSPIRED_RAGE_ABILITY_ID}\"): a genuinely valid PF1 posture, so no rage \
                 bonus, penalty, or budget is claimed. This grounds the Inspired Rage execution \
                 engine's \"inactive\" branch only; singing is grounded separately below when an \
                 active, in-budget activation is present"
            ),
        });
        ground_raging_climber_and_swimmer(
            skald_level,
            "Skald",
            "class_feature.acg.skald",
            false,
            is_supported_skald_single_class(input),
            explanations,
        );
        return;
    };

    let charisma_modifier = ability_modifier_for(ability_modifiers, "charisma");
    let rounds_per_day = skald_inspired_rage_rounds_per_day(
        charisma_modifier,
        skald_level,
        &input.chosen.selected_feats,
    );

    if let Some(rounds_consumed) = activation.rounds_consumed_today
        && i32::from(rounds_consumed) > i32::from(rounds_per_day) {
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.acg.skald.inspired_rage_execution.rounds_exceeded".to_owned(),
                message: format!(
                    "Skald level {skald_level} Raging Song activation claims \
                     {rounds_consumed} rounds consumed today, exceeding the grounded \
                     rounds-per-day budget of {rounds_per_day} (3 + Charisma modifier \
                     ({charisma_modifier}) + 2 * (level - 1)): a genuine posture violation, so \
                     no rage bonus, penalty, or budget is claimed for this input"
                ),
                claim_blocking: true,
            });
            return;
        }

    match activation.active_state {
        ActiveState::EquippedActive => {
            let (strength_constitution_bonus, will_save_bonus) =
                skald_inspired_rage_tier(skald_level);
            let rounds_consumed_today = activation.rounds_consumed_today.unwrap_or(0);
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.skald.inspired_rage_execution.active".to_owned(),
                value: 0,
                detail: format!(
                    "Skald level {skald_level} is actively singing Inspired Rage, within the \
                     grounded rounds-per-day budget ({rounds_per_day} rounds; \
                     {rounds_consumed_today} consumed today). The \
                     +{strength_constitution_bonus} Strength / +{strength_constitution_bonus} \
                     Constitution / +{will_save_bonus} Will morale bonuses and the \
                     {SKALD_INSPIRED_RAGE_ARMOR_CLASS_PENALTY} Armor Class penalty are applied to \
                     the integrated ability modifiers, total saves, and baseline Armor Class \
                     respectively -- see apply_skald_inspired_rage_ability_bonuses, \
                     compute_total_saves, and compute_combat_baseline. Self-application is \
                     modeled by inference from the corpus's \"allies other than the skald\" \
                     exception clause (an exception naming the skald specifically only makes \
                     sense if she is among the affected targets by default); the corpus carries \
                     no explicit self-inclusion language here, unlike Bard's \"including \
                     yourself\""
                ),
            });
            ground_raging_climber_and_swimmer(
                skald_level,
                "Skald",
                "class_feature.acg.skald",
                true,
                is_supported_skald_single_class(input),
                explanations,
            );
        }
        ActiveState::SelectedInactive | ActiveState::Absent => {
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.skald.inspired_rage_execution.not_singing".to_owned(),
                value: 0,
                detail: format!(
                    "Skald level {skald_level} has a \"{SKALD_INSPIRED_RAGE_ABILITY_ID}\" \
                     activation entry but it is not active for this snapshot: a genuinely valid \
                     PF1 posture (available but not currently singing), so no rage bonus, \
                     penalty, or budget is claimed"
                ),
            });
            ground_raging_climber_and_swimmer(
                skald_level,
                "Skald",
                "class_feature.acg.skald",
                false,
                is_supported_skald_single_class(input),
                explanations,
            );
        }
    }
}

/// Skald's own self-Damage Reduction magnitude at `level` (deepening
/// 2026-07-26, task #7): 0 below level 9, 1 from level 9, 2 from level
/// 14, 3 from level 19 -- verified directly against the raw corpus DESC
/// text ("At 9th level, a skald gains damage reduction... At 14th and
/// 19th level this damage reduction rises by 1 point"), the identical
/// shape to Barbarian's own `barbarian_damage_reduction_amount`-style
/// progression (see `BARBARIAN_DAMAGE_REDUCTION_LEVEL` and its sibling
/// constants).
pub(super) fn skald_damage_reduction_amount(level: u8) -> i16 {
    if level < SKALD_DAMAGE_REDUCTION_LEVEL {
        0
    } else if level < SKALD_DAMAGE_REDUCTION_TWO_LEVEL {
        1
    } else if level < SKALD_DAMAGE_REDUCTION_THREE_LEVEL {
        2
    } else {
        3
    }
}

/// Grounds Skald's own self-Damage Reduction as a standalone explanation
/// record, mirroring Barbarian's own `class_feature.barbarian.
/// damage_reduction` shape exactly (deepening 2026-07-26, task #7): a
/// real, level-gated, flat-magnitude fact, never applied to any incoming-
/// damage total (none exists anywhere in this codebase). Always grounds a
/// record, including an honest value-0 "not yet gained" record below
/// level 9, the same "ground the absence, don't omit it" discipline every
/// level-gated fact this session uses. The DR's own ally-extension via
/// Raging Song stays explicitly deferred -- this engine models no allies
/// or ally-targeting mechanism at all -- so only the skald's own self-DR
/// is grounded; this function never claim-blocks.
pub(super) fn ground_skald_damage_reduction(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    let damage_reduction_value = skald_damage_reduction_amount(level);
    if damage_reduction_value == 0 {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.damage_reduction".to_owned(),
            value: 0,
            detail: format!(
                "Skald Damage Reduction at skald level {level}: correctly absent at level \
                 {level} by PF1 Advanced Class Guide level gate; the at-grant magnitude is named \
                 but not computed. Damage Reduction is a 9th-level skald class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.damage_reduction".to_owned(),
            value: damage_reduction_value,
            detail: format!(
                "Skald Damage Reduction granted at skald level {level} (PF1 Advanced Class \
                 Guide, 9th-level skald class feature, rising by 1 point at 14th and 19th level \
                 -- the level-{level} magnitude is {damage_reduction_value}/-): subtract \
                 {damage_reduction_value} from the damage the skald takes each time she is dealt \
                 damage from a weapon or a natural attack. This applies to the skald's own \
                 damage only -- the same DR granted to allies affected by her Raging Song stays \
                 deferred, since this codebase models no allies or ally-targeting mechanism; the \
                 subtraction against an actual incoming-damage total is also not computed, since \
                 no damage-resolution engine or incoming-damage total exists anywhere in this \
                 codebase."
            ),
        });
    }
}

/// Skald's own Bardic Knowledge magnitude at `level` (deepening
/// 2026-07-26, task #7): "adds half his class level (minimum 1)... on
/// all Knowledge skill checks, and may make all Knowledge skill checks
/// untrained" -- verified directly against `acg_abilities_class.lst`'s
/// own `BONUS:VAR|BardicKnowledgeSkillBonus|max(1,SkaldLVL/2)` (or
/// equivalent DESC-derived formula), byte-identical to Bard's own
/// already-shipped `bardic_knowledge_bonus` formula
/// (`(level_value / 2).max(1)`). Kept as a separate Skald-named copy
/// rather than calling Bard's function directly, the same "parallel
/// copy over cross-class-function-reuse" discipline this closure's own
/// base-spells-per-day table already used.
pub(super) fn skald_bardic_knowledge_bonus(level: u8) -> i16 {
    (i16::from(level) / 2).max(1)
}

/// Grounds Skald's own Bardic Knowledge as a standalone explanation
/// record, mirroring Bard's own `class_chassis.bard.bardic_knowledge`
/// shape exactly (deepening 2026-07-26, task #7, correcting an earlier
/// over-strict "needs a live consumer" exclusion -- see Inquisitor's own
/// task #18 for the same correction applied first): no Knowledge-skill
/// total exists anywhere in this codebase, so this grounds only the flat
/// competence bonus value, naming honestly that it is not a full
/// Knowledge-check resolution engine. Unconditional on class ownership
/// and level alone; never claim-blocks.
pub(super) fn ground_skald_bardic_knowledge(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    let bardic_knowledge_bonus = skald_bardic_knowledge_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.skald.bardic_knowledge_bonus".to_owned(),
        value: bardic_knowledge_bonus,
        detail: format!(
            "Skald level {level} Bardic Knowledge: a competence bonus on Knowledge skill checks \
             equal to max(skald level / 2, 1) = {bardic_knowledge_bonus}, and lets the skald \
             make any Knowledge skill check untrained. No Knowledge-skill total exists anywhere \
             in this codebase, so this grounds only the flat bonus value -- not a full \
             Knowledge-check resolution engine, mirroring Bard's own Bardic Knowledge"
        ),
    });
}

/// Skald's own Well-Versed magnitude (task #50): flat, not level-scaled
/// (see `SKALD_WELL_VERSED_BONUS`'s own doc comment for the corpus
/// verification). Pure function so the standalone explanation record
/// below has one source of truth for the magnitude.
pub(super) fn skald_well_versed_bonus() -> i16 {
    SKALD_WELL_VERSED_BONUS
}

/// Grounds Skald's own Well-Versed as a standalone explanation record
/// (task #50), mirroring Bard's own `class_feature.bard.well_versed`
/// shape exactly: below `SKALD_WELL_VERSED_LEVEL` this is a correct PF1
/// level-gate absence (value 0); at or above it, a flat +4 bonus on
/// saving throws against bardic performance, sonic, and language-
/// dependent effects. Never applied to any actual save total, since no
/// saving-throw-resolution engine exists in this codebase; never claim-
/// blocks.
pub(super) fn ground_skald_well_versed(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    if level < SKALD_WELL_VERSED_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.well_versed".to_owned(),
            value: 0,
            detail: format!(
                "Skald Well-Versed at skald level {level}: correctly absent at level {level} by \
                 PF1 Advanced Class Guide level gate; the at-grant rule is named but not \
                 computed. Well-Versed is a 2nd-level Skald class feature."
            ),
        });
    } else {
        let bonus = skald_well_versed_bonus();
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.well_versed".to_owned(),
            value: bonus,
            detail: format!(
                "Skald Well-Versed granted at skald level {level} (PF1 Advanced Class Guide, \
                 2nd-level Skald class feature): a flat +{bonus} bonus on saving throws made \
                 against bardic performance, sonic, and language-dependent effects, not \
                 level-scaled -- byte-identical to Bard's own Well-Versed. This is a standalone \
                 explanation record only; it is never applied to any actual save total because \
                 no saving-throw-resolution engine exists anywhere in this codebase"
            ),
        });
    }
}

/// Skald's own Spell Kenning uses-per-day magnitude (task #50):
/// `(1+SkaldLVL)/6` verified directly against the raw corpus's own
/// `BONUS:VAR|SkaldSpellKenningUsesPerDay|(1+SkaldLVL)/6` token, self-
/// gating to 0 below level 5 by its own floor division (matching the
/// real grant level exactly -- see `SKALD_SPELL_KENNING_LEVEL`'s own
/// doc comment).
pub(super) fn skald_spell_kenning_uses_per_day(level: u8) -> i16 {
    (1 + i16::from(level)) / 6
}

/// Grounds Skald's own Spell Kenning pool size as a standalone
/// explanation record (task #50), the same "pool size, use not
/// modelled" shape as `ground_swashbuckler_deeds`'s Derring-Do/Charmed
/// Life uses-per-day records: no spell-casting-from-another-class'-list
/// mechanism exists anywhere in this codebase, so only the flat
/// uses-per-day count is grounded, named honestly. Below
/// `SKALD_SPELL_KENNING_LEVEL` this is a correct PF1 level-gate absence
/// (value 0); never claim-blocks.
pub(super) fn ground_skald_spell_kenning(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    if level < SKALD_SPELL_KENNING_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.spell_kenning_uses_per_day".to_owned(),
            value: 0,
            detail: format!(
                "Skald Spell Kenning at skald level {level}: correctly absent at level {level} \
                 by PF1 Advanced Class Guide level gate; the at-grant rule is named but not \
                 computed. Spell Kenning is a 5th-level Skald class feature."
            ),
        });
    } else {
        let uses_per_day = skald_spell_kenning_uses_per_day(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.spell_kenning_uses_per_day".to_owned(),
            value: uses_per_day,
            detail: format!(
                "Skald Spell Kenning granted at skald level {level} (PF1 Advanced Class Guide, \
                 5th-level Skald class feature): usable {uses_per_day} times per day, letting \
                 the skald cast a bard/cleric/sorcerer-wizard spell as if it were one of his own \
                 known skald spells, expending a skald spell slot of the same level. This \
                 grounds only the flat uses-per-day pool size -- not the spell-borrowing \
                 mechanism itself, since no cross-class spell-list-borrowing engine exists \
                 anywhere in this codebase"
            ),
        });
    }
}

/// Skald's own Lore Master uses-per-day magnitude (task #50):
/// `min((SkaldLVL-1)/6,3)` verified directly against the raw corpus's
/// own `BONUS:VAR|SkaldLoreMasterUsesPerDay|min((SkaldLVL-1)/6,3)`
/// token -- a genuine two-argument `min()`, unlike Versatile
/// Performance's single-argument one below. Self-gates to 0 below level
/// 7 by its own floor division, rising to 1/day at 13th, and capping at
/// `SKALD_LORE_MASTER_MAX_USES_PER_DAY` (3) at level 19 (see
/// `SKALD_LORE_MASTER_LEVEL`'s own doc comment for the real per-level
/// grant-row confirmation).
pub(super) fn skald_lore_master_uses_per_day(level: u8) -> i16 {
    ((i16::from(level) - 1) / 6).min(SKALD_LORE_MASTER_MAX_USES_PER_DAY)
}

/// Grounds Skald's own Lore Master pool size as a standalone
/// explanation record (task #50), mirroring Bard's own
/// `class_feature.bard.lore_master` shape and the same "pool size, use
/// not modelled" idiom as `ground_swashbuckler_deeds`: neither the
/// take-10 nor the take-20 mechanic is executed against any Knowledge
/// check, since no skill-check-resolution engine exists anywhere in
/// this codebase. Below `SKALD_LORE_MASTER_LEVEL` this is a correct PF1
/// level-gate absence (value 0); never claim-blocks.
pub(super) fn ground_skald_lore_master(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    if level < SKALD_LORE_MASTER_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.lore_master_uses_per_day".to_owned(),
            value: 0,
            detail: format!(
                "Skald Lore Master at skald level {level}: correctly absent at level {level} by \
                 PF1 Advanced Class Guide level gate; the at-grant rule is named but not \
                 computed. Lore Master is a 7th-level Skald class feature."
            ),
        });
    } else {
        let uses_per_day = skald_lore_master_uses_per_day(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.lore_master_uses_per_day".to_owned(),
            value: uses_per_day,
            detail: format!(
                "Skald Lore Master granted at skald level {level} (PF1 Advanced Class Guide, \
                 7th-level Skald class feature): take 10 on any Knowledge skill check he has \
                 ranks in (an at-will capability with no flat magnitude to ground), plus take 20 \
                 on any Knowledge skill check {uses_per_day} times per day (rising to 2/day at \
                 13th level and 3/day at 19th level). This grounds only the take-20 half's flat \
                 uses-per-day pool size -- not a full Knowledge-check resolution engine"
            ),
        });
    }
}

/// Skald's own Versatile Performance slot-count magnitude (task #50):
/// `(SkaldLVL+3)/5`, verified directly against the raw corpus's own
/// `BONUS:ABILITYPOOL|Skald Versatile Performance|min((SkaldLVL+3)/5)`
/// token. That token's `min()` is genuinely single-argument in the raw
/// corpus (confirmed directly against `acg_abilities_class.lst`, not a
/// transcription defect the way Raging Song's token is) -- min of one
/// argument is just that argument, so this grounds `(SkaldLVL+3)/5`
/// directly rather than inventing a second cap operand. Self-gates to 0
/// below level 2 by its own floor division, matching the real per-level
/// grant row and the DESC's own schedule (1 slot at level 2-6, 2 at
/// 7-11, 3 at 12-16, continuing every 5 levels thereafter -- see
/// `SKALD_VERSATILE_PERFORMANCE_LEVEL`'s own doc comment).
pub(super) fn skald_versatile_performance_slot_count(level: u8) -> i16 {
    (i16::from(level) + 3) / 5
}

/// Grounds Skald's own Versatile Performance slot count as a standalone
/// explanation record (task #50), the same pool-size-only shape as
/// Skald's own Rage Powers below (not Bard's own fixed three-slot
/// choice-list table, since Skald's own corpus formula genuinely keeps
/// growing past 3 slots at higher levels, a different shape from
/// Bard's). No Perform-substitution execution mechanism exists anywhere
/// in this codebase, so only the flat slot count is grounded. Below
/// `SKALD_VERSATILE_PERFORMANCE_LEVEL` this is a correct PF1 level-gate
/// absence (value 0); never claim-blocks.
pub(super) fn ground_skald_versatile_performance(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    if level < SKALD_VERSATILE_PERFORMANCE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.versatile_performance_slot_count".to_owned(),
            value: 0,
            detail: format!(
                "Skald Versatile Performance at skald level {level}: correctly absent at level \
                 {level} by PF1 Advanced Class Guide level gate; the at-grant rule is named but \
                 not computed. Versatile Performance is a 2nd-level Skald class feature."
            ),
        });
    } else {
        let slot_count = skald_versatile_performance_slot_count(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.versatile_performance_slot_count".to_owned(),
            value: slot_count,
            detail: format!(
                "Skald Versatile Performance granted at skald level {level} (PF1 Advanced Class \
                 Guide, 2nd-level Skald class feature): {slot_count} Perform-substitution \
                 slot(s) selected so far (1 at level 2-6, 2 at 7-11, 3 at 12-16, and so on every \
                 5 levels). This grounds only the flat slot-count magnitude -- not which Perform \
                 type is chosen or any actual skill-substitution execution, since no \
                 skill-check-resolution engine or choice-selection mechanism for this feature \
                 exists anywhere in this codebase"
            ),
        });
    }
}

/// Skald's own Rage Powers pool-SIZE magnitude (task #50): `RagePowersLVL
/// / 3`, verified directly against the raw corpus's own
/// `BONUS:ABILITYPOOL|Rage Power|RagePowersLVL/3` and
/// `BONUS:VAR|RagePowersLVL|SkaldLVL` tokens under `KEY:Skald ~ Rage
/// Powers`. `RagePowersLVL` is set unconditionally from `SkaldLVL` (no
/// Swashbuckler-deed-style borrowed-variable gap), so this grounds
/// `SkaldLVL/3` directly. This is Skald's own POOL-SIZE count only --
/// which individual rage powers are selectable/legal for a Skald to
/// grant via Raging Song (a separate build) is explicitly out of scope
/// here. Self-gates to 0 below level 3 by its own floor division,
/// matching the real per-level grant row and the DESC's own "at 3rd
/// level and every 3 levels thereafter" schedule (see
/// `SKALD_RAGE_POWERS_LEVEL`'s own doc comment).
pub(super) fn skald_rage_powers_pool_size(level: u8) -> i16 {
    i16::from(level) / 3
}

/// Grounds Skald's own Rage Powers pool size as a standalone
/// explanation record (task #50), the same pool-size-only shape as
/// `swashbuckler_panache_max`/Warpriest Blessing uses/day: no
/// rage-power-selection choice list or execution mechanism exists for
/// Skald anywhere in this codebase (individual rage powers are a
/// separate build), so only the flat count is grounded. Below
/// `SKALD_RAGE_POWERS_LEVEL` this is a correct PF1 level-gate absence
/// (value 0); never claim-blocks.
pub(super) fn ground_skald_rage_powers_pool_size(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    if level < SKALD_RAGE_POWERS_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.rage_powers_pool_size".to_owned(),
            value: 0,
            detail: format!(
                "Skald Rage Powers at skald level {level}: correctly absent at level {level} by \
                 PF1 Advanced Class Guide level gate; the at-grant rule is named but not \
                 computed. Rage Powers is a 3rd-level Skald class feature."
            ),
        });
    } else {
        let pool_size = skald_rage_powers_pool_size(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.rage_powers_pool_size".to_owned(),
            value: pool_size,
            detail: format!(
                "Skald Rage Powers pool size at skald level {level} (PF1 Advanced Class Guide, \
                 3rd-level Skald class feature, one additional rage power every 3 levels \
                 thereafter): {pool_size} rage power(s) known so far. This grounds only the \
                 flat pool-size magnitude -- not which individual rage powers are known, legal, \
                 or applied to Raging Song, since no rage-power-selection or execution \
                 mechanism exists for Skald anywhere in this codebase"
            ),
        });
    }
}

/// Song of Strength's bonus on allies' Strength checks and
/// Strength-based skill checks: `SkaldLVL/2`.
///
/// **Sourced from DESC prose**, not a `BONUS:` token -- the record
/// (`acg_abilities_class.lst:1741`) carries no numeric token at all.
/// The rule reads "allies within 60 feet who can hear the skald may add
/// 1/2 the skald's level to a Strength check or Strength-based skill
/// check".
pub(super) fn skald_song_of_strength_bonus(level: u8) -> i16 {
    i16::from(level) / 2
}

/// Cantrips known, read from the level-0 column of Skald's own
/// `KNOWN:` table in `acg_classes.lst:301-320`: `KNOWN:4,2` at 1st,
/// `KNOWN:5,3` at 2nd, `KNOWN:6,4` at 3rd, and 6 from there on.
///
/// The paired `CAST:` column is `0` at every level, which in PCGen means
/// *unlimited* rather than *none* -- cantrips "do not consume any slots
/// and may be used again". Reading that 0 as a per-day count would
/// report a Skald who can cast no cantrips at all.
pub(super) fn skald_cantrips_known(level: u8) -> i16 {
    match level {
        1 => 4,
        2 => 5,
        _ => 6,
    }
}

/// Grant levels for Skald's remaining named features, per
/// `acg_classes.lst:283-299`.
pub(super) const SKALD_SONG_OF_MARCHING_LEVEL: u8 = 3;

pub(super) const SKALD_UNCANNY_DODGE_LEVEL: u8 = 4;

pub(super) const SKALD_SONG_OF_STRENGTH_LEVEL: u8 = 6;

pub(super) const SKALD_IMPROVED_UNCANNY_DODGE_LEVEL: u8 = 8;

pub(super) const SKALD_DIRGE_OF_DOOM_LEVEL: u8 = 10;

pub(super) const SKALD_SONG_OF_THE_FALLEN_LEVEL: u8 = 14;

pub(super) const SKALD_MASTER_SKALD_LEVEL: u8 = 20;

/// The Skald features whose corpus records carry **no numeric token of
/// any kind**, as `(grant level, display name, corpus DESC excerpt)`.
///
/// Each verified field by field on its own `KEY:Skald ~ <Name>` record.
/// Every one is `KEY` + `CATEGORY` + `TYPE` + `DESC` + `SOURCEPAGE`,
/// with no `BONUS` and no `DEFINE`.
///
/// **Namespace matters here more than anywhere else in this file.**
/// Skald's Uncanny Dodge and Improved Uncanny Dodge are genuinely
/// Skald-owned records (`KEY:Skald ~ Uncanny Dodge`), NOT Barbarian's or
/// Rogue's -- a bare name grep hits those instead and would wrongly
/// suggest the feature is already covered. The same trap applies to
/// Skald's Lore Master and Versatile Performance, which collide by name
/// with Bard's but carry their own Skald-prefixed keys and their own
/// `Skald*` variables; both are grounded separately with real
/// magnitudes.
///
/// Song of Strength is deliberately excluded: it is not zero-magnitude
/// (its DESC carries a real `1/2 level` bonus) and is grounded with a
/// real value.
pub(super) const SKALD_ZERO_MAGNITUDE_FEATURES: &[(u8, &str, &str)] = &[
    (
        SKALD_SONG_OF_MARCHING_LEVEL,
        "Song of Marching",
        "By expending 1 round of raging song, the skald invigorates allies within 60 feet, who \
         may hustle for the next hour; this movement counts as a walk (not a hustle) for the \
         purpose of accruing nonlethal damage and fatigue",
    ),
    (
        SKALD_UNCANNY_DODGE_LEVEL,
        "Uncanny Dodge",
        "He cannot be caught flat-footed, nor does he lose his Dex bonus to AC if the attacker is \
         invisible. He still loses his Dexterity bonus to AC if he is immobilized ... If a skald \
         already has uncanny dodge from a different class, he automatically gains improved \
         uncanny dodge instead",
    ),
    (
        SKALD_IMPROVED_UNCANNY_DODGE_LEVEL,
        "Improved Uncanny Dodge",
        "a skald can no longer be flanked. This defense denies enemies the ability to sneak \
         attack the skald by flanking him, unless the attacker has at least four more levels in a \
         class that grants sneak attack than the target has skald levels",
    ),
    (
        SKALD_DIRGE_OF_DOOM_LEVEL,
        "Dirge of Doom",
        "a skald can create a sense of growing dread in his enemies, causing them to become \
         shaken. This only affects enemies that are within 30 feet and able to hear the skald's \
         performance ... This cannot cause a creature to become frightened or panicked",
    ),
    (
        SKALD_SONG_OF_THE_FALLEN_LEVEL,
        "Song of the Fallen",
        "The skald selects a dead ally within 60 feet and expends 1 round of raging song to bring \
         that ally back to life. The revived ally is alive but staggered ... The ally \
         automatically dies if the skald ends this performance or is interrupted",
    ),
    (
        SKALD_MASTER_SKALD_LEVEL,
        "Master Skald",
        "a skald's inspired rage no longer gives allies a penalty to AC, nor limits what skills \
         or abilities they can use ... when making a full attack, affected allies may make an \
         additional attack each round (as if using a haste effect)",
    ),
];

/// Grounds Skald's remaining named class features (task #91): Raging
/// Song's own rounds-per-day pool, Cantrips known, the Scribe Scroll
/// bonus-feat grant, Song of Strength's real magnitude, and the six
/// zero-magnitude features in `SKALD_ZERO_MAGNITUDE_FEATURES`.
///
/// Together with the magnitudes already grounded elsewhere (Inspired
/// Rage, Damage Reduction, Bardic Knowledge, Well-Versed, Spell Kenning,
/// Lore Master, Versatile Performance, the Rage Powers pool, and Raging
/// Climber/Raging Swimmer), this closes every named feature on Skald's
/// corpus class table.
pub(super) fn ground_skald_remaining_named_features(
    level: u8,
    charisma_modifier: i16,
    selected_feats: &[String],
    explanations: &mut Vec<ComputationExplanation>,
) {
    // Reuses `skald_inspired_rage_rounds_per_day` rather than deriving
    // its own copy: in PF1 every raging song spends from ONE shared
    // per-day pool, so a second formula here would not merely duplicate
    // code, it would assert a second pool that does not exist. That
    // function also already carries this file's ratified ruling on the
    // corpus's defective `3+CHA+(2*SkaldLVL)` token -- see its doc
    // comment. An earlier draft of this closure added a parallel
    // `skald_raging_song_rounds_per_day` before finding it.
    let rounds = skald_inspired_rage_rounds_per_day(charisma_modifier, level, selected_feats);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.skald.raging_song_rounds_per_day".to_owned(),
        value: rounds,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|SkaldRagingSongRoundsPerDay|6, and re-impose a widened prerequisite
            //   accepting either pool
            "Skald level {level} Raging Song: {rounds} rounds per day (3 + Charisma modifier \
             {charisma_modifier:+} + 2 per level after the first + Extra Performance feat ({:+}) -- \
             that Core feat looks Bard-only in cr_feats.lst, but acg_feats.lst's own \
             `CATEGORY=FEAT|Extra Performance.MOD` records clear its Bardic-Performance-only \
             prerequisite, add). Raging Song is the umbrella performance that Inspired Rage, Song of \
             Marching, Song of Strength, Dirge of Doom and Song of the Fallen all spend from -- ONE \
             shared pool, which is why this reads the same `skald_inspired_rage_rounds_per_day` the \
             rage-execution budget enforces against rather than deriving a second figure. Starting a \
             song is a standard action, a move action at 7th and a swift action at 13th; this \
             codebase models no action economy for that, and the 20%%-failure-chance-while-deaf \
             clause is a resolution, not a magnitude. The corpus's own magnitude for this \
             record reads 3+CHA+(2*SkaldLVL), two higher than its own rule text at every level; that \
             defect was already identified and knowingly overridden in favour of the rule text -- \
             see `skald_inspired_rage_rounds_per_day`'s doc comment for the standing ruling",
            extra_resource_feat_bonus(
                selected_feats,
                EXTRA_PERFORMANCE_FEAT_KEY,
                EXTRA_ROUNDS_PER_DAY
            )
        ),
    });

    let cantrips = skald_cantrips_known(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.skald.cantrips_known".to_owned(),
        value: cantrips,
        detail: format!(
            "Skald level {level} Cantrips: {cantrips} 0-level spells known, read from the \
             level-0 column of Skald's own KNOWN: table (4 at 1st, 5 at 2nd, 6 from 3rd on). \
             They are cast at will: the paired CAST: column is 0 at every level, which in PCGen \
             means UNLIMITED rather than none -- reading that 0 as a per-day count would report a \
             Skald who can cast no cantrips at all. Grounds the known-count only; the separate \
             spontaneous spell-level-access, per-day and save-DC records cover levels 1-4"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.skald.scribe_scroll_bonus_feat".to_owned(),
        value: 1,
        detail: format!(
            "Skald level {level} Scribe Scroll (granted at 1st level, corpus \
             KEY:Skald ~ Scribe Scroll): 1 bonus feat, granted automatically rather than chosen \
             -- the record's sole functional token is ABILITY:FEAT|AUTOMATIC|Scribe Scroll, with \
             no BONUS or DEFINE. Grounded as a count-of-1 grant record, the same idiom as \
             Wizard's own Scribe Scroll grant. This codebase resolves no item-creation, so the \
             feat's benefit is not computed; what grounds is that the slot is filled by a known \
             feat rather than left to a chooser"
        ),
    });

    if level >= SKALD_SONG_OF_STRENGTH_LEVEL {
        let strength_bonus = skald_song_of_strength_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.skald.song_of_strength_bonus".to_owned(),
            value: strength_bonus,
            detail: format!(
                "Skald level {level} Song of Strength (granted at level \
                 {SKALD_SONG_OF_STRENGTH_LEVEL}): allies within 60 feet who can hear the skald \
                 may add +{strength_bonus} (half the skald's level) to a Strength check or \
                 Strength-based skill check, once each round. The magnitude is transcribed from \
                 the record's DESC prose, which is the only place it exists -- the record carries \
                 no BONUS or DEFINE token. Grounds the magnitude only: the bonus lands on ALLIES, \
                 and this codebase computes no ally's skill totals, so nothing consumes it"
            ),
        });
    }

    for (grant_level, display_name, description) in SKALD_ZERO_MAGNITUDE_FEATURES {
        if level < *grant_level {
            continue;
        }
        explanations.push(ComputationExplanation {
            id: format!(
                "class_feature.acg.skald.{}_grant",
                class_feature_id_slug(display_name)
            ),
            value: 0,
            detail: format!(
                "Skald level {level} {display_name}, granted at level {grant_level} (corpus \
                 KEY:Skald ~ {display_name}): \"{description}\" This is a bounded grant-only \
                 identity record (value 0, non-fabricated): the record carries no BONUS and no \
                 DEFINE -- verified field by field -- so it has no magnitude to compute, now or \
                 ever. The key namespace is load-bearing: this is Skald's OWN record, not \
                 Barbarian's or Rogue's or Bard's same-named one, which a bare name grep would \
                 have matched instead"
            ),
        });
    }
}

/// Pushes the new, narrower diagnostic replacing
/// `class_feature.acg.skald.unsupported` for Skald specifically (per the
/// adversarial review's finding 2, updated for the spellcasting closure):
/// named ONLY the genuinely still-missing pieces (Skald's own remaining
/// named features beyond Inspired Rage and known-spell posture), unlike
/// the retired diagnostic's blanket "no named class-feature computation...
/// grounded anywhere" claim, which is now false for Skald.
///
/// **Updated (v0.6 alpha swarm, risks item 8, Skald spellcasting
/// closure)**: this diagnostic no longer claims spellcasting is
/// ungrounded -- known-spell posture is now genuinely validated by
/// `ground_or_block_skald_spellcasting`, reusing Bard's own spell list
/// and progression tables (verified identical to Skald's own). Unlike
/// Bard, whose own remaining named features were already grounded in an
/// earlier SD13-E5 cycle (so Bard reaches full `Computed` once its own
/// known-spell/performance postures are valid), Skald's OTHER named
/// features remain completely unbuilt, so this diagnostic still
/// claim-blocks unconditionally -- Skald does not reach `Computed` this
/// closure either, confirmed directly rather than assumed by analogy to
/// Bard.
///
/// **Updated again (deepening, 2026-07-26, task #7)**: Damage Reduction's
/// self-only half and Bardic Knowledge are also no longer named as
/// missing here -- both now genuinely wired (see
/// `ground_skald_damage_reduction`/`ground_skald_bardic_knowledge`).
///
/// **Updated again (task #50)**: Well-Versed, and the flat pool-size/
/// count magnitudes for Spell Kenning, Lore Master, Versatile
/// Performance, and Rage Powers are also no longer named as flatly
/// missing here -- all five are now genuinely wired (see
/// `ground_skald_well_versed`/`ground_skald_spell_kenning`/
/// `ground_skald_lore_master`/`ground_skald_versatile_performance`/
/// `ground_skald_rage_powers_pool_size`). Well-Versed is fully covered
/// (a flat, never-applied save bonus needs no further execution); the
/// other four still have a genuinely missing execution half named
/// explicitly (Spell Kenning's cross-class spell-borrowing, Lore
/// Master's take-10/20 mechanic, Versatile Performance's Perform-type
/// choice and skill-substitution, Rage Powers' individual-power
/// selection and Raging Song application) -- the same "grounds the
/// magnitude, names the still-missing mechanism" shape as Damage
/// Reduction's own ally-extension above.
///
/// Pushed exactly once from the top-level Skald dispatch branch,
/// independent of any of these features' own state.
///
/// **Updated again (task #54, 2026-07-28)**: Raging Climber's and Raging
/// Swimmer's own self-use magnitude (while singing, landing on the real
/// Climb/Swim totals) are also no longer named as missing here -- see
/// `ground_raging_climber_and_swimmer`. At the time task #54 landed, the
/// Rage Powers pool-count formula (`RagePowersLVL/3`) and the
/// ally-granting "shared-list access" stayed fully deferred -- only
/// these two canonical-narrowing representatives' own magnitude was
/// real, out of the wider 60-record Rage Powers family.
///
/// **Reconciled with task #50 (rebased on top of #54, same day)**: the
/// pool-count formula itself is now ALSO grounded (see
/// `ground_skald_rage_powers_pool_size`), so it is folded into task
/// #50's own "Rage Powers" acknowledgment below rather than left in
/// #54's "still deferred" framing. What remains genuinely missing for
/// Rage Powers is narrower than either task alone described: the
/// ally-granting "shared-list access" (applying chosen rage powers to
/// allies via Raging Song) and the other 58 named-but-unmodeled rage
/// powers beyond the pool-size count and Raging Climber/Swimmer's own
/// two magnitudes.
pub(super) fn push_skald_other_features_deferred_diagnostic(diagnostics: &mut Vec<ComputationDiagnostic>) {
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.skald.other_features_deferred.unsupported".to_owned(),
        message: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   (3) One real corpus disagreement is surfaced rather than hidden: Raging Song's
            //   BONUS:VAR token reads 3+CHA+(2*SkaldLVL), two higher at every level than the rule
            //   text on the same record, which increments only for levels after the first.
            "{SKALD_CLASS_ID} now grounds every named feature on its corpus class table: the \
             base-attack-bonus/base-save chassis pillar, its class-skill list, Inspired Rage, \
             known-spell posture and spontaneous spellcasting, self-only Damage Reduction, Bardic \
             Knowledge, Well-Versed, the flat magnitudes for Spell Kenning, Lore Master, Versatile \
             Performance and the Rage Powers pool, Raging Climber and Raging Swimmer (landing on the \
             real Climb/Swim totals), and -- newly, task #91 -- Raging Song's own rounds-per-day \
             pool, Cantrips known, the Scribe Scroll bonus-feat grant, Song of Strength's half-level \
             bonus, and Song of Marching, Uncanny Dodge, Improved Uncanny Dodge, Dirge of Doom, Song \
             of the Fallen and Master Skald as bounded grant-only identity records. This diagnostic \
             is therefore no longer claim-blocking; it is retained to carry the honest remainder, \
             which is substantial and worth reading. (1) EXECUTION, not magnitude, is what stays \
             deferred: Damage Reduction's ally-extension, Spell Kenning's cross-class \
             spell-borrowing, Lore Master's take-10/take-20, Versatile Performance's Perform-type \
             choice and skill substitution, and Rage Powers' ally-granting shared-list access all \
             have their counts or amounts grounded while the mechanism that would spend them does \
             not exist here. Several also land on ALLIES -- Song of Strength, Song of Marching, Song \
             of the Fallen, Dirge of Doom and Master Skald all modify other creatures -- and this \
             codebase computes no ally's totals, so those magnitudes are derived correctly and \
             consumed by nothing. Under this repo's standalone-fact grounding bar a missing consumer \
             does not block a correctly-derived number. (2) The Rage Powers family is covered at 2 \
             of its 60 loaded records (Raging Climber, Raging Swimmer), a catalog gap narrowed the \
             same way Oracle's Mystery is, not a missing magnitude. Skald draws from the SAME shared \
             pool and the SAME record set as Barbarian -- there is no Skald-specific rage power list \
             -- so the two grounded powers work for Skald only because their value variable is \
             RagePowersLVL, which Skald sets to SkaldLVL. The rule text is implemented; see that \
             record's own explanation. This message previously asserted \"this ACG class has no \
             class-skill list\": Skald's own `KEY:Skald ~ Class Skills` record exists and genuinely \
             includes Climb, Intimidate and Swim (task #78). Master Skald, Uncanny Dodge, Improved \
             Uncanny Dodge, Cantrips and Scribe Scroll were named in NEITHER clause (task #76) and \
             are now grounded"
        ),
        claim_blocking: false,
    });
}

/// Skald's base spells-per-day table (v0.6 alpha swarm, risks item 8,
/// Skald spellcasting closure), verified byte-identical to Bard's own
/// inline `bard_base_spells_per_day` table (`explain_bard_level1_spell_baseline`)
/// against two independent primary sources (aonprd.com and d20pfsrd.com,
/// agreeing with each other). Duplicated here as a small, separate,
/// Skald-named table rather than extracting Bard's own inline table into
/// a shared function -- Bard's table lives inline inside an already-
/// shipped, already-tested function, and extracting it would be a
/// refactor touching that function for zero necessary benefit (the table
/// is only 10 short match arms), the same "parallel copy over refactor-
/// risk" discipline the Wolf/Horse companion functions established.
pub(super) fn skald_base_spells_per_day_table(level: u8) -> [Option<i16>; 4] {
    match level {
        1 => [Some(1), None, None, None],
        2 => [Some(2), None, None, None],
        3 => [Some(3), None, None, None],
        4 => [Some(3), Some(1), None, None],
        5 => [Some(4), Some(2), None, None],
        6 => [Some(4), Some(3), None, None],
        7 => [Some(4), Some(3), Some(1), None],
        8 => [Some(4), Some(4), Some(2), None],
        9 => [Some(5), Some(4), Some(3), None],
        10 => [Some(5), Some(4), Some(3), Some(1)],
        _ => [None, None, None, None],
    }
}

/// Skald's spell-level access ladder and base spells-known/spells-per-day
/// tables are byte-identical to Bard's own (v0.6 alpha swarm, risks item
/// 8, Skald spellcasting closure), verified against two independent
/// primary sources (aonprd.com and d20pfsrd.com, agreeing with each
/// other) before writing this function -- not assumed from the shared
/// `SPELLLIST:1|Bard` corpus token alone. `bard_spell_level_access` and
/// `bard_spells_known_table` are pure, side-effect-free lookups (they
/// push no explanations/diagnostics themselves), so calling them
/// directly here introduces zero risk to Bard's own behavior, unlike the
/// Wolf/Horse companion functions (which push explanation records and
/// were kept as deliberately separate parallel copies for that reason).
/// `bard_spell_list::BARD_SPELL_LIST`/`bard_spell_list::bard_spell_level`
/// are reused directly for the same reason: Skald's `SPELLLIST:1|Bard`
/// confirms this is the same list, not merely a similar one.
pub(super) fn unmet_skald_known_spell_conditions(input: &CharacterInput, skald_level: u8) -> Vec<String> {
    let mut unmet = Vec::new();

    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| s.source_class_id == SKALD_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();

    let access_ceiling = bard_spell_level_access(skald_level);
    let known_table = bard_spells_known_table(skald_level);

    let mut known_per_level: [i16; 5] = [0; 5];
    for spell_id in &known {
        let Some(spell_level) = bard_spell_list::bard_spell_level(spell_id) else {
            unmet.push(format!(
                "known spell '{spell_id}' is not on the real PF1 Bard spell list Skald casts \
                 from"
            ));
            continue;
        };
        if spell_level > 0 && i16::from(spell_level) > access_ceiling {
            unmet.push(format!(
                "known spell '{spell_id}' targets spell level {spell_level}, not yet accessible \
                 at skald level {skald_level} (access ceiling {access_ceiling})"
            ));
            continue;
        }
        if usize::from(spell_level) >= known_per_level.len() {
            unmet.push(format!(
                "known spell '{spell_id}' targets spell level {spell_level}, beyond this bounded \
                 seam's verified spells-known table (spell levels 0-4 only)"
            ));
            continue;
        }
        known_per_level[usize::from(spell_level)] += 1;
    }

    for (index, count) in known_per_level.iter().enumerate() {
        if *count == 0 {
            continue;
        }
        let cap = known_table[index].unwrap_or(0);
        if *count > cap {
            unmet.push(format!(
                "spell level {index} over-known: {count} distinct spells known but only {cap} \
                 slots available on the Skald Spells Known table"
            ));
        }
    }

    unmet
}

/// Grounds the real known-spell posture once
/// `unmet_skald_known_spell_conditions` reports an empty unmet list,
/// mirroring `ground_bard_known_spells` exactly.
pub(super) fn ground_skald_known_spells(
    input: &CharacterInput,
    skald_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| s.source_class_id == SKALD_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.skald.known_spells".to_owned(),
        value: known.len() as i16,
        detail: format!(
            "Skald level {skald_level} known-spell selection ({} spells, AcquisitionMode::Known): \
             {}. Each known spell is verified against the real PF1 Bard spell list Skald casts \
             from (`bard_spell_list::BARD_SPELL_LIST` -- Skald's own `SPELLLIST:1|Bard` corpus \
             token confirms this is the same list, not merely a similar one), the shared \
             spell-level access ceiling (`bard_spell_level_access`), and the shared Spells Known \
             table's own per-level cap (`bard_spells_known_table` -- verified identical to \
             Skald's own table against two independent primary sources). Real PF1 Skald rules \
             have no daily preparation step at all (spontaneous, like Bard) -- a skald's known \
             spells are permanent once learned, cast spontaneously using the already-grounded \
             per-day slot totals. This grounds the known-spell selection for real; it computes \
             no spell save DC resolution against a target and no casting execution",
            known.len(),
            known.join(", ")
        ),
    });
}

/// Grounds or claim-blocks Skald's known-spell posture (v0.6 alpha swarm,
/// risks item 8, Skald spellcasting closure), mirroring
/// `ground_or_block_bard_known_spells` exactly. Called from
/// `compute_acg_class_chassis`'s Skald branch, independent of Inspired
/// Rage's own state -- a Skald can be validly not-singing with a valid
/// known-spell posture, validly singing with an invalid posture, or any
/// other combination, since the two closures are unrelated PF1
/// mechanics.
pub(super) fn ground_or_block_skald_spellcasting(
    input: &CharacterInput,
    skald_level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // Flat records grounded unconditionally, mirroring
    // `explain_bard_level1_spell_baseline`'s own shape exactly: the
    // spell-level access ladder, the base spells-per-day counts, and the
    // base spell-save-DC arithmetic are all real regardless of the
    // known-spell posture's own validity (a Skald with an invalid
    // known-spell posture still has a real access ladder and per-day
    // budget; the posture violation is about WHICH spells are known, not
    // whether the ladder/budget exist at all).
    let access_ceiling = bard_spell_level_access(skald_level);
    explanations.push(ComputationExplanation {
        id: "class_chassis.skald.spontaneous.spell_level_access".to_owned(),
        value: access_ceiling,
        detail: format!(
            "Skald spell-level access at skald level {skald_level}: {access_ceiling} (verified \
             byte-identical to Bard's own access ladder against two independent primary \
             sources, aonprd.com and d20pfsrd.com -- Skald casts from the Bard spell list per \
             its own SPELLLIST:1|Bard corpus token). This grounds the access ladder only: no \
             spells-per-day counts, no spells-known posture, no bonus slots from a high \
             Charisma, and no spell save DCs are computed by this record"
        ),
    });

    let base_spells_per_day = skald_base_spells_per_day_table(skald_level);
    for (index, base_count) in base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = index + 1;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.skald.spontaneous.base_spells_per_day.spell_level_{spell_level}"
            ),
            value: *base_count,
            detail: format!(
                "Skald base spells per day at skald level {skald_level}, spell level \
                 {spell_level}: {base_count}, verified byte-identical to Bard's own base \
                 spells-per-day table against two independent primary sources. This grounds the \
                 base count only: bonus spells per day from a high Charisma are never computed, \
                 spells KNOWN (a separate table) is not grounded, and no spell save DCs are \
                 computed"
            ),
        });
    }

    for spell_level in 1..=access_ceiling {
        let charisma_modifier = ability_modifier_for(ability_modifiers, "charisma");
        let spell_save_dc = 10 + spell_level + charisma_modifier;
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.skald.spontaneous.spell_save_dc.spell_level_{spell_level}"),
            value: spell_save_dc,
            detail: format!(
                "Skald spell save DC at skald level {skald_level}, spell level {spell_level}: \
                 10 + {spell_level} + Charisma modifier {charisma_modifier} = {spell_save_dc} \
                 (the universal PF1 spell-save-DC formula, identical in shape to Bard's own --\
                 Skald's SPELLSTAT is Charisma too). This grounds the base DC formula only: no \
                 saving-throw resolution, no target, no spell selection, and no feat DC \
                 modifiers are computed"
            ),
        });
    }

    let unmet = unmet_skald_known_spell_conditions(input, skald_level);
    if unmet.is_empty() {
        ground_skald_known_spells(input, skald_level, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.skald.spontaneous_known_and_per_day.unsupported".to_owned(),
            message: format!(
                "Skald remains blocked on its spontaneous known-spell / slot posture burden: \
                 spontaneous casting execution (slot consumption, tracking, and casting itself) \
                 is out of scope for this bounded baseline regardless (the spell-level access \
                 ladder, the base spells per day table counts, and the base spell-save-DC \
                 arithmetic are grounded separately as flat records above); unmet known-spell \
                 posture: {}",
                unmet.join("; ")
            ),
            claim_blocking: true,
        });
    }
}

/// Applies Skald Inspired Rage's Strength/Constitution morale bonus to
/// `base` when a valid, active, in-budget Inspired Rage activation is
/// present for this character (v0.6 alpha swarm, risks item 8, first
/// APG/ACG closure). Mirrors `apply_rage_ability_bonuses` exactly --
/// chained immediately after it in `compute_pilot_base_chassis` so a
/// (structurally impossible, but defensively independent) character
/// matching both gates would still see both bonuses layered correctly;
/// class-ownership-gated by construction via `active_skald_inspired_rage_bonus`.
///
/// The Strength/Constitution ability-SCORE bonus (+2/+4/+6, always even)
/// becomes exactly half that as an ability-MODIFIER bonus (+1/+2/+3):
/// modifier = floor((score - 10) / 2), and adding an even N to score adds
/// exactly N/2 to the floored modifier regardless of the original score's
/// parity (identical reasoning to `apply_rage_ability_bonuses`).
pub(super) fn apply_skald_inspired_rage_ability_bonuses(
    base: AbilityModifiers,
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) -> AbilityModifiers {
    let Some((skald_level, strength_constitution_bonus, will_save_bonus)) =
        active_skald_inspired_rage_bonus(input, &base)
    else {
        return base;
    };

    let ability_modifier_bonus = strength_constitution_bonus / 2;
    let boosted = AbilityModifiers {
        strength: base.strength + ability_modifier_bonus,
        constitution: base.constitution + ability_modifier_bonus,
        ..base
    };
    explanations.push(ComputationExplanation {
        id: "ability_modifier.skald.inspired_rage_bonus_applied".to_owned(),
        value: ability_modifier_bonus,
        detail: format!(
            "Skald level {skald_level} Inspired Rage applied to ability modifiers: \
             +{strength_constitution_bonus} Strength / +{strength_constitution_bonus} \
             Constitution morale bonus (ability score) is +{ability_modifier_bonus} Strength \
             modifier / +{ability_modifier_bonus} Constitution modifier (an even score bonus \
             always halves exactly onto the floored modifier). Applied only while actively, \
             validly singing; the Will-save bonus (+{will_save_bonus}) is layered onto \
             compute_total_saves separately, and the \
             {SKALD_INSPIRED_RAGE_ARMOR_CLASS_PENALTY} Armor Class penalty onto \
             compute_combat_baseline separately"
        ),
    });
    boosted
}

/// The bounded Bard milestone level this decomposition surface grounds, if any.
/// Returns the single Bard level when the chosen input is exactly a single-class
/// Bard at one of the supported milestone levels (1 through `MAX_SUPPORTED_BARD_LEVEL`,
/// currently 19). Returns `None` for no Bard, a non-Bard class, a multiclass mix, or
/// any level-20 Bard this slice deliberately does not recognize — each of which stays
/// claim-blocked exactly as before. Mirrors the Fighter `supported_fighter_level` /
/// Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` level-range gate idiom.
pub(super) fn supported_bard_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == BARD_CLASS_ID
                && (1..=MAX_SUPPORTED_BARD_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E4-F7/SD13-E4/SD13-E5 runtime evidence for the deterministic
/// Human Bard level-1/level-2/level-3 spontaneous arcane spell-bearing baseline: one
/// recognition record, the foundational base-attack-bonus / base-save progression
/// pillar (four standalone records), five grounded chassis-class-feature pillars
/// (Bardic Knowledge, the Bardic Performance rounds-per-day budget, the Inspire
/// Courage flat magnitude, and the Fascinate flat Will-save DC and
/// affected-creature-count formulas), a sixth pillar grounded only at level 2
/// (Well-Versed's flat +4 save-bonus magnitude), a seventh pillar grounded only at
/// level 3 (Inspire Competence's flat +2 skill-check magnitude), and two remaining
/// named claim-blocking burdens (the bardic performance-execution engine, the
/// spontaneous spell posture).
///
/// This deliberately does not compute a supported Bard chassis. It grounds no
/// bardic performance execution — no start/maintain action economy, no round
/// tracking or consumption, and no Will-save/targeting resolution for Fascinate,
/// nor anything at all for Countersong, Distraction, or Versatile Performance
/// (all three require either an opposed Perform-check-vs-effect substitution
/// resolution or a choice-gated skill-substitution engine, not a flat number) —
/// and no spell math whatsoever: no spells known, no spells per day, no spell
/// DCs, no bonus spells, no prepared posture, no school choice. It only:
/// - leaves one recognition explanation so the `class:bard:N` identity is acknowledged
///   as a spontaneous arcane spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value),
/// - grounds the foundational base-attack-bonus / base-save progression pillar that
///   every other class row in this matrix (Fighter, Barbarian, Monk, Rogue, Paladin,
///   Druid, Cleric) already has and Bard never had: base attack bonus (3/4 BAB,
///   `classlevel * 3 / 4`, the same formula shape as Rogue/Monk/Druid/Cleric) and
///   base save progression (good Reflex, good Will, poor Fortitude — the same save
///   shape as Rogue, confirmed independently against the raw PF1 Core Rulebook Bard
///   class table rather than assumed from Rogue's own pattern). Both are grounded as
///   flat, standalone `ComputationExplanation` records, mirroring the exact
///   "standalone, not wired into the integrated `PilotBaseChassisComputation`" idiom
///   already used for every other class's own base-attack/base-save grounding: neither
///   is wired into `base_attack_bonus`, `compute_total_saves`, or
///   `compute_combat_baseline`,
/// - grounds the Bardic Knowledge chassis-class-feature pillar for real: PF1 Core
///   Rulebook Bardic Knowledge is a flat competence bonus on Knowledge checks equal
///   to half the bard's level (minimum 1), also letting the bard make any Knowledge
///   check untrained. That flat bonus needs no skill-rank state and no ability
///   modifier (the Intelligence modifier already belongs to the ordinary Knowledge
///   check, not to this class-feature bonus), so it is a bounded, deterministic,
///   level-only value; this grounds only that flat bonus, not a full Knowledge-check
///   resolution,
/// - grounds the flat Bardic Performance surface for real: the rounds-per-day
///   budget (PF1 Core Rulebook Bardic Performance: 4 + Charisma modifier rounds
///   per day at level 1, plus 2 additional rounds per day at each level after
///   1st — verified against d20pfsrd and legacy.aonprd.com rather than assumed
///   from Barbarian's superficially similar Rage-rounds progression, floored at
///   0) and the Inspire Courage flat magnitude (+1 competence bonus on attack
///   and weapon damage rolls, +1 morale bonus on saving throws against charm and
///   fear effects — confirmed unchanged through level 2, since the PF1 Core
///   Rulebook bonus first increases only at bard level 5). These are bounded
///   flat values only; no performance-state engine applies them anywhere,
/// - grounds the Fascinate flat Will-save DC (10 + 1/2 bard level + Charisma
///   modifier) and the Fascinate flat affected-creature count (1 at 1st level,
///   plus one more for every three bard levels beyond 1st) for real, verified
///   against the PF1 Core Rulebook Fascinate rule text rather than assumed from
///   memory. Both are bounded flat values only; neither is ever applied to an
///   actual Will save or targeting outcome,
/// - grounds Well-Versed (SD13-E5, a 2nd-level Bard class feature verified
///   independently against two primary PF1 sources — d20pfsrd and
///   legacy.aonprd.com both list "Versatile performance, well-versed" as the
///   Bard 2nd-level special feature entry) as a flat, non-level-scaled +4
///   standalone magnitude on saving throws against bardic performance, sonic,
///   and language-dependent effects, mirroring the Fighter Bravery idiom: never
///   applied to any actual save total, since no save-resolution engine exists
///   in this codebase. Versatile Performance (the Bard's OTHER 2nd-level
///   feature) is NOT flat — it requires a choice of Perform type and an actual
///   skill-substitution engine — so it is deliberately left named-but-unproven,
///   mirroring how the Monk level-2 bonus feat grant was deliberately left
///   unrecognized by the Monk level-2 widening slice,
/// - grounds Inspire Competence (SD13-E5, a 3rd-level Bard class feature verified
///   independently against two primary PF1 sources — d20pfsrd and
///   legacy.aonprd.com both list "Inspire competence +2" as the Bard 3rd-level
///   special feature entry) as a flat +2 standalone magnitude (a competence bonus
///   on skill checks with a particular skill), mirroring the Fighter Bravery /
///   Rogue Trap Sense / Barbarian Trap Sense / Monk Still Mind idiom: never
///   applied to any actual skill-check total, since no skill-check-resolution
///   engine exists in this codebase, and no task-selection/action-economy engine
///   decides which skill or ally it targets, and
/// - emits two distinct claim-blocking diagnostics naming the still-unproven bardic
///   performance-execution burden (start/maintain action economy, round tracking and
///   consumption, no application of any grounded magnitude/DC/count to an actual
///   total, and the fully-ungrounded Countersong / Distraction performances) and the
///   spontaneous known-spell / slot posture burden explicitly, rather than hiding
///   behind a generic "unsupported caster" label.
///
/// A later SD13-E5 slice widens the level-range gate to level 4, extending every
/// formula above (base attack, base saves, Bardic Knowledge, Bardic Performance
/// rounds/day, Fascinate DC/count) via the same level-valued formulas, and keeping
/// Well-Versed and Inspire Competence granted, without re-deriving any of them.
/// Verified independently against the PF1 Core Rulebook Bard class table
/// (d20pfsrd and legacy.aonprd.com): the level-4 "Special" column is BLANK, so no
/// new pillar is grounded at level 4.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Bard spell-bearing identity, the grounded
/// flat pillars, and the two remaining named burdens legible on the runtime path.
pub(super) fn explain_bard_level1_spell_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // v0.6 alpha swarm, risks item 8: validated regardless of whether Bard
    // appears alone or in a multiclass mix, and regardless of race --
    // checked BEFORE the single-class-only/Human gate below, mirroring the
    // Ranger/Paladin/Sorcerer/Cleric/Druid/Barbarian fix exactly (a
    // false-Computed/false-grounding risk now that `table_class_id`
    // recognizes Bard generically too). The known-spell / per-day posture
    // burden was found NOT yet hoisted here when this slice started --
    // the exact same gate-ordering bug every other class needed fixed
    // before its own spell-posture became conditional, caught and fixed
    // proactively as part of closing it, not left for a future review to
    // find.
    if let Some(bard_level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == BARD_CLASS_ID)
        .map(|class_level| class_level.level)
    {
        ground_or_block_bard_bardic_performance(
            input,
            bard_level,
            ability_modifiers,
            explanations,
            diagnostics,
        );
        ground_or_block_bard_known_spells(input, bard_level, explanations, diagnostics);
    }

    let Some(level) = supported_bard_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Bard spell-bearing
    // identity at the supported level. This is a recognition record only; it
    // fabricates no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.bard".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Bard level {level} spell-bearing baseline: the \
             {BARD_CLASS_ID}:{level} class identity is acknowledged as a spontaneous arcane \
             spell-bearing class on the rules-core seam rather than an undocumented packet placeholder. \
             This is a bounded recognition record only; it fabricates no spell \
             math (spells known, spells per day, spell DCs, bonus spells, or prepared posture), \
             so it carries no fabricated mechanical value (+0). Inspire Courage's own \
             bardic-performance-execution engine (start/maintain, round-budget validation, \
             attack-bonus application) is real now -- see \
             ground_or_block_bard_bardic_performance -- while countersong / distraction / \
             fascinate resolution and the other bardic performances remain unmodeled"
        ),
    });

    // Grounded: the foundational base-attack-bonus / base-save progression pillar.
    // Unlike every other class row in this matrix (Fighter, Barbarian, Monk, Rogue,
    // Paladin, Druid, Cleric all already ground this pillar), Bard had never had it
    // grounded at all until an earlier SD13-E5 slice. Both formulas were verified
    // against the PF1 Core Rulebook Bard class table (d20pfsrd and the legacy
    // Paizo PRD mirror) before writing this code, reading the raw level 1-6 table
    // rows directly (BAB +0/+1/+2/+3/+3/+4, Fort +0/+0/+1/+1/+1/+2, Ref
    // +2/+3/+3/+4/+4/+5, Will +2/+3/+3/+4/+4/+5) rather than trusting memory or
    // assuming Bard's save shape merely because it resembles Rogue's: the level
    // 4/5 BAB values (+3 at both) disambiguate the 3/4-vs-1/2 fraction (level 1
    // alone floors both to +0), and the raw Fort/Ref/Will columns independently
    // confirm good Reflex, good Will, poor Fortitude — the same save shape as
    // Rogue, but checked against Bard's own table rather than assumed from
    // Rogue's. A later SD13-E5 slice widens the level-1-only gate to level 2 and
    // extends every one of the formulas below to level 2 via the same formula,
    // without re-derivation, verified independently against the PF1 Core
    // Rulebook Bard class table: level 2 base attack +1, base saves +0/+3/+3
    // (Fortitude/Reflex/Will).
    let level_value = i16::from(level);

    // Grounded (1/2): 3/4-BAB base-attack progression, the same formula shape as
    // Rogue/Monk/Druid/Cleric (classlevel * 3 / 4). No PCGen .lst file exists for
    // the Bard class in this repo, so the formula cites the PF1 Core Rulebook Bard
    // class table directly.
    let base_attack_bonus = level_value * 3 / 4;
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Bard level {level} base attack bonus from the PF1 Core Rulebook Bard class \
             table's 3/4-BAB progression, the same formula shape as Rogue/Monk/Druid/Cleric: \
             classlevel * 3 / 4 = {base_attack_bonus}. This is a standalone explanation record; \
             it is not wired into the integrated base_attack_bonus field or into \
             compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — poor Fortitude, good Reflex, good
    // Will, verified against the PF1 Core Rulebook Bard class table (Fortitude
    // +0, Reflex +2, Will +2 at level 1; Fortitude +0, Reflex +3, Will +3 at
    // level 2).
    let good_save = level_value / 2 + 2;
    let poor_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.base_save.fortitude".to_owned(),
        value: poor_save,
        detail: format!(
            "Bard level {level} base Fortitude save (poor save) from the PF1 Core Rulebook \
             Bard class table: classlevel/3 = {poor_save}. This is a standalone explanation \
             record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.base_save.reflex".to_owned(),
        value: good_save,
        detail: format!(
            "Bard level {level} base Reflex save (good save) from the PF1 Core Rulebook Bard \
             class table: classlevel/2+2 = {good_save}. This is a standalone explanation \
             record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.base_save.will".to_owned(),
        value: good_save,
        detail: format!(
            "Bard level {level} base Will save (good save) from the PF1 Core Rulebook Bard \
             class table: classlevel/2+2 = {good_save}. This is a standalone explanation \
             record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded for real: the Bardic Knowledge pillar. PF1 Core Rulebook Bardic
    // Knowledge: "A bard adds half his bard level (minimum 1) to Knowledge skill
    // checks and may make all Knowledge skill checks untrained." That is a flat
    // competence bonus, not "half level + INT modifier": the Intelligence modifier
    // is already part of the ordinary Knowledge skill check total (rank + ability
    // modifier + misc bonuses), so it is not an additional term this class-feature
    // bonus contributes on its own. Confirmed unchanged at level 2
    // (max(2/2, 1) = 1, the same value as level 1's floor-forced 1, but reached
    // naturally this time rather than via the floor), via the same formula, not a
    // new record.
    let bardic_knowledge_bonus = (level_value / 2).max(1);
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.bardic_knowledge".to_owned(),
        value: bardic_knowledge_bonus,
        detail: format!(
            "Bard Bardic Knowledge class feature: grants a competence bonus on Knowledge \
             skill checks equal to max(bard level / 2, 1) (PF1 Core Rulebook Bardic Knowledge: \
             half bard level, minimum +1), and lets the bard make any Knowledge skill check \
             untrained. At Bard level {level} this bonus is max({level} / 2, 1) = \
             {bardic_knowledge_bonus}. This grounds only the flat Knowledge-check competence \
             bonus; it is not a full Knowledge-check resolution engine and adds no skill rank, \
             no ability modifier, and no untrained-check gate, and it grounds no bardic \
             performance execution"
        ),
    });

    // Grounded for real: the Bardic Performance rounds-per-day budget. PF1 Core
    // Rulebook Bardic Performance: a level-1 bard can use bardic performance for a
    // number of rounds per day equal to 4 + his Charisma modifier. "At each level
    // after 1st a bard can use bardic performance for 2 additional rounds per
    // day" (verified against d20pfsrd and legacy.aonprd.com before widening,
    // rather than assumed to match Barbarian's superficially similar Rage-rounds
    // progression), so the formula widens to
    // 4 + Charisma modifier + 2 * (level - 1), floored at 0 mirroring the Cleric
    // channel-energy uses-per-day floor.
    // Routed through the shared formula rather than recomputed inline
    // (task #19, 2026-07-27): this displayed total and the two
    // activation-budget checks in `active_bard_inspire_courage_attack_bonus`
    // / `ground_or_block_bard_bardic_performance` must not be able to
    // disagree about the Extra Performance feat's 6 extra rounds.
    let bardic_performance_rounds_per_day = bard_bardic_performance_rounds_per_day(
        ability_modifiers.charisma,
        level,
        &input.chosen.selected_feats,
    );
    let extra_performance_bonus = extra_resource_feat_bonus(
        &input.chosen.selected_feats,
        EXTRA_PERFORMANCE_FEAT_KEY,
        EXTRA_ROUNDS_PER_DAY,
    );
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.bardic_performance_rounds_per_day".to_owned(),
        value: bardic_performance_rounds_per_day,
        detail: format!(
            "Bard Bardic Performance rounds per day at bard level {level} (PF1 Core Rulebook \
             Bardic Performance): 4 + Charisma modifier at level 1, plus 2 additional rounds \
             per day at each level after 1st, floored at 0. At Charisma modifier {} this is \
             max(4 + {} + {BARD_PERFORMANCE_ADDITIONAL_ROUNDS_PER_LEVEL} * ({level} - 1), 0) \
             + Extra Performance feat (+{extra_performance_bonus}) = \
             {bardic_performance_rounds_per_day}. This grounds only the flat daily round \
             budget; no round tracking or consumption, no start/maintain action economy, and \
             no per-performance execution is computed",
            ability_modifiers.charisma, ability_modifiers.charisma
        ),
    });

    // Grounded for real: the Inspire Courage flat magnitude. PF1 Core Rulebook
    // Inspire Courage at bard level 1: affected allies receive a +1 morale bonus
    // on saving throws against charm and fear effects and a +1 competence bonus
    // on attack and weapon damage rolls. Confirmed unchanged through level 4: the
    // PF1 Core Rulebook Inspire Courage bonus first increases (to +2) exactly at
    // bard level 5 (verified independently against d20pfsrd and
    // legacy.aonprd.com's Bard class table before widening this slice, re-checked
    // rather than trusted from an earlier cycle's phrasing at face value: "At 5th
    // level, and every six bard levels thereafter, this bonus increases by +1" —
    // the increase lands AT level 5, not after it, so the earlier cycle's "stays
    // +1 through level 5" framing turns out to have been precise). Uses
    // `bard_inspire_courage_bonus` (v0.6 alpha swarm, risks item 8) rather than
    // re-deriving the tier inline, the same pure function
    // `active_bard_inspire_courage_attack_bonus` calls for the real conditional
    // application.
    let inspire_courage_bonus = bard_inspire_courage_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.inspire_courage_bonus".to_owned(),
        value: inspire_courage_bonus,
        detail: format!(
            "Bard Inspire Courage magnitude at bard level {level} (PF1 Core Rulebook Inspire \
             Courage): a +{inspire_courage_bonus} competence bonus on attack rolls and weapon \
             damage rolls and a +{inspire_courage_bonus} morale bonus on saving throws against \
             charm and fear effects for affected allies. This magnitude increases from +1 to +2 \
             exactly at bard level {BARD_INSPIRE_COURAGE_SECOND_TIER_LEVEL}, again from +2 to +3 \
             exactly at bard level {BARD_INSPIRE_COURAGE_THIRD_TIER_LEVEL}, and again from +3 to \
             +4 exactly at bard level {BARD_INSPIRE_COURAGE_FOURTH_TIER_LEVEL} (PF1 Core \
             Rulebook: \"At 5th level, and every six bard levels thereafter, this bonus \
             increases by +1\"), so it is +{inspire_courage_bonus} at level {level}; the next \
             increase (to +5) is at bard level 23, out of scope for this bounded slice. This is \
             the identity/recognition record only; the real conditional application of the \
             attack-roll bonus to the integrated baseline melee attack bonus (gated on a valid, \
             active, in-budget performance) happens in compute_combat_baseline, not here. The \
             weapon-damage and saves-vs-charm/fear bonuses are not applied to any integrated \
             total: this codebase has no integrated weapon-damage total and no \
             save-vs-effect-type sub-category to apply either to"
        ),
    });

    // Grounded for real: the Fascinate flat Will-save DC formula. PF1 Core
    // Rulebook Fascinate: each creature within range receives a Will save (DC
    // 10 + 1/2 the bard's level + the bard's Charisma modifier) to negate the
    // effect. Verified against the PF1 Core Rulebook Fascinate rule text (d20pfsrd
    // and the legacy Paizo PRD mirror), not trusted from memory alone. This
    // formula already takes bard level as an input variable, so it extends to
    // level 2 without re-derivation. Only the flat DC magnitude is grounded; no
    // Will-save resolution and no application of this DC to any actual save
    // total is computed.
    let fascinate_dc = FASCINATE_DC_BASE + (level_value / 2) + ability_modifiers.charisma;
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.fascinate_dc".to_owned(),
        value: fascinate_dc,
        detail: format!(
            "Bard Fascinate Will save DC at bard level {level} (PF1 Core Rulebook Fascinate): \
             DC = 10 + 1/2 bard level + Charisma modifier. At bard level {level} and Charisma \
             modifier {} this is {FASCINATE_DC_BASE} + ({level} / 2) + {} = {fascinate_dc}. \
             This grounds only the flat DC magnitude; no Will-save resolution, no \
             range/line-of-sight/attention-requirement checking, and no application of this DC \
             to any actual save total is computed because the performance-state engine is not \
             implemented",
            ability_modifiers.charisma, ability_modifiers.charisma
        ),
    });

    // Grounded for real: the Fascinate flat affected-creature-count formula. PF1
    // Core Rulebook Fascinate: a bard can affect one creature at 1st level, and
    // targets one additional creature for every three bard levels attained beyond
    // 1st. Verified against the PF1 Core Rulebook Fascinate rule text the same
    // way as the DC above; this is deliberately NOT "half the bard's level" — a
    // different-looking formula that happens to coincide with the correct one
    // only at level 1, which is exactly the kind of from-memory error a primary
    // source check catches (mirroring the earlier Ranger combat-style and
    // Paladin mercy level-gate corrections). This formula already takes bard
    // level as an input variable, so it extends to level 2 without re-derivation.
    let fascinate_affected_creatures = 1 + (level_value - 1) / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.fascinate_affected_creatures".to_owned(),
        value: fascinate_affected_creatures,
        detail: format!(
            "Bard Fascinate affected-creature count at bard level {level} (PF1 Core Rulebook \
             Fascinate): 1 creature at 1st level, plus one additional creature for every three \
             bard levels attained beyond 1st — formula 1 + (bard level - 1) / 3. At bard level \
             {level} this is 1 + ({level} - 1) / 3 = {fascinate_affected_creatures}. This \
             grounds only the flat creature-count magnitude; no \
             range/line-of-sight/attention-requirement checking and no application of this \
             count to any actual targeting resolution is computed"
        ),
    });

    // Grounded (SD13-E5): Well-Versed, a 2nd-level Bard class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Versatile performance, well-versed" as the
    // Bard 2nd-level special feature entry). Below the level-2 gate this is a
    // correct PF1 Core Rulebook level-gate absence (value 0); at or above it, it
    // is a flat, non-level-scaled +4 standalone magnitude (verified against both
    // primary sources: unlike Bardic Knowledge or Fascinate, this bonus does NOT
    // scale with level), mirroring the Fighter Bravery idiom — never applied to
    // any actual save total, since no save-resolution engine exists in this
    // codebase.
    if level < BARD_WELL_VERSED_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.well_versed".to_owned(),
            value: 0,
            detail: format!(
                "Bard Well-Versed at bard level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Well-Versed is a 2nd-level Bard class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.well_versed".to_owned(),
            value: BARD_WELL_VERSED_BONUS,
            detail: format!(
                "Bard Well-Versed granted at bard level {level} (PF1 Core Rulebook, 2nd-level \
                 Bard class feature): a flat +{BARD_WELL_VERSED_BONUS} bonus on saving throws \
                 made against bardic performance, sonic, and language-dependent effects. Unlike \
                 Bardic Knowledge or Fascinate, this magnitude is not level-scaled. This is a \
                 standalone explanation record only; it is never applied to any actual save \
                 total because no saving-throw-resolution engine exists anywhere in this \
                 codebase"
            ),
        });
    }

    // Grounded (SD13-E5): Inspire Competence, a 3rd-level Bard class feature
    // verified independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Inspire competence +2" as the Bard 3rd-level
    // special feature entry). Below the level-3 gate this is a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it, it is a flat
    // standalone magnitude (a competence bonus on skill checks with a particular
    // skill, verified against both primary sources), mirroring the Fighter
    // Bravery / Rogue Trap Sense / Barbarian Trap Sense / Monk Still Mind idiom —
    // never applied to any actual skill-check total, since no
    // skill-check-resolution engine exists in this codebase, and no
    // task-selection/action-economy engine decides which skill or ally it
    // targets. The magnitude genuinely increases from +2 to +3 exactly at bard
    // level 7 (SD13-E5, verified independently against two primary sources: both
    // list "Inspire competence +3" as the Bard 7th-level special feature entry,
    // and both state the rule text "This bonus increases by +1 for every four
    // levels the bard has attained beyond 3rd"), mirroring the Inspire Courage
    // second-tier idiom exactly; the next increase (to +4) lands at bard level
    // 11, out of scope for this bounded slice.
    if level < BARD_INSPIRE_COMPETENCE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.inspire_competence".to_owned(),
            value: 0,
            detail: format!(
                "Bard Inspire Competence at bard level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Inspire Competence is a 3rd-level Bard class feature."
            ),
        });
    } else {
        let inspire_competence_bonus = if level >= BARD_INSPIRE_COMPETENCE_FIFTH_TIER_LEVEL {
            BARD_INSPIRE_COMPETENCE_BONUS_FIFTH_TIER
        } else if level >= BARD_INSPIRE_COMPETENCE_FOURTH_TIER_LEVEL {
            BARD_INSPIRE_COMPETENCE_BONUS_FOURTH_TIER
        } else if level >= BARD_INSPIRE_COMPETENCE_THIRD_TIER_LEVEL {
            BARD_INSPIRE_COMPETENCE_BONUS_THIRD_TIER
        } else if level >= BARD_INSPIRE_COMPETENCE_SECOND_TIER_LEVEL {
            BARD_INSPIRE_COMPETENCE_BONUS_SECOND_TIER
        } else {
            BARD_INSPIRE_COMPETENCE_BONUS_FIRST_TIER
        };
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.inspire_competence".to_owned(),
            value: inspire_competence_bonus,
            detail: format!(
                "Bard Inspire Competence granted at bard level {level} (PF1 Core Rulebook, \
                 3rd-level Bard class feature): a flat +{inspire_competence_bonus} competence \
                 bonus on skill checks with a particular skill. This magnitude increases from \
                 +2 to +3 exactly at bard level {BARD_INSPIRE_COMPETENCE_SECOND_TIER_LEVEL}, \
                 again from +3 to +4 exactly at bard level \
                 {BARD_INSPIRE_COMPETENCE_THIRD_TIER_LEVEL}, again from +4 to +5 exactly at \
                 bard level {BARD_INSPIRE_COMPETENCE_FOURTH_TIER_LEVEL}, and again from +5 to \
                 +6 exactly at bard level {BARD_INSPIRE_COMPETENCE_FIFTH_TIER_LEVEL} (PF1 Core \
                 Rulebook: \"This bonus increases by +1 for every four levels the bard has \
                 attained beyond 3rd\"), so it is +{inspire_competence_bonus} at level {level}; \
                 the next increase (to +7) is at bard level 23, out of scope for this bounded \
                 slice. This is a standalone explanation record only; it is never applied to any \
                 actual skill-check total because no skill-check-resolution engine exists \
                 anywhere in this codebase, and no task-selection/action-economy engine decides \
                 which skill or ally it targets"
            ),
        });
    }

    // Grounded (SD13-E5): Lore Master, a 5th-level Bard class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Inspire courage +2, lore master 1/day" as the
    // Bard 5th-level special feature entry). Below the level-5 gate this is a
    // correct PF1 Core Rulebook level-gate absence (value 0); at or above it, this
    // grounds only the rule's own flat "1/day" usage-count magnitude for the
    // take-20 half of the feature, mirroring the Paladin Smite Evil / Wizard
    // Force Missile uses-per-day idiom. The rule text grants TWO distinct
    // capabilities: (1) an at-will "take 10 on any Knowledge skill check that he
    // has ranks in" capability, which has no flat magnitude to ground at all (it
    // is a resolution-mode toggle, not a countable resource) and would require a
    // skill-check-resolution engine that does not exist anywhere in this
    // codebase, and (2) "once per day, the bard can take 20 on any Knowledge
    // skill check as a standard action" — a genuinely flat 1/day count, grounded
    // here as a bounded grant-only identity record. Neither the take-10 nor the
    // take-20 mechanic is actually executed against any Knowledge check (no
    // skill-check-resolution engine exists in this codebase), mirroring the
    // Barbarian Improved Uncanny Dodge / Monk Purity of Body idiom exactly: a
    // bounded grant, not an executed mechanic.
    if level < BARD_LORE_MASTER_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.lore_master".to_owned(),
            value: 0,
            detail: format!(
                "Bard Lore Master at bard level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant rule is named but not computed. Lore \
                 Master is a 5th-level Bard class feature."
            ),
        });
    } else {
        let lore_master_uses_per_day = if level >= BARD_LORE_MASTER_THIRD_TIER_LEVEL {
            BARD_LORE_MASTER_TAKE_20_USES_PER_DAY_THIRD_TIER
        } else if level >= BARD_LORE_MASTER_SECOND_TIER_LEVEL {
            BARD_LORE_MASTER_TAKE_20_USES_PER_DAY_SECOND_TIER
        } else {
            BARD_LORE_MASTER_TAKE_20_USES_PER_DAY
        };
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.lore_master".to_owned(),
            value: lore_master_uses_per_day,
            detail: format!(
                "Bard Lore Master granted at bard level {level} (PF1 Core Rulebook, 5th-level \
                 Bard class feature): \"the bard becomes a master of lore and can take 10 on any \
                 Knowledge skill check that he has ranks in... once per day, the bard can take \
                 20 on any Knowledge skill check as a standard action.\" This grounds only the \
                 rule's own flat {lore_master_uses_per_day}/day usage-count magnitude for the \
                 take-20 half of the feature (a bounded grant-only identity record, mirroring \
                 the Paladin Smite Evil / Wizard Force Missile uses-per-day idiom), which \
                 genuinely rises from 1/day to 2/day exactly at bard level \
                 {BARD_LORE_MASTER_SECOND_TIER_LEVEL} (PF1 Core Rulebook: \"Inspire competence \
                 +4, inspire courage +3, lore master 2/day\" at the Bard 11th-level special \
                 feature entry, the same every-sixth-level-after-5th cadence as Inspire \
                 Courage), and again from 2/day to 3/day exactly at bard level \
                 {BARD_LORE_MASTER_THIRD_TIER_LEVEL} (PF1 Core Rulebook: \"Inspire courage +4, \
                 lore master 3/day\" at the Bard 17th-level special feature entry, the same \
                 cadence); the next increase lands beyond bard level 17, out of scope for this \
                 bounded slice. The take-10 capability has no flat magnitude to ground (it is an \
                 at-will resolution-mode toggle, not a countable resource), and neither the \
                 take-10 nor the take-20 mechanic is actually executed against any Knowledge \
                 check, since no skill-check-resolution engine exists anywhere in this codebase"
            ),
        });
    }

    // Grounded (SD13-E5 level-10 slice): Jack-of-All-Trades' 10th-level piece,
    // verified independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Jack-of-all-trades, versatile performance" as
    // the Bard 10th-level "Special" entry). At 10th level the bard "can use any
    // skill, even if the skill normally requires him to be trained" — a
    // genuinely flat, no-choice, no-magnitude grant, grounded as a bounded +0
    // identity/recognition record mirroring the Woodland Stride / Purity of
    // Body idiom: no trained-only skill gating exists anywhere in this
    // codebase's skill computation to lift, so no untrained-use effect is
    // fabricated. The feature's OWN later tiers (16th: all skills become class
    // skills; 19th: take 10 on any skill check) land beyond this tranche's
    // level-10 ceiling and are not grounded or named as records. Below the
    // level-10 gate no record is pushed at all.
    if level >= BARD_JACK_OF_ALL_TRADES_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.jack_of_all_trades".to_owned(),
            value: 0,
            detail: format!(
                "Bard Jack-of-All-Trades granted at bard level {level} (PF1 Core Rulebook, \
                 10th-level bard class feature): the bard can use any skill, even if the \
                 skill normally requires him to be trained. This is a bounded \
                 identity/recognition record only (value 0, non-fabricated): no trained-only \
                 skill gating exists anywhere in this codebase's bounded skill computation to \
                 lift, so this grounds no actual untrained-skill-use effect; the feature's \
                 16th- and 19th-level tiers land beyond this tranche's level-10 ceiling and \
                 are not grounded"
            ),
        });
    }

    // Grounded (SD18): Soothing Performance, a 12th-level Bard class feature
    // verified independently against two primary PF1 sources (d20pfsrd and
    // the Archives of Nethys aonprd.com mirror both list "Soothing
    // performance" as the sole Bard 12th-level "Special" column entry).
    // Below the level-12 gate this is a correct PF1 Core Rulebook
    // level-gate absence (value 0); at or above it, it is a bounded
    // grant-only identity record (value 0, non-fabricated) naming the rule
    // text — mirroring the Monk Diamond Body / Paladin Aura of Justice
    // grant-only idiom exactly: no healing-application engine and no
    // condition-removal engine exist anywhere in this codebase to apply
    // the effect to.
    if level < BARD_SOOTHING_PERFORMANCE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.soothing_performance".to_owned(),
            value: 0,
            detail: format!(
                "Bard Soothing Performance at bard level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Soothing Performance is a 12th-level Bard class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.soothing_performance".to_owned(),
            value: 0,
            detail: format!(
                "Bard Soothing Performance granted at bard level {level} (PF1 Core Rulebook, \
                 12th-level Bard class feature): \"a bard of 12th level or higher can use his \
                 performance to help heal the wounds of his allies... this ability functions \
                 as mass cure serious wounds... this use of bardic performance also removes \
                 the fatigued, sickened, and shaken conditions.\" This is a bounded grant-only \
                 identity record only (value 0, non-fabricated): no healing-application engine \
                 and no condition-removal engine exists anywhere in this codebase to apply the \
                 effect to."
            ),
        });
    }

    // Grounded (SD18): Frightening Tune, a 14th-level Bard class feature
    // verified independently against two primary PF1 sources (d20pfsrd and
    // the Archives of Nethys aonprd.com mirror both list "Frightening tune,
    // Versatile performance" as the Bard 14th-level "Special" column entry).
    // The rule text gives a Will-save DC (10 + 1/2 the bard's level + the
    // bard's Cha modifier) — the exact same formula shape as the
    // already-grounded Fascinate DC, so only that flat DC magnitude is
    // grounded here, mirroring the Fascinate DC idiom exactly. Below the
    // level-14 gate no record is pushed at all. Unlike Fascinate, this
    // feature's affected scope ("each enemy within 30 feet who can hear the
    // performance") is range-based, not a numeric-count formula, so no
    // affected-creature-count record is added for it, and no fear/frightened
    // condition is ever applied because no condition-resolution engine
    // exists anywhere in this codebase.
    if level >= BARD_FRIGHTENING_TUNE_LEVEL {
        let frightening_tune_dc = FASCINATE_DC_BASE + (level_value / 2) + ability_modifiers.charisma;
        explanations.push(ComputationExplanation {
            id: "class_chassis.bard.frightening_tune_dc".to_owned(),
            value: frightening_tune_dc,
            detail: format!(
                "Bard Frightening Tune Will save DC at bard level {level} (PF1 Core Rulebook, \
                 14th-level Bard class feature): DC = 10 + 1/2 bard level + Charisma modifier, \
                 the same formula shape as the Fascinate DC. At bard level {level} and Charisma \
                 modifier {} this is {FASCINATE_DC_BASE} + ({level} / 2) + {} = \
                 {frightening_tune_dc}. This grounds only the flat DC magnitude; no \
                 range/line-of-sight/audible-performance-requirement checking, no \
                 affected-creature-count (the rule text is range-based, not a numeric-count \
                 formula), no Will-save resolution, and no application of a frightened \
                 condition to any target is computed because neither the performance-state \
                 engine nor a condition-resolution engine is implemented",
                ability_modifiers.charisma, ability_modifiers.charisma
            ),
        });
    }

    // Grounded (SD18): Deadly Performance, the Bard's 20th-level class
    // capstone, verified independently against two primary PF1 sources
    // (d20pfsrd and the Archives of Nethys aonprd.com mirror both list
    // "Deadly performance" as the sole Bard 20th-level "Special" column
    // entry). The rule text gives a Will-save DC (10 + 1/2 the bard's
    // level + the bard's Cha modifier) — the exact same formula shape as
    // the already-grounded Fascinate DC and Frightening Tune DC, so only
    // that flat DC magnitude is grounded here, mirroring the Frightening
    // Tune idiom exactly. Below the level-20 gate no record is pushed at
    // all. No audible/visual-performance-requirement checking, no
    // Will-save resolution, and no death-effect application is ever
    // computed because no targeting/range, save-resolution, or
    // death-effect-resolution engine exists anywhere in this codebase.
    if level >= BARD_DEADLY_PERFORMANCE_LEVEL {
        let deadly_performance_dc = FASCINATE_DC_BASE + (level_value / 2) + ability_modifiers.charisma;
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.deadly_performance_dc".to_owned(),
            value: deadly_performance_dc,
            detail: format!(
                "Bard Deadly Performance Will save DC at bard level {level} (PF1 Core Rulebook, \
                 20th-level Bard class capstone): DC = 10 + 1/2 bard level + Charisma modifier, \
                 the same formula shape as the Fascinate DC and Frightening Tune DC. At bard \
                 level {level} and Charisma modifier {} this is {FASCINATE_DC_BASE} + \
                 ({level} / 2) + {} = {deadly_performance_dc}. This grounds only the flat DC \
                 magnitude; no range/line-of-sight/audible-and-visual-performance-requirement \
                 checking, no Will-save resolution, and no death-effect application to any \
                 target is computed because neither the performance-state engine nor a \
                 death-effect-resolution engine is implemented",
                ability_modifiers.charisma, ability_modifiers.charisma
            ),
        });
    }

    // Grounded (SD18): Inspire Heroics, a 15th-level Bard class feature
    // verified independently against two primary PF1 sources (d20pfsrd and
    // the Archives of Nethys aonprd.com mirror both list "Inspire competence
    // +5, inspire heroics" as the Bard 15th-level "Special" column entry).
    // The rule text: "A bard of 15th level or higher can inspire tremendous
    // heroism in himself or a single ally within 30 feet... Inspired
    // creatures gain a +4 morale bonus on saving throws and a +4 dodge bonus
    // to AC." Below the level-15 gate no record is pushed at all (mirroring
    // the Frightening Tune idiom exactly, since a "correctly absent"
    // placeholder is unnecessary busywork across three separate ids); at or
    // above it, it grounds only the two flat, non-level-scaled magnitude
    // numbers (mirroring the Well-Versed flat-magnitude idiom) and the flat
    // base target count of 1 (mirroring the Fascinate affected-creature-count
    // idiom — the rule's own further scaling, "+1 creature per three bard
    // levels beyond 15th," lands at bard level 18, beyond this bounded
    // slice's ceiling, and is not grounded). No targeting, save resolution,
    // or AC application is ever computed because no such engine exists
    // anywhere in this codebase.
    if level >= BARD_INSPIRE_HEROICS_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.inspire_heroics_save_bonus".to_owned(),
            value: BARD_INSPIRE_HEROICS_SAVE_BONUS,
            detail: format!(
                "Bard Inspire Heroics granted at bard level {level} (PF1 Core Rulebook, \
                 15th-level Bard class feature): a flat +{BARD_INSPIRE_HEROICS_SAVE_BONUS} \
                 morale bonus on saving throws for the inspired creature(s). This magnitude is \
                 non-level-scaled (fixed at +4 for the class feature's entire existence), \
                 mirroring the Well-Versed idiom. This is a standalone explanation record only \
                 -- task #88 correction: `defense.total_save.*` ARE real integrated totals \
                 this codebase computes for a Bard, but no targeting/action-economy engine \
                 exists to decide whether the bard targeted herself (this ability can also \
                 target another willing creature) or is currently active at all, so this \
                 bonus is never applied to them"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.inspire_heroics_ac_bonus".to_owned(),
            value: BARD_INSPIRE_HEROICS_AC_BONUS,
            detail: format!(
                "Bard Inspire Heroics granted at bard level {level} (PF1 Core Rulebook, \
                 15th-level Bard class feature): a flat +{BARD_INSPIRE_HEROICS_AC_BONUS} dodge \
                 bonus to AC for the inspired creature(s). This magnitude is non-level-scaled \
                 (fixed at +4 for the class feature's entire existence), mirroring the \
                 Well-Versed idiom. This is a standalone explanation record only -- task #88 \
                 correction: `defense.baseline_armor_class` IS a real integrated AC total this \
                 codebase computes for a Bard, but no targeting/action-economy engine exists \
                 to decide whether the bard targeted herself (this ability can also target \
                 another willing creature) or is currently active at all, so this bonus is \
                 never applied to it"
            ),
        });
        let inspire_heroics_target_count =
            if level >= BARD_INSPIRE_HEROICS_TARGET_COUNT_SECOND_TIER_LEVEL {
                BARD_INSPIRE_HEROICS_BASE_TARGET_COUNT_SECOND_TIER
            } else {
                BARD_INSPIRE_HEROICS_BASE_TARGET_COUNT
            };
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.inspire_heroics_target_count".to_owned(),
            value: inspire_heroics_target_count,
            detail: format!(
                "Bard Inspire Heroics target count at bard level {level} (PF1 Core Rulebook, \
                 15th-level Bard class feature): \"himself or a single ally within 30 feet\" is \
                 a flat {BARD_INSPIRE_HEROICS_BASE_TARGET_COUNT} target starting at level \
                 {BARD_INSPIRE_HEROICS_LEVEL}, mirroring the Fascinate affected-creature-count \
                 idiom. The rule's own further scaling (\"for every three bard levels the \
                 character attains beyond 15th, he can inspire heroics in an additional \
                 creature\") genuinely rises the count to \
                 {BARD_INSPIRE_HEROICS_BASE_TARGET_COUNT_SECOND_TIER} starting exactly at bard \
                 level {BARD_INSPIRE_HEROICS_TARGET_COUNT_SECOND_TIER_LEVEL} (a second tier on \
                 an already-generalized tiered if/else chain, the same idiom as Inspire \
                 Courage/Inspire Competence/Lore Master's own tier additions). This grounds \
                 only the flat count; no targeting or performance-state execution is computed"
            ),
        });
    }

    // `AT-34-E3-001` (`class_feature_option_pool_record_with_magnitude_not_
    // held_by_engine` mechanism, cycle 5): Suggestion, a 6th-level Bard
    // class feature verified directly against this repo's own ingested
    // corpus record (`data/corpus/core_rulebook/class_feature/bardic_
    // performance/suggestion.json`, `BONUS:VAR|SuggestionDC|10+(BardicPerformanceLVL/2)+CHA`)
    // -- the exact same formula shape as the already-grounded Fascinate DC,
    // Frightening Tune DC, and Deadly Performance DC, so only that flat DC
    // magnitude is grounded here, mirroring those idioms exactly. Below the
    // level-6 gate no record is pushed at all. No range/audible-performance
    // checking, no Will-save resolution, and no suggestion-effect
    // application is ever computed because neither the performance-state
    // engine nor an effect-resolution engine is implemented.
    if level >= BARD_SUGGESTION_LEVEL {
        let suggestion_dc = FASCINATE_DC_BASE + (level_value / 2) + ability_modifiers.charisma;
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.suggestion_dc".to_owned(),
            value: suggestion_dc,
            detail: format!(
                "Bard Suggestion Will save DC at bard level {level} (PF1 Core Rulebook, 6th-level \
                 Bard class feature): DC = 10 + 1/2 bard level + Charisma modifier, the same \
                 formula shape as the Fascinate DC. At bard level {level} and Charisma modifier \
                 {} this is {FASCINATE_DC_BASE} + ({level} / 2) + {} = {suggestion_dc}. This \
                 grounds only the flat DC magnitude; no range/audible-performance-requirement \
                 checking, no Will-save resolution, and no application of the suggestion effect \
                 is computed because neither the performance-state engine nor an \
                 effect-resolution engine is implemented",
                ability_modifiers.charisma, ability_modifiers.charisma
            ),
        });
    }

    // `AT-34-E3-001` (same mechanism/cycle as Suggestion above): Mass
    // Suggestion, an 18th-level Bard class feature verified directly
    // against this repo's own ingested corpus record (`.../bardic_
    // performance/mass_suggestion.json`, `BONUS:VAR|MassSuggestionDC|
    // 10+(BardicPerformanceLVL/2)+CHA`) -- the identical DC formula shape,
    // mirroring the Suggestion idiom immediately above. Below the level-18
    // gate no record is pushed at all.
    if level >= BARD_MASS_SUGGESTION_LEVEL {
        let mass_suggestion_dc = FASCINATE_DC_BASE + (level_value / 2) + ability_modifiers.charisma;
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.mass_suggestion_dc".to_owned(),
            value: mass_suggestion_dc,
            detail: format!(
                "Bard Mass Suggestion Will save DC at bard level {level} (PF1 Core Rulebook, \
                 18th-level Bard class feature): DC = 10 + 1/2 bard level + Charisma modifier, \
                 the same formula shape as the Fascinate/Suggestion DC. At bard level {level} \
                 and Charisma modifier {} this is {FASCINATE_DC_BASE} + ({level} / 2) + {} = \
                 {mass_suggestion_dc}. This grounds only the flat DC magnitude; no \
                 range/audible-performance-requirement checking, no multi-target Will-save \
                 resolution, and no application of the suggestion effect is computed because \
                 neither the performance-state engine nor an effect-resolution engine is \
                 implemented",
                ability_modifiers.charisma, ability_modifiers.charisma
            ),
        });
    }

    // `AT-34-E3-001` (same mechanism/cycle): Inspire Greatness, a 9th-level
    // Bard class feature verified directly against this repo's own
    // ingested corpus record (`.../bardic_performance/inspire_greatness
    // .json`, `BONUS:VAR|InspireGreatnessAllies|min((BardicPerformanceLVL-6)/3,4)`)
    // -- the same "flat count formula" shape as the already-grounded
    // Fascinate affected-creature-count, mirroring that idiom. Below the
    // level-9 gate no record is pushed at all; the corpus's own `min(...,4)`
    // cap is preserved exactly, never re-derived as an unbounded rise.
    if level >= BARD_INSPIRE_GREATNESS_LEVEL {
        let inspire_greatness_allies = std::cmp::min(
            (level_value - BARD_INSPIRE_GREATNESS_LEVEL as i16 + 3) / 3,
            BARD_INSPIRE_GREATNESS_MAX_ALLIES,
        );
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.inspire_greatness_allies".to_owned(),
            value: inspire_greatness_allies,
            detail: format!(
                "Bard Inspire Greatness affected-ally count at bard level {level} (PF1 Core \
                 Rulebook, 9th-level Bard class feature): the corpus's own formula \
                 min((bard level - 6) / 3, 4). At bard level {level} this is min(({level} - 6) \
                 / 3, {BARD_INSPIRE_GREATNESS_MAX_ALLIES}) = {inspire_greatness_allies}. This \
                 grounds only the flat ally-count magnitude; no targeting and no application of \
                 the granted bonuses (a copy of the bard's own Bardic Knowledge, extra hit \
                 points, and a +1 bonus on saves against death/negative-energy/spells) to any \
                 ally is computed because no such multi-recipient application engine exists \
                 anywhere in this codebase"
            ),
        });
    }

    // v0.6 alpha swarm, risks item 8: the unconditional "bardic
    // performance-execution engine is missing" diagnostic that used to
    // live here moved to `ground_or_block_bard_bardic_performance`, called
    // at the top of this function -- Inspire Courage's own engine is real
    // now (start/maintain, round-budget validation, attack-bonus
    // application), and its validity depends on
    // `class_ability_activations`, not on level/race alone, so it could
    // not stay a flat unconditional diagnostic at this position. The other
    // six named-but-unexecuted performances (Countersong, Distraction,
    // Versatile Performance, Soothing Performance, Frightening Tune,
    // Inspire Heroics, Deadly Performance) are still named as permanently
    // unmodeled -- see the non-blocking diagnostic
    // `ground_or_block_bard_bardic_performance` pushes unconditionally.

    // SD13-E5: the spontaneous spell-level ACCESS ladder, mirroring the
    // Paladin spell_level_access record and the Cleric/Wizard
    // <CLASS>_<N>TH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL threshold doctrine
    // exactly ("first non-'—' spells-per-day column", verified against the
    // raw table rows of both primary sources). This grounds the highest
    // ACCESSIBLE bard spell level only; the per-day counts themselves are
    // never computed. Cantrips (0th level, "spells known" only) are outside
    // the spells-per-day ladder and are not counted.
    let bard_spell_level_access: i16 = bard_spell_level_access(level);
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.spontaneous.spell_level_access".to_owned(),
        value: bard_spell_level_access,
        detail: format!(
            "Bard spell-level access at bard level {level}: the highest bard spell level \
             (1st+) with a non-\"—\" spells-per-day column in the PF1 Core Rulebook Bard \
             class table is {bard_spell_level_access} (verified against the raw table rows of \
             both primary sources: 1st-level spells from level 1 — the ladder has no zero \
             step — 2nd-level at {BARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 3rd-level \
             at {BARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 4th-level at \
             {BARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}; the 5th-level column stays \
             \"—\" through level 10). Cantrips are \"spells known\" only and sit outside \
             the spells-per-day ladder, so they are not counted. This grounds the access \
             ladder only: no spells-per-day counts, no spells-known posture, no bonus slots \
             from a high Charisma, and no spell save DCs are computed"
        ),
    });

    // SD13-E5: the BASE spells-per-day counts, one record per ACCESSIBLE
    // spell level, as a literal table lookup mirroring the Paladin/Ranger
    // per-day slices and the Cleric domain-slot-count precedent — the PF1
    // spells-per-day table is a lookup table, not arithmetic, so no formula
    // is invented for it. Verified against the raw table rows of both
    // primary sources (identical on d20pfsrd and legacy.aonprd.com): level
    // 1 "1/—/—/—", level 2 "2/—/—/—", level 3 "3/—/—/—", level 4 "3/1/—/—",
    // level 5 "4/2/—/—", level 6 "4/3/—/—", level 7 "4/3/1/—", level 8
    // "4/4/2/—", level 9 "5/4/3/—", level 10 "5/4/3/1". Unlike the
    // Paladin/Ranger tables there are NO "0" entries at levels 1-10 —
    // every accessible column carries a positive base count. Inaccessible
    // spell levels ("—" columns) get no record at all. Only the base
    // counts are grounded: bonus spells per day from a high Charisma are
    // never computed, and spells KNOWN (a separate table) stays untouched.
    let bard_base_spells_per_day: [Option<i16>; 4] = match level {
        1 => [Some(1), None, None, None],
        2 => [Some(2), None, None, None],
        3 => [Some(3), None, None, None],
        4 => [Some(3), Some(1), None, None],
        5 => [Some(4), Some(2), None, None],
        6 => [Some(4), Some(3), None, None],
        7 => [Some(4), Some(3), Some(1), None],
        8 => [Some(4), Some(4), Some(2), None],
        9 => [Some(5), Some(4), Some(3), None],
        10 => [Some(5), Some(4), Some(3), Some(1)],
        _ => [None, None, None, None],
    };
    for (index, base_count) in bard_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = index + 1;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.bard.spontaneous.base_spells_per_day.spell_level_{spell_level}"
            ),
            value: *base_count,
            detail: format!(
                "Bard base spells per day at bard level {level}, spell level {spell_level}: \
                 {base_count}, read directly from the PF1 Core Rulebook Bard class table's \
                 spells-per-day row (verified against the raw table rows of both primary \
                 sources; a literal table lookup, not a derived formula; the Bard table has \
                 no \"0\" entries at levels 1-10). This grounds the base count only: bonus \
                 spells per day from a high Charisma are never computed, spells KNOWN (a \
                 separate table) is not grounded, and no spell save DCs are computed"
            ),
        });
    }

    // SD13-E5: the base spell-save-DC arithmetic, one record per ACCESSIBLE
    // spell level, mirroring the Sorcerer DC slice. Verified against both
    // primary sources, which state the rule identically: "The Difficulty
    // Class for a saving throw against a bard's spell is 10 + the spell
    // level + the bard's Charisma modifier." This is a DIFFERENT formula
    // family from the grounded Fascinate DC (10 + 1/2 bard level + Charisma
    // modifier — a performance DC keyed to bard level, not spell level);
    // both coexist as separate records. This grounds only the base formula
    // over values already on the seam: no saving-throw resolution, no
    // target, no spell selection, and no feat DC modifiers are computed.
    for spell_level in 1..=bard_spell_level_access {
        let spell_save_dc = 10 + spell_level + ability_modifiers.charisma;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.bard.spontaneous.spell_save_dc.spell_level_{spell_level}"
            ),
            value: spell_save_dc,
            detail: format!(
                "Bard spell save DC at bard level {level}, spell level {spell_level}: 10 + \
                 {spell_level} + Charisma modifier {} = {spell_save_dc} (PF1 Core Rulebook, \
                 verified identically on both primary sources: \"The Difficulty Class for a \
                 saving throw against a bard's spell is 10 + the spell level + the bard's \
                 Charisma modifier\"). This grounds the base DC formula only: no \
                 saving-throw resolution, no target, no spell selection, and no feat DC \
                 modifiers are computed",
                ability_modifiers.charisma
            ),
        });
    }

    // SD13-E5: the BASE spells-KNOWN counts, one record per spell level with
    // a non-"—" column in the Bard Spells Known table, as a literal table
    // lookup per the same doctrine as the per-day family. Verified against
    // the raw table rows of both primary sources (identical on d20pfsrd and
    // legacy.aonprd.com): level 1 "4/2/—/—/—", level 2 "5/3/—/—/—", level 3
    // "6/4/—/—/—", level 4 "6/4/2/—/—", level 5 "6/4/3/—/—", level 6
    // "6/4/4/—/—", level 7 "6/5/4/2/—", level 8 "6/5/4/3/—", level 9
    // "6/5/4/4/—", level 10 "6/5/5/4/2" (0th through 4th spell level).
    // UNLIKE the spells-per-day table, this table INCLUDES the 0th level
    // (cantrips) — exactly where the access-ladder record's "cantrips are
    // spells known only" note lands. Only the known COUNTS are grounded:
    // the selection of WHICH spells are known is never computed.
    let bard_spells_known: [Option<i16>; 5] = bard_spells_known_table(level);
    for (spell_level, known_count) in bard_spells_known.iter().enumerate() {
        let Some(known_count) = known_count else {
            continue;
        };
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.bard.spontaneous.spells_known.spell_level_{spell_level}"),
            value: *known_count,
            detail: format!(
                "Bard base spells known at bard level {level}, spell level {spell_level}: \
                 {known_count}, read directly from the PF1 Core Rulebook Bard Spells Known \
                 table (verified against the raw table rows of both primary sources; a \
                 literal table lookup, not a derived formula; unlike the spells-per-day \
                 table, this table includes the 0th level — cantrips are spells known \
                 only). This grounds the base known count only: the selection of WHICH \
                 spells are known from the Bard list is never computed — no spell-list \
                 content, no spell identities, and no swap/retraining rules"
            ),
        });
    }

    // SD13-E5: the bonus spells per day from a high Charisma, one record
    // per ACCESSIBLE spell level (1st+; cantrips never gain bonus spells),
    // from PF1's shared Table: Ability Modifiers and Bonus Spells,
    // mirroring the Sorcerer bonus slice — verified against both primary
    // sources' ability-scores pages: for modifier m and spell level N, 0
    // when m < N, otherwise (m - N)/4 + 1, gated by the grounded access
    // ladder per the identical rule text on both sources. The bonus is
    // never added to the base per-day counts here — no total is computed.
    for spell_level in 1..=bard_spell_level_access {
        let bonus_spells = if ability_modifiers.charisma < spell_level {
            0
        } else {
            (ability_modifiers.charisma - spell_level) / 4 + 1
        };
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.bard.spontaneous.bonus_spells_per_day.spell_level_{spell_level}"
            ),
            value: bonus_spells,
            detail: format!(
                "Bard bonus spells per day at bard level {level}, spell level \
                 {spell_level}: {bonus_spells} from Charisma modifier {} (PF1 Core Rulebook \
                 Table: Ability Modifiers and Bonus Spells, verified identically on both \
                 primary sources; for modifier m and spell level N the table value is 0 \
                 when m < N, otherwise (m - N)/4 + 1, and bonus spells apply only to spell \
                 levels the character is of a high enough class level to cast — the \
                 grounded access ladder). A computed 0 means the modifier grants no bonus \
                 at this spell level; it is never added to the base per-day count here — no \
                 total is computed, no spell selection, and no spell save DCs",
                ability_modifiers.charisma
            ),
        });
    }

    // SD13-E5: the TOTAL spells per day — the pure sum of the two records
    // grounded above (base table count + Charisma bonus count) per
    // ACCESSIBLE spell level, mirroring the Sorcerer total slice. No new
    // rules content: each input record carries its own two-source
    // verification. Counts only — no spell selection, no
    // spontaneous-casting execution, no slot consumption or tracking, no
    // save resolution. Cantrips have no per-day column and no bonus, so no
    // total exists for them.
    for (index, base_count) in bard_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = (index + 1) as i16;
        let bonus_spells = if ability_modifiers.charisma < spell_level {
            0
        } else {
            (ability_modifiers.charisma - spell_level) / 4 + 1
        };
        let total_spells = base_count + bonus_spells;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.bard.spontaneous.total_spells_per_day.spell_level_{spell_level}"
            ),
            value: total_spells,
            detail: format!(
                "Bard total spells per day at bard level {level}, spell level \
                 {spell_level}: base table count {base_count} + Charisma bonus \
                 {bonus_spells} = {total_spells} — the pure sum of the two separately \
                 grounded records (each carrying its own two-source verification), giving \
                 the actual castable slot count per day. This grounds the count only: no \
                 spell selection (WHICH spells are known stays unproven), no \
                 spontaneous-casting execution, no slot consumption or tracking, and no \
                 spell save resolution"
            ),
        });
    }

    // SD13-E5: the three Versatile Performance choice slots (gates 2/6/10),
    // the last row of the repeat-grant queue — numbered slots per the proven
    // idiom, restricted-list recognitions over the nine verified Perform
    // types. The skill-SUBSTITUTION engine (using the Perform bonus in
    // place of the associated skills' bonuses) is exactly the choice-gated
    // engine burden the performance blocker names; no skill total is
    // modified and nothing is fabricated from these recognitions.
    for (slot_number, grant_level, choice_id) in BARD_VERSATILE_PERFORMANCE_SLOTS {
        if level < grant_level {
            continue;
        }
        let Some(selection) = choice_selection(input, choice_id) else {
            continue;
        };
        let record_id = if slot_number == 1 {
            "class_chassis.bard.versatile_performance_choice".to_owned()
        } else {
            format!("class_chassis.bard.versatile_performance_{slot_number}_choice")
        };
        let recognized = BARD_VERSATILE_PERFORMANCE_TYPES
            .iter()
            .find(|(sel, _, _)| *sel == selection);
        let detail = if let Some((_, name, pair)) = recognized {
            format!(
                "Bard Versatile Performance slot {slot_number} selection ({choice_id} -> \
                 {selection}) at the level-{grant_level} grant (PF1 Core Rulebook, verified \
                 identically on both primary sources: \"At 2nd level, a bard can choose one \
                 type of Perform skill... At 6th level, and every 4 levels thereafter, the \
                 bard can select an additional type of Perform to substitute.\"). The \
                 level-{level} selection names {name}, whose verified associated skills are \
                 {pair}. This is a +0 recognition record of the numbered choice slot only: \
                 the skill-substitution engine (using the Perform bonus in place of the \
                 associated skills' bonuses) is the named engine burden and no skill total \
                 is modified by this record"
            )
        } else {
            format!(
                "Bard Versatile Performance slot {slot_number} choice is present \
                 ({choice_id} -> {selection}), but only the nine verified PF1 Core Rulebook \
                 Perform types are recognized on this bounded seam; no Perform-type \
                 identity is grounded, no skill pair is named, and no skill total is \
                 modified (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: record_id,
            value: 0,
            detail,
        });
    }

    // v0.6 alpha swarm, risks item 8: the unconditional "known-spell / slot
    // posture burden" diagnostic that used to live here moved to
    // `ground_or_block_bard_known_spells`, called at the top of this
    // function -- the engine is real now (mirrors Sorcerer's own
    // spontaneous known-spell validation exactly), and its validity
    // depends on `spells_selected`, not on level/race alone, so it could
    // not stay a flat unconditional diagnostic at this position.
}

/// The highest ACCESSIBLE bard spell level (1st+) at the given bard level
/// -- cantrips (0th level) have no access gate and sit outside this
/// ladder. Pure function (v0.6 alpha swarm, risks item 8) so the
/// informational explanation record and the real known-spell validation
/// (`unmet_bard_known_spell_conditions`) share one source of truth,
/// mirroring `sorcerer_spell_level_access`'s own shape. Levels 11+ are not
/// yet verified beyond this bounded seam's existing level-1-10 table, so
/// this stays capped at 4 (the highest level this seam has verified data
/// for) -- a future widening must extend both this and
/// `bard_spells_known_table` together, the same split Sorcerer's own
/// bonus-spells-at-3rd+ level required.
pub(super) fn bard_spell_level_access(level: u8) -> i16 {
    if level >= BARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        4
    } else if level >= BARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        3
    } else if level >= BARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        2
    } else {
        1
    }
}

/// PF1 Core Rulebook Bard Spells Known table (0th through 4th spell
/// level), verified against the raw table rows of both primary sources.
/// Pure function (v0.6 alpha swarm, risks item 8) mirroring
/// `sorcerer_spells_known_table`'s own shape -- levels 11-20 return all
/// `None` (this seam's verified data stops at level 10 / spell level 4;
/// a future widening must extend this table for real, not silently
/// assume higher levels), which correctly still validates a genuinely
/// empty (zero known spells) posture as valid at any level, the same
/// "zero selected spells is always valid" idiom every prepared/known
/// caster class this session already uses.
pub(super) fn bard_spells_known_table(level: u8) -> [Option<i16>; 5] {
    match level {
        1 => [Some(4), Some(2), None, None, None],
        2 => [Some(5), Some(3), None, None, None],
        3 => [Some(6), Some(4), None, None, None],
        4 => [Some(6), Some(4), Some(2), None, None],
        5 => [Some(6), Some(4), Some(3), None, None],
        6 => [Some(6), Some(4), Some(4), None, None],
        7 => [Some(6), Some(5), Some(4), Some(2), None],
        8 => [Some(6), Some(5), Some(4), Some(3), None],
        9 => [Some(6), Some(5), Some(4), Some(4), None],
        10 => [Some(6), Some(5), Some(5), Some(4), Some(2)],
        _ => [None, None, None, None, None],
    }
}

/// Return the list of unmet conditions for the Bard's spontaneous
/// known-spell posture (v0.6 alpha swarm, risks item 8), mirroring
/// `unmet_sorcerer_known_spell_conditions` exactly (a spontaneous caster,
/// like Sorcerer -- known spells are permanent, not a per-day consumable
/// resource, unlike Ranger/Paladin/Cleric/Druid's prepared posture). An
/// empty list means the posture is genuinely valid -- zero known spells
/// is always a valid PF1 posture, not every Bard has selected their full
/// complement.
pub(super) fn unmet_bard_known_spell_conditions(input: &CharacterInput, bard_level: u8) -> Vec<String> {
    let mut unmet = Vec::new();

    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| s.source_class_id == BARD_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();

    let access_ceiling = bard_spell_level_access(bard_level);
    let known_table = bard_spells_known_table(bard_level);

    let mut known_per_level: [i16; 5] = [0; 5];
    for spell_id in &known {
        let Some(spell_level) = bard_spell_list::bard_spell_level(spell_id) else {
            unmet.push(format!(
                "known spell '{spell_id}' is not on the real PF1 bard spell list"
            ));
            continue;
        };
        if spell_level > 0 && i16::from(spell_level) > access_ceiling {
            unmet.push(format!(
                "known spell '{spell_id}' targets spell level {spell_level}, not yet accessible \
                 at bard level {bard_level} (access ceiling {access_ceiling})"
            ));
            continue;
        }
        if usize::from(spell_level) >= known_per_level.len() {
            unmet.push(format!(
                "known spell '{spell_id}' targets spell level {spell_level}, beyond this bounded \
                 seam's verified spells-known table (spell levels 0-4 only)"
            ));
            continue;
        }
        known_per_level[usize::from(spell_level)] += 1;
    }

    for (index, count) in known_per_level.iter().enumerate() {
        if *count == 0 {
            continue;
        }
        let cap = known_table[index].unwrap_or(0);
        if *count > cap {
            unmet.push(format!(
                "spell level {index} over-known: {count} distinct spells known but only {cap} \
                 slots available on the Bard Spells Known table"
            ));
        }
    }

    unmet
}

/// Ground the real known-spell posture once
/// `unmet_bard_known_spell_conditions` reports an empty unmet list,
/// mirroring `ground_sorcerer_known_spells` exactly.
pub(super) fn ground_bard_known_spells(
    input: &CharacterInput,
    bard_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| s.source_class_id == BARD_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known)
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.bard.known_spells".to_owned(),
        value: known.len() as i16,
        detail: format!(
            "Bard level {bard_level} known-spell selection ({} spells, AcquisitionMode::Known): \
             {}. Each known spell is verified against the real PF1 bard spell list \
             (`bard_spell_list::BARD_SPELL_LIST`, all ingested books), the bard's own \
             spell-level access ceiling, and \
             the Bard Spells Known table's own per-level cap. Real PF1 Bard rules have no daily \
             preparation step at all (like Sorcerer, unlike Ranger/Paladin/Cleric/Druid) -- a \
             bard's known spells are permanent once learned, cast spontaneously using the \
             already-grounded per-day slot totals. This grounds the known-spell selection for \
             real; it computes no spell save DC resolution against a target and no casting \
             execution",
            known.len(),
            known.join(", ")
        ),
    });
}

/// Grounds or claim-blocks Bard's spontaneous known-spell / per-day
/// posture burden for `bard_level` (v0.6 alpha swarm, risks item 8).
/// Called from the top of `explain_bard_level1_spell_baseline`, gated
/// only on Bard class-ownership -- independent of race/single-class
/// status, the exact gate-ordering fix this session has applied to every
/// other class's own spell/ability posture. Mirrors the Sorcerer known-
/// spell closure's shape exactly (a spontaneous caster, not prepared).
pub(super) fn ground_or_block_bard_known_spells(
    input: &CharacterInput,
    bard_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let unmet = unmet_bard_known_spell_conditions(input, bard_level);
    if unmet.is_empty() {
        ground_bard_known_spells(input, bard_level, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.bard.spontaneous_known_and_per_day.unsupported".to_owned(),
            message: format!(
                "Bard remains blocked on its spontaneous known-spell / slot posture burden: \
                 spontaneous casting execution (slot consumption, tracking, and casting itself) \
                 is out of scope for this bounded baseline regardless (the spell-level access \
                 ladder, the base spells per day table counts, the base spell-save-DC \
                 arithmetic, the base spells-known table counts, the Charisma bonus spell slot \
                 counts, and the integrated base+bonus totals are grounded separately as flat \
                 records); unmet known-spell posture: {}",
                unmet.join("; ")
            ),
            claim_blocking: true,
        });
    }
}

/// Bard's Bardic Performance rounds-per-day budget: 4 + Charisma modifier +
/// 2 * (level - 1), floored at 0 (PF1 Core Rulebook Bardic Performance:
/// "4 + his Charisma modifier ... at each level after 1st a bard can use
/// bardic performance for 2 additional rounds per day"). Pure function
/// (v0.6 alpha swarm, risks item 8) so the informational explanation
/// record inside `explain_bard_level1_spell_baseline` and the real
/// bardic-performance validation (`ground_or_block_bard_bardic_performance`,
/// `active_bard_inspire_courage_attack_bonus`) call the identical formula
/// rather than either duplicating it or parsing it back out of explanation
/// text.
pub(super) fn bard_bardic_performance_rounds_per_day(
    charisma_modifier: i16,
    level: u8,
    selected_feats: &[String],
) -> i16 {
    (4 + charisma_modifier
        + BARD_PERFORMANCE_ADDITIONAL_ROUNDS_PER_LEVEL * (i16::from(level) - 1))
    .max(0)
        + extra_resource_feat_bonus(
            selected_feats,
            EXTRA_PERFORMANCE_FEAT_KEY,
            EXTRA_ROUNDS_PER_DAY,
        )
}

/// Bard's Inspire Courage magnitude tier, rising from +1 at level 1 to +2
/// at `BARD_INSPIRE_COURAGE_SECOND_TIER_LEVEL`, +3 at
/// `BARD_INSPIRE_COURAGE_THIRD_TIER_LEVEL`, +4 at
/// `BARD_INSPIRE_COURAGE_FOURTH_TIER_LEVEL`. Pure function (v0.6 alpha
/// swarm, risks item 8) so the informational flat-magnitude explanation
/// record and the real attack-bonus application
/// (`active_bard_inspire_courage_attack_bonus`) share one source of truth.
pub(super) fn bard_inspire_courage_bonus(level: u8) -> i16 {
    if level >= BARD_INSPIRE_COURAGE_FOURTH_TIER_LEVEL {
        BARD_INSPIRE_COURAGE_BONUS_FOURTH_TIER
    } else if level >= BARD_INSPIRE_COURAGE_THIRD_TIER_LEVEL {
        BARD_INSPIRE_COURAGE_BONUS_THIRD_TIER
    } else if level >= BARD_INSPIRE_COURAGE_SECOND_TIER_LEVEL {
        BARD_INSPIRE_COURAGE_BONUS_SECOND_TIER
    } else {
        BARD_INSPIRE_COURAGE_BONUS_FIRST_TIER
    }
}

/// Whether `input` is a Bard validly, actively performing Inspire Courage
/// right now, and if so, the attack/damage competence bonus to apply (v0.6
/// alpha swarm, risks item 8). Class-ownership-gated by construction: only
/// returns `Some` when `class_levels` actually contains Bard, so a
/// non-Bard character's stray `class_ability_activations` entry for
/// `BARD_BARDIC_PERFORMANCE_ABILITY_ID` is never read at all (mirrors the
/// Barbarian Rage spoofed-activation shape exactly). The schema has no
/// field distinguishing which performance is active, so an active,
/// in-budget `"bardic_performance"` entry is interpreted as Inspire
/// Courage specifically -- the only performance this bounded slice models
/// (matches the pre-existing `inspire_courage_bonus` explanation record's
/// own assumption). An activation present but not
/// `ActiveState::EquippedActive`, or one that exceeds the grounded
/// rounds-per-day budget, is treated the same as "not performing" here --
/// pushes no diagnostic itself (`ground_or_block_bard_bardic_performance`
/// is the single place that pushes the over-budget claim-blocking
/// diagnostic and the informational recognition records).
pub(super) fn active_bard_inspire_courage_attack_bonus(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
) -> Option<(u8, i16)> {
    let bard_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == BARD_CLASS_ID)
        .map(|class_level| class_level.level)?;

    let activation = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == BARD_BARDIC_PERFORMANCE_ABILITY_ID)?;

    if activation.active_state != ActiveState::EquippedActive {
        return None;
    }

    if let Some(rounds_consumed) = activation.rounds_consumed_today {
        let charisma_modifier = ability_modifier_for(ability_modifiers, "charisma");
        let rounds_per_day = bard_bardic_performance_rounds_per_day(charisma_modifier, bard_level, &input.chosen.selected_feats);
        if i32::from(rounds_consumed) > i32::from(rounds_per_day) {
            return None;
        }
    }

    Some((bard_level, bard_inspire_courage_bonus(bard_level)))
}

/// Grounds or claim-blocks Bard's Bardic Performance execution engine for
/// `bard_level` (v0.6 alpha swarm, risks item 8). Called from the top of
/// `explain_bard_level1_spell_baseline`, gated only on Bard class-ownership
/// -- independent of race/single-class status, mirroring the Barbarian
/// Rage gate-ordering fix exactly.
///
/// A character who simply isn't performing (no `class_ability_activations`
/// entry for `BARD_BARDIC_PERFORMANCE_ABILITY_ID`, or one present but
/// `active_state != EquippedActive`) is a genuinely valid PF1 posture --
/// not every Bard is always performing -- so this grounds a real "not
/// performing" recognition record rather than claim-blocking, mirroring
/// "zero prepared spells is always valid" / "not raging is always valid"
/// from the earlier classes. An activation that IS active but exceeds the
/// grounded rounds-per-day budget is a genuine posture violation and
/// claim-blocks, mirroring every other over-budget check landed this
/// session. Separately, an unconditional, NON-blocking diagnostic always
/// names the six other bardic performances (Countersong, Distraction,
/// Versatile Performance, Soothing Performance, Frightening Tune, Inspire
/// Heroics, Deadly Performance) as permanently unmodeled -- the schema has
/// no way to represent using any of them, so this is a genuine, honest,
/// non-blocking capability gap rather than a stub.
pub(super) fn ground_or_block_bard_bardic_performance(
    input: &CharacterInput,
    bard_level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.bard.bardic_performance_execution.other_performances_not_modeled"
            .to_owned(),
        message: format!(
            "Bard level {bard_level}: only Inspire Courage's mechanics are modeled among \
             bardic performances -- the schema has no field distinguishing which performance is \
             active, so Countersong, Distraction, Versatile Performance, Soothing Performance, \
             Frightening Tune, Inspire Heroics, and Deadly Performance remain entirely \
             unexecuted regardless of activation state (each requires its own opposed-check, \
             skill-substitution, healing, condition-removal, or death-effect-resolution engine, \
             none of which exists in this codebase). Named honestly rather than silently \
             modeled or silently dropped; this does not block a genuinely valid Inspire Courage \
             posture"
        ),
        claim_blocking: false,
    });

    let Some(activation) = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == BARD_BARDIC_PERFORMANCE_ABILITY_ID)
    else {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.bardic_performance_execution.not_performing".to_owned(),
            value: 0,
            detail: format!(
                "Bard level {bard_level} is not currently performing (no \
                 class_ability_activations entry for \"{BARD_BARDIC_PERFORMANCE_ABILITY_ID}\"): \
                 a genuinely valid PF1 posture, so no performance bonus or budget is claimed. \
                 This grounds the bardic-performance-execution engine's \"inactive\" branch \
                 only; entering Inspire Courage is grounded separately below when an active, \
                 in-budget activation is present"
            ),
        });
        return;
    };

    let charisma_modifier = ability_modifier_for(ability_modifiers, "charisma");
    let rounds_per_day = bard_bardic_performance_rounds_per_day(charisma_modifier, bard_level, &input.chosen.selected_feats);

    if let Some(rounds_consumed) = activation.rounds_consumed_today
        && i32::from(rounds_consumed) > i32::from(rounds_per_day) {
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.bard.bardic_performance_execution.rounds_exceeded".to_owned(),
                message: format!(
                    "Bard level {bard_level} bardic performance activation claims \
                     {rounds_consumed} rounds consumed today, exceeding the grounded \
                     rounds-per-day budget of {rounds_per_day} (4 + Charisma modifier \
                     ({charisma_modifier}) + 2 * (level - 1), floored at 0): a genuine posture \
                     violation, so no performance bonus is claimed for this input"
                ),
                claim_blocking: true,
            });
            return;
        }

    match activation.active_state {
        ActiveState::EquippedActive => {
            let bonus = bard_inspire_courage_bonus(bard_level);
            let rounds_consumed_today = activation.rounds_consumed_today.unwrap_or(0);
            explanations.push(ComputationExplanation {
                id: "class_feature.bard.bardic_performance_execution.active".to_owned(),
                value: bonus,
                detail: format!(
                    "Bard level {bard_level} is actively performing Inspire Courage, within \
                     the grounded rounds-per-day budget ({rounds_per_day} rounds; \
                     {rounds_consumed_today} consumed today). The +{bonus} competence bonus on \
                     attack rolls is applied to the integrated baseline melee attack bonus -- \
                     see compute_combat_baseline. The matching +{bonus} competence bonus on \
                     weapon damage rolls and the +{bonus} morale bonus on saves against charm \
                     and fear effects are not applied to any integrated total: this codebase has \
                     no integrated weapon-damage total and no save-vs-effect-type sub-category \
                     to apply either to, so both stay flat, non-fabricated magnitudes only \
                     (see class_chassis.bard.inspire_courage_bonus)"
                ),
            });
        }
        ActiveState::SelectedInactive | ActiveState::Absent => {
            explanations.push(ComputationExplanation {
                id: "class_feature.bard.bardic_performance_execution.not_performing".to_owned(),
                value: 0,
                detail: format!(
                    "Bard level {bard_level} has a \"{BARD_BARDIC_PERFORMANCE_ABILITY_ID}\" \
                     activation entry but it is not active for this snapshot: a genuinely valid \
                     PF1 posture (available but not currently performing), so no performance \
                     bonus or budget is claimed"
                ),
            });
        }
    }
}

/// v0.6 alpha swarm, risks item 8, first APG/ACG closure (2026-07-25):
/// Skald's Inspired Rage mirrors Barbarian's Rage execution shape exactly
/// (same four-value STR/CON/Will/AC pattern), but UNLIKE every
/// `<class>_dispatch_widening_safety_tests` module since Barbarian's own,
/// a valid Skald posture never reaches `Computed` -- Skald's own
/// spellcasting and every other named-but-unbuilt class feature stay
/// permanently claim-blocked via `class_feature.acg.skald.spellcasting_deferred.unsupported`,
/// the same permanent-burden shape Sorcerer's own bloodline diagnostic
/// keeps regardless of Arcane Bond's own resolution. This module also
/// exercises the genuinely new chassis-integration gate
/// (`is_supported_skald_single_class`) itself, not just the Rage-mirroring
/// pillar work.
#[cfg(test)]
mod skald_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, ActiveState, CharacterClassLevel,
        CharacterInput, HeadlessReceiptStatus, FIGHTER_CLASS_ID, SKALD_CLASS_ID,
        SKALD_INSPIRED_RAGE_ABILITY_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_skald_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SKALD_CLASS_ID.to_owned(), level }];
        input
    }

    /// A single-class Human Skald who is not singing Raging Song (no
    /// `class_ability_activations` entry at all) is a genuinely valid PF1
    /// posture -- reaches only as far as this slice allows, i.e. `Blocked`
    /// on the new, narrower spellcasting_deferred diagnostic alone (never
    /// the retired generic one), with the honest "not singing" recognition
    /// record grounded.
    ///
    /// **Task #91 flips the status assertion** -- Skald's last named
    /// features are grounded, so the class computes. The point this test
    /// actually protects is unchanged and still asserted below: not
    /// singing is a valid posture that produces an honest recognition
    /// record, not a diagnostic.
    #[test]
    fn single_class_skald_not_singing_computes_with_an_honest_not_singing_record() {
        let input = human_skald_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "not singing is a valid posture and must not block: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.skald.inspired_rage_execution.not_singing"),
            "expected the honest not-singing recognition record: {:?}",
            receipt.computation.explanations
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.skald.unsupported"),
            "the retired generic diagnostic must never appear for Skald: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.skald.other_features_deferred.unsupported"
                    && !d.claim_blocking),
            "the remainder record must survive as a NON-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Human Skald actively, validly singing Inspired Rage
    /// (within budget) applies the real Strength/Constitution/Will/Armor-
    /// Class bonuses and penalty to the integrated totals -- but still
    /// stays `Blocked` (other_features_deferred), unlike Barbarian's own
    /// equivalent test which reaches `Computed`.
    ///
    /// Charisma 8 (fixture base) -> -1 modifier: rounds per day = 3 + (-1)
    /// + 2*(1-1) = 2, so only 1-2 rounds consumed today stays in budget.
    ///
    /// **Task #91 flips the status assertion** -- with Skald's last named
    /// features grounded the class computes. What this test exists to
    /// protect is the bonus integration below, which is unchanged.
    #[test]
    fn single_class_skald_actively_singing_in_budget_applies_real_bonuses() {
        let mut input = human_skald_input(1);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: SKALD_INSPIRED_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(1),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "an in-budget, actively singing Skald is a fully valid posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.skald.other_features_deferred.unsupported"
                    && !d.claim_blocking),
            "expected the non-blocking remainder record even while singing: {:?}",
            receipt.computation.diagnostics
        );

        // Base fixture is Strength 16 (+4 with the fixture's chosen Human +2
        // floating Strength bonus applied), Constitution 14 (+2). Inspired
        // Rage (level 1) adds +2 Strength / +2 Constitution ability SCORE,
        // i.e. +1/+1 ability MODIFIER.
        assert_eq!(receipt.computation.ability_modifiers.strength, 5);
        assert_eq!(receipt.computation.ability_modifiers.constitution, 3);

        let will_save = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.total_save.will")
            .expect("total Will save must be grounded");
        // Base Will save (Skald level 1, good Will: 2) + Wisdom modifier
        // (12 -> +1) + feat bonus (0) + Inspired Rage Will bonus (+1) = 4.
        assert_eq!(
            will_save.value, 4,
            "Inspired Rage's Will-save morale bonus must be applied: {:?}",
            will_save
        );

        let armor_class = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .expect("baseline Armor Class must be grounded");
        // Base AC (10 + Chain Shirt 4 + DEX +2 + Dodge 1 = 17) - Inspired
        // Rage penalty (1) = 16.
        assert_eq!(
            armor_class.value, 16,
            "Inspired Rage's Armor Class penalty must be applied: {:?}",
            armor_class
        );
    }

    /// An Inspired Rage activation that exceeds the grounded rounds-per-day
    /// budget is a genuine posture violation and must claim-block -- never
    /// silently capped, mirroring every other over-budget check landed
    /// this session.
    #[test]
    fn single_class_skald_over_budget_inspired_rage_stays_blocked_and_applies_no_bonus() {
        let mut input = human_skald_input(1);
        // Charisma 8 (-1): rounds per day = 3 + (-1) + 2*(1-1) = 2.
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: SKALD_INSPIRED_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(3),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.acg.skald.inspired_rage_execution.rounds_exceeded"
                    && d.claim_blocking),
            "expected the over-budget claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert_eq!(
            receipt.computation.ability_modifiers.strength, 4,
            "no Inspired Rage bonus is applied for an over-budget, invalid posture: {:?}",
            receipt.computation.ability_modifiers
        );
    }

    /// A non-Skald character carrying a spoofed `"inspired_rage"`
    /// activation entry must have it silently ignored, not applied -- the
    /// class-ownership gate is by construction
    /// (`active_skald_inspired_rage_bonus` only ever reads
    /// `class_ability_activations` after confirming `class_levels` contains
    /// Skald), not a bolt-on rejection. Also proves Fighter's own golden
    /// path (including reaching `Computed`) is unaffected by a stray Skald
    /// activation entry.
    #[test]
    fn non_skald_characters_spoofed_inspired_rage_activation_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: SKALD_INSPIRED_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Skald inspired_rage entry: \
             {:?}",
            receipt.computation.diagnostics
        );
        assert_eq!(
            receipt.computation.ability_modifiers.strength, 4,
            "a non-Skald character's spoofed inspired_rage entry must never apply a bonus: {:?}",
            receipt.computation.ability_modifiers
        );
    }

    /// v0.6 alpha swarm, risks item 8 (Skald spellcasting closure): a
    /// single-class Skald knowing 2 real, valid, in-budget 1st-level
    /// spells (within skald level 1's real cap of 2, byte-identical to
    /// Bard's own cap) must NOT trip the spell-posture diagnostic --
    /// proving real validation, not a vacuous check, mirroring Bard's own
    /// known-spell closure exactly (same spell names too, since Skald
    /// casts from the same Bard spell list).
    #[test]
    fn single_class_skald_with_valid_known_spells_does_not_trip_the_spell_posture() {
        let mut input = human_skald_input(1);
        for spell_id in ["Alarm", "Charm Person"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: SKALD_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.skald.spontaneous_known_and_per_day.unsupported"),
            "2 distinct first-level spells is within skald level 1's real cap of 2: {:?}",
            receipt.computation.diagnostics
        );
        let known = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.skald.known_spells")
            .expect("expected the real known-spell posture to be grounded");
        assert_eq!(known.value, 2, "expected 2 known spells grounded: {:?}", known);
    }

    /// A single-class Skald at level 1 (cap 2 first-level spells known)
    /// knowing 3 distinct first-level spells over-knows its real cap and
    /// must carry the spell-posture diagnostic.
    #[test]
    fn single_class_skald_over_known_spells_stays_blocked_on_spell_posture() {
        let mut input = human_skald_input(1);
        for spell_id in ["Alarm", "Charm Person", "Cause Fear"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: SKALD_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.skald.spontaneous_known_and_per_day.unsupported"
                    && d.claim_blocking),
            "skald level 1 only knows 2 first-level spells; 3 distinct known spells over-knows \
             the real cap: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Skald preparing a spell not on the real Bard spell
    /// list Skald casts from must carry the diagnostic.
    #[test]
    fn single_class_skald_with_an_off_list_known_spell_stays_blocked_on_spell_posture() {
        let mut input = human_skald_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Not A Real Spell".to_owned(),
            source_class_id: SKALD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.skald.spontaneous_known_and_per_day.unsupported"
                    && d.claim_blocking),
            "an off-list spell must trip the spell-posture diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Skald knowing a real 2nd-level bard spell at level 1
    /// (2nd-level access begins at level 4, identical to Bard's own
    /// ladder) must carry the diagnostic -- proving the access-ceiling
    /// check, not just the off-list/over-known checks.
    #[test]
    fn single_class_skald_with_an_inaccessible_known_spell_stays_blocked() {
        let mut input = human_skald_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Alter Self".to_owned(),
            source_class_id: SKALD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.skald.spontaneous_known_and_per_day.unsupported"
                    && d.claim_blocking),
            "a 2nd-level bard spell is not accessible at skald level 1: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The flat spellcasting chassis records (access ladder, base
    /// spells-per-day, spell save DC) are grounded for real, mirroring
    /// Bard's own reference values exactly (byte-identical tables).
    /// Charisma 8 (fixture base) -> -1 modifier.
    #[test]
    fn single_class_skald_grounds_the_flat_spellcasting_chassis_records() {
        let input = human_skald_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        let access = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.skald.spontaneous.spell_level_access")
            .expect("expected the spell-level access ladder to be grounded");
        assert_eq!(access.value, 1, "skald level 1 access ceiling is 1st level: {:?}", access);

        let base_per_day = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.skald.spontaneous.base_spells_per_day.spell_level_1")
            .expect("expected the base spells-per-day record to be grounded");
        assert_eq!(base_per_day.value, 1, "skald level 1 base 1st-level spells per day: {:?}", base_per_day);

        let dc = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.skald.spontaneous.spell_save_dc.spell_level_1")
            .expect("expected the spell save DC record to be grounded");
        // 10 + spell level 1 + Charisma modifier (-1) = 10.
        assert_eq!(dc.value, 10, "10 + 1 + (-1) = 10: {:?}", dc);
    }

    /// Skald's own self-Damage Reduction (deepening 2026-07-26, task #7)
    /// is honestly absent below level 9 -- a real PF1 level gate, not an
    /// omission -- mirroring Barbarian's own "not yet gained" discipline
    /// for its identically-shaped Damage Reduction feature.
    #[test]
    fn single_class_skald_damage_reduction_is_honestly_absent_below_level_9() {
        let input = human_skald_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        let dr = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.skald.damage_reduction")
            .expect("expected the Damage Reduction record to be grounded even when absent");
        assert_eq!(dr.value, 0, "level 1 Skald has no Damage Reduction yet: {:?}", dr);
    }

    /// Skald's own self-Damage Reduction progresses 1/9th, 2/14th, 3/19th,
    /// unconditional on class ownership and level alone (no choice or
    /// activation gate, unlike Inspired Rage), and never claim-blocks.
    #[test]
    fn single_class_skald_damage_reduction_progresses_at_the_real_level_gates() {
        for (level, expected) in [(8, 0), (9, 1), (13, 1), (14, 2), (18, 2), (19, 3), (20, 3)] {
            let input = human_skald_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let dr = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.skald.damage_reduction")
                .unwrap_or_else(|| panic!("expected Damage Reduction grounded at level {level}"));
            assert_eq!(dr.value, expected, "level {level} Damage Reduction: {:?}", dr);

            assert!(
                !receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id.contains("damage_reduction") && d.claim_blocking),
                "Damage Reduction must never claim-block at level {level}: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// The other_features_deferred diagnostic now acknowledges Damage
    /// Reduction (self-only) as grounded in its own preamble, while still
    /// naming the ally-extension as the genuinely still-missing half --
    /// mirroring Brawler's own "acknowledges Cunning and Strike as
    /// grounded" pattern.
    #[test]
    fn other_features_deferred_acknowledges_self_damage_reduction_as_grounded() {
        let input = human_skald_input(9);
        let receipt = build_pilot_headless_receipt(&input);

        let deferred = receipt
            .computation
            .diagnostics
            .iter()
            .find(|d| d.id == "class_feature.acg.skald.other_features_deferred.unsupported")
            .expect("expected the other_features_deferred diagnostic");
        assert!(
            deferred.message.contains("self-only Damage Reduction"),
            "expected the preamble to acknowledge Damage Reduction as grounded: {}",
            deferred.message
        );
        assert!(
            deferred.message.contains("ally-extension"),
            "expected the still-missing ally-extension to be named: {}",
            deferred.message
        );
    }

    /// Skald's own Bardic Knowledge (deepening 2026-07-26, task #7) is
    /// grounded as a standalone flat competence bonus, unconditional on
    /// class ownership and level alone, mirroring Bard's own Bardic
    /// Knowledge and Inquisitor's own Monster Lore (task #18) -- both
    /// already established that a real magnitude needs no live consumer.
    #[test]
    fn single_class_skald_gets_the_unconditional_bardic_knowledge_bonus() {
        for (level, expected) in [(1, 1), (2, 1), (3, 1), (4, 2), (10, 5), (20, 10)] {
            let input = human_skald_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let bardic_knowledge = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.skald.bardic_knowledge_bonus")
                .unwrap_or_else(|| panic!("expected Bardic Knowledge grounded at level {level}"));
            assert_eq!(
                bardic_knowledge.value, expected,
                "level {level} Bardic Knowledge: {:?}",
                bardic_knowledge
            );
        }
    }

    /// The other_features_deferred diagnostic now also acknowledges
    /// Bardic Knowledge as grounded, no longer naming it as missing.
    #[test]
    fn other_features_deferred_acknowledges_bardic_knowledge_as_grounded() {
        let input = human_skald_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        let deferred = receipt
            .computation
            .diagnostics
            .iter()
            .find(|d| d.id == "class_feature.acg.skald.other_features_deferred.unsupported")
            .expect("expected the other_features_deferred diagnostic");
        assert!(
            deferred.message.contains("Bardic Knowledge,"),
            "expected the preamble to acknowledge Bardic Knowledge as grounded: {}",
            deferred.message
        );
        assert!(
            !deferred.message.contains("Bardic Knowledge-analog"),
            "the stale 'still missing' Bardic Knowledge-analog phrasing must be gone: {}",
            deferred.message
        );
    }

    /// Task #54: Raging Climber/Raging Swimmer's own self-use magnitude is
    /// now grounded and no longer named as missing.
    ///
    /// **Reconciled with task #50 (rebased on top of #54, same day)**:
    /// the wider Rage Powers pool-count formula is ALSO now grounded (see
    /// `ground_skald_rage_powers_pool_size`), so it is no longer named as
    /// deferred either -- only the ally-granting "shared-list access"
    /// (applying chosen rage powers to allies via Raging Song) and the
    /// other 58 named-but-unmodeled rage powers genuinely stay deferred.
    #[test]
    fn other_features_deferred_acknowledges_raging_climber_and_swimmer_but_not_the_wider_rage_powers_feature()
    {
        let input = human_skald_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        let deferred = receipt
            .computation
            .diagnostics
            .iter()
            .find(|d| d.id == "class_feature.acg.skald.other_features_deferred.unsupported")
            .expect("expected the other_features_deferred diagnostic");
        assert!(
            deferred.message.contains("Raging Climber and Raging Swimmer"),
            "expected the message to credit Raging Climber/Raging Swimmer as grounded: {}",
            deferred.message
        );
        assert!(
            deferred.message.contains("ally-granting shared-list access")
                && deferred.message.contains("2 of its 60 loaded records"),
            "expected the wider Rage Powers feature (minus the now-grounded pool-count and the \
             two grounded powers) to still be named as deferred: {}",
            deferred.message
        );
        assert!(
            !deferred.message.contains("Rage Powers pool-count"),
            "the pool-count is now grounded too (task #50) and must no longer be named as \
             deferred: {}",
            deferred.message
        );
        // Task #91: the Rage Powers catalog gap is a chooser-narrowing
        // remainder, not a blocker -- Skald computes with it open.
        assert!(
            !deferred.claim_blocking,
            "a catalog gap in a chooser list must not block the class: {deferred:?}"
        );
    }

    /// Raging Song's rounds-per-day pool is the SAME pool Inspired Rage's
    /// budget enforcement spends from -- one shared pool, per PF1 -- so
    /// the two records must always agree. A second, independently derived
    /// figure here would assert a pool that does not exist.
    ///
    /// It also follows the rule text, not the corpus's defective
    /// `3+CHA+(2*SkaldLVL)` token, which is 2 high at every level. The
    /// fixture's Charisma 8 (-1) makes that difference visible: 2 rather
    /// than 4 at level 1.
    #[test]
    fn raging_song_rounds_per_day_shares_the_inspired_rage_pool_and_follows_the_rule_text() {
        for (level, want) in [(1u8, 2i16), (2, 4), (3, 6), (20, 40)] {
            assert_eq!(
                super::skald_inspired_rage_rounds_per_day(-1, level, &[]),
                want,
                "level {level} with Charisma modifier -1"
            );
        }

        for level in [1u8, 5, 20] {
            let receipt = build_pilot_headless_receipt(&human_skald_input(level));
            let rounds = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.skald.raging_song_rounds_per_day")
                .unwrap_or_else(|| panic!("Raging Song rounds must ground at level {level}"));
            assert_eq!(
                rounds.value,
                super::skald_inspired_rage_rounds_per_day(-1, level, &[]),
                "Raging Song and Inspired Rage must report ONE shared pool at level {level}"
            );
        }
    }

    /// Cantrips known come from the level-0 column of the KNOWN table
    /// (4/5/6), NOT from the paired CAST column, which is 0 at every
    /// level because cantrips are unlimited. Reading CAST would report a
    /// Skald who can cast none.
    #[test]
    fn skald_cantrips_read_the_known_column_not_the_unlimited_cast_column() {
        for (level, want) in [(1u8, 4i16), (2, 5), (3, 6), (20, 6)] {
            assert_eq!(super::skald_cantrips_known(level), want, "level {level}");
        }
        let receipt = build_pilot_headless_receipt(&human_skald_input(1));
        let cantrips = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.skald.cantrips_known")
            .expect("Cantrips ground from level 1");
        assert_eq!(cantrips.value, 4);
        assert_ne!(cantrips.value, 0, "0 would mean the CAST column was read instead");
    }

    /// Song of Strength carries a REAL magnitude (half the skald's
    /// level) that exists only in DESC prose, so it must not be filed
    /// with the zero-magnitude features. The six that genuinely are
    /// zero-magnitude ground at their own gates with value 0, and each
    /// must cite its Skald-namespaced key -- a bare name grep would have
    /// hit Barbarian's, Rogue's or Bard's same-named records instead.
    #[test]
    fn skald_zero_magnitude_features_gate_correctly_and_song_of_strength_is_not_among_them() {
        let song_of_strength = "class_feature.acg.skald.song_of_strength_bonus";
        assert!(
            !build_pilot_headless_receipt(&human_skald_input(5))
                .computation
                .explanations
                .iter()
                .any(|e| e.id == song_of_strength),
            "Song of Strength is a 6th-level feature"
        );
        for (level, want) in [(6u8, 3i16), (7, 3), (20, 10)] {
            let receipt = build_pilot_headless_receipt(&human_skald_input(level));
            assert_eq!(
                receipt
                    .computation
                    .explanations
                    .iter()
                    .find(|e| e.id == song_of_strength)
                    .map(|e| e.value),
                Some(want),
                "Song of Strength at level {level} is half the skald's level"
            );
        }

        for (gate, name) in [
            (3u8, "song_of_marching"),
            (4, "uncanny_dodge"),
            (8, "improved_uncanny_dodge"),
            (10, "dirge_of_doom"),
            (14, "song_of_the_fallen"),
            (20, "master_skald"),
        ] {
            let id = format!("class_feature.acg.skald.{name}_grant");
            assert!(
                !build_pilot_headless_receipt(&human_skald_input(gate - 1))
                    .computation
                    .explanations
                    .iter()
                    .any(|e| e.id == id),
                "{id} must not exist below its level-{gate} gate"
            );
            let receipt = build_pilot_headless_receipt(&human_skald_input(gate));
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("{id} must ground at level {gate}"));
            assert_eq!(record.value, 0, "{id} is genuinely zero-magnitude");
            assert!(
                record.detail.contains("KEY:Skald ~ "),
                "{id} must cite its Skald-namespaced corpus key so it can never be confused \
                 with Barbarian's, Rogue's or Bard's same-named record: {}",
                record.detail
            );
        }
    }

    /// Skald's own Well-Versed (task #50) is honestly absent below level 2
    /// -- a real PF1 level gate, confirmed against the real corpus's own
    /// per-level `ABILITY:...Skald ~ Well-Versed` grant row -- then grounds
    /// a flat, non-level-scaled +4, byte-identical to Bard's own
    /// Well-Versed magnitude.
    #[test]
    fn single_class_skald_well_versed_self_gates_at_level_2_and_grounds_a_flat_4() {
        for (level, expected) in [(1, 0), (2, 4), (3, 4), (20, 4)] {
            let input = human_skald_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let well_versed = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.skald.well_versed")
                .unwrap_or_else(|| panic!("expected Well-Versed grounded at level {level}"));
            assert_eq!(well_versed.value, expected, "level {level} Well-Versed: {:?}", well_versed);

            assert!(
                !receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id.contains("well_versed") && d.claim_blocking),
                "Well-Versed must never claim-block at level {level}: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// Skald's own Spell Kenning uses-per-day (task #50): `(1+SkaldLVL)/6`
    /// self-gates to 0 below level 5, matching the real corpus's own
    /// per-level grant row, then rises to 1/day at 5th, 2/day at 11th,
    /// 3/day at 17th -- verified directly against the raw corpus DESC
    /// text's own schedule.
    #[test]
    fn single_class_skald_spell_kenning_self_gates_and_progresses_at_the_real_level_gates() {
        for (level, expected) in
            [(1, 0), (4, 0), (5, 1), (10, 1), (11, 2), (16, 2), (17, 3), (20, 3)]
        {
            let input = human_skald_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let spell_kenning = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.skald.spell_kenning_uses_per_day")
                .unwrap_or_else(|| panic!("expected Spell Kenning grounded at level {level}"));
            assert_eq!(
                spell_kenning.value, expected,
                "level {level} Spell Kenning: {:?}",
                spell_kenning
            );

            assert!(
                !receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id.contains("spell_kenning") && d.claim_blocking),
                "Spell Kenning must never claim-block at level {level}: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// Skald's own Lore Master uses-per-day (task #50):
    /// `min((SkaldLVL-1)/6,3)`, a genuine two-argument `min()`, self-gates
    /// to 0 below level 7, matching the real corpus's own per-level grant
    /// row, then rises to 1/day at 7th, 2/day at 13th, and caps at 3/day
    /// at 19th.
    #[test]
    fn single_class_skald_lore_master_self_gates_and_caps_at_three_per_day() {
        for (level, expected) in
            [(1, 0), (6, 0), (7, 1), (12, 1), (13, 2), (18, 2), (19, 3), (20, 3)]
        {
            let input = human_skald_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let lore_master = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.skald.lore_master_uses_per_day")
                .unwrap_or_else(|| panic!("expected Lore Master grounded at level {level}"));
            assert_eq!(lore_master.value, expected, "level {level} Lore Master: {:?}", lore_master);

            assert!(
                !receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id.contains("lore_master") && d.claim_blocking),
                "Lore Master must never claim-block at level {level}: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// Skald's own Versatile Performance slot count (task #50):
    /// `(SkaldLVL+3)/5`, a genuinely single-argument `min()` in the raw
    /// corpus (ground the term itself, not an invented second cap
    /// operand). Self-gates to 0 below level 2, matching the real corpus's
    /// own per-level grant row and the DESC's own schedule: 1 slot at
    /// level 2-6, 2 at 7-11, 3 at 12-16, continuing to grow every 5 levels
    /// thereafter.
    #[test]
    fn single_class_skald_versatile_performance_self_gates_and_grows_every_five_levels() {
        for (level, expected) in
            [(1, 0), (2, 1), (6, 1), (7, 2), (11, 2), (12, 3), (16, 3), (17, 4), (20, 4)]
        {
            let input = human_skald_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let versatile_performance = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.skald.versatile_performance_slot_count")
                .unwrap_or_else(|| {
                    panic!("expected Versatile Performance grounded at level {level}")
                });
            assert_eq!(
                versatile_performance.value, expected,
                "level {level} Versatile Performance: {:?}",
                versatile_performance
            );

            assert!(
                !receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id.contains("versatile_performance") && d.claim_blocking),
                "Versatile Performance must never claim-block at level {level}: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// Skald's own Rage Powers pool SIZE (task #50, distinct from the
    /// actual rage-power selection/execution built separately): `SkaldLVL
    /// / 3` (`RagePowersLVL` is set unconditionally from `SkaldLVL` in the
    /// real corpus, no borrowed-variable gap). Self-gates to 0 below level
    /// 3, matching the real corpus's own per-level grant row and the
    /// DESC's own "at 3rd level and every 3 levels thereafter" schedule.
    #[test]
    fn single_class_skald_rage_powers_pool_size_self_gates_and_grows_every_three_levels() {
        for (level, expected) in [(1, 0), (2, 0), (3, 1), (5, 1), (6, 2), (8, 2), (9, 3), (20, 6)] {
            let input = human_skald_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let rage_powers = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.skald.rage_powers_pool_size")
                .unwrap_or_else(|| panic!("expected Rage Powers pool size grounded at level {level}"));
            assert_eq!(rage_powers.value, expected, "level {level} Rage Powers pool size: {:?}", rage_powers);

            assert!(
                !receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id.contains("rage_powers") && d.claim_blocking),
                "Rage Powers pool size must never claim-block at level {level}: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// The other_features_deferred diagnostic (task #50) now also
    /// acknowledges Well-Versed, Spell Kenning, Lore Master, Versatile
    /// Performance, and Rage Powers' own pool size as grounded, while
    /// still naming the genuinely still-missing execution pieces (Spell
    /// Kenning's spell-borrowing, Lore Master's take-10/20, Versatile
    /// Performance's Perform-type choice, Rage Powers' individual-power
    /// selection) and the four remaining tokenless Raging Song variants.
    #[test]
    fn other_features_deferred_acknowledges_the_five_new_flat_magnitudes_as_grounded() {
        let input = human_skald_input(20);
        let receipt = build_pilot_headless_receipt(&input);

        let deferred = receipt
            .computation
            .diagnostics
            .iter()
            .find(|d| d.id == "class_feature.acg.skald.other_features_deferred.unsupported")
            .expect("expected the other_features_deferred diagnostic");
        for acknowledged in [
            "Well-Versed",
            "Spell Kenning",
            "Lore Master",
            "Versatile Performance",
            "Rage Powers",
        ] {
            assert!(
                deferred.message.contains(acknowledged),
                "expected the preamble to acknowledge {acknowledged} as grounded: {}",
                deferred.message
            );
        }
        assert!(
            deferred.message.contains("Dirge of Doom")
                && deferred.message.contains("Song of Marching")
                && deferred.message.contains("Song of Strength")
                && deferred.message.contains("Song of the Fallen"),
            "expected the four tokenless Raging Song variants to still be named as missing: {}",
            deferred.message
        );
        assert!(
            !deferred.message.contains(
                "Lore Master, Rage Powers shared-list access, Spell Kenning, Versatile \
                 Performance, Well-Versed,"
            ),
            "the stale flat 'still missing' listing for these five must be gone: {}",
            deferred.message
        );
    }
}

/// v0.6 alpha swarm, risks item 8: Bard's Inspire Courage is the second
/// class to exercise the combat-time activation-state pattern Barbarian's
/// cycle proved out. Unlike Barbarian, Bard still carries a SEPARATE
/// permanent diagnostic (`class_spell.bard.spontaneous_known_and_per_day.unsupported`,
/// out of scope for this slice), so a valid Inspire Courage posture stays
/// `Blocked` overall -- these tests check the bardic-performance-execution
/// diagnostic and the real attack-bonus application specifically, not the
/// overall receipt status.
#[cfg(test)]
mod bard_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, ActiveState, AcquisitionMode, CharacterClassLevel,
        CharacterInput, BARD_BARDIC_PERFORMANCE_ABILITY_ID, BARD_CLASS_ID, FIGHTER_CLASS_ID,
        HeadlessReceiptStatus,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_bard_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: BARD_CLASS_ID.to_owned(), level }];
        input
    }

    fn claim_blocking_ids(input: &CharacterInput) -> Vec<String> {
        build_pilot_headless_receipt(input)
            .computation
            .diagnostics
            .into_iter()
            .filter(|d| d.claim_blocking)
            .map(|d| d.id)
            .collect()
    }

    /// A single-class Human Bard who is not performing (no
    /// `class_ability_activations` entry at all) is a genuinely valid PF1
    /// posture -- not every Bard is always performing -- and the
    /// bardic-performance-execution diagnostic does not fire (the
    /// remaining spell-posture diagnostic is a separate, still-permanent
    /// burden, unrelated to this slice).
    #[test]
    fn single_class_bard_not_performing_does_not_trip_the_performance_diagnostic() {
        let input = human_bard_input(1);
        let ids = claim_blocking_ids(&input);

        assert!(
            !ids.iter().any(|id| id.starts_with("class_feature.bard.bardic_performance_execution")),
            "a Bard who isn't performing is a genuinely valid PF1 posture: {ids:?}"
        );
        // v0.6 alpha swarm, risks item 8 (Bard known-spell closure): the
        // spell-posture diagnostic is no longer permanently unconditional
        // either -- this fixture selects zero spells, a genuinely valid
        // posture (mirrors Sorcerer's own "zero known spells is valid"
        // shape), so it no longer fires. A Bard who is neither performing
        // nor carrying an invalid known-spell posture now reaches
        // Computed in full.
        assert!(
            ids.is_empty(),
            "a Bard who isn't performing and has a valid (empty) known-spell posture should \
             have no claim-blocking diagnostics at all: {ids:?}"
        );
        assert_eq!(
            build_pilot_headless_receipt(&input).status,
            super::HeadlessReceiptStatus::Computed
        );
    }

    /// A single-class Human Bard actively, validly performing Inspire
    /// Courage (within budget) does not trip the performance diagnostic,
    /// and the real attack-bonus is genuinely applied to the integrated
    /// baseline melee attack bonus, not merely described.
    #[test]
    fn single_class_bard_actively_performing_in_budget_applies_the_attack_bonus() {
        let mut input = human_bard_input(1);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARD_BARDIC_PERFORMANCE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(1),
        });

        let receipt = build_pilot_headless_receipt(&input);
        let ids: Vec<String> = receipt
            .computation
            .diagnostics
            .iter()
            .filter(|d| d.claim_blocking)
            .map(|d| d.id.clone())
            .collect();
        assert!(
            !ids.iter().any(|id| id.starts_with("class_feature.bard.bardic_performance_execution")),
            "an active, in-budget Inspire Courage is a genuinely valid PF1 posture: {ids:?}"
        );

        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        // Bard level 1 base attack bonus (3/4 BAB: 1*3/4 = 0) + Strength
        // modifier (+4, fixture's chosen Human +2 Strength applied to
        // base 16) + Weapon Focus (+1) + Inspire Courage (+1, first tier)
        // = 6.
        assert_eq!(
            melee_attack_bonus.value, 6,
            "Inspire Courage's attack-roll bonus must be applied: {melee_attack_bonus:?}"
        );
    }

    /// A Bardic Performance activation that exceeds the grounded
    /// rounds-per-day budget is a genuine posture violation and must
    /// claim-block -- never silently capped.
    #[test]
    fn single_class_bard_over_budget_performance_claim_blocks() {
        let mut input = human_bard_input(1);
        // Charisma 8 (-1 modifier): rounds per day = max(4 + (-1) + 2*(1-1), 0) = 3.
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARD_BARDIC_PERFORMANCE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(4),
        });

        let ids = claim_blocking_ids(&input);
        assert!(
            ids.contains(
                &"class_feature.bard.bardic_performance_execution.rounds_exceeded".to_owned()
            ),
            "expected the over-budget claim-blocking diagnostic: {ids:?}"
        );

        let receipt = build_pilot_headless_receipt(&input);
        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        assert_eq!(
            melee_attack_bonus.value, 5,
            "no Inspire Courage bonus is applied for an over-budget, invalid posture: \
             {melee_attack_bonus:?}"
        );
    }

    /// A non-Bard character carrying a spoofed `"bardic_performance"`
    /// activation entry must have it silently ignored, not applied -- the
    /// class-ownership gate is by construction
    /// (`active_bard_inspire_courage_attack_bonus` only ever reads
    /// `class_ability_activations` after confirming `class_levels`
    /// contains Bard), not a bolt-on rejection.
    #[test]
    fn non_bard_characters_spoofed_performance_activation_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARD_BARDIC_PERFORMANCE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);
        assert_eq!(
            receipt.status,
            super::HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Bard performance entry: {:?}",
            receipt.computation.diagnostics
        );
        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        assert_eq!(
            melee_attack_bonus.value, 6,
            "a non-Bard character's spoofed performance entry must never apply a bonus: \
             {melee_attack_bonus:?}"
        );
    }

    /// v0.6 alpha swarm, risks item 8 (Bard known-spell closure): a
    /// single-class Bard knowing 2 real, valid, in-budget 1st-level spells
    /// (within bard level 1's real cap of 2) must NOT trip the spell-
    /// posture diagnostic -- proving real validation, not a vacuous check,
    /// mirroring Sorcerer's own known-spell closure exactly.
    #[test]
    fn single_class_bard_with_valid_known_spells_does_not_trip_the_spell_posture() {
        let mut input = human_bard_input(1);
        for spell_id in ["Alarm", "Charm Person"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: BARD_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.bard.spontaneous_known_and_per_day.unsupported"),
            "2 distinct first-level spells is within bard level 1's real cap of 2: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Bard at level 1 (cap 2 first-level spells known)
    /// knowing 3 distinct first-level spells over-knows its real cap and
    /// must carry the spell-posture diagnostic.
    #[test]
    fn single_class_bard_over_known_spells_stays_blocked_on_spell_posture() {
        let mut input = human_bard_input(1);
        for spell_id in ["Alarm", "Charm Person", "Cause Fear"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: BARD_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.bard.spontaneous_known_and_per_day.unsupported"
                    && d.claim_blocking),
            "bard level 1 only knows 2 first-level spells; 3 distinct known spells over-knows \
             the real cap: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Bard preparing a spell not on the real PF1 Core
    /// Rulebook bard spell list at all must carry the diagnostic.
    #[test]
    fn single_class_bard_with_an_off_list_known_spell_stays_blocked_on_spell_posture() {
        let mut input = human_bard_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Not A Real Spell".to_owned(),
            source_class_id: BARD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.bard.spontaneous_known_and_per_day.unsupported"
                    && d.claim_blocking),
            "an off-list spell must trip the spell-posture diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Bard knowing a real 2nd-level bard spell at level 1
    /// (2nd-level access begins at bard level 4) must carry the
    /// diagnostic -- proving the access-ceiling check, not just the
    /// off-list/over-known checks.
    #[test]
    fn single_class_bard_with_an_inaccessible_known_spell_stays_blocked() {
        let mut input = human_bard_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Alter Self".to_owned(),
            source_class_id: BARD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.bard.spontaneous_known_and_per_day.unsupported"
                    && d.claim_blocking),
            "a 2nd-level bard spell is not accessible at bard level 1: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Multiclass safety, verified directly: a Bard-containing multiclass
    /// mix with a genuine known-spell posture violation must still stay
    /// Blocked, mirroring Sorcerer's own multiclass-safety test.
    #[test]
    fn bard_fighter_multiclass_with_an_invalid_known_spell_stays_blocked() {
        let mut input = human_bard_input(1);
        input.chosen.class_levels.push(CharacterClassLevel {
            class_id: FIGHTER_CLASS_ID.to_owned(),
            level: 1,
        });
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Alter Self".to_owned(),
            source_class_id: BARD_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "a Bard+Fighter multiclass must not reach Computed while Bard's known-spell posture \
             is genuinely violated: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.bard.spontaneous_known_and_per_day.unsupported"
                    && d.claim_blocking),
            "expected the real spell-posture diagnostic to fire in the multiclass mix too: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// task #88 correction: Inspire Heroics' own detail strings (authored
    /// 2026-07-16, Tranche 3, before any AC/save-total integration existed)
    /// used to claim "no AC-application engine exists anywhere in this
    /// codebase" and "no save-resolution engine exists anywhere in this
    /// codebase" -- both false now: Bard is a `table_class_id`-recognized,
    /// dispatch-supported chassis (proven by Inspire Courage's own real
    /// attack-bonus integration), so a GE-06-posture Bard reaches the same
    /// real `defense.baseline_armor_class` / `defense.total_save.*` totals
    /// every other supported class does. Inspire Heroics' own magnitudes
    /// simply aren't wired into either (a real gap: no targeting engine
    /// exists to decide self vs. ally vs. inactive).
    #[test]
    fn bard_inspire_heroics_detail_no_longer_falsely_claims_no_ac_save_totals_exist() {
        // BARD_INSPIRE_HEROICS_LEVEL is 15 (PF1 Core Rulebook grant level).
        let receipt = build_pilot_headless_receipt(&human_bard_input(15));

        let save_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.bard.inspire_heroics_save_bonus")
            .expect("Inspire Heroics save bonus must ground at level 15");
        assert!(
            !save_bonus.detail.contains("no save-resolution engine exists"),
            "the corrected detail must not repeat the false no-total-exists claim: {:?}",
            save_bonus
        );
        assert!(
            save_bonus.detail.contains("defense.total_save"),
            "the corrected detail must name the real save totals it isn't wired into: {:?}",
            save_bonus
        );

        let ac_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.bard.inspire_heroics_ac_bonus")
            .expect("Inspire Heroics AC bonus must ground at level 15");
        assert!(
            !ac_bonus.detail.contains("no AC-application engine exists"),
            "the corrected detail must not repeat the false no-total-exists claim: {:?}",
            ac_bonus
        );
        assert!(
            ac_bonus.detail.contains("defense.baseline_armor_class"),
            "the corrected detail must name the real AC total it isn't wired into: {:?}",
            ac_bonus
        );

        let baseline_ac = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .expect(
                "a GE-06-posture Bard is a table-class-id-recognized supported chassis, so \
                 baseline AC must be real, not absent",
            );
        assert!(
            !baseline_ac.detail.contains("Inspire Heroics"),
            "Inspire Heroics' +4 must NOT be folded into baseline AC yet -- the corrected \
             claim says 'not wired in', not 'wired in': {:?}",
            baseline_ac
        );

        let total_will_save = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.total_save.will")
            .expect("total saves must also be real for a supported Bard, not absent");
        assert!(
            !total_will_save.detail.contains("Inspire Heroics"),
            "Inspire Heroics' +4 must NOT be folded into total saves yet: {:?}",
            total_will_save
        );
    }
}

