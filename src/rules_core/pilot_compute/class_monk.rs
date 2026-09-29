#[allow(unused_imports)]
pub(crate) use super::*;

// SD13-E3/E5 martial chassis baseline identity, mirroring the Barbarian pattern. Monk
// is a non-spell pure martial class with a distinct four-pillar bounded burden; this
// slice recognizes its bounded single-class level-1/level-2/level-3 identity as
// direct runtime evidence and grounds base-attack / base-save progression, unarmed
// strike damage die, the Flurry of Blows flat surface, AC Bonus, the level-1 bonus
// feat choice-slot recognition, (SD13-E5) Evasion at level 2, and (SD13-E5) Still
// Mind at level 3, but no level-1 bonus feat mechanics execution, no ki pool, and no
// level-4+ martial progression.
pub(super) const MONK_CLASS_ID: &str = "class:monk";

/// SD13-E5 Monk level-range gate, mirroring the Fighter `supported_fighter_level` /
/// Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` idiom.
// A further SD13-E5 slice widens the gate to level 9 (verified independently
// against d20pfsrd and legacy.aonprd.com): level 9 base attack bonus stays +6
// (9 * 3 / 4) and all three good saves stay +6 (9 / 2 + 2), integer-division
// coincidences; the unarmed strike die stays 1d10 (the band spans levels
// 8-11); the Flurry flat attack modifier genuinely rises to +7 (level - 2)
// while the attack count stays 3 (the next count change lands at 15th); the
// ki pool and Slow Fall's 40-ft reach both stay at their level-8 values (the
// next Slow Fall reach increase lands at 10th); the level-9 "Special" column
// reads "Improved evasion" — a genuinely NEW named entry, grounded by this
// slice as a +0 identity/recognition record only
// (MONK_IMPROVED_EVASION_LEVEL), mirroring the Evasion / Rogue
// Improved-Uncanny-Dodge precedent; no damage-resolution engine exists here,
// so no damage math is fabricated from it. A further SD13-E5 slice widens
// the gate to level 10 — the tranche ceiling (verified independently against
// d20pfsrd and legacy.aonprd.com): level 10 base attack genuinely rises to
// +7 (10 * 3 / 4) and all three good saves genuinely rise to +7
// (10 / 2 + 2); the unarmed die stays 1d10 (band 8-11); the Flurry flat
// attack modifier genuinely rises to +8 (level - 2) with the count staying 3
// (next change 15th); the ki pool genuinely rises to 8 (10 / 2 + Wis mod)
// and Slow Fall's reach genuinely rises to 50 ft (named explicitly in the
// level-10 "Special" column; task #49 later unified this and every other
// Slow Fall reach gate into `monk_slow_fall_reach_feet`'s single
// floor(MonkLVL/2)*10 formula); the column's other two entries stay named-but-unproven:
// the repeat "Bonus feat" grant (like the level-2/6 repeats) and "ki pool
// (lawful)" — the ki-strike lawful DR-bypass property needs a
// DR/attack-resolution engine that does not exist here, mirroring how the
// 4th-level magic and 7th-level cold-iron/silver ki-strike properties were
// never fabricated either. A further SD18 slice widens the gate to level 11
// (verified independently against d20pfsrd and legacy.aonprd.com): level 11
// base attack genuinely rises to +8 (11 * 3 / 4) while all three good saves
// stay +7 (11 / 2 + 2, an integer-division coincidence with level 10); the
// unarmed die stays 1d10 (band 8-11); the Flurry flat attack modifier
// genuinely rises to +9 (level - 2) with the count staying 3; the ki pool
// stays 8 (11 / 2 + Wisdom modifier, an integer-division coincidence) and
// Slow Fall's reach stays 50 ft (the next rise lands at 12th); the level-11
// "Special" column reads "Diamond body" only, grounded as a new bounded
// grant-only poison-immunity identity record mirroring Purity of Body. A
// further SD18 slice (cycle-2026-07-15T0600) widens the gate to level 12,
// the loop's eighth §3.2 level-12 widening (verified independently against
// d20pfsrd and the Archives of Nethys aonprd.com mirror): level 12 base
// attack genuinely rises to +9 (12 * 3 / 4) and all three good saves
// genuinely rise to +8 (12 / 2 + 2); the unarmed strike damage die
// genuinely steps up from 1d10 to 2d6 (the 2d6 band spans levels 12-15),
// grounded as two facets — the die-face facet (MONK_UNARMED_DAMAGE_DIE_
// THIRD_STEP_UP_LEVEL) and a new die-count facet, since every level 1-11
// band was a single die and this is the first level at which the count
// itself rises; the Flurry flat attack modifier genuinely rises to +10
// (level - 2) with the count staying 3; the ki pool genuinely rises to 9
// (12 / 2 + Wisdom modifier); Slow Fall's reach genuinely rises to 60 ft
// (named explicitly in the level-12 "Special" column); the level-12
// "Special" column's other entry, Abundant Step, was left named-but-unproven
// at the time this widening landed (it requires both a ki-point-spending
// action-economy engine and a dimension-door-equivalent
// teleportation-resolution engine, neither of which existed in this
// codebase) -- task #49 (below) later grounds its one flat magnitude
// (caster level = monk level) while leaving the dimension-door execution
// itself unmodeled, the same standalone-magnitude idiom Wholeness of Body
// and High Jump already used.
//
// A further task #49 slice (2026-07-28) widens the gate to level 20 -- the
// PF1 Core Rulebook Monk capstone, verified independently against both
// primary sources' full level 12-20 class table. Three previously-shipped
// formulas needed re-verification through their newly-reachable upper
// bands FIRST, as their own commit, before the cap moved (see
// `monk_unarmed_strike_damage_die`, `monk_flurry_of_blows_attack_count`,
// and `monk_slow_fall_reach_feet`'s own doc comments): the unarmed strike
// damage die genuinely extends to 2d8 (16-19) and 2d10 (20); Flurry of
// Blows genuinely gains a fourth attack at 15th; and Slow Fall's reach
// genuinely extends through 90 ft (18th) before becoming unlimited ("fall
// any distance without harm") at 20th rather than a naive 100-ft
// extrapolation. Seven capstone-band features are grounded by this
// widening: Abundant Step (12th, see above), Diamond Soul (13th, SR = 10 +
// monk level), Quivering Palm (15th, DC = 10 + 1/2 monk level + Wisdom
// modifier and a monk-level-days duration), Timeless Body (17th,
// grant-only, no numeric formula token), Tongue of the Sun and Moon (17th,
// grant-only), Empty Body (19th, grant-only -- its 3-ki-point cost and
// 1-minute duration are both FIXED, not level-scaled), and Perfect Self
// (20th, DR 10/chaotic -- see `MONK_PERFECT_SELF_LEVEL`'s own doc comment
// for why this is newly REAL rather than dead code). Perfect Self's
// separate Outsider-type clause carries no numeric magnitude and needs a
// creature-type-conditioned resolution engine that does not exist here, so
// it stays deferred regardless of level, unchanged from the earlier
// (superseded) attempt's own conclusion.
pub(super) const MAX_SUPPORTED_MONK_LEVEL: u8 = 20;

// PF1 Core Rulebook level gate at which Monk gains Wholeness of Body (7th
// level, verified independently against two primary sources: d20pfsrd and
// legacy.aonprd.com both name Wholeness of Body as the Monk 7th-level
// special feature entry, alongside an upgrade to the ki pool's
// damage-reduction-bypass material). Wholeness of Body itself ("a monk can
// heal his own wounds as a standard action... a number of hit points of
// damage equal to his monk level by using 2 points from his ki pool") is
// checked and confirmed NOT flat: it requires both a ki-point-consumption/
// action-economy engine and a healing-resolution engine, neither of which
// exists anywhere in this codebase (mirroring exactly why the ki pool's own
// point-spending was already left unimplemented at level 4). The ki pool's
// material-bypass upgrade likewise requires a damage-reduction-bypass-
// resolution engine that does not exist here. Neither is grounded as a
// record at level 7 or level 8 (it stays granted-but-unexecuted, unchanged),
// mirroring the Bard Suggestion / Monk High Jump precedent of naming a
// checked-but-unproven feature without fabricating a value for it. No new
// const is introduced for this level gate since no code branches on it.
// PF1 Core Rulebook level gate at which Monk gains an actual new "Special"
// entry at 8th level: verified independently against two primary sources
// (d20pfsrd and legacy.aonprd.com), the Monk class table's level-8 row names
// only "Slow fall 40 ft." — a rise in the already-grounded Slow Fall
// record's own reach magnitude, not a brand-new class feature. Both primary
// sources were checked specifically for Improved Uncanny Dodge (a
// commonly-repeated but WRONG assumption for Monk, carried over from other
// classes' 8th-level tables): neither source lists it anywhere on the Monk
// class table at any level, so no such record is grounded or fabricated
// here.
/// PF1 Core Rulebook level gate at which Monk gains Evasion (2nd level, verified
/// independently against two primary sources: d20pfsrd and legacy.aonprd.com both
/// list "Bonus feat, evasion" as the Monk 2nd-level special feature entry).
pub(super) const MONK_EVASION_LEVEL: u8 = 2;

/// PF1 Core Rulebook level gate at which Monk gains Improved Evasion (9th level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Improved evasion" as the Monk 9th-level
/// "Special" column entry).
pub(super) const MONK_IMPROVED_EVASION_LEVEL: u8 = 9;

/// PF1 Core Rulebook level gate at which Monk gains Still Mind (3rd level, verified
/// independently against two primary sources: d20pfsrd and legacy.aonprd.com both
/// list "Fast movement, maneuver training, still mind" as the Monk 3rd-level
/// special feature entry). Fast Movement (a speed bonus) and Maneuver Training (a
/// CMB/CMD-substitution rule) are also granted at this same level but are
/// deliberately left named-but-unproven this slice: no speed-total engine and no
/// CMB/CMD engine exist anywhere in this codebase to attach either to. Still Mind
/// alone is grounded, since it is a flat, non-level-scaled magnitude (+2 on saves
/// vs. enchantment spells and effects) matching the Fighter Bravery / Paladin
/// Divine Grace / Rogue Trap Sense idiom exactly.
pub(super) const MONK_STILL_MIND_LEVEL: u8 = 3;

/// The monk level at which Fast Movement and Maneuver Training are both
/// granted, verified directly against `cr_abilities_class.lst`'s own
/// grant lines (`ABILITY:Monk Class Feature|AUTOMATIC|Monk ~ Fast
/// Movement|...|PREVARGTEQ:Monk_CFP_Level,3` and the identical shape for
/// Maneuver Training). The `PREVAREQ:Monk_CF_FastMovement,0` half of each
/// grant is an archetype-suppression flag: it `DEFINE`s to 0 and is only
/// ever set by `PREFACT:1,ABILITIES,...=True` on archetype records this
/// repo does not ingest, so it is provably vacuous here -- the same
/// shape already confirmed on Cavalier, Brawler, Slayer and Shaman.
pub(super) const MONK_FAST_MOVEMENT_LEVEL: u8 = 3;

pub(super) const MONK_MANEUVER_TRAINING_LEVEL: u8 = 3;

/// `PREVARGTEQ:Monk_CFP_Level,5` on High Jump's own grant line.
pub(super) const MONK_HIGH_JUMP_LEVEL: u8 = 5;

/// `PREVARGTEQ:Monk_CFP_Level,7` on Wholeness of Body's own grant line.
pub(super) const MONK_WHOLENESS_OF_BODY_LEVEL: u8 = 7;

// The 4th-level (1d6->1d8) and 8th-level (1d8->1d10) unarmed-strike-die
// step-ups were previously tracked by their own consts
// (MONK_UNARMED_DAMAGE_DIE_STEP_UP_LEVEL / _SECOND_STEP_UP_LEVEL), each
// verified independently against d20pfsrd and legacy.aonprd.com's
// Medium-monk unarmed damage progression table (1d6 at 1-3, 1d8 at 4-7,
// 1d10 at 8-11, 2d6 at 12-15, 2d8 at 16-19, 2d10 at 20). Task #49 folded
// both step-ups into `monk_unarmed_strike_damage_die`'s single
// `min(5, MonkLVL/4)` band-index formula, which reproduces both exactly;
// the two single-purpose consts are retired in favor of the one function.
/// PF1 Core Rulebook level gate at which the Medium-monk unarmed strike damage
/// die steps up again, from 1d10 to 2d6 (12th level, verified independently
/// against the same two primary sources' Medium-monk unarmed damage
/// progression table: the 2d6 band runs levels 12-15). Unlike the two prior
/// step-ups, this is the first level at which the die COUNT itself rises
/// (from a single die to two dice), not just the face size, so it is
/// grounded as two facets — the die-face facet (still `unarmed_die_value`)
/// and a new, standalone die-count facet — mirroring the Flurry of Blows
/// attack-bonus/attack-count split.
pub(super) const MONK_UNARMED_DAMAGE_DIE_THIRD_STEP_UP_LEVEL: u8 = 12;

/// PF1 Core Rulebook level gate at which Flurry of Blows grants a third
/// attack (8th level, verified independently against two primary sources'
/// verbatim Flurry of Blows rule text: "At 8th level, the monk can make two
/// additional attacks when he uses flurry of blows, as if using Improved
/// Two-Weapon Fighting" — i.e. two bonus attacks instead of one, for three
/// total attacks on a flurry full-attack action, up from two at levels 1-7).
pub(super) const MONK_FLURRY_THIRD_ATTACK_LEVEL: u8 = 8;

/// PF1 Core Rulebook level gate at which Flurry of Blows grants a FOURTH
/// attack (15th level, task #49, verified independently against both
/// primary sources' verbatim Flurry of Blows rule text: flurry functions as
/// Two-Weapon Fighting at 1st, Improved TWF at 8th, and Greater TWF at
/// 15th -- four total attacks on a flurry full-attack action, up from three
/// at levels 8-14). Previously unreachable under
/// `MAX_SUPPORTED_MONK_LEVEL = 12`; the attack count was hard-capped at 3.
pub(super) const MONK_FLURRY_FOURTH_ATTACK_LEVEL: u8 = 15;

/// PF1 Core Rulebook level at which Slow Fall stops being a finite
/// reduction and becomes "fall any distance without harm" (task #49,
/// verified independently against both primary sources: the level-20
/// "Special" column reads "Slow fall any distance", and the rule text
/// itself states the wall-assisted descent absorbs unlimited fall damage
/// rather than continuing the arithmetic 10-ft-per-two-levels progression
/// to a finite 100 ft).
pub(super) const MONK_SLOW_FALL_ANY_DISTANCE_LEVEL: u8 = 20;

// Slow Fall's reach magnitude at 8th (40 ft), 10th (50 ft), 12th (60 ft),
// and 6th (30 ft) level was previously tracked by four separate consts
// (MONK_SLOW_FALL_FORTY_FOOT_REACH_LEVEL / _FIFTY_FOOT_REACH_LEVEL /
// _SIXTY_FOOT_REACH_LEVEL / _INCREASED_REACH_LEVEL), each verified
// independently against both primary sources' Monk class table "Special"
// column ("Slow fall 40 ft." / "slow fall 50 ft." / "slow fall 60 ft." /
// "Bonus feat, slow fall 30 ft."), and each confirmed to carry no OTHER new
// class feature at its level (specifically checked and confirmed NOT
// Improved Uncanny Dodge at 8th, which Monk never gains at any level per
// either source, and the level-6 "Bonus feat" entry confirmed to be the
// same open-ended repeat bonus-feat choice-list shape already
// named-but-unproven at 2nd level, not a new automatic grant). Task #49
// unified all of these into the single `floor(MonkLVL/2)*10` formula
// (`monk_slow_fall_reach_feet`), which reproduces every one of these
// checked values exactly for levels 4-12 and extends the same verified
// formula honestly through level 18, so the four single-purpose consts are
// retired in favor of the one function; their verification history is
// preserved here rather than silently dropped.
/// PF1 Core Rulebook level gate at which Monk gains the ki pool and Slow Fall
/// (4th level, verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Ki pool (magic), slow fall 20 ft." as the Monk
/// 4th-level special feature entry).
pub(super) const MONK_KI_POOL_AND_SLOW_FALL_LEVEL: u8 = 4;

/// PF1 Core Rulebook level gate at which Monk gains Purity of Body (5th level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "High jump, purity of body" as the Monk
/// 5th-level special feature entry). High Jump, the OTHER 5th-level "Special"
/// column entry, is deliberately left named-but-unproven this slice: it
/// requires wiring the monk's level into an Acrobatics-check total and
/// spending a ki point (an action-economy/resource-consumption engine this
/// codebase deliberately does not implement for the ki pool either), so it is
/// checked and confirmed NOT flat rather than fabricated.
///
/// **Stale-justification correction (v0.6 Receipt-to-Sheet slice 1 item 5).**
/// This comment used to justify the deferral with "no skill-check-total engine
/// exists in this codebase". That is false, and has been for some time:
/// `skill_allocation::allocate_skill_ranks` returns a real `SkillTotals` whose
/// per-skill `total_modifier` sums ranks, the key ability modifier and the
/// class-skill bonus, applies PF1's class/cross-class rank caps, and is wired
/// into `PilotReceipt.skills` by `contract.rs`'s `to_pilot_receipt`. The
/// deferral itself still stands, for two reasons that are actually true:
///   * that engine's recognized skill universe is a bounded five
///     (`skill:climb`, `skill:swim`, `skill:intimidate`, `skill:diplomacy`,
///     `skill:disable_device` — see `skill_key_ability_modifier`), and
///     Acrobatics is not in it, so there is no Acrobatics total to add to;
///   * High Jump's bonus applies only to the *jump* use of Acrobatics, and
///     `SkillTotal` has no sub-use dimension — folding it into a whole-skill
///     total would silently inflate Acrobatics balance and tumble checks too,
///     which is a wrong number rather than a missing one.
///
/// Purity of Body alone is grounded, since it is a flat, non-level-scaled
/// grant (disease immunity) matching the Barbarian/Rogue Uncanny Dodge / Monk
/// Slow Fall grant-only idiom exactly.
pub(super) const MONK_PURITY_OF_BODY_LEVEL: u8 = 5;

/// The PF1 Core Rulebook level at which Diamond Body is granted (SD18):
/// "at 11th level, a monk gains immunity to all poisons," verified
/// independently against both d20pfsrd and legacy.aonprd.com, both of which
/// list "Diamond body" as the sole level-11 Monk class table "Special"
/// column entry. Grounded as a bounded grant-only identity record, mirroring
/// the Purity of Body (disease immunity) idiom exactly: no
/// poison-resolution engine exists in this codebase to apply the immunity
/// to.
pub(super) const MONK_DIAMOND_BODY_LEVEL: u8 = 11;

/// Task #49 capstone-band level gates, each verified independently against
/// both primary sources (d20pfsrd and the Archives of Nethys aonprd.com
/// mirror) this session's Monk class table read: Abundant Step (12th --
/// already inside the pre-widening 1..=12 range, a pre-existing gap rather
/// than one this widening admits), Diamond Soul (13th), Quivering Palm
/// (15th), Timeless Body (17th), Tongue of the Sun and Moon (17th, the
/// SAME level as Timeless Body -- both are the level-17 "Special" column's
/// two entries), and Empty Body (19th). Perfect Self's own 20th-level gate
/// is named separately below (`MONK_PERFECT_SELF_LEVEL`), since it also
/// needs its DR magnitude (`MONK_PERFECT_SELF_DAMAGE_REDUCTION`) named
/// alongside it.
pub(super) const MONK_ABUNDANT_STEP_LEVEL: u8 = 12;

pub(super) const MONK_DIAMOND_SOUL_LEVEL: u8 = 13;

pub(super) const MONK_QUIVERING_PALM_LEVEL: u8 = 15;

pub(super) const MONK_TIMELESS_BODY_LEVEL: u8 = 17;

pub(super) const MONK_TONGUE_OF_SUN_AND_MOON_LEVEL: u8 = 17;

pub(super) const MONK_EMPTY_BODY_LEVEL: u8 = 19;

/// Perfect Self's grant level and DR magnitude, read directly off the PF1
/// Core Rulebook Monk class table's level-20 row and its own rule text:
/// "the monk gains damage reduction 10/chaotic" (corpus `DR:10/Chaotic`).
///
/// **History**: an earlier attempt (task #41) tried to ground this DR while
/// `MAX_SUPPORTED_MONK_LEVEL` was still 12 -- since the gate (20) sat
/// entirely above the ceiling, `explain_monk_level1_chassis` could never be
/// reached at level 20 at all, so any granting branch would have been
/// PROVABLY DEAD CODE, not a real feature. That attempt correctly recorded
/// only the absence form and deliberately did not add a granting branch.
/// Task #49 widens the ceiling to 20, making level 20 genuinely reachable
/// for the first time -- so the granting branch below is now real, live
/// code, re-derived fresh from the corpus rather than copied from the old
/// (superseded) attempt.
pub(super) const MONK_PERFECT_SELF_LEVEL: u8 = 20;

pub(super) const MONK_PERFECT_SELF_DAMAGE_REDUCTION: i16 = 10;

/// The PF1 Core Rulebook level at which the level-1 monk bonus feat (and the
/// automatic Improved Unarmed Strike grant) always occurs, independent of the
/// character's current supported level. Kept distinct from the generic
/// `supported_monk_level` current-level value so widening to level 2 does not
/// accidentally relabel the level-1-specific bonus feat grant as a level-2 one —
/// PF1 grants monks a SEPARATE bonus feat at 2nd level, recognized by a further
/// SD13-E5 slice as its own numbered choice slot (see
/// `MONK_SECOND_BONUS_FEAT_GRANT_LEVEL`).
pub(super) const MONK_BONUS_FEAT_GRANT_LEVEL: u8 = 1;

/// PF1 Core Rulebook level gate of the Monk's SECOND bonus feat ("At 1st
/// level, 2nd level, and every 4 levels thereafter, a monk may select a bonus
/// feat" — verified identically on both primary sources). The 2nd-level
/// repeat grant draws from the same corrected seven-feat 1st/2nd-level list
/// as slot 1; the 6th/10th-level repeat grants (with their own list
/// additions) stay unrecognized on this bounded seam. This is the
/// repeat-grant PROVING slot: a repeat grant grounds as a numbered choice
/// slot with its own level gate, the idiom the ranger favored-enemy/terrain
/// and combat-style-feat slices already established — no "list-growth
/// mechanism" is needed.
pub(super) const MONK_SECOND_BONUS_FEAT_GRANT_LEVEL: u8 = 2;

pub(super) const MONK_SECOND_BONUS_FEAT_CHOICE_ID: &str = "choice:monk_bonus_feat_2";

/// PF1 Core Rulebook gates of the Monk's THIRD and FOURTH bonus feats ("every
/// 4 levels thereafter" = 6th and 10th). Each slot draws from its own WIDENED
/// list, verified identically on both primary sources: at 6th level
/// "Gorgon's Fist, Improved Bull Rush, Improved Disarm, Improved Feint,
/// Improved Trip, and Mobility" join the base seven; at 10th level "Improved
/// Critical, Medusa's Wrath, Snatch Arrows, and Spring Attack" join. Note
/// the deliberate symmetry with the list-correction slice: Improved Trip is
/// NOT a base-list member but genuinely joins at 6th.
pub(super) const MONK_THIRD_BONUS_FEAT_GRANT_LEVEL: u8 = 6;

pub(super) const MONK_THIRD_BONUS_FEAT_CHOICE_ID: &str = "choice:monk_bonus_feat_3";

pub(super) const MONK_FOURTH_BONUS_FEAT_GRANT_LEVEL: u8 = 10;

pub(super) const MONK_FOURTH_BONUS_FEAT_CHOICE_ID: &str = "choice:monk_bonus_feat_4";

/// The base seven-feat 1st/2nd-level list as (selection, display-name) pairs,
/// shared by every monk bonus-feat slot's recognition.
pub(super) const MONK_BONUS_FEAT_BASE_LIST: [(&str, &str); 7] = [
    ("feat:catch_off_guard", "Catch Off-Guard"),
    ("feat:combat_reflexes", "Combat Reflexes"),
    ("feat:deflect_arrows", "Deflect Arrows"),
    ("feat:dodge", "Dodge"),
    ("feat:improved_grapple", "Improved Grapple"),
    ("feat:scorpion_style", "Scorpion Style"),
    ("feat:throw_anything", "Throw Anything"),
];

/// The 6th-level list additions (slot 3 and later).
pub(super) const MONK_BONUS_FEAT_SIXTH_LEVEL_ADDITIONS: [(&str, &str); 6] = [
    ("feat:gorgons_fist", "Gorgon's Fist"),
    ("feat:improved_bull_rush", "Improved Bull Rush"),
    ("feat:improved_disarm", "Improved Disarm"),
    ("feat:improved_feint", "Improved Feint"),
    ("feat:improved_trip", "Improved Trip"),
    ("feat:mobility", "Mobility"),
];

/// The 10th-level list additions (slot 4).
pub(super) const MONK_BONUS_FEAT_TENTH_LEVEL_ADDITIONS: [(&str, &str); 4] = [
    ("feat:improved_critical", "Improved Critical"),
    ("feat:medusas_wrath", "Medusa's Wrath"),
    ("feat:snatch_arrows", "Snatch Arrows"),
    ("feat:spring_attack", "Spring Attack"),
];

// SD13-E5 Monk level-1 bonus feat choice-slot recognition. RULES CORRECTION
// (SD13-E5): an earlier version of this seam recognized Improved Trip and
// Stunning Fist as restricted-list members and claimed that was the PF1 Core
// Rulebook list. Both primary sources (d20pfsrd and legacy.aonprd.com, re-read
// for this correction) give the actual PF1 CRB 1st/2nd-level list as Catch
// Off-Guard, Combat Reflexes, Deflect Arrows, Dodge, Improved Grapple,
// Scorpion Style, and Throw Anything ("At 6th level, the following feats are
// added to the list: Gorgon's Fist, Improved Bull Rush, Improved Disarm,
// Improved Feint, Improved Trip, and Mobility" — so Improved Trip is a
// 6th-level addition, not a base member; the 6th/10th additions stay
// unrecognized on this bounded seam). Improved Unarmed Strike AND Stunning
// Fist are both deliberately excluded: the PF1 Core Rulebook grants each to
// every monk automatically at level 1 ("At 1st level, the monk gains Stunning
// Fist as a bonus feat, even if he does not meet the prerequisites"),
// separate from this chosen bonus feat, and this codebase does not ground
// those automatic grants either, so neither is ever a choice-set member here.
pub(super) const MONK_BONUS_FEAT_CHOICE_ID: &str = "choice:monk_bonus_feat";

/// PF1 Core Rulebook Improved Grapple's flat bonus magnitude (v0.6 alpha
/// swarm, risks item 8, Monk remaining-feats closure): "+2 bonus on
/// checks made to grapple a foe" and "+2 bonus to your Combat Maneuver
/// Defense whenever an opponent tries to grapple you" (corpus:
/// `BONUS:VAR|CMB_Grapple,CMD_Grapple|2`) -- both share this same
/// magnitude, mirroring `DWARF_STABILITY_CMD_BONUS`'s own bundled-
/// magnitude idiom exactly.
pub(super) const MONK_IMPROVED_GRAPPLE_BONUS: i16 = 2;

/// Grounds the Unchained Monk's unarmed strike damage die, in the same two
/// facets the Core Rulebook Monk's chassis grounds: the die's face size and,
/// separately, its count.
///
/// # Why this exists, given `monk_features.rs` declined to model it
///
/// That module's doc comment declined on the grounds that
/// [`monk_unarmed_strike_damage_die`] "already states that progression". The
/// function did; the sheet did not. Those rows are pushed only by
/// `explain_monk_level1_chassis`, which returns early unless the character
/// holds Core Rulebook `class:monk` (and is Human), so an Unchained Monk got
/// the roster row naming Unarmed Strike and no number at any level. The
/// remedy is to call the existing function from this path, not to write a
/// second ladder.
///
/// # The progressions are identical, established against the corpus
///
/// * `pu_abilities_class.lst:464` (`Unchained Monk ~ Unarmed Strike`) grants
///   `ABILITY:Internal|AUTOMATIC|Monk ~ Unarmed Damage` — the very record
///   `cr_abilities_class.lst:1118` grants for the Core Rulebook Monk — and
///   carries no damage token of its own.
/// * Pathfinder Unchained emits **no** `MonkUnarmedDamage*` or `UDAM` token
///   anywhere in its `.lst` files, so it overrides nothing about that record:
///   no `.MOD`, no `BONUS:VAR`, no replacement band table.
/// * The shared record (`cr_abilities_class.lst:1280`) picks its band with
///   `BONUS:VAR|MonkUnarmedDamageProgression|(min(5,MonkUnarmedDamageLVL/4))`
///   over `BONUS:VAR|MonkUnarmedDamageLVL|MonkLVL`, which is
///   [`monk_unarmed_strike_damage_die`]'s `min(5, level / 4)` exactly.
///
/// # Why the size column is read, when the Core Rulebook path does not
///
/// The shared record fans each band out per creature size —
/// `Monk Unarmed Damage LVL 8 (Small)`, `… (Medium)`, and seven more. The
/// Core Rulebook path never had to choose, because it is gated on `race:human`.
/// This path is not: `race_resolver::RACE_SIZES` gives the 18 playable races
/// 13 Medium and 5 Small, so handing a Gnome or a Halfling the Medium ladder
/// would put a specific, checkable, wrong die on a player's sheet. Only the
/// two columns a playable race can occupy are modelled; any other size grounds
/// an honest absence rather than a substituted number.
///
/// # Both facets are emitted at every level, unlike the Core Rulebook path
///
/// `explain_monk_level1_chassis` withholds the count row below level 12,
/// because that facet was introduced by a later cycle at the level where the
/// count first rises. A face size with no count beside it is ambiguous on the
/// sheet — `10` alone does not distinguish 1d10 from 2d10 — so this path emits
/// both from level 1. The Core Rulebook path is deliberately left exactly as
/// it is; `tests/sd27_unchained_monk_unarmed_strike_reaches_the_sheet.rs`'s
/// `the_core_rulebook_monk_is_byte_identical` is the guard on that.
pub(super) fn ground_unchained_monk_unarmed_strike_damage(
    level: u8,
    race_id: &str,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let resolved = race_size_for_race_token(race_id).and_then(|size| {
        monk_unarmed_strike_damage_die_for_size(level, size).map(|die| (size, die))
    });
    let Some((size, (die_face, die_count, die_name))) = resolved else {
        push_deferred_class_features(
            "class_feature.pu.unchained_monk.unarmed_strike_damage_die.unsupported",
            format!(
                "Unchained Monk level {level}: the unarmed strike damage die is a column of the \
                 shared Core Rulebook `Monk ~ Unarmed Damage` record \
                 (cr_abilities_class.lst:1280) chosen by the character's creature SIZE, and \
                 race {race_id:?} resolves to no size this engine models. Only the Small and \
                 Medium columns are transcribed -- the two every one of the 18 playable races \
                 occupies -- so nothing is computed here rather than the Medium ladder being \
                 substituted, which would be a specific, checkable, wrong die. This diagnostic \
                 is NOT claim-blocking: every other Unchained Monk magnitude on this sheet is \
                 size-independent and unaffected"
            ),
            explanations,
            diagnostics,
        );
        return;
    };
    let size_label = size_label(size);

    explanations.push(ComputationExplanation {
        id: "class_feature.pu.unchained_monk.unarmed_strike_damage_die".to_owned(),
        value: die_face,
        detail: format!(
            "Unchained Monk level {level} Unarmed Strike: {die_name} for a {size_label} monk, so \
             the die-face-size facet is {die_face} (the d{die_face} in {die_name}); the count \
             facet is grounded separately below. Pathfinder Unchained does NOT restate this \
             progression -- `Unchained Monk ~ Unarmed Strike` (pu_abilities_class.lst:464) \
             grants the shared Core Rulebook record `Monk ~ Unarmed Damage` \
             (cr_abilities_class.lst:1280) and adds no damage token, and Pathfinder Unchained \
             writes no MonkUnarmedDamage or UDAM token anywhere, so the band ladder \
             min(5, level / 4) over 1d6/1d8/1d10/2d6/2d8/2d10 (Medium) is literally the Core \
             Rulebook Monk's and is read from the same function. The size column matters: the \
             shared record fans every band out per creature size, and a {size_label} monk reads \
             the {size_label} column. No damage roll, damage total or attack-resolution engine \
             is computed -- this is the die, not a result"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_feature.pu.unchained_monk.unarmed_strike_damage_die_count".to_owned(),
        value: die_count,
        detail: format!(
            "Unchained Monk level {level} Unarmed Strike damage die count: {die_count} for a \
             {size_label} monk ({die_name}). Only this count facet is grounded here; the \
             die-face-size facet is grounded separately above. Emitted at every level rather \
             than only from the level the count first rises, because a face size with no count \
             beside it does not distinguish 1d10 from 2d10 on the sheet"
        ),
    });
}

/// Grounds the Unchained Monk's named features
/// (`rules_tables::pathfinder_unchained::monk_features`).
///
/// `total_base_attack_bonus` is the chassis row's own value, passed in
/// rather than recomputed, because Flurry of Blows keys off base attack
/// bonus and not off level — the one Unchained Monk magnitude that would be
/// wrong if it were derived from the class level directly.
///
/// `race_id` is read for one thing only: the creature size the unarmed strike
/// damage die is a column of. See
/// [`ground_unchained_monk_unarmed_strike_damage`].
pub(super) fn ground_unchained_monk_class_features(
    level: u8,
    total_base_attack_bonus: i16,
    ability_modifiers: &AbilityModifiers,
    race_id: &str,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    ground_unchained_monk_unarmed_strike_damage(level, race_id, explanations, diagnostics);

    let ac_from_level = monk_features::armor_class_bonus_from_level(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.pu.unchained_monk.armor_class_bonus_level_component".to_owned(),
        value: ac_from_level,
        detail: format!(
            "Unchained Monk level {level} AC Bonus, level component: +{ac_from_level} \
             (min(level / 4, 5))"
        ),
    });
    let ac_bonus = monk_features::armor_class_bonus(level, ability_modifiers.wisdom);
    explanations.push(ComputationExplanation {
        id: "class_feature.pu.unchained_monk.armor_class_bonus".to_owned(),
        value: ac_bonus,
        detail: format!(
            "Unchained Monk level {level} AC Bonus: +{ac_bonus} to Armor Class and CMD when \
             unarmored and unencumbered (level component +{ac_from_level} plus max(Wisdom \
             modifier {}, 0)). Grounded as a standalone magnitude: the unarmored/unencumbered \
             condition is a real gate this engine cannot evaluate, so it is stated and NOT \
             silently folded into the character's Armor Class total",
            ability_modifiers.wisdom
        ),
    });
    if let Some(feats) = monk_features::bonus_feats_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.bonus_feats_known".to_owned(),
            value: feats,
            detail: format!(
                "Unchained Monk level {level} Bonus Feat: {feats} bonus feats \
                 (1 + max((level + 2) / 4, 0)). The Unchained Monk bonus-feat list is not \
                 ingested, so this is the count of picks and not a catalogue of options"
            ),
        });
    }
    let flurry = monk_features::flurry_attack_count(total_base_attack_bonus);
    explanations.push(ComputationExplanation {
        id: "class_feature.pu.unchained_monk.flurry_attack_count".to_owned(),
        value: flurry,
        detail: format!(
            "Unchained Monk level {level} Flurry of Blows: {flurry} attacks at a total base \
             attack bonus of {total_base_attack_bonus} \
             (2 + (bab>=6) + 2 x (bab>=11) + (bab>=16)). The Unchained Monk reaches 6 where the \
             Core Rulebook Monk stops at 4"
        ),
    });
    if let Some(feet) = monk_features::fast_movement_bonus_feet(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.fast_movement_bonus_feet".to_owned(),
            value: feet,
            detail: format!(
                "Unchained Monk level {level} Fast Movement: +{feet} feet to base land speed \
                 (10 x (level / 3)) when unarmored and unencumbered. The condition is stated, \
                 not silently applied"
            ),
        });
    }
    // The corpus makes the ki stat a choice (Wisdom, Charisma or
    // Intelligence, behind `BONUS:ABILITYPOOL|Ki Pool Stat Choice`). No
    // picker for it exists, and Wisdom is the plain Unchained Monk's stat,
    // so Wisdom is passed and the substitution is named in the record
    // rather than hidden.
    if let Some(ki) = monk_features::ki_points(level, ability_modifiers.wisdom) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.ki_points".to_owned(),
            value: ki,
            detail: format!(
                "Unchained Monk level {level} Ki Pool: {ki} ki points (level / 2 + ki-stat \
                 modifier). The ki stat is a corpus-declared choice of Wisdom, Charisma or \
                 Intelligence; no picker for it exists in this engine, so the plain Unchained \
                 Monk's Wisdom modifier {} is used and the substitution is recorded here rather \
                 than assumed silently",
                ability_modifiers.wisdom
            ),
        });
    }
    if let Some(powers) = monk_features::ki_powers_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.ki_powers_known".to_owned(),
            value: powers,
            detail: format!(
                "Unchained Monk level {level} Ki Powers: a pool of {powers} ((level - 2) / 2). \
                 The 31 ki powers this pool is spent on are not ingested"
            ),
        });
    }
    if let Some(strikes) = monk_features::style_strikes_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.style_strikes_known".to_owned(),
            value: strikes,
            detail: format!(
                "Unchained Monk level {level} Style Strike: {strikes} style strikes known \
                 ((level - 1) / 4). The 10 style strikes this pool is spent on are not ingested"
            ),
        });
    }
    if let Some(bonus) = monk_features::still_mind_save_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.still_mind_save_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Unchained Monk level {level} Still Mind: +{bonus} on saving throws against \
                 enchantment spells and effects. Conditional on the effect's school, which this \
                 engine does not resolve at save time, so it is NOT added to any resting save \
                 total"
            ),
        });
    }
    if let Some(dr) = monk_features::perfect_self_damage_reduction(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.perfect_self_damage_reduction".to_owned(),
            value: dr,
            detail: format!(
                "Unchained Monk level {level} Perfect Self: damage reduction {dr}/chaotic"
            ),
        });
    }
    if let Some(monk_level) = monk_features::stunning_fist_monk_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.stunning_fist_monk_level".to_owned(),
            value: monk_level,
            detail: format!(
                "Unchained Monk level {level} Stunning Fist: contributes an effective monk level \
                 of {monk_level}. The save DC and uses per day live on the shared Core Rulebook \
                 Stunning Fist FEAT record (already grounded in rules_core::feat_effects), not on \
                 this class row -- feat-lane content stays in the feat lane"
            ),
        });
    }
    // Four Unchained Monk records whose whole numeric content is their own
    // English DESC: and which therefore computed nothing until now. Each
    // number below is read out of the sentence quoted in
    // `monk_features::prose_derived`, and that sentence is re-read off the
    // ingested corpus record by that module's own tests.
    if let Some(percent) =
        monk_features::prose_derived::evasion_damage_percent_on_a_successful_reflex_save(level)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.evasion_damage_percent_on_a_made_reflex_save"
                .to_owned(),
            value: percent,
            detail: format!(
                "Unchained Monk level {level} Evasion: on a successful Reflex save against an \
                 attack that normally deals half damage on a save, the monk takes {percent}% of \
                 the damage -- none at all, where the default is half. Only while wearing light \
                 armor or none, and a helpless monk gains no benefit; both conditions are stated \
                 rather than applied, because this engine evaluates neither"
            ),
        });
    }
    if let Some(percent) =
        monk_features::prose_derived::improved_evasion_damage_percent_on_a_failed_reflex_save(level)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.improved_evasion_damage_percent_on_a_failed_reflex_save"
                .to_owned(),
            value: percent,
            detail: format!(
                "Unchained Monk level {level} Improved Evasion: from level 9 a failed Reflex save \
                 costs only {percent}% of the damage, while a successful one still costs none. A \
                 helpless monk gains no benefit"
            ),
        });
    }
    if let Some(rolls) = monk_features::prose_derived::flawless_mind_will_save_rolls(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.flawless_mind_will_save_rolls".to_owned(),
            value: rolls,
            detail: format!(
                "Unchained Monk level {level} Flawless Mind: every Will save is rolled {rolls} \
                 times and the better result taken. A failed Will save against an effect lasting \
                 longer than 1 hour may additionally be re-attempted at the end of each hour -- a \
                 retry interval, not a second magnitude, and this engine has no save resolution \
                 for either clause to act on"
            ),
        });
    }
    if let Some(penalty) = monk_features::prose_derived::timeless_body_aging_ability_penalty(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_monk.timeless_body_aging_ability_penalty".to_owned(),
            value: penalty,
            detail: format!(
                "Unchained Monk level {level} Timeless Body: the monk's ability-score penalty for \
                 aging is {penalty} and he cannot be magically aged. A real zero, not a filler \
                 one. Penalties already taken remain, age BONUSES still accrue, and the monk still \
                 dies of old age when his time is up -- this number is the penalty, not immortality"
            ),
        });
    }

    push_deferred_class_features(
        "class_feature.pu.unchained_monk.other_features_deferred.unsupported",
            "class:unchained_monk grounds every Unchained Monk magnitude this book states as a \
             formula token: its own chassis (d10 hit die, FULL base attack bonus, good Fortitude \
             and Reflex and POOR Will -- all four genuinely different from the Core Rulebook \
             Monk's), the AC Bonus and its level component, the bonus-feat count, the Flurry of \
             Blows attack count, Fast Movement, the Ki Pool, the ki-power and style-strike pool \
             sizes, Still Mind, Perfect Self's damage reduction, and Stunning Fist's effective \
             monk level -- plus four whose only numbers are in their own prose: Evasion's and \
             Improved Evasion's damage percentages, Flawless Mind's two Will-save rolls, and \
             Timeless Body's zero aging penalty. This diagnostic is NOT claim-blocking; it \
             carries the honest remainder. \
             What is missing: (1) the 31 ki powers, 10 style strikes and the Unchained Monk \
             bonus-feat list -- pool sizes are real, the option catalogues are not ingested; (2) \
             the ki-stat choice has no picker, so Wisdom is used and said so; (3) APPLICATION -- \
             the AC Bonus is gated on being unarmored and unencumbered, and Fast Movement the \
             same, neither of which this engine can evaluate, so both are standalone magnitudes \
             rather than contributions to a total; (4) unarmed strike damage IS grounded above, \
             and is not restated as a second ladder: the corpus grants the SAME shared Core \
             Rulebook record and Pathfinder Unchained overrides nothing about it, so the \
             engine's existing progression is called rather than copied. What remains missing \
             there is the other seven size columns of that record -- only Small and Medium are \
             transcribed, which is every size a playable race occupies; (5) \
             Purity of Body (immunity to all diseases) and Tongue of the Sun and Moon (speak with \
             any living creature) state NO number anywhere -- not in a formula token and not in \
             their prose -- so nothing is computed for them and nothing is invented; the same is \
             true of Perfect Self's Outsider-type clause. The four prose-derived numbers above \
             are magnitudes and not applications: this engine resolves no saving throws, so \
             Evasion, Improved Evasion and Flawless Mind change no rolled outcome, and it models \
             no aging, so Timeless Body cancels a penalty that was never applied; (6) archetype \
             suppression flags \
             (`Monk_CF_*`) are not implemented, so every progression above is the unsuppressed one"
            .to_owned(),
        explanations,
        diagnostics,
    );
}

/// The bounded Monk milestone level this decomposition surface grounds, if any.
/// Returns the single Monk level when the chosen input is exactly a single-class
/// Monk at one of the supported milestone levels (1 through 10). Returns
/// `None` for no Monk, a non-Monk class, a multiclass mix, or any level-11+ Monk
/// this slice deliberately does not recognize — each of which stays
/// claim-blocked exactly as before. Mirrors the Fighter `supported_fighter_level`
/// / Paladin `supported_paladin_level` / Rogue `supported_rogue_level` /
/// Barbarian `supported_barbarian_level` level-range gate idiom.
pub(super) fn supported_monk_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == MONK_CLASS_ID
                && (1..=MAX_SUPPORTED_MONK_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E3/E5 runtime evidence for the deterministic Human Monk
/// level-1/level-2/level-3/level-4/level-5/level-6/level-7/level-8 martial chassis,
/// mirroring the Barbarian/Rogue level-range-gate pattern, and now grounding ten
/// named pillar burdens at every supported level (base-attack, base-save, AC Bonus, the
/// unarmed strike die / Flurry of Blows flat surface, the level-1 bonus feat
/// choice-slot recognition, at level 2, Evasion, at level 3, Still Mind, at level
/// 4, the ki pool's flat size and Slow Fall, and at level 5, Purity of Body)
/// while keeping it explicitly claim-blocked on the recognized bonus feat's own
/// mechanics (an execution engine, not a flat number).
///
/// This grounds the Monk base-attack progression (3/4 BAB: `classlevel * 3 / 4`),
/// the base-save progression (good Fortitude, Reflex, and Will: `classlevel/2+2`
/// each — Monk is unusual among the martial classes recognized so far in having all
/// three saves good rather than a 2-good/1-poor or 1-good/2-poor split), the AC
/// Bonus (the positive Wisdom modifier added to AC, asserted unconditionally on this
/// deterministic unarmored fixture), the Medium-monk unarmed strike damage die size
/// (1d6 at levels 1-3 — die size only, mirroring the Rogue sneak-attack die-count
/// record: no damage roll or damage total is computed), the Flurry of Blows flat
/// surface (two attacks, each at monk level - 2 before ability modifiers), the
/// level-1 bonus feat choice-slot selection when it names one of the PF1 Core
/// Rulebook restricted Monk bonus feat list's five feats (Combat Reflexes, Deflect
/// Arrows, Improved Grapple, Improved Trip, Stunning Fist), mirroring the Sorcerer
/// bloodline choice / Cleric domain choice / Druid nature-bond choice recognition
/// idiom, (SD13-E5) Evasion, a 2nd-level Monk class feature verified
/// independently against two primary PF1 sources (d20pfsrd and legacy.aonprd.com
/// both list "Bonus feat, evasion" as the Monk 2nd-level special feature entry) —
/// grounded as a bounded identity/recognition record only, mirroring exactly how
/// Rogue's own `class_feature.rogue.evasion` was grounded (value 0, correct
/// level-gate absence below level 2, granted-but-unexecuted rule text at level 2,
/// no saving-throw-resolution or damage-resolution engine), and (SD13-E5) Still
/// Mind, a 3rd-level Monk class feature verified independently against the same
/// two primary sources (both list "Fast movement, maneuver training, still mind"
/// as the Monk 3rd-level special feature entry) — grounded as a bounded
/// flat-magnitude record (a flat +2 on saves vs. enchantment spells and effects,
/// value 0 as a correct level-gate absence below level 3), mirroring the Fighter
/// Bravery / Paladin Divine Grace / Rogue Trap Sense idiom, never applied to any
/// actual save total. Fast Movement and Maneuver Training, the class table's other
/// two 3rd-level "Special" column entries, stay named-but-unproven. At level 4,
/// the unarmed strike damage die steps up from 1d6 to 1d8 (verified independently
/// against the same two primary sources' Medium-monk damage progression table),
/// the ki pool's flat size is grounded as a standalone flat-magnitude record
/// (1/2 monk level + Wisdom modifier, mirroring the Barbarian rage rounds-per-day
/// / Paladin lay-on-hands-uses-per-day idiom — no ki-point consumption tracking,
/// no action-economy engine, and no application of any ki power), and Slow Fall
/// is grounded as a bounded grant-only identity record (no fall-damage-resolution
/// engine exists in this codebase). At level 5, Purity of Body is grounded as a
/// bounded grant-only identity record (a flat disease-immunity grant, no
/// disease-resolution engine exists in this codebase); High Jump, the level-5
/// class table's OTHER "Special" column entry, is checked and confirmed NOT flat
/// (it requires wiring the monk's level into an Acrobatics-check total and
/// spending a ki point) and is deliberately left named-but-unproven. Further
/// SD13-E5 slices widen the gate to level 6 (Slow Fall's own reach magnitude
/// genuinely rising from 20 ft to 30 ft) and to level 7 (base attack, base
/// saves, the unarmed strike die, and the Flurry of Blows flat surface all
/// extend via the same pre-existing formulas with no re-derivation; Wholeness
/// of Body, the level-7 "Special" column's new feature, is checked and
/// confirmed NOT flat — it requires a ki-point-consumption/action-economy
/// engine and a healing-resolution engine, neither of which exists in this
/// codebase — and is deliberately left named-but-unproven, mirroring the High
/// Jump precedent). A still further SD13-E5 slice widens the gate to level 8
/// (base attack and base saves extend via the same pre-existing formulas; the
/// unarmed strike damage die genuinely rises to 1d10 — the 1d10 band starts at
/// level 8; the Flurry of Blows attack count genuinely rises from 2 to 3 —
/// verified independently against both primary sources' verbatim Flurry of
/// Blows rule text, "At 8th level, the monk can make two additional attacks";
/// Slow Fall's own reach magnitude genuinely rises from 30 ft to 40 ft; the ki
/// pool's flat size genuinely rises via the same pre-existing formula; Evasion,
/// Still Mind, and Purity of Body all stay granted unchanged. Both primary
/// sources' level-8 "Special" column names only the Slow Fall reach rise —
/// checked and specifically confirmed NOT Improved Uncanny Dodge, which Monk
/// never gains at any level per either source — so no new class-feature
/// record is grounded or fabricated at level 8). It still
/// grounds no attack-resolution or damage-roll engine, no monk-weapon flurry, no
/// level-9+ unarmed damage die progression, no ki-power execution, no level-4+ AC
/// Bonus dodge-bonus progression, no "unarmored and unencumbered" runtime
/// state-check engine, no wiring into integrated combat totals, no level-9+
/// martial progression, no Wholeness of Body execution, no level-2/level-6 bonus
/// feat grant (PF1 grants monks SEPARATE bonus feats at 2nd and 6th level; this
/// widening does not add a second choice-slot or recognition for either), and no
/// execution of what the recognized level-1 bonus
/// feat actually does, for the opponent-dependent resolution piece only
/// (no attack-of-opportunity trigger engine, no grapple-check engine, no
/// ranged-attack-deflection engine, no target's-own-saving-throw engine).
/// **Updated (Monk remaining-feats closure)**: this no longer means zero
/// value is ever grounded for Combat Reflexes/Scorpion Style/Improved
/// Grapple specifically -- each now grounds a flat, standalone number
/// purely from the Monk's own stats (extra-AoO capacity, a save DC and
/// duration, a flat CMB/CMD magnitude) when recognized as the level-1
/// bonus feat; only the opponent-dependent resolution around each stays
/// unmodeled. Deflect Arrows has no such standalone number at all and
/// stays fully claim-blocked, unchanged. It:
/// - leaves one chassis-recognition explanation so the `class:monk:N` identity is
///   acknowledged as a non-hybrid martial baseline rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value),
/// - leaves grounded explanation records for base-attack, the three base saves,
///   AC Bonus, the unarmed strike damage die, the flurry flat attack
///   bonus/attack count, Evasion, Still Mind, the ki pool's flat size, Slow
///   Fall, and Purity of Body,
/// - conditionally leaves one grounded explanation recognizing the level-1 bonus
///   feat choice-slot selection when a `choice:monk_bonus_feat` selection is
///   present (carrying no fabricated mechanical value, since the recognized
///   feat's own mechanics are an execution engine rather than a number), and
/// - emits one claim-blocking diagnostic naming the still-missing burden (the
///   recognized bonus feat's own mechanics, or the bonus feat grant entirely when
///   no restricted-list selection is recognized) explicitly, rather than hiding
///   behind a single generic "unsupported class" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture (`defense.baseline_armor_class` stays gated to Fighter
/// and is untouched here) but makes the Monk martial identity, its grounded pillars,
/// its recognized bonus feat choice, and its one remaining named burden legible on
/// the runtime path.
///
/// PF1 Core Rulebook Combat Reflexes (v0.6 alpha swarm, risks item 8, Monk
/// remaining-feats closure): the corpus formula token
/// `BONUS:VAR|CombatReflexesAttacks|DEX` resolves to the Dexterity
/// modifier, floored at 0 (a feat never subtracts from the base
/// attack-of-opportunity capacity). Pure function so the real grounding
/// below shares one source of truth with any future consumer.
pub(super) fn monk_combat_reflexes_additional_attacks_of_opportunity(dexterity_modifier: i16) -> i16 {
    dexterity_modifier.max(0)
}

/// PF1 Core Rulebook Scorpion Style's save DC (v0.6 alpha swarm, risks
/// item 8, Monk remaining-feats closure): the corpus formula
/// `10+(TL/2)+WIS` -- `level` here is this bounded single-class seam's
/// own Monk level, standing in for total character level since no
/// multiclass mix is admitted here. Pure function so the real grounding
/// below shares one source of truth with any future consumer.
/// Monk Fast Movement's enhancement bonus to land speed:
/// `10*floor(MonkFastMovementLVL/3)` feet, where `MonkFastMovementLVL`
/// resolves to `MonkLVL` -- verified directly against
/// `cr_abilities_class.lst`'s own
/// `BONUS:VAR|MonkFastMovementBonus|10*floor(MonkFastMovementLVL/3)`.
/// +10 ft at levels 3-5, +20 at 6-8, +30 at 9-11, +40 at 12.
///
/// Unlike Barbarian's own Fast Movement (a flat +10 at every level), the
/// monk's scales -- so the two are NOT the same magnitude despite sharing
/// a feature name, and the Barbarian precedent is a shape precedent only.
pub(super) fn monk_fast_movement_bonus_feet(level: u8) -> i16 {
    10 * (i16::from(level) / 3)
}

/// Monk Maneuver Training's CMB base-attack substitution:
/// `CMB_BAB = ManeuverTrainingLVL - ManeuverTrainingBAB`, i.e.
/// `MonkLVL - MonkLVL*3/4` -- verified against the corpus's own three
/// tokens. This is the DELTA the corpus adds, not the resulting CMB: the
/// monk's full level replaces his 3/4 base attack bonus for combat
/// maneuvers only, so the corpus expresses it as the difference to add
/// on top of the BAB already counted.
///
/// Integer division floors, matching PF1's own 3/4 BAB progression, so
/// the delta is 1 at levels 3-4, 2 at 5-8, 3 at 9-12.
pub(super) fn monk_maneuver_training_cmb_bonus(level: u8) -> i16 {
    let level = i16::from(level);
    level - (level * 3 / 4)
}

/// Monk AC Bonus's level-4+ dodge-bonus progression (PF1 Core Rulebook Monk
/// class table, AC Bonus feature): "starting at 4th level, she gains a
/// further +1 dodge bonus to her AC and CMD... every four levels
/// thereafter... this bonus increases by a further +1." `+0` below level 4,
/// `+1` at 4-7, `+2` at 8-11, `+3` at 12-15, `+4` at 16-19, `+5` at 20 —
/// confirmed against the real pinned PCGen oracle
/// (`core_rulebook:class_feature:monk_ac_bonus`, L20 Human Monk: real
/// export `7` = Wisdom-to-AC `2` + this progression's `5`;
/// `AT-33-E5-remainder-charbuild_cycle_receipt.md`).
pub(super) fn monk_ac_bonus_dodge_progression(level: u8) -> i16 {
    if level < 4 {
        0
    } else {
        1 + i16::from((level - 4) / 4)
    }
}

/// Monk High Jump's flat bonus on Acrobatics checks made to jump:
/// `HighJumpBonus = HighJumpLVL = MonkLVL` -- verified against
/// `BONUS:SITUATION|Acrobatics=When Jumping|HighJumpBonus`.
///
/// **This flat bonus is unconditional.** The record's own DESC separates
/// it from the ki-boost clause: "You can adds +%1 to all Acrobatics
/// checks made to jump ... By spending 1 point from your ki pool as a
/// swift action, you gain a +20 bonus". Only the +20 costs ki. An earlier
/// deferral of this feature cited the ki cost as a blocker for the whole
/// ability; that conflated the two clauses.
pub fn monk_high_jump_acrobatics_bonus(level: u8) -> i16 {
    i16::from(level)
}

/// Monk Wholeness of Body's self-heal magnitude: `WholenessOfBody =
/// WholenessOfBodyLVL = MonkLVL` hit points, for 2 ki points -- verified
/// against the corpus's own two `BONUS:VAR` tokens.
pub fn monk_wholeness_of_body_healing(level: u8) -> i16 {
    i16::from(level)
}

pub fn monk_scorpion_style_dc(level: u8, wisdom_modifier: i16) -> i16 {
    10 + i16::from(level) / 2 + wisdom_modifier
}

/// The Medium monk's unarmed strike damage die, as `(die face size, die
/// count, display name)` -- PF1 Core Rulebook Monk class table, verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror (task #49): 1d6/1d8/1d10/2d6/2d8/2d10 at levels
/// 1-3/4-7/8-11/12-15/16-19/20 (`min(5, MonkLVL/4)` band index). The
/// `min(5, ...)` is what proves the progression stops at 2d10 rather than
/// continuing past level 20.
///
/// Extracted from what was previously an inline 4-arm `if`/`else` capped at
/// 2d6 (bands 4 and 5, 2d8 and 2d10, were unreachable under the old
/// `MAX_SUPPORTED_MONK_LEVEL = 12` and ungrounded). This function reproduces
/// the old inline logic exactly for levels 1-11 and extends it through 20.
pub(super) fn monk_unarmed_strike_damage_die(level: u8) -> (i16, i16, &'static str) {
    match (i16::from(level) / 4).min(5) {
        0 => (6, 1, "1d6"),
        1 => (8, 1, "1d8"),
        2 => (10, 1, "1d10"),
        3 => (6, 2, "2d6"),
        4 => (8, 2, "2d8"),
        _ => (10, 2, "2d10"),
    }
}

/// The **Small** monk's unarmed strike damage die, in the same
/// `(die face size, die count, display name)` shape
/// [`monk_unarmed_strike_damage_die`] returns for the Medium column.
///
/// Transcribed from the shared Core Rulebook record's own per-size band rows
/// in `core_rulebook/cr_abilities_class.lst`, whose `UDAM:` tokens are the
/// authority:
///
/// | band | record | line | `UDAM:` |
/// |---|---|---:|---|
/// | 0 (levels 1-3)   | `Monk Unarmed Damage LVL 1 (Small)`  | 1296 | `1d4` |
/// | 1 (levels 4-7)   | `Monk Unarmed Damage LVL 4 (Small)`  | 1306 | `1d6` |
/// | 2 (levels 8-11)  | `Monk Unarmed Damage LVL 8 (Small)`  | 1316 | `1d8` |
/// | 3 (levels 12-15) | `Monk Unarmed Damage LVL 12 (Small)` | 1326 | `1d10` |
/// | 4 (levels 16-19) | `Monk Unarmed Damage LVL 16 (Small)` | 1336 | `2d6` |
/// | 5 (level 20)     | `Monk Unarmed Damage LVL 20 (Small)` | 1346 | `2d8` |
///
/// The band index is the same `min(5, level / 4)` the Medium column uses --
/// it comes from `BONUS:VAR|MonkUnarmedDamageProgression` on the parent
/// record, which is shared across all nine size columns -- so only the six
/// values differ, never the ladder shape.
///
/// **This is not a second progression; it is the same progression read one
/// column across.** It exists because
/// `ground_unchained_monk_unarmed_strike_damage` can be reached by a Gnome,
/// Halfling, Goblin, Kobold or Svirfneblin, where `explain_monk_level1_chassis`
/// cannot be reached by anything but a Human.
pub(super) fn small_monk_unarmed_strike_damage_die(level: u8) -> (i16, i16, &'static str) {
    match (i16::from(level) / 4).min(5) {
        0 => (4, 1, "1d4"),
        1 => (6, 1, "1d6"),
        2 => (8, 1, "1d8"),
        3 => (10, 1, "1d10"),
        4 => (6, 2, "2d6"),
        _ => (8, 2, "2d8"),
    }
}

/// The unarmed strike damage die for one creature size, or `None` for a size
/// this engine has not transcribed a column for.
///
/// `None` is the honest answer rather than a fallback, and the caller grounds
/// an absence notice on it. Substituting the Medium column for an unmodelled
/// size would be a plausible-looking wrong die, which is the failure mode this
/// repo's history is a list of.
///
/// Only Small and Medium are transcribed because `race_resolver::RACE_SIZES`
/// gives the 18 playable races exactly those two -- 13 Medium, 5 Small. A race
/// arriving at another size reaches the absence notice, and
/// `every_playable_race_reads_its_own_size_column` fails the moment the roster
/// stops matching this claim.
pub(super) fn monk_unarmed_strike_damage_die_for_size(
    level: u8,
    size: SizeCategory,
) -> Option<(i16, i16, &'static str)> {
    match size {
        SizeCategory::Medium => Some(monk_unarmed_strike_damage_die(level)),
        SizeCategory::Small => Some(small_monk_unarmed_strike_damage_die(level)),
        _ => None,
    }
}

/// The number of attacks a Flurry of Blows full-attack grants: 2 at 1st
/// level (Two-Weapon Fighting shape), 3 at 8th (Improved TWF -- "the monk
/// can make two additional attacks when he uses flurry of blows"), and 4 at
/// 15th (Greater TWF), verified independently against both primary
/// sources' verbatim Flurry of Blows rule text (task #49).
///
/// Deliberately NOT the corpus's own `BONUS:VAR|FlurryAttacks|
/// 2+(FlurryLVL>=6)+(>=8)+(>=11)+(>=15)+(>=16)` token, which reaches 7 by
/// level 20 by counting total attack-routine entries including the
/// base-attack iteratives that arrive at 6/11/16 -- a different quantity
/// from the flurry-granted attack count this function documents.
/// Substituting it would look like "using the corpus value" while silently
/// changing what is measured.
pub(super) fn monk_flurry_of_blows_attack_count(level: u8) -> i16 {
    if level < MONK_FLURRY_THIRD_ATTACK_LEVEL {
        2
    } else if level < MONK_FLURRY_FOURTH_ATTACK_LEVEL {
        3
    } else {
        4
    }
}

/// How many feet shorter a fall is treated as, for Slow Fall. `None` means
/// "any distance" -- the level-20 case, which is genuinely unlimited rather
/// than a large finite number.
///
/// `floor(MonkLVL/2)*10` reproduces the previously hand-written 20/30/40/
/// 50/60 ladder (levels 4/6/8/10/12) exactly, and extends it honestly:
/// 70/80/90 ft at levels 14/16/18. **Level 20 is not 100 ft** -- PF1's own
/// rule text at 20th level reads "he can use a nearby wall to slow his
/// descent and fall any distance without harm" rather than continuing the
/// arithmetic progression, verified independently against both primary
/// sources (task #49). Callers must reach this only at or above the Slow
/// Fall grant level (4th); below it the feature does not exist.
pub(super) fn monk_slow_fall_reach_feet(level: u8) -> Option<i16> {
    if level >= MONK_SLOW_FALL_ANY_DISTANCE_LEVEL {
        return None;
    }
    Some(i16::from(level) / 2 * 10)
}

pub(super) fn explain_monk_level1_chassis(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(level) = supported_monk_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Monk martial
    // chassis identity at the supported level. This is a recognition record only;
    // it fabricates no mechanical value.
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.bounded_progression".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Monk level {level} martial chassis: the \
             {MONK_CLASS_ID}:{level} class identity is acknowledged as a pure non-hybrid martial \
             baseline on the rules-core seam rather than an undocumented packet placeholder. \
             This is a bounded chassis-recognition record only; the base-attack, base-save, AC \
             Bonus, unarmed-strike-die, Flurry of Blows flat-surface, Evasion, and Still Mind \
             values are grounded separately below, and this record itself grounds no level-1 \
             bonus feat grant, no attack-resolution engine, no ki pool, and no level-4+ martial \
             progression, so it carries no fabricated mechanical value (+0)"
        ),
    });

    let level_value = i16::from(level);

    // Grounded (1/6): Monk 3/4-BAB base-attack progression from the PF1 Core
    // Rulebook Monk class table. No PCGen cr_classes.lst entry is used here (this
    // repo carries no Monk .lst source), so the formula cites the rulebook table
    // directly rather than inventing a line reference.
    let base_attack_bonus = level_value * 3 / 4;
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Monk level {level} base attack bonus from the PF1 Core Rulebook Monk class table's \
             3/4-BAB progression: classlevel * 3 / 4 = {base_attack_bonus}"
        ),
    });

    // Grounded (2/6): Monk base-save progression. Unlike Fighter/Barbarian/Rogue's
    // 2-good/1-poor or 1-good/2-poor split, the PF1 Core Rulebook Monk class table
    // gives all three base saves (Fortitude, Reflex, and Will) the good progression.
    let base_save_value = level_value / 2 + 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.base_save.fortitude".to_owned(),
        value: base_save_value,
        detail: format!(
            "Monk level {level} base Fortitude save from the PF1 Core Rulebook Monk class \
             table: Monk is unusual in having all three saves good (unlike Fighter's/\
             Barbarian's/Rogue's mixed good/poor split), so Fortitude uses the good-save formula \
             classlevel/2+2 = {base_save_value}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.base_save.reflex".to_owned(),
        value: base_save_value,
        detail: format!(
            "Monk level {level} base Reflex save from the PF1 Core Rulebook Monk class table: \
             Monk is unusual in having all three saves good (unlike Fighter's/Barbarian's/\
             Rogue's mixed good/poor split), so Reflex uses the good-save formula \
             classlevel/2+2 = {base_save_value}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.base_save.will".to_owned(),
        value: base_save_value,
        detail: format!(
            "Monk level {level} base Will save from the PF1 Core Rulebook Monk class table: \
             Monk is unusual in having all three saves good (unlike Fighter's/Barbarian's/\
             Rogue's mixed good/poor split), so Will uses the good-save formula \
             classlevel/2+2 = {base_save_value}"
        ),
    });

    // Grounded (3/6): AC Bonus (Wisdom-to-AC + level-4+ dodge progression). PF1:
    // "she adds her Wisdom bonus, if any, to her AC" (only a positive Wisdom
    // modifier is added, never subtracted for a negative Wisdom modifier) "and
    // starting at 4th level, she gains a further +1 dodge bonus... every four
    // levels thereafter... increases by a further +1" (`monk_ac_bonus_dodge_progression`,
    // AT-33-E5-003 fix — confirmed against the real pinned PCGen oracle:
    // `core_rulebook:class_feature:monk_ac_bonus` at L20 exports `7`, not the
    // Wisdom-only component alone). This still grounds no "unarmored and
    // unencumbered" runtime state-check engine (no such engine exists anywhere
    // in this codebase yet), so the value is asserted unconditionally on the
    // deterministic Human Monk fixture, which is by construction unarmored.
    let ac_bonus_dodge = monk_ac_bonus_dodge_progression(level);
    let ac_bonus = ability_modifiers.wisdom.max(0) + ac_bonus_dodge;
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.ac_bonus".to_owned(),
        value: ac_bonus,
        detail: format!(
            "Monk level {level} AC Bonus: Wisdom bonus (if positive) added to AC and CMD while \
             unarmored and unencumbered = max({}, 0), plus the level-4+ dodge-bonus progression \
             ({ac_bonus_dodge} at this level) = {ac_bonus}. This still grounds no \
             \"unarmored and unencumbered\" runtime state-check engine (none exists in this \
             codebase yet); the value is asserted unconditionally on the deterministic Human Monk \
             fixture, which is by construction unarmored",
            ability_modifiers.wisdom
        ),
    });

    // Grounded (4/6): unarmed strike damage die. PF1 Core Rulebook Monk class table:
    // a Medium monk deals 1d6 unarmed strike damage at levels 1-3, stepping up to
    // 1d8 at levels 4-7, then 1d10 at levels 8-11, then 2d6 at levels 12-15
    // (verified independently against d20pfsrd and the Archives of Nethys
    // aonprd.com mirror: the full Medium-monk progression is
    // 1d6/1d8/1d10/2d6/2d8/2d10 at levels 1-3/4-7/8-11/12-15/16-19/20). Levels
    // 1-11 grounded only the die-size facet (a single die throughout); level 12
    // is the first level at which the die COUNT itself rises (from one die to
    // two), so a second, standalone die-count facet is grounded starting here,
    // mirroring the Flurry of Blows attack-bonus/attack-count split — no damage
    // roll, damage total, or attack-resolution engine is computed, and the
    // level-16+ die progression (2d8 and beyond) is not grounded.
    let (unarmed_die_value, unarmed_die_count, unarmed_die_name) =
        monk_unarmed_strike_damage_die(level);
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.unarmed_strike_damage_die".to_owned(),
        value: unarmed_die_value,
        detail: format!(
            "Monk level {level} unarmed strike from the PF1 Core Rulebook Monk class table: a \
             Medium monk deals 1d6 unarmed strike damage at levels 1-3, stepping up to 1d8 at \
             levels 4-7, to 1d10 at levels 8-11, and to 2d6 at levels 12-15, so it is \
             {unarmed_die_name} at level {level}. Only the die-face-size facet \
             ({unarmed_die_value}, i.e. the d{unarmed_die_value} in {unarmed_die_name}) is \
             grounded on this record (the die-count facet is grounded separately below); no \
             damage roll or damage total is computed and no attack-resolution engine exists. Two \
             PF1 unarmed-strike rules are recorded as statements only: the monk may choose to \
             deal lethal or nonlethal damage with no penalty on the attack roll, and monk \
             unarmed strikes carry no off-hand penalty (a monk applies her full Strength bonus \
             on damage rolls for all her unarmed strikes). The higher-level unarmed damage die \
             progression beyond level 15 (2d8 and beyond) is not grounded"
        ),
    });
    // Grounded (SD18): the unarmed strike damage die's COUNT facet, standalone
    // from the die-face-size facet above. Every level 1-11 band is implicitly a
    // single die, never previously surfaced as its own record; level 12 is the
    // first level at which the PF1 Core Rulebook Medium-monk unarmed damage
    // progression rises the die count itself (to 2d6), verified independently
    // against both primary sources' Medium-monk unarmed damage progression
    // table. Mirroring the Improved Evasion idiom (no record at all below its
    // introduction gate, rather than an explicit level-gate-absence record):
    // this new facet is grounded starting only at its introduction level, since
    // it names a genuinely new computational dimension (die count), not a PF1
    // rule-text-named feature with its own "correctly absent below" wording.
    // Mirrors the Flurry of Blows attack-bonus/attack-count split: only the
    // count facet is grounded here, no damage roll or damage total is
    // computed.
    if level >= MONK_UNARMED_DAMAGE_DIE_THIRD_STEP_UP_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.unarmed_strike_damage_die_count".to_owned(),
            value: unarmed_die_count,
            detail: format!(
                "Monk level {level} unarmed strike damage die count from the PF1 Core Rulebook \
                 Monk class table: the die count genuinely rises to 2 at level 12 (the 2d6 band \
                 spans levels 12-15), so the count is {unarmed_die_count} at level {level}. Only \
                 this count facet is grounded; the die-face-size facet is grounded separately \
                 above, and no damage roll, damage total, or attack-resolution engine is \
                 computed"
            ),
        });
    }

    // Grounded (5/6): Flurry of Blows flat attack surface, in two facets. PF1 Core
    // Rulebook: when making a flurry of blows as a full-attack action, the monk uses
    // her monk level in place of her base attack bonus and takes a -2 penalty on all
    // attacks; the flat pre-ability-modifier attack bonus is monk level - 2 (-1 at
    // level 1, +0 at level 2, +1 at level 3, ..., +5 at level 7, +6 at level 8,
    // matching the PF1 CRB table's "-1/-1" through "+6/+6" entries), and the flurry
    // grants two attacks at levels 1-7, rising to three attacks at level 8 —
    // verified independently against both primary sources' verbatim Flurry of Blows
    // rule text ("At 8th level, the monk can make two additional attacks when he
    // uses flurry of blows, as if using Improved Two-Weapon Fighting"). Only these
    // flat facets are grounded; no attack-resolution engine, no monk-weapon flurry,
    // and no wiring into integrated combat totals is implemented.
    let flurry_attack_bonus = level_value - 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.flurry_of_blows_attack_bonus".to_owned(),
        value: flurry_attack_bonus,
        detail: format!(
            "Monk level {level} Flurry of Blows flat attack modifier from the PF1 Core Rulebook: \
             when using flurry as a full-attack action the monk uses her monk level in place of \
             her base attack bonus and takes a -2 penalty on all attack rolls, so the flat \
             modifier is monk level - 2 = {level_value} - 2 = {flurry_attack_bonus} on each \
             flurry attack, before ability modifiers. Only this flat pre-ability modifier is \
             grounded; no attack-resolution engine, no monk-weapon flurry, and no wiring into \
             integrated combat totals is implemented"
        ),
    });
    let flurry_attack_count = monk_flurry_of_blows_attack_count(level);
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.flurry_of_blows_attack_count".to_owned(),
        value: flurry_attack_count,
        detail: format!(
            "Monk level {level} Flurry of Blows attack count from the PF1 Core Rulebook: a \
             level-{level} flurry grants {additional_attacks} on a full attack, i.e. \
             {attack_count_words}, each at the flat pre-ability modifier grounded separately. \
             The attack count stays 2 at levels 1-7 and rises to 3 at level 8 — verified \
             independently against both primary sources' verbatim Flurry of Blows rule text \
             (\"At 8th level, the monk can make two additional attacks when he uses flurry of \
             blows, as if using Improved Two-Weapon Fighting\"). Only the count facet \
             ({flurry_attack_count}) is grounded; no attack-resolution engine and no \
             monk-weapon flurry support is implemented",
            additional_attacks = if level < MONK_FLURRY_THIRD_ATTACK_LEVEL {
                "one additional attack"
            } else {
                "two additional attacks"
            },
            attack_count_words = if level < MONK_FLURRY_THIRD_ATTACK_LEVEL {
                "two attacks"
            } else {
                "three attacks"
            }
        ),
    });

    // Grounded (6/6, SD13-E5): Evasion, a 2nd-level Monk class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Bonus feat, evasion" as the Monk 2nd-level
    // special feature entry — the same rule text and level gate as Rogue's own
    // Evasion). Below the level-2 gate this is a correct PF1 Core Rulebook
    // level-gate absence (value 0); at or above it, it is a bounded
    // identity/recognition record only (value 0, non-fabricated) naming the rule
    // text — mirroring exactly how Rogue's own `class_feature.rogue.evasion` was
    // grounded, without folding into an actual saving-throw-resolution or
    // damage-resolution engine, neither of which exists in this codebase.
    if level < MONK_EVASION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.evasion".to_owned(),
            value: 0,
            detail: format!(
                "Monk Evasion at monk level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant rule is named but not computed. Evasion \
                 is a 2nd-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.evasion".to_owned(),
            value: 0,
            detail: format!(
                "Monk Evasion granted at monk level {level} (PF1 Core Rulebook, 2nd-level monk \
                 class feature): if the monk makes a successful Reflex saving throw against an \
                 attack that normally deals half damage on a successful save, she instead takes \
                 no damage; Evasion has no effect if the monk fails the saving throw, and it has \
                 no effect at all against attacks that do not allow a saving throw for half \
                 damage. This is a bounded identity/recognition record only (value 0, \
                 non-fabricated): no saving-throw-resolution engine and no damage-resolution \
                 engine exists anywhere in this codebase to apply it, so this grounds no actual \
                 damage reduction on any save outcome"
            ),
        });
    }

    // Grounded (SD13-E5 level-9 slice): Improved Evasion, the 9th-level Monk
    // class feature verified independently against two primary PF1 sources
    // (d20pfsrd and legacy.aonprd.com both list "Improved evasion" as the Monk
    // 9th-level "Special" entry). An upgrade of the 2nd-level Evasion identity:
    // the monk still takes no damage on a successful Reflex save, and
    // henceforth takes only HALF damage on a failed save. Grounded as a bounded
    // +0 identity/recognition record only below/at the gate, mirroring exactly
    // how Evasion itself and Rogue's Improved Uncanny Dodge were grounded — no
    // saving-throw-resolution or damage-resolution engine exists in this
    // codebase, so no damage math is fabricated from the record. Below the
    // level-9 gate no record is pushed at all (the level-8 slice's own negative
    // control pins that absence).
    if level >= MONK_IMPROVED_EVASION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.improved_evasion".to_owned(),
            value: 0,
            detail: format!(
                "Monk Improved Evasion granted at monk level {level} (PF1 Core Rulebook, \
                 9th-level monk class feature): the monk's Evasion improves — she still takes \
                 no damage on a successful Reflex saving throw against attacks, and henceforth \
                 takes only half damage on a failed save. This is a bounded \
                 identity/recognition record only (value 0, non-fabricated): no \
                 saving-throw-resolution engine and no damage-resolution engine exists anywhere \
                 in this codebase to apply it, so this grounds no actual damage reduction on \
                 any save outcome"
            ),
        });
    }

    // Grounded (SD13-E5): Still Mind, a 3rd-level Monk class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Fast movement, maneuver training, still mind"
    // as the Monk 3rd-level special feature entry). Below the level-3 gate this is
    // a correct PF1 Core Rulebook level-gate absence (value 0); at or above it, it
    // is a bounded flat-magnitude record only (a flat +2, not level-scaled) naming
    // the rule text — mirroring the Fighter Bravery / Paladin Divine Grace / Rogue
    // Trap Sense idiom: never applied to any actual save total, since no
    // saving-throw-resolution engine exists anywhere in this codebase. Fast
    // Movement and Maneuver Training, the class table's other two 3rd-level
    // "Special" column entries, are deliberately left named-but-unproven this
    // slice: no speed-total engine and no CMB/CMD engine exist in this codebase to
    // attach either to.
    if level < MONK_STILL_MIND_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.still_mind".to_owned(),
            value: 0,
            detail: format!(
                "Monk Still Mind at monk level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant magnitude is named but not computed. \
                 Still Mind is a 3rd-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.still_mind".to_owned(),
            value: 2,
            detail: format!(
                "Monk Still Mind granted at monk level {level} (PF1 Core Rulebook, 3rd-level \
                 monk class feature): a monk of 3rd level or higher gains a flat +2 bonus on \
                 saving throws against enchantment spells and effects; this magnitude does not \
                 scale further with level. This is a bounded flat-magnitude record only, \
                 non-fabricated: it is never applied to any actual save total, since no \
                 saving-throw-resolution engine exists anywhere in this codebase to apply it"
            ),
        });
    }

    // Grounded (SD13-E5): the ki pool's flat size, a 4th-level Monk class
    // feature verified independently against two primary PF1 sources (d20pfsrd
    // and legacy.aonprd.com both give the formula: "the number of points in a
    // monk's ki pool is equal to 1/2 his monk level + his Wisdom modifier" —
    // neither primary source states a minimum floor on the pool itself, unlike
    // some other flat-magnitude records this codebase grounds elsewhere).
    // Below the level-4 gate this is a correct PF1 Core Rulebook level-gate
    // absence (value 0); at or above it, it is a bounded flat-magnitude record
    // only (the standalone pool-size number), mirroring the Barbarian rage
    // rounds-per-day / Paladin lay-on-hands-uses-per-day idiom: this grounds
    // only the flat resource-count magnitude. No ki-point consumption tracking,
    // no action-economy engine, and no application of any ki power (the extra
    // attack, the +4 AC dodge bonus, or the +20-ft. speed bonus usable as a
    // swift action) is computed anywhere in this codebase.
    if level < MONK_KI_POOL_AND_SLOW_FALL_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.ki_pool_size".to_owned(),
            value: 0,
            detail: format!(
                "Monk ki pool at monk level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant formula is named but not computed. The \
                 ki pool is a 4th-level monk class feature."
            ),
        });
    } else {
        let extra_ki_bonus = extra_resource_feat_bonus(
            &input.chosen.selected_feats,
            EXTRA_KI_FEAT_KEY,
            EXTRA_POINTS_PER_DAY,
        );
        let ki_pool_size = level_value / 2 + ability_modifiers.wisdom + extra_ki_bonus;
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.ki_pool_size".to_owned(),
            value: ki_pool_size,
            detail: format!(
                "Monk ki pool granted at monk level {level} (PF1 Core Rulebook, 4th-level monk \
                 class feature): \"the number of points in a monk's ki pool is equal to 1/2 his \
                 monk level + his Wisdom modifier\" = {level_value} / 2 + {} + Extra Ki feat \
                 (+{extra_ki_bonus}) = {ki_pool_size}. \
                 This grounds only the flat pool-size number; it computes no ki-point \
                 consumption tracking, no action-economy engine, and no application of any ki \
                 power (the extra attack, the +4 AC dodge bonus, or the +20-ft. speed bonus \
                 usable as a swift action), none of which exists anywhere in this codebase",
                ability_modifiers.wisdom
            ),
        });
    }

    // Grounded (SD13-E5): Slow Fall, the other 4th-level Monk class feature
    // verified independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Ki pool (magic), slow fall 20 ft." as the
    // Monk 4th-level special feature entry). Below the level-4 gate this is a
    // correct PF1 Core Rulebook level-gate absence (value 0); at or above it,
    // it is a bounded grant-only identity record (value 0, non-fabricated)
    // naming the rule text — mirroring the Barbarian Uncanny Dodge / Rogue
    // Uncanny Dodge / Druid Woodland Stride grant-only idiom: no
    // fall-damage-resolution engine exists anywhere in this codebase to apply
    // the 20-foot reduction to.
    if level < MONK_KI_POOL_AND_SLOW_FALL_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.slow_fall".to_owned(),
            value: 0,
            detail: format!(
                "Monk Slow Fall at monk level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant rule is named but not computed. Slow \
                 Fall is a 4th-level monk class feature."
            ),
        });
    } else {
        let slow_fall_reach = monk_slow_fall_reach_feet(level);
        let slow_fall_reach_text = match slow_fall_reach {
            Some(feet) => format!("{feet} feet"),
            None => "any distance".to_owned(),
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.slow_fall".to_owned(),
            value: 0,
            detail: format!(
                "Monk Slow Fall granted at monk level {level} (PF1 Core Rulebook, 4th-level monk \
                 class feature whose own reach magnitude is floor(monk level / 2) * 10 ft, \
                 rising 10 ft every even level thereafter -- to 30 ft. at 6th level, 40 ft. at \
                 8th level, 50 ft. at 10th level, 60 ft. at 12th level, 70 ft. at 14th level, 80 \
                 ft. at 16th level, and 90 ft. at 18th level -- and becoming ANY distance at 20th \
                 level, which the feature's own rule text states outright rather than continuing \
                 the progression to a finite 100): \"a monk within arm's reach of a wall can use \
                 it to slow his descent\" — she takes falling damage as if the fall were \
                 {slow_fall_reach_text} shorter than it actually is. This is a bounded \
                 grant-only identity record only (value 0, non-fabricated): no \
                 fall-damage-resolution engine exists anywhere in this codebase to apply the \
                 {slow_fall_reach_text} reduction to"
            ),
        });
    }

    // Grounded (SD13-E5): Purity of Body, a 5th-level Monk class feature
    // verified independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "High jump, purity of body" as the Monk
    // 5th-level special feature entry). Below the level-5 gate this is a
    // correct PF1 Core Rulebook level-gate absence (value 0); at or above it,
    // it is a bounded grant-only identity record (value 0, non-fabricated)
    // naming the rule text — mirroring the Barbarian/Rogue Uncanny Dodge /
    // Monk Slow Fall grant-only idiom: no disease-resolution engine exists
    // anywhere in this codebase to apply the immunity to. High Jump, the
    // level-5 class table's OTHER "Special" column entry, is now grounded
    // (task #36) -- see `monk_high_jump_acrobatics_bonus`. The earlier
    // deferral here was stale on two counts: it required an
    // "Acrobatics-check total" to wire into, which the corrected
    // standalone-grounding bar (risks item 52) no longer demands, and it
    // cited the ki cost as blocking the whole ability when the record's own
    // DESC scopes the ki spend to a SEPARATE +20 clause. The flat +MonkLVL
    // is unconditional; only the +20 boost costs ki, and only that half
    // stays deferred.
    if level < MONK_PURITY_OF_BODY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.purity_of_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Purity of Body at monk level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Purity of Body is a 5th-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.purity_of_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Purity of Body granted at monk level {level} (PF1 Core Rulebook, 5th-level \
                 monk class feature): \"at 5th level, a monk gains immunity to all diseases, \
                 including supernatural and magical diseases.\" This is a bounded grant-only \
                 identity record only (value 0, non-fabricated): no disease-resolution engine \
                 exists anywhere in this codebase to apply the immunity to. High Jump, the \
                 level-5 class table's other \"Special\" column entry, is grounded separately \
                 as class_chassis.monk.high_jump (task #36)"
            ),
        });
    }

    // Task #36: four real, in-range, corpus-verified Monk features. All four
    // level gates read directly off `cr_abilities_class.lst`'s own grant
    // lines (`PREVARGTEQ:Monk_CFP_Level,{3,3,5,7}`), all well under
    // MAX_SUPPORTED_MONK_LEVEL = 12.
    if level < MONK_FAST_MOVEMENT_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.fast_movement".to_owned(),
            value: 0,
            detail: format!(
                "Monk Fast Movement at monk level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant magnitude is named but not computed. \
                 Fast Movement is a 3rd-level monk class feature."
            ),
        });
    } else {
        let feet = monk_fast_movement_bonus_feet(level);
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.fast_movement".to_owned(),
            value: feet,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   The corpus applies it as BONUS:MOVEADD|TYPE=Walk with TYPE=Enhancement, gated
                //   on PREVAREQ:ENCUMBERANCE,0 and zero equipped armor: a monk in armor or under a
                //   medium/heavy load loses it entirely.
                "Monk Fast Movement at monk level {level}: a +{feet} ft enhancement bonus to land \
                 speed (10*floor(MonkLVL/3), so +10 at levels 3-5, +20 at 6-8, +30 at 9-11, +40 at \
                 12). Unlike Barbarian's own Fast Movement, which is a flat +10 at every level, the \
                 monk's SCALES -- same feature name, different magnitude. That condition is NOT \
                 enforced here -- this engine models no encumbrance state -- so this grounds the \
                 magnitude a qualifying monk gets, and the armor/load condition is named rather than \
                 applied. Worth knowing the deterministic fixture wears a Chain Shirt, which would \
                 suppress this bonus for a real character in that posture"
            ),
        });
    }

    if level < MONK_MANEUVER_TRAINING_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.maneuver_training_cmb_bonus".to_owned(),
            value: 0,
            detail: format!(
                "Monk Maneuver Training at monk level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant magnitude is named but not computed. \
                 Maneuver Training is a 3rd-level monk class feature."
            ),
        });
    } else {
        let cmb_bonus = monk_maneuver_training_cmb_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.maneuver_training_cmb_bonus".to_owned(),
            value: cmb_bonus,
            detail: format!(
                "Monk Maneuver Training at monk level {level}: +{cmb_bonus} to combat maneuver \
                 bonus only (corpus CMB_BAB = MonkLVL - MonkLVL*3/4). A monk uses his full \
                 monk level in place of his 3/4 base attack bonus when calculating CMB, so the \
                 corpus expresses the feature as the DELTA to add on top of the BAB already \
                 counted -- 1 at levels 3-4, 2 at 5-8, 3 at 9-12. This grounds that delta as a \
                 standalone record; no Combat Maneuver Bonus total exists in this codebase to \
                 add it into, the same reason Brawler's own Maneuver Training grounds \
                 standalone"
            ),
        });
    }

    if level < MONK_HIGH_JUMP_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.high_jump".to_owned(),
            value: 0,
            detail: format!(
                "Monk High Jump at monk level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant magnitude is named but not computed. \
                 High Jump is a 5th-level monk class feature."
            ),
        });
    } else {
        let jump_bonus = monk_high_jump_acrobatics_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.high_jump".to_owned(),
            value: jump_bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:SITUATION|Acrobatics=When Jumping|HighJumpBonus, where HighJumpBonus =
                //   MonkLVL
                "Monk High Jump at monk level {level}: +{jump_bonus} on all Acrobatics checks made \
                 to jump, vertical and horizontal. The monk also always counts as having a running \
                 start. This flat bonus is UNCONDITIONAL -- the record's own DESC scopes the ki cost \
                 to a separate clause (\"By spending 1 point from your ki pool as a swift action, \
                 you gain a +20 bonus\"), and only that +20 boost stays deferred. Grounded as a \
                 standalone record: Acrobatics is not among the three skills \
                 compute_selected_skill_modifiers tracks (Climb/Intimidate/Swim), so there is no \
                 check total to layer it onto"
            ),
        });
    }

    if level < MONK_WHOLENESS_OF_BODY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.wholeness_of_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Wholeness of Body at monk level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant magnitude is named but not computed. \
                 Wholeness of Body is a 7th-level monk class feature."
            ),
        });
    } else {
        let healing = monk_wholeness_of_body_healing(level);
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.wholeness_of_body".to_owned(),
            value: healing,
            detail: format!(
                "Monk Wholeness of Body at monk level {level}: heals {healing} hit points of \
                 damage as a standard action, for 2 points from the ki pool (corpus \
                 WholenessOfBody = WholenessOfBodyLVL = MonkLVL). Grounded as a standalone \
                 magnitude: this engine computes a maximum-hit-point total but tracks no \
                 current damage to heal, and no ki-spending resource engine exists, so the \
                 2-ki cost and the action are named rather than enforced"
            ),
        });
    }

    // Grounded (SD18): Diamond Body, an 11th-level Monk class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Diamond body" as the sole Monk 11th-level
    // "Special" column entry). Below the level-11 gate this is a correct PF1
    // Core Rulebook level-gate absence (value 0); at or above it, it is a
    // bounded grant-only identity record (value 0, non-fabricated) naming the
    // rule text — mirroring the Purity of Body grant-only idiom exactly: no
    // poison-resolution engine exists anywhere in this codebase to apply the
    // immunity to.
    if level < MONK_DIAMOND_BODY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.diamond_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Diamond Body at monk level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Diamond Body is an 11th-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.diamond_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Diamond Body granted at monk level {level} (PF1 Core Rulebook, 11th-level \
                 monk class feature): \"at 11th level, a monk gains immunity to all poisons.\" \
                 This is a bounded grant-only identity record only (value 0, non-fabricated): no \
                 poison-resolution engine exists anywhere in this codebase to apply the immunity \
                 to."
            ),
        });
    }

    // Task #49: seven capstone-band Monk features, all newly reachable now
    // that `MAX_SUPPORTED_MONK_LEVEL` is 20 (Abundant Step's gate, 12, was
    // already inside the OLD 1..=12 range -- a pre-existing gap, not one
    // this widening itself admits, but nobody built it while the cap
    // already covered it, so it is grounded here alongside the other six).
    // Every gate and formula below is cross-checked independently against
    // both primary sources (d20pfsrd and the Archives of Nethys aonprd.com
    // mirror), matching the two-branch "ground the absence, don't omit it"
    // convention task #46 established for this function: an explicit
    // value-0 "correctly absent below gate" record below, and the real
    // magnitude (or a bounded grant-only identity record, for the four
    // features with no PF1 numeric formula token) at or above it.

    // Abundant Step (12th level): "he can slip magically between spaces,
    // as if using the spell dimension door" for 2 ki points as a move
    // action; "his caster level for this effect is equal to his monk
    // level" (corpus AbundantStepCasterLVL = MonkLVL). Only the caster-level
    // magnitude is grounded; no ki-point-spending action-economy engine and
    // no dimension-door-equivalent teleportation-resolution engine exist
    // anywhere in this codebase, so the effect itself stays named-but-
    // unexecuted, mirroring the Wholeness of Body / High Jump precedent of
    // grounding the one flat number a feature's own rule text names while
    // leaving its execution unmodeled.
    if level < MONK_ABUNDANT_STEP_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.abundant_step_caster_level".to_owned(),
            value: 0,
            detail: format!(
                "Monk Abundant Step at monk level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant magnitude is named but not computed. \
                 Abundant Step is a 12th-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.abundant_step_caster_level".to_owned(),
            value: level_value,
            detail: format!(
                "Monk Abundant Step granted at monk level {level} (PF1 Core Rulebook, 12th-level \
                 monk class feature): for 2 ki points as a move action, the monk can slip \
                 magically between spaces as if using the spell dimension door, using her monk \
                 level as caster level for the effect (corpus AbundantStepCasterLVL = MonkLVL = \
                 {level_value}). Only this caster-level magnitude is grounded; no \
                 ki-point-spending action-economy engine and no dimension-door-equivalent \
                 teleportation-resolution engine exist anywhere in this codebase, so the \
                 dimension-door effect itself stays named but not executed"
            ),
        });
    }

    // Diamond Soul (13th level): "a monk gains spell resistance equal to
    // his current monk level + 10" (corpus DiamondSoul = 10 + MonkLVL).
    // Grounded as a standalone flat-magnitude record; no spell-resistance
    // total or caster-level-check-vs-SR resolution engine exists anywhere
    // in this codebase to apply it to, mirroring the ki pool's own
    // standalone-magnitude idiom.
    if level < MONK_DIAMOND_SOUL_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.diamond_soul_spell_resistance".to_owned(),
            value: 0,
            detail: format!(
                "Monk Diamond Soul at monk level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant magnitude is named but not computed. \
                 Diamond Soul is a 13th-level monk class feature."
            ),
        });
    } else {
        let diamond_soul_sr = 10 + level_value;
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.diamond_soul_spell_resistance".to_owned(),
            value: diamond_soul_sr,
            detail: format!(
                "Monk Diamond Soul granted at monk level {level} (PF1 Core Rulebook, 13th-level \
                 monk class feature): \"a monk gains spell resistance equal to his current monk \
                 level + 10\" = {level_value} + 10 = {diamond_soul_sr}; a spellcaster must \
                 succeed on a caster level check (1d20 + caster level vs. this SR) to affect the \
                 monk with a spell. This grounds only the flat SR magnitude; no spell-resistance \
                 total or caster-level-check-vs-SR resolution engine exists anywhere in this \
                 codebase to apply it to"
            ),
        });
    }

    // Quivering Palm (15th level), two facets: the Fortitude save DC
    // (corpus: 10 + 1/2 monk level + Wisdom modifier) and the duration
    // during which the monk may will the target to die (corpus: a number
    // of days equal to her monk level). Only these two flat magnitudes are
    // grounded; the announce-before-attacking action economy, the
    // triggering unarmed strike, and the target's own Fortitude save are
    // all unmodeled -- no save-or-die resolution engine exists anywhere in
    // this codebase.
    if level < MONK_QUIVERING_PALM_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.quivering_palm_dc".to_owned(),
            value: 0,
            detail: format!(
                "Monk Quivering Palm DC at monk level {level}: correctly absent at level {level} \
                 by PF1 Core Rulebook level gate; the at-grant formula is named but not \
                 computed. Quivering Palm is a 15th-level monk class feature."
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.quivering_palm_duration_days".to_owned(),
            value: 0,
            detail: format!(
                "Monk Quivering Palm duration at monk level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant formula is named but not \
                 computed. Quivering Palm is a 15th-level monk class feature."
            ),
        });
    } else {
        let quivering_palm_dc = 10 + level_value / 2 + ability_modifiers.wisdom;
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.quivering_palm_dc".to_owned(),
            value: quivering_palm_dc,
            detail: format!(
                "Monk Quivering Palm granted at monk level {level} (PF1 Core Rulebook, \
                 15th-level monk class feature, usable once per day): once per day, the monk \
                 may announce she is using it before making an attack; if the attack hits and \
                 deals damage, the target must succeed on a Fortitude save (DC 10 + 1/2 monk \
                 level + Wisdom modifier = 10 + {level_value} / 2 + {} = {quivering_palm_dc}) or \
                 be slain later at the monk's will. Only this DC magnitude is grounded; the \
                 announce-before-attacking action economy, the triggering unarmed strike, and \
                 the target's own Fortitude save are unmodeled -- no save-or-die resolution \
                 engine exists anywhere in this codebase",
                ability_modifiers.wisdom
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.quivering_palm_duration_days".to_owned(),
            value: level_value,
            detail: format!(
                "Monk Quivering Palm granted at monk level {level} (PF1 Core Rulebook, \
                 15th-level monk class feature): within a number of days equal to her monk \
                 level ({level_value}) after a successful Quivering Palm strike, the monk can \
                 will the affected target to die as a free action (absent a successful \
                 Fortitude save). This grounds only the flat day-count magnitude; no calendar, \
                 duration-tracking, or save-or-die resolution engine exists anywhere in this \
                 codebase to apply it to"
            ),
        });
    }

    // Timeless Body (17th level): "no longer takes penalties to his \
    // ability scores for aging and cannot be magically aged" (existing \
    // penalties remain; age-related bonuses continue to accrue). No PF1 \
    // numeric formula token exists for this feature -- it is a pure state
    // grant -- so it is grounded as a bounded grant-only identity record
    // only (value 0, non-fabricated), mirroring the Diamond Body / Purity
    // of Body grant-only idiom exactly: no aging-penalty-application engine
    // exists anywhere in this codebase to apply the immunity to.
    if level < MONK_TIMELESS_BODY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.timeless_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Timeless Body at monk level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Timeless Body is a 17th-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.timeless_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Timeless Body granted at monk level {level} (PF1 Core Rulebook, 17th-level \
                 monk class feature): \"a monk no longer takes penalties to his ability scores \
                 for aging and cannot be magically aged. Any penalties he had already suffered, \
                 however, remain in place. Bonuses still accrue.\" This is a bounded grant-only \
                 identity record only (value 0, non-fabricated): no aging-penalty-application \
                 engine exists anywhere in this codebase to apply the immunity to."
            ),
        });
    }

    // Tongue of the Sun and Moon (17th level): "a monk of 17th level or \
    // higher can speak with any living creature." No PF1 numeric formula
    // token exists for this feature; grounded as a bounded grant-only
    // identity record only, mirroring the same idiom.
    if level < MONK_TONGUE_OF_SUN_AND_MOON_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.tongue_of_the_sun_and_moon".to_owned(),
            value: 0,
            detail: format!(
                "Monk Tongue of the Sun and Moon at monk level {level}: correctly absent at \
                 level {level} by PF1 Core Rulebook level gate; the at-grant rule is named but \
                 not computed. Tongue of the Sun and Moon is a 17th-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.tongue_of_the_sun_and_moon".to_owned(),
            value: 0,
            detail: format!(
                "Monk Tongue of the Sun and Moon granted at monk level {level} (PF1 Core \
                 Rulebook, 17th-level monk class feature): \"a monk of 17th level or higher can \
                 speak with any living creature.\" This is a bounded grant-only identity record \
                 only (value 0, non-fabricated): no creature-communication engine exists \
                 anywhere in this codebase to apply it to."
            ),
        });
    }

    // Empty Body (19th level): "the monk can use ki points to assume an \
    // ethereal state ... as though using the spell etherealness," \
    // consuming 3 ki points for 1 minute, affecting only herself -- both
    // the ki cost and the duration are FIXED (not level-scaled), unlike
    // Wholeness of Body's MonkLVL-scaled healing, so there is no
    // level-varying magnitude to ground; grounded as a bounded grant-only
    // identity record only, mirroring the Diamond Body idiom: no
    // ki-point-consumption engine and no etherealness-resolution engine
    // exist anywhere in this codebase to apply it to.
    if level < MONK_EMPTY_BODY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.empty_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Empty Body at monk level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Empty Body is a 19th-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.empty_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Empty Body granted at monk level {level} (PF1 Core Rulebook, 19th-level \
                 monk class feature): \"the monk can use ki points to assume an ethereal state \
                 for 1 minute, as though using the spell etherealness. This ability costs 3 \
                 points from the monk's ki pool and affects only the monk.\" Both the 3-ki cost \
                 and the 1-minute duration are FIXED, not level-scaled, so there is no \
                 level-varying magnitude to ground. This is a bounded grant-only identity record \
                 only (value 0, non-fabricated): no ki-point-consumption engine and no \
                 etherealness-resolution engine exist anywhere in this codebase to apply it to."
            ),
        });
    }

    // Perfect Self (20th level, task #49 -- previously PROVABLY DEAD CODE
    // under the old MAX_SUPPORTED_MONK_LEVEL = 12, since its 20th-level
    // gate sat entirely above the old ceiling and `explain_monk_level1_chassis`
    // could never be reached at level 20 at all; now genuinely reachable
    // and built for real). "At 20th level, a monk's body becomes a
    // perfect weapon... the monk gains damage reduction 10/chaotic" (corpus
    // DR:10/Chaotic on the level-20 AUTOMATIC grant record). Only the DR
    // magnitude is grounded, as a standalone flat number; no
    // damage-reduction-application/damage-resolution engine exists
    // anywhere in this codebase to apply it to, mirroring exactly how
    // Skald's and Bloodrager's own damage-reduction records were grounded
    // standalone. Perfect Self's OTHER clause -- the monk is forevermore
    // treated as an outsider rather than her previous creature type, for
    // spells and magical effects, while still being returnable from the
    // dead as her previous type -- carries no numeric magnitude at all and
    // needs a creature-type-conditioned spell-effect-resolution engine that
    // does not exist here, so it stays deferred regardless of level (this
    // is the same conclusion the earlier attempt reached, just reached now
    // for a genuinely reachable feature instead of one masked by dead code).
    if level < MONK_PERFECT_SELF_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.perfect_self_damage_reduction".to_owned(),
            value: 0,
            detail: format!(
                "Monk Perfect Self damage reduction at monk level {level}: correctly absent at \
                 level {level} by PF1 Core Rulebook level gate; the at-grant magnitude is named \
                 but not computed. Perfect Self is a {MONK_PERFECT_SELF_LEVEL}th-level monk \
                 class feature granting DR {MONK_PERFECT_SELF_DAMAGE_REDUCTION}/chaotic (corpus \
                 DR:10/Chaotic). Its other clause -- the monk is treated as an outsider rather \
                 than her previous creature type for spells and magical effects, while still \
                 returnable from the dead as her previous type -- carries no numeric magnitude \
                 and needs a creature-type-conditioned resolution engine that does not exist \
                 here, so it stays deferred regardless of level."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.perfect_self_damage_reduction".to_owned(),
            value: MONK_PERFECT_SELF_DAMAGE_REDUCTION,
            detail: format!(
                "Monk Perfect Self granted at monk level {level} (PF1 Core Rulebook, 20th-level \
                 monk class feature): \"the monk gains damage reduction \
                 {MONK_PERFECT_SELF_DAMAGE_REDUCTION}/chaotic\" (corpus DR:10/Chaotic). This \
                 grounds only the flat DR magnitude; no damage-reduction-application or \
                 damage-resolution engine exists anywhere in this codebase to apply it to. Its \
                 other clause -- the monk is forevermore treated as an outsider rather than her \
                 previous creature type for spells and magical effects, while still returnable \
                 from the dead as her previous type -- carries no numeric magnitude and needs a \
                 creature-type-conditioned resolution engine that does not exist here, so it \
                 stays deferred regardless of level."
            ),
        });
    }

    // Recognized (SD13-E5): the level-1 bonus feat choice-slot selection is
    // recognized as chosen input when it names one of the PF1 Core Rulebook
    // restricted Monk bonus feat list's five feats (Combat Reflexes, Deflect
    // Arrows, Improved Grapple, Improved Trip, Stunning Fist), mirroring the
    // Sorcerer bloodline choice / Cleric domain choice / Druid nature-bond choice
    // recognition idiom. This is recognition of the choice-slot identity only; it
    // fabricates no feat-effect execution (no attack-of-opportunity engine for
    // Combat Reflexes, no ranged-deflection engine for Deflect Arrows, no
    // grapple-check engine for Improved Grapple, no trip-check engine for Improved
    // Trip, and no DC/save engine for Stunning Fist). A selection present but
    // outside this restricted list is acknowledged without naming a specific
    // restricted-list feat, mirroring the Sorcerer bloodline choice's
    // present-but-unrecognized branch, so no restricted-list feat identity is
    // fabricated for a selection this bounded seam does not know. This is always
    // the level-1 bonus feat (`MONK_BONUS_FEAT_GRANT_LEVEL`), carried forward
    // unchanged at level 2 — PF1 grants monks a SEPARATE bonus feat at 2nd level
    // that this bounded seam deliberately does not recognize.
    let bonus_feat_selection = choice_selection(input, MONK_BONUS_FEAT_CHOICE_ID);
    let recognized_bonus_feat_name = bonus_feat_selection.and_then(|selection| {
        if selection == CATCH_OFF_GUARD_FEAT_SELECTION {
            Some("Catch Off-Guard")
        } else if selection == COMBAT_REFLEXES_FEAT_SELECTION {
            Some("Combat Reflexes")
        } else if selection == DEFLECT_ARROWS_FEAT_SELECTION {
            Some("Deflect Arrows")
        } else if selection == "feat:dodge" {
            Some("Dodge")
        } else if selection == IMPROVED_GRAPPLE_FEAT_SELECTION {
            Some("Improved Grapple")
        } else if selection == SCORPION_STYLE_FEAT_SELECTION {
            Some("Scorpion Style")
        } else if selection == THROW_ANYTHING_FEAT_SELECTION {
            Some("Throw Anything")
        } else {
            None
        }
    });
    if let Some(selection) = bonus_feat_selection {
        // v0.6 alpha swarm, risks item 8 (Monk scoping follow-up, adversarial
        // review fix): this record's own claim about "{feat_name}'s own
        // mechanics are not grounded here" was stale for every feat this
        // seam separately closes below (Dodge, Catch Off-Guard, Throw
        // Anything, and now Combat Reflexes, Scorpion Style, Improved
        // Grapple) -- the blanket claim was already false for the first
        // three before this fix, just never caught. Made conditional so
        // this recognition record never asserts a specific feat's
        // mechanics are ungrounded when they genuinely are grounded
        // elsewhere (see each feat's own dedicated explanation record
        // below), while every other restricted-list feat (Deflect Arrows,
        // and any 6th/10th-level addition selected here) keeps the
        // original honest claim unchanged.
        let feat_own_mechanics_closed_elsewhere = |feat_name: &str| {
            matches!(
                feat_name,
                "Dodge"
                    | "Catch Off-Guard"
                    | "Throw Anything"
                    | "Combat Reflexes"
                    | "Scorpion Style"
                    | "Improved Grapple"
            )
        };
        let detail = if let Some(feat_name) = recognized_bonus_feat_name {
            let mechanics_clause = if feat_own_mechanics_closed_elsewhere(feat_name) {
                format!(
                    "{feat_name}'s own mechanics are grounded separately below (when genuinely \
                     applicable/active), not fabricated as part of this choice-recognition \
                     record itself"
                )
            } else {
                format!(
                    "{feat_name}'s own mechanics are not grounded here, and no \
                     attack-resolution, grapple-check, or DC/save engine exists in this \
                     codebase for it"
                )
            };
            format!(
                "Monk level {MONK_BONUS_FEAT_GRANT_LEVEL} bonus feat choice recognized: the \
                 canonical deterministic selection ({MONK_BONUS_FEAT_CHOICE_ID} -> {selection}) \
                 names {feat_name}, drawn from the PF1 Core Rulebook restricted Monk bonus feat \
                 list (Catch Off-Guard, Combat Reflexes, Deflect Arrows, Dodge, Improved \
                 Grapple, Scorpion Style, Throw Anything — verified identically on both \
                 primary sources; the 6th/10th-level list additions, including Improved \
                 Trip, stay unrecognized on this bounded seam), as chosen input on the \
                 compute seam. This is a recognition \
                 record of the choice slot only, so it carries no fabricated mechanical value \
                 (+0): {mechanics_clause}. Improved Unarmed Strike and Stunning Fist are not \
                 part of this restricted choice set \
                 because the PF1 Core Rulebook grants each to every monk automatically at level \
                 {MONK_BONUS_FEAT_GRANT_LEVEL} (\"the monk gains Stunning Fist as a bonus \
                 feat, even if he does not meet the prerequisites\"), separate from this \
                 chosen bonus feat. Improved Unarmed Strike's own mechanics stay ungrounded, \
                 but Stunning Fist's do NOT: its save DC and uses-per-day are grounded \
                 separately as feat.standalone.stunning_fist.save_dc / .uses_per_day, in this \
                 same receipt. Corrected 2026-07-27 (task #52) -- this sentence previously \
                 claimed the codebase grounds neither automatic grant, which contradicted two \
                 records sitting directly above it"
            )
        } else {
            format!(
                "Monk level {MONK_BONUS_FEAT_GRANT_LEVEL} bonus feat choice slot is present \
                 ({MONK_BONUS_FEAT_CHOICE_ID} -> {selection}), but only the PF1 Core Rulebook \
                 restricted Monk bonus feat list (Catch Off-Guard, Combat Reflexes, Deflect \
                 Arrows, Dodge, Improved Grapple, Scorpion Style, Throw Anything) is \
                 recognized on this bounded seam — a feat:stunning_fist selection in \
                 particular is not a list member because Stunning Fist is the automatic \
                 1st-level monk grant, never a choice; no \
                 restricted-list feat identity is grounded and no mechanical value is fabricated \
                 (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.bonus_feat_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    // SD13-E5: the SECOND bonus feat, the level-2 repeat grant, recognized
    // as its own numbered choice slot (the repeat-grant proving slice —
    // mirroring choice:ranger_favored_enemy_2 / favored_terrain_2 /
    // combat_style_bonus_feat_2). Same corrected seven-feat list, same
    // automatic-grant exclusions, same present-but-unrecognized branch.
    if level >= MONK_SECOND_BONUS_FEAT_GRANT_LEVEL
        && let Some(second_selection) =
            choice_selection(input, MONK_SECOND_BONUS_FEAT_CHOICE_ID)
    {
        let recognized_second_name = if second_selection == CATCH_OFF_GUARD_FEAT_SELECTION {
            Some("Catch Off-Guard")
        } else if second_selection == COMBAT_REFLEXES_FEAT_SELECTION {
            Some("Combat Reflexes")
        } else if second_selection == DEFLECT_ARROWS_FEAT_SELECTION {
            Some("Deflect Arrows")
        } else if second_selection == "feat:dodge" {
            Some("Dodge")
        } else if second_selection == IMPROVED_GRAPPLE_FEAT_SELECTION {
            Some("Improved Grapple")
        } else if second_selection == SCORPION_STYLE_FEAT_SELECTION {
            Some("Scorpion Style")
        } else if second_selection == THROW_ANYTHING_FEAT_SELECTION {
            Some("Throw Anything")
        } else {
            None
        };
        let detail = if let Some(feat_name) = recognized_second_name {
            format!(
                "Monk level {MONK_SECOND_BONUS_FEAT_GRANT_LEVEL} SECOND bonus feat choice \
                 recognized ({MONK_SECOND_BONUS_FEAT_CHOICE_ID} -> {second_selection}): the \
                 PF1 Core Rulebook grants a repeat bonus feat at 2nd level (\"At 1st level, \
                 2nd level, and every 4 levels thereafter, a monk may select a bonus \
                 feat\"), drawn from the same corrected restricted list as slot 1 (Catch \
                 Off-Guard, Combat Reflexes, Deflect Arrows, Dodge, Improved Grapple, \
                 Scorpion Style, Throw Anything — the 6th/10th-level list additions are not \
                 yet available at 2nd level). This character's second selection names \
                 {feat_name}. This is a recognition record of the numbered choice slot only \
                 (+0) — the repeat grant needs no list-growth mechanism, just its own slot \
                 and level gate, mirroring the ranger second-favored-enemy idiom; \
                 {feat_name}'s own mechanics are not grounded, and the 6th/10th-level repeat \
                 grants stay unrecognized"
            )
        } else if second_selection == "feat:stunning_fist" {
            format!(
                "Monk level {MONK_SECOND_BONUS_FEAT_GRANT_LEVEL} SECOND bonus feat choice \
                 slot is present ({MONK_SECOND_BONUS_FEAT_CHOICE_ID} -> {second_selection}), \
                 but Stunning Fist is the automatic 1st-level monk grant (\"even if he does \
                 not meet the prerequisites\"), never a restricted-list choice member; no \
                 feat identity is grounded and no mechanical value is fabricated (+0)"
            )
        } else {
            format!(
                "Monk level {MONK_SECOND_BONUS_FEAT_GRANT_LEVEL} SECOND bonus feat choice \
                 slot is present ({MONK_SECOND_BONUS_FEAT_CHOICE_ID} -> {second_selection}), \
                 but only the corrected PF1 Core Rulebook 1st/2nd-level restricted list \
                 (Catch Off-Guard, Combat Reflexes, Deflect Arrows, Dodge, Improved Grapple, \
                 Scorpion Style, Throw Anything) is recognized on this bounded seam; no \
                 restricted-list feat identity is grounded and no mechanical value is \
                 fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.bonus_feat_2_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    // SD13-E5: the THIRD and FOURTH bonus feats (gates 6/10), the remaining
    // numbered slots of the monk's level-10 bonus-feat family, each drawing
    // from its own WIDENED list per the verified additions.
    let later_bonus_feat_slots: [(u8, u8, &str, bool); 2] = [
        (
            3,
            MONK_THIRD_BONUS_FEAT_GRANT_LEVEL,
            MONK_THIRD_BONUS_FEAT_CHOICE_ID,
            false,
        ),
        (
            4,
            MONK_FOURTH_BONUS_FEAT_GRANT_LEVEL,
            MONK_FOURTH_BONUS_FEAT_CHOICE_ID,
            true,
        ),
    ];
    for (slot_number, grant_level, choice_id, includes_tenth) in later_bonus_feat_slots {
        if level < grant_level {
            continue;
        }
        let Some(selection) = choice_selection(input, choice_id) else {
            continue;
        };
        let recognized = MONK_BONUS_FEAT_BASE_LIST
            .iter()
            .chain(MONK_BONUS_FEAT_SIXTH_LEVEL_ADDITIONS.iter())
            .chain(
                MONK_BONUS_FEAT_TENTH_LEVEL_ADDITIONS
                    .iter()
                    .take(if includes_tenth { 4 } else { 0 }),
            )
            .find(|(sel, _)| *sel == selection)
            .map(|(_, name)| *name);
        let detail = if let Some(feat_name) = recognized {
            format!(
                "Monk bonus feat slot {slot_number} recognized ({choice_id} -> {selection}) \
                 at the level-{grant_level} grant (\"At 1st level, 2nd level, and every 4 \
                 levels thereafter, a monk may select a bonus feat\"). This character's \
                 selection names {feat_name}, drawn from the slot's widened PF1 Core \
                 Rulebook list: the base seven feats plus the verified 6th-level additions \
                 (Gorgon's Fist, Improved Bull Rush, Improved Disarm, Improved Feint, \
                 Improved Trip, Mobility){}. This is a +0 recognition record of the numbered \
                 choice slot only; {feat_name}'s own mechanics are not grounded, and \
                 Improved Unarmed Strike / Stunning Fist remain automatic grants, never \
                 choice-set members",
                if includes_tenth {
                    " plus the verified 10th-level additions (Improved Critical, Medusa's \
                     Wrath, Snatch Arrows, Spring Attack)"
                } else {
                    ""
                }
            )
        } else {
            format!(
                "Monk bonus feat slot {slot_number} choice is present ({choice_id} -> \
                 {selection}), but only the slot's widened PF1 Core Rulebook restricted \
                 list is recognized on this bounded seam; no restricted-list feat identity \
                 is grounded and no mechanical value is fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.monk.bonus_feat_{slot_number}_choice"),
            value: 0,
            detail,
        });
    }

    // v0.6 alpha swarm, risks item 8 (Monk scoping follow-up, 2026-07-25):
    // Dodge is a genuine exception to "no feat-effect engine exists" -- its
    // entire mechanical effect (a flat +1 dodge bonus to Armor Class,
    // `DODGE_AC_BONUS`) is ALREADY computed unconditionally in
    // `compute_combat_baseline` whenever `chosen.selected_feats` contains
    // `DODGE_FEAT_ID`, regardless of which choice slot granted it (mirrors
    // exactly how the Human bonus-feat slot's own Dodge grant already
    // works). A Monk whose level-1 bonus feat choice recognizably names
    // Dodge AND who also carries `feat:dodge` in `selected_feats` (so the
    // bonus is genuinely, not just nominally, active) is not blocked on
    // this feature: the claimed benefit is already real, not fabricated.
    // A Monk who chose Dodge via the slot but does NOT carry it in
    // `selected_feats` (an inconsistent/incomplete input) still blocks --
    // this is not a silent pass, it is a genuine unmet precondition.
    let dodge_bonus_feat_is_genuinely_active = recognized_bonus_feat_name == Some("Dodge")
        && feat_identity::holds(&input.chosen.selected_feats, DODGE_FEAT_ID);

    if dodge_bonus_feat_is_genuinely_active {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.bounded_progression.bonus_feat.dodge_active".to_owned(),
            value: DODGE_AC_BONUS,
            detail: format!(
                "Monk level {level} level-1 bonus feat is Dodge, and it is genuinely active: \
                 the same flat +{DODGE_AC_BONUS} dodge bonus to Armor Class \
                 `compute_combat_baseline` already applies unconditionally for any character \
                 carrying feat:dodge in selected_feats (regardless of which choice slot granted \
                 it) is already being applied here, not merely claimed as chosen input. Unlike \
                 the four remaining restricted-list feats with zero execution engine (Combat \
                 Reflexes, Deflect Arrows, Improved Grapple, Scorpion Style), Dodge needed no new \
                 feat-effect engine to close this burden -- its effect was already real elsewhere \
                 in this codebase"
            ),
        });
        return;
    }

    // v0.6 alpha swarm, risks item 8 (Monk scoping follow-up, second pass):
    // Catch Off-Guard and Throw Anything are a SECOND genuine exception,
    // verified independently against two primary sources (d20pfsrd and the
    // Archives of Nethys aonprd.com mirror, byte-for-byte agreement).
    // Catch Off-Guard: "You do not suffer any penalties for using an
    // improvised melee weapon. Unarmed opponents are flat-footed against
    // any attacks you make with an improvised melee weapon" (Normal: "You
    // take a -4 penalty on attack rolls made with an improvised weapon").
    // Throw Anything: "You do not suffer any penalties for using an
    // improvised ranged weapon. You receive a +1 circumstance bonus on
    // attack rolls made with thrown splash weapons" (same Normal penalty).
    // Both feats' entire benefit applies ONLY to improvised or splash
    // weapons -- and this codebase's bounded combat-baseline computation
    // (`compute_combat_baseline`) always computes a single deterministic
    // Longsword attack, a real (non-improvised, non-splash) weapon, with
    // no improvised-weapon penalty and no splash-weapon bonus modeled
    // anywhere in this codebase (confirmed by direct search: zero
    // "improvised"/"splash" references outside this comment). So under
    // every input this bounded slice can compute, both feats' triggering
    // condition never arises -- their benefit is genuinely, provably zero
    // here, not merely unclaimed, mirroring exactly how the Barbarian
    // illiteracy burden was retired as vacuous rather than left
    // claim-blocked. Unlike Dodge, no `selected_feats` cross-check is
    // needed: this is true for every input regardless of feat selection.
    if recognized_bonus_feat_name == Some("Catch Off-Guard")
        || recognized_bonus_feat_name == Some("Throw Anything")
    {
        let feat_name = recognized_bonus_feat_name.expect("checked above");
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.bounded_progression.bonus_feat.improvised_weapon_penalty_absent"
                .to_owned(),
            value: 0,
            detail: format!(
                "Monk level {level} level-1 bonus feat is {feat_name}: its entire benefit \
                 (removing the -4 improvised/splash-weapon attack penalty{}) applies only to \
                 improvised or splash weapons, and this codebase's bounded combat-baseline \
                 computation always computes a single deterministic Longsword attack -- a real, \
                 proficient weapon, never improvised or splash. No improvised-weapon penalty and \
                 no splash-weapon bonus is modeled anywhere in this codebase, so {feat_name}'s \
                 triggering condition never arises here: its benefit is genuinely zero under \
                 every input this bounded slice can compute, not merely unclaimed. This record \
                 documents that correction only; it carries no mechanical value (+0)",
                if feat_name == "Throw Anything" {
                    ", plus a +1 circumstance bonus on thrown splash weapon attacks"
                } else {
                    ""
                }
            ),
        });
        return;
    }

    // v0.6 alpha swarm, risks item 8 (Monk remaining-feats closure,
    // adversarially reviewed 2026-07-25): Combat Reflexes, Scorpion Style,
    // and Improved Grapple are a THIRD genuine exception -- each grants a
    // flat, standalone number derived purely from the Monk's own stats
    // (extra attack-of-opportunity capacity, a save DC and duration, a
    // flat CMB/CMD magnitude), not an opponent-dependent resolution.
    // Mirrors the Sneak-Attack-die-count / Dwarf-Stability-CMD-magnitude
    // idiom exactly: the NUMBER is real; the EVENT it would feed into (an
    // opponent actually triggering an attack of opportunity, an opponent
    // failing the Fortitude save, an opposed grapple check) is not
    // resolved, since no such engine exists anywhere in this codebase.
    // Deflect Arrows is deliberately NOT included here -- unlike these
    // three, it has no standalone numeric value at all (its entire
    // benefit IS the resolution: an opponent's ranged attack occurring
    // and being negated), so it stays exactly as claim-blocked as before,
    // confirmed correctly categorized by the adversarial review.
    if recognized_bonus_feat_name == Some("Combat Reflexes") {
        // PF1 Core Rulebook Combat Reflexes (cr_feats.lst:34): "You may
        // make %1 additional attacks of opportunity per round... " with
        // %1 substituting the corpus's own BONUS:VAR|CombatReflexesAttacks|DEX
        // formula token -- not a literal quoted "equal to your Dexterity
        // bonus" phrase (that description is this codebase's own gloss on
        // the formula, named honestly as such rather than presented as a
        // direct quote). A negative Dexterity modifier grants no
        // additional attacks (a feat never subtracts from the base
        // capacity), so this floors at 0.
        let additional_attacks_of_opportunity = monk_combat_reflexes_additional_attacks_of_opportunity(
            ability_modifiers.dexterity,
        );
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.bounded_progression.bonus_feat.combat_reflexes_capacity"
                .to_owned(),
            value: additional_attacks_of_opportunity,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|CombatReflexesAttacks|DEX
                "Monk level {level} level-1 bonus feat is Combat Reflexes: the corpus formula \
                 for its extra attacks of opportunity resolves to \
                 max(Dexterity modifier, 0) = {additional_attacks_of_opportunity} additional \
                 attacks of opportunity available per round (this codebase's own gloss on the \
                 formula, not a literal quote of the feat's BENEFIT text, which only says \
                 \"You may make %1 additional attacks of opportunity per round\" with %1 \
                 substituting that formula). This grounds only the CAPACITY as a flat number \
                 derived from the Monk's own Dexterity modifier -- it does not claim any \
                 attack of opportunity is ever actually triggered, since no \
                 attack-of-opportunity trigger engine (tracking an opponent's own movement or \
                 actions) exists anywhere in this codebase. With this feat, attacks of \
                 opportunity may also be made while flat-footed, per the same BENEFIT text; \
                 flat-footed state is not modeled here either"
            ),
        });
        return;
    }
    if recognized_bonus_feat_name == Some("Scorpion Style") {
        // PF1 Core Rulebook Scorpion Style (cr_feats.lst:142): "the
        // target's base land speed is reduced to 5 feet for a number of
        // rounds equal to your Wisdom modifier unless it makes a
        // Fortitude saving throw (DC %1)" with corpus formula
        // "10+(TL/2)+WIS" -- TL is total/character level, not
        // specifically "monk level"; this bounded seam is single-class
        // Human Monk only, so TL and the Monk's own level are identical
        // here, named precisely as such rather than silently conflated
        // for a hypothetical multiclass mix this seam doesn't admit.
        let wisdom_modifier = ability_modifiers.wisdom;
        let save_dc = monk_scorpion_style_dc(level, wisdom_modifier);
        let duration_rounds = wisdom_modifier.max(0);
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.bounded_progression.bonus_feat.scorpion_style_dc".to_owned(),
            value: save_dc,
            detail: format!(
                "Monk level {level} level-1 bonus feat is Scorpion Style: the corpus formula \
                 \"10+(TL/2)+WIS\" (DC %1 in the feat's own BENEFIT text) resolves to \
                 10 + (total level / 2) + Wisdom modifier = 10 + ({level}/2) + {wisdom_modifier} \
                 = {save_dc}, using this bounded single-class seam's own Monk level as total \
                 level (no multiclass mix is admitted here). This grounds only the Fortitude \
                 save DC a target would need to beat, not the target's own saving throw -- no \
                 opponent Fortitude-save resolution engine exists anywhere in this codebase, \
                 and the unarmed strike attack roll needed to trigger this effect at all is \
                 not resolved either"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.bounded_progression.bonus_feat.scorpion_style_duration"
                .to_owned(),
            value: duration_rounds,
            detail: format!(
                "Monk level {level} Scorpion Style speed-reduction duration: \"a number of \
                 rounds equal to your Wisdom modifier\" = max({wisdom_modifier}, 0) = \
                 {duration_rounds} rounds (a Wisdom penalty grants no negative duration). This \
                 grounds only the duration NUMBER; whether the effect is ever actually applied \
                 to a target depends on the unresolved attack roll and save above"
            ),
        });
        return;
    }
    if recognized_bonus_feat_name == Some("Improved Grapple") {
        // PF1 Core Rulebook Improved Grapple (cr_feats.lst:98): "You do
        // not provoke an attack of opportunity when performing a grapple
        // combat maneuver. In addition, you receive a +2 bonus on checks
        // made to grapple a foe. You also receive a +2 bonus to your
        // Combat Maneuver Defense whenever an opponent tries to grapple
        // you" (corpus: BONUS:VAR|CMB_Grapple,CMD_Grapple|2). Mirrors the
        // already-grounded Dwarf Stability idiom exactly (a flat
        // CMB/CMD-bonus-magnitude recognition record, not a Combat-
        // Maneuver-Bonus/Defense-total engine, which does not exist
        // anywhere in this codebase) -- per the adversarial review, this
        // needs no new CMB/CMD baseline pillar, since the bonus magnitude
        // alone is the same self-contained shape Dwarf Stability already
        // proved out.
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.bounded_progression.bonus_feat.improved_grapple_bonus"
                .to_owned(),
            value: MONK_IMPROVED_GRAPPLE_BONUS,
            detail: format!(
                "Monk level {level} level-1 bonus feat is Improved Grapple: a flat \
                 {MONK_IMPROVED_GRAPPLE_BONUS:+} bonus on checks made to grapple a foe, and a \
                 flat {MONK_IMPROVED_GRAPPLE_BONUS:+} bonus to Combat Maneuver Defense whenever \
                 an opponent tries to grapple you (not a general CMD bonus -- grapple-specific \
                 only, per the feat's own BENEFIT text), mirroring the already-grounded Dwarf \
                 Stability flat-CMD-bonus-magnitude idiom exactly. No Combat-Maneuver-Bonus- or \
                 Combat-Maneuver-Defense-total engine exists anywhere in this codebase, so no \
                 grapple check or opposed-check resolution is fabricated from this record -- \
                 only the flat bonus magnitude is grounded. This feat also removes the attack \
                 of opportunity normally provoked when performing a grapple combat maneuver; \
                 attack-of-opportunity provocation is not tracked anywhere in this codebase, so \
                 this is a vacuous-correction note (+0 contribution to this record), not a \
                 fabricated mechanic"
            ),
        });
        return;
    }

    // Deflect Arrows has NO numeric magnitude anywhere in the corpus: its
    // record carries no `BONUS:` token at all, and its entire benefit is a
    // resolution the player invokes at the table ("once per round, when you
    // would normally be hit with an attack from a ranged weapon, you may
    // deflect it"). There is no number for this engine to compute -- not
    // "not yet", but ever. Treating that as an unbuilt engine describes a
    // gap that can never close.
    //
    // What the player actually needs is the rulebook text, and the Feats tab
    // renders it. So this is complete once the description genuinely reaches
    // them -- and `feat_description_completion` checks that against live
    // data rather than assuming it, precisely because "complete" must never
    // be claimed for something the player cannot see (see that module's own
    // doc comment, and `docs/governance/no-stub-mvp-doctrine.md`). When it
    // does NOT reach them, this falls through to the diagnostic below
    // unchanged, exactly as the Dodge unmet-precondition branch does.
    if recognized_bonus_feat_name == Some("Deflect Arrows")
        && let ZeroMagnitudeResolution::TextComplete {
            description,
            surface,
        } = feat_description_completion(input, "Deflect Arrows")
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.bounded_progression.bonus_feat.text_complete".to_owned(),
            value: 0,
            detail: format!(
                "Monk level {level} level-1 bonus feat is Deflect Arrows, and it is complete \
                 with no computed magnitude (+0, non-fabricated). The corpus record carries no \
                 BONUS token of any kind -- there is no number to ground, now or ever, because \
                 the feat's entire benefit IS a resolution the player invokes at the table \
                 (once per round, negating one ranged attack that would have hit). What the \
                 player needs is the rule text, and they have it: this character carries the \
                 feat on `selected_feats`, which the character sheet's {surface:?} renders \
                 together with the feat's real corpus description -- \"{description}\". This \
                 record claims no attack-resolution or ranged-deflection engine exists (none \
                 does); it claims only that a feature with no magnitude is served by showing \
                 its description, which is verified here against the live feat catalog and \
                 this character's own recorded feats rather than assumed"
            ),
        });
        return;
    }

    // Still blocked (the one remaining named burden): the level-1 bonus feat's own
    // mechanics. The choice-slot identity is recognized above (when present and
    // in-list); this diagnostic narrows to naming only what remains
    // unimplemented, and it names the specific recognized feat only when this
    // seam actually recognized one, mirroring the Druid animal-companion
    // blocker's conditional message — so it never asserts a specific feat's
    // mechanics as "remaining" for a character whose chosen feat this seam did
    // not recognize.
    let bonus_feat_message = if let Some(feat_name) = recognized_bonus_feat_name {
        if feat_name == "Dodge" {
            format!(
                "Monk level {level} remains blocked on its level-1 bonus feat: the recognized \
                 choice (Dodge) is acknowledged as chosen input, but Dodge's own +{DODGE_AC_BONUS} \
                 dodge bonus to Armor Class is not yet genuinely active -- feat:dodge is not \
                 present in selected_feats for this input, so the claimed benefit is not yet \
                 real (this is a genuine unmet precondition, not a missing engine: Dodge's \
                 mechanics are already computed elsewhere in this codebase whenever selected_feats \
                 carries feat:dodge)"
            )
        } else {
            format!(
                "Monk level {level} remains blocked on its level-1 bonus feat: the recognized \
                 choice ({feat_name}) is acknowledged as chosen input only. Of the seven feats \
                 the corpus makes available at `MonkBonusFeatLVL,1` (Catch Off-Guard, Combat \
                 Reflexes, Deflect Arrows, Dodge, Improved Grapple, Scorpion Style, Throw \
                 Anything), six have grounded mechanics and return before this diagnostic is \
                 reached; Deflect Arrows is the only one that does not. Deflect Arrows has no \
                 numeric magnitude to compute at all, so it is NOT blocked on any unbuilt \
                 engine -- it is complete as soon as its real description reaches the player, \
                 and it does not for this input: {DEFLECT_ARROWS_FEAT_SELECTION} is absent from \
                 `selected_feats`, the only field the character sheet's Feats tab renders from, \
                 so this character's sheet shows the player nothing for it. This is a genuine \
                 unmet precondition on the input, not a missing engine, and it closes by \
                 recording the feat the player actually picked (every real in-app pick does) -- \
                 exactly the shape of the Dodge branch above. Corrected 2026-07-28: this \
                 message previously blamed a missing incoming-attack/opponent-interaction \
                 engine, which named a gap that could never close, since a feat with no \
                 magnitude has nothing for such an engine to compute"
            )
        }
    } else {
        format!(
            "Monk level {level} remains blocked on its bonus feat grant: no choice from the \
             restricted Monk feat list is recognized as chosen input, so nothing is resolved. \
             Six of the seven feats available at `MonkBonusFeatLVL,1` (Catch Off-Guard, Combat \
             Reflexes, Dodge, Improved Grapple, Scorpion Style, Throw Anything) DO have \
             grounded mechanics once a choice is recorded -- this message previously claimed \
             \"no feat-selection or feat-prerequisite engine exists here\", which stopped being \
             true once those landed (task #76). What is genuinely unbuilt: Deflect Arrows \
             (blocked on the incoming-attack engine, task #15), and the higher-tier options the \
             corpus gates at `MonkBonusFeatLVL,6` (Gorgon's Fist, Improved Bull Rush, Improved \
             Disarm, Improved Feint, Improved Trip, Mobility) and `,10` (Improved Critical, \
             Medusa's Wrath, Snatch Arrows, Spring Attack), none of which are grounded. Only \
             ONE bonus-feat slot is modelled here regardless of level, while the corpus pool is \
             `1+max((MonkBonusFeatLVL+2)/4,0)` -- six slots at Monk level 20, now reachable \
             since the level cap widened (task #49)"
        )
    };
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned(),
        message: bonus_feat_message,
        claim_blocking: true,
    });
}

#[cfg(test)]
mod monk_task36_feature_tests {
    use super::{
        build_pilot_headless_receipt, monk_ac_bonus_dodge_progression,
        monk_fast_movement_bonus_feet, monk_high_jump_acrobatics_bonus,
        monk_maneuver_training_cmb_bonus, monk_wholeness_of_body_healing, CharacterClassLevel,
        CharacterInput, MONK_CLASS_ID,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn monk(level: u8) -> CharacterInput {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: MONK_CLASS_ID.to_owned(), level }];
        input
    }

    fn value(level: u8, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(&monk(level))
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    /// Scales, unlike Barbarian's flat +10 -- the whole reason the
    /// Barbarian precedent is a shape precedent only.
    #[test]
    fn fast_movement_scales_every_three_levels() {
        assert_eq!(monk_fast_movement_bonus_feet(3), 10);
        assert_eq!(monk_fast_movement_bonus_feet(5), 10);
        assert_eq!(monk_fast_movement_bonus_feet(6), 20);
        assert_eq!(monk_fast_movement_bonus_feet(9), 30);
        assert_eq!(monk_fast_movement_bonus_feet(12), 40);
    }

    /// The corpus token is the DELTA (MonkLVL - MonkLVL*3/4), not the
    /// resulting CMB. Integer division floors.
    #[test]
    fn maneuver_training_is_the_delta_over_three_quarter_bab() {
        assert_eq!(monk_maneuver_training_cmb_bonus(3), 1);
        assert_eq!(monk_maneuver_training_cmb_bonus(4), 1);
        assert_eq!(monk_maneuver_training_cmb_bonus(5), 2);
        assert_eq!(monk_maneuver_training_cmb_bonus(8), 2);
        assert_eq!(monk_maneuver_training_cmb_bonus(9), 3);
        assert_eq!(monk_maneuver_training_cmb_bonus(12), 3);
    }

    #[test]
    fn high_jump_and_wholeness_are_flat_monk_level() {
        assert_eq!(monk_high_jump_acrobatics_bonus(5), 5);
        assert_eq!(monk_high_jump_acrobatics_bonus(12), 12);
        assert_eq!(monk_wholeness_of_body_healing(7), 7);
        assert_eq!(monk_wholeness_of_body_healing(12), 12);
    }

    /// AC Bonus's level-4+ dodge-bonus progression: `+0` below level 4,
    /// stepping `+1` every four levels starting at 4, reaching `+5` at 20 —
    /// the real PF1 Monk class table, confirmed against the pinned PCGen
    /// oracle's real export for a level-20 Monk
    /// (`core_rulebook:class_feature:monk_ac_bonus`,
    /// `AT-33-E5-remainder-charbuild_cycle_receipt.md`: oracle `7` at WIS-
    /// mod `2`, i.e. dodge component `5` at level 20).
    #[test]
    fn ac_bonus_dodge_progression_steps_every_four_levels_from_four() {
        assert_eq!(monk_ac_bonus_dodge_progression(1), 0);
        assert_eq!(monk_ac_bonus_dodge_progression(3), 0);
        assert_eq!(monk_ac_bonus_dodge_progression(4), 1);
        assert_eq!(monk_ac_bonus_dodge_progression(7), 1);
        assert_eq!(monk_ac_bonus_dodge_progression(8), 2);
        assert_eq!(monk_ac_bonus_dodge_progression(11), 2);
        assert_eq!(monk_ac_bonus_dodge_progression(12), 3);
        assert_eq!(monk_ac_bonus_dodge_progression(15), 3);
        assert_eq!(monk_ac_bonus_dodge_progression(16), 4);
        assert_eq!(monk_ac_bonus_dodge_progression(19), 4);
        assert_eq!(monk_ac_bonus_dodge_progression(20), 5);
    }

    /// The dispatched `class_chassis.monk.ac_bonus` value carries the
    /// dodge progression on top of whatever this fixture's own Wisdom
    /// modifier contributes -- isolating the dodge delta so this test does
    /// not depend on the fixture's specific ability scores.
    #[test]
    fn dispatched_ac_bonus_carries_the_dodge_progression_on_top_of_wisdom() {
        let wisdom_only = value(1, "class_chassis.monk.ac_bonus").expect("grounded at level 1");
        assert_eq!(
            value(4, "class_chassis.monk.ac_bonus"),
            Some(wisdom_only + 1)
        );
        assert_eq!(
            value(8, "class_chassis.monk.ac_bonus"),
            Some(wisdom_only + 2)
        );
        assert_eq!(
            value(20, "class_chassis.monk.ac_bonus"),
            Some(wisdom_only + 5)
        );
    }

    /// Every gate read off the corpus's own grant lines: 3/3/5/7. Each
    /// feature must be absent one level below its gate and present at it,
    /// through the real dispatch -- not just the pure formula.
    #[test]
    fn each_feature_appears_exactly_at_its_corpus_level_gate() {
        for (id, gate) in [
            ("class_chassis.monk.fast_movement", 3u8),
            ("class_chassis.monk.maneuver_training_cmb_bonus", 3),
            ("class_chassis.monk.high_jump", 5),
            ("class_chassis.monk.wholeness_of_body", 7),
        ] {
            assert_eq!(
                value(gate - 1, id),
                Some(0),
                "{id} must ground a value-0 absence record at level {} -- \"ground the \
                 absence, don't omit it\", matching Purity of Body directly above it and \
                 Skald's/Barbarian's DR (task #46)",
                gate - 1
            );
            assert!(
                value(gate, id).unwrap_or(0) > 0,
                "{id} must carry a real magnitude at level {gate}"
            );
        }
    }

    /// The grounded values at each gate, through the real receipt.
    #[test]
    fn the_grounded_values_match_the_corpus_formulas_at_their_gates() {
        assert_eq!(value(3, "class_chassis.monk.fast_movement"), Some(10));
        assert_eq!(value(12, "class_chassis.monk.fast_movement"), Some(40));
        assert_eq!(value(3, "class_chassis.monk.maneuver_training_cmb_bonus"), Some(1));
        assert_eq!(value(12, "class_chassis.monk.maneuver_training_cmb_bonus"), Some(3));
        assert_eq!(value(5, "class_chassis.monk.high_jump"), Some(5));
        assert_eq!(value(7, "class_chassis.monk.wholeness_of_body"), Some(7));
    }

    /// High Jump's flat bonus is unconditional -- the ki cost belongs to a
    /// separate +20 clause. The explanation must say so, since the earlier
    /// deferral got this exact distinction wrong.
    #[test]
    fn high_jump_explanation_scopes_the_ki_cost_to_the_separate_boost() {
        let receipt = build_pilot_headless_receipt(&monk(5));
        let detail = &receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.monk.high_jump")
            .expect("High Jump must be grounded at level 5")
            .detail;
        assert!(detail.contains("UNCONDITIONAL"), "{detail}");
        assert!(detail.contains("+20"), "{detail}");
    }
}

/// Task #49: the three Monk formulas that were hard-capped below level 20,
/// extracted as pure functions so their upper bands are directly testable
/// BEFORE `MAX_SUPPORTED_MONK_LEVEL` widens (this module's own tests exercise
/// levels up to 20 directly against the pure functions; the gated dispatch
/// path in `explain_monk_level1_chassis` cannot reach past level 12 yet, so
/// this commit is provably inert there -- see `monk_task36_feature_tests`
/// and the `sd13_monk_level*`/`sd18_monk_level*` integration suites, which
/// keep pinning the exact level 1-12 values unchanged).
///
/// Every band below is derived from the PF1 Core Rulebook Monk class table
/// and Flurry of Blows / Slow Fall rule text (cross-checked independently
/// against d20pfsrd and the Archives of Nethys aonprd.com mirror this
/// session), not from memory alone.
#[cfg(test)]
mod monk_task49_level_range_formula_tests {
    use super::{
        monk_flurry_of_blows_attack_count, monk_slow_fall_reach_feet,
        monk_unarmed_strike_damage_die,
    };

    /// Medium-monk unarmed strike damage: 1d6/1d8/1d10/2d6/2d8/2d10 at
    /// levels 1-3/4-7/8-11/12-15/16-19/20 (`min(5, MonkLVL/4)` band index).
    /// The min(5, ...) is what proves the progression stops at 2d10 rather
    /// than continuing past level 20.
    #[test]
    fn unarmed_strike_damage_die_covers_all_six_bands_through_level_twenty() {
        for (level, want) in [
            (1u8, (6i16, 1i16, "1d6")),
            (3, (6, 1, "1d6")),
            (4, (8, 1, "1d8")),
            (7, (8, 1, "1d8")),
            (8, (10, 1, "1d10")),
            (11, (10, 1, "1d10")),
            (12, (6, 2, "2d6")),
            (15, (6, 2, "2d6")),
            // The two bands that did not exist before this task.
            (16, (8, 2, "2d8")),
            (19, (8, 2, "2d8")),
            (20, (10, 2, "2d10")),
        ] {
            assert_eq!(monk_unarmed_strike_damage_die(level), want, "monk level {level}");
        }
    }

    /// Flurry grants attacks as Two-Weapon Fighting (1st, 2 attacks),
    /// Improved TWF (8th, 3 attacks), then Greater TWF (15th, 4 attacks).
    /// Deliberately NOT the corpus's own `FlurryAttacks` BONUS:VAR token,
    /// which counts total attack-routine entries including BAB iteratives
    /// (reaching 7 by level 20) -- a different quantity from the
    /// flurry-granted attack count this record documents.
    #[test]
    fn flurry_attack_count_gains_its_fourth_attack_at_fifteenth_level() {
        for (level, want) in [(1u8, 2i16), (7, 2), (8, 3), (14, 3), (15, 4), (20, 4)] {
            assert_eq!(monk_flurry_of_blows_attack_count(level), want, "monk level {level}");
        }
    }

    /// Slow Fall's reach is `floor(MonkLVL/2)*10` ft at every gate from 4th
    /// through 18th (this single formula reproduces the previously
    /// hand-written 20/30/40/50/60 ladder exactly for levels 4-12). At 20th
    /// the feature's own rule text switches to "fall any distance without
    /// harm" rather than continuing to a finite 100 ft -- `None` carries
    /// that unlimited case honestly rather than fabricating a number.
    #[test]
    fn slow_fall_reach_follows_one_formula_and_is_unlimited_at_twenty() {
        for (level, want) in [
            (4u8, Some(20i16)),
            (5, Some(20)),
            (6, Some(30)),
            (8, Some(40)),
            (10, Some(50)),
            (12, Some(60)),
            // Every band below here was frozen at 60 before this task.
            (14, Some(70)),
            (16, Some(80)),
            (18, Some(90)),
            (19, Some(90)),
            (20, None),
        ] {
            assert_eq!(monk_slow_fall_reach_feet(level), want, "monk level {level}");
        }
    }
}

/// v0.6 alpha swarm, risks item 8 (Monk scoping follow-up): Dodge is a
/// genuine exception among Monk's 7 restricted-list bonus feats -- its
/// effect was already real elsewhere in this codebase (the unconditional
/// `DODGE_AC_BONUS` gate in `compute_combat_baseline`), so recognizing it
/// closes this specific burden with no new feat-effect engine. The other
/// six feats (Catch Off-Guard, Combat Reflexes, Deflect Arrows, Improved
/// Grapple, Scorpion Style, Throw Anything) are untouched and stay
/// unconditionally blocked -- Monk as a whole does not reach `Computed`
/// from this slice (base-attack/base-save/fast-movement pillars are also
/// still standalone, not integrated, since `table_class_id` was not
/// widened for Monk this slice).
#[cfg(test)]
mod monk_bonus_feat_dodge_closure_tests {
    use super::{build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, MONK_CLASS_ID};
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_monk_input(dodge_choice: bool, dodge_selected_feat: bool) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: MONK_CLASS_ID.to_owned(), level: 1 }];
        if dodge_choice {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: "choice:monk_bonus_feat".to_owned(),
                selection_id: "feat:dodge".to_owned(),
            });
        }
        if !dodge_selected_feat {
            input.chosen.selected_feats.retain(|feat| feat != "feat:dodge");
        }
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

    /// A Monk who chose Dodge as their level-1 bonus feat, and genuinely
    /// carries feat:dodge in selected_feats (so the AC bonus is really
    /// active, not just claimed), is not blocked on the bonus-feat burden.
    #[test]
    fn monk_with_dodge_bonus_feat_genuinely_active_does_not_trip_the_diagnostic() {
        let input = human_monk_input(true, true);
        let ids = claim_blocking_ids(&input);

        assert!(
            !ids.contains(
                &"class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned()
            ),
            "Dodge's effect is already real elsewhere in this codebase, so a genuinely active \
             Dodge bonus feat must not claim-block: {ids:?}"
        );

        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.monk.bounded_progression.bonus_feat.dodge_active"),
            "expected the real dodge_active recognition record: {:?}",
            receipt.computation.explanations
        );
    }

    /// A Monk who chose Dodge via the bonus-feat slot but does NOT
    /// genuinely carry feat:dodge in selected_feats (an inconsistent
    /// input) still blocks -- this is a real unmet precondition, not a
    /// missing engine, so it must not silently pass.
    #[test]
    fn monk_with_dodge_bonus_feat_choice_but_not_genuinely_active_still_blocks() {
        let input = human_monk_input(true, false);
        let ids = claim_blocking_ids(&input);

        assert!(
            ids.contains(
                &"class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned()
            ),
            "Dodge chosen via the slot but absent from selected_feats is a genuine unmet \
             precondition, not a silently-passing posture: {ids:?}"
        );
    }

    /// A Monk with no recognized bonus-feat choice at all still blocks
    /// (the original, unconditional behavior for every feat except Dodge,
    /// Catch Off-Guard, and Throw Anything).
    #[test]
    fn monk_with_no_bonus_feat_choice_still_blocks() {
        let input = human_monk_input(false, false);
        let ids = claim_blocking_ids(&input);

        assert!(
            ids.contains(
                &"class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned()
            ),
            "an unrecognized/absent bonus-feat choice must still block: {ids:?}"
        );
    }
}

/// v0.6 alpha swarm, risks item 8 (Monk scoping follow-up, second pass):
/// Catch Off-Guard and Throw Anything's entire benefit applies only to
/// improvised/splash weapons, which this codebase's bounded
/// combat-baseline computation never models (always a single
/// deterministic Longsword, a real proficient weapon) -- so recognizing
/// either as the Monk's bonus feat closes the burden with no new
/// feat-effect engine, the same "genuinely vacuous under this bounded
/// slice" shape as the retired Barbarian illiteracy burden. Unlike
/// Dodge, no `selected_feats` cross-check is needed: this is true for
/// every input regardless of feat selection.
#[cfg(test)]
mod monk_bonus_feat_improvised_weapon_closure_tests {
    use super::{build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, MONK_CLASS_ID};
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_monk_input_with_bonus_feat(feat_selection: &str) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: MONK_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:monk_bonus_feat".to_owned(),
            selection_id: feat_selection.to_owned(),
        });
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

    /// A Monk whose level-1 bonus feat is Catch Off-Guard is not blocked:
    /// its improvised-melee-weapon benefit never triggers in this
    /// codebase's always-Longsword bounded computation.
    #[test]
    fn monk_with_catch_off_guard_bonus_feat_does_not_trip_the_diagnostic() {
        let input = human_monk_input_with_bonus_feat("feat:catch_off_guard");
        let ids = claim_blocking_ids(&input);

        assert!(
            !ids.contains(
                &"class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned()
            ),
            "Catch Off-Guard's benefit never triggers under this bounded Longsword-only \
             computation, so it must not claim-block: {ids:?}"
        );

        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            receipt.computation.explanations.iter().any(|e| e.id
                == "class_feature.monk.bounded_progression.bonus_feat.improvised_weapon_penalty_absent"),
            "expected the real vacuous-under-this-scope recognition record: {:?}",
            receipt.computation.explanations
        );
    }

    /// A Monk whose level-1 bonus feat is Throw Anything is not blocked
    /// either, for the identical reason (improvised ranged weapons and
    /// splash weapons are never modeled).
    #[test]
    fn monk_with_throw_anything_bonus_feat_does_not_trip_the_diagnostic() {
        let input = human_monk_input_with_bonus_feat("feat:throw_anything");
        let ids = claim_blocking_ids(&input);

        assert!(
            !ids.contains(
                &"class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned()
            ),
            "Throw Anything's benefit never triggers under this bounded Longsword-only \
             computation, so it must not claim-block: {ids:?}"
        );
    }

    /// A Monk whose level-1 bonus feat is Deflect Arrows, chosen through
    /// the slot but NOT carried in `selected_feats`, still blocks.
    ///
    /// This is the integrity half of the zero-magnitude/description rule.
    /// Deflect Arrows has no number to compute -- but the Feats tab renders
    /// from `selected_feats` and nothing else, so for this input the player
    /// sees no description anywhere and the feature is genuinely unserved.
    /// Mirrors the Dodge unmet-precondition case directly above.
    #[test]
    fn monk_deflect_arrows_not_surfaced_to_the_player_still_blocks() {
        let input = human_monk_input_with_bonus_feat("feat:deflect_arrows");
        let ids = claim_blocking_ids(&input);

        assert!(
            ids.contains(
                &"class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned()
            ),
            "Deflect Arrows chosen via the slot but absent from selected_feats reaches the \
             player nowhere, so it must still block: {ids:?}"
        );
    }

    /// The same Monk, with `feat:deflect_arrows` genuinely recorded in
    /// `selected_feats` -- the shape every real in-app pick produces, since
    /// both `handleAddFeat` and the level-up bonus-feat pick route through
    /// `add_feat_selection` -- is NOT blocked.
    ///
    /// Deflect Arrows carries no numeric magnitude anywhere in the corpus:
    /// its entire benefit is a resolution the player invokes at the table.
    /// There is no number for this engine to compute, now or ever. What the
    /// player needs is the rulebook text, and the Feats tab renders it. So
    /// the feature is complete, and a claim-blocking diagnostic asserting an
    /// unbuilt engine would be false.
    #[test]
    fn monk_deflect_arrows_surfaced_to_the_player_is_text_complete() {
        let mut input = human_monk_input_with_bonus_feat("feat:deflect_arrows");
        input.chosen.selected_feats.push("feat:deflect_arrows".to_owned());
        let ids = claim_blocking_ids(&input);

        assert!(
            !ids.contains(
                &"class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned()
            ),
            "Deflect Arrows has no magnitude to compute and its real description reaches the \
             player on the Feats tab, so it must not claim-block: {ids:?}"
        );

        let receipt = build_pilot_headless_receipt(&input);
        let record = receipt
            .computation
            .explanations
            .iter()
            .find(|e| {
                e.id == "class_feature.monk.bounded_progression.bonus_feat.text_complete"
            })
            .expect("expected the text-complete recognition record");
        assert_eq!(record.value, 0, "a text-complete record fabricates no magnitude");
        // The record must quote the real surfaced text, not a restatement --
        // otherwise it could drift away from what the app actually shows.
        let catalog_text = crate::rules_core::rules_tables::crb::feats::feat_tables()
            .iter()
            .find(|e| e.key == "Deflect Arrows")
            .and_then(|e| e.description)
            .expect("the CRB catalog carries Deflect Arrows with a description");
        assert!(
            record.detail.contains(catalog_text),
            "the explanation must quote the exact text the player sees: {:?}",
            record.detail
        );
    }

    /// What actually stands between Monk and `Computed`, pinned exactly.
    ///
    /// **Updated 2026-07-29 — this test did exactly the job it was written
    /// to do.** It previously asserted Monk's blocker set was exactly the
    /// four chassis-integration diagnostics
    /// (`class_chassis.unsupported`, `combat.baseline_unsupported`,
    /// `defense.total_save.unsupported`,
    /// `skill.selected_modifier.unsupported`), deliberately as an EXACT
    /// set rather than a "still blocked" smoke check, so it would fail
    /// loudly the moment `table_class_id` learned `class:monk`. It did
    /// fail, and this is the promised update rather than a deletion.
    ///
    /// All four are now gone: `table_class_id` maps `class:monk`, so
    /// `is_supported_generic_single_class` -- and therefore
    /// `has_supported_class_chassis` -- accepts Monk, and
    /// `compute_generic_table_chassis` reads the corpus-backed
    /// `class_tables()` Monk row that was always there.
    ///
    /// Monk still does NOT reach `Computed` in this posture, and this test
    /// continues to refuse to pretend otherwise -- but for a genuinely
    /// different and much smaller reason. With a bonus feat both CHOSEN
    /// and recorded in `selected_feats`, the blocker set is empty and the
    /// receipt is `Computed`; that is what this test now pins. The
    /// separate `monk_and_summoner_chassis_recognition_tests` module pins
    /// the no-bonus-feat posture, where the lone remaining blocker is the
    /// bonus-feat grant itself.
    #[test]
    fn monk_remaining_blockers_are_chassis_integration_only() {
        let mut input = human_monk_input_with_bonus_feat("feat:deflect_arrows");
        input.chosen.selected_feats.push("feat:deflect_arrows".to_owned());

        let ids = claim_blocking_ids(&input);

        assert!(
            ids.is_empty(),
            "the four chassis-integration blockers must all be gone now that \
             table_class_id maps class:monk, and the bonus-feat burden is already \
             satisfied by this input: {ids:?}"
        );
        assert_eq!(
            build_pilot_headless_receipt(&input).status,
            super::HeadlessReceiptStatus::Computed,
            "a Monk whose bonus feat is both chosen and recorded now reaches Computed"
        );
    }
}

/// v0.6 alpha swarm, risks item 8 (Monk remaining-feats closure,
/// adversarially reviewed 2026-07-25): Combat Reflexes, Scorpion Style,
/// and Improved Grapple each grant a flat, standalone number derived
/// purely from the Monk's own stats -- mirrors the Dodge/Catch-Off-Guard/
/// Throw-Anything closure shape, but for a genuinely new reason (a real
/// number grounded alongside an honestly-unresolved opponent-dependent
/// trigger, not a vacuous-under-this-scope precondition).
#[cfg(test)]
mod monk_bonus_feat_remaining_three_closure_tests {
    use super::{build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, MONK_CLASS_ID};
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_monk_input_with_bonus_feat(feat_selection: &str) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: MONK_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:monk_bonus_feat".to_owned(),
            selection_id: feat_selection.to_owned(),
        });
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

    /// A Monk whose level-1 bonus feat is Combat Reflexes is not blocked,
    /// and the real extra-AoO capacity is grounded: Dexterity 14 (+2
    /// modifier, unaffected by the fixture's Human Strength bonus) ->
    /// max(2, 0) = 2.
    #[test]
    fn monk_with_combat_reflexes_bonus_feat_grounds_the_real_capacity() {
        let input = human_monk_input_with_bonus_feat("feat:combat_reflexes");
        let ids = claim_blocking_ids(&input);

        assert!(
            !ids.contains(
                &"class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned()
            ),
            "Combat Reflexes' extra-AoO capacity is a real, groundable number, so it must not \
             claim-block: {ids:?}"
        );

        let receipt = build_pilot_headless_receipt(&input);
        let capacity = receipt
            .computation
            .explanations
            .iter()
            .find(|e| {
                e.id == "class_feature.monk.bounded_progression.bonus_feat.combat_reflexes_capacity"
            })
            .expect("expected the real combat_reflexes_capacity record");
        assert_eq!(capacity.value, 2, "Dexterity +2 modifier -> 2 additional AoOs: {:?}", capacity);
    }

    /// A Monk whose level-1 bonus feat is Scorpion Style is not blocked,
    /// and the real save DC and duration are grounded: level 1, Wisdom 12
    /// (+1 modifier) -> DC = 10 + (1/2=0) + 1 = 11, duration = max(1, 0)
    /// = 1 round.
    #[test]
    fn monk_with_scorpion_style_bonus_feat_grounds_the_real_dc_and_duration() {
        let input = human_monk_input_with_bonus_feat("feat:scorpion_style");
        let ids = claim_blocking_ids(&input);

        assert!(
            !ids.contains(
                &"class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned()
            ),
            "Scorpion Style's save DC and duration are real, groundable numbers, so it must not \
             claim-block: {ids:?}"
        );

        let receipt = build_pilot_headless_receipt(&input);
        let dc = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.monk.bounded_progression.bonus_feat.scorpion_style_dc")
            .expect("expected the real scorpion_style_dc record");
        assert_eq!(dc.value, 11, "10 + (level 1 / 2 = 0) + Wisdom +1 = 11: {:?}", dc);

        let duration = receipt
            .computation
            .explanations
            .iter()
            .find(|e| {
                e.id == "class_feature.monk.bounded_progression.bonus_feat.scorpion_style_duration"
            })
            .expect("expected the real scorpion_style_duration record");
        assert_eq!(duration.value, 1, "Wisdom +1 modifier -> 1 round duration: {:?}", duration);
    }

    /// A Monk whose level-1 bonus feat is Improved Grapple is not
    /// blocked, and the real flat +2 CMB/CMD-grapple bonus magnitude is
    /// grounded, mirroring the Dwarf Stability idiom.
    #[test]
    fn monk_with_improved_grapple_bonus_feat_grounds_the_real_bonus_magnitude() {
        let input = human_monk_input_with_bonus_feat("feat:improved_grapple");
        let ids = claim_blocking_ids(&input);

        assert!(
            !ids.contains(
                &"class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned()
            ),
            "Improved Grapple's +2/+2 bonus magnitude is a real, groundable number, so it must \
             not claim-block: {ids:?}"
        );

        let receipt = build_pilot_headless_receipt(&input);
        let bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| {
                e.id == "class_feature.monk.bounded_progression.bonus_feat.improved_grapple_bonus"
            })
            .expect("expected the real improved_grapple_bonus record");
        assert_eq!(bonus.value, 2, "flat +2 CMB/CMD-grapple bonus magnitude: {:?}", bonus);
    }
}

