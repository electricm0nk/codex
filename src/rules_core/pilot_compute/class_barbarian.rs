#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm, risks item 8 (second APG/ACG closure): ACG
/// Bloodrager, a Barbarian+Sorcerer hybrid whose Bloodrage class feature
/// is, per the PCGen corpus's own DESC text, explicitly "the barbarian's
/// rage class feature for the purpose of feat prerequisites, feat
/// abilities, magic item abilities, and spell effects" -- a near-exact
/// mechanical clone of Barbarian's own Rage (same base four values, same
/// rounds-per-day formula once algebraically simplified, and the same
/// 11th/17th/20th tier/fatigue-immunity/mighty thresholds as
/// `BARBARIAN_GREATER_RAGE_LEVEL`/`BARBARIAN_TIRELESS_RAGE_LEVEL`/
/// `BARBARIAN_MIGHTY_RAGE_LEVEL`, reused directly rather than
/// re-declared). Unlike Skald, Bloodrage is self-only by RAW with no
/// exception-clause self-application inference needed.
pub(super) const BLOODRAGER_CLASS_ID: &str = "class:bloodrager";

/// The first level at which a Bloodrager can cast at all (task #1,
/// 2026-07-27). Verified directly against `acg_classes.lst`: the class
/// block carries NO `CAST:`/`KNOWN:` row whatsoever below this level,
/// and its caster-level token is independently gated
/// `BONUS:CASTERLEVEL|Bloodrager|Caster_Level_Bloodrager|PRECLASS:1,Bloodrager=4`.
/// A level 1-3 Bloodrager therefore has no spellcasting to be missing.
pub(super) const BLOODRAGER_FIRST_CASTING_LEVEL: u8 = 4;

/// `ClassAbilityActivation.ability_id` for Bloodrager Bloodrage.
pub(super) const BLOODRAGER_BLOODRAGE_ABILITY_ID: &str = "bloodrage";

/// PF1 Advanced Class Guide Bloodrage's Armor Class penalty: -2,
/// unconditionally the same at every tier -- identical to Barbarian's own
/// Rage penalty (`BARBARIAN_RAGE_ARMOR_CLASS_PENALTY`), verified
/// independently against the PCGen corpus DESC text rather than assumed
/// from the Rage-equivalence claim alone.
pub(super) const BLOODRAGER_BLOODRAGE_ARMOR_CLASS_PENALTY: i16 = -2;

/// PF1 Advanced Class Guide Bloodrage rounds-per-day base: the corpus
/// formula is `2 + Constitution modifier + 2*level`; algebraically
/// identical to Barbarian's own `4 + Constitution modifier + 2*(level-1)`
/// (both reduce to `2 + Constitution modifier + 2*level`), so this base
/// constant is 2 (not Barbarian's 4) to match the corpus's own additive
/// decomposition -- see `bloodrager_bloodrage_rounds_per_day`'s own doc
/// comment for the exact formula used.
pub(super) const BLOODRAGER_BLOODRAGE_BASE_ROUNDS_PER_DAY: i16 = 2;

/// Bloodrager's Bloodline chooser, and the ONE canonical bloodline this
/// codebase grounds out of the corpus's 10 (v0.6 alpha swarm, Bloodrager
/// spellcasting-shaped closure).
///
/// Bloodrager's bloodlines are PARALLEL to Sorcerer's, not shared with
/// them (task #59) -- each is its own `KEY:<X> Bloodrager Bloodline ~
/// ...` family in `acg_abilities_class.lst`, so none of Sorcerer's
/// shipped bloodline work transfers. Arcane is the canonical pick for
/// the same reason it already is for Sorcerer (`bloodline:arcane` in
/// `compose_character_input`), keeping one bloodline name across both
/// classes rather than inventing a second convention -- but the grounding
/// underneath is Bloodrager's own separate corpus records, not Sorcerer's.
pub(super) const BLOODRAGER_BLOODLINE_CHOICE_ID: &str = "choice:bloodrager_bloodline";

pub(super) const ARCANE_BLOODRAGER_BLOODLINE_SELECTION: &str = "bloodline:arcane";

/// The Arcane Bloodrager Bloodline's own power ladder gates, read
/// verbatim off each record's `PREVARGTEQ:Bloodrager_Arcane_
/// BloodlineProgressionLVL,<n>` token: Disruptive Bloodrage 1, Arcane
/// Bloodrage 4, Greater Arcane Bloodrage 8, Caster's Scourge 12, True
/// Arcane Bloodrage 16, Caster's Bane 20.
pub(super) const ARCANE_BLOODRAGER_DISRUPTIVE_BLOODRAGE_LEVEL: u8 = 1;

pub(super) const ARCANE_BLOODRAGER_ARCANE_BLOODRAGE_LEVEL: u8 = 4;

pub(super) const ARCANE_BLOODRAGER_GREATER_ARCANE_BLOODRAGE_LEVEL: u8 = 8;

pub(super) const ARCANE_BLOODRAGER_CASTERS_SCOURGE_LEVEL: u8 = 12;

pub(super) const ARCANE_BLOODRAGER_TRUE_ARCANE_BLOODRAGE_LEVEL: u8 = 16;

pub(super) const ARCANE_BLOODRAGER_CASTERS_BANE_LEVEL: u8 = 20;

/// Disruptive Bloodrage's own magnitude: "The DC to cast spells
/// defensively increases by 2 for enemies within your threatened area"
/// (`KEY:Arcane Bloodrager Bloodline ~ Disruptive Bloodrage` DESC).
pub(super) const ARCANE_BLOODRAGER_DISRUPTIVE_BLOODRAGE_DC_INCREASE: i16 = 2;

// SD13-E3/E5 martial chassis baseline identity. Barbarian is a non-spell pure
// martial class; the bounded single-class level-1 identity is recognized as
// direct runtime evidence, with base-attack / base-save progression, the
// fast-movement +10 ft. speed extension, and Rage's flat numeric surface
// grounded as standalone records. No rage-state execution engine, weapon
// familiarity, or level-2+ martial progression is grounded.
pub(super) const BARBARIAN_CLASS_ID: &str = "class:barbarian";

/// `ClassAbilityActivation.ability_id` for Barbarian Rage (v0.6 alpha swarm,
/// risks item 8) -- the flat compound-string idiom `character_input.rs`'s
/// `ClassAbilityActivation` doc comment specifies, not a per-class enum.
pub(super) const BARBARIAN_RAGE_ABILITY_ID: &str = "rage";

/// PF1 Core Rulebook Rage's Armor Class penalty: -2, unconditionally the
/// same at every tier (Rage/Greater Rage/Mighty Rage) -- Greater Rage's own
/// rule text confirms "the -2 penalty to AC remains".
pub(super) const BARBARIAN_RAGE_ARMOR_CLASS_PENALTY: i16 = -2;

/// PF1 Core Rulebook rage power slots, verified identically on both primary
/// sources: "Starting at 2nd level, a barbarian gains a rage power. She
/// gains another rage power for every two levels of barbarian attained
/// after 2nd level." — gates 2/4/6/8/10/12/14/16/18/20 within the tranche
/// ceiling (the SD18 level-18 widening added the ninth slot at gate 18;
/// the SD18 level-20 widening added the tenth and FINAL slot at gate 20,
/// the last rage-power grant within PF1's 1-20 character-level cap).
/// "Unless otherwise noted, a barbarian cannot select an individual power
/// more than once." Numbered
/// slots per the proven repeat-grant idiom; open-ended recognition (no
/// power-list validation — d20pfsrd merges non-CRB powers into its list,
/// the same superset pattern as the mercy tiers, and the open-ended idiom
/// sidesteps list encoding entirely).
pub(super) const BARBARIAN_RAGE_POWER_SLOTS: [(u8, u8, &str); 10] = [
    (1, 2, "choice:barbarian_rage_power"),
    (2, 4, "choice:barbarian_rage_power_2"),
    (3, 6, "choice:barbarian_rage_power_3"),
    (4, 8, "choice:barbarian_rage_power_4"),
    (5, 10, "choice:barbarian_rage_power_5"),
    (6, 12, "choice:barbarian_rage_power_6"),
    (7, 14, "choice:barbarian_rage_power_7"),
    (8, 16, "choice:barbarian_rage_power_8"),
    (9, 18, "choice:barbarian_rage_power_9"),
    (10, 20, "choice:barbarian_rage_power_10"),
];

/// SD31-E4-F2-004: Pathfinder Unchained's OWN Rage Power chooser slots, a
/// SEPARATE numbered-slot family from [`BARBARIAN_RAGE_POWER_SLOTS`] --
/// `decisions.md §10`'s AMENDMENT ("rogue and unchained rogue are two
/// completely different classes -- one does not replace the other") forbids
/// folding Unchained Barbarian's own chooser into the base class's, which is
/// exactly the shape wave 12 found and declined to credit
/// (`OPEN-ISSUES.md`/`progress.md` `SD31-E4-F2-003` §3: the probe's own
/// "free ride" through `owner="barbarian"` mapped every Unchained Rage Power
/// key to book `core_rulebook`, so the real `pathfinder_unchained` record
/// could never satisfy `classify()`'s book-attribution guard). Same 10-gate
/// cadence as the base class (`pu_abilities_class.lst:328-337`'s own
/// `PREVARGTEQ:RagePowersLVL,<2|4|6|8|10|12|14|16|18|20>` pool-reduction
/// gates, byte-identical to the base Barbarian's own grant levels), a
/// DISTINCT `choice:unchained_barbarian_rage_power[_N]` id family so a
/// selection recorded under one class can never satisfy the other's slot.
pub(super) const UNCHAINED_BARBARIAN_RAGE_POWER_SLOTS: [(u8, u8, &str); 10] = [
    (1, 2, "choice:unchained_barbarian_rage_power"),
    (2, 4, "choice:unchained_barbarian_rage_power_2"),
    (3, 6, "choice:unchained_barbarian_rage_power_3"),
    (4, 8, "choice:unchained_barbarian_rage_power_4"),
    (5, 10, "choice:unchained_barbarian_rage_power_5"),
    (6, 12, "choice:unchained_barbarian_rage_power_6"),
    (7, 14, "choice:unchained_barbarian_rage_power_7"),
    (8, 16, "choice:unchained_barbarian_rage_power_8"),
    (9, 18, "choice:unchained_barbarian_rage_power_9"),
    (10, 20, "choice:unchained_barbarian_rage_power_10"),
];

/// SD13-E5 Barbarian level-range gate, mirroring the Fighter
/// `supported_fighter_level` / Paladin `supported_paladin_level` / Rogue
/// `supported_rogue_level` idiom. Monk's own level-range gate is
/// `supported_monk_level` / `MAX_SUPPORTED_MONK_LEVEL`, unrelated to this
/// Barbarian gate.
///
/// A further SD13-E5 slice widens the gate to level 9 (verified independently
/// against d20pfsrd and legacy.aonprd.com): level 9 base attack bonus
/// genuinely rises to +9 (full BAB) while poor Reflex/Will both genuinely
/// rise to +3 (9 / 3) and good Fortitude stays +6 (9 / 2 + 2, an
/// integer-division coincidence); the rage rounds-per-day pool genuinely
/// rises to 23 (4 + Con mod + 2 per level after 1st) while the four flat
/// rage-surface magnitudes stay at their standard-rage values (the next
/// change is Greater Rage at 11th); the level-9 "Special" column reads
/// "Trap sense +3" — a tier-rise on the already-grounded Trap Sense formula
/// pillar (level / 3), not a new class feature; Damage Reduction stays 1/—
/// (the next DR rise lands at 10th); level 9 is NOT a rage-power level
/// (powers land at 2/4/6/8/10...), so no new pillar is grounded.
///
/// A further SD13-E5 slice widens the gate to level 10 — the tranche ceiling
/// (verified independently against d20pfsrd and legacy.aonprd.com): level 10
/// base attack genuinely rises to +10 (full BAB) and good Fortitude
/// genuinely rises to +7 (10 / 2 + 2), while poor Reflex/Will both stay +3
/// (10 / 3, coincidences); the rage rounds-per-day pool genuinely rises to
/// 25 (4 + Con mod + 2 per level after 1st) with the rage-surface magnitudes
/// staying standard (Greater Rage at 11th); the level-10 "Special" column
/// reads "Damage reduction 2/—, rage power": DR genuinely rises to 2
/// (BARBARIAN_DAMAGE_REDUCTION_TWO_LEVEL — rising by 1 at 10th and every
/// three levels thereafter) as a tier on the existing flat-magnitude pillar,
/// and the rage-power entry is the same open-ended choice-list feature left
/// named-but-unproven at 2/4/6/8; Trap Sense stays +3 (next rise 12th).
///
/// A still further SD18 slice widens the gate to level 11 (verified
/// independently against d20pfsrd and legacy.aonprd.com): base-attack
/// (classlevel = 11) genuinely rises to +11, and base-save stays Fortitude
/// +7 / Reflex +3 / Will +3 (11/2+2 and 11/3, both integer-division
/// coincidences unchanged from level 10); the rage rounds-per-day pool
/// genuinely rises to 27 (4 + Con mod + 2 per level after 1st); the
/// level-11 "Special" column reads "Greater rage" only — Greater Rage
/// GENUINELY RISES the flat while-raging Strength and Constitution morale
/// bonuses from +4 to +6 and the Will-save morale bonus from +2 to +3 (the
/// Armor Class penalty stays -2), a magnitude-rise on the already-grounded
/// rage-constant pillar mirroring exactly how Trap Sense's and Damage
/// Reduction's own flat magnitudes were widened at their rise levels; level
/// 11 is NOT a rage-power level (powers land at 2/4/6/8/10/12...), so no
/// new rage-power-selection-slot-count engine is invented; Trap Sense stays
/// +3 (next rise 12th) and Damage Reduction stays 2/— (next rise 13th).
///
/// A still further SD18 slice widens the gate to level 12 (verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror, byte-for-byte agreement): base-attack (classlevel = 12)
/// genuinely rises to +12, and base-save genuinely rises to Fortitude +8
/// (12/2+2) / Reflex +4 / Will +4 (12/3, both genuinely risen from +3); the
/// rage rounds-per-day pool genuinely rises to 29 (4 + Con mod + 2 per
/// level after 1st); the level-12 "Special" column reads "Rage power, trap
/// sense +4" — Trap Sense GENUINELY RISES to +4 (12/3), a magnitude-rise on
/// the already-grounded Trap Sense flat-magnitude formula pillar, mirroring
/// exactly how the level-6/level-9 Trap Sense rises and the level-10/
/// level-11 Damage Reduction/Greater Rage rises were widened; the
/// rage-power entry is the SAME open-ended choice-list feature already
/// deliberately left named-but-unproven-in-effect at levels 2/4/6/8/10 — a
/// sixth numbered slot (gate 12) is added to `BARBARIAN_RAGE_POWER_SLOTS`
/// mirroring the proven repeat-grant idiom exactly, so no new
/// rage-power-EFFECT engine is invented; the Greater Rage constants
/// (+6/+6/+3/-2) and Damage Reduction (2/—, next rise 13th) both stay
/// unchanged from level 11.
///
/// A still further SD18 slice widens the gate to level 13 (verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror, byte-for-byte agreement): base-attack (classlevel = 13) genuinely
/// rises to +13, while base-save stays Fortitude +8 (13/2+2), Reflex +4, and
/// Will +4 (13/3, both integer-division coincidences unchanged from level
/// 12); the rage rounds-per-day pool genuinely rises to 31 (4 + Con mod + 2
/// per level after 1st); the level-13 "Special" column reads "Damage
/// reduction 3/-" only — Damage Reduction GENUINELY RISES to 3/- via a THIRD
/// tier constant (`BARBARIAN_DAMAGE_REDUCTION_THREE_LEVEL`), mirroring
/// exactly how the level-7/level-10 two-tier idiom was established; Trap
/// Sense stays +4 (13/3, next rise 15th) and level 13 is NOT a rage-power
/// level (powers land at 2/4/6/8/10/12/14...), so no seventh
/// rage-power-selection-slot-count engine is invented.
///
/// A still further SD18 slice widens the gate to level 14 (verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror, byte-for-byte agreement): base-attack (classlevel = 14) genuinely
/// rises to +14, good Fortitude genuinely rises to +9 (14/2+2), while poor
/// Reflex/Will both stay +4 (14/3, integer-division coincidences unchanged
/// from level 13); the rage rounds-per-day pool genuinely rises to 33 (4 +
/// Con mod + 2 per level after 1st); the level-14 "Special" column reads
/// "Indomitable will, rage power" — level 14 IS a rage-power level (powers
/// land at 2/4/6/8/10/12/14...), so a seventh numbered slot (gate 14) is
/// added to `BARBARIAN_RAGE_POWER_SLOTS` mirroring the proven repeat-grant
/// idiom exactly, no new rage-power-EFFECT engine invented; Indomitable
/// Will is a genuinely NEW named class feature ("a barbarian gains a +4
/// morale bonus on Will saves to resist enchantment spells and effects
/// while raging"), grounded as a fifth flat while-raging magnitude
/// (`BARBARIAN_INDOMITABLE_WILL_ENCHANTMENT_WILL_SAVE_BONUS`), mirroring
/// exactly the shape of the four pre-existing flat rage constants (Strength/
/// Constitution/Will-save morale bonuses, AC penalty): a bounded
/// flat-magnitude record only, never applied to any actual Will-save total,
/// since no saving-throw-resolution engine, no spell-school-classification
/// engine, and no rage-state execution engine exists anywhere in this
/// codebase to decide when a save is against an enchantment effect or
/// whether the barbarian is currently raging; Trap Sense stays +4 (14/3,
/// next rise 15th) and Damage Reduction stays 3/- (next rise 16th).
///
/// A still further SD18 slice — the loop's FIRST §3.2 level-15 landing,
/// opening the level-15 sweep — widens the gate to level 15 (verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror, byte-for-byte agreement): base-attack (classlevel = 15) genuinely
/// rises to +15 (full BAB), and poor Reflex/Will both genuinely rise to +5
/// (15/3), while good Fortitude stays +9 (15/2+2, an integer-division
/// coincidence with level 14); the rage rounds-per-day pool genuinely rises
/// to 35 (4 + Con mod + 2 per level after 1st); the level-15 "Special"
/// column reads "Trap sense +5" only — Trap Sense GENUINELY RISES to +5
/// (15/3) via the SAME pre-existing flat-magnitude formula pillar used at
/// every prior tier (3rd/6th/9th/12th), so this slice needs no new tier
/// constant, no new record type, and no new choice slot at all: the
/// formula is already level-generic. Level 15 is NOT a rage-power level
/// (powers land at 2/4/6/8/10/12/14/16/18/20...), so no eighth
/// rage-power-selection-slot-count engine is invented; Damage Reduction
/// stays 3/- (next rise 16th) and Indomitable Will's flat +4 magnitude
/// carries over unchanged (already unconditional at level >= 14).
///
/// A still further SD18 slice — the loop's FIRST §3.2 level-16 landing,
/// opening the level-16 sweep — widens the gate to level 16 (verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror, byte-for-byte agreement): base-attack (classlevel = 16)
/// genuinely rises to +16 (full BAB), and good Fortitude genuinely rises to
/// +10 (16/2+2), while poor Reflex/Will both stay +5 (16/3, an
/// integer-division coincidence with level 15); the rage rounds-per-day
/// pool genuinely rises to 37 (4 + Con mod + 2 per level after 1st); the
/// level-16 "Special" column reads "Damage reduction 4/-, rage power" —
/// Damage Reduction GENUINELY RISES to 4/- via a FOURTH tier constant
/// (`BARBARIAN_DAMAGE_REDUCTION_FOUR_LEVEL`), mirroring exactly how the
/// level-10/level-13 two-tier-then-three-tier idiom was established (the
/// same "10th level and every three barbarian levels thereafter" cadence:
/// 10, 13, 16); level 16 IS a rage-power level (powers land at
/// 2/4/6/8/10/12/14/16/18/20), so an EIGHTH numbered slot (gate 16) is
/// added to `BARBARIAN_RAGE_POWER_SLOTS` mirroring the proven repeat-grant
/// idiom exactly, no new rage-power-EFFECT engine invented; Trap Sense
/// stays +5 (16/3, next rise 18th) and Indomitable Will's flat +4
/// magnitude carries over unchanged.
///
/// A still further SD18 slice — the loop's EIGHTH §3.2 level-17 landing,
/// after Ranger, Bard, Rogue, Fighter, Wizard, Cleric, and Paladin — widens
/// the gate to level 17 (verified independently against d20pfsrd and the
/// Archives of Nethys aonprd.com mirror, byte-for-byte agreement across the
/// full levels-15-through-19 block, so a third source was not required):
/// base-attack (classlevel = 17) genuinely rises to +17 (full BAB), and
/// good Fortitude stays +10 (17/2+2, an integer-division coincidence with
/// level 16), while poor Reflex/Will both stay +5 (17/3, also an
/// integer-division coincidence with level 16); the rage rounds-per-day
/// pool genuinely rises to 39 (4 + Con mod + 2 per level after 1st); the
/// level-17 "Special" column reads "Tireless rage" only — a genuinely NEW
/// class feature ("Starting at 17th level, a barbarian no longer becomes
/// fatigued at the end of her rage"), grounded here as a bounded
/// grant-only identity record (value 0, non-fabricated) via a new
/// `BARBARIAN_TIRELESS_RAGE_LEVEL` gate constant, mirroring the
/// Indomitable Will / Paladin Aura-of-Justice/Aura-of-Faith/
/// Aura-of-Righteousness idiom exactly: no rage-state execution engine
/// exists anywhere in this codebase (confirmed by direct inspection), so
/// there is no fatigue-application mechanism for Tireless Rage to interact
/// with, and none is fabricated. 17 is NOT a rage-power level (powers land
/// at 2/4/6/8/10/12/14/16/18/20), so no ninth numbered slot is added; Trap
/// Sense stays +5 (17/3, next rise 18th), Damage Reduction stays 4/- (next
/// rise 19th), and Indomitable Will's flat +4 magnitude carries over
/// unchanged.
///
/// A still further SD18 slice — the loop's FIFTH §3.2 level-18 landing,
/// after Wizard, Cleric, Paladin, and Fighter — widens the gate to level 18
/// (verified independently against d20pfsrd and the Archives of Nethys
/// aonprd.com mirror, byte-for-byte agreement across the full
/// levels-15-through-20 block, so a third source was not required):
/// base-attack (classlevel = 18) genuinely rises to +18 (full BAB), and
/// good Fortitude genuinely rises to +11 (18/2+2), while poor Reflex/Will
/// both genuinely rise to +6 (18/3); the rage rounds-per-day pool
/// genuinely rises to 41 (4 + Con mod + 2 per level after 1st); the
/// level-18 "Special" column reads "Rage power, trap sense +6": 18 IS a
/// rage-power level (powers land at 2/4/6/8/10/12/14/16/18/20), so a NINTH
/// numbered slot (`BARBARIAN_RAGE_POWER_SLOTS`, gate 18,
/// `choice:barbarian_rage_power_9`) is added mirroring the proven
/// repeat-grant idiom exactly, no rage-power-EFFECT engine invented; Trap
/// Sense genuinely rises to +6 (18/3, the same pre-existing formula, up
/// from +5 at level 17); Damage Reduction stays 4/- (next rise 19th);
/// Indomitable Will's flat +4 magnitude and Tireless Rage both carry over
/// unchanged. This needed ZERO new record types and ZERO formula changes
/// on base attack, base saves, rage rounds, or Trap Sense (all were
/// already level-generic formulas) — only a ninth numbered rage-power slot
/// appended to `BARBARIAN_RAGE_POWER_SLOTS`.
///
/// A still further SD18 slice — the loop's FIRST §3.2 level-19 landing,
/// opening the level-19 sweep — widens the gate to level 19 (verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror, byte-for-byte agreement across the full levels-15-through-20
/// block, so a third source was not required): base-attack (classlevel = 19)
/// genuinely rises to +19 (full BAB), while good Fortitude stays +11
/// (19/2+2, an integer-division coincidence with level 18) and poor
/// Reflex/Will both stay +6 (19/3, also an integer-division coincidence with
/// level 18); the rage rounds-per-day pool genuinely rises to 43 (4 + Con
/// mod + 2 per level after 1st); the level-19 "Special" column reads
/// "Damage reduction 5/-" only — Damage Reduction GENUINELY RISES to 5/-
/// via a FIFTH tier constant (`BARBARIAN_DAMAGE_REDUCTION_FIVE_LEVEL`),
/// mirroring exactly how the level-10/level-13/level-16 three-prior-tier
/// idiom was established (the same "10th level and every three barbarian
/// levels thereafter" cadence: 10, 13, 16, 19); level 19 is NOT a
/// rage-power level (powers land at 2/4/6/8/10/12/14/16/18/20), so no tenth
/// numbered slot is added; Trap Sense stays +6 (19/3, its next rise would
/// be 21st, outside the PF1 1-20 level range) and Indomitable Will's flat
/// +4 magnitude and Tireless Rage both carry over unchanged. This needed
/// ZERO new record types and ZERO new choice slots — only a new
/// damage-reduction tier constant and one new arm on the existing
/// flat-magnitude formula, mirroring the level-13/level-16 idiom exactly.
///
/// A still further SD18 slice — the loop's TENTH §3.2 level-20 landing
/// candidate, widening the level-20 sweep — widens the gate to level 20,
/// the FINAL level within PF1's 1-20 character-level cap (verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror, byte-for-byte agreement: level 20 reads "+20/+15/+10/+5 | +12 |
/// +6 | +6 | Mighty rage, Rage power"): base-attack (classlevel = 20)
/// genuinely rises to +20 (full BAB) and good Fortitude genuinely rises to
/// +12 (20/2+2, up from +11), while poor Reflex/Will both stay +6 (20/3,
/// an integer-division coincidence with level 19); the rage rounds-per-day
/// pool genuinely rises to 45 (4 + Con mod + 2 per level after 1st). The
/// level-20 "Special" column's Mighty Rage entry is a genuine THIRD tier
/// on the SAME flat rage-surface constants already grounded at level 1 and
/// widened at level 11 (Greater Rage): the Strength/Constitution morale
/// bonus rises from +6 to +8 and the Will-save morale bonus rises from +3
/// to +4 via a new `BARBARIAN_MIGHTY_RAGE_LEVEL` gate constant, mirroring
/// the Greater Rage precedent exactly — no new record type, just a third
/// arm on the existing tiered formula. Level 20 IS a rage-power level
/// (powers land at 2/4/6/8/10/12/14/16/18/20), so a TENTH and FINAL
/// numbered slot (`choice:barbarian_rage_power_10`) is appended to
/// `BARBARIAN_RAGE_POWER_SLOTS`, mirroring the proven repeat-grant idiom
/// exactly; Damage Reduction stays 5/- (next rise would be 22nd, outside
/// the PF1 1-20 range) and Trap Sense stays +6 (next rise would be 21st,
/// also outside range); Indomitable Will's flat +4 magnitude and Tireless
/// Rage both carry over unchanged. This needed ZERO new record types and
/// ZERO new choice-slot mechanisms — only a new rage-magnitude tier
/// constant and a tenth numbered rage-power slot. This closes the
/// Barbarian per-level arithmetic-widening frontier: level 20 is the
/// final level within PF1's 1-20 character-level cap.
pub(super) const MAX_SUPPORTED_BARBARIAN_LEVEL: u8 = 20;

/// PF1 Core Rulebook level gate at which Barbarian Rage becomes Greater Rage
/// (11th level — "At 11th level, a barbarian's rage improves. She gains a
/// +6 morale bonus to Strength and Constitution and a +3 morale bonus on
/// Will saves ... the –2 penalty to AC remains", verified independently
/// against d20pfsrd and legacy.aonprd.com).
pub(super) const BARBARIAN_GREATER_RAGE_LEVEL: u8 = 11;

/// PF1 Core Rulebook level gate at which Barbarian Greater Rage becomes
/// Mighty Rage (20th level — "At 20th level, when a barbarian enters rage,
/// the morale bonus to her Strength and Constitution increases to +8 and
/// the morale bonus on her Will saves increases to +4", verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror, byte-for-byte agreement). A third tier on the same flat
/// rage-surface magnitude formula established at level 1 (Rage) and level
/// 11 (Greater Rage).
pub(super) const BARBARIAN_MIGHTY_RAGE_LEVEL: u8 = 20;

/// PF1 Core Rulebook level gate at which Barbarian gains Uncanny Dodge (2nd level,
/// verified against two independent primary sources — d20pfsrd and legacy.aonprd.com
/// both list "Rage power, uncanny dodge" as the Barbarian 2nd-level special feature
/// entry).
pub(super) const BARBARIAN_UNCANNY_DODGE_LEVEL: u8 = 2;

/// PF1 Core Rulebook level gate at which Barbarian gains Trap Sense (3rd level,
/// verified independently against two primary sources — d20pfsrd and
/// legacy.aonprd.com both list "Trap sense +1" as the Barbarian 3rd-level special
/// feature entry).
pub(super) const BARBARIAN_TRAP_SENSE_LEVEL: u8 = 3;

/// PF1 Core Rulebook level gate at which Barbarian gains Improved Uncanny Dodge
/// (5th level, verified independently against two primary sources — d20pfsrd and
/// legacy.aonprd.com both list "Improved uncanny dodge" as the Barbarian 5th-level
/// special feature entry).
pub(super) const BARBARIAN_IMPROVED_UNCANNY_DODGE_LEVEL: u8 = 5;

/// PF1 Core Rulebook level gate at which Barbarian gains Damage Reduction (7th
/// level, verified independently against two primary sources — d20pfsrd and
/// legacy.aonprd.com both list "Damage reduction 1/-" as the Barbarian 7th-level
/// special feature entry, and both give the rule text "At 7th level, a barbarian
/// gains damage reduction. Subtract 1 from the damage the barbarian takes each
/// time she is dealt damage from a weapon or a natural attack").
pub(super) const BARBARIAN_DAMAGE_REDUCTION_LEVEL: u8 = 7;

/// PF1 Core Rulebook level gate at which Barbarian Damage Reduction rises to
/// 2/— (10th level — "At 10th level, and every three barbarian levels
/// thereafter, this damage reduction rises by 1 point", verified
/// independently against d20pfsrd and legacy.aonprd.com).
pub(super) const BARBARIAN_DAMAGE_REDUCTION_TWO_LEVEL: u8 = 10;

/// PF1 Core Rulebook level gate at which Barbarian Damage Reduction rises to
/// 3/— (13th level — the same "every three barbarian levels thereafter"
/// clause applied a second time from the 10th-level gate, verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror: both name "Damage reduction 3/-" as the Barbarian 13th-level
/// "Special" class table entry).
pub(super) const BARBARIAN_DAMAGE_REDUCTION_THREE_LEVEL: u8 = 13;

/// PF1 Core Rulebook level gate at which Barbarian Damage Reduction rises to
/// 4/— (16th level — the same "every three barbarian levels thereafter"
/// clause applied a third time from the 10th-level gate, verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror: both name "Damage reduction 4/-, rage power" as the Barbarian
/// 16th-level "Special" class table entry).
pub(super) const BARBARIAN_DAMAGE_REDUCTION_FOUR_LEVEL: u8 = 16;

/// PF1 Core Rulebook level gate at which Barbarian Damage Reduction rises to
/// 5/— (19th level — the same "every three barbarian levels thereafter"
/// clause applied a fourth time from the 10th-level gate, verified
/// independently against d20pfsrd and the Archives of Nethys aonprd.com
/// mirror: both name "Damage reduction 5/-" as the Barbarian 19th-level
/// "Special" class table entry).
pub(super) const BARBARIAN_DAMAGE_REDUCTION_FIVE_LEVEL: u8 = 19;

/// PF1 Core Rulebook level gate at which Barbarian gains Indomitable Will
/// (14th level, verified independently against d20pfsrd and the Archives of
/// Nethys aonprd.com mirror, byte-for-byte agreement: both name "Indomitable
/// will, rage power" as the Barbarian 14th-level "Special" class table
/// entry, and both give the rule text "While she is raging, a barbarian
/// gains a +4 morale bonus on Will saves to resist enchantment spells and
/// effects").
pub(super) const BARBARIAN_INDOMITABLE_WILL_LEVEL: u8 = 14;

/// The flat Indomitable Will Will-save morale bonus (+4), grounded as a
/// value-only record mirroring the four pre-existing flat while-raging rage
/// constants (Strength/Constitution/Will-save morale bonuses, AC penalty):
/// never applied to any actual Will-save total.
pub(super) const BARBARIAN_INDOMITABLE_WILL_ENCHANTMENT_WILL_SAVE_BONUS: i16 = 4;

/// PF1 Core Rulebook level gate at which Barbarian gains Tireless Rage
/// (17th level, verified independently against d20pfsrd and the Archives of
/// Nethys aonprd.com mirror, byte-for-byte agreement across the full
/// levels-15-through-19 block: both name "Tireless rage" as the Barbarian
/// 17th-level "Special" class table entry, and both give the rule text
/// "Starting at 17th level, a barbarian no longer becomes fatigued at the
/// end of her rage"). This is a bounded grant-only identity record only
/// (value 0, non-fabricated): no rage-state execution engine exists
/// anywhere in this codebase to track when a rage ends or apply a fatigue
/// condition, so there is no fatigue-application mechanism for this
/// feature to interact with, and none is fabricated.
pub(super) const BARBARIAN_TIRELESS_RAGE_LEVEL: u8 = 17;

/// v0.6 alpha swarm, risks item 8 (second APG/ACG closure): whether
/// `input` is a single-class Bloodrager at a level within
/// `acg::class_chassis_resolve`'s declared ceiling for Bloodrager --
/// mirrors `is_supported_skald_single_class` exactly, including the same
/// exact-match discipline (`== Some(AcgClassId::Bloodrager)`, not a broad
/// `.is_some()` that would admit any of the 10 ACG classes).
pub(super) fn is_supported_bloodrager_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if AcgClassId::from_class_id_str(&class_level.class_id) != Some(AcgClassId::Bloodrager) {
        return false;
    }
    acg::class_chassis_resolve(AcgClassId::Bloodrager, class_level.level, RuleSetId::Acg).is_some()
}

/// Grounds Raging Climber's and Raging Swimmer's shared `RagePowersLVL`
/// magnitude as two explanation records (task #54, canonical-narrowing
/// follow-on to #53's `rage-powers-canonical-narrowing-scoping.md`: a
/// 60-record Rage Powers chooser family with zero magnitudes grounded
/// before this pair). Called from both `ground_or_block_barbarian_rage`
/// and `ground_or_block_skald_inspired_rage`, at each of their own
/// "currently raging/singing or not" branch points, so the grounded
/// records read the SAME activation truth the rage-execution engine
/// itself already established rather than re-deriving it independently --
/// they cannot disagree.
///
/// Corpus (verified directly against `cr_abilities_class.lst`, `KEY:Rage
/// Power ~ Raging Climber` / `~ Raging Swimmer`):
/// `BONUS:VAR|RagingClimberBonus|RagePowersLVL` /
/// `BONUS:VAR|RagingSwimmerBonus|RagePowersLVL` (no arithmetic -- the
/// magnitude IS the level) and
/// `BONUS:SKILL|Climb|RagingClimberBonus|PREVAREQ:Raging,1` /
/// `BONUS:SKILL|Swim|RagingSwimmerBonus|PREVAREQ:Raging,1`. Grounds BOTH
/// branches ("ground the absence, don't omit it"): while actively raging/
/// singing, the magnitude is `level` (= RagePowersLVL); while not, an
/// honest value-0 "not currently benefiting" record -- unlike a
/// level-gated feature, `PREVAREQ:Raging,1` makes the corpus bonus itself
/// entirely conditional on the active-Raging state, not on level alone.
///
/// This is the canonical-narrowing representative of the wider Rage
/// Powers family: it does NOT validate that the character actually
/// selected Raging Climber or Raging Swimmer among their own limited
/// rage-power slots -- that chooser-selection enforcement is the same
/// still-open engine burden every one of the other 58 named-but-unmodeled
/// rage powers in the family carries (see `BARBARIAN_RAGE_POWER_SLOTS`'s
/// own open-ended, unvalidated selection recognitions, which record
/// WHICH power string was picked but apply no power's effect). Applying
/// this magnitude unconditionally to every actively-raging Barbarian/
/// Skald is the deliberate canonical-narrowing simplification #53's own
/// scoping doc recommends, not an oversight.
pub(super) fn ground_raging_climber_and_swimmer(
    level: u8,
    class_label: &str,
    id_prefix: &str,
    is_raging: bool,
    in_bounded_scope: bool,
    explanations: &mut Vec<ComputationExplanation>,
) {
    // Task #74. These are real per-class skill-bonus magnitudes, so they must
    // stay inside their class's bounded scope like every other
    // `class_feature.<class>.*` record -- a multiclass character, or one at a
    // level outside the class's supported range, must ground nothing here.
    //
    // They sit next to the rage/inspired-rage EXECUTION-STATE recognition
    // records (`rage_execution.not_raging`, `inspired_rage_execution.
    // not_singing`), which are deliberately universal: those exist precisely so
    // a spoofed activation on a non-Barbarian is caught rather than silently
    // ignored, and the negative-control tests allow-list them by exact id for
    // that reason. Task #54 added these calls beside those records and
    // inherited that universality by accident -- but a skill bonus is not an
    // execution-state recognition, so it does not belong in the allow-list.
    //
    // The guard lives here rather than at the call sites on purpose: there are
    // six of them across two classes, and a seventh added later would silently
    // reopen the leak. Each caller supplies its own class-appropriate gate.
    if !in_bounded_scope {
        return;
    }

    let magnitude = if is_raging { i16::from(level) } else { 0 };

    for (power_name, bonus_var, skill) in [
        ("Raging Climber", "RagingClimberBonus", "Climb"),
        ("Raging Swimmer", "RagingSwimmerBonus", "Swim"),
    ] {
        let slug = power_name.to_lowercase().replace(' ', "_");
        let detail = if is_raging {
            format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   PREVAREQ:Raging,1
                "{class_label} level {level} {power_name} (PF1 Core Rulebook rage power, \
                 canonical-narrowing representative of the 60-record Rage Powers family, task \
                 #54/#53): while raging, a +{magnitude} enhancement bonus to all {skill} skill \
                 checks (the corpus states the magnitude as the rage-power level {bonus_var} \
                 with no arithmetic, and that level tracks {class_label} level directly). Gated on \
                 the same active-Raging state as the \
                 rage-execution engine above, and integrated \
                 into the real {skill} total in compute_selected_skill_modifiers. This grounds the \
                 magnitude formula only, unconditionally for any actively raging {class_label} -- it \
                 does not validate that {power_name} was actually chosen among the character's \
                 limited rage-power slots (out of scope per #53's own canonical-narrowing \
                 recommendation; see ground_raging_climber_and_swimmer's own doc comment)"
            )
        } else {
            format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   PREVAREQ:Raging,1 is entirely conditional on the active-Raging state, unlike a
                //   level-gated feature
                "{class_label} level {level} {power_name}: not currently raging, so no enhancement \
                 bonus to {skill} is claimed (the corpus grants it only while raging). Mirrors the \
                 rage-execution engine's own \"not raging\" posture above -- a genuinely valid PF1 \
                 state, not a claim-blocking one"
            )
        };
        explanations.push(ComputationExplanation {
            id: format!("{id_prefix}.{slug}"),
            value: magnitude,
            detail,
        });
    }
}

/// Bloodrager's Bloodrage rounds-per-day budget: the PCGen corpus formula
/// `2 + Constitution modifier + 2*level` (v0.6 alpha swarm, risks item 8,
/// second APG/ACG closure) -- algebraically identical to Barbarian's own
/// `4 + Constitution modifier + 2*(level-1)`, both reducing to
/// `2 + Constitution modifier + 2*level`, so this uses
/// `BLOODRAGER_BLOODRAGE_BASE_ROUNDS_PER_DAY` (2) with a plain `2*level`
/// term rather than Barbarian's own `2*(level-1)` shape, matching the
/// corpus's own additive decomposition exactly.
pub(super) fn bloodrager_bloodrage_rounds_per_day(constitution_modifier: i16, level: u8) -> i16 {
    BLOODRAGER_BLOODRAGE_BASE_ROUNDS_PER_DAY + constitution_modifier + 2 * i16::from(level)
}

/// Bloodrager's Bloodrage magnitude tier: (Strength/Constitution morale
/// bonus, Will-save morale bonus), verified against the PCGen corpus DESC
/// text and `BONUS:VAR` formulas -- rising from Bloodrage (+4/+4/+2) to
/// Greater Bloodrage at `BARBARIAN_GREATER_RAGE_LEVEL` (+6/+6/+3) to
/// Mighty Bloodrage at `BARBARIAN_MIGHTY_RAGE_LEVEL` (+8/+8/+4),
/// identical thresholds and magnitudes to Barbarian's own Rage tiers, so
/// the shared constants are reused directly rather than re-declared under
/// a Bloodrager-specific name. The Armor Class penalty stays -2 at every
/// tier (not part of this tuple, mirrors Barbarian's own shape) --
/// callers needing it use `BLOODRAGER_BLOODRAGE_ARMOR_CLASS_PENALTY`
/// directly.
pub(super) fn bloodrager_bloodrage_tier(level: u8) -> (i16, i16) {
    if level >= BARBARIAN_MIGHTY_RAGE_LEVEL {
        (8, 4)
    } else if level >= BARBARIAN_GREATER_RAGE_LEVEL {
        (6, 3)
    } else {
        (4, 2)
    }
}

/// Whether `input` is a Bloodrager validly, actively bloodraging right
/// now, and if so, the magnitude tier to apply (v0.6 alpha swarm, risks
/// item 8, second APG/ACG closure). Class-ownership-gated by construction,
/// mirroring `active_barbarian_rage_bonus`/`active_skald_inspired_rage_bonus`
/// exactly: only returns `Some` when `class_levels` actually contains
/// Bloodrager, so a non-Bloodrager character's stray
/// `class_ability_activations` entry for `BLOODRAGER_BLOODRAGE_ABILITY_ID`
/// is never read at all. Unlike Skald, Bloodrage is self-only by RAW, so
/// no self-application inference or disclaimer is needed here.
pub(super) fn active_bloodrager_bloodrage_bonus(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
) -> Option<(u8, i16, i16)> {
    let bloodrager_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == BLOODRAGER_CLASS_ID)
        .map(|class_level| class_level.level)?;

    let activation = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == BLOODRAGER_BLOODRAGE_ABILITY_ID)?;

    if activation.active_state != ActiveState::EquippedActive {
        return None;
    }

    if let Some(rounds_consumed) = activation.rounds_consumed_today {
        let constitution_modifier = ability_modifier_for(ability_modifiers, "constitution");
        let rounds_per_day =
            bloodrager_bloodrage_rounds_per_day(constitution_modifier, bloodrager_level);
        if i32::from(rounds_consumed) > i32::from(rounds_per_day) {
            return None;
        }
    }

    let (strength_constitution_bonus, will_save_bonus) = bloodrager_bloodrage_tier(bloodrager_level);
    Some((bloodrager_level, strength_constitution_bonus, will_save_bonus))
}

/// Bloodrager Fast Movement: a flat `+10` ft to land speed
/// (`BONUS:VAR|BloodrageMovementBonus|10`), granted on the class table's
/// own level-1 row.
///
/// Flat, unlike Monk's own Fast Movement (`10*floor(MonkLVL/3)`, task
/// #36) -- a third class sharing that feature name with a third
/// magnitude, so none of the three is a magnitude precedent for another.
/// Barbarian's is also flat +10.
///
/// Like Monk's, it is armor/encumbrance-conditional: the
/// `BONUS:MOVEADD` carries `PREVARLT:ENCUMBERANCE,2` plus a no-heavy-armor
/// clause. This engine models no encumbrance state, so the magnitude is
/// grounded and the condition named.
pub(super) const BLOODRAGER_FAST_MOVEMENT_FEET: i16 = 10;

/// Bloodrager Blood Sanctuary's flat save bonus against spells:
/// `BONUS:VAR|BloodragerBloodSanctuaryBonus|2`, class-table level 3.
pub(super) const BLOODRAGER_BLOOD_SANCTUARY_BONUS: i16 = 2;

/// Bloodrager Indomitable Will's own level gate and magnitude (task
/// #83), from `KEY:Bloodrager ~ Indomitable Will`: "At 14th level, a
/// bloodrager gains a +4 bonus on Will saves to resist enchantment
/// spells while bloodraging", carried in the corpus as
/// `ASPECT:SaveBonus|While Bloodraging +4 vs. enchantments`.
///
/// **Deliberately its own constants rather than reusing Barbarian's**
/// already-grounded `BARBARIAN_INDOMITABLE_WILL_*`, which happen to hold
/// the same 14/+4. The two are separate corpus records on separate
/// classes that coincide numerically; sharing a constant would encode a
/// dependency the corpus does not have, and would silently propagate an
/// errata to one class into the other.
pub(super) const BLOODRAGER_INDOMITABLE_WILL_LEVEL: u8 = 14;

pub(super) const BLOODRAGER_INDOMITABLE_WILL_ENCHANTMENT_WILL_SAVE_BONUS: i16 = 4;

/// Blood Casting's and Eschew Materials' own level gates (task #83),
/// both 4th-level bloodrager class features per their own corpus
/// records' DESC.
///
/// Both are deliberately separate from `BLOODRAGER_FIRST_CASTING_LEVEL`,
/// which is also 4. That one is derived from the class's own
/// `BONUS:CASTERLEVEL|...|PRECLASS:1,Bloodrager=4`; these two come from
/// their own `KEY:Bloodrager ~ ...` records. Three independent corpus
/// facts that agree on a number are still three facts, and collapsing
/// them would make a future divergence invisible.
pub(super) const BLOODRAGER_BLOOD_CASTING_LEVEL: u8 = 4;

pub(super) const BLOODRAGER_ESCHEW_MATERIALS_LEVEL: u8 = 4;

/// Greater (11) and Mighty (20) Bloodrage are NOT new logic here: the
/// shipped `bloodrager_bloodrage_tier` already returns the full
/// progression (+4/+2 base, +6/+3 from 11, +8/+4 from 20), reusing
/// Barbarian's own `GREATER_RAGE`/`MIGHTY_RAGE` level constants, whose
/// gates are numerically identical for Bloodrager. What was missing was
/// only a NAMED standalone record for each tier at its own gate -- the
/// magnitudes were computed but reachable only through the active
/// bloodraging path, so a non-raging bloodrager's receipt never mentioned
/// them. These records reuse that same function rather than restating the
/// progression, so the two cannot drift apart.
/// **Two independent things produce the 7th-level gate, and it is worth
/// knowing both.** The `.MOD` grant line carries only the
/// archetype-suppression flag (`PREVAREQ:Bloodrager_CF_DamageReduction,0`,
/// provably vacuous here) and no level `PRE` at all -- but the Bloodrager
/// CLASS TABLE grants the ability on its own level-7 row, and the formula
/// independently reaches 1 only at level 7. Corrected 2026-07-27 (task
/// #42): an earlier revision of this comment said the gate was produced
/// "solely by the formula", which overstated it by overlooking the class
/// table's per-level grant row.
///
/// Clamped at 0 for a real reason, not defensively: the raw expression
/// goes NEGATIVE below level 4 (at level 1, `(1-4)/3` is -1 under Rust's
/// truncating division), and a negative DR would be a fabricated value
/// rather than an absent one.
pub(super) fn bloodrager_damage_reduction_amount(level: u8) -> i16 {
    ((i16::from(level) - 4) / 3).max(0)
}

/// Grounds Bloodrager's six remaining flat class features (task #42).
/// Gates read off the Bloodrager CLASS TABLE's own per-level `ABILITY:`
/// rows -- 1/2/3/5/11/20 -- NOT off the `.MOD` grant lines, which carry
/// only archetype-suppression flags and no level `PRE` at all. That is
/// the second grant mechanism in this corpus and the one that actually
/// carries the gates here; looking only at the grant lines would suggest
/// every one of these is ungated.
///
/// Every feature grounds BOTH branches -- a value-0 "correctly absent"
/// record below its gate -- following the "ground the absence, don't omit
/// it" discipline `ground_skald_damage_reduction` and Barbarian's own DR
/// already use.
pub(super) fn ground_bloodrager_remaining_features(
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let gated = |gate: u8, granted: i16| if level >= gate { granted } else { 0 };

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.bloodrager.fast_movement".to_owned(),
        value: gated(1, BLOODRAGER_FAST_MOVEMENT_FEET),
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|BloodrageMovementBonus|10
            //   PREVARLT:ENCUMBERANCE,2 plus a no-heavy-armor clause
            "Bloodrager Fast Movement at bloodrager level {level}: a flat \
             +{BLOODRAGER_FAST_MOVEMENT_FEET} ft to land speed, granted on the class table's own \
             level-1 row. FLAT, unlike Monk's own Fast Movement (10*floor(MonkLVL/3), task #36) -- a \
             third class sharing this feature name with a third magnitude, so none of the three is a \
             magnitude precedent for another. Like Monk's, the corpus speed increase is \
             armor/encumbrance-conditional: this engine models no encumbrance state, so the \
             magnitude is grounded and the condition named rather than applied"
        ),
    });

    let uncanny_dodge_flanking_level = gated(2, i16::from(level));
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.bloodrager.uncanny_dodge_flanking_level".to_owned(),
        value: uncanny_dodge_flanking_level,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|UncannyDodgeFlankingLevel|BloodragerLVL, class table level 2
            "Bloodrager Uncanny Dodge at bloodrager level {level}: the bloodrager cannot be caught \
             flat-footed and keeps his Dexterity bonus to AC when flanked, and a rogue needs level \
             {uncanny_dodge_flanking_level} or higher to flank him. The magnitude lives on an \
             INTERNAL 'Uncanny Dodge Tracker' record rather than the named feature's own record, \
             which is why a KEY:Bloodrager ~ Uncanny Dodge lookup finds no tokens. Grounded as a \
             scoped standalone fact: no flanking, flat-footed, or initiative-order engine exists in \
             this codebase to apply it"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.bloodrager.blood_sanctuary".to_owned(),
        value: gated(3, BLOODRAGER_BLOOD_SANCTUARY_BONUS),
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|BloodragerBloodSanctuaryBonus|2, class table level 3
            "Bloodrager Blood Sanctuary at bloodrager level {level}: a flat \
             +{BLOODRAGER_BLOOD_SANCTUARY_BONUS} bonus on saving throws against spells he casts on \
             himself, and against spells from creatures of his own bloodline. Grounded as a flat \
             magnitude only: it is never applied to any save total, since the scope condition (whose \
             spell, and whose bloodline) is not represented here"
        ),
    });

    // The corpus adds a SECOND +1 to the same UncannyDodgeLVL var at
    // level 5, so the tier count is 1 from level 2 and 2 from level 5.
    let uncanny_dodge_tier = gated(2, 1) + gated(5, 1);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.bloodrager.improved_uncanny_dodge_tier".to_owned(),
        value: uncanny_dodge_tier,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|UncannyDodgeLVL|1|PREVARGTEQ:BloodragerLVL,5
            "Bloodrager Uncanny Dodge tier at bloodrager level {level}: {uncanny_dodge_tier} (0 \
             below level 2, 1 from level 2, 2 from level 5). The corpus expresses Improved Uncanny \
             Dodge as a SECOND +1 to the same UncannyDodgeLVL variable rather than a distinct \
             magnitude, so the two tiers are facets of one counter. At tier 2 the bloodrager can no \
             longer be flanked except by a rogue of high enough level. No flanking engine exists \
             here to apply either tier"
        ),
    });

    let (bloodrage_ability_bonus, bloodrage_save_bonus) = bloodrager_bloodrage_tier(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.bloodrager.bloodrage_tier_ability_bonus".to_owned(),
        value: bloodrage_ability_bonus,
        detail: format!(
            "Bloodrager Bloodrage morale bonus to Strength and Constitution at bloodrager \
             level {level}: +{bloodrage_ability_bonus} (+4 base, +6 from Greater Bloodrage at \
             level 11, +8 from Mighty Bloodrage at level 20). Greater and Mighty Bloodrage \
             carry IDENTICAL corpus tokens \
             and STACK rather than replace, which is why both records look the same: +4/+4 \
             base, +6/+6 from 11, +8/+8 at 20, matching PF1's own published progression. This \
             grounds the tier magnitude; whether the bloodrager is currently raging is handled \
             by the Bloodrage execution records"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_feature.acg.bloodrager.bloodrage_tier_save_bonus".to_owned(),
        value: bloodrage_save_bonus,
        detail: format!(
            "Bloodrager Bloodrage morale bonus to Will saves at bloodrager level {level}: +\
             {bloodrage_save_bonus} (+2 base, +3 from level 11, +4 from level 20 -- the same \
             stacking Greater/Mighty structure as the ability bonus above). Reuses the shipped \
             bloodrager_bloodrage_tier, so this record and the active-bloodrage path cannot \
             disagree"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.bloodrager.indomitable_will".to_owned(),
        value: gated(
            BLOODRAGER_INDOMITABLE_WILL_LEVEL,
            BLOODRAGER_INDOMITABLE_WILL_ENCHANTMENT_WILL_SAVE_BONUS,
        ),
        detail: if level < BLOODRAGER_INDOMITABLE_WILL_LEVEL {
            format!(
                "Bloodrager Indomitable Will at bloodrager level {level}: correctly absent at \
                 level {level} by its own corpus level gate; the at-grant magnitude is named but \
                 not computed. Indomitable Will is a 14th-level bloodrager class feature."
            )
        } else {
            format!(
                "Bloodrager Indomitable Will granted at bloodrager level {level} (PF1 Advanced \
                 Class Guide, 14th-level bloodrager class feature): while bloodraging, a \
                 +{BLOODRAGER_INDOMITABLE_WILL_ENCHANTMENT_WILL_SAVE_BONUS} bonus on Will saves \
                 to resist enchantment spells, which the corpus states STACKS with all other \
                 modifiers including the Will-save morale bonus the bloodrage itself grants \
                 (`ASPECT:SaveBonus|While Bloodraging +4 vs. enchantments`). This is a bounded \
                 flat-magnitude record only: no saving-throw-resolution engine, no \
                 spell-school-classification engine (to decide an incoming save is against an \
                 enchantment), and no bloodrage-state gate is applied to it here, so it grounds \
                 no actual Will-save bonus. Same shape and magnitude as Barbarian's own \
                 already-grounded Indomitable Will, but a SEPARATE corpus record on a separate \
                 class -- the two coincide numerically rather than sharing a source"
            )
        },
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.bloodrager.blood_casting".to_owned(),
        value: 0,
        detail: if level < BLOODRAGER_BLOOD_CASTING_LEVEL {
            format!(
                "Bloodrager Blood Casting at bloodrager level {level}: correctly absent at level \
                 {level} by its own corpus level gate. Blood Casting is a 4th-level bloodrager \
                 class feature."
            )
        } else {
            format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   Its corpus record carries no `BONUS:` token of any kind -- the feature is a
                //   permission, not a magnitude.
                "Bloodrager Blood Casting granted at bloodrager level {level} (PF1 Advanced Class \
                 Guide, 4th-level bloodrager class feature): the bloodrager can cast his bloodrager \
                 spells while bloodraging, may cast them defensively, and may attempt concentration \
                 checks for them while bloodraging. Grounded at +0 as genuinely vacuous under this \
                 scope rather than fabricated: this codebase models no concentration check anywhere \
                 (no id contains `concentration`), and imposes no casting restriction while \
                 bloodraging for Blood Casting to lift. The same treatment Monk's Catch \
                 Off-Guard/Throw Anything already uses: the benefit is real, its triggering \
                 condition never arises here"
            )
        },
    });

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.bloodrager.eschew_materials".to_owned(),
        value: 0,
        detail: if level < BLOODRAGER_ESCHEW_MATERIALS_LEVEL {
            format!(
                "Bloodrager Eschew Materials at bloodrager level {level}: correctly absent at \
                 level {level} by its own corpus level gate. The Eschew Materials bonus feat is \
                 granted at 4th level."
            )
        } else {
            format!(
                "Bloodrager Eschew Materials granted at bloodrager level {level} (PF1 Advanced \
                 Class Guide, 4th-level bloodrager class feature): the bloodrager gains Eschew \
                 Materials as a bonus feat (`ABILITY:FEAT|AUTOMATIC|Eschew Materials`), letting \
                 him cast a spell with a material component costing 1 gp or less without that \
                 component. A boolean feat grant, not a numeric bonus, so it carries no \
                 fabricated mechanical value (+0) -- the same shape as Sorcerer's own \
                 already-grounded `class_chassis.sorcerer.eschew_materials`, differing only in \
                 the level at which each class grants it. It grounds no spell math and no \
                 material-component economy, neither of which this codebase models"
            )
        },
    });
}

/// Grounds Bloodrager's own Damage Reduction, mirroring
/// `ground_skald_damage_reduction`/`class_feature.barbarian.damage_reduction`
/// exactly: a real, level-gated, flat-magnitude fact never applied to any
/// incoming-damage total (none exists anywhere in this codebase). Grounds
/// BOTH branches, including an honest value-0 "not yet gained" record
/// below level 7 -- the same "ground the absence, don't omit it"
/// discipline the sibling DR facts use.
///
/// Passive, so it is grounded regardless of whether the bloodrager is
/// currently bloodraging: unlike the Bloodrage bonuses, the corpus record
/// carries no activation condition at all.
pub(super) fn ground_bloodrager_damage_reduction(level: u8, explanations: &mut Vec<ComputationExplanation>) {
    let damage_reduction_value = bloodrager_damage_reduction_amount(level);
    if damage_reduction_value == 0 {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.bloodrager.damage_reduction".to_owned(),
            value: 0,
            detail: format!(
                "Bloodrager Damage Reduction at bloodrager level {level}: correctly absent at \
                 level {level} by PF1 Advanced Class Guide level gate; the at-grant magnitude \
                 is named but not computed. Damage Reduction is a 7th-level bloodrager class \
                 feature. Note the corpus expresses that gate purely through its own formula \
                 ((BloodragerLVL-4)/3, which first reaches 1 at level 7) -- the grant line \
                 carries no level PRE token at all"
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.bloodrager.damage_reduction".to_owned(),
            value: damage_reduction_value,
            detail: format!(
                "Bloodrager Damage Reduction granted at bloodrager level {level} (PF1 Advanced \
                 Class Guide, 7th-level bloodrager class feature rising by 1 every three levels \
                 -- corpus DR:BloodragerDR/- with BloodragerDR = (BloodragerLVL-4)/3, so the \
                 level-{level} magnitude is {damage_reduction_value}/-): subtract \
                 {damage_reduction_value} from the damage the bloodrager takes each time he is \
                 dealt damage from a weapon or a natural attack. The DR is bypassed by nothing \
                 (\"/-\"), unlike Paladin's own DR 5/evil. This is passive and applies whether \
                 or not he is bloodraging. The subtraction against an actual incoming-damage \
                 total is not computed: no damage-resolution engine or incoming-damage total \
                 exists anywhere in this codebase"
            ),
        });
    }
}

/// Grounds or claim-blocks Bloodrager's Bloodrage execution engine for
/// `bloodrager_level` (v0.6 alpha swarm, risks item 8, second APG/ACG
/// closure). Called from `compute_acg_class_chassis`'s Bloodrager branch,
/// gated only on Bloodrager class-ownership -- mirrors
/// `ground_or_block_barbarian_rage`/`ground_or_block_skald_inspired_rage`
/// structurally, including the same fatigue-not-modeled honesty note
/// below `BARBARIAN_TIRELESS_RAGE_LEVEL` (Bloodrager's own Tireless
/// Bloodrage fires at the identical 17th-level threshold, per the corpus).
///
/// Unlike Barbarian, this is Bloodrager's ONLY grounded class feature this
/// slice -- spellcasting (Bloodrager casts from its own
/// `SPELLLIST:1|Bloodrager`, not Bard's list) and every other
/// named-but-unbuilt Bloodrager feature remain claim-blocked via the new,
/// narrower `class_feature.acg.bloodrager.spellcasting_deferred.unsupported`
/// diagnostic pushed unconditionally below, mirroring Skald's own
/// diagnostic-honesty fix.
pub(super) fn ground_or_block_bloodrager_bloodrage(
    input: &CharacterInput,
    bloodrager_level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // Passive and activation-independent, so it is grounded before the
    // bloodraging branch splits -- both branches must carry it.
    ground_bloodrager_damage_reduction(bloodrager_level, explanations);
    ground_bloodrager_remaining_features(bloodrager_level, explanations);
    ground_bloodrager_arcane_bloodline(input, bloodrager_level, ability_modifiers, explanations);

    // SD-32 T12 Epic 8 row 18 cycle 6: the same "select ONE bloodline, inherit every one of its
    // real corpus powers" generic pass cycle 5 wired for Sorcerer's own 51 unmodelled bloodlines
    // (Bloodrager's bloodlines are PARALLEL to Sorcerer's, not shared -- `push_bloodrager_other_
    // features_deferred_diagnostic`'s own doc, task #59 -- so this is Bloodrager's own separate
    // corpus work, purely additive alongside `ground_bloodrager_arcane_bloodline`'s hand-modelled
    // Arcane branch above). Named by cycle 5's receipt as "same mechanism, not yet called" and
    // left unwired purely as a time-boxing choice, not a mechanism gap.
    push_generic_pool_group_selection_magnitude(
        input,
        bloodrager_level,
        ability_modifiers,
        BLOODRAGER_BLOODLINE_CHOICE_ID,
        "Bloodrager",
        "Bloodline",
        "bloodline:",
        "class_feature.acg.bloodrager.bloodline.generic",
        1,
        explanations,
    );

    let Some(activation) = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == BLOODRAGER_BLOODRAGE_ABILITY_ID)
    else {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.bloodrager.bloodrage_execution.not_raging".to_owned(),
            value: 0,
            detail: format!(
                "Bloodrager level {bloodrager_level} is not currently bloodraging (no \
                 class_ability_activations entry for \
                 \"{BLOODRAGER_BLOODRAGE_ABILITY_ID}\"): a genuinely valid PF1 posture, so no \
                 rage bonus, penalty, or budget is claimed. This grounds the Bloodrage execution \
                 engine's \"inactive\" branch only; bloodraging is grounded separately below \
                 when an active, in-budget activation is present"
            ),
        });
        push_bloodrager_spellcasting_deferred_diagnostic(input, bloodrager_level, diagnostics);
        push_bloodrager_other_features_deferred_diagnostic(input, diagnostics);
        ground_bloodrager_spell_tables(bloodrager_level, explanations);
        ground_or_block_bloodrager_known_spells(
            input,
            bloodrager_level,
            explanations,
            diagnostics,
        );
        return;
    };

    let constitution_modifier = ability_modifier_for(ability_modifiers, "constitution");
    let rounds_per_day =
        bloodrager_bloodrage_rounds_per_day(constitution_modifier, bloodrager_level);

    if let Some(rounds_consumed) = activation.rounds_consumed_today
        && i32::from(rounds_consumed) > i32::from(rounds_per_day) {
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.acg.bloodrager.bloodrage_execution.rounds_exceeded".to_owned(),
                message: format!(
                    "Bloodrager level {bloodrager_level} Bloodrage activation claims \
                     {rounds_consumed} rounds consumed today, exceeding the grounded \
                     rounds-per-day budget of {rounds_per_day} (2 + Constitution modifier \
                     ({constitution_modifier}) + 2*level): a genuine posture violation, so no \
                     rage bonus, penalty, or budget is claimed for this input"
                ),
                claim_blocking: true,
            });
            push_bloodrager_spellcasting_deferred_diagnostic(input, bloodrager_level, diagnostics);
        push_bloodrager_other_features_deferred_diagnostic(input, diagnostics);
        ground_bloodrager_spell_tables(bloodrager_level, explanations);
        ground_or_block_bloodrager_known_spells(
            input,
            bloodrager_level,
            explanations,
            diagnostics,
        );
            return;
        }

    match activation.active_state {
        ActiveState::EquippedActive => {
            let (strength_constitution_bonus, will_save_bonus) =
                bloodrager_bloodrage_tier(bloodrager_level);
            let rounds_consumed_today = activation.rounds_consumed_today.unwrap_or(0);
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.bloodrager.bloodrage_execution.active".to_owned(),
                value: 0,
                detail: format!(
                    "Bloodrager level {bloodrager_level} is actively bloodraging, within the \
                     grounded rounds-per-day budget ({rounds_per_day} rounds; \
                     {rounds_consumed_today} consumed today). The \
                     +{strength_constitution_bonus} Strength / +{strength_constitution_bonus} \
                     Constitution / +{will_save_bonus} Will morale bonuses and the \
                     {BLOODRAGER_BLOODRAGE_ARMOR_CLASS_PENALTY} Armor Class penalty are applied \
                     to the integrated ability modifiers, total saves, and baseline Armor Class \
                     respectively -- see apply_bloodrager_bloodrage_ability_bonuses, \
                     compute_total_saves, and compute_combat_baseline. Bloodrage is self-only by \
                     RAW (no exception-clause self-application inference is needed, unlike \
                     Skald's Inspired Rage)"
                ),
            });
            if bloodrager_level < BARBARIAN_TIRELESS_RAGE_LEVEL {
                diagnostics.push(ComputationDiagnostic {
                    id: "class_feature.acg.bloodrager.bloodrage_execution.fatigue_not_modeled"
                        .to_owned(),
                    message: format!(
                        "Bloodrager level {bloodrager_level} is bloodraging below the Tireless \
                         Bloodrage threshold ({BARBARIAN_TIRELESS_RAGE_LEVEL}th level): PF1 \
                         Bloodrage causes fatigue once the bloodrage ends (for twice the number \
                         of rounds spent bloodraging), which this codebase does not yet \
                         represent as a transient post-bloodrage state, so no fatigue condition \
                         is applied. Named honestly rather than silently modeled or silently \
                         dropped"
                    ),
                    claim_blocking: false,
                });
            }
        }
        ActiveState::SelectedInactive | ActiveState::Absent => {
            explanations.push(ComputationExplanation {
                id: "class_feature.acg.bloodrager.bloodrage_execution.not_raging".to_owned(),
                value: 0,
                detail: format!(
                    "Bloodrager level {bloodrager_level} has a \
                     \"{BLOODRAGER_BLOODRAGE_ABILITY_ID}\" activation entry but it is not active \
                     for this snapshot: a genuinely valid PF1 posture (available but not \
                     currently bloodraging), so no rage bonus, penalty, or budget is claimed"
                ),
            });
        }
    }

    push_bloodrager_spellcasting_deferred_diagnostic(input, bloodrager_level, diagnostics);
        push_bloodrager_other_features_deferred_diagnostic(input, diagnostics);
        ground_bloodrager_spell_tables(bloodrager_level, explanations);
        ground_or_block_bloodrager_known_spells(
            input,
            bloodrager_level,
            explanations,
            diagnostics,
        );
}

/// Pushes the new, narrower diagnostic replacing
/// `class_feature.acg.bloodrager.unsupported` for Bloodrager specifically
/// (mirroring `push_skald_spellcasting_deferred_diagnostic`): named ONLY
/// the genuinely still-missing pieces (spellcasting from Bloodrager's own
/// spell list, and every other named-but-unbuilt Bloodrager class feature
/// beyond Bloodrage), unlike the retired diagnostic's blanket "no named
/// class-feature computation... grounded anywhere" claim, which is now
/// false for Bloodrager. Pushed regardless of Bloodrage's own raging
/// state.
///
/// **Canonical narrowing (v0.6 alpha swarm, Bloodrager
/// spellcasting-shaped closure).** By its own text this diagnostic had
/// already narrowed to ONE remaining item: the Bloodline bonus spells,
/// which are downstream of the Bloodline slot rather than of anything
/// spellcasting-specific. So it now resolves the same way that slot does
/// -- a recognized, grounded bloodline (whose four bonus spells ARE
/// grounded, see `ground_bloodrager_arcane_bloodline_bonus_spells`)
/// downgrades it to a NON-blocking note naming the other nine
/// bloodlines' bonus spells; any other (or no) selection keeps it
/// claim-blocking, unchanged.
pub(super) fn push_bloodrager_spellcasting_deferred_diagnostic(
    input: &CharacterInput,
    level: u8,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if level >= BLOODRAGER_FIRST_CASTING_LEVEL {
        let bloodline_recognized = choice_selection(input, BLOODRAGER_BLOODLINE_CHOICE_ID)
            == Some(ARCANE_BLOODRAGER_BLOODLINE_SELECTION);
        let shared = "it casts from its own spell list (`SPELLLIST:1|Bloodrager`, no borrowed \
             list), its spells-per-day and spells-known tables are grounded, and as of task #87 \
             its 200-entry `acg::bloodrager_spell_list` is genuinely WIRED -- a recorded \
             known-spell selection is validated against that real list, the access ceiling \
             derived from the shipped Spells Known table, and that table's own per-level caps, \
             grounding `class_spell.acg.bloodrager.known_spells` when the posture holds and \
             claim-blocking with the specific unmet reasons when it does not. No spell save DC \
             resolution against a target and no casting execution is claimed for any Bloodrager \
             spell. This message has twice described the spell list wrongly: it said \"not \
             built\" when the list had existed since task #1, then (task #83) \"built but NOT \
             WIRED\", which was true when written and stopped being true here";
        let message = if bloodline_recognized {
            format!(
                "{BLOODRAGER_CLASS_ID} level {level} spellcasting posture is grounded: {shared}. \
                 The one item this diagnostic still named -- the Bloodline bonus spells -- is \
                 grounded for the one bloodline this codebase grounds (Arcane: Magic Missile at \
                 7th, Invisibility at 10th, Lightning Bolt at 13th, Dimension Door at 16th, \
                 each read off the bloodline record's own `SPELLKNOWN:` token). What stays \
                 deferred, honestly and non-blockingly: the other 9 bloodlines' own four bonus \
                 spells apiece, and the Elemental bloodline's element sub-choice -- both \
                 downstream of the Bloodline slot, not of spellcasting"
            )
        } else {
            format!(
                "{BLOODRAGER_CLASS_ID} level {level} remains blocked on its spellcasting \
                 posture: {shared}. What remains genuinely unbuilt: the Bloodline bonus spells \
                 for the 9 bloodlines this codebase does not ground (4 spells each, first \
                 granted at bloodline progression level 7) and the Elemental bloodline's own \
                 element sub-choice -- both downstream of the Bloodline slot itself, which is \
                 this class's other open blocker, and no recognized bloodline choice is present \
                 here"
            )
        };
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.acg.bloodrager.spellcasting_deferred.unsupported".to_owned(),
            message,
            claim_blocking: !bloodline_recognized,
        });
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.acg.bloodrager.spellcasting_absent_by_level_gate".to_owned(),
            message: format!(
                "{BLOODRAGER_CLASS_ID} level {level} has NO spellcasting to defer: the real \
                 class block carries no spells-per-day or spells-known row at all below level \
                 {BLOODRAGER_FIRST_CASTING_LEVEL}, and its caster level is itself gated on \
                 reaching bloodrager level {BLOODRAGER_FIRST_CASTING_LEVEL}. Correctly absent by \
                 level gate rather than missing, so this does not block"
            ),
            claim_blocking: false,
        });
    }
}

/// Pushes Bloodrager's genuinely-still-missing class features (task #1,
/// 2026-07-27), split out of the old single diagnostic so that the
/// spellcasting half can be level-aware while this half stays
/// unconditional.
///
/// The old message also asserted this class "has no class-skill list",
/// which is FALSE: `acg_abilities_class.lst`'s own
/// `KEY:Bloodrager ~ Class Skills` record lists 11 skills (Acrobatics,
/// Climb, Craft, Handle Animal, Intimidate, Knowledge (Arcana),
/// Perception, Ride, Spellcraft, Survival, Swim). ACG encodes class
/// skills on a separate internal ability rather than a `CSKILL:` token
/// on the class line -- a `CSKILL:`-shaped search finds nothing for ANY
/// ACG class and would wrongly conclude the list is absent.
/// **Canonical narrowing (v0.6 alpha swarm, Bloodrager
/// spellcasting-shaped closure).** This used to claim-block
/// unconditionally on "the entire Bloodline slot". One bloodline of the
/// ten -- Arcane -- is now genuinely grounded
/// (`ground_bloodrager_arcane_bloodline`), so this takes the same shape
/// `ground_or_block_arcanist_metamagic_knowledge` established: a
/// recognized, grounded bloodline downgrades this to a NON-blocking note
/// naming the other nine and the parts of Arcane's own ladder that stay
/// unmodelled; any other (or no) selection keeps it claim-blocking,
/// unchanged.
pub(super) fn push_bloodrager_other_features_deferred_diagnostic(
    input: &CharacterInput,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let bloodline_recognized = choice_selection(input, BLOODRAGER_BLOODLINE_CHOICE_ID)
        == Some(ARCANE_BLOODRAGER_BLOODLINE_SELECTION);

    let grounded = "its base-attack-bonus/base-save chassis pillar, Bloodrage, its class-skill \
         list, its spells-per-day/spells-known tables, Fast Movement, Uncanny Dodge and \
         Improved Uncanny Dodge, Blood Sanctuary, Damage Reduction, the \
         Greater/Tireless/Mighty Bloodrage tiers, Indomitable Will, Blood Casting and Eschew \
         Materials";

    let remainder = format!(
        // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
        //   `PREABILITY:1,CATEGORY=Archetype,Bloodrager Archetype ~ Crossblooded Rager`
        "the other 9 bloodlines (Aberrant, Abyssal, Celestial, Destined, Draconic, Elemental, Fey, \
         Infernal, Undead) each carry their own separate power ladder and their own separate four \
         bonus spells, and none of them is grounded; the Elemental bloodline's own element \
         sub-choice is not modelled either. Within Arcane itself, four of the six ladder rungs stay \
         unmodelled: Arcane Bloodrage (level {ARCANE_BLOODRAGER_ARCANE_BLOODRAGE_LEVEL}), Greater \
         Arcane Bloodrage ({ARCANE_BLOODRAGER_GREATER_ARCANE_BLOODRAGE_LEVEL}) and True Arcane \
         Bloodrage ({ARCANE_BLOODRAGER_TRUE_ARCANE_BLOODRAGE_LEVEL}) each apply a chosen SPELL's \
         effects to the bloodrager for the bloodrage's duration, and this engine executes no spell \
         effects at all, so grounding them would fabricate a subsystem; Caster's Bane \
         ({ARCANE_BLOODRAGER_CASTERS_BANE_LEVEL}) is a provocation rule with no magnitude, and this \
         codebase has no attack-of-opportunity provocation model for it to be true or false against. \
         Crossblooded Bloodline Selection is excluded deliberately: it is archetype-gated and \
         archetypes are out of scope for base-class chassis, per task #67. No class-feature \
         execution is fabricated in this bounded chassis baseline. This message previously listed \
         Fast Movement, Uncanny Dodge, Blood Sanctuary, Damage Reduction and the Bloodrage tiers as \
         ungrounded -- all five are in fact grounded (tasks #39/#42) -- and then, after task #76 \
         added them, listed Indomitable Will, Blood Casting and Eschew Materials, all three of which \
         are now genuinely grounded by task #83"
    );

    let message = if bloodline_recognized {
        format!(
            "{BLOODRAGER_CLASS_ID} has {grounded}, plus one real, corpus-verified bloodline \
             chosen from its own 10-bloodline chooser (Arcane: Disruptive Bloodrage's +2 \
             defensive-casting DC, Caster's Scourge's extra attack-of-opportunity pool, and all \
             four Arcane bloodline bonus spells at their own 7th/10th/13th/16th grant levels) \
             -- the canonical narrowing this codebase applies to every large class chooser \
             (Cleric's Good domain, Wizard's Evocation school, Oracle's Mystery, Arcanist's \
             Metamagic Knowledge). Bloodrager's bloodlines are PARALLEL to Sorcerer's rather \
             than shared with them (task #59), so this is Bloodrager's own separate corpus work, \
             not reuse. What is deferred, honestly and non-blockingly: {remainder}"
        )
    } else {
        format!(
            "{BLOODRAGER_CLASS_ID} remains blocked beyond {grounded}: the Bloodline slot is the \
             ONE remaining class-feature gap here, and no recognized bloodline choice is \
             present. Exactly one of the ten (Arcane) is grounded in this codebase; {remainder}"
        )
    };

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.bloodrager.other_features_deferred.unsupported".to_owned(),
        message,
        claim_blocking: !bloodline_recognized,
    });
}

/// Grounds Bloodrager's one canonical bloodline, Arcane (v0.6 alpha
/// swarm, Bloodrager spellcasting-shaped closure), gated on a recognized
/// `choice:bloodrager_bloodline` selection.
///
/// Every gate and every magnitude is transcribed directly from
/// `acg_abilities_class.lst`'s own `KEY:Arcane Bloodrager Bloodline ~ ...`
/// family and the base-class `CATEGORY:Bloodrager Bloodline /
/// TYPE:BloodragerBloodlineChoice` record that sets its progression
/// variables (line 618). Two corpus traps were checked and avoided:
///
/// - The bloodline-spell gates 7/10/13/16 are the BASE class's own
///   (`if(Bloodrager_Arcane_BloodlineProgressionLVL>=7)` etc). A second
///   record carrying the same `Bloodrager_Arcane_BloodlineSpellLvl*`
///   variable names uses 7/9/11/13 instead -- that one is
///   `TYPE:EldritchScionBloodragerBloodlineChoice`, the Eldritch Scion
///   ARCHETYPE, which is out of scope for base-class chassis per task
///   #67. Reading it would have shifted three of the four grant levels.
/// - `max(1,DEX)` on Caster's Scourge is PCGen's Dexterity MODIFIER, not
///   the score (`DEXSCORE` is the score token), which matches the ACG's
///   own printed rule.
pub(super) fn ground_bloodrager_arcane_bloodline(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if choice_selection(input, BLOODRAGER_BLOODLINE_CHOICE_ID)
        != Some(ARCANE_BLOODRAGER_BLOODLINE_SELECTION)
    {
        return;
    }

    if level >= ARCANE_BLOODRAGER_DISRUPTIVE_BLOODRAGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.bloodrager.bloodline.arcane.disruptive_bloodrage_dc_increase"
                .to_owned(),
            value: ARCANE_BLOODRAGER_DISRUPTIVE_BLOODRAGE_DC_INCREASE,
            detail: format!(
                "Bloodrager level {level} Arcane bloodline, Disruptive Bloodrage (its \
                 level-{ARCANE_BLOODRAGER_DISRUPTIVE_BLOODRAGE_LEVEL} power): the DC to cast \
                 spells defensively increases by \
                 +{ARCANE_BLOODRAGER_DISRUPTIVE_BLOODRAGE_DC_INCREASE} for enemies within this \
                 bloodrager's threatened area, stacking with the Disruptive feat's own \
                 increase. A magnitude that applies to an OPPONENT's concentration check; this \
                 engine computes no opponent state and no concentration check, so the flat \
                 increase is grounded standalone rather than folded into a total that does not \
                 exist -- the same scope-condition-versus-quantity line Slayer's and \
                 Investigator's own Studied Target/Studied Combat records already draw"
            ),
        });
    }

    if level >= ARCANE_BLOODRAGER_CASTERS_SCOURGE_LEVEL {
        let extra_attacks = ability_modifiers.dexterity.max(1);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.bloodrager.bloodline.arcane.casters_scourge_extra_attacks"
                .to_owned(),
            value: extra_attacks,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|Bloodrager_Arcane_CastersScourge_Times|max(1,DEX)`,
                "Bloodrager level {level} Arcane bloodline, Caster's Scourge (its \
                 level-{ARCANE_BLOODRAGER_CASTERS_SCOURGE_LEVEL} power): a pool of {extra_attacks} \
                 extra attacks of opportunity (` PCGen's DEX being the Dexterity MODIFIER ({}) \
                 rather than the score, so max(1, {}) = {extra_attacks}). Usable only against \
                 spellcasters who cast or attempted to cast defensively in the threatened area, and \
                 still requiring Spellbreaker or the Caster's Bane power to actually attack a \
                 successful defensive caster. This codebase tracks no attack-of-opportunity pool at \
                 all, so the count grounds standalone and no provocation is resolved",
                ability_modifiers.dexterity, ability_modifiers.dexterity
            ),
        });
    }

    ground_bloodrager_arcane_bloodline_bonus_spells(level, explanations);
}

/// The Arcane Bloodrager Bloodline's four bonus spells and their own
/// grant levels, transcribed from `KEY:Arcane Bloodrager Bloodline ~
/// Bonus Spells`'s own `SPELLKNOWN:CLASS|Bloodrager=<n>|<spell>` tokens
/// paired with the base-class gates that enable each one.
pub(super) const ARCANE_BLOODRAGER_BONUS_SPELLS: [(u8, u8, &str); 4] = [
    (7, 1, "Magic Missile"),
    (10, 2, "Invisibility"),
    (13, 3, "Lightning Bolt"),
    (16, 4, "Dimension Door"),
];

/// Grounds the Arcane bloodline's bonus spells known for `level`.
///
/// These are granted IN ADDITION to the class's own Spells Known table
/// and are deliberately not validated against
/// `acg::bloodrager_spell_list`: two of the four (Invisibility, Dimension
/// Door) are genuinely absent from `SPELLLIST:1|Bloodrager`, which is
/// correct rather than a data gap -- a bloodline bonus spell is granted
/// by the bloodline record's own `SPELLKNOWN:` token, not drawn from the
/// class list. Checking them against that list would have wrongly
/// rejected half of them.
pub(super) fn ground_bloodrager_arcane_bloodline_bonus_spells(
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let granted: Vec<String> = ARCANE_BLOODRAGER_BONUS_SPELLS
        .iter()
        .filter(|(grant_level, _, _)| level >= *grant_level)
        .map(|(grant_level, spell_level, name)| {
            format!("{name} (spell level {spell_level}, granted at bloodrager level {grant_level})")
        })
        .collect();

    if granted.is_empty() {
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.bloodrager.bloodline.arcane.bonus_spells_known".to_owned(),
            value: 0,
            detail: format!(
                "Bloodrager level {level} Arcane bloodline bonus spells: none yet. The first is \
                 granted at bloodrager level {}, per the base class's own \
                 `if(Bloodrager_Arcane_BloodlineProgressionLVL>=7)` gate -- correctly absent \
                 here rather than silently omitted",
                ARCANE_BLOODRAGER_BONUS_SPELLS[0].0
            ),
        });
        return;
    }

    explanations.push(ComputationExplanation {
        id: "class_feature.acg.bloodrager.bloodline.arcane.bonus_spells_known".to_owned(),
        value: granted.len() as i16,
        detail: format!(
            "Bloodrager level {level} Arcane bloodline bonus spells known ({}): {}. Each is \
             added to spells known by the bloodline record's own `SPELLKNOWN:CLASS|Bloodrager=\
             <n>|<spell>` token, IN ADDITION to the class Spells Known table's own count -- so \
             these do not consume a Spells Known slot and are not validated against \
             `SPELLLIST:1|Bloodrager` (Invisibility and Dimension Door are genuinely not on \
             that list, which is correct for a bloodline grant). Grounds which spells are known \
             and when; no casting execution and no save DC resolution against a target is \
             claimed for any of them",
            granted.len(),
            granted.join("; ")
        ),
    });
}

/// Bloodrager's spells per day for `level`, indexed by spell level 1-4
/// (task #1, 2026-07-27). `None` below level
/// `BLOODRAGER_FIRST_CASTING_LEVEL`, where the real class block carries
/// no `CAST:` row at all.
///
/// Transcribed directly from `acg_classes.lst`'s own per-level `CAST:`
/// tokens -- these are literal table rows, not formulas, so unlike
/// Oracle's or Arcanist's tables there is nothing to re-derive.
///
/// **The corpus rows carry a LEADING 0-level column that is a genuine
/// zero at every one of the 17 rows** (`CAST:0,1` through
/// `CAST:0,4,4,3,2`): Bloodragers get no 0-level spells at all, which is
/// why `KNOWN:` also starts `0,2` and why the table tops out at 4th
/// level. That column is deliberately NOT represented here. It is NOT
/// Oracle's `CAST:0,3` sentinel, where the leading zero means "orisons
/// known at will with no daily cap" -- carrying that reading across
/// would fabricate at-will cantrips this class never gets.
pub(super) fn bloodrager_spells_per_day(level: u8) -> Option<[i16; 4]> {
    let row: [i16; 4] = match level {
        4..=6 => [1, 0, 0, 0],
        7..=8 => [1, 1, 0, 0],
        9 => [2, 1, 0, 0],
        10..=11 => [2, 1, 1, 0],
        12 => [2, 2, 1, 0],
        13..=14 => [3, 2, 1, 1],
        15 => [3, 2, 2, 1],
        16 => [3, 3, 2, 1],
        17 => [4, 3, 2, 1],
        18 => [4, 3, 2, 2],
        19 => [4, 3, 3, 2],
        20 => [4, 4, 3, 2],
        _ => return None,
    };
    Some(row)
}

/// Bloodrager's spells known for `level`, indexed by spell level 1-4
/// (task #1, 2026-07-27). Same shape, source, and leading-zero caveat as
/// `bloodrager_spells_per_day` -- transcribed from the class block's own
/// `KNOWN:` tokens.
pub(super) fn bloodrager_spells_known(level: u8) -> Option<[i16; 4]> {
    let row: [i16; 4] = match level {
        4 => [2, 0, 0, 0],
        5 => [3, 0, 0, 0],
        6 => [4, 0, 0, 0],
        7 => [4, 2, 0, 0],
        8 => [4, 3, 0, 0],
        9 => [5, 4, 0, 0],
        10 => [5, 4, 2, 0],
        11 => [5, 4, 3, 0],
        12 => [6, 5, 4, 0],
        13 => [6, 5, 4, 2],
        14 => [6, 5, 4, 3],
        15..=17 => [6, 6, 5, 4],
        18..=20 => [6, 6, 6, 5],
        _ => return None,
    };
    Some(row)
}

/// Grounds Bloodrager's spells-per-day and spells-known tables as
/// standalone explanation records (task #1, 2026-07-27), one per spell
/// level that the character actually has slots or known spells for.
/// Grounds nothing below `BLOODRAGER_FIRST_CASTING_LEVEL`.
/// The highest spell level a Bloodrager of `level` can know, or 0 for a
/// non-caster (task #87).
///
/// **Derived from the shipped `bloodrager_spells_known` table rather than
/// transcribed as a second table of its own.** Bard's own
/// `bard_spell_level_access` is a hand-written ladder of level constants,
/// and copying that shape here would create a second source of truth that
/// can silently drift from the table it is supposed to describe. Deriving
/// it makes drift impossible by construction.
///
/// **Bloodragers have NO 0-level spells**, so this table is indexed 1-4,
/// not 0-4 like Bard's/Skald's. The `+ 1` converts a 0-based array index
/// into a 1-based spell level; it is not an off-by-one.
pub(super) fn bloodrager_spell_level_access(level: u8) -> i16 {
    bloodrager_spells_known(level).map_or(0, |known| {
        known.iter().rposition(|count| *count > 0).map_or(0, |index| index as i16 + 1)
    })
}

/// Reports every reason this character's Bloodrager known-spell posture is
/// not yet valid (task #87), mirroring
/// `unmet_skald_known_spell_conditions`'s own shape. An empty list means
/// the posture is genuinely satisfiable and can be grounded for real.
///
/// This is the real consumer of `acg::bloodrager_spell_list`, which
/// carried 200 corpus-verified entries with zero consumers before this.
///
/// Deliberately DIFFERENT from Skald's in one structural way: Skald/Bard
/// index their known-spell arrays from spell level 0 because they get
/// cantrips. Bloodragers never do, so spell level 0 is rejected outright
/// rather than indexed, and every real level maps to `spell_level - 1`.
/// Copying Skald's `known_per_level[spell_level]` verbatim would both
/// shift every cap by one and silently admit a 0-level spell this class
/// cannot have.
pub(super) fn unmet_bloodrager_known_spell_conditions(input: &CharacterInput, level: u8) -> Vec<String> {
    let mut unmet = Vec::new();

    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == BLOODRAGER_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    let Some(known_table) = bloodrager_spells_known(level) else {
        if !known.is_empty() {
            unmet.push(format!(
                "{} known spell(s) recorded at bloodrager level {level}, which has no \
                 spellcasting at all (Bloodragers first cast at level \
                 {BLOODRAGER_FIRST_CASTING_LEVEL})",
                known.len()
            ));
        }
        return unmet;
    };

    let access_ceiling = bloodrager_spell_level_access(level);
    let mut known_per_level = [0i16; 4];

    for spell_id in &known {
        let Some(spell_level) = acg::bloodrager_spell_list::bloodrager_spell_level(spell_id) else {
            unmet.push(format!(
                "known spell '{spell_id}' is not on the real PF1 Bloodrager spell list \
                 (`SPELLLIST:1|Bloodrager`, its own list rather than a borrowed one)"
            ));
            continue;
        };
        if spell_level == 0 {
            unmet.push(format!(
                "known spell '{spell_id}' is a 0-level spell; Bloodragers gain no 0-level \
                 spells at any level"
            ));
            continue;
        }
        if i16::from(spell_level) > access_ceiling {
            unmet.push(format!(
                "known spell '{spell_id}' targets spell level {spell_level}, not yet accessible \
                 at bloodrager level {level} (access ceiling {access_ceiling})"
            ));
            continue;
        }
        known_per_level[usize::from(spell_level) - 1] += 1;
    }

    for (index, count) in known_per_level.iter().enumerate() {
        if *count == 0 {
            continue;
        }
        let cap = known_table[index];
        if *count > cap {
            unmet.push(format!(
                "spell level {} over-known: {count} distinct spells known but only {cap} slots \
                 available on the Bloodrager Spells Known table at level {level}",
                index + 1
            ));
        }
    }

    unmet
}

/// Grounds the real known-spell posture, or claim-blocks it with the
/// specific unmet reasons (task #87). Mirrors
/// `ground_or_block_skald_known_spells`'s own two-branch shape.
///
/// Called from all three of `ground_or_block_bloodrager_bloodrage`'s exit
/// paths (not-raging, invalid activation, and the active/rounds-exceeded
/// tail) so the posture is evaluated identically regardless of bloodrage
/// state -- which spells a bloodrager knows does not depend on whether he
/// is currently raging.
pub(super) fn ground_or_block_bloodrager_known_spells(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if level < BLOODRAGER_FIRST_CASTING_LEVEL {
        return;
    }
    let unmet = unmet_bloodrager_known_spell_conditions(input, level);
    if unmet.is_empty() {
        ground_bloodrager_known_spells(input, level, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.acg.bloodrager.spontaneous_known_and_per_day.unsupported".to_owned(),
            message: format!(
                "{BLOODRAGER_CLASS_ID} known-spell posture is not satisfied: {}",
                unmet.join("; ")
            ),
            claim_blocking: true,
        });
    }
}

/// Grounds the real Bloodrager known-spell posture once
/// `unmet_bloodrager_known_spell_conditions` reports an empty unmet list
/// (task #87), mirroring `ground_skald_known_spells`.
pub(super) fn ground_bloodrager_known_spells(
    input: &CharacterInput,
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == BLOODRAGER_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.acg.bloodrager.known_spells".to_owned(),
        value: known.len() as i16,
        detail: format!(
            "Bloodrager level {level} known-spell selection ({} spells, \
             AcquisitionMode::Known): {}. Each known spell is verified against the real PF1 \
             Bloodrager spell list (`acg::bloodrager_spell_list`, 200 corpus-verified entries \
             extracted from its own `SPELLLIST:1|Bloodrager` token -- its own list, not a \
             borrowed one), against the access ceiling derived from the shipped Spells Known \
             table, and against that table's own per-level caps. Real PF1 Bloodrager rules have \
             no daily preparation step (spontaneous) -- known spells are permanent once learned \
             and cast using the already-grounded per-day slot totals. This grounds the \
             known-spell SELECTION for real; it computes no spell save DC resolution against a \
             target and no casting execution, and the Bloodline bonus spells remain deferred",
            known.len(),
            if known.is_empty() { "none".to_owned() } else { known.join(", ") }
        ),
    });
}

pub(super) fn ground_bloodrager_spell_tables(
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let (Some(per_day), Some(known)) =
        (bloodrager_spells_per_day(level), bloodrager_spells_known(level))
    else {
        return;
    };
    for (index, (slots, known_count)) in per_day.iter().zip(known.iter()).enumerate() {
        let spell_level = index + 1;
        if *slots == 0 && *known_count == 0 {
            continue;
        }
        explanations.push(ComputationExplanation {
            id: format!("class_spell.acg.bloodrager.spells_per_day.spell_level_{spell_level}"),
            value: *slots,
            detail: format!(
                "Bloodrager level {level} spells per day at spell level {spell_level}: {slots}, \
                 read directly from the PF1 Advanced Class Guide Bloodrager class block's own \
                 per-level CAST: token. Bloodragers gain no 0-level spells at any level, so no \
                 cantrip row exists. Charisma bonus spells are not folded in here"
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!("class_spell.acg.bloodrager.spells_known.spell_level_{spell_level}"),
            value: *known_count,
            detail: format!(
                "Bloodrager level {level} spells known at spell level {spell_level}: \
                 {known_count}, read directly from the class block's own KNOWN: token. This is \
                 the SLOT COUNT; which specific spells are known is grounded separately by \
                 `class_spell.acg.bloodrager.known_spells`, validated against the real \
                 200-entry `acg::bloodrager_spell_list` (task #87). This detail previously said \
                 that list was \"not built\" and called it \"183-entry\": both were stale -- the \
                 list has been built since task #1, and 183 was its pre-correction count from \
                 before its own generation bug (short by 17) was fixed"
            ),
        });
    }
}

/// Applies Bloodrager Bloodrage's Strength/Constitution morale bonus to
/// `base` when a valid, active, in-budget Bloodrage activation is present
/// for this character (v0.6 alpha swarm, risks item 8, second APG/ACG
/// closure). Mirrors `apply_rage_ability_bonuses`/
/// `apply_skald_inspired_rage_ability_bonuses` exactly -- chained
/// immediately after Skald's in `compute_pilot_base_chassis`;
/// class-ownership-gated by construction via
/// `active_bloodrager_bloodrage_bonus`.
///
/// The Strength/Constitution ability-SCORE bonus (+4/+6/+8, always even)
/// becomes exactly half that as an ability-MODIFIER bonus (+2/+3/+4):
/// modifier = floor((score - 10) / 2), and adding an even N to score adds
/// exactly N/2 to the floored modifier regardless of the original score's
/// parity (identical reasoning to `apply_rage_ability_bonuses`).
pub(super) fn apply_bloodrager_bloodrage_ability_bonuses(
    base: AbilityModifiers,
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) -> AbilityModifiers {
    let Some((bloodrager_level, strength_constitution_bonus, will_save_bonus)) =
        active_bloodrager_bloodrage_bonus(input, &base)
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
        id: "ability_modifier.bloodrager.bloodrage_bonus_applied".to_owned(),
        value: ability_modifier_bonus,
        detail: format!(
            "Bloodrager level {bloodrager_level} Bloodrage applied to ability modifiers: \
             +{strength_constitution_bonus} Strength / +{strength_constitution_bonus} \
             Constitution morale bonus (ability score) is +{ability_modifier_bonus} Strength \
             modifier / +{ability_modifier_bonus} Constitution modifier (an even score bonus \
             always halves exactly onto the floored modifier). Applied only while actively, \
             validly bloodraging; the Will-save bonus (+{will_save_bonus}) is layered onto \
             compute_total_saves separately, and the \
             {BLOODRAGER_BLOODRAGE_ARMOR_CLASS_PENALTY} Armor Class penalty onto \
             compute_combat_baseline separately"
        ),
    });
    boosted
}

/// Grounds the Unchained Barbarian's named features
/// (`rules_tables::pathfinder_unchained::barbarian_features`).
///
/// Every magnitude below is that module's own pure function, called with
/// this character's real level and ability modifiers. Nothing is recomputed
/// here, and nothing is emitted for a feature the character has not reached
/// — each function returns `None` below its grant level and this function
/// simply pushes no record in that case, which is the same "absent means
/// not yet granted" contract the ACG/APG groundings use.
pub(super) fn ground_unchained_barbarian_class_features(
    level: u8,
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if let Some(rounds) = barbarian_features::rage_rounds_per_day(level, ability_modifiers.constitution) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.rage_rounds_per_day".to_owned(),
            value: rounds,
            detail: format!(
                "Unchained Barbarian level {level} Rage: {rounds} rounds per day \
                 (2 + Constitution modifier {} + 2 x level)",
                ability_modifiers.constitution
            ),
        });
    }
    if let Some(bonus) = barbarian_features::rage_morale_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.rage_morale_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Unchained Barbarian level {level} Rage: a +{bonus} morale bonus on melee attack \
                 rolls, melee and thrown damage rolls, and Will saves while raging. This is the \
                 sharpest divergence from the Core Rulebook Barbarian, which instead raises \
                 Strength and Constitution by 4 and Will by 2"
            ),
        });
    }
    if let Some(penalty) = barbarian_features::rage_armor_class_penalty(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.rage_armor_class_penalty".to_owned(),
            value: penalty,
            detail: format!(
                "Unchained Barbarian level {level} Rage: a {penalty} penalty to Armor Class while \
                 raging. Grounded as a standalone magnitude -- rage is an activated state and no \
                 activation model exists here, so it is deliberately NOT folded into the \
                 character's resting Armor Class total"
            ),
        });
    }
    // Single-class only (`compute_class_chassis` routes multiclass away
    // before this function can be reached), so character level == class
    // level here. The two arguments stay distinct anyway, because the
    // corpus makes the multiplicand total level and the multiplier class
    // level, and collapsing them would be wrong the day multiclass lands.
    if let Some(temp_hp) = barbarian_features::rage_temporary_hit_points(level, level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.rage_temporary_hit_points".to_owned(),
            value: temp_hp,
            detail: format!(
                "Unchained Barbarian level {level} Rage: {temp_hp} temporary hit points while \
                 raging (character level x 2, rising to x3 at level 11 and x4 at level 20). A \
                 standalone magnitude: this engine tracks max and current hit points but has no \
                 temporary-hit-point total to add it to, so nothing consumes it yet"
            ),
        });
    }
    if let Some(powers) = barbarian_features::rage_powers_known(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.rage_powers_known".to_owned(),
            value: powers,
            detail: format!(
                "Unchained Barbarian level {level} Rage Powers: a pool of {powers} (level / 2). \
                 The 54 Unchained Rage Powers this pool is spent on are not modelled anywhere in \
                 this repo, so this is the size of the pool and not a claim that a catalogue of \
                 choices exists"
            ),
        });
    }
    if let Some(bonus) = barbarian_features::danger_sense_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.danger_sense_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Unchained Barbarian level {level} Danger Sense: +{bonus} (level / 3) on Reflex \
                 saves to avoid traps and on Perception checks to notice them"
            ),
        });
    }
    if let Some(dr) = barbarian_features::damage_reduction(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.damage_reduction".to_owned(),
            value: dr,
            detail: format!(
                "Unchained Barbarian level {level} Damage Reduction: {dr}/- ((level - 4) / 3)"
            ),
        });
    }
    if let Some(feet) = barbarian_features::fast_movement_bonus_feet(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.fast_movement_bonus_feet".to_owned(),
            value: feet,
            detail: format!(
                "Unchained Barbarian level {level} Fast Movement: +{feet} feet to base land speed \
                 when carrying no more than a medium load and wearing no heavy armor. The \
                 load/armor condition is stated, not silently applied -- this engine has no \
                 encumbrance model, so the magnitude is grounded and the condition is not"
            ),
        });
    }
    if let Some(bonus) = barbarian_features::indomitable_will_save_bonus(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.indomitable_will_save_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Unchained Barbarian level {level} Indomitable Will: +{bonus} on Will saves \
                 against enchantment spells and effects while raging. Conditional on raging, so \
                 it is NOT added to the character's resting Will save total"
            ),
        });
    }
    if let Some(flanking_level) = barbarian_features::uncanny_dodge_flanking_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.uncanny_dodge_flanking_level".to_owned(),
            value: flanking_level,
            detail: format!(
                "Unchained Barbarian level {level} Uncanny Dodge: counts as a level-{flanking_level} \
                 defender for the rule that only a rogue of at least four levels higher can flank \
                 them"
            ),
        });
    }
    let tier = barbarian_features::uncanny_dodge_tier(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.pu.unchained_barbarian.uncanny_dodge_tier".to_owned(),
        value: i16::from(tier),
        detail: format!(
            "Unchained Barbarian level {level} Uncanny Dodge tier {tier}: 0 below level 2, 1 at \
             levels 2-4 (Uncanny Dodge), 2 from level 5 (Improved Uncanny Dodge)"
        ),
    });
    // Greater Rage and Mighty Rage are their own ingested records
    // (`:294` / `:296`) and each carries two real formula tokens, but until
    // now their `+1`s vanished into the Rage totals above and the rows a
    // player reads under those two names carried no number at all. These
    // state the value each record is responsible for producing.
    if let Some(bonus) = barbarian_features::greater_rage_morale_bonus(level) {
        let multiplier = barbarian_features::rage_temporary_hit_point_multiplier(level)
            .expect("Rage is granted wherever Greater Rage is");
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.greater_rage_morale_bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|RageBonus|1 and BONUS:VAR|RageBonusHP|TL from level 11
                "Unchained Barbarian level {level} Greater Rage: the rage morale bonus is +{bonus} \
                 and rage temporary hit points are character level x {multiplier}. Greater Rage's \
                 own row adds one to each; the values shown are the resulting totals, which is what \
                 the character actually has"
            ),
        });
    }
    if let Some(bonus) = barbarian_features::mighty_rage_morale_bonus(level) {
        let multiplier = barbarian_features::rage_temporary_hit_point_multiplier(level)
            .expect("Rage is granted wherever Mighty Rage is");
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.mighty_rage_morale_bonus".to_owned(),
            value: bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|RageBonus|1, BONUS:VAR|RageBonusHP|TL
                "Unchained Barbarian level {level} Mighty Rage: the rage morale bonus reaches \
                 +{bonus} and rage temporary hit points character level x {multiplier}. Mighty \
                 Rage's row carries the identical pair of tokens as Greater Rage, stacking a second \
                 time from level 20"
            ),
        });
    }
    if let Some(rounds) =
        barbarian_features::prose_derived::tireless_rage_temporary_hit_point_lockout_rounds(level)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.tireless_rage_lockout_rounds".to_owned(),
            value: rounds,
            detail: format!(
                "Unchained Barbarian level {level} Tireless Rage: you are no longer fatigued when \
                 a rage ends, but raging again within {rounds} rounds (1 minute) of the last rage \
                 ending grants no temporary hit points. The row carries no formula token at all -- \
                 the 1 minute is read out of its own DESC and converted to the rounds every other \
                 rage magnitude here is measured in"
            ),
        });
    }

    // SD31-E4-F2-004: Unchained Barbarian's OWN Rage Power chooser consumer
    // -- a genuinely separate wiring from the base class's
    // (`barbarian_selected_rage_power`/`UNCHAINED_BARBARIAN_RAGE_POWER_SLOTS`
    // is its own 10-slot family, never the base's), per `decisions.md §10`'s
    // AMENDMENT that Unchained classes must resolve as their own class, not
    // fold into the base. Superstition is the representative here too, for
    // the same reason it is the base class's: `KEY:Unchained Rage Power ~
    // Superstition`'s own `BONUS:VAR|SuperstitionSaveBonus|2+floor(
    // RagePowersLVL/4)` token (`pu_abilities_class.lst:389`) is
    // byte-identical in shape to the base's, carries no `Raging`-state gate
    // either, and its own `RagePowersLVL` is
    // `BONUS:VAR|RagePowersLVL|BarbarianLVL` on the Unchained Barbarian's own
    // internal `Rage Powers` record (`pu_abilities_class.lst:291`) -- the
    // SAME chain the base class's formula rests on, so
    // `barbarian_superstition_save_bonus` (a pure function of `level` alone)
    // is reused rather than duplicated: this class's own `level` parameter
    // IS that chain's `BarbarianLVL` for a character built through this
    // class's own chassis.
    if unchained_barbarian_selected_rage_power(
        input,
        level,
        UNCHAINED_SUPERSTITION_RAGE_POWER_SELECTION,
    ) {
        let superstition_bonus = barbarian_superstition_save_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.pu.unchained_barbarian.rage_power.superstition.save_bonus"
                .to_owned(),
            value: superstition_bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   , `BONUS:VAR|SuperstitionSaveBonus|2+floor(RagePowersLVL/4)`, RagePowersLVL =
                //   BarbarianLVL = this class's own level
                "Unchained Barbarian level {level} with the Superstition rage power selected \
                 (Pathfinder Unchained, `KEY:Unchained Rage Power ~ Superstition`): a \
                 +{superstition_bonus} competence bonus on saving throws made to resist spells, \
                 supernatural abilities, and spell-like abilities (base \
                 {SUPERSTITION_SAVE_BONUS_BASE}, +1 every {SUPERSTITION_SAVE_BONUS_LEVEL_DIVISOR} \
                 levels), validated against the real, corpus-verified 54-member Unchained Rage Power \
                 pool so an invented or drifted id can never ground it. Standalone magnitude only, \
                 matching the base class's own Superstition closure's posture -- not yet integrated \
                 into a total saves field. The other 53 Unchained Rage Powers remain \
                 named-but-unproven -- this is a representative-pool closure, not a claim that the \
                 family is exhausted"
            ),
        });
    }

    push_deferred_class_features(
        "class_feature.pu.unchained_barbarian.other_features_deferred.unsupported",
            "class:unchained_barbarian grounds every Unchained Barbarian magnitude this book \
             states as a formula token: the chassis (borrowed unchanged from the Core Rulebook \
             Barbarian, which the corpus record confirms it does not override), Rage's rounds \
             per day, morale bonus, Armor Class penalty and temporary hit points, the Rage Power \
             pool size, Danger Sense, Damage Reduction, Fast Movement, Indomitable Will, the \
             Uncanny Dodge flanking level and tier, the morale bonus and temporary-hit-point \
             multiplier Greater Rage and Mighty Rage each produce, Tireless Rage's \
             temporary-hit-point lockout, and one real, corpus-verified representative Rage \
             Power (Superstition) of its own separate 54-member pool. This diagnostic is NOT \
             claim-blocking; it carries the honest remainder. What is missing: (1) the other 53 \
             Unchained Rage Powers -- the pool size is real, Superstition is now wired, the rest \
             of the catalogue is not; (2) APPLICATION rather than magnitude -- Rage is an \
             activated state with no activation model here, so the morale bonus, Armor Class \
             penalty, temporary hit points and Indomitable Will are derived correctly but \
             deliberately not folded into any resting total; (3) Fast Movement's \
             encumbrance/heavy-armor condition, which this engine cannot evaluate; (4) Tireless \
             Rage's other clause -- no longer being fatigued when a rage ends -- removes a \
             condition this engine does not track, so it carries no magnitude; and Weapon and \
             Armor Proficiency is a proficiency-lane fact this engine models per-item, not \
             per-class"
            .to_owned(),
        explanations,
        diagnostics,
    );
}

/// The bounded Barbarian milestone level this decomposition surface grounds, if any.
/// Returns the single Barbarian level when the chosen input is exactly a
/// single-class Barbarian at one of the supported milestone levels (1 through
/// 10). Returns `None` for no Barbarian, a non-Barbarian class, a multiclass mix,
/// or any level-11+ Barbarian this slice deliberately does not recognize — each of which
/// stays claim-blocked exactly as before. Mirrors the Fighter `supported_fighter_level`
/// / Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Monk
/// `supported_monk_level` level-range gate idiom.
pub(super) fn supported_barbarian_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == BARBARIAN_CLASS_ID
                && (1..=MAX_SUPPORTED_BARBARIAN_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E3/E5 runtime evidence for the deterministic Human Barbarian
/// level-1/level-2/level-3/level-4 martial chassis. Base-attack progression, base-save
/// progression, and the fast-movement speed-extension value are grounded directly at
/// every supported level. The SD13-E5 slice resolves the formerly-named illiteracy
/// burden as vacuous — the PF1 Core Rulebook Barbarian is NOT illiterate; illiteracy
/// is a D&D 3.5e Barbarian trait that never existed in PF1, so under the fixture's
/// `pf1.core_rulebook` source package there was never anything to implement — and
/// grounds Rage's flat numeric surface: rage rounds per day (4 + Constitution modifier,
/// growing by 2 more rounds per level after 1st, claim-blocked instead of grounded
/// when that sum is non-positive) and the flat while-raging constants (a morale
/// bonus to Strength, a morale bonus to Constitution, a morale bonus on Will saves,
/// and an armor class penalty, unchanged by level), values only. A later SD13-E5
/// slice widens the level-1-only gate (`martial_level1_class`) to a level-range gate
/// (`supported_barbarian_level`, 1..=2), mirroring the Fighter/Paladin/Rogue
/// level-range-gate idiom, and a further SD13-E5 slice grounds Uncanny Dodge, the
/// PF1 Core Rulebook Barbarian's 2nd-level "Special" class table entry (verified
/// independently against d20pfsrd and legacy.aonprd.com, both naming "Rage power,
/// uncanny dodge" as the level-2 row), as a bounded identity/recognition record only
/// (`class_feature.barbarian.uncanny_dodge`, value 0) — a level-gate-absence record
/// below level 2, a granted-but-unexecuted rule-text recognition record at or above
/// it, mirroring exactly how Rogue's/Monk's own Evasion and Druid's Woodland Stride
/// were grounded, with no flat-footed-state tracking, no Armor Class computation, and
/// no invisibility-detection engine implemented. The level-2 row's other named entry,
/// a Rage Power choice (a genuinely open-ended choice-list feature), is deliberately
/// left named-but-unproven, mirroring the Monk level-2 bonus feat grant / Bard
/// Versatile Performance precedent. A still further SD13-E5 slice widens the gate to
/// level 3 (`MAX_SUPPORTED_BARBARIAN_LEVEL = 3`, mirroring the Rogue/Monk level-3
/// widening idiom) and grounds Trap Sense, the PF1 Core Rulebook Barbarian's 3rd-level
/// "Special" class table entry (verified independently against d20pfsrd and
/// legacy.aonprd.com, both naming "Trap sense +1" as the level-3 row), as a bounded
/// flat-magnitude record only (`class_feature.barbarian.trap_sense`, barbarian level /
/// 3, floor; +1 at level 3) — a level-gate-absence record below level 3, a
/// flat-magnitude recognition record at or above it, mirroring exactly how Rogue's own
/// Trap Sense was grounded, never applied to any actual Reflex-save total or Armor
/// Class total. A still further SD13-E5 slice widens the gate to level 4
/// (`MAX_SUPPORTED_BARBARIAN_LEVEL = 4`, mirroring the Rogue/Monk level-4 widening
/// idiom, verified independently against d20pfsrd and legacy.aonprd.com: the level-4
/// row is BAB +4, Fort +4, Ref +1, Will +1, Special "Rage power"): base-attack
/// (classlevel = 4), base-save (Fortitude +4, Reflex +1, Will +1), fast movement
/// (unchanged flat +10 ft.), and rage rounds per day (4 + Constitution modifier + 2 *
/// (level - 1), 13 on the Con 16 fixture at level 4) are extended to level 4 via the
/// same formulas, and Uncanny Dodge and Trap Sense both stay granted (not re-derived;
/// Trap Sense stays at the same +1 magnitude, since the PF1 Core Rulebook bonus does
/// not rise again until barbarian level 6). The level-4 row's only named "Special"
/// entry is another Rage Power grant — the same genuinely open-ended choice-list
/// feature already deliberately left named-but-unproven at level 2, not a new type of
/// class feature — so this widening grounds no new pillar beyond the arithmetic
/// extension above. A still further SD13-E5 slice widens the gate to level 5
/// (`MAX_SUPPORTED_BARBARIAN_LEVEL = 5`, mirroring the Rogue/Monk level-5 widening
/// idiom, verified independently against d20pfsrd and legacy.aonprd.com: the level-5
/// row is BAB +5, Fort +4, Ref +1, Will +1, Special "Improved uncanny dodge"):
/// base-attack (classlevel = 5), base-save (Fortitude +4, Reflex +1, Will +1), fast
/// movement (unchanged flat +10 ft.), and rage rounds per day (4 + Constitution
/// modifier + 2 * (level - 1), 15 on the Con 16 fixture at level 5) are extended to
/// level 5 via the same formulas, and Uncanny Dodge and Trap Sense both stay granted
/// (not re-derived; Trap Sense stays at the same +1 magnitude, since the PF1 Core
/// Rulebook bonus does not rise again until barbarian level 6). The level-5 row's
/// "Special" entry, Improved Uncanny Dodge (verified independently against d20pfsrd
/// and legacy.aonprd.com: "At 5th level and higher, a barbarian can no longer be
/// flanked. This defense denies a rogue the ability to sneak attack the barbarian by
/// flanking her, unless the attacker has at least four more rogue levels than the
/// target has barbarian levels."), IS a genuinely new class feature, not another Rage
/// Power grant — and its own grant is flat/identity-shaped exactly like Uncanny
/// Dodge's own record, so it is newly grounded as a bounded identity/recognition
/// record only (`class_feature.barbarian.improved_uncanny_dodge`, value 0): a
/// level-gate absence below level 5, a granted-but-unexecuted rule-text recognition
/// record at or above it. The rule's own CONDITIONAL piece — comparing the attacking
/// rogue's own levels against the barbarian's own levels to decide whether the
/// immunity is actually pierced — is not computed: no flanking-resolution engine, no
/// attacker-level-comparison engine, and no sneak-attack-trigger engine exists
/// anywhere in this codebase, so this slice grounds only the bounded grant, mirroring
/// exactly how Uncanny Dodge itself was grounded. A still further SD13-E5 slice widens
/// the gate to level 6 (`MAX_SUPPORTED_BARBARIAN_LEVEL = 6`, mirroring the Rogue's own
/// level-6 widening idiom, verified independently against d20pfsrd and
/// legacy.aonprd.com: the level-6 row is BAB +6, Fort +5, Ref +2, Will +2, Special
/// "Rage power, trap sense +2"): base-attack (classlevel = 6), base-save (Fortitude
/// +5, Reflex +2, Will +2), fast movement (unchanged flat +10 ft.), and rage rounds
/// per day (4 + Constitution modifier + 2 * (level - 1), 17 on the Con 16 fixture at
/// level 6) are extended to level 6 via the same formulas, and Uncanny Dodge and
/// Improved Uncanny Dodge both stay granted (not re-derived). Trap Sense's own flat
/// magnitude GENUINELY RISES at level 6 (barbarian level / 3, floor: `6 / 3 = 2`, up
/// from `1` at levels 3-5) via the same pre-existing formula, matching the class
/// table's own "trap sense +2" entry exactly — this is a value change, not a new
/// record, mirroring exactly how Rogue's own level-6 Trap Sense rise was grounded.
/// The level-6 row's other named "Special" entry is another Rage Power grant — the
/// same genuinely open-ended choice-list feature already deliberately left
/// named-but-unproven at levels 2 and 4, not a new type of class feature — so this
/// widening grounds no new pillar beyond the arithmetic extension and the Trap Sense
/// magnitude rise above. A still further SD13-E5 slice widens the gate to level 7
/// (`MAX_SUPPORTED_BARBARIAN_LEVEL = 7`, mirroring the Rogue's own level-7 widening
/// idiom, verified independently against d20pfsrd and legacy.aonprd.com: the level-7
/// row is BAB +7, Fort +5, Ref +2, Will +2, Special "Damage reduction 1/-"):
/// base-attack (classlevel = 7), base-save (Fortitude +5, Reflex +2, Will +2), fast
/// movement (unchanged flat +10 ft.), and rage rounds per day (4 + Constitution
/// modifier + 2 * (level - 1), 19 on the Con 16 fixture at level 7) are extended to
/// level 7 via the same formulas, and Uncanny Dodge, Trap Sense, and Improved Uncanny
/// Dodge all stay granted (not re-derived; Trap Sense stays at the same +2 magnitude,
/// since the PF1 Core Rulebook bonus does not rise again until barbarian level 9). The
/// level-7 row's "Special" entry, Damage Reduction 1/- (verified independently against
/// d20pfsrd and legacy.aonprd.com: "at 7th level, a barbarian gains damage reduction.
/// Subtract 1 from the damage the barbarian takes each time she is dealt damage from a
/// weapon or a natural attack"), IS a genuinely new class feature, NOT another Rage
/// Power grant — both primary sources confirm Rage Powers are granted at 2nd, 4th,
/// 6th, 8th, and 10th barbarian level, not 7th, so there is no new Rage Power grant to
/// leave named-but-unproven at this level and no rage-power-selection-slot-count
/// engine is invented. Damage Reduction's own flat magnitude (1 point) is
/// flat/identity-shaped exactly like Trap Sense's own magnitude, so it is newly
/// grounded as a bounded flat-magnitude record only
/// (`class_feature.barbarian.damage_reduction`, value 1 at or above level 7, value 0
/// below it): the rule's own APPLICATION piece (subtracting the value from incoming
/// weapon/natural-attack damage) is not computed, since no damage-resolution engine
/// and no incoming-damage total exists anywhere in this codebase. A still further
/// SD13-E5 slice widens the gate to level 8 (`MAX_SUPPORTED_BARBARIAN_LEVEL = 8`,
/// mirroring the Rogue's/Monk's own level-8 widening idiom, verified independently
/// against d20pfsrd and legacy.aonprd.com: the level-8 row is BAB +8, Fort +6, Ref +2,
/// Will +2, Special "Rage power" only): base-attack (classlevel = 8), base-save
/// (Fortitude +6, Reflex +2, Will +2), fast movement (unchanged flat +10 ft.), and
/// rage rounds per day (4 + Constitution modifier + 2 * (level - 1), 21 on the Con 16
/// fixture at level 8) are extended to level 8 via the same formulas, and Uncanny
/// Dodge, Trap Sense, Improved Uncanny Dodge, and Damage Reduction all stay granted
/// (not re-derived; Trap Sense stays at the same +2 magnitude, since the PF1 Core
/// Rulebook bonus does not rise again until barbarian level 9, and Damage Reduction
/// stays at the same 1-point magnitude, since it does not rise again until barbarian
/// level 10). The level-8 row's "Special" entry is another Rage Power grant — both
/// primary sources confirm Rage Powers are granted at 2nd, 4th, 6th, 8th, and 10th
/// barbarian level, so this is the SAME genuinely open-ended choice-list feature
/// already deliberately left named-but-unproven at levels 2, 4, and 6, not a new type
/// of class feature — so this widening grounds no new pillar beyond the arithmetic
/// extension above and no rage-power-selection-slot-count engine is invented.
/// Otherwise only the rage-state execution burden, the Rage Power choice-list
/// feature, weapon familiarity, the Improved Uncanny Dodge flanking-resolution
/// engine, and the Damage Reduction application engine stay explicitly claim-blocked.
///
/// This deliberately does not compute a supported martial chassis: the grounded
/// base-attack, base-save, fast-movement, rage, Uncanny Dodge, Trap Sense, Improved
/// Uncanny Dodge, and Damage Reduction explanation records below are standalone (not
/// wired into `PilotBaseChassisComputation.base_attack_bonus`, `compute_total_saves`,
/// `compute_combat_baseline`, the integrated ability modifiers, or any
/// speed/movement/flat-footed/Armor-Class/incoming-damage total), so the integrated
/// pilot surface still reports a blocked posture on this input. It grounds no
/// rage-state engine, no weapon familiarity, no Rage Power choice-list feature, no
/// flat-footed-state tracking, no Armor Class computation, no invisibility-detection
/// engine, no flanking-resolution engine, no damage-reduction-resolution engine, and
/// no level-9+ martial progression. It only:
/// - leaves one chassis-recognition explanation so the `class:barbarian:N` identity
///   (at the supported level, 1, 2, 3, 4, 5, 6, 7, or 8) is acknowledged as a non-hybrid
///   martial baseline rather than an undocumented packet placeholder (direct runtime
///   evidence, carrying no fabricated mechanical value),
/// - leaves five grounded explanation records naming the full-BAB base-attack
///   bonus, the good-Fortitude/poor-Reflex/poor-Will base saves, and the flat
///   +10 ft. fast-movement value,
/// - leaves one grounded rules-correction record documenting that the illiteracy
///   burden was vacuous (`class_chassis.barbarian.illiteracy_absent`, +0),
/// - leaves up to five grounded rage explanation records naming rage rounds per day
///   (4 + Constitution modifier, omitted in favor of a claim-blocking diagnostic when
///   that sum is non-positive) and the four flat rage constants, values only,
/// - leaves one grounded Uncanny Dodge identity/recognition record (level-gate
///   absence below level 2, granted-but-unexecuted rule-text recognition at or
///   above it, value 0 either way),
/// - leaves one grounded Trap Sense flat-magnitude record (level-gate absence below
///   level 3, value 0; flat magnitude at or above it, barbarian level / 3),
/// - leaves one grounded Improved Uncanny Dodge identity/recognition record
///   (level-gate absence below level 5, granted-but-unexecuted rule-text recognition
///   at or above it, value 0 either way),
/// - leaves one grounded Damage Reduction flat-magnitude record (level-gate absence
///   below level 7, value 0; flat magnitude of 1 at or above it, never applied to any
///   incoming-damage total), and
/// - emits one claim-blocking diagnostic naming the still-missing rage-state
///   execution engine explicitly (activation/deactivation, round-by-round rage
///   round consumption, fatigue after rage, and temporary application of the rage
///   constants to computed totals), rather than hiding behind a single generic
///   "unsupported class" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Barbarian martial identity, its grounded
/// pillar values, and its remaining named pillar burden legible on the runtime path.
///
/// **v0.6 alpha swarm, risks item 8 update**: the rage-state execution engine
/// this doc comment describes as permanently missing is now real (see
/// `ground_or_block_barbarian_rage`, called at the top of this function,
/// checked regardless of race/single-class status -- mirroring the Ranger/
/// Paladin/Sorcerer/Cleric/Druid gate-ordering fix exactly, since
/// `table_class_id` now recognizes Barbarian generically too). Everything
/// else this doc comment says about the OTHER named-but-unexecuted features
/// (Rage Power choice-list, weapon familiarity, Improved Uncanny Dodge
/// flanking, Damage Reduction application) is still accurate: only the
/// rage-execution burden itself became real.
pub(super) fn explain_barbarian_level1_chassis(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // v0.6 alpha swarm, risks item 8: validated regardless of whether
    // Barbarian appears alone or in a multiclass mix, and regardless of
    // race -- checked BEFORE the single-class-only/Human gate below,
    // mirroring the Ranger/Paladin/Sorcerer/Cleric/Druid fix exactly (a
    // false-Computed/false-grounding risk now that `table_class_id`
    // recognizes Barbarian generically).
    if let Some(barbarian_level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == BARBARIAN_CLASS_ID)
        .map(|class_level| class_level.level)
    {
        ground_or_block_barbarian_rage(
            input,
            barbarian_level,
            ability_modifiers,
            explanations,
            diagnostics,
        );
    }

    let Some(level) = supported_barbarian_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    let class_id = BARBARIAN_CLASS_ID;
    let class_name = "Barbarian";
    let chassis_id = "class_chassis.barbarian.bounded_progression";
    let level_value = i16::from(level);

    // Direct runtime evidence: recognize the deterministic Human Barbarian chassis
    // identity at the supported level. This is a recognition record only; it
    // fabricates no mechanical value.
    explanations.push(ComputationExplanation {
        id: chassis_id.to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human {class_name} level {level} martial chassis: \
             the {class_id}:{level} class identity is acknowledged as a pure non-hybrid \
             martial baseline on the rules-core seam rather than an undocumented packet placeholder. This \
             is a bounded chassis-recognition record only; it grounds no rage-state execution engine, no \
             weapon familiarity, and no level-5+ martial progression, so it carries no fabricated \
             mechanical value (+0). The base-attack, base-save, fast-movement, and flat rage pillar \
             values are grounded separately as standalone explanation records"
        ),
    });

    // Grounded (1/3): full-BAB base-attack progression, same formula shape as
    // Fighter's cr_classes.lst:139 BONUS:COMBAT|BASEAB|classlevel. No PCGen .lst
    // file exists for the Barbarian class in this repo, so this cites the PF1 Core
    // Rulebook Barbarian class table directly.
    let base_attack_bonus = level_value;
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:COMBAT|BASEAB|classlevel
            "{class_name} level {level} base attack bonus from the PF1 Core Rulebook Barbarian class \
             table (full base-attack progression, same formula shape as Fighter's \
             cr_classes.lst:139): classlevel = {base_attack_bonus}. This is a standalone explanation \
             record; it is not wired into the integrated base_attack_bonus field or into \
             compute_combat_baseline"
        ),
    });

    // Grounded (2/3): base-save progression — good Fortitude, poor Reflex, poor
    // Will, same formula shape as Fighter's cr_classes.lst:139 base-save cadence.
    // Extended to every supported level via the same formulas, not re-derived.
    let fortitude_save = level_value / 2 + 2;
    let reflex_save = level_value / 3;
    let will_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.base_save.fortitude".to_owned(),
        value: fortitude_save,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   cr_classes.lst:139 BONUS:SAVE|BASE.Fortitude|classlevel/2+2:
            "{class_name} level {level} base Fortitude save (good save) from the PF1 Core Rulebook \
             Barbarian class table, same formula shape as Fighter's classlevel/2+2 = \
             {fortitude_save}. This is a standalone explanation record; it is not wired into \
             compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.base_save.reflex".to_owned(),
        value: reflex_save,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   cr_classes.lst:139 BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3:
            "{class_name} level {level} base Reflex save (poor save) from the PF1 Core Rulebook \
             Barbarian class table, same formula shape as Fighter's classlevel/3 = {reflex_save}. \
             This is a standalone explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.base_save.will".to_owned(),
        value: will_save,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   cr_classes.lst:139 BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3:
            "{class_name} level {level} base Will save (poor save) from the PF1 Core Rulebook \
             Barbarian class table, same formula shape as Fighter's classlevel/3 = {will_save}. This \
             is a standalone explanation record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded (3/3): the fast-movement flat +10 ft. speed value. This grounds only
    // the flat bonus value itself, not a runtime armor/encumbrance-state check
    // engine — no such engine exists anywhere in this codebase yet — so the value
    // is asserted unconditionally rather than computed from armor/load state, and
    // it is not wired into any speed/movement total. The PF1 Core Rulebook
    // fast-movement bonus does not scale with level, so this is the same flat +10
    // ft. value at every supported level.
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.fast_movement".to_owned(),
        value: 10,
        detail: "Barbarian fast movement: +10 ft. land speed extension while wearing no heavy armor \
             and carrying no heavy load (PF1 Core Rulebook Barbarian class table). This slice grounds \
             only the flat +10 ft. value, not a runtime armor/encumbrance-state check engine — no such \
             engine exists anywhere in this codebase yet — so the value is asserted unconditionally \
             rather than computed from armor/load state, and it is not wired into any speed/movement \
             total. This flat value does not scale with barbarian level"
            .to_owned(),
    });

    // Rules correction: the formerly-named illiteracy burden was vacuous under the
    // fixture's pf1.core_rulebook source package. Illiteracy is a D&D 3.5e Barbarian
    // trait; the PF1 Core Rulebook Barbarian is not illiterate, so there was never
    // anything to implement. The resolution is documented as a grounded value-0
    // record rather than silently dropped, and the old claim-blocking diagnostic
    // (class_feature.barbarian.bounded_progression.illiteracy.unsupported) is retired.
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.illiteracy_absent".to_owned(),
        value: 0,
        detail: format!(
            "{class_name} illiteracy burden resolved as vacuous: the PF1 Core Rulebook {class_name} \
             is NOT illiterate — illiteracy is a D&D 3.5e {class_name} class trait that was removed in \
             Pathfinder 1e and never existed under the pf1.core_rulebook source package this fixture \
             names. The previously catalogued illiteracy burden therefore named a rule with no PF1 \
             existence, and retiring it is a rules correction, not an uplift. This record documents \
             that correction only; it carries no mechanical value (+0)"
        ),
    });

    // Grounded: Rage's flat numeric surface, values only. Rage rounds per day is the
    // one Constitution-derived rage number the PF1 Core Rulebook Rage class feature
    // grounds at level 1 (4 + Constitution modifier) and grows by a further flat +2
    // rounds "at each level after 1st" (PF1 Core Rulebook Rage: "She can rage for a
    // number of rounds per day equal to 4 + her Constitution modifier. At each level
    // after 1st, she can rage for 2 additional rounds."), generalized here as
    // 4 + Constitution modifier + 2 * (level - 1). At level 1 this collapses to the
    // original 4 + Constitution modifier (2 * 0 = 0 extra rounds), so the grounded
    // level-1 truth is unchanged by this widening. At a low enough Constitution
    // modifier that sum is non-positive, which is not a real PF1 rounds-per-day
    // count, so this slice claim-blocks the record instead of asserting a
    // fabricated zero/negative value — the deterministic Con 16 fixture (modifier
    // +3, 7 rounds at level 1, 9 rounds at level 2) never hits this branch, but the
    // public compute seam accepts any Human Barbarian input.
    let constitution_modifier = ability_modifier_for(ability_modifiers, "constitution");
    let rage_rounds_per_day = barbarian_rage_rounds_per_day(constitution_modifier, level, &input.chosen.selected_feats);
    if rage_rounds_per_day > 0 {
        explanations.push(ComputationExplanation {
            id: "class_chassis.barbarian.rage_rounds_per_day".to_owned(),
            value: rage_rounds_per_day,
            detail: format!(
                "{class_name} level {level} rage rounds per day from the PF1 Core \
                 Rulebook Rage class feature: 4 + Constitution modifier + 2 * (level - 1) = 4 + \
                 {constitution_modifier} + 2 * ({level_value} - 1) = {rage_rounds_per_day} rounds per \
                 day at level {level} (the +2-additional-rounds-per-level-after-1st rule). This is a \
                 standalone explanation record: no round is ever consumed, tracked, or restored by \
                 this slice"
            ),
        });
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.barbarian.rage_rounds_per_day.unsupported".to_owned(),
            message: format!(
                "{class_name} level {level} rage rounds per day (4 + Constitution modifier + 2 * \
                 (level - 1)) is not grounded for this input: 4 + {constitution_modifier} + 2 * \
                 ({level_value} - 1) = {rage_rounds_per_day}, a non-positive count with no PF1 Core \
                 Rulebook meaning. This slice does not assert a fabricated zero/negative \
                 rounds-per-day value, so no rage rounds per day is claimed for this Constitution \
                 score"
            ),
            claim_blocking: true,
        });
    }

    // Grounded: the four flat while-raging constants, as value-only records.
    // These are identity/recognition values only; the real conditional
    // application (v0.6 alpha swarm, risks item 8) happens in
    // `apply_rage_ability_bonuses` (Strength/Constitution), `compute_total_saves`
    // (Will), and `compute_combat_baseline` (Armor Class), each gated on a
    // valid, active, in-budget `class_ability_activations` entry -- see
    // `active_barbarian_rage_bonus`. At level 11+ (BARBARIAN_GREATER_RAGE_LEVEL),
    // Rage becomes Greater Rage: the Strength/Constitution morale bonuses
    // genuinely rise from +4 to +6 and the Will-save morale bonus genuinely
    // rises from +2 to +3; the Armor Class penalty stays -2 either way (PF1
    // Core Rulebook Greater Rage: "the -2 penalty to AC remains"). At level
    // 20+ (BARBARIAN_MIGHTY_RAGE_LEVEL), Greater Rage becomes Mighty Rage: the
    // Strength/Constitution morale bonuses genuinely rise again to +8 and the
    // Will-save morale bonus genuinely rises to +4 (PF1 Core Rulebook Mighty
    // Rage: "the morale bonus to her Strength and Constitution increases to
    // +8 and the morale bonus on her Will saves increases to +4"). This is a
    // third tier on the same flat-constant pillar via `barbarian_rage_tier`,
    // the same pure function `active_barbarian_rage_bonus` calls.
    let (strength_bonus, constitution_bonus, will_save_bonus, rage_source_feature) =
        barbarian_rage_tier(level);
    let rage_constants: [(&str, i16, String, &str); 4] = [
        (
            "class_chassis.barbarian.rage.strength_morale_bonus",
            strength_bonus,
            format!("+{strength_bonus} morale bonus to Strength while raging"),
            "morale Strength",
        ),
        (
            "class_chassis.barbarian.rage.constitution_morale_bonus",
            constitution_bonus,
            format!("+{constitution_bonus} morale bonus to Constitution while raging"),
            "morale Constitution",
        ),
        (
            "class_chassis.barbarian.rage.will_save_morale_bonus",
            will_save_bonus,
            format!("+{will_save_bonus} morale bonus on Will saves while raging"),
            "morale Will saves",
        ),
        (
            "class_chassis.barbarian.rage.armor_class_penalty",
            -2,
            "-2 penalty to Armor Class while raging".to_owned(),
            "AC",
        ),
    ];
    for (id, value, effect, _short_label) in &rage_constants {
        explanations.push(ComputationExplanation {
            id: (*id).to_owned(),
            value: *value,
            detail: format!(
                "{class_name} {rage_source_feature} flat constant from the PF1 Core Rulebook \
                 {rage_source_feature} class feature: {effect}. This is the identity/recognition \
                 record only; the real conditional application to the integrated ability \
                 modifiers, total saves, and Armor Class (gated on a valid, active, in-budget \
                 rage activation) happens in `apply_rage_ability_bonuses`, `compute_total_saves`, \
                 and `compute_combat_baseline` respectively, not here"
            ),
        });
    }

    // Grounded (SD13-E5): Uncanny Dodge, a 2nd-level Barbarian class feature verified
    // independently against two primary PF1 sources (d20pfsrd and legacy.aonprd.com
    // both list "Rage power, uncanny dodge" as the Barbarian 2nd-level special feature
    // entry). Below the level-2 gate this is a correct PF1 Core Rulebook level-gate
    // absence (value 0); at or above it, it is a bounded identity/recognition record
    // only (value 0, non-fabricated) naming the rule text — mirroring exactly how
    // Rogue's/Monk's own Evasion and Druid's Woodland Stride were grounded, without
    // folding into any actual flat-footed-state tracking, Armor Class computation, or
    // invisibility-detection engine, none of which exists in this codebase. The level-2
    // row's OTHER named entry, a Rage Power choice (a genuinely open-ended choice-list
    // feature, a new-subsystem-shaped burden), is deliberately left named-but-unproven
    // this slice, mirroring how the Monk level-2 bonus feat grant and the Bard
    // Versatile Performance were each deliberately left unrecognized: no new
    // choice-slot and no new diagnostic was added for it.
    if level < BARBARIAN_UNCANNY_DODGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Uncanny Dodge at barbarian level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Uncanny Dodge is a 2nd-level barbarian class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Uncanny Dodge granted at barbarian level {level} (PF1 Core Rulebook, \
                 2nd-level barbarian class feature, part of the \"Rage power, uncanny dodge\" \
                 table entry): she cannot be caught flat-footed, and she retains her Dexterity \
                 bonus to Armor Class even if the attacker is invisible; she still loses her \
                 Dexterity bonus to Armor Class if immobilized, and a successful feint action can \
                 still strip it away. This is a bounded identity/recognition record only (value 0, \
                 non-fabricated): no flat-footed-state tracking, no Armor Class computation, and \
                 no invisibility-detection engine exists anywhere in this codebase to apply it, so \
                 this grounds no actual flat-footed immunity or Dexterity-to-AC retention"
            ),
        });
    }

    // Grounded (SD13-E5): Trap Sense, a 3rd-level Barbarian class feature (verified
    // independently against d20pfsrd and legacy.aonprd.com: both name "Trap sense +1"
    // as the Barbarian 3rd-level "Special" class table entry). Below the level-3 gate
    // this is a correct PF1 Core Rulebook level-gate absence (value 0); at or above it,
    // it is a bounded flat-magnitude record only (barbarian level / 3, floor) naming
    // the rule text — mirroring exactly how Rogue's own Trap Sense was grounded: the
    // magnitude is never applied to any actual Reflex-save total or Armor Class total,
    // since no saving-throw-resolution or armor-class-resolution engine exists in this
    // codebase, and no trap-detection or trap-triggering engine exists to decide when
    // it would apply.
    if level < BARBARIAN_TRAP_SENSE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.trap_sense".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Trap Sense at barbarian level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant magnitude is named but \
                 not computed. Trap Sense is a 3rd-level barbarian class feature."
            ),
        });
    } else {
        let trap_sense_bonus = level_value / 3;
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.trap_sense".to_owned(),
            value: trap_sense_bonus,
            detail: format!(
                "Barbarian Trap Sense granted at barbarian level {level} (PF1 Core Rulebook, \
                 3rd-level barbarian class feature): a +{trap_sense_bonus} bonus on Reflex \
                 saves made to avoid traps and a +{trap_sense_bonus} dodge bonus to AC against \
                 attacks made by traps (barbarian level / 3 = {trap_sense_bonus}; this bonus \
                 rises further at 6th/9th/12th/15th/18th barbarian level, beyond this bounded \
                 slice). This is a bounded flat-magnitude record only, non-fabricated: it is \
                 never applied to any actual Reflex-save total or AC total, since no \
                 saving-throw-resolution or armor-class-resolution engine exists anywhere in \
                 this codebase to apply it, and no trap-detection or trap-triggering engine \
                 exists to decide when it would apply"
            ),
        });
    }

    // Grounded (SD13-E5): Improved Uncanny Dodge, a 5th-level Barbarian class
    // feature (verified independently against d20pfsrd and legacy.aonprd.com: both
    // name "Improved uncanny dodge" as the Barbarian 5th-level "Special" class table
    // entry). Below the level-5 gate this is a correct PF1 Core Rulebook level-gate
    // absence (value 0); at or above it, it is a bounded identity/recognition record
    // only (value 0, non-fabricated) naming the rule text — mirroring exactly how
    // Uncanny Dodge itself was grounded. The rule's own CONDITIONAL piece (comparing
    // the attacking rogue's own levels against the barbarian's own levels to decide
    // whether the immunity is actually pierced) is never applied: no
    // flanking-resolution engine, no attacker-level-comparison engine, and no
    // sneak-attack-trigger engine exists anywhere in this codebase, so this grounds
    // no actual flanking immunity or sneak-attack denial.
    if level < BARBARIAN_IMPROVED_UNCANNY_DODGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.improved_uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Improved Uncanny Dodge at barbarian level {level}: correctly \
                 absent at level {level} by PF1 Core Rulebook level gate; the at-grant rule \
                 is named but not computed. Improved Uncanny Dodge is a 5th-level barbarian \
                 class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.improved_uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Improved Uncanny Dodge granted at barbarian level {level} (PF1 \
                 Core Rulebook, 5th-level barbarian class feature): at 5th level and higher, a \
                 barbarian can no longer be flanked, denying a rogue the ability to sneak \
                 attack her by flanking unless the attacker has at least four more rogue \
                 levels than the barbarian has barbarian levels. This is a bounded \
                 identity/recognition record only (value 0, non-fabricated): no \
                 flanking-resolution engine, no attacker-level-comparison engine, and no \
                 sneak-attack-trigger engine exists anywhere in this codebase to apply it, so \
                 this grounds no actual flanking immunity or sneak-attack denial"
            ),
        });
    }

    // Grounded (SD13-E5): Damage Reduction, a 7th-level Barbarian class feature
    // (verified independently against d20pfsrd and legacy.aonprd.com: both name
    // "Damage reduction 1/-" as the Barbarian 7th-level "Special" class table entry,
    // with the rule text "At 7th level, a barbarian gains damage reduction. Subtract 1
    // from the damage the barbarian takes each time she is dealt damage from a weapon
    // or a natural attack"). Below the level-7 gate this is a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it, it is a bounded
    // flat-magnitude record only (a flat value of 1, non-fabricated) naming the rule
    // text — mirroring exactly how Trap Sense's own flat magnitude was grounded: the
    // magnitude is never applied to any actual incoming-damage total, since no
    // damage-resolution engine or incoming-damage total exists anywhere in this
    // codebase. Both primary sources' level-7 "Special" column names Damage Reduction
    // only, not a Rage Power grant — Rage Powers are granted at 2nd, 4th, 6th, 8th,
    // and 10th barbarian level, not 7th — so no rage-power-selection-slot-count engine
    // is invented here.
    if level < BARBARIAN_DAMAGE_REDUCTION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.damage_reduction".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Damage Reduction at barbarian level {level}: correctly absent at \
                 level {level} by PF1 Core Rulebook level gate; the at-grant magnitude is named \
                 but not computed. Damage Reduction is a 7th-level barbarian class feature."
            ),
        });
    } else {
        let damage_reduction_value: i16 = if level < BARBARIAN_DAMAGE_REDUCTION_TWO_LEVEL {
            1
        } else if level < BARBARIAN_DAMAGE_REDUCTION_THREE_LEVEL {
            2
        } else if level < BARBARIAN_DAMAGE_REDUCTION_FOUR_LEVEL {
            3
        } else if level < BARBARIAN_DAMAGE_REDUCTION_FIVE_LEVEL {
            4
        } else {
            5
        };
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.damage_reduction".to_owned(),
            value: damage_reduction_value,
            detail: format!(
                "Barbarian Damage Reduction granted at barbarian level {level} (PF1 Core \
                 Rulebook, 7th-level barbarian class feature, \"Damage reduction 1/-\", \
                 rising by 1 point at 10th level and every three levels thereafter — the \
                 level-{level} magnitude is {damage_reduction_value}/-): \
                 subtract {damage_reduction_value} from the damage the barbarian takes each \
                 time she is dealt damage \
                 from a weapon or a natural attack. This is a bounded flat-magnitude record \
                 only (value {damage_reduction_value}, non-fabricated): no damage-resolution \
                 engine and no \
                 incoming-damage total exists anywhere in this codebase to apply it, so this \
                 grounds no actual damage reduction"
            ),
        });
    }

    // Grounded (SD18): Indomitable Will, a 14th-level Barbarian class feature
    // (verified independently against d20pfsrd and the Archives of Nethys
    // aonprd.com mirror, byte-for-byte agreement: both name "Indomitable will,
    // rage power" as the Barbarian 14th-level "Special" class table entry, and
    // both give the rule text "While she is raging, a barbarian gains a +4
    // morale bonus on Will saves to resist enchantment spells and effects").
    // Below the level-14 gate this is a correct PF1 Core Rulebook level-gate
    // absence (value 0); at or above it, it is a bounded flat-magnitude record
    // only (a flat value of 4, non-fabricated) naming the rule text —
    // mirroring exactly how the four pre-existing flat while-raging rage
    // constants were grounded: the magnitude is never applied to any actual
    // Will-save total, since no saving-throw-resolution engine, no
    // spell-school-classification engine (to decide whether an incoming save
    // is against an enchantment effect), and no rage-state execution engine
    // (to decide whether the barbarian is currently raging) exists anywhere
    // in this codebase to apply it.
    if level < BARBARIAN_INDOMITABLE_WILL_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.indomitable_will".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Indomitable Will at barbarian level {level}: correctly absent at \
                 level {level} by PF1 Core Rulebook level gate; the at-grant magnitude is named \
                 but not computed. Indomitable Will is a 14th-level barbarian class feature."
            ),
        });
    } else {
        let indomitable_will_bonus = BARBARIAN_INDOMITABLE_WILL_ENCHANTMENT_WILL_SAVE_BONUS;
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.indomitable_will".to_owned(),
            value: indomitable_will_bonus,
            detail: format!(
                "Barbarian Indomitable Will granted at barbarian level {level} (PF1 Core \
                 Rulebook, 14th-level barbarian class feature): while raging, a +{indomitable_will_bonus} \
                 morale bonus on Will saves to resist enchantment spells and effects. This is a \
                 bounded flat-magnitude record only (value {indomitable_will_bonus}, \
                 non-fabricated): no saving-throw-resolution engine, no \
                 spell-school-classification engine, and no rage-state execution engine exists \
                 anywhere in this codebase to apply it, so this grounds no actual Will-save \
                 bonus"
            ),
        });
    }

    // Grounded (SD18): Tireless Rage, a 17th-level Barbarian class feature
    // (verified independently against d20pfsrd and the Archives of Nethys
    // aonprd.com mirror, byte-for-byte agreement across the full
    // levels-15-through-19 block: both name "Tireless rage" as the Barbarian
    // 17th-level "Special" class table entry, and both give the rule text
    // "Starting at 17th level, a barbarian no longer becomes fatigued at the
    // end of her rage"). Below the level-17 gate this is a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it, it transitions
    // to a bounded GRANT-only identity record (value 0, non-fabricated)
    // mirroring the Indomitable Will / Paladin Aura-of-Justice /
    // Aura-of-Faith / Aura-of-Righteousness idiom exactly: no rage-state
    // execution engine exists anywhere in this codebase to track rage
    // activation, round-by-round consumption, or the moment a rage ends, so
    // there is no fatigue-application mechanism for this feature to
    // interact with, and none is fabricated.
    if level < BARBARIAN_TIRELESS_RAGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.barbarian.tireless_rage".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Tireless Rage at barbarian level {level}: correctly absent at \
                 level {level} by PF1 Core Rulebook level gate; the at-grant rule is named but \
                 not computed. Tireless Rage is a 17th-level barbarian class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.barbarian.tireless_rage".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Tireless Rage granted at barbarian level {level} (PF1 Core \
                 Rulebook, 17th-level barbarian class feature): \"Starting at 17th level, a \
                 barbarian no longer becomes fatigued at the end of her rage.\" This is a \
                 bounded grant-only identity record only (value 0, non-fabricated): no \
                 rage-state execution engine exists anywhere in this codebase to track rage \
                 activation, round-by-round consumption, or the moment a rage ends, so there \
                 is no fatigue-application mechanism for this feature to interact with, and \
                 none is fabricated."
            ),
        });
    }

    // SD13-E5: the five rage power choice slots (gates 2/4/6/8/10), the
    // discharge of the rage-power-choice-list deferrals — numbered slots per
    // the proven repeat-grant idiom, open-ended recognitions fabricating
    // NOTHING about any power's effect: rage powers function only while
    // raging, and the rage-state execution engine below stays the named
    // engine burden untouched.
    for (slot_number, grant_level, choice_id) in BARBARIAN_RAGE_POWER_SLOTS {
        if level < grant_level {
            continue;
        }
        let Some(power) = choice_selection(input, choice_id) else {
            continue;
        };
        let record_id = if slot_number == 1 {
            "class_chassis.barbarian.rage_power_choice".to_owned()
        } else {
            format!("class_chassis.barbarian.rage_power_{slot_number}_choice")
        };
        explanations.push(ComputationExplanation {
            id: record_id,
            value: 0,
            detail: format!(
                "Barbarian rage power slot {slot_number} selection ({choice_id} -> {power}) \
                 at the level-{grant_level} grant (PF1 Core Rulebook, verified identically \
                 on both primary sources: \"Starting at 2nd level, a barbarian gains a rage \
                 power. She gains another rage power for every two levels of barbarian \
                 attained after 2nd level.\"; \"Unless otherwise noted, a barbarian cannot \
                 select an individual power more than once.\"). The level-{level} selection \
                 for this slot is {power}, recognized as a bounded +0 record of the numbered \
                 choice slot only (open-ended raw string, no power-list validation): rage \
                 powers grant their benefits only while raging, and the rage-state execution \
                 engine — activation, round tracking, application of any power's effect — is \
                 exactly the named engine burden this row still claim-blocks, so nothing is \
                 fabricated from this recognition"
            ),
        });
    }

    // SD31-E4-F2-003: the loop above deliberately grounds no per-power
    // magnitude (open-ended raw string, no power-list validation), which is
    // exactly why every one of the pool's corpus-wide records read as
    // `NoConsumerDelta` on the board's own `--class-feature-probe` --
    // recognising WHICH string was picked is not the same as computing a
    // value that differs by WHAT was picked. This closure grounds one real,
    // corpus-verified representative (Superstition) as the actual chooser
    // consumer this family has been missing, validated against
    // `CORE_RULEBOOK_RAGE_POWER_POOL` so an invented or drifted id can never
    // ground it.
    if barbarian_selected_rage_power(input, level, SUPERSTITION_RAGE_POWER_SELECTION) {
        let superstition_bonus = barbarian_superstition_save_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.rage_power.superstition.save_bonus".to_owned(),
            value: superstition_bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   , `BONUS:VAR|SuperstitionSaveBonus|2+RagePowersLVL/4`, RagePowersLVL =
                //   BarbarianLVL
                //   The corpus's own `BONUS:VAR` token for this power carries no `Raging`-state
                //   gate (checked directly against the full raw row -- unlike Raging Climber/Raging
                //   Swimmer's or Witch's Ward hex's own explicit gates), so this grounds once the
                //   power is selected, not only while actively raging.
                "Barbarian level {level} with the Superstition rage power selected (PF1 Core \
                 Rulebook, `KEY:Rage Power ~ Superstition`): a +{superstition_bonus} morale bonus on \
                 saving throws made to resist spells, supernatural abilities, and spell-like \
                 abilities (base {SUPERSTITION_SAVE_BONUS_BASE}, +1 every \
                 {SUPERSTITION_SAVE_BONUS_LEVEL_DIVISOR} barbarian levels). Standalone magnitude \
                 only -- not yet integrated into the real total saves this codebase computes, the \
                 same posture Witch's Ward hex magnitude carries; the corpus record's OTHER clause \
                 (\"While raging, you cannot be a willing target of any spell and must make saving \
                 throws to resist all spells, even those cast by allies\") is an unmodeled \
                 restriction, not a magnitude, and is not fabricated here. The other 27 Core \
                 Rulebook Rage Powers, and the wider ~60-record corpus-wide family across \
                 ACG/APG/UC/UW/AG/HA/UI and the player-companion books, remain named-but-unproven -- \
                 this is a representative-pool closure, not a claim that the family is exhausted."
            ),
        });
    }

    // v0.6 alpha swarm, risks item 8: the unconditional "rage-state
    // execution engine is missing" diagnostic that used to live here moved
    // to `ground_or_block_barbarian_rage`, called at the top of this
    // function -- the engine is real now, and its validity depends on
    // `class_ability_activations`, not on level/race alone, so it could
    // not stay a flat unconditional diagnostic at this position.
}

/// True when the Barbarian's own selection across ANY of the ten numbered
/// Rage Power slots (`BARBARIAN_RAGE_POWER_SLOTS`) names the given real,
/// corpus-verified Rage Power. Composes
/// `archetype_resolver::chooser_option_selected` per eligible slot (only
/// slots the character's level has actually reached) -- a Rage Power may
/// land in any numbered slot depending on the level at which the player
/// picked it, so a single-slot check would silently miss most valid
/// postures. Validated against `CORE_RULEBOOK_RAGE_POWER_POOL`, never a
/// bare string match, so an untrusted or drifted id can never ground a
/// value (`chooser_option_selected`'s own contract).
///
/// **Accepts a `"rage_power:"`-namespaced selection identically to the bare
/// form** (`rage_power_selection_denamespaced`). Real bug found live via
/// DoD-8 driving the actual desktop app, not a hypothetical: a bare
/// (zero-colon) `selection_id` cannot be PERSISTED through the real save
/// path at all -- `saved_character::local_store::validate_character_input`
/// requires every `selected_choices` entry to carry at least one colon to
/// round-trip through the fixture grammar (the exact shape the Arcanist
/// Metamagic Knowledge seed already hit and fixed, see
/// `apps/desktop/src-tauri/src/pf1_adapter.rs`'s
/// `EMPOWER_SPELL_METAMAGIC_SELECTION` doc comment). But `CLASS_FEATURE_
/// POOLS` (`v06_work_inventory.rs`, lane 6's file) registers this pool's
/// namespace as EMPTY, so the board's own `--class-feature-probe`
/// generates the BARE form. Both are the same real, corpus-verified value;
/// this de-namespaces the incoming selection before validating it against
/// the pool, rather than picking one convention and breaking the other
/// consumer.
pub(super) fn barbarian_selected_rage_power(input: &CharacterInput, level: u8, option_id: &str) -> bool {
    BARBARIAN_RAGE_POWER_SLOTS.iter().any(|(_, grant_level, choice_id)| {
        if level < *grant_level {
            return false;
        }
        input.chosen.selected_choices.iter().any(|choice| {
            choice.choice_set_id == *choice_id
                && rage_power_selection_denamespaced(&choice.selection_id) == option_id
                && CORE_RULEBOOK_RAGE_POWER_POOL.contains(&option_id)
        })
    })
}

/// The Unchained Barbarian's own analogue of [`barbarian_selected_rage_power`]
/// -- structurally identical (numbered-slot scan, denamespace, then validate
/// against the real pool), but over the SEPARATE
/// [`UNCHAINED_BARBARIAN_RAGE_POWER_SLOTS`]/[`UNCHAINED_RAGE_POWER_POOL`]
/// family, so a base-class selection can never satisfy this check and vice
/// versa (`decisions.md §10` AMENDMENT: distinct classes, distinct records).
/// Reuses [`rage_power_selection_denamespaced`] -- the namespace strip is a
/// generic string operation with no base-class-specific meaning, and
/// `v06_work_inventory.rs`'s `CLASS_FEATURE_POOLS` registers this pool's own
/// namespace column EMPTY too (mirroring the base pool), so the board's own
/// `--class-feature-probe` generates the identical bare-slug shape here.
pub(super) fn unchained_barbarian_selected_rage_power(input: &CharacterInput, level: u8, option_id: &str) -> bool {
    UNCHAINED_BARBARIAN_RAGE_POWER_SLOTS.iter().any(|(_, grant_level, choice_id)| {
        if level < *grant_level {
            return false;
        }
        input.chosen.selected_choices.iter().any(|choice| {
            choice.choice_set_id == *choice_id
                && rage_power_selection_denamespaced(&choice.selection_id) == option_id
                && UNCHAINED_RAGE_POWER_POOL.contains(&option_id)
        })
    })
}

/// PF1 Core Rulebook Superstition rage power: `BONUS:VAR|SuperstitionSaveBonus|
/// 2+RagePowersLVL/4`, `RagePowersLVL` = `BarbarianLVL` on the base class.
/// Integer division, matching the corpus formula exactly (no rounding
/// beyond PCGen's own floor-toward-zero `/` on non-negative operands).
///
/// **Also the Unchained Barbarian's own formula, reused rather than
/// duplicated** (`SD31-E4-F2-004`): `KEY:Unchained Rage Power ~ Superstition`
/// carries the byte-identical `BONUS:VAR|SuperstitionSaveBonus|2+floor(
/// RagePowersLVL/4)` token, and that class's own internal `Rage Powers`
/// record independently states `BONUS:VAR|RagePowersLVL|BarbarianLVL` too
/// (`pu_abilities_class.lst:291`) -- the same variable chain, so this pure
/// function of `level` alone is correct for either caller.
pub(super) fn barbarian_superstition_save_bonus(level: u8) -> i16 {
    SUPERSTITION_SAVE_BONUS_BASE + i16::from(level) / SUPERSTITION_SAVE_BONUS_LEVEL_DIVISOR
}

/// Barbarian's Rage rounds-per-day budget: 4 + Constitution modifier + 2 *
/// (level - 1) (PF1 Core Rulebook Rage: "4 + her Constitution modifier ...
/// at each level after 1st, she can rage for 2 additional rounds"). Pure
/// function (v0.6 alpha swarm, risks item 8) so the informational
/// explanation record inside `explain_barbarian_level1_chassis` and the
/// real rage-execution validation (`ground_or_block_barbarian_rage`,
/// `active_barbarian_rage_bonus`) call the identical formula rather than
/// either duplicating it or parsing it back out of explanation text.
pub(super) fn barbarian_rage_rounds_per_day(constitution_modifier: i16, level: u8, selected_feats: &[String]) -> i16 {
    4 + constitution_modifier
        + 2 * (i16::from(level) - 1)
        + extra_resource_feat_bonus(selected_feats, EXTRA_RAGE_FEAT_KEY, EXTRA_ROUNDS_PER_DAY)
}

/// Barbarian's Rage magnitude tier: (Strength morale bonus, Constitution
/// morale bonus, Will-save morale bonus, feature name), rising from Rage
/// (+4/+4/+2) to Greater Rage at `BARBARIAN_GREATER_RAGE_LEVEL` (+6/+6/+3)
/// to Mighty Rage at `BARBARIAN_MIGHTY_RAGE_LEVEL` (+8/+8/+4). The Armor
/// Class penalty stays -2 at every tier (PF1 Core Rulebook Greater Rage:
/// "the -2 penalty to AC remains"), so it is not part of this tuple --
/// callers needing it use `BARBARIAN_RAGE_ARMOR_CLASS_PENALTY` directly.
/// Pure function (v0.6 alpha swarm, risks item 8) so the informational
/// flat-constant explanation records and the real rage-execution engine
/// (ability modifiers, total saves, Armor Class) share one source of truth.
pub(super) fn barbarian_rage_tier(level: u8) -> (i16, i16, i16, &'static str) {
    if level >= BARBARIAN_MIGHTY_RAGE_LEVEL {
        (8, 8, 4, "Mighty Rage")
    } else if level >= BARBARIAN_GREATER_RAGE_LEVEL {
        (6, 6, 3, "Greater Rage")
    } else {
        (4, 4, 2, "Rage")
    }
}

/// Whether `input` is a Barbarian validly, actively raging right now, and
/// if so, the magnitude tier to apply (v0.6 alpha swarm, risks item 8).
/// Class-ownership-gated by construction: only returns `Some` when
/// `class_levels` actually contains Barbarian, so a non-Barbarian
/// character's stray `class_ability_activations` entry for
/// `BARBARIAN_RAGE_ABILITY_ID` is never read at all, not merely rejected
/// after the fact (mirrors the Ranger/Paladin/Sorcerer/Cleric/Druid
/// spell-posture shape exactly). An activation present but not
/// `ActiveState::EquippedActive`, or one that exceeds the grounded
/// rounds-per-day budget, is treated the same as "not raging" here --
/// pushes no diagnostic itself (`ground_or_block_barbarian_rage` is the
/// single place that pushes the over-budget claim-blocking diagnostic and
/// the informational recognition records), so no value is fabricated for
/// an already-claim-blocked posture, mirroring how an over-prepared
/// Ranger/Paladin/Cleric/Druid grounds no spell math either.
pub(super) fn active_barbarian_rage_bonus(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
) -> Option<(u8, i16, i16, i16, &'static str)> {
    let barbarian_level = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == BARBARIAN_CLASS_ID)
        .map(|class_level| class_level.level)?;

    let activation = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == BARBARIAN_RAGE_ABILITY_ID)?;

    if activation.active_state != ActiveState::EquippedActive {
        return None;
    }

    if let Some(rounds_consumed) = activation.rounds_consumed_today {
        let constitution_modifier = ability_modifier_for(ability_modifiers, "constitution");
        let rounds_per_day = barbarian_rage_rounds_per_day(constitution_modifier, barbarian_level, &input.chosen.selected_feats);
        if i32::from(rounds_consumed) > i32::from(rounds_per_day) {
            return None;
        }
    }

    let (strength_bonus, constitution_bonus, will_save_bonus, rage_source_feature) =
        barbarian_rage_tier(barbarian_level);
    Some((
        barbarian_level,
        strength_bonus,
        constitution_bonus,
        will_save_bonus,
        rage_source_feature,
    ))
}

/// Grounds or claim-blocks Barbarian's Rage execution engine for
/// `barbarian_level` (v0.6 alpha swarm, risks item 8). Called from the top
/// of `explain_barbarian_level1_chassis`, gated only on Barbarian
/// class-ownership -- independent of race/single-class status, so a
/// multiclass or non-Human Barbarian gets the identical validation a
/// single-class Human Barbarian does (the gate-ordering fix this session
/// already applied to Ranger/Paladin/Sorcerer/Cleric/Druid).
///
/// A character who simply isn't raging (no `class_ability_activations`
/// entry for `BARBARIAN_RAGE_ABILITY_ID`, or one present but
/// `active_state != EquippedActive`) is a genuinely valid PF1 posture --
/// not every Barbarian is always raging -- so this grounds a real
/// "not raging" recognition record rather than claim-blocking, mirroring
/// "zero prepared spells is always valid" from the spell-posture classes.
/// An activation that IS active but exceeds the grounded rounds-per-day
/// budget is a genuine posture violation and claim-blocks, mirroring every
/// other over-budget check landed this session (Ranger/Paladin/Cleric/
/// Druid's over-prepared-slot checks, Sorcerer's over-known check) --
/// never silently capped.
pub(super) fn ground_or_block_barbarian_rage(
    input: &CharacterInput,
    barbarian_level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(activation) = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == BARBARIAN_RAGE_ABILITY_ID)
    else {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.rage_execution.not_raging".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian level {barbarian_level} is not currently raging (no \
                 class_ability_activations entry for \"{BARBARIAN_RAGE_ABILITY_ID}\"): a \
                 genuinely valid PF1 posture, so no rage bonus, penalty, or budget is claimed. \
                 This grounds the rage-state execution engine's \"inactive\" branch only; \
                 entering rage is grounded separately below when an active, in-budget activation \
                 is present"
            ),
        });
        ground_raging_climber_and_swimmer(
            barbarian_level,
            "Barbarian",
            "class_feature.barbarian",
            false,
            supported_barbarian_level(input).is_some(),
            explanations,
        );
        return;
    };

    let constitution_modifier = ability_modifier_for(ability_modifiers, "constitution");
    let rounds_per_day = barbarian_rage_rounds_per_day(constitution_modifier, barbarian_level, &input.chosen.selected_feats);

    if let Some(rounds_consumed) = activation.rounds_consumed_today
        && i32::from(rounds_consumed) > i32::from(rounds_per_day) {
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.barbarian.rage_execution.rounds_exceeded".to_owned(),
                message: format!(
                    "Barbarian level {barbarian_level} rage activation claims \
                     {rounds_consumed} rounds consumed today, exceeding the grounded \
                     rounds-per-day budget of {rounds_per_day} (4 + Constitution modifier \
                     ({constitution_modifier}) + 2 * (level - 1)): a genuine posture violation, \
                     so no rage bonus, penalty, or budget is claimed for this input"
                ),
                claim_blocking: true,
            });
            return;
        }

    match activation.active_state {
        ActiveState::EquippedActive => {
            let (strength_bonus, constitution_bonus, will_save_bonus, rage_source_feature) =
                barbarian_rage_tier(barbarian_level);
            let rounds_consumed_today = activation.rounds_consumed_today.unwrap_or(0);
            explanations.push(ComputationExplanation {
                id: "class_feature.barbarian.rage_execution.active".to_owned(),
                value: 0,
                detail: format!(
                    "Barbarian level {barbarian_level} is actively raging \
                     ({rage_source_feature}), within the grounded rounds-per-day budget \
                     ({rounds_per_day} rounds; {rounds_consumed_today} consumed today). The \
                     +{strength_bonus} Strength / +{constitution_bonus} Constitution / \
                     +{will_save_bonus} Will morale bonuses and the \
                     {BARBARIAN_RAGE_ARMOR_CLASS_PENALTY} Armor Class penalty are applied to the \
                     integrated ability modifiers, total saves, and baseline Armor Class \
                     respectively -- see apply_rage_ability_bonuses, compute_total_saves, and \
                     compute_combat_baseline"
                ),
            });
            // Fatigue after rage ends has no representation in this schema yet
            // (a second, separate transient state, per the combat-time
            // activation-state scoping doc's open question) -- named honestly
            // rather than silently modeled or silently ignored, mirroring
            // every other "grant-only identity record, no execution engine"
            // note this session already uses. Non-blocking: the gap is
            // named, not hidden, but it does not claim-block an otherwise
            // valid rage posture.
            if barbarian_level < BARBARIAN_TIRELESS_RAGE_LEVEL {
                diagnostics.push(ComputationDiagnostic {
                    id: "class_feature.barbarian.rage_execution.fatigue_not_modeled".to_owned(),
                    message: format!(
                        "Barbarian level {barbarian_level} is raging below the Tireless Rage \
                         threshold ({BARBARIAN_TIRELESS_RAGE_LEVEL}th level): PF1 Rage causes \
                         fatigue once the rage ends, which this codebase does not yet represent \
                         as a transient post-rage state, so no fatigue condition (-2 Strength, \
                         -2 Dexterity, no run/charge) is applied. Named honestly rather than \
                         silently modeled or silently dropped"
                    ),
                    claim_blocking: false,
                });
            }
            ground_raging_climber_and_swimmer(
                barbarian_level,
                "Barbarian",
                "class_feature.barbarian",
                true,
                supported_barbarian_level(input).is_some(),
                explanations,
            );
        }
        ActiveState::SelectedInactive | ActiveState::Absent => {
            explanations.push(ComputationExplanation {
                id: "class_feature.barbarian.rage_execution.not_raging".to_owned(),
                value: 0,
                detail: format!(
                    "Barbarian level {barbarian_level} has a \
                     \"{BARBARIAN_RAGE_ABILITY_ID}\" activation entry but it is not active for \
                     this snapshot: a genuinely valid PF1 posture (available but not currently \
                     raging), so no rage bonus, penalty, or budget is claimed"
                ),
            });
            ground_raging_climber_and_swimmer(
                barbarian_level,
                "Barbarian",
                "class_feature.barbarian",
                false,
                supported_barbarian_level(input).is_some(),
                explanations,
            );
        }
    }
}

/// v0.6 alpha swarm, risks item 8: Barbarian, Sorcerer, and Cleric all
/// eventually reached `Computed` from what started as a permanent,
/// unconditional class-feature burden (Rage execution, Arcane Bond,
/// Touch of Good respectively) -- and Druid's animal companion joined
/// them too. Unlike every `<class>_dispatch_widening_safety_tests` module
/// before Barbarian's own, a valid posture in these classes actually
/// reaches `Computed`, not merely "blocked only on the permanent burden."
#[cfg(test)]
mod barbarian_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, ActiveState, CharacterClassLevel, CharacterInput,
        HeadlessReceiptStatus, BARBARIAN_CLASS_ID, BARBARIAN_RAGE_ABILITY_ID, FIGHTER_CLASS_ID,
        ROGUE_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, ClassAbilityActivation};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_barbarian_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: BARBARIAN_CLASS_ID.to_owned(), level }];
        input
    }

    /// A single-class Human Barbarian who is not raging (no
    /// `class_ability_activations` entry at all) is a genuinely valid PF1
    /// posture -- not every Barbarian is always raging -- and reaches
    /// `Computed` with zero claim-blocking diagnostics, the same golden
    /// path as Fighter now that `table_class_id` recognizes Barbarian and
    /// the rage-execution engine is real and conditional.
    #[test]
    fn single_class_barbarian_not_raging_reaches_computed() {
        let input = human_barbarian_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Barbarian who isn't raging is a genuinely valid PF1 posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.barbarian.rage_execution.not_raging"),
            "expected the honest not-raging recognition record: {:?}",
            receipt.computation.explanations
        );
    }

    /// A single-class Human Barbarian actively, validly raging (within
    /// budget) also reaches `Computed` (Fatigue is named but non-blocking
    /// below the Tireless Rage threshold), and the Strength/Constitution/
    /// Will/Armor-Class bonuses and penalty are genuinely applied to the
    /// integrated totals, not merely described.
    #[test]
    fn single_class_barbarian_actively_raging_in_budget_reaches_computed_and_applies_bonuses() {
        let mut input = human_barbarian_input(1);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARBARIAN_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(1),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "an active, in-budget Rage is a genuinely valid PF1 posture: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.barbarian.rage_execution.fatigue_not_modeled"
                    && !d.claim_blocking),
            "expected the honest, non-blocking Fatigue-not-modeled diagnostic below level 17: {:?}",
            receipt.computation.diagnostics
        );

        // Base fixture is Strength 16 (+3 from score, +4 with the fixture's
        // chosen Human +2 floating Strength bonus applied), Constitution 14
        // (+2). Rage (level 1, not Greater/Mighty) adds +4 Strength / +4
        // Constitution ability SCORE, i.e. +2/+2 ability MODIFIER.
        assert_eq!(receipt.computation.ability_modifiers.strength, 6);
        assert_eq!(receipt.computation.ability_modifiers.constitution, 4);

        let will_save = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.total_save.will")
            .expect("total Will save must be grounded");
        // Base Will save (barbarian level 1: 1/3 = 0) + Wisdom modifier
        // (12 -> +1) + feat bonus (0) + Rage Will bonus (+2) = 3.
        assert_eq!(will_save.value, 3, "Rage's Will-save morale bonus must be applied: {:?}", will_save);

        let armor_class = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .expect("baseline Armor Class must be grounded");
        // Base AC (10 + Chain Shirt 4 + DEX +2 + Dodge 1 = 17) - Rage penalty
        // (2) = 15.
        assert_eq!(armor_class.value, 15, "Rage's Armor Class penalty must be applied: {:?}", armor_class);
    }

    /// A Rage activation that exceeds the grounded rounds-per-day budget is
    /// a genuine posture violation and must claim-block -- never silently
    /// capped, mirroring every other over-budget check landed this session.
    #[test]
    fn single_class_barbarian_over_budget_rage_stays_blocked() {
        let mut input = human_barbarian_input(1);
        // Constitution 14 (+2): rounds per day = 4 + 2 + 2*(1-1) = 6.
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARBARIAN_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(7),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.barbarian.rage_execution.rounds_exceeded"
                    && d.claim_blocking),
            "expected the over-budget claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert_eq!(
            receipt.computation.ability_modifiers.strength, 4,
            "no rage bonus is applied for an over-budget, invalid posture: {:?}",
            receipt.computation.ability_modifiers
        );
    }

    /// A Rage activation present but not `EquippedActive` (available but
    /// not currently raging) is a genuinely valid posture too -- reaches
    /// `Computed`, and applies no bonus.
    #[test]
    fn single_class_barbarian_selected_inactive_rage_reaches_computed_with_no_bonus() {
        let mut input = human_barbarian_input(1);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARBARIAN_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::SelectedInactive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);
        assert_eq!(receipt.computation.ability_modifiers.strength, 4);
    }

    /// A non-Barbarian character carrying a spoofed `"rage"` activation
    /// entry must have it silently ignored, not applied -- the
    /// class-ownership gate is by construction (`active_barbarian_rage_bonus`
    /// only ever reads `class_ability_activations` after confirming
    /// `class_levels` contains Barbarian), not a bolt-on rejection.
    #[test]
    fn non_barbarian_characters_spoofed_rage_activation_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARBARIAN_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Barbarian rage entry: {:?}",
            receipt.computation.diagnostics
        );
        assert_eq!(
            receipt.computation.ability_modifiers.strength, 4,
            "a non-Barbarian character's spoofed rage entry must never apply a bonus: {:?}",
            receipt.computation.ability_modifiers
        );
    }

    /// A Barbarian+Rogue multiclass mix with a valid, active, in-budget
    /// Rage still gets the rage bonus applied -- the class-ownership gate
    /// checks "does `class_levels` contain Barbarian", not "is Barbarian
    /// the character's only class."
    #[test]
    fn barbarian_rogue_multiclass_with_active_rage_still_applies_the_bonus() {
        let mut input = human_barbarian_input(2);
        input.chosen.class_levels.push(CharacterClassLevel {
            class_id: ROGUE_CLASS_ID.to_owned(),
            level: 1,
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARBARIAN_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(1),
        });

        let receipt = build_pilot_headless_receipt(&input);

        // Constitution 14 (+2) -> +2 Constitution modifier bonus from Rage.
        assert_eq!(
            receipt.computation.ability_modifiers.constitution, 4,
            "a Barbarian+Rogue multiclass mix must still get Rage's bonus applied: {:?}",
            receipt.computation.ability_modifiers
        );
    }

    /// Rage's tier rises to Greater Rage at 11th level: the Strength/
    /// Constitution/Will bonuses rise from +4/+4/+2 to +6/+6/+3 (ability
    /// modifier +3/+3), and the Armor Class penalty stays -2.
    #[test]
    fn single_class_barbarian_at_greater_rage_level_applies_the_higher_tier() {
        let mut input = human_barbarian_input(11);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARBARIAN_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);
        // Base Strength 16 (+4 with the fixture's chosen Human +2 floating
        // Strength bonus applied) + Greater Rage's +3 modifier bonus = +7.
        assert_eq!(receipt.computation.ability_modifiers.strength, 7);
    }
}

/// Task #54: Raging Climber and Raging Swimmer, the two-power canonical-
/// narrowing follow-on to #53's `rage-powers-canonical-narrowing-scoping.md`
/// (60-record Rage Powers chooser family, only these two grounded). Both
/// are purely passive (`BONUS:VAR|RagingClimberBonus|RagePowersLVL` /
/// `BONUS:VAR|RagingSwimmerBonus|RagePowersLVL`, no arithmetic) and gated
/// on the corpus's own `PREVAREQ:Raging,1` -- reusing
/// `active_barbarian_rage_bonus`/`active_skald_inspired_rage_bonus`
/// exactly, so the grounded records cannot disagree with the already-
/// shipped rage-execution engine. Shared by Barbarian (`RagePowersLVL` =
/// `BarbarianLVL`) and Skald (`RagePowersLVL` = `SkaldLVL`); Bloodrager's
/// own `RagePowersLVL` connection is archetype-only (`Bloodrager
/// Archetype ~ Primalist`), never base Bloodrager, so it is not credited.
#[cfg(test)]
mod raging_climber_and_swimmer_tests {
    use super::{
        compute_pilot_base_chassis, ActiveState, CharacterClassLevel, CharacterInput,
        ComputationExplanation, BARBARIAN_CLASS_ID, BARBARIAN_RAGE_ABILITY_ID, SKALD_CLASS_ID,
        SKALD_INSPIRED_RAGE_ABILITY_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, ClassAbilityActivation};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_barbarian_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: BARBARIAN_CLASS_ID.to_owned(), level }];
        input
    }

    fn human_skald_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SKALD_CLASS_ID.to_owned(), level }];
        input
    }

    fn explanation<'a>(explanations: &'a [ComputationExplanation], id: &str) -> &'a ComputationExplanation {
        explanations
            .iter()
            .find(|e| e.id == id)
            .unwrap_or_else(|| panic!("expected explanation record {id}, got: {explanations:?}"))
    }

    fn climb_and_swim(input: &CharacterInput) -> (i16, i16) {
        let computation = compute_pilot_base_chassis(input);
        let climb = explanation(&computation.explanations, "skill.selected_modifier.climb").value;
        let swim = explanation(&computation.explanations, "skill.selected_modifier.swim").value;
        (climb, swim)
    }

    #[test]
    fn barbarian_not_raging_grounds_absence_and_no_skill_bonus() {
        let input = human_barbarian_input(5);
        let computation = compute_pilot_base_chassis(&input);

        let climber = explanation(&computation.explanations, "class_feature.barbarian.raging_climber");
        assert_eq!(climber.value, 0, "no bonus while not raging: {climber:?}");
        let swimmer = explanation(&computation.explanations, "class_feature.barbarian.raging_swimmer");
        assert_eq!(swimmer.value, 0, "no bonus while not raging: {swimmer:?}");

        // Baseline: rank 1 + Strength modifier (16+2 Human bonus = 18 -> +4)
        // + no class-skill bonus (this engine's `selected_skill_climb_is_
        // class_skill`/`swim` do not list Barbarian) + Chain Shirt ACP (-2,
        // no Fighter armor training since this isn't a Fighter) + 0 feat
        // bonus = 3.
        let (climb, swim) = climb_and_swim(&input);
        assert_eq!(climb, 3, "unraged Barbarian Climb total must carry no Raging Climber bonus");
        assert_eq!(swim, 3, "unraged Barbarian Swim total must carry no Raging Swimmer bonus");
    }

    #[test]
    fn barbarian_actively_raging_applies_rage_powers_level_to_climb_and_swim() {
        let mut input = human_barbarian_input(5);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARBARIAN_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let computation = compute_pilot_base_chassis(&input);
        let climber = explanation(&computation.explanations, "class_feature.barbarian.raging_climber");
        assert_eq!(climber.value, 5, "RagePowersLVL = BarbarianLVL = 5 while raging: {climber:?}");
        let swimmer = explanation(&computation.explanations, "class_feature.barbarian.raging_swimmer");
        assert_eq!(swimmer.value, 5, "RagePowersLVL = BarbarianLVL = 5 while raging: {swimmer:?}");

        let (climb, swim) = climb_and_swim(&input);
        // Baseline 3 (see the not-raging test) + RagePowersLVL 5 + the extra
        // +2 Strength modifier Rage's own tier-4 Strength score bonus adds
        // (18 -> 22, +4 -> +6 modifier) = 10. Proves the two bonuses
        // genuinely stack through the shared Strength-modifier term rather
        // than double-counting or clobbering each other.
        assert_eq!(climb, 10, "Raging Climber's +5 enhancement bonus must land on the real Climb total");
        assert_eq!(swim, 10, "Raging Swimmer's +5 enhancement bonus must land on the real Swim total");
    }

    #[test]
    fn barbarian_selected_but_inactive_rage_grounds_absence() {
        let mut input = human_barbarian_input(5);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARBARIAN_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::SelectedInactive,
            rounds_consumed_today: None,
        });

        let computation = compute_pilot_base_chassis(&input);
        let climber = explanation(&computation.explanations, "class_feature.barbarian.raging_climber");
        assert_eq!(climber.value, 0, "available but not raging must carry no bonus: {climber:?}");

        let (climb, _swim) = climb_and_swim(&input);
        assert_eq!(climb, 3, "an available-but-inactive rage must not inflate the Climb total");
    }

    #[test]
    fn skald_not_singing_grounds_absence_and_no_skill_bonus() {
        let input = human_skald_input(3);
        let computation = compute_pilot_base_chassis(&input);

        let climber = explanation(&computation.explanations, "class_feature.acg.skald.raging_climber");
        assert_eq!(climber.value, 0, "no bonus while not singing: {climber:?}");
        let swimmer = explanation(&computation.explanations, "class_feature.acg.skald.raging_swimmer");
        assert_eq!(swimmer.value, 0, "no bonus while not singing: {swimmer:?}");
    }

    #[test]
    fn skald_actively_singing_applies_rage_powers_level_to_climb_and_swim() {
        let baseline = human_skald_input(3);
        let (baseline_climb, baseline_swim) = climb_and_swim(&baseline);

        let mut input = human_skald_input(3);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: SKALD_INSPIRED_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let computation = compute_pilot_base_chassis(&input);
        let climber = explanation(&computation.explanations, "class_feature.acg.skald.raging_climber");
        assert_eq!(climber.value, 3, "RagePowersLVL = SkaldLVL = 3 while singing: {climber:?}");
        let swimmer = explanation(&computation.explanations, "class_feature.acg.skald.raging_swimmer");
        assert_eq!(swimmer.value, 3, "RagePowersLVL = SkaldLVL = 3 while singing: {swimmer:?}");

        let (climb, swim) = climb_and_swim(&input);
        // RagePowersLVL (3) + the extra +1 Strength modifier Inspired
        // Rage's own tier-3 Strength score bonus adds (18 -> 20, +4 -> +5
        // modifier) = +4 over baseline.
        assert_eq!(climb, baseline_climb + 4, "Raging Climber's bonus must land on the real Climb total");
        assert_eq!(swim, baseline_swim + 4, "Raging Swimmer's bonus must land on the real Swim total");
    }

    /// Class-ownership-gated by construction: a Fighter carrying a spoofed
    /// `rage`/`inspired_rage` activation must never ground either record or
    /// apply any bonus, mirroring `non_skald_characters_spoofed_inspired_rage_activation_is_ignored`.
    #[test]
    fn non_barbarian_non_skald_never_grounds_raging_climber_or_swimmer() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BARBARIAN_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: SKALD_INSPIRED_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id.contains("raging_climber") || e.id.contains("raging_swimmer")),
            "a Fighter must never ground Raging Climber/Swimmer records, spoofed activations or \
             not: {:?}",
            computation.explanations
        );

        let (climb, swim) = climb_and_swim(&input);
        // Fighter's own golden Climb/Swim total (rank 1 + Strength +4 +
        // class-skill bonus +3 + Chain Shirt ACP -2 = 6, matching
        // ge06_pilot_selected_skill_modifiers.rs's own golden value) must be
        // unaffected by a stray Barbarian/Skald rage activation.
        assert_eq!(climb, 6, "a Fighter's Climb total must never be inflated by a spoofed rage activation");
        assert_eq!(swim, 6, "a Fighter's Swim total must never be inflated by a spoofed rage activation");
    }
}

/// SD31-E4-F2-003: the `Rage Power` pool `chooser_option_selected` had never
/// grounded a real per-power magnitude before this test module -- the
/// pre-existing `BARBARIAN_RAGE_POWER_SLOTS` loop (`explain_barbarian_
/// level1_chassis`) deliberately records WHICH raw string was picked with a
/// flat +0 (open-ended, no power-list validation), so every one of the
/// pool's ~60 corpus-wide records read as `NoConsumerDelta` on the board's
/// own `--class-feature-probe`. Superstition is this cycle's representative
/// (mirrors the standing "ground one representative option per pool
/// honestly" ruling): grounds a real, level-scaling save bonus once the
/// player's own recorded selection names it, validated against the real
/// Core Rulebook Rage Power pool so an invented id can never ground it.
#[cfg(test)]
mod barbarian_rage_power_superstition_tests {
    use super::{
        compute_pilot_base_chassis, CharacterClassLevel, CharacterInput, ComputationExplanation,
        BARBARIAN_CLASS_ID, SUPERSTITION_RAGE_POWER_SELECTION,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    const SUPERSTITION_RECORD_ID: &str = "class_feature.barbarian.rage_power.superstition.save_bonus";

    fn human_barbarian_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: BARBARIAN_CLASS_ID.to_owned(), level }];
        input
    }

    fn explanation<'a>(
        explanations: &'a [ComputationExplanation],
        id: &str,
    ) -> &'a ComputationExplanation {
        explanations
            .iter()
            .find(|e| e.id == id)
            .unwrap_or_else(|| panic!("expected explanation record {id}, got: {explanations:?}"))
    }

    #[test]
    fn barbarian_without_any_rage_power_selection_grounds_no_superstition_record() {
        let input = human_barbarian_input(5);
        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == SUPERSTITION_RECORD_ID),
            "no Superstition record without a real recorded selection: {:?}",
            computation.explanations
        );
    }

    #[test]
    fn barbarian_level5_with_superstition_selected_at_slot1_grounds_the_real_save_bonus() {
        let mut input = human_barbarian_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:barbarian_rage_power".to_owned(),
            selection_id: SUPERSTITION_RAGE_POWER_SELECTION.to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);
        let bonus = explanation(&computation.explanations, SUPERSTITION_RECORD_ID);
        // PF1 Core Rulebook `BONUS:VAR|SuperstitionSaveBonus|2+RagePowersLVL/4`,
        // RagePowersLVL = BarbarianLVL = 5: 2 + 5/4 (integer division) = 3.
        assert_eq!(bonus.value, 3, "Superstition save bonus at barbarian level 5: {bonus:?}");
    }

    /// Real bug found live via DoD-8 driving the actual desktop app: a bare
    /// (zero-colon) `selection_id` cannot be PERSISTED at all --
    /// `saved_character::local_store::validate_character_input` requires at
    /// least one colon to round-trip through the fixture grammar (the same
    /// shape the Arcanist Metamagic Knowledge seed hit and fixed, see
    /// `apps/desktop/src-tauri/src/pf1_adapter.rs`'s
    /// `EMPOWER_SPELL_METAMAGIC_SELECTION` doc comment). `pf1_adapter.rs`'s
    /// own Path A seed therefore persists the NAMESPACED form
    /// `"rage_power:superstition"`, not the bare `SUPERSTITION_RAGE_POWER_
    /// SELECTION` the board's own `--class-feature-probe` generates (per
    /// `CLASS_FEATURE_POOLS`'s registered EMPTY namespace for this pool).
    /// Both must ground the SAME real value.
    #[test]
    fn a_namespaced_seed_selection_also_grounds_the_real_save_bonus() {
        let mut input = human_barbarian_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:barbarian_rage_power".to_owned(),
            selection_id: "rage_power:superstition".to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);
        let bonus = explanation(&computation.explanations, SUPERSTITION_RECORD_ID);
        assert_eq!(
            bonus.value, 3,
            "the namespaced seed form must ground the identical real value: {bonus:?}"
        );
    }

    #[test]
    fn barbarian_level9_with_superstition_selected_at_a_later_numbered_slot_still_grounds() {
        let mut input = human_barbarian_input(9);
        // Picked at the 8th-level slot (`choice:barbarian_rage_power_4`), not
        // slot 1 -- proves the check spans every numbered slot a Rage Power
        // can land in, not only the first.
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:barbarian_rage_power_4".to_owned(),
            selection_id: SUPERSTITION_RAGE_POWER_SELECTION.to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);
        let bonus = explanation(&computation.explanations, SUPERSTITION_RECORD_ID);
        // 2 + 9/4 (integer division) = 4.
        assert_eq!(bonus.value, 4, "Superstition save bonus at barbarian level 9: {bonus:?}");
    }

    /// MUTATION-PROOF guard: an invented selection id must never ground the
    /// real magnitude, even though it is recorded under a real choice-set id
    /// a Barbarian genuinely owns. Confirms `chooser_option_selected`'s own
    /// `corpus_pool.contains` check is load-bearing here, not decorative.
    #[test]
    fn an_invented_selection_id_never_grounds_the_superstition_bonus() {
        let mut input = human_barbarian_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:barbarian_rage_power".to_owned(),
            selection_id: "made_up_power".to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == SUPERSTITION_RECORD_ID),
            "an untrusted/invented selection id must never ground a real magnitude: {:?}",
            computation.explanations
        );
    }

    /// A DIFFERENT real, corpus-verified Rage Power selection must not
    /// spuriously ground Superstition's own record -- the same same-class,
    /// different-SLOT collision shape row 186/wave-11's own regression
    /// guarded against for the pool-name matcher.
    #[test]
    fn a_different_real_rage_power_selection_never_grounds_superstition() {
        let mut input = human_barbarian_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:barbarian_rage_power".to_owned(),
            selection_id: "surprise_accuracy".to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == SUPERSTITION_RECORD_ID),
            "selecting a DIFFERENT real Rage Power must never ground Superstition's own \
             record: {:?}",
            computation.explanations
        );
    }

    /// Reachability proof, per this cycle's own dispatch ("Reachability is
    /// proven through a headless pilot receipt, never a resolver unit
    /// test"): the real `build_pilot_headless_receipt` entry point --
    /// the same one `v06_class_state_dump`'s dashboard surface and the
    /// desktop app's own reach-gate both call -- reaches `Computed` (not
    /// `Blocked`) for a Human Barbarian with a genuine Superstition
    /// selection, and the real receipt's own explanations carry the
    /// grounded save-bonus record.
    #[test]
    fn the_headless_pilot_receipt_reaches_computed_with_superstition_selected() {
        use super::{build_pilot_headless_receipt, HeadlessReceiptStatus};

        let mut input = human_barbarian_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:barbarian_rage_power".to_owned(),
            selection_id: SUPERSTITION_RAGE_POWER_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Human Barbarian with Superstition selected must reach Computed, not Blocked: \
             {:?}",
            receipt.computation.diagnostics
        );
        let bonus = explanation(&receipt.computation.explanations, SUPERSTITION_RECORD_ID);
        assert_eq!(bonus.value, 3, "the real headless receipt must carry the grounded value: {bonus:?}");
    }
}

/// SD31-E4-F2-004: Unchained Barbarian's OWN Rage Power chooser had no
/// consumer-delta wiring at all -- `decisions.md §10`'s AMENDMENT forbids
/// resolving it by folding into the base Barbarian's chooser (the exact
/// "free ride" `SD31-E4-F2-003` found and correctly declined to credit, per
/// its own §3), so this closure wires the class's own SEPARATE
/// `UNCHAINED_BARBARIAN_RAGE_POWER_SLOTS`/`UNCHAINED_RAGE_POWER_POOL` family.
/// Mirrors `barbarian_rage_power_superstition_tests` structurally, including
/// its two mutation-proof negative controls.
#[cfg(test)]
mod unchained_barbarian_rage_power_superstition_tests {
    use super::{
        compute_pilot_base_chassis, CharacterClassLevel, CharacterInput, ComputationExplanation,
        UNCHAINED_SUPERSTITION_RAGE_POWER_SELECTION,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    const UNCHAINED_BARBARIAN_CLASS_ID: &str = "class:unchained_barbarian";
    const SUPERSTITION_RECORD_ID: &str =
        "class_feature.pu.unchained_barbarian.rage_power.superstition.save_bonus";

    fn human_unchained_barbarian_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: UNCHAINED_BARBARIAN_CLASS_ID.to_owned(), level }];
        input
    }

    fn explanation<'a>(
        explanations: &'a [ComputationExplanation],
        id: &str,
    ) -> &'a ComputationExplanation {
        explanations
            .iter()
            .find(|e| e.id == id)
            .unwrap_or_else(|| panic!("expected explanation record {id}, got: {explanations:?}"))
    }

    #[test]
    fn unchained_barbarian_without_any_rage_power_selection_grounds_no_superstition_record() {
        let input = human_unchained_barbarian_input(5);
        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation.explanations.iter().any(|e| e.id == SUPERSTITION_RECORD_ID),
            "no Superstition record without a real recorded selection: {:?}",
            computation.explanations
        );
    }

    #[test]
    fn unchained_barbarian_level5_with_superstition_selected_at_slot1_grounds_the_real_save_bonus()
    {
        let mut input = human_unchained_barbarian_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:unchained_barbarian_rage_power".to_owned(),
            selection_id: UNCHAINED_SUPERSTITION_RAGE_POWER_SELECTION.to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);
        let bonus = explanation(&computation.explanations, SUPERSTITION_RECORD_ID);
        // 2 + 5/4 (integer division) = 3, same formula as the base class.
        assert_eq!(bonus.value, 3, "Superstition save bonus at Unchained Barbarian level 5: {bonus:?}");
    }

    #[test]
    fn unchained_barbarian_level9_with_superstition_selected_at_a_later_numbered_slot_still_grounds()
    {
        let mut input = human_unchained_barbarian_input(9);
        // Picked at the 8th-level slot, not slot 1 -- proves the check spans
        // every numbered slot, not only the first.
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:unchained_barbarian_rage_power_4".to_owned(),
            selection_id: UNCHAINED_SUPERSTITION_RAGE_POWER_SELECTION.to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);
        let bonus = explanation(&computation.explanations, SUPERSTITION_RECORD_ID);
        assert_eq!(bonus.value, 4, "Superstition save bonus at Unchained Barbarian level 9: {bonus:?}");
    }

    /// MUTATION-PROOF guard 1: a selection recorded under the BASE class's
    /// own choice-set id must never satisfy the Unchained class's check --
    /// the two are separate slot families by design (`decisions.md §10`
    /// AMENDMENT), and this is the exact "free ride" `SD31-E4-F2-003` found
    /// and declined.
    #[test]
    fn a_base_class_slot_selection_never_grounds_the_unchained_superstition_bonus() {
        let mut input = human_unchained_barbarian_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:barbarian_rage_power".to_owned(),
            selection_id: UNCHAINED_SUPERSTITION_RAGE_POWER_SELECTION.to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation.explanations.iter().any(|e| e.id == SUPERSTITION_RECORD_ID),
            "a selection recorded under the BASE class's choice-set id must never ground the \
             Unchained class's own record: {:?}",
            computation.explanations
        );
    }

    /// MUTATION-PROOF guard 2: an invented selection id must never ground the
    /// real magnitude, even under the correct Unchained choice-set id.
    #[test]
    fn an_invented_selection_id_never_grounds_the_unchained_superstition_bonus() {
        let mut input = human_unchained_barbarian_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:unchained_barbarian_rage_power".to_owned(),
            selection_id: "made_up_power".to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation.explanations.iter().any(|e| e.id == SUPERSTITION_RECORD_ID),
            "an untrusted/invented selection id must never ground a real magnitude: {:?}",
            computation.explanations
        );
    }

    /// A DIFFERENT real, corpus-verified Unchained Rage Power selection must
    /// not spuriously ground Superstition's own record -- the same
    /// same-class-different-slot collision shape wave 11's own regression
    /// guards against for the pool-name matcher.
    #[test]
    fn a_different_real_unchained_rage_power_selection_never_grounds_superstition() {
        let mut input = human_unchained_barbarian_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:unchained_barbarian_rage_power".to_owned(),
            selection_id: "knockback".to_owned(),
        });

        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation.explanations.iter().any(|e| e.id == SUPERSTITION_RECORD_ID),
            "selecting a DIFFERENT real Unchained Rage Power must never ground Superstition's own \
             record: {:?}",
            computation.explanations
        );
    }

    /// Reachability proof, per this cycle's own dispatch ("Reachability is
    /// proven through a headless pilot receipt, never a resolver unit
    /// test"): the real `build_pilot_headless_receipt` entry point reaches
    /// `Computed` (not `Blocked`) for a Human Unchained Barbarian with a
    /// genuine Superstition selection, and the real receipt's own
    /// explanations carry the grounded save-bonus record.
    #[test]
    fn the_headless_pilot_receipt_reaches_computed_with_unchained_superstition_selected() {
        use super::{build_pilot_headless_receipt, HeadlessReceiptStatus};

        let mut input = human_unchained_barbarian_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:unchained_barbarian_rage_power".to_owned(),
            selection_id: UNCHAINED_SUPERSTITION_RAGE_POWER_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Human Unchained Barbarian with Superstition selected must reach Computed, not \
             Blocked: {:?}",
            receipt.computation.diagnostics
        );
        let bonus = explanation(&receipt.computation.explanations, SUPERSTITION_RECORD_ID);
        assert_eq!(bonus.value, 3, "the real headless receipt must carry the grounded value: {bonus:?}");
    }
}

/// v0.6 alpha swarm, risks item 8, second APG/ACG closure (2026-07-25):
/// Bloodrager's Bloodrage mirrors Barbarian's Rage even more closely than
/// Skald's Inspired Rage did (identical base four values, identical
/// 11th/17th/20th tier thresholds once the rounds-per-day formula is
/// algebraically simplified) -- see
/// `docs/release/v0.6/bloodrager-acg-second-class-scoping.md` for the
/// full corpus verification. Like Skald, a valid Bloodrager posture never
/// reaches `Computed` this slice (spellcasting stays permanently
/// deferred), so this module mirrors `skald_dispatch_widening_safety_tests`'s
/// shape, not Barbarian's own (which does reach `Computed`).
#[cfg(test)]
mod bloodrager_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, ActiveState, CharacterClassLevel,
        CharacterInput, HeadlessReceiptStatus, BLOODRAGER_CLASS_ID,
        BLOODRAGER_BLOODRAGE_ABILITY_ID, FIGHTER_CLASS_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_bloodrager_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: BLOODRAGER_CLASS_ID.to_owned(), level }];
        input
    }

    /// A single-class Human Bloodrager who is not bloodraging (no
    /// `class_ability_activations` entry at all) is a genuinely valid PF1
    /// posture -- reaches only as far as this slice allows, i.e. `Blocked`
    /// on the new, narrower spellcasting_deferred diagnostic alone (never
    /// the retired generic one), with the honest "not raging" recognition
    /// record grounded.
    #[test]
    fn single_class_bloodrager_not_raging_stays_blocked_on_deferred_spellcasting_only() {
        let input = human_bloodrager_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Bloodrager must stay Blocked on spellcasting alone: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.bloodrager.bloodrage_execution.not_raging"),
            "expected the honest not-raging recognition record: {:?}",
            receipt.computation.explanations
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.bloodrager.unsupported"),
            "the retired generic diagnostic must never appear for Bloodrager: {:?}",
            receipt.computation.diagnostics
        );
        // Level 1 Bloodragers have NO spellcasting (task #1, 2026-07-27):
        // the class block carries no CAST:/KNOWN: row below level 4, so
        // the block at this level is other-features, not spellcasting.
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.acg.bloodrager.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the new narrower spellcasting_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Human Bloodrager actively, validly bloodraging
    /// (within budget) applies the real Strength/Constitution/Will/Armor-
    /// Class bonuses and penalty to the integrated totals -- but still
    /// stays `Blocked` (spellcasting_deferred), unlike Barbarian's own
    /// equivalent test which reaches `Computed`.
    ///
    /// Constitution 14 (fixture base) -> +2 modifier: rounds per day =
    /// 2 + 2 + 2*1 = 6, so up to 6 rounds consumed today stays in budget.
    #[test]
    fn single_class_bloodrager_actively_raging_in_budget_applies_real_bonuses_but_stays_blocked() {
        let mut input = human_bloodrager_input(1);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BLOODRAGER_BLOODRAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(1),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Bloodrager stays Blocked on spellcasting even while actively, validly bloodraging: \
             {:?}",
            receipt.computation.diagnostics
        );
        // Level 1 Bloodragers have NO spellcasting (task #1, 2026-07-27):
        // the class block carries no CAST:/KNOWN: row below level 4, so
        // the block at this level is other-features, not spellcasting.
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.acg.bloodrager.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the spellcasting_deferred diagnostic even while bloodraging: {:?}",
            receipt.computation.diagnostics
        );

        // Base fixture is Strength 16 (+4 with the fixture's chosen Human
        // +2 floating Strength bonus applied), Constitution 14 (+2).
        // Bloodrage (level 1) adds +4 Strength / +4 Constitution ability
        // SCORE, i.e. +2/+2 ability MODIFIER.
        assert_eq!(receipt.computation.ability_modifiers.strength, 6);
        assert_eq!(receipt.computation.ability_modifiers.constitution, 4);

        let will_save = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.total_save.will")
            .expect("total Will save must be grounded");
        // Base Will save (Bloodrager level 1, poor Will: 0) + Wisdom
        // modifier (12 -> +1) + feat bonus (0) + Bloodrage Will bonus
        // (+2) = 3.
        assert_eq!(
            will_save.value, 3,
            "Bloodrage's Will-save morale bonus must be applied: {:?}",
            will_save
        );

        let armor_class = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "defense.baseline_armor_class")
            .expect("baseline Armor Class must be grounded");
        // Base AC (10 + Chain Shirt 4 + DEX +2 + Dodge 1 = 17) - Bloodrage
        // penalty (2) = 15.
        assert_eq!(
            armor_class.value, 15,
            "Bloodrage's Armor Class penalty must be applied: {:?}",
            armor_class
        );
    }

    /// A Bloodrage activation that exceeds the grounded rounds-per-day
    /// budget is a genuine posture violation and must claim-block -- never
    /// silently capped, mirroring every other over-budget check landed
    /// this session.
    #[test]
    fn single_class_bloodrager_over_budget_bloodrage_stays_blocked_and_applies_no_bonus() {
        let mut input = human_bloodrager_input(1);
        // Constitution 14 (+2): rounds per day = 2 + 2 + 2*1 = 6.
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BLOODRAGER_BLOODRAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: Some(7),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id
                    == "class_feature.acg.bloodrager.bloodrage_execution.rounds_exceeded"
                    && d.claim_blocking),
            "expected the over-budget claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert_eq!(
            receipt.computation.ability_modifiers.strength, 4,
            "no Bloodrage bonus is applied for an over-budget, invalid posture: {:?}",
            receipt.computation.ability_modifiers
        );
    }

    /// A non-Bloodrager character carrying a spoofed `"bloodrage"`
    /// activation entry must have it silently ignored, not applied -- the
    /// class-ownership gate is by construction
    /// (`active_bloodrager_bloodrage_bonus` only ever reads
    /// `class_ability_activations` after confirming `class_levels`
    /// contains Bloodrager), not a bolt-on rejection. Also proves
    /// Fighter's own golden path (including reaching `Computed`) is
    /// unaffected by a stray Bloodrager activation entry.
    #[test]
    fn non_bloodrager_characters_spoofed_bloodrage_activation_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BLOODRAGER_BLOODRAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Bloodrager bloodrage entry: \
             {:?}",
            receipt.computation.diagnostics
        );
        assert_eq!(
            receipt.computation.ability_modifiers.strength, 4,
            "a non-Bloodrager character's spoofed bloodrage entry must never apply a bonus: {:?}",
            receipt.computation.ability_modifiers
        );
    }

    /// Bloodrager has NO spellcasting below 4th level: `acg_classes.lst`
    /// carries no `CAST:`/`KNOWN:` row at all for levels 1-3, and the
    /// caster-level token itself is gated `PRECLASS:1,Bloodrager=4`.
    /// Claim-blocking a level 1-3 Bloodrager for missing spellcasting
    /// blocks it for a feature RAW says it does not yet have -- and
    /// level 1 is exactly where every other ACG/APG class test in this
    /// codebase is anchored.
    #[test]
    fn a_bloodrager_below_level_four_is_not_claim_blocked_for_missing_spellcasting() {
        for level in 1..=3u8 {
            let receipt = build_pilot_headless_receipt(&human_bloodrager_input(level));
            assert!(
                !receipt.computation.diagnostics.iter().any(|d| {
                    d.id == "class_feature.acg.bloodrager.spellcasting_deferred.unsupported"
                        && d.claim_blocking
                }),
                "level {level} Bloodrager has no spellcasting to defer: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// From 4th level the spellcasting deferral is genuine again.
    #[test]
    fn a_bloodrager_at_level_four_or_above_is_still_blocked_on_real_spellcasting() {
        for level in [4u8, 5, 20] {
            let receipt = build_pilot_headless_receipt(&human_bloodrager_input(level));
            assert!(
                receipt.computation.diagnostics.iter().any(|d| {
                    d.id == "class_feature.acg.bloodrager.spellcasting_deferred.unsupported"
                        && d.claim_blocking
                }),
                "level {level} Bloodrager genuinely has spellcasting: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// Bloodrager stays Blocked at EVERY level regardless -- the
    /// level-awareness fix narrows why it is blocked, it does not move
    /// the class toward Computed. Fast Movement, Uncanny Dodge, Blood
    /// Sanctuary, Damage Reduction, the Greater/Tireless/Mighty tiers,
    /// and the whole Bloodline slot remain unbuilt.
    #[test]
    fn bloodrager_stays_blocked_at_every_level_on_other_features() {
        for level in [1u8, 3, 4, 20] {
            let receipt = build_pilot_headless_receipt(&human_bloodrager_input(level));
            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Blocked,
                "level {level} Bloodrager must stay Blocked: {:?}",
                receipt.computation.diagnostics
            );
            assert!(
                receipt.computation.diagnostics.iter().any(|d| {
                    d.id == "class_feature.acg.bloodrager.other_features_deferred.unsupported"
                        && d.claim_blocking
                }),
                "level {level} must carry the other-features block: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// The shipped diagnostic asserted Bloodrager "has no class-skill
    /// list". It has one: `acg_abilities_class.lst`'s own
    /// `KEY:Bloodrager ~ Class Skills` record lists 11 skills. That is
    /// user-visible text asserting something untrue about the corpus.
    #[test]
    fn no_bloodrager_diagnostic_claims_the_class_has_no_class_skill_list() {
        let receipt = build_pilot_headless_receipt(&human_bloodrager_input(1));
        for diagnostic in &receipt.computation.diagnostics {
            assert!(
                !diagnostic.message.contains("no class-skill list"),
                "Bloodrager DOES have a class-skill list (11 skills): {diagnostic:?}"
            );
        }
    }

    /// Bloodrager's real list includes all three of Climb, Intimidate,
    /// and Swim -- the 7th instance of the class-skill-bonus widening,
    /// same all-three shape as Warpriest/Slayer/Brawler.
    #[test]
    fn bloodrager_earns_the_class_skill_bonus_on_all_three_tracked_skills() {
        let receipt = build_pilot_headless_receipt(&human_bloodrager_input(1));
        for skill in ["climb", "intimidate", "swim"] {
            let record = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == format!("skill.selected_modifier.{skill}"))
                .unwrap_or_else(|| panic!("{skill} modifier must be computed for a Bloodrager"));
            assert!(
                record.detail.contains("class-skill bonus (+3)"),
                "{skill} is on Bloodrager's real 11-skill list: {record:?}"
            );
        }
    }

    /// The spells-per-day and spells-known tables, read straight off the
    /// class block's own `CAST:`/`KNOWN:` tokens. Independently
    /// re-transcribed here from the corpus rather than copied from the
    /// implementation, and covering every level 4-20 boundary.
    ///
    /// The leading column is a genuine ZERO at every row -- Bloodragers
    /// get no 0-level spells at all. That is NOT Oracle's "orisons at
    /// will, no daily cap" sentinel; reading it that way would fabricate
    /// at-will cantrips.
    #[test]
    fn bloodrager_spell_tables_match_the_real_class_block_at_every_level() {
        for level in 1..4u8 {
            assert_eq!(
                super::bloodrager_spells_per_day(level),
                None,
                "level {level} has no CAST: row at all in the corpus"
            );
            assert_eq!(super::bloodrager_spells_known(level), None);
        }

        // (level, per-day 1st-4th, known 1st-4th) transcribed from
        // acg_classes.lst's own rows; the always-zero 0-level column is
        // deliberately not represented.
        let rows: [(u8, [i16; 4], [i16; 4]); 17] = [
            (4, [1, 0, 0, 0], [2, 0, 0, 0]),
            (5, [1, 0, 0, 0], [3, 0, 0, 0]),
            (6, [1, 0, 0, 0], [4, 0, 0, 0]),
            (7, [1, 1, 0, 0], [4, 2, 0, 0]),
            (8, [1, 1, 0, 0], [4, 3, 0, 0]),
            (9, [2, 1, 0, 0], [5, 4, 0, 0]),
            (10, [2, 1, 1, 0], [5, 4, 2, 0]),
            (11, [2, 1, 1, 0], [5, 4, 3, 0]),
            (12, [2, 2, 1, 0], [6, 5, 4, 0]),
            (13, [3, 2, 1, 1], [6, 5, 4, 2]),
            (14, [3, 2, 1, 1], [6, 5, 4, 3]),
            (15, [3, 2, 2, 1], [6, 6, 5, 4]),
            (16, [3, 3, 2, 1], [6, 6, 5, 4]),
            (17, [4, 3, 2, 1], [6, 6, 5, 4]),
            (18, [4, 3, 2, 2], [6, 6, 6, 5]),
            (19, [4, 3, 3, 2], [6, 6, 6, 5]),
            (20, [4, 4, 3, 2], [6, 6, 6, 5]),
        ];
        for (level, per_day, known) in rows {
            assert_eq!(
                super::bloodrager_spells_per_day(level),
                Some(per_day),
                "level {level} spells per day"
            );
            assert_eq!(
                super::bloodrager_spells_known(level),
                Some(known),
                "level {level} spells known"
            );
        }
    }

    /// Bloodrager's max spell level is 4, never 9 -- the class block
    /// never carries more than five columns.
    #[test]
    fn bloodrager_never_gains_a_spell_level_above_fourth() {
        for level in 4..=20u8 {
            let per_day = super::bloodrager_spells_per_day(level).expect("table row exists");
            assert_eq!(per_day.len(), 4, "only spell levels 1-4 are ever represented");
        }
        // 4th-level slots first appear at character level 13.
        for level in 4..13u8 {
            assert_eq!(super::bloodrager_spells_per_day(level).unwrap()[3], 0);
        }
        assert_eq!(super::bloodrager_spells_per_day(13).unwrap()[3], 1);
    }

    /// Task #87 wires the previously-orphaned
    /// `acg::bloodrager_spell_list` (200 corpus-verified entries, zero
    /// consumers before this) into a real known-spell validator, mirroring
    /// Skald's own spontaneous three-part shape.
    ///
    /// A single-class Bloodrager at level 4 (spells known `[2, 0, 0, 0]`,
    /// so a cap of two 1st-level spells and no access above 1st) knowing
    /// exactly two real 1st-level Bloodrager spells is a genuinely valid
    /// posture: the selection grounds and its own posture diagnostic must
    /// not fire.
    #[test]
    fn bloodrager_with_a_valid_known_spell_posture_grounds_the_selection() {
        let mut input = human_bloodrager_input(4);
        for spell_id in ["Burning Hands", "Blade Lash"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: BLOODRAGER_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt.computation.diagnostics.iter().any(|d| d.id
                == "class_spell.acg.bloodrager.spontaneous_known_and_per_day.unsupported"),
            "two real 1st-level spells is within bloodrager level 4's cap of 2: {:?}",
            receipt.computation.diagnostics
        );
        let known = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.bloodrager.known_spells")
            .expect("the known-spell posture must be grounded");
        assert_eq!(known.value, 2, "expected 2 known spells grounded: {known:?}");
    }

    /// A spell that is genuinely NOT on Bloodrager's own list must trip
    /// the posture. This is the assertion that actually proves the list is
    /// wired: it can only pass if `bloodrager_spell_level` is consulted.
    /// "Cure Light Wounds" is real PF1 content and is verifiably absent
    /// from `BLOODRAGER_SPELL_LIST`.
    #[test]
    fn bloodrager_knowing_an_off_list_spell_trips_the_posture() {
        let mut input = human_bloodrager_input(4);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: BLOODRAGER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt.computation.diagnostics.iter().any(|d| d.id
                == "class_spell.acg.bloodrager.spontaneous_known_and_per_day.unsupported"),
            "an off-list spell must not be silently accepted: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Over-knowing the real per-level cap trips the posture: bloodrager
    /// level 4 knows only two 1st-level spells.
    #[test]
    fn bloodrager_over_knowing_its_cap_trips_the_posture() {
        let mut input = human_bloodrager_input(4);
        for spell_id in ["Burning Hands", "Blade Lash", "Break"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: BLOODRAGER_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt.computation.diagnostics.iter().any(|d| d.id
                == "class_spell.acg.bloodrager.spontaneous_known_and_per_day.unsupported"),
            "3 first-level spells over-knows level 4's cap of 2: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A real Bloodrager spell of a level the character cannot yet access
    /// trips the posture. Acid Arrow is a real 2nd-level Bloodrager spell;
    /// a level-4 bloodrager's access ceiling is 1st.
    #[test]
    fn bloodrager_knowing_above_its_access_ceiling_trips_the_posture() {
        let mut input = human_bloodrager_input(4);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Acid Arrow".to_owned(),
            source_class_id: BLOODRAGER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Known,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt.computation.diagnostics.iter().any(|d| d.id
                == "class_spell.acg.bloodrager.spontaneous_known_and_per_day.unsupported"),
            "a 2nd-level spell is not accessible at bloodrager level 4: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The access ceiling is DERIVED from the shipped spells-known table
    /// rather than transcribed separately, so it cannot drift from its
    /// own source. Bloodragers have NO 0-level spells, so the ceiling is
    /// never 0 for a caster and the table is indexed 1-4, not 0-4 --
    /// the off-by-one that copying Skald's own 0-indexed shape would
    /// have introduced.
    #[test]
    fn the_access_ceiling_tracks_the_shipped_spells_known_table() {
        for level in 4..=20u8 {
            let known = super::bloodrager_spells_known(level).expect("table row exists");
            let expected =
                known.iter().rposition(|count| *count > 0).map_or(0, |index| index as i16 + 1);
            assert_eq!(
                super::bloodrager_spell_level_access(level),
                expected,
                "level {level}'s ceiling must be the highest spell level it can know"
            );
        }
        assert_eq!(super::bloodrager_spell_level_access(4), 1, "level 4 reaches only 1st");
        assert_eq!(super::bloodrager_spell_level_access(13), 4, "4th-level access begins at 13");
    }

    /// Anti-orphan regression guard (task #87). The spell list sat built
    /// but with ZERO consumers from task #1 until #87, which is how a
    /// diagnostic came to claim it was "not built" for that whole span --
    /// nothing exercised it, so nothing contradicted the claim.
    ///
    /// This test fails if the validator ever stops consulting the real
    /// list, which is the specific way that regression would recur: a
    /// validator that accepted any spell id would still pass every
    /// happy-path assertion above. Two spells that differ ONLY in whether
    /// they are on Bloodrager's list must produce different postures.
    #[test]
    fn the_known_spell_validator_actually_consults_the_real_spell_list() {
        let posture_holds = |spell_id: &str| {
            let mut input = human_bloodrager_input(4);
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: BLOODRAGER_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Known,
            });
            !build_pilot_headless_receipt(&input).computation.diagnostics.iter().any(|d| {
                d.id == "class_spell.acg.bloodrager.spontaneous_known_and_per_day.unsupported"
            })
        };

        assert!(
            super::acg::bloodrager_spell_list::bloodrager_spell_level("Burning Hands").is_some(),
            "fixture assumption: Burning Hands IS on the real list"
        );
        assert!(
            super::acg::bloodrager_spell_list::bloodrager_spell_level("Cure Light Wounds")
                .is_none(),
            "fixture assumption: Cure Light Wounds is NOT on the real list"
        );
        assert!(posture_holds("Burning Hands"), "an on-list spell must be accepted");
        assert!(
            !posture_holds("Cure Light Wounds"),
            "an off-list spell must be rejected -- if this passes, the validator is no longer \
             consulting the real spell list and the module is effectively orphaned again"
        );
    }
}

/// v0.6 alpha swarm, risks item 8 (Shaman full-build closure, 12th
/// ACG/APG class-specific closure): tests the Life Spirit choice
/// dispatch (Channel's flat uses-per-day/dice/DC facts), mirroring the
/// established dispatch-widening test module shape.
/// The nine non-Life Spirits' base abilities (task #12, stage 3).
#[cfg(test)]
mod bloodrager_remaining_features_tests {
    use super::{build_pilot_headless_receipt, CharacterClassLevel};
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn value(level: u8, id: &str) -> Option<i16> {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: "class:bloodrager".to_owned(), level }];
        build_pilot_headless_receipt(&input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    /// Blood Casting and Eschew Materials are BOOLEAN grants: both carry
    /// value 0 whether or not the character has reached their gate, so a
    /// value-only assertion could not tell "granted" from "correctly
    /// absent" and would pass against a record that never fires at all.
    /// Their tests read the detail instead (task #83).
    fn detail(level: u8, id: &str) -> Option<String> {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: "class:bloodrager".to_owned(), level }];
        build_pilot_headless_receipt(&input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.detail.clone())
    }

    /// Indomitable Will: a flat +4 on Will saves to resist enchantment
    /// spells while bloodraging, at bloodrager level 14 (task #83).
    /// Verified against `KEY:Bloodrager ~ Indomitable Will`'s own
    /// `ASPECT:SaveBonus|While Bloodraging +4 vs. enchantments` and its
    /// DESC. The corpus record carries NO `BONUS:` token, so like
    /// Barbarian's own already-grounded Indomitable Will this is a flat
    /// magnitude record only.
    #[test]
    fn indomitable_will_appears_exactly_at_its_level_14_gate() {
        let id = "class_feature.acg.bloodrager.indomitable_will";
        assert_eq!(value(13, id), Some(0), "absence at 13 must itself be grounded");
        assert_eq!(value(14, id), Some(4), "the +4 lands at its own level-14 gate");
        assert_eq!(value(20, id), Some(4), "no second tier exists");
    }

    /// Eschew Materials: a real automatic bonus-feat grant at level 4
    /// (`ABILITY:FEAT|AUTOMATIC|Eschew Materials`), mirroring Sorcerer's
    /// own `class_chassis.sorcerer.eschew_materials` boolean-grant record
    /// (task #83).
    #[test]
    fn eschew_materials_is_granted_at_its_level_4_gate() {
        let id = "class_feature.acg.bloodrager.eschew_materials";
        assert!(
            detail(3, id).expect("absence must be grounded").contains("correctly absent"),
            "below its gate the record must say so rather than imply a grant"
        );
        assert!(
            detail(4, id).expect("the grant must be grounded").contains("granted"),
            "at level 4 the feat is genuinely granted"
        );
        assert_eq!(value(4, id), Some(0), "a boolean feat grant carries no fabricated magnitude");
    }

    /// Blood Casting: the ability to cast and concentrate on bloodrager
    /// spells while bloodraging, at level 4. Grounded as genuinely
    /// vacuous under this scope -- it lifts a restriction this codebase
    /// does not model (no concentration engine exists anywhere) -- the
    /// same shape as Monk's Catch Off-Guard/Throw Anything record
    /// (task #83).
    #[test]
    fn blood_casting_is_granted_at_level_4_and_is_vacuous_under_this_scope() {
        let id = "class_feature.acg.bloodrager.blood_casting";
        assert!(
            detail(3, id).expect("absence must be grounded").contains("correctly absent"),
            "below its gate the record must say so"
        );
        let granted = detail(4, id).expect("the grant must be grounded");
        assert!(granted.contains("granted"), "at level 4 the ability is genuinely granted");
        assert!(
            granted.contains("concentration"),
            "the detail must name the unmodeled restriction it lifts, not imply a live benefit"
        );
        assert_eq!(value(4, id), Some(0), "no fabricated magnitude");
    }

    /// Gates come from the CLASS TABLE's per-level rows (1/2/3/5), not the
    /// .MOD grant lines, which carry no level PRE at all.
    #[test]
    fn each_feature_appears_exactly_at_its_class_table_gate() {
        for (id, gate, granted) in [
            ("class_feature.acg.bloodrager.fast_movement", 1u8, 10i16),
            ("class_feature.acg.bloodrager.uncanny_dodge_flanking_level", 2, 2),
            ("class_feature.acg.bloodrager.blood_sanctuary", 3, 2),
        ] {
            if gate > 1 {
                assert_eq!(value(gate - 1, id), Some(0), "{id} absence must be grounded");
            }
            assert_eq!(value(gate, id), Some(granted), "{id} at its gate");
        }
    }

    /// Uncanny Dodge's magnitude tracks the bloodrager's own level.
    #[test]
    fn uncanny_dodge_flanking_level_tracks_class_level() {
        assert_eq!(value(2, "class_feature.acg.bloodrager.uncanny_dodge_flanking_level"), Some(2));
        assert_eq!(value(9, "class_feature.acg.bloodrager.uncanny_dodge_flanking_level"), Some(9));
    }

    /// Improved Uncanny Dodge is a SECOND +1 to the same counter, so the
    /// tier is 0 / 1 / 2 across levels 1 / 2-4 / 5+.
    #[test]
    fn the_uncanny_dodge_tier_counter_steps_twice() {
        let id = "class_feature.acg.bloodrager.improved_uncanny_dodge_tier";
        assert_eq!(value(1, id), Some(0));
        assert_eq!(value(2, id), Some(1));
        assert_eq!(value(4, id), Some(1));
        assert_eq!(value(5, id), Some(2));
        assert_eq!(value(20, id), Some(2), "no third tier exists");
    }

    /// Greater/Mighty Bloodrage reuse the shipped bloodrager_bloodrage_tier,
    /// so these records must match its progression exactly.
    #[test]
    fn the_bloodrage_tier_records_match_the_shipped_progression() {
        let ability = "class_feature.acg.bloodrager.bloodrage_tier_ability_bonus";
        let save = "class_feature.acg.bloodrager.bloodrage_tier_save_bonus";
        assert_eq!((value(1, ability), value(1, save)), (Some(4), Some(2)));
        assert_eq!((value(10, ability), value(10, save)), (Some(4), Some(2)));
        assert_eq!((value(11, ability), value(11, save)), (Some(6), Some(3)), "Greater at 11");
        assert_eq!((value(19, ability), value(19, save)), (Some(6), Some(3)));
        assert_eq!((value(20, ability), value(20, save)), (Some(8), Some(4)), "Mighty at 20");
    }

    /// These ground for a NON-raging bloodrager too -- previously the tier
    /// magnitudes were reachable only through the active bloodrage path,
    /// so a non-raging receipt never named Greater/Mighty at all.
    #[test]
    fn the_tier_records_ground_without_an_active_bloodrage() {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: "class:bloodrager".to_owned(), level: 20 }];
        let receipt = build_pilot_headless_receipt(&input);
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.bloodrager.bloodrage_execution.not_raging"),
            "must be the not-raging branch"
        );
        assert_eq!(
            value(20, "class_feature.acg.bloodrager.bloodrage_tier_ability_bonus"),
            Some(8),
            "Mighty Bloodrage must still be named"
        );
    }
}

#[cfg(test)]
mod bloodrager_damage_reduction_tests {
    use super::{
        bloodrager_damage_reduction_amount, build_pilot_headless_receipt, CharacterClassLevel,
        CharacterInput,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn bloodrager(level: u8) -> CharacterInput {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: "class:bloodrager".to_owned(), level }];
        input
    }

    fn dr_record(level: u8) -> Option<i16> {
        build_pilot_headless_receipt(&bloodrager(level))
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.bloodrager.damage_reduction")
            .map(|e| e.value)
    }

    /// DR 1/- at 7th, +1 every three levels thereafter.
    #[test]
    fn the_magnitude_matches_the_corpus_formula() {
        assert_eq!(bloodrager_damage_reduction_amount(7), 1);
        assert_eq!(bloodrager_damage_reduction_amount(9), 1);
        assert_eq!(bloodrager_damage_reduction_amount(10), 2);
        assert_eq!(bloodrager_damage_reduction_amount(13), 3);
        assert_eq!(bloodrager_damage_reduction_amount(16), 4);
        assert_eq!(bloodrager_damage_reduction_amount(19), 5);
        assert_eq!(bloodrager_damage_reduction_amount(20), 5);
    }

    /// The gate is implicit in the integer division -- the formula simply
    /// has not reached 1 yet. Level 6 is the last absent level.
    #[test]
    fn the_gate_is_produced_by_the_formula_not_a_pre_token() {
        for level in 1..=6 {
            assert_eq!(
                bloodrager_damage_reduction_amount(level),
                0,
                "level {level} must have no DR"
            );
        }
        assert_eq!(bloodrager_damage_reduction_amount(7), 1, "first granted at 7");
    }

    /// The clamp is load-bearing, not defensive: the raw expression is
    /// NEGATIVE below level 4 under truncating division, and a negative
    /// DR would be a fabricated value rather than an absent one.
    #[test]
    fn the_clamp_prevents_a_negative_damage_reduction() {
        assert_eq!((i16::from(1u8) - 4) / 3, -1, "raw expression really does go negative");
        assert_eq!(bloodrager_damage_reduction_amount(1), 0);
        assert_eq!(bloodrager_damage_reduction_amount(3), 0);
    }

    /// Both branches ground a record -- "ground the absence, don't omit
    /// it", matching Skald's and Barbarian's own DR facts.
    #[test]
    fn both_branches_ground_a_record_through_the_live_dispatch() {
        assert_eq!(dr_record(6), Some(0), "the absence must be grounded, not omitted");
        assert_eq!(dr_record(7), Some(1));
        assert_eq!(dr_record(20), Some(5));
    }

    /// Passive: the corpus record carries no activation condition, so it
    /// grounds whether or not the bloodrager is bloodraging. The fixture
    /// carries no bloodrage activation, which is the not-raging branch.
    #[test]
    fn it_grounds_on_the_not_raging_branch_too() {
        let receipt = build_pilot_headless_receipt(&bloodrager(7));
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.bloodrager.bloodrage_execution.not_raging"),
            "fixture must exercise the not-raging branch"
        );
        assert_eq!(dr_record(7), Some(1), "DR must ground on that branch anyway");
    }
}

