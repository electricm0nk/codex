#[allow(unused_imports)]
pub(crate) use super::*;

// SD13-E3-F6 hybrid chassis baseline identities. Paladin and Ranger are hybrid
// (martial + later spellcasting) classes; this slice recognizes only their bounded
// single-class level-1 chassis as direct runtime evidence and grounds no class-feature
// or spell math for either.
pub(super) const PALADIN_CLASS_ID: &str = "class:paladin";

pub(super) const RANGER_CLASS_ID: &str = "class:ranger";

// SD13-E5 Paladin milestone widening. The accepted level-1/level-2/level-3
// chassis-and-spell-burden separation is now joined by level 4, the PF1 Core
// Rulebook level gate at which Smite Evil's uses/day genuinely increases (to
// 2/day) and Channel Positive Energy is newly granted, and by level 5, the
// PF1 Core Rulebook level gate at which the effective-caster-level gate
// genuinely increases again (to 2) and Channel Positive Energy's flat die
// count genuinely increases (to 3d6). Divine Bond, the PF1 CRB's OTHER
// 5th-level paladin class feature, was checked against a primary source
// (legacy.aonprd.com's Core Rulebook Paladin page) and confirmed NOT flat:
// it requires an activation/resource-consumption engine plus either an
// ongoing weapon-enhancement subsystem or a full mount
// stat-block/advancement subsystem, neither of which exists in this
// codebase, mirroring the Monk High Jump / Wizard level-5 bonus feat
// precedent exactly -- so it is deliberately left named-but-unproven, not
// fabricated. Level 6 joined next (both good saves and lay on hands/heal
// dice genuinely increase again; the level-6 repeat Mercy grant was checked
// and confirmed to need a mercy-list-growth mechanism this codebase has not
// grounded, so it stays named-but-unproven). Level 7 joins next in turn (PF1
// CRB level-7 "Special" column: "Smite evil 3/day" only, verified
// independently against d20pfsrd and legacy.aonprd.com -- base saves and lay
// on hands both stay numerically unchanged from level 6, an
// integer-division coincidence, while Smite Evil uses/day, the
// effective-caster-level gate, and Channel Positive Energy's die count all
// genuinely increase again; level 7 is not one of the repeat-Mercy-grant
// levels, 3/6/9/..., so nothing new is left unproven for Mercy here).
// Level 8 joins next in turn (PF1 CRB level-8 "Special" column: "Aura of
// resolve" only, verified independently against d20pfsrd and
// legacy.aonprd.com -- base attack and both good saves genuinely rise, poor
// Reflex stays +2, an integer-division coincidence; Lay on Hands genuinely
// rises on both axes (uses 6, heal dice 4) and Smite Evil's damage bonus
// rises to 8 (= paladin level) while its uses/day stay 3, the next rise
// landing at 10th; the effective caster level rises to 5 (8 - 3); Channel
// Positive Energy's die count stays 4, the effective-cleric dice rising at
// odd levels so the next rise lands at 9th. Aura of Resolve itself was
// checked rather than assumed away and confirmed NOT flat: immunity to
// charm spells/spell-like abilities plus a +4 morale aura for allies within
// 10 feet while conscious needs a condition-immunity engine and an
// ally-aura/positional engine, neither of which exists in this codebase --
// exactly like Aura of Courage and Divine Health before it, it stays
// deliberately named-but-unproven, not fabricated). Level 9 joins next in
// turn (PF1 CRB level-9 "Special" column: "Mercy" only, verified
// independently against d20pfsrd and legacy.aonprd.com -- base attack
// genuinely rises to +9 and poor Reflex genuinely rises to +3 while both
// good saves stay +6, integer-division coincidences; Smite Evil stays 3/day
// (next rise 10th) with its damage bonus rising to 9 (= paladin level); Lay
// on Hands stays at uses 6 / heal dice 4, integer-division coincidences;
// the effective caster level rises to 6 (9 - 3); Channel Positive Energy's
// die count genuinely rises to 5, the effective-cleric dice rising at odd
// levels. 9th IS a repeat-Mercy-grant level (the 3rd/6th/9th cadence), but
// exactly like the level-6 repeat grant, recognizing a second mercy
// selection needs the mercy-list-growth mechanism this codebase has never
// grounded, so the repeat grant stays deliberately named-but-unproven and
// the single grounded level-3 selection carries over unchanged). Level 10
// joins last, closing the level-10 band across every level-banded class row
// at the tranche ceiling (PF1 CRB level-10 "Special" column: "Smite evil
// 4/day" only, verified independently against d20pfsrd and
// legacy.aonprd.com -- base attack genuinely rises to +10 and both good
// saves genuinely rise to +7 (10 / 2 + 2) while poor Reflex stays +3, an
// integer-division coincidence; Smite Evil's uses/day genuinely rise to 4
// via the already-grounded threshold formula (1 + (10 - 1) / 3) with its
// damage bonus rising to 10 (= paladin level); Lay on Hands genuinely rises
// on both axes (uses 7, heal dice 5); the effective caster level rises to 7
// (10 - 3); Channel Positive Energy's die count stays 5, the
// effective-cleric dice rising at odd levels so the next rise lands at
// 11th; 10th is NOT a repeat-Mercy-grant level, so the single grounded
// level-3 selection carries over unchanged and no new named feature is left
// unproven by this slice. SD18 widens the gate again to level 11: base
// attack bonus genuinely rises to 11 (full BAB) while all three base saves
// stay numerically unchanged (11/2+2=7, 11/3=3, integer-division
// coincidences); Smite Evil's uses/day stay 4/day (another integer-division
// coincidence, the next rise lands at 13th) but its damage bonus genuinely
// rises to 11; Lay on Hands stays numerically unchanged on both axes;
// Channel Positive Energy's die count genuinely rises to 6 (ceil(11/2), the
// effective-cleric dice rising at odd levels); the effective caster level
// genuinely rises to 8 (11-3); the 3rd-level spell's base count and
// integrated total both genuinely rise from the honest ZERO at level 10 to
// 1 (the raw spells-per-day table row is "2/1/1/--" at level 11, verified
// independently against d20pfsrd and legacy.aonprd.com); 11th is NOT a
// repeat-Mercy-grant level, so the single grounded level-3 selection
// carries over unchanged again; and the level-11 "Special" column reads
// "Aura of justice" only (verified independently against both primary
// sources) -- grounded as a new bounded grant-only identity record
// (class_chassis.paladin.aura_of_justice), mirroring the Monk Diamond Body
// grant-only idiom exactly: no ally-aura/positional engine and no
// smite-evil-resource-sharing execution engine exists anywhere in this
// codebase to apply the shared smite to. A further SD18 slice
// (cycle-2026-07-15T0700) widens the gate again to level 12, verified
// independently against d20pfsrd and legacy.aonprd.com: base attack bonus
// genuinely rises to 12 (full BAB) and this time ALL THREE base saves
// genuinely rise too (good Fortitude/Will 12/2+2=8, poor Reflex 12/3=4,
// unlike level 11 where all three stayed numerically unchanged); Smite
// Evil's uses/day stay 4/day (another integer-division coincidence, the
// next rise lands at 13th) but its damage bonus genuinely rises to 12; Lay
// on Hands genuinely rises on both axes (uses 8, heal dice 6); Channel
// Positive Energy's die count stays 6 ((12+1)/2=6, an integer-division
// coincidence with level 11); the effective caster level genuinely rises
// to 9 (12-3); the 2nd-level spell's base count and integrated total both
// genuinely rise (base 1->2, total 2->3, the raw spells-per-day table row
// is "2/2/1/--" at level 12), while the 1st- and 3rd-level counts/totals
// stay numerically unchanged; and the level-12 "Special" column reads
// "Mercy" only (verified independently against both primary sources) --
// 12th IS a repeat-Mercy-grant level (the 3rd/6th/9th/12th cadence),
// grounded here as a FOURTH numbered mercy choice slot
// (class_chassis.paladin.mercy_4_choice), mirroring the proven slot-2/
// slot-3 idiom exactly. SD18 cycle-2026-07-15T1800 widens this once more
// to level 13: base attack genuinely rises to 13 (all three base saves
// stay numerically unchanged, integer-division coincidences); the
// level-13 "Special" column reads only "Smite evil 5/day" (verified
// independently against d20pfsrd and legacy.aonprd.com), which is NOT a
// new named feature -- the pre-existing smite-evil-uses-per-day formula
// is level-generic and already yields 5 at level 13 with no code change;
// 13th is NOT a repeat-Mercy-grant level, so no fifth mercy slot is
// added. The base spells-per-day table's level-13 row is "3/2/1/0",
// genuinely opening a 4th spell-level column
// (`PALADIN_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL`) for the first
// time, mirroring the Ranger level-13 widening's identical table shape
// (Paladin and Ranger share the same PF1 CRB spells-per-day table).
// SD18 cycle-2026-07-15T2500 widens this once more to level 14: base
// attack genuinely rises to 14 and, unlike level 13, ALL THREE base saves
// genuinely rise too (good Fortitude/Will 14/2+2=9, poor Reflex 14/3=4,
// an integer-division coincidence with level 13); the level-14 "Special"
// column reads only "Aura of faith" (verified independently against
// d20pfsrd, the Archives of Nethys aonprd.com mirror, and
// legacy.aonprd.com, all three agreeing byte-for-byte) -- a genuinely NEW
// class feature, grounded as a bounded grant-only identity record
// (class_chassis.paladin.aura_of_faith), mirroring the Aura of Justice /
// Monk Diamond Body idiom exactly. 14th is NOT a repeat-Mercy-grant
// level, so no fifth mercy slot is added. The base spells-per-day
// table's level-14 row is "3/2/1/1": only the 4th-level column genuinely
// rises (from 0 to 1), the first castable 4th-level paladin spell slot.
// SD18 cycle-2026-07-15T4300 widens this once more to level 15: base
// attack genuinely rises to 15 (full BAB) and poor Reflex genuinely rises
// to 5 (15/3, up from 4), while both good saves stay numerically unchanged
// at 9 (15/2+2, an integer-division coincidence with level 14). The
// level-15 "Special" column reads only "Mercy" (verified independently
// against d20pfsrd and the Archives of Nethys aonprd.com mirror,
// byte-for-byte agreement) -- 15th IS a repeat-Mercy-grant level (the
// 3rd/6th/9th/12th/15th cadence), grounded here as a FIFTH numbered mercy
// choice slot, mirroring the proven slot-2/3/4 idiom exactly; unlike the
// 6th/9th/12th-level repeat grants, both sources agree the CRB's named
// mercy-list tiers stop growing after 12th level, so the fifth slot's
// cited tier text names no new mercy condition, only the fifth pick from
// the existing pool. The base spells-per-day table's level-15 row is
// "3/2/2/1": only the 3rd-level column genuinely rises (from 1 to 2).
// SD18 cycle-2026-07-15T5400 widens this once more to level 16: base
// attack genuinely rises to 16 (full BAB) and, unlike level 15, BOTH good
// saves genuinely rise too (Fortitude/Will 16/2+2=10, up from 9), while
// poor Reflex stays numerically unchanged at 5 (16/3, an
// integer-division coincidence with level 15). The level-16 "Special"
// column reads only "Smite evil 6/day" (verified independently against
// d20pfsrd and the Archives of Nethys aonprd.com mirror, byte-for-byte
// agreement, so a third source was not required) -- this is NOT a new
// named feature: the pre-existing smite-evil-uses-per-day formula is
// already level-generic and yields 6 at level 16 with no code change.
// 16th is NOT a repeat-Mercy-grant level, so no sixth mercy slot is
// added. The base spells-per-day table's level-16 row is "3/3/2/1": only
// the 2nd-level column genuinely rises (from 2 to 3). SD18
// cycle-2026-07-15T10700 widens this once more to level 17: base attack
// genuinely rises to 17 (full BAB) while ALL THREE base saves stay
// numerically unchanged from level 16 (good Fortitude/Will 17/2+2=10,
// poor Reflex 17/3=5, integer-division coincidences). The level-17
// "Special" column reads only "Aura of righteousness" (verified
// independently against d20pfsrd and the Archives of Nethys aonprd.com
// mirror, byte-for-byte agreement, so a third source was not required)
// -- a genuinely NEW class feature, grounded as a THIRD bounded
// grant-only identity record mirroring the Aura of Justice / Aura of
// Faith idiom exactly: no damage-reduction-application engine and no
// compulsion-immunity-check engine exists anywhere in this codebase to
// apply "DR 5/evil and immunity to compulsion spells and spell-like
// abilities" to. 17th is NOT a repeat-Mercy-grant level, so no sixth
// mercy slot is added. Smite Evil's uses-per-day formula stays 6/day
// (an integer-division coincidence with level 16; the next rise lands
// at level 19) while its damage bonus genuinely rises to 17. The base
// spells-per-day table's level-17 row is "4/3/2/1": only the 1st-level
// column genuinely rises (from 3 to 4).
// SD18 cycle-2026-07-15T15000 widens this once more to level 18: base
// attack genuinely rises to 18 (full BAB) and BOTH good saves genuinely
// rise to 11 (18/2+2), while poor Reflex genuinely rises to 6 (18/3).
// The level-18 "Special" column reads only "Mercy" (verified
// independently against d20pfsrd and the Archives of Nethys aonprd.com
// mirror, byte-for-byte agreement) -- 18th IS a repeat-Mercy-grant level
// (the 3rd/6th/9th/12th/15th/18th cadence), grounded here as a SIXTH
// numbered mercy choice slot, mirroring the proven slot-2/3/4/5 idiom
// exactly; like the 15th-level repeat grant, both sources agree the
// CRB's named mercy-list tiers stop growing after 12th level, so the
// sixth slot's cited tier text names no new mercy condition, only the
// sixth pick from the existing pool. Smite Evil's uses-per-day formula
// stays 6/day (an integer-division coincidence with level 17; the next
// rise lands at level 19) while its damage bonus genuinely rises to 18.
// The base spells-per-day table's level-18 row is "4/3/2/2": only the
// 4th-level column genuinely rises (from 1 to 2). The base spells-per-day
// table's level-19 row is "4/3/3/2": only the 3rd-level column genuinely
// rises (from 2 to 3). The base spells-per-day table's level-20 row is
// "4/4/3/3" (verified independently against raw HTML fetches of d20pfsrd
// and the Archives of Nethys aonprd.com mirror, byte-for-byte agreement):
// the 2nd-level AND 4th-level columns BOTH genuinely rise simultaneously
// (2nd from 3 to 4, 4th from 2 to 3) -- the first level in this row's own
// widening history where two columns rise at once. Nothing here grounds
// level 21+ Paladin (PF1's 1-20 character-level cap).
pub(super) const MAX_SUPPORTED_PALADIN_LEVEL: u8 = 20;

// Aura of Faith is a 14th-level paladin feature in the PF1 Core Rulebook
// (verified independently against d20pfsrd, the Archives of Nethys
// aonprd.com mirror, and legacy.aonprd.com, all three agreeing
// byte-for-byte): "At 14th level, a paladin's weapons are treated as
// good-aligned for the purposes of overcoming damage reduction.
// Additionally, any attack made against an enemy within 10 feet of her is
// treated as good-aligned for the purposes of overcoming damage
// reduction." Below this level its honest computed surface is its
// correct ABSENCE (value 0); at or above it, this slice grounds a
// bounded GRANT-only identity record (mirroring the Aura of Justice /
// Monk Diamond Body idiom exactly): no alignment-treatment execution
// engine and no damage-reduction-overcoming resolution engine exists
// anywhere in this codebase to apply this to.
pub(super) const PALADIN_AURA_OF_FAITH_LEVEL: u8 = 14;

// Aura of Justice is an 11th-level paladin feature in the PF1 Core Rulebook
// (verified independently against d20pfsrd and legacy.aonprd.com): "At 11th
// level, a paladin can expend two uses of her smite evil ability to grant
// the ability to smite evil to all allies within 10 feet, using her
// bonuses, but through their own weapons." Below this level its honest
// computed surface is its correct ABSENCE (value 0); at or above it, this
// slice grounds a bounded GRANT-only identity record (mirroring the Monk
// Diamond Body idiom exactly): no ally-aura/positional engine and no
// smite-evil-resource-sharing execution engine exists anywhere in this
// codebase to apply the shared smite to.
pub(super) const PALADIN_AURA_OF_JUSTICE_LEVEL: u8 = 11;

// Aura of Righteousness is a 17th-level paladin feature in the PF1 Core
// Rulebook (verified independently against d20pfsrd and the Archives of
// Nethys aonprd.com mirror, both agreeing byte-for-byte): "At 17th level,
// a paladin gains DR 5/evil and immunity to compulsion spells and
// spell-like abilities." Below this level its honest computed surface is
// its correct ABSENCE (value 0); at or above it, this slice grounds a
// bounded GRANT-only identity record (mirroring the Aura of Justice /
// Aura of Faith idiom exactly): no damage-reduction-application engine
// and no compulsion-immunity-check engine exists anywhere in this
// codebase to apply this to.
pub(super) const PALADIN_AURA_OF_RIGHTEOUSNESS_LEVEL: u8 = 17;

/// The DR clause of Aura of Righteousness: a flat `DR 5/evil`, verified
/// against the feature's own corpus token `DR:5/Evil` and its `DESC` ("You
/// gain DR 5/Evil and immunity to compulsion spells and spell-like
/// abilities"). Flat at every level -- PF1 grants the paladin no further DR
/// tier after 17th, unlike Barbarian's or Skald's own tiered DR.
pub(super) const PALADIN_AURA_OF_RIGHTEOUSNESS_DAMAGE_REDUCTION: i16 = 5;

// Holy Champion is the 20th-level paladin capstone in the PF1 Core Rulebook
// (verified independently against a raw HTML fetch of d20pfsrd.com's own
// class table and description, and a raw HTML fetch of the Archives of
// Nethys aonprd.com mirror's ClassDisplay.aspx, both agreeing byte-for-byte,
// bypassing AI-summarization to guard against a tool-extraction artifact):
// "At 20th level, a paladin becomes a conduit for the power of her god. Her
// DR increases to 10/evil. Whenever she uses smite evil and successfully
// strikes an evil outsider, the outsider is also subject to a banishment,
// using her paladin level as the caster level... After the banishment
// effect and the damage from the attack is resolved, the smite immediately
// ends. In addition, whenever she channels positive energy or uses lay on
// hands to heal a creature, she heals the maximum possible amount." Below
// this level its honest computed surface is its correct ABSENCE (value 0);
// at or above it, this slice grounds a bounded GRANT-only identity record
// (mirroring the Aura of Justice / Aura of Faith / Aura of Righteousness
// idiom exactly): no damage-reduction-application engine, no
// banishment-spell-effect-resolution engine, and no healing-maximization
// execution engine exists anywhere in this codebase to apply any of this
// to.
pub(super) const PALADIN_HOLY_CHAMPION_LEVEL: u8 = 20;

// Lay on hands and divine grace are both 2nd-level paladin features in the PF1 Core
// Rulebook. Below this level their honest computed surface is their correct
// ABSENCE; at or above it, this slice grounds their flat numeric formulas.
pub(super) const PALADIN_LAY_ON_HANDS_DIVINE_GRACE_LEVEL: u8 = 2;

// Mercy is a 3rd-level paladin feature (gained at 3rd level and every three levels
// thereafter). Below this level its honest computed surface is its correct
// ABSENCE; at or above it (SD13-E5 level-3 widening), this slice grounds a
// bounded GRANT-only identity record (mirroring the Barbarian Uncanny Dodge /
// Ranger Endurance idiom) plus, when the fixture provides one, a choice-
// recognition record naming whichever mercy was selected (mirroring the Ranger
// Favored Terrain / Sorcerer bloodline choice-slot idiom). PF1 Core Rulebook
// Mercy (verified independently against legacy.aonprd.com's Core Rulebook
// Paladin page): "At 3rd level, and every three levels thereafter, a paladin
// can select one mercy. Each mercy adds an effect to the paladin's lay on
// hands ability." The first, 3rd-level tier of the mercy list is Fatigued,
// Shaken, and Sickened -- verified against the Core-Rulebook-scoped primary
// source rather than the aggregated Archives of Nethys mercy table, which also
// lists two additional 3rd-level-tier mercies (Deceived, Riled) sourced from a
// later supplement (Ultimate Combat), out of scope for this Core-Rulebook-only
// grounding. This grounds only the CHOICE recognition; the selected mercy's own
// effect (curing the named condition automatically whenever lay on hands is
// used) is NOT computed, since no lay-on-hands execution engine exists
// anywhere in this codebase.
pub(super) const PALADIN_MERCY_LEVEL: u8 = 3;

// Channel Positive Energy is a 4th-level paladin feature in the PF1 Core
// Rulebook (verified independently against legacy.aonprd.com's Core Rulebook
// Paladin page): "When a paladin reaches 4th level, she gains the
// supernatural ability to channel positive energy like a cleric. Using this
// ability consumes two uses of her lay on hands ability. A paladin uses her
// level as her effective cleric level when channeling positive energy."
// Below this level its honest computed surface is its correct ABSENCE; at or
// above it (SD13-E5 level-4 widening), this slice grounds only the flat
// channel-energy die-count magnitude (ceil(effective cleric level / 2)),
// mirroring the Cleric Channel Energy dice-count idiom exactly. No
// healing/damage-resolution execution, no heal-vs-harm target selection, and
// no lay-on-hands-resource-consumption bookkeeping is computed.
pub(super) const PALADIN_CHANNEL_POSITIVE_ENERGY_LEVEL: u8 = 4;

/// The paladin level at which 1st-level paladin spells first become available,
/// verified against the raw PF1 Core Rulebook Paladin spells-per-day table rows
/// (d20pfsrd and legacy.aonprd.com, identical): level 3 shows no spells-per-day
/// columns at all, level 4 shows "0/—/—/—" — the first non-"—" 1st-level
/// column. A "0" entry is real access, not absence: "When Table: Paladin
/// indicates that the paladin gets 0 spells per day of a given spell level,
/// she gains only the bonus spells she would be entitled to based on her
/// Charisma score for that spell level" (quoted identically by both sources).
pub(super) const PALADIN_FIRST_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 4;

/// The paladin level at which 2nd-level paladin spells first become available,
/// verified against the raw table rows (both sources): level 6 shows
/// "1/—/—/—", level 7 shows "1/0/—/—" — the first non-"—" 2nd-level column.
pub(super) const PALADIN_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 7;

/// The paladin level at which 3rd-level paladin spells first become available,
/// verified against the raw table rows (both sources): level 9 shows
/// "2/1/—/—", level 10 shows "2/1/0/—" — the first non-"—" 3rd-level column.
pub(super) const PALADIN_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 10;

/// The paladin level at which 4th-level paladin spells first become available,
/// verified against the raw table rows (d20pfsrd and legacy.aonprd.com, both
/// byte-for-byte identical): level 12 shows "2/2/1/—", level 13 shows
/// "3/2/1/0" — the first non-"—" 4th-level column. Matches the Ranger
/// `RANGER_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` threshold exactly
/// (Paladin and Ranger share the same PF1 CRB spells-per-day table shape).
pub(super) const PALADIN_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 13;

/// SD13-E5 Paladin Mercy choice-slot id. The deterministic level-3 fixture names a
/// chosen mercy (e.g. `mercy:shaken`); the compute seam recognizes whichever raw
/// mercy string was actually selected, mirroring `choice:ranger_favored_terrain`'s
/// open-ended (non-restricted-list) recognition idiom exactly -- no enum
/// validation against the mercy list is performed here.
pub(super) const PALADIN_MERCY_CHOICE_ID: &str = "choice:paladin_mercy";

/// PF1 Core Rulebook gates of the paladin's SECOND and THIRD mercies ("at
/// 6th level and every three levels thereafter" per the level-3 mercy rule).
/// The CRB mercy tiers were verified for this slice: legacy.aonprd.com (Core
/// Rulebook only) gives 3rd = Fatigued/Shaken/Sickened, 6th ADDS
/// Dazed/Diseased/Staggered, 9th ADDS
/// Cursed/Exhausted/Frightened/Nauseated/Poisoned; d20pfsrd's lists are
/// supersets whose CRB subset matches exactly — its extra entries are
/// non-CRB expansions outside this seam's pf1.core_rulebook source package.
/// Numbered slots per the proven repeat-grant idiom; this discharges the
/// "mercy-list-growth mechanism" deferrals recorded by the level-6 and
/// level-9 chassis slices.
pub(super) const PALADIN_SECOND_MERCY_GRANT_LEVEL: u8 = 6;

pub(super) const PALADIN_SECOND_MERCY_CHOICE_ID: &str = "choice:paladin_mercy_2";

pub(super) const PALADIN_THIRD_MERCY_GRANT_LEVEL: u8 = 9;

pub(super) const PALADIN_THIRD_MERCY_CHOICE_ID: &str = "choice:paladin_mercy_3";

/// PF1 Core Rulebook gate of the paladin's FOURTH mercy ("every three levels
/// thereafter" from the level-3 mercy rule: 3, 6, 9, 12). The CRB 12th-level
/// tier was verified for this SD18 slice: legacy.aonprd.com (Core Rulebook
/// only) ADDS Blinded/Deafened/Paralyzed/Stunned; d20pfsrd's list is a
/// superset whose CRB subset matches exactly -- its extra entries
/// (Amputated, Ensorcelled, Petrified) are non-CRB expansions outside this
/// seam's pf1.core_rulebook source package. Numbered slot per the proven
/// repeat-grant idiom, mirroring slot 2/3 exactly.
pub(super) const PALADIN_FOURTH_MERCY_GRANT_LEVEL: u8 = 12;

pub(super) const PALADIN_FOURTH_MERCY_CHOICE_ID: &str = "choice:paladin_mercy_4";

/// PF1 Core Rulebook gate of the paladin's FIFTH mercy ("every three levels
/// thereafter" from the level-3 mercy rule: 3, 6, 9, 12, 15). Verified
/// independently for this SD18 slice against d20pfsrd and the Archives of
/// Nethys aonprd.com mirror (byte-for-byte agreement): UNLIKE the
/// 6th/9th/12th-level repeat grants, no new named mercy-list tier is added
/// at 15th level -- both sources agree the CRB's named mercy conditions
/// stop growing after the 12th-level tier, so the 15th-level grant is
/// simply a fifth pick from the already-existing 3rd/6th/9th/12th-tier
/// pool. Numbered slot per the proven repeat-grant idiom, mirroring slot
/// 2/3/4 exactly.
pub(super) const PALADIN_FIFTH_MERCY_GRANT_LEVEL: u8 = 15;

pub(super) const PALADIN_FIFTH_MERCY_CHOICE_ID: &str = "choice:paladin_mercy_5";

/// PF1 Core Rulebook gate of the paladin's SIXTH mercy ("every three levels
/// thereafter" from the level-3 mercy rule: 3, 6, 9, 12, 15, 18). Verified
/// independently for this SD18 slice against d20pfsrd and the Archives of
/// Nethys aonprd.com mirror (byte-for-byte agreement): like the 15th-level
/// grant, no new named mercy-list tier is added at 18th level -- both
/// sources agree the CRB's named mercy conditions stop growing after the
/// 12th-level tier, so the 18th-level grant is simply a sixth pick from the
/// already-existing 3rd/6th/9th/12th-tier pool. Numbered slot per the
/// proven repeat-grant idiom, mirroring slot 2/3/4/5 exactly.
pub(super) const PALADIN_SIXTH_MERCY_GRANT_LEVEL: u8 = 18;

pub(super) const PALADIN_SIXTH_MERCY_CHOICE_ID: &str = "choice:paladin_mercy_6";

// SD13-E5 Ranger Combat Style correction. Combat Style Feat is a 2nd-level ranger
// feature in the PF1 Core Rulebook: the ranger selects a combat style (archery or
// two-weapon combat) and gains its first bonus feat TOGETHER at 2nd level -- these
// are not separable into a level-1 style choice plus a level-2 feat grant, as an
// earlier version of the Ranger combat-style diagnostic incorrectly claimed. Below
// this gate, combat style is always a correct ABSENCE, mirroring PALADIN_MERCY_LEVEL;
// at or above it (SD13-E5 level-2 widening), the style choice and its bonus feat are
// finally grounded for real as recognition records -- see
// RANGER_COMBAT_STYLE_CHOICE_ID below.
pub(super) const RANGER_COMBAT_STYLE_LEVEL: u8 = 2;

// SD13-E5 Ranger level-range widening. The accepted level-1 Ranger per-pillar
// decomposition (base attack/base save progression, Track, the Favored Enemy flat
// surface, and the combat-style level-gate absence) is joined by level 2, the PF1
// Core Rulebook level gate at which Combat Style Feat is actually granted, by
// level 3 (SD13-E5), the level gate at which Endurance and Favored Terrain are
// granted, by level 4 (SD13-E5), the level gate at which Hunter's Bond is
// granted, by level 5 (SD13-E5), the level gate at which the Favored Enemy
// rule's own 5th-level interval (a second favored enemy plus a +2 bonus increase
// to any one favored enemy of the ranger's choice) is granted, by level 6
// (SD13-E5), the level gate at which the ranger's SECOND combat-style bonus feat
// is granted (verified independently against d20pfsrd and legacy.aonprd.com: both
// state "The ranger's expertise manifests in the form of bonus feats at 2nd, 6th,
// 10th, 14th, and 18th level" -- 6th level is the very next milestone after 2nd,
// not 3rd/4th/5th as some earlier framings assumed), and by level 7 (SD13-E5),
// the level gate at which Woodland Stride is granted (verified independently
// against d20pfsrd and legacy.aonprd.com: both list "Woodland stride" as the
// Ranger 7th-level "Special" column entry, and both state the exact rule text,
// "a ranger may move through any sort of undergrowth ... at his normal speed and
// without taking damage or suffering any other impairment ... magically
// manipulated undergrowth ... still affects him normally" -- an automatic,
// no-choice, no-numeric-magnitude grant, grounded as a bounded identity record
// only, mirroring the Endurance grant-only idiom). Neither the Favored Enemy
// rule's next interval (10th level) nor the Combat Style Feat's next bonus feat
// (10th level) is reached at level 7, so both stay unchanged, re-verified rather
// than assumed. A still later SD13-E5 slice widens the gate once more to level 8,
// the level gate at which Swift Tracker is granted (verified independently
// against d20pfsrd and legacy.aonprd.com: both list the level-8 "Special" column
// as naming TWO entries, "Swift tracker" and "2nd favored terrain"). Swift
// Tracker ("a ranger can move at his normal speed while using Survival to follow
// tracks without taking the normal -5 penalty. He takes only a -10 penalty
// (instead of the normal -20) when moving at up to twice normal speed while
// tracking") only modifies a tracking-while-moving penalty resolution that does
// not exist anywhere in this codebase (this codebase grounds only the flat Track
// skill-bonus magnitude, never a check-execution/movement-penalty engine), so it
// is a genuinely flat/identity-shaped, no-choice, no-magnitude grant, grounded
// as a bounded identity record only, mirroring the Woodland Stride grant-only
// idiom exactly. The level-8 "2nd favored terrain" entry mirrors the Favored
// Enemy 5th-level idiom already grounded in this codebase (a second
// terrain-type selection plus a bonus-increase-target choice), but is a
// multi-record burden of its own -- deliberately left named-but-unproven this
// slice, a real newly discovered burden for a future slice, not an invented
// one. A further SD13-E5 slice widens the gate to level 9 (verified
// independently against d20pfsrd and legacy.aonprd.com): level 9 base attack
// genuinely rises to +9 (full BAB) and poor Will genuinely rises to +3
// (9 / 3), while both good saves stay +6 (9 / 2 + 2, integer-division
// coincidences); Track stays 4 (max(9/2, 1), a coincidence); the
// favored-enemy/favored-terrain/hunter's-bond facets all carry over unchanged
// (the next favored-enemy grant lands at 10th, the next favored-terrain grant
// at 13th, both checked rather than assumed); the level-9 "Special" column
// reads "Evasion" — a genuinely NEW class feature, grounded as a +0
// identity/recognition record only (RANGER_EVASION_LEVEL), mirroring Rogue's
// and Monk's own Evasion records; no damage-resolution engine exists here, so
// no damage math is fabricated from it. A further SD13-E5 slice widens the
// gate to level 10 — the tranche ceiling (verified independently against
// d20pfsrd and legacy.aonprd.com): level 10 base attack genuinely rises to
// +10 (full BAB) and both good saves genuinely rise to +7 (10 / 2 + 2),
// while poor Will stays +3 (10 / 3, a coincidence); Track genuinely rises to
// 5 (max(10/2, 1)); the level-10 "Special" column reads "3rd favored enemy,
// combat style feat": the THIRD combat-style bonus feat is grounded as a
// restricted-list choice recognition mirroring the 2nd/6th grants exactly
// (RANGER_COMBAT_STYLE_BONUS_FEAT_3_LEVEL; Archery adds Pinpoint Targeting
// and Shot on the Run at 10th, Two-Weapon Combat adds Greater Two-Weapon
// Fighting and Two-Weapon Rend), while the "3rd favored enemy" interval (a
// third enemy-type selection PLUS the rule's own second +2
// bonus-increase-target choice) is a real, newly-discovered multi-record
// burden deliberately left named-but-unproven this slice, mirroring the
// level-8 2nd-favored-terrain deferral precedent exactly.
//
// SD18 cycle-2026-07-14T2300 widens the gate once more to level 11,
// extending base attack/base save/Track to level 11 via the same formulas
// (all three stay numerically unchanged from level 10 -- 11/2+2 = 7,
// 11/3 = 3, max(11/2, 1) = 5 -- integer-division coincidences, re-verified
// against d20pfsrd and the Archives of Nethys aonprd.com mirror rather than
// assumed) and grounds the class table's 11th-level "Special" column entry,
// "Quarry" (verified independently against both primary sources -- no other
// new class feature is gained at 11th level). Quarry is grounded as a
// bundle mirroring precedent exactly: a grant-only identity record (value
// 0, mirroring the Woodland Stride/Swift Tracker idiom) for the
// take-10-while-tracking and auto-confirm-critical-threats behaviors, since
// neither a Survival-check-execution engine nor a critical-confirmation-roll
// engine exists anywhere in this codebase; an open-ended target-choice
// recognition record (mirroring the Favored Enemy/Favored Terrain
// choice-recognition idiom exactly, no restricted-list validation, no
// favored-enemy-type matching); and the rule's own flat +2 insight
// attack-roll magnitude as a standalone, non-applied record (mirroring the
// Favored Enemy attack/damage-bonus idiom exactly). No active-quarry state
// (the 24-hour reselection cooldown, the 1-hour post-kill cooldown, or "only
// one quarry at a time") is tracked. A still later SD18 slice widens the
// gate once more to level 12, extending base attack/base save/Track to
// level 12 via the same formulas (all three genuinely rise, unlike level
// 11's integer-division coincidences) and grounds Camouflage, the class
// table's 12th-level "Special" column entry (`RANGER_CAMOUFLAGE_LEVEL`), as
// a grant-only identity record. A still later SD18 slice widens the gate
// once more to level 13, extending base attack to level 13 (base saves stay
// numerically unchanged from level 12, an integer-division coincidence) and
// grounds the class table's 13th-level "Special" column entry, "3rd favored
// terrain" (`RANGER_FAVORED_TERRAIN_THIRD_INTERVAL_LEVEL`) — the exact
// structural mirror of the already-grounded Favored Enemy 10th-level
// interval — plus the spell-level access ladder's genuinely new 4th-level
// column (`RANGER_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL`) and the base
// spells-per-day table's own level-13 row. A still later SD18 slice
// (cycle-2026-07-15T2100) widens the gate once more to level 14: base
// attack bonus and both good saves genuinely rise, the base spells-per-day
// table's 4th-level column genuinely rises from 0 to 1, and the level-14
// "Special" column's FOURTH combat-style bonus feat
// (`RANGER_COMBAT_STYLE_BONUS_FEAT_4_LEVEL`) is grounded as an open-ended
// +0 recognition record, NOT a restricted-list match — the PF1 Core
// Rulebook's own Combat Style feat lists (Archery, Two-Weapon Combat) do
// not tabulate any named options beyond the 10th-level tier, verified
// independently against three sources dedicated to the combat-style feat
// lists themselves. A still later SD18 slice (cycle-2026-07-15T4000) widens
// the gate once more to level 15: base attack bonus genuinely rises to 15
// (full BAB) while both good saves stay 9 (15/2+2, an integer-division
// coincidence with level 14) and poor Will genuinely rises to 5 (15/3, up
// from 4); the level-15 "Special" column reads "4th favored enemy" — the
// Favored Enemy rule's own 15th-level interval, the exact structural mirror
// of the already-grounded 10th-level interval — plus the base
// spells-per-day table's own level-15 row (3/2/2/1, the 3rd-level column
// genuinely rising from 1 to 2). A still later SD18 slice
// (cycle-2026-07-15T6100) widens the gate once more to level 16: base
// attack bonus genuinely rises to 16 (full BAB) and both good saves
// genuinely rise to 10 (16/2+2), while poor Will stays 5 (16/3, an
// integer-division coincidence); the level-16 "Special" column reads
// "Improved evasion" (`RANGER_IMPROVED_EVASION_LEVEL`) — grounded as a
// bounded +0 identity/recognition record, mirroring Monk's own Improved
// Evasion and Ranger's own base Evasion idiom exactly — plus the base
// spells-per-day table's own level-16 row (3/3/2/1, the 1st-level column
// genuinely rising from 2 to 3). A still later SD18 slice
// (cycle-2026-07-15T7000) widens the gate once more to level 17: base
// attack bonus genuinely rises to 17 (full BAB), while both good saves
// stay 10 (17/2+2) and poor Will stays 5 (17/3), both integer-division
// coincidences with level 16; the level-17 "Special" column reads "Hide in
// plain sight" (`RANGER_HIDE_IN_PLAIN_SIGHT_LEVEL`) — grounded as a
// bounded +0 identity/recognition record, mirroring Camouflage's own
// idiom exactly — plus the base spells-per-day table's own level-17 row
// (4/3/2/1, the 1st-level column genuinely rising from 3 to 4). A still
// later SD18 slice (cycle-2026-07-16T0244) widens the gate once more to
// level 18: base attack bonus genuinely rises to 18 (full BAB), and ALL
// THREE base saves genuinely rise this time (good Fortitude/Reflex to 11,
// `18/2+2`; poor Will to 6, `18/3`) — unlike level 17's all-coincidence
// row; the level-18 "Special" column reads "4th favored terrain, combat
// style feat" (verified independently against d20pfsrd and the Archives
// of Nethys aonprd.com mirror, byte-for-byte agreement, confirming the
// prior cycle's own carried-forward hypothesis exactly): the FOURTH
// Favored Terrain interval (`RANGER_FAVORED_TERRAIN_FOURTH_INTERVAL_LEVEL`,
// the exact structural mirror of the already-grounded Favored Enemy
// 15th-level interval) plus the FIFTH combat-style bonus feat
// (`RANGER_COMBAT_STYLE_BONUS_FEAT_5_LEVEL`), grounded as an open-ended +0
// recognition record mirroring the fourth bonus feat's own idiom exactly
// (the PF1 Core Rulebook's own Combat Style feat tables do not tabulate
// any named options beyond the 10th-level tier) — plus the base
// spells-per-day table's own level-18 row (4/3/2/2, the 4th-level column
// genuinely rising from 1 to 2, numerically identical to the already-
// landed Paladin level-18 row). A still later SD18 slice
// (cycle-2026-07-16T3200) widens the gate once more to level 19: base
// attack bonus genuinely rises to 19 (full BAB), while both good saves
// stay 11 (19/2+2) and poor Will stays 6 (19/3), both integer-division
// coincidences with level 18; the level-19 "Special" column reads
// "Improved quarry" (verified independently against d20pfsrd and the
// Archives of Nethys aonprd.com mirror, both byte-for-byte identical) —
// an UPGRADE of the already-grounded 11th-level Quarry identity
// (`RANGER_QUARRY_LEVEL`), the exact structural mirror of Improved
// Evasion's own upgrade of Evasion: a new bounded grant-only identity
// record (`RANGER_IMPROVED_QUARRY_LEVEL`) names the free-action
// reselection, take-20-while-tracking, and reduced 10-minute cooldown
// upgrades, while the already-existing `quarry_attack_bonus` explanation
// genuinely rises from +2 to +4 on the same id, mirroring the Bard
// Inspire Competence tiered-magnitude idiom — plus the base
// spells-per-day table's own level-19 row (4/3/3/2, the 3rd-level column
// genuinely rising from 2 to 3). A still later SD18 slice
// (cycle-2026-07-16T1600) widens the gate once more to level 20, the
// FINAL level within PF1's 1-20 character-level cap: base attack bonus
// genuinely rises to 20 (full BAB) and both good saves genuinely rise to
// 12 (20/2+2), while poor Will stays 6 (20/3, an integer-division
// coincidence with level 19); the level-20 "Special" column reads "5th
// favored enemy, master hunter" (verified independently against d20pfsrd
// and the Archives of Nethys aonprd.com mirror, both byte-for-byte
// identical) — the Favored Enemy rule's own FINAL 20th-level interval
// (`RANGER_FAVORED_ENEMY_FIFTH_INTERVAL_LEVEL`), the exact structural
// mirror of the 15th-level interval, plus Master Hunter
// (`RANGER_MASTER_HUNTER_LEVEL`), a brand-new capstone with no player
// choice involved, grounded as a bounded grant-only identity record
// mirroring the Paladin Holy Champion idiom — plus the base
// spells-per-day table's own level-20 row (4/4/3/3, the 2nd- and
// 4th-level columns both genuinely rising at once). This closes Ranger's
// own per-level arithmetic-widening frontier.
pub(super) const MAX_SUPPORTED_RANGER_LEVEL: u8 = 20;

/// PF1 Core Rulebook level gate at which Camouflage is granted (verified
/// independently against two primary sources: both d20pfsrd and the
/// Archives of Nethys aonprd.com mirror list "Camouflage" as the sole
/// Ranger 12th-level "Special" column entry). Camouflage is an automatic,
/// no-choice grant with no numeric magnitude of its own: "A ranger of 12th
/// level or higher can use the Stealth skill to hide, even while being
/// observed, as long as she is within any sort of natural terrain that
/// grants at least partial concealment or partial cover." Only the grant
/// identity is grounded here; no terrain-detection engine and no
/// Stealth-check-execution engine exists anywhere in this codebase.
pub(super) const RANGER_CAMOUFLAGE_LEVEL: u8 = 12;

/// PF1 Core Rulebook level gate at which Woodland Stride is granted (verified
/// independently against two primary sources: both d20pfsrd and
/// legacy.aonprd.com list "Woodland stride" as the Ranger 7th-level "Special"
/// column entry, with no other new class feature named at 7th level). Woodland
/// Stride is an automatic, no-choice grant with no numeric magnitude of its own:
/// "a ranger may move through any sort of undergrowth (such as natural thorns,
/// briars, overgrown areas, and similar terrain) at his normal speed and without
/// taking damage or suffering any other impairment. However, magically
/// manipulated undergrowth still affects him normally." No terrain-detection or
/// movement-resolution engine exists in this codebase, so only the grant
/// identity itself is grounded.
pub(super) const RANGER_WOODLAND_STRIDE_LEVEL: u8 = 7;

/// PF1 Core Rulebook level gate at which Swift Tracker is granted (verified
/// independently against two primary sources: both d20pfsrd and
/// legacy.aonprd.com list "Swift tracker" as one of two Ranger 8th-level
/// "Special" column entries, alongside "2nd favored terrain"). Swift Tracker is
/// an automatic, no-choice grant with no numeric magnitude of its own: "a
/// ranger can move at his normal speed while using Survival to follow tracks
/// without taking the normal -5 penalty. He takes only a -10 penalty (instead
/// of the normal -20) when moving at up to twice normal speed while tracking."
/// No tracking-while-moving check-execution/movement-penalty engine exists in
/// this codebase (this codebase grounds only the flat Track skill-bonus
/// magnitude), so only the grant identity itself is grounded, mirroring the
/// Woodland Stride idiom exactly.
pub(super) const RANGER_SWIFT_TRACKER_LEVEL: u8 = 8;

/// PF1 Core Rulebook level gate at which Ranger gains Evasion (9th level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Evasion" as the Ranger 9th-level "Special"
/// column entry — the same rule text as Rogue's and Monk's own Evasion).
pub(super) const RANGER_EVASION_LEVEL: u8 = 9;

/// PF1 Core Rulebook level gate at which Ranger gains Improved Evasion (16th
/// level, verified independently against two primary sources: d20pfsrd and
/// the Archives of Nethys aonprd.com mirror both list "Improved evasion" as
/// the Ranger 16th-level "Special" column entry, byte-for-byte agreement).
/// An upgrade of the 9th-level Evasion identity: the ranger still takes no
/// damage on a successful Reflex save, and henceforth takes only HALF
/// damage on a failed save. Grounded as a bounded +0 identity/recognition
/// record only at/above the gate, mirroring exactly how Monk's own
/// Improved Evasion (`MONK_IMPROVED_EVASION_LEVEL`) and Ranger's own base
/// Evasion (`RANGER_EVASION_LEVEL`) were grounded — no
/// saving-throw-resolution or damage-resolution engine exists anywhere in
/// this codebase, so no damage math is fabricated from the record.
pub(super) const RANGER_IMPROVED_EVASION_LEVEL: u8 = 16;

/// PF1 Core Rulebook level gate at which Ranger gains Hide in Plain Sight
/// (17th level, verified independently against three primary sources:
/// d20pfsrd, the Archives of Nethys aonprd.com mirror, and
/// legacy.aonprd.com all list "Hide in plain sight" as the sole Ranger
/// 17th-level "Special" column entry, byte-for-byte agreement). Hide in
/// Plain Sight carries no numeric magnitude of its own and only modifies a
/// hide-while-observed check resolution that does not exist anywhere in
/// this codebase -- exactly like Camouflage
/// (`RANGER_CAMOUFLAGE_LEVEL`), it is a genuinely flat/identity-shaped,
/// no-choice, no-magnitude grant: "While in any of his favored terrains, a
/// ranger of 17th level or higher can use the Stealth skill even while
/// being observed."
pub(super) const RANGER_HIDE_IN_PLAIN_SIGHT_LEVEL: u8 = 17;

/// PF1 Core Rulebook level gate at which Ranger gains Quarry (11th level,
/// verified independently against two primary sources: d20pfsrd and the
/// Archives of Nethys aonprd.com mirror both list "Quarry" as the sole
/// Ranger 11th-level "Special" column entry): "At 11th level, a ranger can
/// select one target within line of sight as his quarry... While tracking
/// his quarry, a ranger can take 10 on his Survival skill checks while
/// moving at normal speed without penalty. In addition, the ranger receives
/// a +2 insight bonus on attack rolls made against his quarry, and he
/// confirms all critical threats against the quarry automatically... Once a
/// ranger has selected a quarry, he cannot select a different quarry until
/// 24 hours have passed or the current quarry is dead." Only the flat
/// magnitude and the grant identity are grounded here; the reselection
/// cooldown state and any check/roll-resolution engine are not.
pub(super) const RANGER_QUARRY_LEVEL: u8 = 11;

/// The open-ended chosen-input identity naming which target the ranger has
/// designated as his quarry (PF1 Core Rulebook: "must correspond to one of
/// his favored enemy types"). Mirrors the Favored Enemy/Favored Terrain
/// choice-recognition idiom exactly: raw string interpolation, no
/// restricted-list validation, and no matching against the ranger's own
/// recognized favored-enemy types.
pub(super) const RANGER_QUARRY_CHOICE_ID: &str = "choice:ranger_quarry_target";

/// PF1 Core Rulebook level gate at which Quarry improves (19th level,
/// verified independently against two primary sources: d20pfsrd and the
/// Archives of Nethys aonprd.com mirror both list "Improved quarry" as
/// the sole Ranger 19th-level "Special" column entry, with identical rule
/// text): "At 19th level, the ranger's ability to hunt his quarry
/// improves. He can now select a quarry as a free action, and can now
/// take 20 while using Survival to track his quarry, while moving at
/// normal speed without penalty. His insight bonus to attack his quarry
/// increases to +4. If his quarry is killed or dismissed, he can select
/// a new one after 10 minutes have passed." This is the exact structural
/// mirror of Improved Evasion's own upgrade of Evasion: the free-action
/// reselection, take-20-while-tracking, and reduced-cooldown behaviors
/// are grounded as a bounded grant-only identity record (mirroring the
/// Improved Evasion idiom exactly, pushed only at/above this gate with
/// no separate absence record below it); the insight attack-roll bonus
/// increase is grounded by widening the already-existing
/// `quarry_attack_bonus` magnitude on the same explanation id, mirroring
/// the Bard Inspire Competence tiered-magnitude idiom.
pub(super) const RANGER_IMPROVED_QUARRY_LEVEL: u8 = 19;

/// PF1 Core Rulebook level gate at which the Favored Enemy rule's 5th-level
/// interval is granted (verified independently against two primary sources:
/// both d20pfsrd and legacy.aonprd.com list "2nd favored enemy" as the Ranger
/// 5th-level "Special" column entry, and both state the exact rule text: "At
/// 5th level and every five levels thereafter (10th, 15th, and 20th level), the
/// ranger may select an additional favored enemy. In addition, at each such
/// interval, the bonus against any one favored enemy (including the one just
/// selected, if so desired) increases by 2." This is genuinely two things at
/// once: a second favored-enemy TYPE selection (open-ended, mirroring the first
/// favored enemy's own choice-slot idiom) and a separate, independent choice of
/// WHICH one favored enemy (the newly selected one or an already-held one)
/// receives the +2 magnitude increase -- it is NOT an automatic bump to the
/// first favored enemy, so this slice grounds the target as its own restricted
/// two-option choice-slot (mirroring the Hunter's Bond/combat-style restricted
/// two-option idiom) rather than assuming a specific outcome.
pub(super) const RANGER_FAVORED_ENEMY_SECOND_INTERVAL_LEVEL: u8 = 5;

/// SD13-E5 Ranger second favored-enemy choice-slot id. The deterministic fixture
/// names a second favored-enemy type (e.g. `enemy:undead`); the compute seam
/// recognizes whichever raw enemy-type string was actually selected, mirroring
/// `choice:ranger_favored_enemy`'s open-ended (non-restricted-list) recognition
/// idiom exactly.
pub(super) const RANGER_FAVORED_ENEMY_SECOND_CHOICE_ID: &str = "choice:ranger_favored_enemy_2";

/// SD13-E5 Ranger favored-enemy bonus-increase target choice-slot id. Names
/// which ONE of the (now two) favored enemies receives the rule's own +2
/// magnitude increase at the 5th-level interval -- a restricted two-option
/// choice (`enemy:first` or `enemy:second`), mirroring the Hunter's Bond
/// restricted two-option choice idiom exactly (unlike the open-ended favored
/// enemy TYPE choice-slots themselves).
pub(super) const RANGER_FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID: &str =
    "choice:ranger_favored_enemy_bonus_increase_target";
pub(super) const RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION: &str = "enemy:first";

pub(super) const RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION: &str = "enemy:second";

/// PF1 Core Rulebook level gate of the Favored Enemy rule's SECOND interval
/// (verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both state "At 5th level and every five levels
/// thereafter (10th, 15th, and 20th level), the ranger may select an
/// additional favored enemy. In addition, at each such interval, the bonus
/// against any one favored enemy (including the one just selected, if so
/// desired) increases by +2." — each interval carries its OWN
/// bonus-increase-target choice). The 10th-level interval is grounded here;
/// the 15th-level interval is grounded by a later SD18 slice (see
/// `RANGER_FAVORED_ENEMY_FOURTH_INTERVAL_LEVEL`); the 20th-level interval
/// stays out of scope.
pub(super) const RANGER_FAVORED_ENEMY_THIRD_INTERVAL_LEVEL: u8 = 10;

/// SD13-E5 Ranger THIRD Favored Enemy choice-slot id, mirroring
/// `choice:ranger_favored_enemy_2`'s open-ended (non-restricted-list)
/// recognition idiom exactly.
pub(super) const RANGER_FAVORED_ENEMY_THIRD_CHOICE_ID: &str = "choice:ranger_favored_enemy_3";

/// SD13-E5 Ranger 10th-level-interval bonus-increase TARGET choice-slot id,
/// mirroring `choice:ranger_favored_enemy_bonus_increase_target`'s
/// restricted idiom, widened to the three-enemy set (`enemy:first` /
/// `enemy:second` / `enemy:third`); any other selection is surfaced without
/// grounding a target identity and no boost is fabricated from it.
pub(super) const RANGER_FAVORED_ENEMY_SECOND_BONUS_INCREASE_CHOICE_ID: &str =
    "choice:ranger_favored_enemy_bonus_increase_target_2";
pub(super) const RANGER_FAVORED_ENEMY_BONUS_INCREASE_THIRD_SELECTION: &str = "enemy:third";

/// PF1 Core Rulebook level gate of the Favored Enemy rule's FOURTH interval
/// (verified independently against two primary sources: d20pfsrd and the
/// Archives of Nethys aonprd.com mirror, both agreeing byte-for-byte on the
/// class table's level-15 "Special" column entry, "4th favored enemy", and
/// on the rule's own text: "At 5th level and every five levels thereafter
/// (10th, 15th, and 20th level), the ranger may select an additional
/// favored enemy. In addition, at each such interval, the bonus against any
/// one favored enemy (including the one just selected, if so desired)
/// increases by +2." — each interval carries its OWN bonus-increase-target
/// choice). The 15th-level interval is grounded here; the 20th-level
/// interval stays out of scope.
pub(super) const RANGER_FAVORED_ENEMY_FOURTH_INTERVAL_LEVEL: u8 = 15;

/// SD18 Ranger FOURTH Favored Enemy choice-slot id, mirroring
/// `choice:ranger_favored_enemy_3`'s open-ended (non-restricted-list)
/// recognition idiom exactly.
pub(super) const RANGER_FAVORED_ENEMY_FOURTH_CHOICE_ID: &str = "choice:ranger_favored_enemy_4";

/// SD18 Ranger 15th-level-interval bonus-increase TARGET choice-slot id,
/// mirroring `choice:ranger_favored_enemy_bonus_increase_target_2`'s
/// restricted idiom, widened to the four-enemy set (`enemy:first` /
/// `enemy:second` / `enemy:third` / `enemy:fourth`); any other selection is
/// surfaced without grounding a target identity and no boost is fabricated
/// from it.
pub(super) const RANGER_FAVORED_ENEMY_THIRD_BONUS_INCREASE_CHOICE_ID: &str =
    "choice:ranger_favored_enemy_bonus_increase_target_3";
pub(super) const RANGER_FAVORED_ENEMY_BONUS_INCREASE_FOURTH_SELECTION: &str = "enemy:fourth";

/// PF1 Core Rulebook level gate of the Favored Enemy rule's FIFTH and FINAL
/// interval (verified independently against two primary sources: d20pfsrd
/// and the Archives of Nethys aonprd.com mirror, both agreeing byte-for-byte
/// on the class table's level-20 "Special" column entry, "5th favored
/// enemy, master hunter", and on the rule's own text: "At 5th level and
/// every five levels thereafter (10th, 15th, and 20th level), the ranger
/// may select an additional favored enemy. In addition, at each such
/// interval, the bonus against any one favored enemy... increases by +2."
/// — this is the last interval within PF1's 1-20 character-level cap).
pub(super) const RANGER_FAVORED_ENEMY_FIFTH_INTERVAL_LEVEL: u8 = 20;

/// SD18 Ranger FIFTH Favored Enemy choice-slot id, mirroring
/// `choice:ranger_favored_enemy_4`'s open-ended (non-restricted-list)
/// recognition idiom exactly.
pub(super) const RANGER_FAVORED_ENEMY_FIFTH_CHOICE_ID: &str = "choice:ranger_favored_enemy_5";

/// SD18 Ranger 20th-level-interval bonus-increase TARGET choice-slot id,
/// mirroring `choice:ranger_favored_enemy_bonus_increase_target_3`'s
/// restricted idiom, widened to the five-enemy set (`enemy:first` /
/// `enemy:second` / `enemy:third` / `enemy:fourth` / `enemy:fifth`); any
/// other selection is surfaced without grounding a target identity and no
/// boost is fabricated from it.
pub(super) const RANGER_FAVORED_ENEMY_FOURTH_BONUS_INCREASE_CHOICE_ID: &str =
    "choice:ranger_favored_enemy_bonus_increase_target_4";
pub(super) const RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIFTH_SELECTION: &str = "enemy:fifth";

/// PF1 Core Rulebook level gate at which Ranger becomes a Master Hunter
/// (verified independently against two primary sources: d20pfsrd and the
/// Archives of Nethys aonprd.com mirror, both byte-for-byte identical: "A
/// ranger of 20th level becomes a master hunter. He can always move at
/// full speed while using Survival to follow tracks without penalty. He
/// can, as a standard action, make a single attack against a favored enemy
/// at his full attack bonus. If the attack hits, the target takes damage
/// normally and must make a Fortitude save or die..."). This is the
/// Ranger's 20th-level capstone, mirroring the Paladin Holy Champion
/// capstone idiom exactly (a bounded grant-only identity record; no
/// action-economy engine, no attack-resolution engine, and no
/// saving-throw-resolution engine exists anywhere in this codebase to
/// apply any of this to).
pub(super) const RANGER_MASTER_HUNTER_LEVEL: u8 = 20;

/// PF1 Core Rulebook level gate at which Ranger gains Endurance (3rd level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Endurance, favored terrain" as the Ranger
/// 3rd-level special feature entry). Endurance is a bonus feat granted
/// automatically, with no player choice involved ("A ranger gains Endurance as a
/// bonus feat at 3rd level"), so it is grounded as a bounded grant-only identity
/// record, mirroring the Wizard Scribe Scroll / Barbarian Uncanny Dodge idiom.
pub(super) const RANGER_ENDURANCE_LEVEL: u8 = 3;

/// PF1 Core Rulebook level gate at which Ranger gains Favored Terrain (3rd level,
/// the same gate as Endurance -- both are the two named entries in the class
/// table's 3rd-level "Special" column, verified independently against two primary
/// sources: d20pfsrd and legacy.aonprd.com both list "Endurance, favored terrain"
/// and both state the exact bonus text: "+2 bonus on Initiative checks and
/// Knowledge (geography), Perception, Stealth, and Survival skill checks" made
/// when the ranger is in the chosen terrain, selected from Table: Ranger Favored
/// Terrains' fixed eleven-entry list (Cold, Desert, Forest, Jungle, Mountain,
/// Plains, Planes, Swamp, Underground, Urban, Water). Unlike Endurance, Favored
/// Terrain is a genuine player choice, so this slice grounds a choice-slot
/// recognition record (naming whichever terrain was selected, mirroring the
/// Favored Enemy choice-recognition idiom exactly) plus the rule's own flat +2
/// magnitude, grounded as a standalone, non-applied record -- no
/// terrain-detection engine decides whether the character is actually in the
/// chosen terrain, and the +2 is never wired into any actual Initiative total or
/// skill-check total. The 8th-level additional-terrain and bonus-increase
/// interval is grounded by a later SD13-E5 slice (see
/// `RANGER_FAVORED_TERRAIN_SECOND_INTERVAL_LEVEL`), and the 13th-level
/// interval is grounded by a still later SD18 slice (see
/// `RANGER_FAVORED_TERRAIN_THIRD_INTERVAL_LEVEL`), and the 18th-level
/// interval is grounded by a still later SD18 slice (see
/// `RANGER_FAVORED_TERRAIN_FOURTH_INTERVAL_LEVEL`).
pub(super) const RANGER_FAVORED_TERRAIN_LEVEL: u8 = 3;

/// SD13-E5 Ranger Favored Terrain choice-slot id. The deterministic fixture names
/// a chosen terrain (e.g. `terrain:forest`); the compute seam recognizes whichever
/// raw terrain string was actually selected, mirroring
/// `choice:ranger_favored_enemy`'s open-ended (non-restricted-list) recognition
/// idiom exactly -- no enum validation against the Table: Ranger Favored Terrains
/// list is performed here.
pub(super) const RANGER_FAVORED_TERRAIN_CHOICE_ID: &str = "choice:ranger_favored_terrain";

/// PF1 Core Rulebook level gate at which Ranger gains an additional favored
/// terrain (verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both state "At 8th level and every five levels
/// thereafter, the ranger may select an additional favored terrain. In
/// addition, at each such interval, the skill bonus and initiative bonus in
/// any one favored terrain (including the one just selected, if so desired),
/// increases by +2." — the exact structural mirror of the Favored Enemy
/// 5th-level interval already grounded on this seam). The 8th-level,
/// 13th-level, and 18th-level intervals are all grounded (see
/// `RANGER_FAVORED_TERRAIN_THIRD_INTERVAL_LEVEL` and
/// `RANGER_FAVORED_TERRAIN_FOURTH_INTERVAL_LEVEL`).
pub(super) const RANGER_FAVORED_TERRAIN_SECOND_INTERVAL_LEVEL: u8 = 8;

/// SD13-E5 Ranger SECOND Favored Terrain choice-slot id, mirroring
/// `choice:ranger_favored_enemy_2`'s open-ended (non-restricted-list)
/// recognition idiom exactly — no enum validation against Table: Ranger
/// Favored Terrains is performed here.
pub(super) const RANGER_FAVORED_TERRAIN_SECOND_CHOICE_ID: &str = "choice:ranger_favored_terrain_2";

/// SD13-E5 Ranger Favored Terrain bonus-increase TARGET choice-slot id,
/// mirroring `choice:ranger_favored_enemy_bonus_increase_target`'s
/// restricted-pair idiom exactly: only `terrain:first` / `terrain:second`
/// are recognized; any other selection is surfaced without grounding a
/// target identity and no boost is fabricated from it.
pub(super) const RANGER_FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID: &str =
    "choice:ranger_favored_terrain_bonus_increase_target";
pub(super) const RANGER_FAVORED_TERRAIN_BONUS_INCREASE_FIRST_SELECTION: &str = "terrain:first";

pub(super) const RANGER_FAVORED_TERRAIN_BONUS_INCREASE_SECOND_SELECTION: &str = "terrain:second";

/// PF1 Core Rulebook level gate at which Ranger gains a THIRD additional
/// favored terrain (verified independently against three primary sources —
/// d20pfsrd, the Archives of Nethys aonprd.com mirror, and legacy.aonprd.com
/// all agree byte-for-byte): "At 8th level and every five levels thereafter,
/// the ranger may select an additional favored terrain. In addition, at each
/// such interval, the skill bonus and initiative bonus in any one favored
/// terrain (including the one just selected, if so desired), increases by
/// +2." The 13th-level interval (8 + 5) is the exact structural mirror of
/// the already-grounded Favored Enemy 10th-level interval
/// (`RANGER_FAVORED_ENEMY_THIRD_INTERVAL_LEVEL`). The 18th-level interval
/// is grounded by a still later SD18 slice (see
/// `RANGER_FAVORED_TERRAIN_FOURTH_INTERVAL_LEVEL`).
pub(super) const RANGER_FAVORED_TERRAIN_THIRD_INTERVAL_LEVEL: u8 = 13;

/// SD18 Ranger THIRD Favored Terrain choice-slot id, mirroring
/// `choice:ranger_favored_enemy_3`'s open-ended (non-restricted-list)
/// recognition idiom exactly -- no enum validation against Table: Ranger
/// Favored Terrains is performed here.
pub(super) const RANGER_FAVORED_TERRAIN_THIRD_CHOICE_ID: &str = "choice:ranger_favored_terrain_3";

/// SD18 Ranger 13th-level-interval bonus-increase TARGET choice-slot id,
/// mirroring `choice:ranger_favored_enemy_bonus_increase_target_2`'s
/// restricted idiom, widened to the three-terrain set (`terrain:first` /
/// `terrain:second` / `terrain:third`); any other selection is surfaced
/// without grounding a target identity and no boost is fabricated from it.
pub(super) const RANGER_FAVORED_TERRAIN_SECOND_BONUS_INCREASE_CHOICE_ID: &str =
    "choice:ranger_favored_terrain_bonus_increase_target_2";
pub(super) const RANGER_FAVORED_TERRAIN_BONUS_INCREASE_THIRD_SELECTION: &str = "terrain:third";

/// PF1 Core Rulebook level gate at which Ranger gains a FOURTH additional
/// favored terrain (verified independently against two primary sources —
/// d20pfsrd and the Archives of Nethys aonprd.com mirror, byte-for-byte
/// agreement): "At 8th level and every five levels thereafter, the ranger
/// may select an additional favored terrain. In addition, at each such
/// interval, the skill bonus and initiative bonus in any one favored
/// terrain (including the one just selected, if so desired), increases by
/// +2." The 18th-level interval (13 + 5) is the exact structural mirror of
/// the already-grounded Favored Enemy 15th-level interval
/// (`RANGER_FAVORED_ENEMY_FOURTH_INTERVAL_LEVEL`).
pub(super) const RANGER_FAVORED_TERRAIN_FOURTH_INTERVAL_LEVEL: u8 = 18;

/// SD18 Ranger FOURTH Favored Terrain choice-slot id, mirroring
/// `choice:ranger_favored_enemy_4`'s open-ended (non-restricted-list)
/// recognition idiom exactly -- no enum validation against Table: Ranger
/// Favored Terrains is performed here.
pub(super) const RANGER_FAVORED_TERRAIN_FOURTH_CHOICE_ID: &str = "choice:ranger_favored_terrain_4";

/// SD18 Ranger 18th-level-interval bonus-increase TARGET choice-slot id,
/// mirroring `choice:ranger_favored_enemy_bonus_increase_target_3`'s
/// restricted idiom, widened to the four-terrain set (`terrain:first` /
/// `terrain:second` / `terrain:third` / `terrain:fourth`); any other
/// selection is surfaced without grounding a target identity and no boost
/// is fabricated from it.
pub(super) const RANGER_FAVORED_TERRAIN_THIRD_BONUS_INCREASE_CHOICE_ID: &str =
    "choice:ranger_favored_terrain_bonus_increase_target_3";
pub(super) const RANGER_FAVORED_TERRAIN_BONUS_INCREASE_FOURTH_SELECTION: &str = "terrain:fourth";

/// The ranger level at which 1st-level ranger spells first become available,
/// verified against the raw PF1 Core Rulebook Ranger spells-per-day table rows
/// (d20pfsrd and legacy.aonprd.com, identical): levels 1-3 show no
/// spells-per-day columns at all, level 4 shows "0/—/—/—" — the first non-"—"
/// 1st-level column. A "0" entry is real access, not absence: "When Table:
/// Ranger indicates that the ranger gets 0 spells per day of a given spell
/// level, he gains only the bonus spells he would be entitled to based on his
/// Wisdom score for that spell level" (quoted identically by both sources —
/// Wisdom, not the Paladin's Charisma). The same sources state the
/// caster-level rule: "At 4th level and higher, his caster level is equal to
/// his ranger level – 3" — the exact rule shape already grounded for the
/// Paladin.
pub(super) const RANGER_FIRST_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 4;

/// The ranger level at which 2nd-level ranger spells first become available,
/// verified against the raw table rows (both sources): level 6 shows
/// "1/—/—/—", level 7 shows "1/0/—/—" — the first non-"—" 2nd-level column.
pub(super) const RANGER_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 7;

/// The ranger level at which 3rd-level ranger spells first become available,
/// verified against the raw table rows (both sources): level 9 shows
/// "2/1/—/—", level 10 shows "2/1/0/—" — the first non-"—" 3rd-level column.
pub(super) const RANGER_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 10;

/// The ranger level at which 4th-level ranger spells first become available,
/// verified independently against three primary sources (d20pfsrd, the
/// Archives of Nethys aonprd.com mirror, and legacy.aonprd.com, all
/// byte-for-byte identical): level 12 shows "2/2/1/—", level 13 shows
/// "3/2/1/0" — the first non-"—" 4th-level column. The SAME level-13 row
/// also genuinely raises the 1st-level column from 2 to 3 (a literal table
/// lookup value, not a formula), while the 2nd/3rd-level columns stay
/// numerically unchanged (2/1).
pub(super) const RANGER_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 13;

/// PF1 Core Rulebook level gate at which Ranger gains Hunter's Bond (4th level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Hunter's bond" as the Ranger 4th-level "Special"
/// column entry, and both state the exact rule text: "At 4th level, a ranger
/// forms a bond with his hunting companions. This bond can take one of two
/// forms. Once the form is chosen, it cannot be changed." The first form, a
/// bond to his companions, grants the ranger the ability to spend a move action
/// to grant allies within 30 feet who can see or hear him half his favored-enemy
/// bonus against a single target of the appropriate type -- a genuinely
/// flat-shaped magnitude (half the already-grounded Favored Enemy bonus), grounded
/// as a standalone, non-applied record: no move-action/action-economy engine, no
/// ally-range-and-perception check, and no favored-enemy target-type matching is
/// implemented. The second form, an animal companion, is deliberately left
/// named-but-unproven: it would require a full animal-companion stat
/// block/advancement subsystem that does not exist anywhere in this codebase, a
/// new-subsystem-shaped burden, not a slice-shaped one.
pub(super) const RANGER_HUNTERS_BOND_LEVEL: u8 = 4;

/// SD13-E5 Ranger Hunter's Bond choice-slot id. The deterministic fixture names
/// which of the two mutually exclusive forms was chosen (`form:bond` or
/// `form:companion`), mirroring `choice:ranger_combat_style`'s restricted
/// two-option recognition idiom (unlike the open-ended Favored Enemy/Favored
/// Terrain choice-slots, Hunter's Bond only has two legal forms).
pub(super) const RANGER_HUNTERS_BOND_CHOICE_ID: &str = "choice:ranger_hunters_bond";

pub(super) const RANGER_HUNTERS_BOND_COMPANION_SELECTION: &str = "form:companion";

pub(super) const RANGER_HUNTERS_BOND_BOND_SELECTION: &str = "form:bond";

// SD13-E5 Ranger combat style choice-slot recognition, grounded once the level-range
// gate reaches RANGER_COMBAT_STYLE_LEVEL (2nd level). PF1 Core Rulebook Combat Style
// Feat: at 2nd level a ranger selects one combat style -- Archery or Two-Weapon
// Combat, the two PF1 Core Rulebook options -- and gains the first bonus feat from
// that style's own restricted list (verified against legacy.aonprd.com's Core
// Rulebook Ranger page before writing any code): the Archery style's 2nd-level list
// is Far Shot, Point-Blank Shot, Precise Shot, and Rapid Shot; the Two-Weapon Combat
// style's 2nd-level list is Double Slice, Improved Shield Bash, Quick Draw, and
// Two-Weapon Fighting. Both the STYLE CHOICE and the chosen BONUS FEAT are
// recognized as chosen-input identity only (+0 each); no feat's own mechanical
// effect (e.g. Point-Blank Shot's attack/damage bonus within 30 ft.) is computed
// anywhere in this codebase.
pub(super) const RANGER_COMBAT_STYLE_CHOICE_ID: &str = "choice:ranger_combat_style";

pub(super) const RANGER_COMBAT_STYLE_ARCHERY_SELECTION: &str = "style:archery";

pub(super) const RANGER_COMBAT_STYLE_TWO_WEAPON_COMBAT_SELECTION: &str = "style:two_weapon_combat";

pub(super) const RANGER_COMBAT_STYLE_BONUS_FEAT_CHOICE_ID: &str = "choice:ranger_combat_style_bonus_feat";

// SD13-E5 Ranger SECOND combat style bonus feat, granted at 6th level (verified
// independently against d20pfsrd and legacy.aonprd.com's Core Rulebook Ranger
// page before writing any code: "The ranger's expertise manifests in the form of
// bonus feats at 2nd, 6th, 10th, 14th, and 18th level" -- 6th level is the very
// next milestone after 2nd). PF1 Core Rulebook Combat Style Feat text: "He can
// choose feats from his selected combat style, even if he does not have the
// normal prerequisites." Both primary sources agree on which feats each style's
// list gains specifically at 6th level (as distinct from the 2nd-level list
// already grounded above): the Archery style's 6th-level list is Improved
// Precise Shot and Manyshot; the Two-Weapon Combat style's 6th-level list is
// Improved Two-Weapon Fighting and Two-Weapon Defense. This grounds only a
// restricted-list recognition of the specific feat named at this milestone
// (gated on the same style choice already recognized at 2nd level); it does not
// validate the second choice against the cumulative (2nd+6th level) list, so a
// selection re-picking one of the 2nd-level list's own feats at this gate is
// deliberately left unrecognized rather than silently accepted as if it were
// general-purpose feat validation. No feat's own mechanical effect is computed
// anywhere in this codebase.
pub(super) const RANGER_COMBAT_STYLE_BONUS_FEAT_2_LEVEL: u8 = 6;

pub(super) const RANGER_COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID: &str =
    "choice:ranger_combat_style_bonus_feat_2";
pub(super) const RANGER_COMBAT_STYLE_BONUS_FEAT_3_LEVEL: u8 = 10;

pub(super) const RANGER_COMBAT_STYLE_BONUS_FEAT_3_CHOICE_ID: &str =
    "choice:ranger_combat_style_bonus_feat_3";
// PF1 Core Rulebook Archery combat style, 10th-level bonus feat list.
pub(super) const PINPOINT_TARGETING_FEAT_SELECTION: &str = "feat:pinpoint_targeting";

// SD18 cycle-2026-07-15T2100: the ranger's FOURTH combat-style bonus feat,
// granted at 14th level (bonus feats land at 2nd, 6th, 10th, 14th, and 18th
// ranger level per the Combat Style Feat class feature's own rule text).
// Unlike the 2nd/6th/10th-level grants, the PF1 Core Rulebook's own Ranger
// Combat Styles tables do not tabulate any NEW named feat options at the
// 14th-level tier — verified independently against three sources dedicated
// to the combat-style feat lists specifically (d20pfsrd's Ranger Combat
// Styles page, the Archives of Nethys aonprd.com RangerCombatStyles page,
// and a Paizo rules-forum thread addressing the same question directly):
// all three agree the printed Core Rulebook list of named options stops
// after the 10th-level tier; later sourcebooks (e.g. the Advanced Player's
// Guide) are the ones that add named 14th/18th-level options, and those are
// outside SD-18's Core-Rulebook-only scope. So this slot is recognized as
// an OPEN-ENDED +0 identity record (mirroring the Favored Terrain/Quarry
// choice-recognition idiom: raw string interpolation, no restricted-list
// validation), not the closed-restricted-list idiom used for feats 1-3.
pub(super) const RANGER_COMBAT_STYLE_BONUS_FEAT_4_LEVEL: u8 = 14;

pub(super) const RANGER_COMBAT_STYLE_BONUS_FEAT_4_CHOICE_ID: &str =
    "choice:ranger_combat_style_bonus_feat_4";
// SD18 cycle-2026-07-16T0244 (level-18 widening): the ranger's FIFTH
// combat-style bonus feat, granted at 18th level (bonus feats land at 2nd,
// 6th, 10th, 14th, and 18th ranger level per the Combat Style Feat class
// feature's own rule text, verified independently against d20pfsrd and the
// Archives of Nethys aonprd.com mirror). Mirroring the fourth bonus feat's
// own reasoning exactly: the PF1 Core Rulebook's own Ranger Combat Styles
// tables (Archery, Two-Weapon Combat) do not tabulate any NEW named feat
// options beyond the 10th-level tier (verified independently against three
// sources dedicated to the combat-style feat lists specifically —
// d20pfsrd's Ranger Combat Styles page, the Archives of Nethys aonprd.com
// RangerCombatStyles page, and a Paizo rules-forum thread — all three agree
// the printed list stops after 10th level; later sourcebooks add named
// 14th/18th-level options, outside SD-18's Core-Rulebook-only scope). So
// this slot is recognized as an OPEN-ENDED +0 identity record (mirroring
// the Favored Terrain/Quarry choice-recognition idiom exactly), not the
// closed-restricted-list idiom used for feats 1-3.
pub(super) const RANGER_COMBAT_STYLE_BONUS_FEAT_5_LEVEL: u8 = 18;

pub(super) const RANGER_COMBAT_STYLE_BONUS_FEAT_5_CHOICE_ID: &str =
    "choice:ranger_combat_style_bonus_feat_5";

/// SD-34 AT-34-E3-001 (`decisions.md §16`, "only the count grounds" is
/// ratified precedent). The full, exhaustive Archery ∪ Two-Weapon Combat
/// named-option pool this engine's corpus carries for Ranger Combat Style
/// Feat — verified against
/// `data/corpus/core_rulebook/class_feature/ranger_combat_style_feat/*.json`
/// (16 records total: the 2nd-, 6th-, and 10th-level restricted lists
/// already hand-recognized above by the SPECIFIC-choice idiom
/// (`RANGER_COMBAT_STYLE_BONUS_FEAT_CHOICE_ID` et al.) — no 14th- or
/// 18th-level slot names a NEW option in the Core Rulebook, so this list is
/// exhaustive against the corpus, not a lower bound.
///
/// Only the COUNT of granted slots is grounded as a magnitude by
/// `ground_ranger_combat_style_feat_pool` below; WHICH style (Archery or
/// Two-Weapon Combat) the ranger picked, and therefore which half of this
/// combined list actually applies, is a player choice this function
/// deliberately does not model — the ratified Fighter/Cavalier/Brawler/
/// Arcane-bloodline treatment. The style-recognition idiom above grounds
/// identity for the SPECIFIC feat a character's own recorded input names;
/// this pool exists so a character whose style choice this seam has not
/// recognized still has its slot count grounded and its full eligible pool
/// named, rather than silently staying an unclaimed record.
pub(super) const RANGER_COMBAT_STYLE_FEAT_POOL: &[&str] = &[
    "Double Slice",
    "Far Shot",
    "Greater Two-Weapon Fighting",
    "Improved Precise Shot",
    "Improved Shield Bash",
    "Improved Two-Weapon Fighting",
    "Manyshot",
    "Pinpoint Targeting",
    "Point-Blank Shot",
    "Precise Shot",
    "Quick Draw",
    "Rapid Shot",
    "Shot on the Run",
    "Two-Weapon Defense",
    "Two-Weapon Fighting",
    "Two-Weapon Rend",
];

/// The Ranger combat-style-feat pool's slot count at `level`: 0 below
/// `RANGER_COMBAT_STYLE_LEVEL`, then 1 + one more every 4 levels (1 at 2nd,
/// 2 at 6th, 3 at 10th, 4 at 14th, 5 at 18th — the same "2nd, 6th, 10th,
/// 14th, 18th" milestone progression the specific-choice idiom above
/// already documents, verified independently against d20pfsrd and
/// legacy.aonprd.com before writing any code).
pub(super) fn ranger_combat_style_feat_pool_slot_count(level: u8) -> i16 {
    if level < RANGER_COMBAT_STYLE_LEVEL {
        0
    } else {
        1 + i16::from((level - RANGER_COMBAT_STYLE_LEVEL) / 4)
    }
}

/// Grounds the Ranger combat-style-feat pool's slot COUNT
/// (`ranger_combat_style_feat_pool_slot_count`, style-invariant) and names
/// its full eligible set (`RANGER_COMBAT_STYLE_FEAT_POOL`), without seeding
/// any default choice — the ratified Fighter/Cavalier/Brawler/Arcane-
/// bloodline treatment (`decisions.md §16`). Runs unconditionally once the
/// gate is reached, independent of whether this seam has recognized any
/// `RANGER_COMBAT_STYLE_CHOICE_ID` selection — the specific-choice idiom
/// above grounds identity for a chosen style/feat pair; this function
/// grounds the count and the full pool regardless of whether that choice
/// was ever recognized.
///
/// One non-claim-blocking diagnostic is emitted per eligible option,
/// carrying the SAME templated message regardless of which option it
/// names — deliberately, not an oversight: this pool's own names collide
/// under substring containment (`"Precise Shot"` inside `"Improved Precise
/// Shot"`; `"Two-Weapon Fighting"` inside both `"Improved Two-Weapon
/// Fighting"` and `"Greater Two-Weapon Fighting"`), so the downstream
/// classifier's own substring-based diagnostic lookup
/// (`v06_work_inventory.rs::diagnostic_id_names_feature`) can legitimately
/// resolve a given corpus record to a different-but-textually-related
/// diagnostic than the one this loop built for its own name. Templating
/// every message identically makes that indeterminacy harmless: whichever
/// diagnostic a lookup lands on, the content it reports is equally true of
/// the record it was asked about.
pub(super) fn ground_ranger_combat_style_feat_pool(
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let slot_count = ranger_combat_style_feat_pool_slot_count(level);
    let eligible_count = RANGER_COMBAT_STYLE_FEAT_POOL.len();
    let detail = if slot_count == 0 {
        format!(
            "Ranger combat style feat pool slot count at ranger level {level}: none yet, \
             correctly absent by PF1 Core Rulebook level gate (the first slot is granted at \
             ranger level {RANGER_COMBAT_STYLE_LEVEL})"
        )
    } else {
        format!(
            "Ranger combat style feat pool slot count at ranger level {level}: {slot_count} \
             slot(s) granted (one at 2nd, 6th, 10th, 14th, and 18th ranger level). This grounds \
             regardless of whether this seam has recognized which combat style (Archery or \
             Two-Weapon Combat) the character chose. Only the COUNT grounds; which of the \
             {eligible_count} corpus-wide eligible feats (the combined Archery and Two-Weapon \
             Combat lists) fills each slot is a player choice this bounded seam does not \
             model, the ratified Fighter/Cavalier/Brawler/Arcane-bloodline treatment"
        )
    };
    explanations.push(ComputationExplanation {
        id: "class_feature.ranger.combat_style_feat_pool.slot_count".to_owned(),
        value: slot_count,
        detail,
    });
    if slot_count == 0 {
        return;
    }
    for feat in RANGER_COMBAT_STYLE_FEAT_POOL {
        diagnostics.push(ComputationDiagnostic {
            id: format!(
                "class_feature.ranger.combat_style_feat_pool.option.{}.not_modelled",
                slugify_id_segment(feat)
            ),
            message: format!(
                "Ranger combat style feat pool at ranger level {level}: the slot count above \
                 is grounded, but WHICH of the {eligible_count} corpus-wide eligible feats (the \
                 combined Archery and Two-Weapon Combat lists) fills any given slot is a player \
                 choice not resolved on this bounded seam; no default feat is fabricated for \
                 any slot"
            ),
            claim_blocking: false,
        });
    }
}

#[cfg(test)]
mod ranger_combat_style_feat_pool_tests {
    use super::{
        ground_ranger_combat_style_feat_pool, ComputationDiagnostic, ComputationExplanation,
        RANGER_COMBAT_STYLE_FEAT_POOL,
    };

    /// SD-34 AT-34-E3-001 (`decisions.md §16`): the pool's slot count grounds
    /// regardless of whether this seam has recognized which style the
    /// character chose.
    #[test]
    fn ranger_combat_style_feat_pool_slot_count_grounds_at_the_2nd_level_gate() {
        let mut explanations: Vec<ComputationExplanation> = Vec::new();
        let mut diagnostics: Vec<ComputationDiagnostic> = Vec::new();
        ground_ranger_combat_style_feat_pool(2, &mut explanations, &mut diagnostics);
        let count_explanation = explanations
            .iter()
            .find(|e| e.id == "class_feature.ranger.combat_style_feat_pool.slot_count")
            .expect("the slot count must ground at level 2");
        assert_eq!(count_explanation.value, 1);
    }

    #[test]
    fn ranger_combat_style_feat_pool_slot_count_is_correctly_absent_below_the_grant_level() {
        let mut explanations: Vec<ComputationExplanation> = Vec::new();
        let mut diagnostics: Vec<ComputationDiagnostic> = Vec::new();
        ground_ranger_combat_style_feat_pool(1, &mut explanations, &mut diagnostics);
        let count_explanation = explanations
            .iter()
            .find(|e| e.id == "class_feature.ranger.combat_style_feat_pool.slot_count")
            .expect("the slot count record must still be present, valued at 0");
        assert_eq!(count_explanation.value, 0);
        assert!(
            diagnostics.is_empty(),
            "no per-option diagnostic should fire before any slot is granted"
        );
    }

    #[test]
    fn ranger_combat_style_feat_pool_names_every_eligible_option_once_a_slot_is_granted() {
        let mut explanations: Vec<ComputationExplanation> = Vec::new();
        let mut diagnostics: Vec<ComputationDiagnostic> = Vec::new();
        ground_ranger_combat_style_feat_pool(2, &mut explanations, &mut diagnostics);
        assert_eq!(diagnostics.len(), RANGER_COMBAT_STYLE_FEAT_POOL.len());
        assert!(
            diagnostics.iter().any(|d| d.id.contains(".ranger.") && d.id.contains("double_slice")),
            "Double Slice must have its own matching diagnostic id"
        );
        assert!(diagnostics.iter().all(|d| !d.claim_blocking), "non-claim-blocking only");
    }

    #[test]
    fn ranger_combat_style_feat_pool_slot_count_reaches_five_by_18th_level() {
        let mut explanations: Vec<ComputationExplanation> = Vec::new();
        let mut diagnostics: Vec<ComputationDiagnostic> = Vec::new();
        ground_ranger_combat_style_feat_pool(18, &mut explanations, &mut diagnostics);
        let count_explanation = explanations
            .iter()
            .find(|e| e.id == "class_feature.ranger.combat_style_feat_pool.slot_count")
            .expect("the slot count must ground at level 18");
        assert_eq!(count_explanation.value, 5);
    }
}

pub(super) const VANARA_TREE_STRANGER_TRAIT_KEY: &str = "Vanara ~ Tree Stranger";

pub(super) const TREE_STRANGER_VANARA_SPEED_FEET: i16 = 30;

/// The bounded Paladin milestone level this decomposition surface grounds, if
/// any. Returns the single Paladin level when the chosen input is exactly a
/// single-class Paladin at one of the supported milestone levels (1 through
/// 10). Returns `None` for no Paladin, a non-Paladin class, a multiclass mix,
/// the Ranger hybrid (which has its own F6 class-feature decomposition
/// lane), or any level-11+ Paladin this slice deliberately does not
/// recognize — each of which stays claim-blocked exactly as before.
pub(super) fn supported_paladin_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == PALADIN_CLASS_ID
                && (1..=MAX_SUPPORTED_PALADIN_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E3/E4/E5 runtime evidence for the deterministic Human
/// Paladin level-1/level-2/level-3 chassis and spell burden as a separable pair
/// of diagnostics.
///
/// This sits on top of the accepted SD13-F6 hybrid baseline: F6 already proves
/// the deterministic Human Paladin level-1 hybrid identity is acknowledged on
/// the compute seam and emits a single combined non-spell class-feature
/// blocker plus a single combined later-spell blocker. This slice proves the
/// per-burden separation Paladin actually needs, widened by the SD13-E5
/// level-2 milestone, further SD13-E5 slices' level-3 milestone (mercy) and
/// level-4 milestone (Smite Evil 2/day, Channel Positive Energy grant), the
/// level-5 milestone (the effective-caster-level gate and Channel Positive
/// Energy's die count both genuinely widen again; Divine Bond, the level-5
/// "Special" column's other entry, was checked and confirmed NOT flat, so it
/// stays deliberately named-but-unproven), the level-6 milestone (both good
/// base saves and lay on hands genuinely widen again, and the
/// effective-caster-level gate widens again; the level-6 "Special" column's
/// repeat "Mercy" entry -- an additional mercy becomes selectable at 6th
/// level -- was checked and confirmed to require a mercy-list-growth
/// mechanism this codebase has not already grounded, so it stays
/// deliberately named-but-unproven, mirroring the Divine Bond precedent),
/// and this file's own level-7 milestone (Smite Evil's uses/day, the
/// effective-caster-level gate, and Channel Positive Energy's die count all
/// genuinely widen again; base saves and lay on hands stay numerically
/// unchanged from level 6, an integer-division coincidence; the level-7
/// "Special" column reads "Smite evil 3/day" only, verified independently
/// against d20pfsrd and legacy.aonprd.com, and level 7 is not one of the
/// repeat-Mercy-grant levels 3/6/9/..., so nothing new is left unproven for
/// Mercy here):
///
/// - one grounded numeric explanation set for the foundational base-attack-
///   bonus / base-save progression pillar, computed for real at every
///   supported level (1..=3): full base attack bonus (the same formula shape
///   as Fighter/Barbarian/Ranger), and good Fortitude / good Will / poor
///   Reflex base saves (NOT the same save shape as Ranger's good
///   Fortitude/Reflex, poor Will). Both formulas were verified independently
///   against the PF1 Core Rulebook Paladin class table before grounding.
///
/// - one grounded numeric explanation set for the fourth named non-spell
///   pillar, Smite Evil, computed for real at every supported level:
///   * PF1 Core Rulebook Smite Evil: 1 use per day below level 4, an
///     attack-roll bonus equal to the paladin's Charisma modifier (if
///     positive — the rule text applies the Charisma bonus "if any", never a
///     penalty), and a damage bonus equal to the paladin's class level. This
///     grounds only that flat numeric formula; it grounds no alignment /
///     evil-subtype target resolution, no swift-action activation
///     bookkeeping, no deflection-AC-vs-target bonus, and no
///     evil-outsider/evil-dragon/undead damage doubling.
///
/// - below the level-2 gate (i.e. at level 1), two grounded level-gate
///   records (value 0 each) whose honest computed surface is their correct
///   ABSENCE by PF1 Core Rulebook level gate:
///   * `lay on hands` — a 2nd-level paladin feature (heals 1d6 per two paladin
///     levels; uses/day = 1/2 paladin level + Charisma modifier); the at-grant
///     formula is named but not computed
///   * `divine grace` — a 2nd-level paladin feature (+Charisma bonus on all
///     saving throws); the at-grant formula is named but not computed
///
/// - at or above the level-2 gate, lay on hands and divine grace are grounded
///   for real as bounded, flat numeric formulas with no execution engine
///   behind them (no healing-resolution engine, no saving-throw-resolution
///   engine):
///   * `lay on hands` uses per day = 1/2 paladin level + Charisma modifier;
///     the heal amount is stated as a flat, non-fabricated d6-die-count
///     magnitude (1d6 per two paladin levels), never a rolled value —
///     mirroring how Smite Evil's damage bonus is a flat scalar, not a
///     dice-roll execution
///   * `divine grace` grants a Charisma-modifier bonus on all saving throws,
///     applied only if positive — mirroring the "applied only if positive"
///     idiom already used for Smite Evil's attack bonus
///
/// - below the level-3 gate (levels 1-2), `mercy` stays a grounded level-gate
///   absence record (value 0); at or above it (SD13-E5 level-3 widening), it
///   transitions to a bounded GRANT-only identity record (mirroring the
///   Barbarian Uncanny Dodge / Ranger Endurance idiom) plus, when the
///   deterministic fixture provides one, a choice-recognition record naming
///   which mercy was selected (mirroring the Ranger Favored Terrain /
///   Sorcerer bloodline choice-slot idiom): mercy is a 3rd-level paladin
///   feature (gained at 3rd level and every three levels thereafter; a
///   paladin selects one mercy from the list, and each mercy adds an effect
///   to lay on hands). The selected mercy's own effect (curing the named
///   condition automatically whenever lay on hands is used) is NOT computed,
///   since no lay-on-hands execution engine exists in this codebase.
///
/// - one grounded numeric explanation (SD13-E5) for the partial-caster
///   IDENTITY itself, distinct from the spell burden it sits next to:
///   * PF1 Core Rulebook effective caster level = max(paladin level − 3, 0);
///     spells begin at paladin level 4. At the bounded level-1 baseline this
///     grounds to 0 — the same "correct absence" idiom already used for the
///     lay on hands / divine grace / mercy level gates above. This grounds
///     only the caster-level gate arithmetic; it fabricates no spells known,
///     no spells per day, no bonus spell slots, and no spell save DCs.
///
/// - one explicit claim-blocking diagnostic for the partial-caster spell
///   burden, distinct from the grounded chassis records, unchanged by this
///   slice:
///   * Paladin is a divine partial caster in PF1 Core Rulebook (spells begin
///     at paladin level 4; effective caster level = paladin level − 3); the
///     blocker names this partial-caster posture so the later spell-burden
///     closure cannot collapse Paladin into a full divine caster shape
///     (Cleric / Druid) and so partial-caster pressure stays visible on the
///     runtime path.
///
/// This deliberately does not compute a supported spell surface, and it does
/// not ground level 8+, Divine Bond (the level-5 "Special" column's other
/// entry, checked against a primary source and confirmed to need an
/// activation/resource-consumption engine plus either a weapon-enhancement
/// subsystem or a full mount stat-block/advancement subsystem), or a second
/// mercy-selection slot (the level-6 "Special" column's repeat "Mercy" entry,
/// checked against a primary source and confirmed to need a mercy-list-growth
/// mechanism this codebase has not already grounded). Beyond the grounded
/// Smite Evil, Channel Positive Energy, lay on hands, and divine grace
/// numeric formulas, the mercy grant/choice recognition, and the grounded
/// effective-caster-level gate, it grounds no spell slots, no spell source
/// lineage, no spells known or prepared posture, no deity resolution, no
/// domain mechanics, no alignment-target resolution, no healing-resource
/// accounting, and no saving-throw-resolution engine. It only emits the
/// grounded records and the remaining spell blocker that prove the F6 surface
/// remains separable on the runtime path.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input;
/// the F6 hybrid chassis emission already preserves a single class-feature
/// blocker and a single spell blocker (both gated to the bounded hybrid
/// baseline level, so they only fire at level 1). This seam adds per-burden
/// granularity next to the F6 surface, never replacing it, so the F6
/// acceptance test continues to pass.
pub(super) fn explain_paladin_level1_chassis_and_spell_burden_separation(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // v0.6 alpha swarm, risks item 8, second slice (2026-07-25): Paladin's
    // partial-caster spell posture is validated regardless of whether
    // Paladin appears alone or in a multiclass mix, and regardless of race
    // -- checked BEFORE the single-class-only/Human gate below, mirroring
    // the Ranger fix exactly (`ranger_dispatch_widening_safety_tests`'
    // own doc comment has the full history of why this ordering matters:
    // `table_class_id` recognizing Paladin makes
    // `multiclass_class_level_supported`/`is_supported_multiclass_mix`
    // accept a Paladin-containing mix too, and `compute_multiclass_base_chassis`
    // deliberately discards each isolated per-class sub-computation's own
    // diagnostics, so nothing else in the multiclass path would ever
    // surface this burden). `unmet_paladin_prepared_spell_conditions`
    // mirrors `unmet_ranger_prepared_spell_conditions` exactly, substituting
    // Charisma for Wisdom and `paladin_spell_list::PALADIN_SPELL_LIST` for
    // the ranger list -- see that function's own doc comment for why zero
    // prepared spells is valid (real PF1 doesn't require filling every
    // slot, and paladin spells aren't accessible before level 4 at all).
    if let Some(paladin_level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == PALADIN_CLASS_ID)
        .map(|class_level| class_level.level)
    {
        let unmet =
            unmet_paladin_prepared_spell_conditions(input, paladin_level, ability_modifiers);
        if unmet.is_empty() {
            ground_paladin_prepared_spells(input, paladin_level, ability_modifiers, explanations);
        } else {
            diagnostics.push(ComputationDiagnostic {
                id: "class_spell.paladin.partial_caster.unsupported".to_owned(),
                message: format!(
                    "Paladin remains blocked on its divine partial-caster spell burden: Paladin \
                     is a partial caster (spells begin at paladin level 4, with effective caster \
                     level = paladin level - 3 in PF1 Core Rulebook); unmet prepared-spell \
                     posture: {}",
                    unmet.join("; ")
                ),
                claim_blocking: true,
            });
        }
    }

    let Some(level) = supported_paladin_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Smite Evil, the fourth named non-spell pillar, is grounded for real: a
    // bounded, flat numeric formula with no execution engine behind it. PF1
    // Core Rulebook: 1 use/day below level 4, attack-roll bonus = Charisma
    // modifier (if positive; the rule never applies it as a penalty), damage
    // bonus = paladin level.
    let paladin_level = i16::from(level);
    let charisma_modifier = ability_modifier_for(ability_modifiers, "charisma");

    // Grounded (SD13-E5): the foundational base-attack-bonus / base-save progression
    // pillar. Unlike every other class row in this matrix (Fighter, Barbarian, Monk,
    // Rogue, Druid, Cleric, Bard, Sorcerer, Wizard, and by the immediately preceding
    // cycle, Ranger), Paladin had never had this pillar grounded at all, despite
    // Paladin already supporting a level-range gate (1..=2) unlike Ranger's
    // level-1-only gate at the time its own gap was closed. Both formulas were
    // verified against the PF1 Core Rulebook Paladin class table (d20pfsrd and the
    // legacy Paizo PRD mirror) before writing this code, reading the raw level 1-6
    // table rows directly (BAB +1/+2/+3/+4/+5/+6, Fort +2/+3/+3/+4/+4/+5, Ref
    // +0/+0/+1/+1/+1/+2, Will +2/+3/+3/+4/+4/+5) rather than assuming Paladin
    // matched Ranger's exact shape: Paladin is full BAB (the same shape as
    // Fighter/Barbarian/Ranger), but its good saves are Fortitude AND Will (poor
    // Reflex) -- NOT Ranger's good Fortitude/Reflex, poor Will. Paladin level 8+
    // remains out of scope; the flat base-attack and base-save numbers are
    // grounded here, extended across the now-supported level 1..=7 range (level
    // 5's Fortitude/Will/Reflex values were numerically unchanged from level 4, an
    // integer-division coincidence; level 6's values genuinely increase again;
    // level 7's values stay numerically unchanged from level 6, another
    // integer-division coincidence, re-verified rather than assumed).
    let good_save = paladin_level / 2 + 2;
    let poor_save = paladin_level / 3;

    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.base_attack_bonus".to_owned(),
        value: paladin_level,
        detail: format!(
            "Paladin level {level} base attack bonus from the PF1 Core Rulebook Paladin class \
             table (full base-attack progression, the same formula shape as \
             Fighter/Barbarian/Ranger): classlevel = {paladin_level}. This is a standalone \
             explanation record; it is not wired into the integrated base_attack_bonus field or \
             into compute_combat_baseline"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.base_save.fortitude".to_owned(),
        value: good_save,
        detail: format!(
            "Paladin level {level} base Fortitude save (good save) from the PF1 Core Rulebook \
             Paladin class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.base_save.reflex".to_owned(),
        value: poor_save,
        detail: format!(
            "Paladin level {level} base Reflex save (poor save) from the PF1 Core Rulebook \
             Paladin class table: classlevel/3 = {poor_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.base_save.will".to_owned(),
        value: good_save,
        detail: format!(
            "Paladin level {level} base Will save (good save) from the PF1 Core Rulebook \
             Paladin class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });

    // Smite Evil uses per day genuinely increases at level 4 (PF1 Core Rulebook:
    // 1/day below level 4, +1 at level 4 and every three levels thereafter, to a
    // maximum of 7/day at level 19 -- verified independently against d20pfsrd
    // and legacy.aonprd.com rather than assumed to stay at 1). The formula
    // `1 + (paladin level - 1) / 3` correctly yields 1 at levels 1-3 and 2 at
    // levels 4-6, then GENUINELY increases to 3 at level 7 (the PF1 CRB
    // level-7 "Special" column reads "Smite evil 3/day", verified
    // independently rather than assumed to stay at 2; the next increase does
    // not land until level 10, out of scope for this bounded level-7
    // baseline).
    let smite_evil_uses_per_day: i16 = 1 + (paladin_level - 1) / 3;
    let smite_evil_attack_bonus = charisma_modifier.max(0);
    let smite_evil_damage_bonus = paladin_level;

    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.smite_evil_uses_per_day".to_owned(),
        value: smite_evil_uses_per_day,
        detail: format!(
            "Paladin Smite Evil uses per day at paladin level {level} (PF1 Core Rulebook: 1/day \
             below level 4, +1 at level 4 and every three levels thereafter, to a maximum of \
             7/day at level 19): 1 + ({paladin_level} - 1) / 3 = {smite_evil_uses_per_day}. This \
             grounds only the flat per-day resource count; it computes no swift-action activation \
             bookkeeping and no per-use consumption tracking"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.smite_evil_attack_bonus".to_owned(),
        value: smite_evil_attack_bonus,
        detail: format!(
            "Paladin Smite Evil attack-roll bonus: the paladin's Charisma modifier, applied only \
             if positive (PF1 Core Rulebook: \"the paladin adds her Charisma modifier, if any, to \
             her attack roll\", never as a penalty) = max({charisma_modifier}, 0) = \
             {smite_evil_attack_bonus}. This grounds only the flat attack-roll bonus; it computes \
             no alignment or evil-subtype target resolution"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.smite_evil_damage_bonus".to_owned(),
        value: smite_evil_damage_bonus,
        detail: format!(
            "Paladin Smite Evil damage bonus: equal to the paladin's class level (PF1 Core \
             Rulebook: 2x paladin level against evil outsiders, evil dragons, and undead, which \
             this bounded formula does not distinguish) = {smite_evil_damage_bonus} at paladin \
             level {level}. This grounds only the flat per-hit damage bonus; it computes no \
             evil-outsider/evil-dragon/undead damage doubling and no deflection-AC bonus against \
             the smited target"
        ),
    });

    if level < PALADIN_LAY_ON_HANDS_DIVINE_GRACE_LEVEL {
        // Below the level-2 gate, lay on hands and divine grace are grounded
        // as correct PF1 Core Rulebook level-gate absences (value 0 each).
        // Each record names the at-grant formula without computing it; no
        // heal amount or save bonus is fabricated.
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.level_gate.lay_on_hands".to_owned(),
            value: 0,
            detail: format!(
                "Paladin lay on hands at paladin level {level}: correctly absent at level {level} \
                 by PF1 CRB level gate; at-grant formula named but not computed. Lay on hands is a \
                 2nd-level paladin feature: heals 1d6 per two paladin levels, uses/day = 1/2 \
                 paladin level + Charisma modifier"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.level_gate.divine_grace".to_owned(),
            value: 0,
            detail: format!(
                "Paladin divine grace at paladin level {level}: correctly absent at level {level} \
                 by PF1 CRB level gate; at-grant formula named but not computed. Divine grace is a \
                 2nd-level paladin feature: +Charisma bonus on all saving throws"
            ),
        });
    } else {
        // At or above the level-2 gate, lay on hands and divine grace are
        // grounded for real: bounded, flat numeric formulas with no
        // execution engine behind them.
        let extra_lay_on_hands_bonus = extra_resource_feat_bonus(
            &input.chosen.selected_feats,
            EXTRA_LAY_ON_HANDS_FEAT_KEY,
            EXTRA_POINTS_PER_DAY,
        );
        let lay_on_hands_uses_per_day =
            paladin_level / 2 + charisma_modifier + extra_lay_on_hands_bonus;
        let lay_on_hands_heal_dice = paladin_level / 2;
        let divine_grace_save_bonus = charisma_modifier.max(0);

        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.lay_on_hands_uses_per_day".to_owned(),
            value: lay_on_hands_uses_per_day,
            detail: format!(
                "Paladin lay on hands uses per day at paladin level {level} (PF1 Core Rulebook, \
                 2nd-level paladin feature): 1/2 paladin level + Charisma modifier = \
                 {paladin_level} / 2 + {charisma_modifier} + Extra Lay On Hands feat \
                 (+{extra_lay_on_hands_bonus}) = {lay_on_hands_uses_per_day}. This \
                 grounds only the flat per-day resource count; it computes no \
                 healing-resolution execution engine and no per-use consumption tracking"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.lay_on_hands_heal_amount".to_owned(),
            value: lay_on_hands_heal_dice,
            detail: format!(
                "Paladin lay on hands heal amount at paladin level {level} (PF1 Core Rulebook, \
                 2nd-level paladin feature): 1d6 per two paladin levels = {lay_on_hands_heal_dice}d6 \
                 at paladin level {level}. This grounds only the flat die-count magnitude, stated \
                 as a non-fabricated record; it computes no dice-roll execution and no \
                 healing-resource accounting"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.divine_grace_save_bonus".to_owned(),
            value: divine_grace_save_bonus,
            detail: format!(
                "Paladin divine grace saving-throw bonus at paladin level {level} (PF1 Core \
                 Rulebook, 2nd-level paladin feature): the paladin's Charisma modifier, applied \
                 only if positive (never as a penalty) = max({charisma_modifier}, 0) = \
                 {divine_grace_save_bonus}, applied to all saving throws. This grounds only the \
                 flat saving-throw bonus magnitude; it computes no saving-throw-resolution engine"
            ),
        });
    }

    // Mercy: below the level-3 gate, this stays a grounded level-gate absence
    // (value 0); at or above it (SD13-E5 level-3 widening), it transitions to a
    // bounded GRANT-only identity record (mirroring the Barbarian Uncanny Dodge /
    // Ranger Endurance idiom), plus -- when the deterministic fixture provides
    // one -- a further choice-recognition record naming which mercy was selected
    // (mirroring the Ranger Favored Terrain / Sorcerer bloodline choice-slot
    // idiom). No mercy effect (curing the named condition when lay on hands is
    // used) is ever fabricated; no lay-on-hands execution engine exists in this
    // codebase.
    if level < PALADIN_MERCY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.level_gate.mercy".to_owned(),
            value: 0,
            detail: format!(
                "Paladin mercy at paladin level {level}: correctly absent at level {level} by PF1 \
                 CRB level gate (mercy is a {PALADIN_MERCY_LEVEL}rd-level paladin feature, gained \
                 at 3rd level and every three levels thereafter); at-grant formula named but not \
                 computed. Mercy is chosen from the mercy list and attaches to lay on hands"
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.mercy_granted".to_owned(),
            value: 0,
            detail: format!(
                "Paladin mercy granted at paladin level {level} (PF1 Core Rulebook, \
                 {PALADIN_MERCY_LEVEL}rd-level paladin feature, gained at 3rd level and every \
                 three levels thereafter): \"a paladin can select one mercy. Each mercy adds an \
                 effect to the paladin's lay on hands ability\" (verified independently against \
                 legacy.aonprd.com's Core Rulebook Paladin page). The first, 3rd-level tier of \
                 the mercy list is Fatigued, Shaken, and Sickened. This is a bounded grant-only \
                 identity record (value 0, non-fabricated): which specific mercy was selected is \
                 recognized separately below when present, and the selected mercy's own effect \
                 (curing the named condition automatically whenever lay on hands is used) is not \
                 computed, since no lay-on-hands execution engine exists anywhere in this codebase"
            ),
        });

        if let Some(selected_mercy) = choice_selection(input, PALADIN_MERCY_CHOICE_ID) {
            explanations.push(ComputationExplanation {
                id: "class_chassis.paladin.mercy_choice".to_owned(),
                value: 0,
                detail: format!(
                    "Paladin mercy selection ({PALADIN_MERCY_CHOICE_ID} -> {selected_mercy}): the \
                     level-{level} mercy chosen for this character is {selected_mercy}. This is a \
                     bounded recognition record of the chosen mercy only; no restricted-list \
                     validation is performed (mirroring the Ranger Favored Terrain / Sorcerer \
                     bloodline class-skill choice-recognition idiom), and the mercy's own effect \
                     is not computed, since no lay-on-hands execution engine exists anywhere in \
                     this codebase"
                ),
            });
        }
    }

    // Channel Positive Energy: below the level-4 gate (levels 1-3), this stays a
    // correct level-gate absence record (value 0); at or above it (SD13-E5
    // level-4 widening), it transitions to a bounded, flat-magnitude record
    // grounding only the channel-energy die count, mirroring the Cleric Channel
    // Energy dice-count idiom exactly (ceil(effective level / 2)). No
    // healing/damage-resolution execution, no heal-vs-harm target selection,
    // and no lay-on-hands-resource-consumption bookkeeping is computed.
    if level < PALADIN_CHANNEL_POSITIVE_ENERGY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.level_gate.channel_positive_energy".to_owned(),
            value: 0,
            detail: format!(
                "Paladin channel positive energy at paladin level {level}: correctly absent at \
                 level {level} by PF1 CRB level gate (channel positive energy is a \
                 {PALADIN_CHANNEL_POSITIVE_ENERGY_LEVEL}th-level paladin feature); at-grant \
                 formula named but not computed. Channel positive energy lets a paladin channel \
                 positive energy like a cleric, using her paladin level as her effective cleric \
                 level, consuming two uses of lay on hands per use"
            ),
        });
    } else {
        let channel_positive_energy_dice = (paladin_level + 1) / 2;
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.channel_positive_energy_dice".to_owned(),
            value: channel_positive_energy_dice,
            detail: format!(
                "Paladin channel positive energy dice at paladin level {level} (PF1 Core \
                 Rulebook, {PALADIN_CHANNEL_POSITIVE_ENERGY_LEVEL}th-level paladin feature, \
                 verified independently against d20pfsrd and legacy.aonprd.com: \"When a paladin \
                 reaches 4th level, she gains the supernatural ability to channel positive \
                 energy like a cleric. Using this ability consumes two uses of her lay on hands \
                 ability. A paladin uses her level as her effective cleric level when channeling \
                 positive energy.\"): ceil(paladin level / 2) = ceil({paladin_level} / 2) = \
                 {channel_positive_energy_dice}d6, mirroring the same die-count formula already \
                 grounded for Cleric's own Channel Energy. This grounds only the flat die-count \
                 magnitude and the lay-on-hands-use-cost identity; it computes no \
                 healing/damage-resolution execution, no heal-vs-harm target selection, and no \
                 lay-on-hands-resource-consumption bookkeeping"
            ),
        });
    }

    // Aura of Justice: below the level-11 gate, this stays a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it (SD18 level-11
    // widening), it transitions to a bounded GRANT-only identity record
    // (mirroring the Monk Diamond Body idiom exactly). No ally-aura/positional
    // engine and no smite-evil-resource-sharing execution engine exists
    // anywhere in this codebase to apply the shared smite to.
    if level < PALADIN_AURA_OF_JUSTICE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.aura_of_justice".to_owned(),
            value: 0,
            detail: format!(
                "Paladin Aura of Justice at paladin level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Aura of Justice is an 11th-level paladin class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.aura_of_justice".to_owned(),
            value: 0,
            detail: format!(
                "Paladin Aura of Justice granted at paladin level {level} (PF1 Core Rulebook, \
                 11th-level paladin class feature): \"At 11th level, a paladin can expend two \
                 uses of her smite evil ability to grant the ability to smite evil to all \
                 allies within 10 feet, using her bonuses, but through their own weapons.\" \
                 This is a bounded grant-only identity record only (value 0, non-fabricated): \
                 no ally-aura/positional engine and no smite-evil-resource-sharing execution \
                 engine exists anywhere in this codebase to apply the shared smite to."
            ),
        });
    }

    // Aura of Faith: below the level-14 gate, this stays a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it (SD18 level-14
    // widening), it transitions to a bounded GRANT-only identity record
    // (mirroring the Aura of Justice / Monk Diamond Body idiom exactly). No
    // alignment-treatment execution engine and no damage-reduction-
    // overcoming resolution engine exists anywhere in this codebase to
    // apply this to.
    if level < PALADIN_AURA_OF_FAITH_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.aura_of_faith".to_owned(),
            value: 0,
            detail: format!(
                "Paladin Aura of Faith at paladin level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Aura of Faith is a 14th-level paladin class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.aura_of_faith".to_owned(),
            value: 0,
            detail: format!(
                "Paladin Aura of Faith granted at paladin level {level} (PF1 Core Rulebook, \
                 14th-level paladin class feature): \"At 14th level, a paladin's weapons are \
                 treated as good-aligned for the purposes of overcoming damage reduction. \
                 Additionally, any attack made against an enemy within 10 feet of her is \
                 treated as good-aligned for the purposes of overcoming damage reduction.\" \
                 This is a bounded grant-only identity record only (value 0, non-fabricated): \
                 no alignment-treatment execution engine and no damage-reduction-overcoming \
                 resolution engine exists anywhere in this codebase to apply this to."
            ),
        });
    }

    // Aura of Righteousness: below the level-17 gate, both records below stay
    // correct PF1 Core Rulebook level-gate absences (value 0); at or above it
    // (SD18 level-17 widening), the aura stays a bounded GRANT-only identity
    // record while its DR clause grounds a real magnitude of its own.
    //
    // The aura record deliberately stays value 0 even now: the feature has
    // three clauses, and only one of them is grounded. Compulsion immunity
    // needs a spell-effect-type engine, and the ally +4 morale bonus is an
    // aura affecting OTHER creatures -- neither is modelled anywhere here.
    // Folding DR's 5 into the aura's own value would claim the whole feature
    // is computed when two thirds of it is not.
    if level < PALADIN_AURA_OF_RIGHTEOUSNESS_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.aura_of_righteousness".to_owned(),
            value: 0,
            detail: format!(
                "Paladin Aura of Righteousness at paladin level {level}: correctly absent at \
                 level {level} by PF1 Core Rulebook level gate; the at-grant rule is named but \
                 not computed. Aura of Righteousness is a 17th-level paladin class feature."
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.damage_reduction".to_owned(),
            value: 0,
            detail: format!(
                "Paladin Damage Reduction at paladin level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate. Paladin gains DR 5/evil as one clause \
                 of Aura of Righteousness, a 17th-level class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.aura_of_righteousness".to_owned(),
            value: 0,
            detail: format!(
                "Paladin Aura of Righteousness granted at paladin level {level} (PF1 Core \
                 Rulebook, 17th-level paladin class feature): \"At 17th level, a paladin \
                 gains DR 5/evil and immunity to compulsion spells and spell-like abilities.\" \
                 This stays a bounded grant-only identity record (value 0, non-fabricated) \
                 because two of its three clauses remain ungrounded: compulsion immunity needs \
                 a spell-effect-type engine, and the ally +4 morale bonus against fear and \
                 compulsion applies to OTHER creatures within 10 feet, which this codebase \
                 models nowhere. Its DR clause IS grounded, separately, as \
                 class_chassis.paladin.damage_reduction."
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.damage_reduction".to_owned(),
            value: PALADIN_AURA_OF_RIGHTEOUSNESS_DAMAGE_REDUCTION,
            detail: format!(
                "Paladin Damage Reduction granted at paladin level {level} (PF1 Core Rulebook, \
                 the DR clause of the 17th-level Aura of Righteousness): DR \
                 {PALADIN_AURA_OF_RIGHTEOUSNESS_DAMAGE_REDUCTION}/evil, verified against the \
                 feature's own corpus token `DR:5/Evil`. Flat -- PF1 grants the paladin no \
                 further DR tier at any later level, so this same value carries through 20. \
                 Grounds the magnitude and its /evil bypass condition; no damage-reduction \
                 APPLICATION engine exists here to subtract it from incoming damage, exactly \
                 the way class_feature.barbarian.damage_reduction, \
                 class_feature.acg.skald.damage_reduction and Fighter's Armor Mastery DR are \
                 already grounded. A DR value is a property of the character, not of any \
                 incoming attack"
            ),
        });
    }

    // Holy Champion: below the level-20 gate, this stays a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it (SD18 level-20
    // widening, the class capstone), it transitions to a bounded GRANT-only
    // identity record (mirroring the Aura of Justice / Aura of Faith / Aura
    // of Righteousness idiom exactly). No damage-reduction-application
    // engine, no banishment-spell-effect-resolution engine, and no
    // healing-maximization execution engine exists anywhere in this
    // codebase to apply this to.
    if level < PALADIN_HOLY_CHAMPION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.holy_champion".to_owned(),
            value: 0,
            detail: format!(
                "Paladin Holy Champion at paladin level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Holy Champion is the 20th-level paladin capstone."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.holy_champion".to_owned(),
            value: 0,
            detail: format!(
                "Paladin Holy Champion granted at paladin level {level} (PF1 Core Rulebook, \
                 20th-level paladin capstone): \"At 20th level, a paladin becomes a conduit for \
                 the power of her god. Her DR increases to 10/evil. Whenever she uses smite evil \
                 and successfully strikes an evil outsider, the outsider is also subject to a \
                 banishment, using her paladin level as the caster level... After the \
                 banishment effect and the damage from the attack is resolved, the smite \
                 immediately ends. In addition, whenever she channels positive energy or uses \
                 lay on hands to heal a creature, she heals the maximum possible amount.\" This \
                 is a bounded grant-only identity record only (value 0, non-fabricated): no \
                 damage-reduction-application engine, no banishment-spell-effect-resolution \
                 engine, and no healing-maximization execution engine exists anywhere in this \
                 codebase to apply any of this to."
            ),
        });
    }

    // SD13-E5: ground the partial-caster IDENTITY itself as one more flat
    // level-gate record, distinct from the still-ungrounded spell burden
    // named below. PF1 Core Rulebook: effective caster level = max(paladin
    // level - 3, 0); spells begin at paladin level 4. At level 1 this
    // correctly grounds to 0 — the same "correct absence" idiom already used
    // for lay on hands / divine grace / mercy above. This grounds only the
    // caster-level gate arithmetic; it fabricates no spells known, no spells
    // per day, no bonus spell slots, and no spell save DCs. The gate
    // genuinely widened at level 5 (to 2, up from 1 at level 4), widened
    // again at level 6 (to 3, up from 2 at level 5), and widens again at
    // level 7 (to 4, up from 3 at level 6), via the same pre-existing
    // formula, no re-derivation. Divine Bond, the level-5 "Special" column's
    // other entry, was checked against a primary source and confirmed to
    // require an activation/resource-consumption engine plus either a
    // weapon-enhancement subsystem or a full mount stat-block/advancement
    // subsystem, so it stays deliberately named-but-unproven -- no
    // explanation or diagnostic record is fabricated for it. Similarly, the
    // level-6 "Special" column's repeat "Mercy" entry (PF1 CRB: an additional
    // mercy becomes selectable at 6th level and every three levels
    // thereafter) was checked and confirmed to require a mercy-list-growth
    // mechanism this codebase has not already grounded (the existing mercy
    // grant/choice records are a single, ungated recognition, not a
    // per-level slot count), so it too stays deliberately named-but-unproven
    // -- no second mercy-choice explanation record is fabricated for it.
    // Level 7's own "Special" column reads "Smite evil 3/day" only (verified
    // independently against d20pfsrd and legacy.aonprd.com) -- level 7 is not
    // one of the repeat-Mercy-grant levels (3, 6, 9, ...), so nothing new is
    // left unproven for Mercy at level 7.

    // SD13-E5: the SECOND and THIRD mercies (gates 6/9), the repeat grants
    // the level-6/9 chassis slices deferred, now grounded as numbered choice
    // slots per the proven repeat-grant idiom — mirroring slot 1's
    // open-ended recognition (whichever raw mercy string was selected, no
    // tier-membership validation), with the verified CRB tier lists cited
    // in each detail. No mercy's effect is computed (no lay-on-hands
    // execution engine exists) and prerequisite chains (e.g. the frightened
    // mercy requiring the shaken mercy) are named, not validated.
    let repeat_mercy_slots: [(u8, u8, &str); 5] = [
        (2, PALADIN_SECOND_MERCY_GRANT_LEVEL, PALADIN_SECOND_MERCY_CHOICE_ID),
        (3, PALADIN_THIRD_MERCY_GRANT_LEVEL, PALADIN_THIRD_MERCY_CHOICE_ID),
        (4, PALADIN_FOURTH_MERCY_GRANT_LEVEL, PALADIN_FOURTH_MERCY_CHOICE_ID),
        (5, PALADIN_FIFTH_MERCY_GRANT_LEVEL, PALADIN_FIFTH_MERCY_CHOICE_ID),
        (6, PALADIN_SIXTH_MERCY_GRANT_LEVEL, PALADIN_SIXTH_MERCY_CHOICE_ID),
    ];
    for (slot_number, grant_level, choice_id) in repeat_mercy_slots {
        if level < grant_level {
            continue;
        }
        let Some(mercy) = choice_selection(input, choice_id) else {
            continue;
        };
        let tier_text = if slot_number == 2 {
            "the 6th-level CRB tier additions are Dazed, Diseased, and Staggered \
             (legacy.aonprd.com Core Rulebook text; d20pfsrd's superset contains them, its \
             extra entries being non-CRB expansions outside this pf1.core_rulebook seam)"
        } else if slot_number == 3 {
            "the 9th-level CRB tier additions are Cursed, Exhausted, Frightened, Nauseated, \
             and Poisoned (legacy.aonprd.com Core Rulebook text; d20pfsrd's superset \
             contains them); the rule text chains prerequisites — Exhausted requires the \
             fatigue mercy, Frightened requires the shaken mercy, Nauseated requires the \
             sickened mercy — which this bounded recognition names but does not validate"
        } else if slot_number == 4 {
            "the 12th-level CRB tier additions are Blinded, Deafened, Paralyzed, and Stunned \
             (legacy.aonprd.com Core Rulebook text; d20pfsrd's superset contains them, its \
             extra entries — Amputated, Ensorcelled, Petrified — being non-CRB expansions \
             outside this pf1.core_rulebook seam)"
        } else if slot_number == 5 {
            "unlike the 6th/9th/12th-level repeat grants, the 15th-level grant adds NO new \
             named mercy-list tier (verified independently against d20pfsrd and the Archives \
             of Nethys aonprd.com mirror, byte-for-byte agreement): the paladin simply selects \
             a fifth mercy from the already-existing 3rd/6th/9th/12th-tier pool"
        } else {
            "like the 15th-level grant, the 18th-level grant adds NO new named mercy-list tier \
             (verified independently against d20pfsrd and the Archives of Nethys aonprd.com \
             mirror, byte-for-byte agreement): the paladin simply selects a sixth mercy from the \
             already-existing 3rd/6th/9th/12th-tier pool"
        };
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.paladin.mercy_{slot_number}_choice"),
            value: 0,
            detail: format!(
                "Paladin mercy slot {slot_number} selection ({choice_id} -> {mercy}) at the \
                 level-{grant_level} repeat grant (PF1 Core Rulebook: a mercy at 3rd level \
                 and an additional mercy at 6th level and every three levels thereafter). \
                 The level-{level} selection for this slot is {mercy}, recognized as a \
                 bounded +0 record of the numbered choice slot only (open-ended raw string, \
                 mirroring slot 1 — no tier-membership validation); {tier_text}. The \
                 selected mercy's own effect on lay on hands is not computed — no \
                 lay-on-hands execution engine exists in this codebase"
            ),
        });
    }

    let paladin_effective_caster_level = (paladin_level - 3).max(0);
    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.partial_caster.effective_caster_level".to_owned(),
        value: paladin_effective_caster_level,
        detail: format!(
            "Paladin effective caster level at paladin level {level}: max(paladin level - 3, 0) = \
             max({paladin_level} - 3, 0) = {paladin_effective_caster_level} (PF1 Core Rulebook: \
             paladin spells begin at paladin level 4). This grounds only the caster-level gate \
             arithmetic; it computes no spells known, no spells per day, no bonus spell slots, \
             and no spell save DCs"
        ),
    });

    // SD13-E5: the partial-caster spell-level ACCESS ladder, mirroring the
    // Cleric/Wizard <CLASS>_<N>TH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL
    // threshold doctrine exactly ("first non-'—' spells-per-day column",
    // verified against the raw table rows of both primary sources, never
    // derived from the effective-caster-level arithmetic). This grounds the
    // highest ACCESSIBLE paladin spell level only; the per-day slot values
    // themselves ("0", "1", "2") are never computed, and the "0"-entry
    // bonus-spells-only nuance is surfaced in the record text.
    let paladin_spell_level_access: i16 = paladin_spell_level_access(level);
    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.partial_caster.spell_level_access".to_owned(),
        value: paladin_spell_level_access,
        detail: format!(
            "Paladin spell-level access at paladin level {level}: the highest paladin spell \
             level with a non-\"—\" spells-per-day column in the PF1 Core Rulebook Paladin \
             class table is {paladin_spell_level_access} (verified against the raw table rows \
             of both primary sources: 1st-level spells begin at paladin level \
             {PALADIN_FIRST_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 2nd-level at \
             {PALADIN_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 3rd-level at \
             {PALADIN_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 4th-level at \
             {PALADIN_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}). A gate-level \"0\" \
             spells-per-day entry is access via Charisma bonus spells only, per the PF1 rule \
             text. This grounds the access ladder only: no spell slot counts, no spells per \
             day, no spells known or prepared posture, no bonus slots from a high Charisma, \
             and no spell save DCs are computed"
        ),
    });

    // SD13-E5: the BASE spells-per-day counts, one record per ACCESSIBLE
    // spell level, as a literal table lookup mirroring the Cleric
    // domain-slot-count precedent — the PF1 spells-per-day table is a lookup
    // table, not arithmetic, so no formula is invented for it. Verified
    // against the raw table rows of both primary sources (identical on
    // d20pfsrd and legacy.aonprd.com): level 4 "0/—/—/—", level 5 "1/—/—/—",
    // level 6 "1/—/—/—", level 7 "1/0/—/—", level 8 "1/1/—/—", level 9
    // "2/1/—/—", level 10 "2/1/0/—", level 11 "2/1/1/—", level 12
    // "2/2/1/—", and (SD18 cycle-2026-07-15T1800) level 13 "3/2/1/0" —
    // verified independently against d20pfsrd and legacy.aonprd.com (a
    // third and fourth fetch disagreed with each other and with this
    // pairing, showing a nonsensical decrease from a "1" at level 12 to a
    // "0" at level 13 on the 4th-level column, rejected as a known
    // tool-extraction artifact since spells-per-day tables never decrease
    // with level, and the accepted 2/2/1/— -> 3/2/1/0 pairing matches the
    // already-landed Ranger level-13 widening's identical table shape), and
    // (SD18 cycle-2026-07-15T2500) level 14 "3/2/1/1" — verified
    // independently against d20pfsrd, the Archives of Nethys aonprd.com
    // mirror, and legacy.aonprd.com, all three agreeing byte-for-byte with
    // no self-contradictory fetches this time: only the 4th-level column
    // genuinely rises (from 0 to 1), the first castable 4th-level paladin
    // spell slot; and (SD18 cycle-2026-07-15T4300) level 15 "3/2/2/1" —
    // verified independently against d20pfsrd and the Archives of Nethys
    // aonprd.com mirror, byte-for-byte agreement with no disagreement or
    // self-contradiction, so a third source was not required: only the
    // 3rd-level column genuinely rises (from 1 to 2), while the 1st/2nd/
    // 4th-level columns stay 3/2/1 numerically unchanged; and (SD18
    // cycle-2026-07-16T2800) level 19 "4/3/3/2" — verified independently
    // against d20pfsrd (a raw HTML parse of the class table, bypassing
    // AI-summarization) and the Archives of Nethys aonprd.com mirror,
    // byte-for-byte agreement with no disagreement or self-contradiction,
    // so a third source was not required: only the 3rd-level column
    // genuinely rises (from 2 to 3), while the 1st/2nd/4th-level columns
    // stay 4/3/2 numerically unchanged; and (SD18 cycle-2026-07-16T1500)
    // level 20 "4/4/3/3" — verified independently against a raw `curl`
    // fetch of d20pfsrd.com's own class table HTML and a raw `curl` fetch
    // of the Archives of Nethys aonprd.com mirror's ClassDisplay.aspx HTML,
    // both bypassing AI-summarization, byte-for-byte agreement with no
    // disagreement: the 1st/3rd-level columns stay 4/3 numerically
    // unchanged, while the 2nd-level AND 4th-level columns BOTH genuinely
    // rise simultaneously (2nd from 3 to 4, 4th from 2 to 3) — the first
    // level in this row's own widening history where two columns rise at
    // once, a deliberate deviation from the single-column-rise pattern seen
    // at every level from 13 through 19, so both raw HTML fetches were
    // double-checked directly to guard against a tool-extraction artifact.
    // A "0" is a
    // genuine table entry (bonus-spells-only access), NOT an absence —
    // inaccessible spell levels ("—" columns) get no record at all. Only
    // the base counts are grounded: bonus spells per day from a high
    // Charisma are never computed.
    let paladin_base_spells_per_day: [Option<i16>; 4] = paladin_base_spells_per_day_table(level);
    for (index, base_count) in paladin_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = index + 1;
        let zero_nuance = if *base_count == 0 {
            " A base count of 0 is a genuine table entry, not an absence: per the PF1 rule \
             text, the paladin gains only the bonus spells she would be entitled to based on \
             her Charisma score for that spell level."
        } else {
            ""
        };
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.paladin.partial_caster.base_spells_per_day.spell_level_{spell_level}"
            ),
            value: *base_count,
            detail: format!(
                "Paladin base spells per day at paladin level {level}, spell level \
                 {spell_level}: {base_count}, read directly from the PF1 Core Rulebook \
                 Paladin class table's spells-per-day row (verified against the raw table \
                 rows of both primary sources; a literal table lookup, not a derived \
                 formula).{zero_nuance} This grounds the base count only: bonus spells per \
                 day from a high Charisma are never computed, no prepared posture or \
                 spell-source lineage is grounded, and no spell save DCs are computed"
            ),
        });
    }

    // SD13-E5: the base spell-save-DC arithmetic, one record per ACCESSIBLE
    // spell level, mirroring the Sorcerer/Bard DC slices. Verified against
    // both primary sources, which state the rule identically: "The
    // Difficulty Class for a saving throw against a paladin's spell is 10 +
    // the spell level + the paladin's Charisma modifier." This grounds only
    // the base formula over values already on the seam (the chosen-ability
    // Charisma modifier and the access ladder): no saving-throw resolution,
    // no target, no spell selection, and no feat DC modifiers are computed.
    for spell_level in 1..=paladin_spell_level_access {
        let spell_save_dc = 10 + spell_level + charisma_modifier;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.paladin.partial_caster.spell_save_dc.spell_level_{spell_level}"
            ),
            value: spell_save_dc,
            detail: format!(
                "Paladin spell save DC at paladin level {level}, spell level {spell_level}: \
                 10 + {spell_level} + Charisma modifier {charisma_modifier} = \
                 {spell_save_dc} (PF1 Core Rulebook, verified identically on both primary \
                 sources: \"The Difficulty Class for a saving throw against a paladin's \
                 spell is 10 + the spell level + the paladin's Charisma modifier\"). This \
                 grounds the base DC formula only: no saving-throw resolution, no target, no \
                 spell selection, and no feat DC modifiers are computed"
            ),
        });
    }

    // SD13-E5: the bonus spells per day from a high Charisma, one record
    // per ACCESSIBLE spell level, from PF1's shared Table: Ability
    // Modifiers and Bonus Spells, mirroring the Sorcerer/Bard bonus slices
    // — verified against both primary sources' ability-scores pages: for
    // modifier m and spell level N, 0 when m < N, otherwise (m - N)/4 + 1,
    // gated by the grounded access ladder. The paladin-specific rule text
    // ("she receives bonus spells per day if she has a high Charisma
    // score") was verified on both class pages. Together with the literal
    // "0" base entries this makes the bonus-spells-only access visible as
    // two grounded records side by side. The bonus is never added to the
    // base per-day counts here — no total is computed.
    for spell_level in 1..=paladin_spell_level_access {
        let bonus_spells = if charisma_modifier < spell_level {
            0
        } else {
            (charisma_modifier - spell_level) / 4 + 1
        };
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.paladin.partial_caster.bonus_spells_per_day.spell_level_{spell_level}"
            ),
            value: bonus_spells,
            detail: format!(
                "Paladin bonus spells per day at paladin level {level}, spell level \
                 {spell_level}: {bonus_spells} from Charisma modifier {charisma_modifier} \
                 (PF1 Core Rulebook Table: Ability Modifiers and Bonus Spells, verified \
                 identically on both primary sources; for modifier m and spell level N the \
                 table value is 0 when m < N, otherwise (m - N)/4 + 1, and bonus spells \
                 apply only to spell levels the character is of a high enough class level \
                 to cast — the grounded access ladder). A computed 0 means the modifier \
                 grants no bonus at this spell level; it is never added to the base per-day \
                 count here — no total is computed, no spell selection, and no spell save \
                 DCs"
            ),
        });
    }

    // SD13-E5: the TOTAL spells per day — the pure sum of the two records
    // grounded above (base table count + Charisma bonus count) per
    // ACCESSIBLE spell level, mirroring the Sorcerer/Bard total slices. No
    // new rules content: each input record carries its own two-source
    // verification. The level-4 "0"-base/1-bonus pair lands here as
    // arithmetic (total 1), and the level-10 3rd-level total is an honest
    // ZERO (a "0" base entry plus a modifier-below-spell-level 0 bonus):
    // accessible but currently uncastable. Counts only — no
    // prepared-posture selection, no casting execution, no slot
    // consumption or tracking, no save resolution.
    for (index, base_count) in paladin_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = (index + 1) as i16;
        let bonus_spells = if charisma_modifier < spell_level {
            0
        } else {
            (charisma_modifier - spell_level) / 4 + 1
        };
        let total_spells = base_count + bonus_spells;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_{spell_level}"
            ),
            value: total_spells,
            detail: format!(
                "Paladin total spells per day at paladin level {level}, spell level \
                 {spell_level}: base table count {base_count} + Charisma bonus \
                 {bonus_spells} = {total_spells} — the pure sum of the two separately \
                 grounded records (each carrying its own two-source verification), giving \
                 the actual castable slot count per day; a total of 0 is honest arithmetic \
                 (accessible spell level, no castable slots at this Charisma). This grounds \
                 the count only: no prepared-posture selection, no casting execution, no \
                 slot consumption or tracking, and no spell save resolution"
            ),
        });
    }
}

/// Paladin's Detect Evil (`core_rulebook:class_feature:paladin_detect_evil`,
/// `cr_abilities_class.lst:1356`): `DEFINE:DetectEvilLVL|0` /
/// `SPELLS:Class|TIMES=ATWILL|CASTERLEVEL=DetectEvilLVL|Detect Evil,11+WIS`
/// / `BONUS:VAR|DetectEvilLVL|PaladinLVL` -- `DetectEvilLVL` is the
/// paladin's own class level (no other producer sets it), used as the
/// at-will spell-like ability's caster level. Race-independent,
/// level-gate-only, the identical shape as the Antipaladin's own mirror
/// feature, `rules_tables::apg::antipaladin_features::
/// detect_good_caster_level` (`decisions.md §22`'s "FURTHER UPDATE,
/// 2026-09-04": this class had the identical structural precedent already
/// built for its own mirror class, just never symmetrically added here).
/// `None` below level 1 -- the class feature's own
/// `PREVARGTEQ:Paladin_CFP_Level,1` grant gate.
pub(super) fn paladin_detect_evil_caster_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// Grounds Paladin's Detect Evil for real. Deliberately the only Paladin
/// class-feature push in this file that runs unconditional on race and
/// single-class status -- unlike
/// `explain_paladin_level1_chassis_and_spell_burden_separation`'s own
/// Human-only/single-class-only gate elsewhere in this file
/// (`supported_paladin_level`'s own `input.chosen.race_id != HUMAN_RACE_ID`
/// check), a pure class-level pass-through
/// (`paladin_detect_evil_caster_level`'s own doc comment) has no reason to
/// inherit that narrower fixture's scope.
/// Fires for any character with a Paladin class level, multiclassed or
/// not, at any level -- `decisions.md §22`'s own note that "Paladin
/// currently has no `ground_paladin_class_features`-style push at all"
/// until this cycle.
pub(super) fn ground_paladin_detect_evil(input: &CharacterInput, explanations: &mut Vec<ComputationExplanation>) {
    let Some(level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == PALADIN_CLASS_ID)
        .map(|class_level| class_level.level)
    else {
        return;
    };
    if let Some(caster_level) = paladin_detect_evil_caster_level(level) {
        explanations.push(ComputationExplanation {
            id: "class_feature.paladin.detect_evil.caster_level".to_owned(),
            value: caster_level,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   `BONUS:VAR|DetectEvilLVL|PaladinLVL`
                "Paladin level {level} Detect Evil: at-will spell-like ability, caster level \
                 {caster_level} (a pure class-level pass-through; PF1 Core Rulebook \
                 `cr_abilities_class.lst`'s)"
            ),
        });
    }
}

/// The highest paladin spell level with a non-"—" spells-per-day column at
/// the given paladin level (0 means no spell access yet). Pure function,
/// race-independent -- mirrors `ranger_spell_level_access` exactly, extracted
/// so both the (Human-only) flat explanation block above and the real
/// prepared-spell validation below share one source of truth.
pub(super) fn paladin_spell_level_access(level: u8) -> i16 {
    if level >= PALADIN_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        4
    } else if level >= PALADIN_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        3
    } else if level >= PALADIN_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        2
    } else if level >= PALADIN_FIRST_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        1
    } else {
        0
    }
}

/// The PF1 Core Rulebook Paladin class table's BASE spells-per-day row, one
/// entry per spell level 1-4 (`None` for an inaccessible "—" column). A
/// literal table lookup, not a derived formula -- see
/// `explain_paladin_level1_chassis_and_spell_burden_separation`'s own doc
/// comment for the two-source verification history of every row. Pure
/// function, race-independent, mirrors `ranger_base_spells_per_day_table`.
pub(super) fn paladin_base_spells_per_day_table(level: u8) -> [Option<i16>; 4] {
    match level {
        4 => [Some(0), None, None, None],
        5 | 6 => [Some(1), None, None, None],
        7 => [Some(1), Some(0), None, None],
        8 => [Some(1), Some(1), None, None],
        9 => [Some(2), Some(1), None, None],
        10 => [Some(2), Some(1), Some(0), None],
        11 => [Some(2), Some(1), Some(1), None],
        12 => [Some(2), Some(2), Some(1), None],
        13 => [Some(3), Some(2), Some(1), Some(0)],
        14 => [Some(3), Some(2), Some(1), Some(1)],
        15 => [Some(3), Some(2), Some(2), Some(1)],
        16 => [Some(3), Some(3), Some(2), Some(1)],
        17 => [Some(4), Some(3), Some(2), Some(1)],
        18 => [Some(4), Some(3), Some(2), Some(2)],
        19 => [Some(4), Some(3), Some(3), Some(2)],
        20 => [Some(4), Some(4), Some(3), Some(3)],
        _ => [None, None, None, None],
    }
}

/// The real per-day slot budget per spell level 1-4 (base table count +
/// Charisma bonus, `None` for an inaccessible column), reusing
/// `ability_bonus_spells` -- mirrors `ranger_total_spells_per_day` exactly,
/// substituting Charisma for Wisdom.
pub(super) fn paladin_total_spells_per_day(level: u8, charisma_modifier: i16) -> [Option<i16>; 4] {
    let base = paladin_base_spells_per_day_table(level);
    let mut total = [None; 4];
    for (index, base_count) in base.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = (index + 1) as i16;
        total[index] = Some(base_count + ability_bonus_spells(charisma_modifier, spell_level));
    }
    total
}

/// Return the list of unmet conditions for Paladin's real prepared-spell
/// posture. Mirrors `unmet_ranger_prepared_spell_conditions` exactly,
/// substituting Charisma for Wisdom and `paladin_spell_list::PALADIN_SPELL_LIST`
/// for the ranger list. An empty list means the posture is fully valid: every
/// `AcquisitionMode::Prepared` selection with `source_class_id ==
/// "class:paladin"` names a real spell on
/// `paladin_spell_list::PALADIN_SPELL_LIST`, at a spell level within the
/// paladin's own access ceiling for their paladin level, and no spell
/// level's prepared count exceeds that level's total slot budget (base
/// table count + Charisma bonus). Zero prepared spells is always valid --
/// see the call site's own doc comment for why.
pub(super) fn unmet_paladin_prepared_spell_conditions(
    input: &CharacterInput,
    paladin_level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Vec<String> {
    let mut unmet = Vec::new();

    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == PALADIN_CLASS_ID
                && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    let access_ceiling = paladin_spell_level_access(paladin_level);
    let total_per_day = paladin_total_spells_per_day(paladin_level, ability_modifiers.charisma);

    let mut consumed_per_level: [i16; 4] = [0; 4];
    for spell_id in &prepared {
        let Some(spell_level) = paladin_spell_list::paladin_spell_level(spell_id) else {
            unmet.push(format!(
                "prepared spell '{spell_id}' is not on the real PF1 paladin spell list"
            ));
            continue;
        };
        if i16::from(spell_level) > access_ceiling {
            unmet.push(format!(
                "prepared spell '{spell_id}' targets spell level {spell_level}, not yet \
                 accessible at paladin level {paladin_level} (access ceiling {access_ceiling})"
            ));
            continue;
        }
        consumed_per_level[usize::from(spell_level) - 1] += 1;
    }

    for (index, consumed) in consumed_per_level.iter().enumerate() {
        if *consumed == 0 {
            continue;
        }
        let spell_level = index + 1;
        let total_slots = total_per_day[index].unwrap_or(0);
        if *consumed > total_slots {
            unmet.push(format!(
                "spell level {spell_level} over-prepared: {consumed} spells prepared but only \
                 {total_slots} slots available (base {} + Charisma bonus)",
                paladin_base_spells_per_day_table(paladin_level)[index].unwrap_or(0)
            ));
        }
    }

    unmet
}

/// Ground the real prepared-spell posture once
/// `unmet_paladin_prepared_spell_conditions` reports an empty unmet list.
/// Mirrors `ground_ranger_prepared_spells` exactly, substituting Charisma
/// for Wisdom.
pub(super) fn ground_paladin_prepared_spells(
    input: &CharacterInput,
    paladin_level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == PALADIN_CLASS_ID
                && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.paladin.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Paladin level {paladin_level} daily preparation selection ({} spells, \
             AcquisitionMode::Prepared): {}. Each prepared spell is verified against the real \
             PF1 paladin spell list (`paladin_spell_list::PALADIN_SPELL_LIST`, all ingested \
             books), \
             the paladin's own spell-level access ceiling, and the per-level slot budget (base \
             table count + Charisma bonus). This grounds the prepared-spell selection for real; \
             it computes no spell save DC resolution against a target and no casting execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let total_per_day = paladin_total_spells_per_day(paladin_level, ability_modifiers.charisma);
    for (index, total) in total_per_day.iter().enumerate() {
        let Some(total) = total else {
            continue;
        };
        let spell_level = index + 1;
        explanations.push(ComputationExplanation {
            id: format!("class_spell.paladin.total_spells_per_day.spell_level_{spell_level}"),
            value: *total,
            detail: format!(
                "Paladin level {paladin_level} total spells per day at spell level \
                 {spell_level}: {total} (base table count + Charisma bonus, the same records \
                 already grounded as \
                 `class_chassis.paladin.partial_caster.total_spells_per_day.spell_level_{spell_level}` \
                 for a Human paladin, computed here independent of race). This is the real slot \
                 budget the daily preparation selection above is validated against"
            ),
        });
    }
}

/// The bounded Ranger milestone level this decomposition surface grounds, if any.
/// Returns the single Ranger level when the chosen input is exactly a single-class
/// Ranger at one of the supported milestone levels (1 through 10). Returns
/// `None` for no Ranger, a non-Ranger class, a multiclass mix, the Paladin hybrid
/// (which has its own decomposition lane), or any level-11+ Ranger this slice
/// deliberately does not recognize — each of which stays claim-blocked exactly as
/// before. Mirrors the
/// Fighter `supported_fighter_level` / Paladin `supported_paladin_level` / Rogue
/// `supported_rogue_level` / Barbarian `supported_barbarian_level` / Monk
/// `supported_monk_level` / Cleric `supported_cleric_level` / Bard
/// `supported_bard_level` / Druid `supported_druid_level` / Sorcerer
/// `supported_sorcerer_level` / Wizard `supported_wizard_level` level-range gate
/// idiom.
pub(super) fn supported_ranger_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == RANGER_CLASS_ID
                && (1..=MAX_SUPPORTED_RANGER_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E3 runtime evidence for the deterministic Human Ranger
/// level-1 chassis as a per-pillar decomposition of the F6 combined non-spell
/// class-feature blocker, grounding all three named pillars for real (Track by
/// the SD13-E3 slice, the Favored Enemy flat surface by the SD13-E5 slice, and
/// the combat style level-gate absence by a later SD13-E5 slice), plus the
/// foundational base-attack-bonus / base-save progression pillar grounded by a
/// later SD13-E5 slice still.
///
/// This sits on top of the accepted SD13-F6 hybrid baseline: F6 already proves
/// the deterministic Human Ranger level-1 hybrid identity is acknowledged on the
/// compute seam and emits a single combined non-spell class-feature blocker
/// (naming favored enemy, combat style, and skill/tracking together) plus a
/// single combined later-spell blocker. This slice proves the per-pillar
/// separation Ranger actually needs:
///
/// - one grounded numeric explanation set (SD13-E5) for the foundational
///   base-attack-bonus / base-save progression pillar, verified against the PF1
///   Core Rulebook Ranger class table (d20pfsrd and legacy.aonprd.com), reading
///   the raw level 1-5 table rows directly and cross-checking the level 4/5
///   base-attack-bonus values to disambiguate full BAB from 3/4 BAB (level 1
///   alone does not disambiguate): full BAB (classlevel), good Fortitude, good
///   Reflex, poor Will (`classlevel/2+2` for the two good saves, `classlevel/3`
///   for the poor save). Grounded as flat, standalone `ComputationExplanation`
///   records, mirroring the Barbarian/Monk/Druid/Cleric/Bard/Sorcerer/Wizard
///   "not wired into `PilotBaseChassisComputation.base_attack_bonus`,
///   `compute_total_saves`, or `compute_combat_baseline`" idiom. Ranger level 2+
///   progression stays deliberately out of scope for this slice.
///
/// - one grounded level-gate explanation (value 0) for the combat style
///   pillar, which retires the `class_feature.ranger.combat_style.unsupported`
///   blocker and corrects a mistaken framing that blocker carried: PF1 Core
///   Rulebook grants the archery-vs-two-weapon-combat style choice and its
///   first bonus feat TOGETHER at 2nd level (`RANGER_COMBAT_STYLE_LEVEL`) —
///   they are not separable into a level-1 style choice plus a level-2 feat
///   grant, as the retired diagnostic incorrectly claimed. At the bounded
///   level-1 baseline this correctly grounds to a value-0 ABSENCE, mirroring
///   the Paladin mercy level-gate idiom (`class_chassis.paladin.level_gate.mercy`);
///   the at-grant selection is named but not computed, and no bonus-feat
///   mechanical value is fabricated.
///
/// - one grounded explanation for the Track pillar, computed for real:
///   the Survival-check bonus to follow tracks equals `max(ranger level / 2, 1)`
///   (PF1 Core Rulebook Track: +1/2 ranger level, minimum +1), which is `1` at
///   the bounded level-1 baseline. This grounds only the flat numeric Track
///   bonus, not a tracking-check execution engine: no full Survival check, no
///   DC resolution, and no tracking narrative is computed.
///
/// - the grounded Favored Enemy FLAT surface (SD13-E5), which retires the
///   `class_feature.ranger.favored_enemy.unsupported` blocker:
///   * recognition of the chosen favored-enemy type from the
///     `choice:ranger_favored_enemy` selection when it is present in chosen
///     input (a +0 recognition record; nothing is fabricated when the choice
///     is absent),
///   * the flat +2 bonus on Bluff, Knowledge, Perception, Sense Motive, and
///     Survival checks against the favored enemy (PF1 CRB level 1), and
///   * the flat +2 bonus on weapon attack AND damage rolls against the
///     favored enemy (PF1 includes attack rolls, unlike D&D 3.5).
///
///   Only the flat magnitudes are grounded: no target-type matching and no
///   conditional-application engine decides whether any specific check or
///   attack is actually made against the favored enemy.
///
/// - a still later SD13-E5 slice widens the level-range gate to level 3
///   (`MAX_SUPPORTED_RANGER_LEVEL`), extending base attack/base save/Track/the
///   Favored Enemy flat surface to level 3 via the same formulas (no
///   re-derivation), and grounds Endurance, the PF1 CRB's 3rd-level Ranger
///   class feature, as a bounded grant-only identity record (value 0): the
///   ranger gains Endurance as a bonus feat automatically, with no player
///   choice involved.
///
/// - a still later SD13-E5 slice grounds Favored Terrain, the class table's
///   other 3rd-level "Special" column entry, once a `choice:ranger_favored_terrain`
///   choice-slot exists in chosen input: a `+0` recognition record naming
///   whichever terrain was selected (mirroring the Favored Enemy choice-recognition
///   idiom exactly — raw string interpolation, no restricted-list validation), and
///   the rule's own flat `+2` magnitude on Initiative/Knowledge (geography)/
///   Perception/Stealth/Survival checks made in the chosen terrain, grounded as a
///   standalone, non-applied record, level-gated at 3rd level exactly like
///   Endurance. No terrain-detection engine and no application of the `+2` to any
///   actual Initiative total or skill-check total is grounded here.
///
/// - a still later SD13-E5 slice widens the level-range gate once more to level
///   4 (`MAX_SUPPORTED_RANGER_LEVEL`), extending base attack/base save/Track/the
///   Favored Enemy flat surface to level 4 via the same formulas (no
///   re-derivation; PF1 Core Rulebook only increases the Favored Enemy bonus at
///   5th ranger level and beyond, so it stays the flat `+2` through level 4),
///   and grounds Hunter's Bond, the class table's 4th-level "Special" column
///   entry: a restricted two-option choice recognition
///   (`choice:ranger_hunters_bond` -> `form:bond` or `form:companion`, mirroring
///   the combat-style choice idiom) is grounded as a `+0` record, an
///   unconditional grant-only identity record (mirroring the Endurance/Favored
///   Terrain idiom) is emitted once the level-4 gate is reached, and -- only
///   when the "bond" form is chosen -- the rule's own flat magnitude (half the
///   already-grounded Favored Enemy bonus) is grounded as a standalone,
///   non-applied record. No move-action/action-economy engine, no
///   ally-range-and-perception check, and no favored-enemy target-type matching
///   is implemented; the "companion" form's own animal-companion stat
///   block/advancement subsystem is deliberately left named-but-unproven.
///
/// - a still later SD13-E5 slice widens the level-range gate once more to level
///   5 (`MAX_SUPPORTED_RANGER_LEVEL`), extending base attack/base save/Track to
///   level 5 via the same formulas (no re-derivation), and grounds the Favored
///   Enemy rule's own 5th-level interval, the class table's 5th-level "Special"
///   column entry ("2nd favored enemy", verified independently against both
///   primary sources): the rule text is "At 5th level and every five levels
///   thereafter... the ranger may select an additional favored enemy. In
///   addition, at each such interval, the bonus against any one favored enemy
///   (including the one just selected, if so desired) increases by 2" — NOT an
///   automatic bump to the first favored enemy. This grounds three things: a
///   second favored-enemy TYPE selection (`choice:ranger_favored_enemy_2`,
///   mirroring the first favored enemy's own open-ended choice-recognition
///   idiom, plus the same flat `+2` base magnitude formula), a restricted
///   two-option choice recognizing WHICH one favored enemy is the
///   bonus-increase target (`choice:ranger_favored_enemy_bonus_increase_target`
///   -> `enemy:first` or `enemy:second`, mirroring the Hunter's Bond/combat-style
///   restricted two-option idiom), and the resulting `+4` magnitude applied only
///   to whichever favored enemy the target choice actually names — absent an
///   explicit target selection, both favored enemies stay the flat `+2`, since
///   nothing is fabricated about which one the ranger picked. Endurance,
///   Favored Terrain, combat style, and Hunter's Bond all stay granted at level
///   5, not re-derived; Hunter's Bond's own ally-bonus magnitude (half the
///   FIRST favored enemy's bonus) naturally recomputes from the same unchanged
///   formula once that magnitude widens to `+4`.
///
/// This deliberately does not compute a supported class-feature surface. It
/// grounds no favored-enemy conditional application, no combat-style feat
/// grant, no animal companion, no favored-terrain breadth beyond the
/// grounded 8th/13th-level intervals (the level-18th additional-terrain and
/// bonus-increase progression stays out of scope), no Hunter's Bond
/// ally-bonus application or animal-companion stat block, and no spell posture.
/// It only emits the grounded Track / Favored Enemy / combat-style / Endurance /
/// Favored Terrain / Hunter's Bond level-gate values that prove the F6 surface
/// remains separable on the runtime path.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input;
/// the F6 hybrid chassis emission already preserves a single class-feature
/// blocker and a single spell blocker. This seam adds per-pillar granularity
/// next to the F6 surface, never replacing it, so the F6 acceptance test
/// continues to pass.
pub(super) fn explain_ranger_level1_chassis_and_class_feature_separation(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // v0.6 alpha swarm, risks item 8 (2026-07-24, real spell posture landed
    // 2026-07-24): Ranger's partial-caster spell posture is validated
    // regardless of whether Ranger appears alone or in a multiclass mix --
    // checked BEFORE the single-class-only gate below (`supported_ranger_level`
    // requires an exact one-element `class_levels` slice) specifically so a
    // Ranger+Fighter/Wizard/Rogue multiclass cannot silently bypass it. This
    // matters because `table_class_id` recognizing Ranger makes
    // `multiclass_class_level_supported`/`is_supported_multiclass_mix`
    // newly accept a Ranger-containing mix too -- `compute_multiclass_base_chassis`
    // deliberately discards each isolated per-class sub-computation's own
    // diagnostics (see that function's own doc comment), so nothing else in
    // the multiclass path would ever surface this burden.
    //
    // Unlike the earlier bounded slice (which pushed this diagnostic
    // unconditionally), the posture is now genuinely computed:
    // `unmet_ranger_prepared_spell_conditions` validates every
    // `AcquisitionMode::Prepared` selection with `source_class_id ==
    // "class:ranger"` against the real `ranger_spell_list::RANGER_SPELL_LIST`
    // (spell-list membership), the character's own spell-level access
    // ceiling for their ranger level, and the per-level slot budget (base +
    // Wisdom bonus, both already grounded elsewhere in this function). A
    // Ranger with zero prepared spells is NOT an unmet condition -- unlike
    // Wizard's bounded slice (which requires at least one recorded and one
    // prepared spell before ever leaving `Blocked`), real PF1 rules do not
    // require a caster to fill every slot, and Ranger spells aren't
    // accessible at all before ranger level 4: requiring a non-empty
    // preparation would leave every level 1-3 Ranger (the most common case,
    // and the exact one frontend's live-verification found broken)
    // permanently blocked. Real PF1 also has no "recorded in a personal
    // spellbook" step for Ranger (unlike Wizard) -- a Ranger prepares
    // directly from the full ranger spell list each day, so this validates
    // `Prepared` selections directly with no prior `Known` requirement.
    if let Some(ranger_level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == RANGER_CLASS_ID)
        .map(|class_level| class_level.level)
    {
        let unmet = unmet_ranger_prepared_spell_conditions(input, ranger_level, ability_modifiers);
        if unmet.is_empty() {
            ground_ranger_prepared_spells(input, ranger_level, ability_modifiers, explanations);
        } else {
            diagnostics.push(ComputationDiagnostic {
                id: "class_spell.ranger.partial_caster.unsupported".to_owned(),
                message: format!(
                    "Ranger remains blocked on its divine, Wisdom-based partial-caster spell \
                     burden: Ranger is a partial caster (spells begin at ranger level 4, with \
                     effective caster level = ranger level - 3 in PF1 Core Rulebook); unmet \
                     prepared-spell posture: {}",
                    unmet.join("; ")
                ),
                claim_blocking: true,
            });
        }
    }

    let Some(level) = supported_ranger_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Grounded (SD13-E5): the foundational base-attack-bonus / base-save progression
    // pillar. Unlike every other class row in this matrix (Fighter, Barbarian, Monk,
    // Rogue, Paladin, Druid, Cleric, Bard, Sorcerer, Wizard all already ground this
    // pillar), Ranger had never had it grounded at all until this slice. Both
    // formulas were verified against the PF1 Core Rulebook Ranger class table
    // (d20pfsrd and the legacy Paizo PRD mirror) before writing this code, reading
    // the raw level 1-5 table rows directly (BAB +1/+2/+3/+4/+5, Fort +2/+3/+3/+4/+4,
    // Ref +2/+3/+3/+4/+4, Will +0/+0/+1/+1/+1) and cross-checking the level 4/5
    // base-attack-bonus values to disambiguate the exact fraction: a full-BAB
    // progression shows +4/+5 at those levels, while a 3/4-BAB progression would show
    // +3/+3 -- the table confirms full BAB, the same shape as Fighter/Barbarian/
    // Paladin. A later SD13-E5 slice widens the level-1-only gate to level 2
    // (`supported_ranger_level`, 1..=MAX_SUPPORTED_RANGER_LEVEL), extending both
    // formulas via the same shape (no re-derivation) and finally grounding the
    // combat-style pillar for real at the 2nd-level gate it was always named for.
    // A still later SD13-E5 slice widens the gate again to level 3, extending both
    // formulas once more and grounding Endurance (a grant-only identity record) and
    // Favored Terrain (a choice recognition record plus a flat +2 magnitude record).
    // A still later SD13-E5 slice widens the gate again to level 4, extending both
    // formulas once more (favored enemy stays flat +2 through level 4; PF1 CRB only
    // increases it at 5th ranger level and beyond) and grounding Hunter's Bond (a
    // restricted two-option choice recognition, a grant-only identity record, and --
    // for the "bond" form only -- a flat, non-applied ally-bonus magnitude record).
    // A still later SD13-E5 slice widens the gate again to level 5, extending both
    // formulas once more and grounding the Favored Enemy rule's own 5th-level
    // interval (a second favored-enemy selection plus a restricted-choice
    // bonus-increase target). A still later SD13-E5 slice widens the gate once more
    // to level 6, extending both formulas once more (Track genuinely rises to 3) and
    // grounding the ranger's SECOND combat-style bonus feat (a restricted-list
    // choice recognition gated on the same style already chosen at 2nd level,
    // mirroring the first bonus feat's own grounding idiom exactly). A still later
    // SD13-E5 slice widens the gate once more to level 7, extending both formulas
    // once more (both stay numerically unchanged from level 6, integer-division
    // coincidences) and grounding Woodland Stride (a grant-only identity record,
    // no numeric magnitude, mirroring the Endurance idiom).
    // Ranger level 8+ progression, the favored-enemy conditional-application engine,
    // either combat-style bonus feat's own mechanics, the level-8th/13th/18th
    // Favored Terrain breadth, Hunter's Bond ally-bonus
    // application/animal-companion stat block, and the ranger spell burden remain
    // deliberately out of scope.
    let level_value = i16::from(level);

    // Grounded (1/2): full-BAB base-attack progression, the same formula shape as
    // Fighter/Barbarian/Paladin (classlevel). No PCGen .lst file exists for the
    // Ranger class in this repo, so the formula cites the PF1 Core Rulebook Ranger
    // class table directly.
    let base_attack_bonus = level_value;
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Ranger level {level} base attack bonus from the PF1 Core Rulebook \
             Ranger class table (full base-attack progression, the same formula shape as \
             Fighter/Barbarian/Paladin): classlevel = {base_attack_bonus}. This is a standalone \
             explanation record; it is not wired into the integrated base_attack_bonus field or \
             into compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — good Fortitude, good Reflex, poor
    // Will, verified against the PF1 Core Rulebook Ranger class table (Fortitude +2,
    // Reflex +2, Will +0 at level 1; +4/+4/+1 at level 4, confirming the same
    // formula shape).
    let good_save = level_value / 2 + 2;
    let poor_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.base_save.fortitude".to_owned(),
        value: good_save,
        detail: format!(
            "Ranger level {level} base Fortitude save (good save) from the PF1 \
             Core Rulebook Ranger class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.base_save.reflex".to_owned(),
        value: good_save,
        detail: format!(
            "Ranger level {level} base Reflex save (good save) from the PF1 Core \
             Rulebook Ranger class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.base_save.will".to_owned(),
        value: poor_save,
        detail: format!(
            "Ranger level {level} base Will save (poor save) from the PF1 Core \
             Rulebook Ranger class table: classlevel/3 = {poor_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });

    if level < RANGER_COMBAT_STYLE_LEVEL {
        // Combat style is a correct ABSENCE below the 2nd-level gate, grounded as a
        // level-gate explanation (value 0), mirroring the Paladin mercy idiom. The
        // former `class_feature.ranger.combat_style.unsupported` blocker is retired: it
        // incorrectly claimed the archery-vs-two-weapon-combat style choice was a
        // level-1 decision separate from a level-2 bonus-feat grant. PF1 Core Rulebook
        // actually grants the style choice and its first bonus feat TOGETHER at 2nd
        // level (RANGER_COMBAT_STYLE_LEVEL), so below that gate there is nothing to
        // recognize: no style is chosen and no bonus feat is granted.
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.level_gate.combat_style".to_owned(),
            value: 0,
            detail: format!(
                "Ranger combat style at ranger level {level}: correctly absent at \
                 level {level} by PF1 CRB level gate; at-grant selection named but not \
                 computed. Combat Style Feat is a {RANGER_COMBAT_STYLE_LEVEL}nd-level ranger feature: \
                 the ranger selects one combat style (archery or two-weapon combat) and gains its \
                 first bonus feat together at {RANGER_COMBAT_STYLE_LEVEL}nd level -- the style choice \
                 and the bonus-feat grant are not separable into a level-1 decision plus a level-2 \
                 grant. (Correction: an earlier version of this record, \
                 `class_feature.ranger.combat_style.unsupported`, incorrectly described the style \
                 choice as a level-1 decision distinct from the level-2 bonus-feat grant; PF1 Core \
                 Rulebook grants both together at 2nd level.)"
            ),
        });
    } else {
        // Grounded (SD13-E5 level-2 widening): Combat Style Feat is finally grounded
        // for real at the gate it was always named for. Both the STYLE CHOICE and its
        // restricted-list BONUS FEAT are recognized as chosen-input identity only
        // (+0 each), mirroring the Monk bonus-feat-choice idiom exactly: no style's
        // or feat's own mechanical effect is computed anywhere in this codebase.
        // Nothing is fabricated when the fixture carries no
        // `choice:ranger_combat_style` selection -- mirroring the Favored Enemy
        // choice-absence idiom below.
        let style_selection = choice_selection(input, RANGER_COMBAT_STYLE_CHOICE_ID);
        let style_name = style_selection.and_then(|selection| {
            if selection == RANGER_COMBAT_STYLE_ARCHERY_SELECTION {
                Some("Archery")
            } else if selection == RANGER_COMBAT_STYLE_TWO_WEAPON_COMBAT_SELECTION {
                Some("Two-Weapon Combat")
            } else {
                None
            }
        });

        if let Some(selection) = style_selection {
            let detail = if let Some(style) = style_name {
                format!(
                    "Ranger combat style selection at ranger level {level} \
                     ({RANGER_COMBAT_STYLE_CHOICE_ID} -> {selection}): names {style}, one of the \
                     two PF1 Core Rulebook combat styles (Archery or Two-Weapon Combat) granted \
                     together with its first bonus feat at {RANGER_COMBAT_STYLE_LEVEL}nd level. \
                     This is a recognition record of the choice slot only (+0): {style}'s own \
                     bonus-feat mechanics are not grounded here, and no feat-selection or \
                     feat-effect engine exists in this codebase"
                )
            } else {
                format!(
                    "Ranger combat style selection at ranger level {level} is present \
                     ({RANGER_COMBAT_STYLE_CHOICE_ID} -> {selection}), but only the PF1 Core \
                     Rulebook restricted pair (Archery, Two-Weapon Combat) is recognized on this \
                     bounded seam; no style identity is grounded and no mechanical value is \
                     fabricated (+0)"
                )
            };
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.combat_style_choice".to_owned(),
                value: 0,
                detail,
            });

            // The bonus feat is recognized only once the style itself is recognized,
            // since the restricted feat list to validate against depends on which
            // style was chosen -- mirroring how Cleric's domain powers are gated on
            // the domain choice itself being recognized.
            if let Some(style) = style_name
                && let Some(feat_selection) =
                    choice_selection(input, RANGER_COMBAT_STYLE_BONUS_FEAT_CHOICE_ID)
            {
                let recognized_feat_name = if style == "Archery" {
                    if feat_selection == FAR_SHOT_FEAT_SELECTION {
                        Some("Far Shot")
                    } else if feat_selection == POINT_BLANK_SHOT_FEAT_SELECTION {
                        Some("Point-Blank Shot")
                    } else if feat_selection == PRECISE_SHOT_FEAT_SELECTION {
                        Some("Precise Shot")
                    } else if feat_selection == RAPID_SHOT_FEAT_SELECTION {
                        Some("Rapid Shot")
                    } else {
                        None
                    }
                } else if feat_selection == DOUBLE_SLICE_FEAT_SELECTION {
                    Some("Double Slice")
                } else if feat_selection == IMPROVED_SHIELD_BASH_FEAT_SELECTION {
                    Some("Improved Shield Bash")
                } else if feat_selection == QUICK_DRAW_FEAT_SELECTION {
                    Some("Quick Draw")
                } else if feat_selection == TWO_WEAPON_FIGHTING_FEAT_SELECTION {
                    Some("Two-Weapon Fighting")
                } else {
                    None
                };

                let detail = if let Some(feat_name) = recognized_feat_name {
                    format!(
                        "Ranger {style} combat style bonus feat at ranger level {level} \
                         ({RANGER_COMBAT_STYLE_BONUS_FEAT_CHOICE_ID} -> {feat_selection}) \
                         names {feat_name}, drawn from the PF1 Core Rulebook {style} combat \
                         style's own {RANGER_COMBAT_STYLE_LEVEL}nd-level restricted feat list. \
                         This is a recognition record of the choice slot only, so it carries \
                         no fabricated mechanical value (+0): {feat_name}'s own mechanics (an \
                         attack/damage-range bonus, a two-weapon penalty reduction, or similar, \
                         depending on the feat) are not grounded here, and no such execution \
                         engine exists in this codebase"
                    )
                } else {
                    format!(
                        "Ranger combat style bonus feat at ranger level {level} is present \
                         ({RANGER_COMBAT_STYLE_BONUS_FEAT_CHOICE_ID} -> {feat_selection}), but \
                         only the PF1 Core Rulebook {style} combat style's own \
                         {RANGER_COMBAT_STYLE_LEVEL}nd-level restricted feat list is recognized \
                         on this bounded seam; no restricted-list feat identity is grounded and \
                         no mechanical value is fabricated (+0)"
                    )
                };
                explanations.push(ComputationExplanation {
                    id: "class_chassis.ranger.combat_style_bonus_feat_choice".to_owned(),
                    value: 0,
                    detail,
                });
            }

            // SD13-E5 level-6 widening: the ranger's SECOND combat-style bonus feat,
            // gated on both the level-6 milestone and the same style already
            // recognized above. Mirrors the first bonus feat's grounding idiom
            // exactly (a restricted-list choice recognition, +0, no mechanical
            // effect computed), validated against each style's own 6th-level list
            // only (Archery: Improved Precise Shot, Manyshot; Two-Weapon Combat:
            // Improved Two-Weapon Fighting, Two-Weapon Defense) rather than the
            // cumulative 2nd+6th-level list.
            if let Some(style) = style_name
                && level >= RANGER_COMBAT_STYLE_BONUS_FEAT_2_LEVEL
                && let Some(feat_selection_2) =
                    choice_selection(input, RANGER_COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID)
            {
                let recognized_feat_name_2 = if style == "Archery" {
                    if feat_selection_2 == IMPROVED_PRECISE_SHOT_FEAT_SELECTION {
                        Some("Improved Precise Shot")
                    } else if feat_selection_2 == MANYSHOT_FEAT_SELECTION {
                        Some("Manyshot")
                    } else {
                        None
                    }
                } else if feat_selection_2 == IMPROVED_TWO_WEAPON_FIGHTING_FEAT_SELECTION {
                    Some("Improved Two-Weapon Fighting")
                } else if feat_selection_2 == TWO_WEAPON_DEFENSE_FEAT_SELECTION {
                    Some("Two-Weapon Defense")
                } else {
                    None
                };

                let detail_2 = if let Some(feat_name) = recognized_feat_name_2 {
                    format!(
                        "Ranger {style} SECOND combat style bonus feat at ranger level {level} \
                         ({RANGER_COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID} -> {feat_selection_2}) \
                         names {feat_name}, drawn from the PF1 Core Rulebook {style} combat \
                         style's own {RANGER_COMBAT_STYLE_BONUS_FEAT_2_LEVEL}th-level restricted \
                         feat list (verified independently against d20pfsrd and \
                         legacy.aonprd.com: the ranger's combat style grants bonus feats at 2nd, \
                         6th, 10th, 14th, and 18th level). This is a recognition record of the \
                         choice slot only, so it carries no fabricated mechanical value (+0): \
                         {feat_name}'s own mechanics are not grounded here, and no such \
                         execution engine exists in this codebase"
                    )
                } else {
                    format!(
                        "Ranger SECOND combat style bonus feat at ranger level {level} is \
                         present ({RANGER_COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID} -> \
                         {feat_selection_2}), but only the PF1 Core Rulebook {style} combat \
                         style's own {RANGER_COMBAT_STYLE_BONUS_FEAT_2_LEVEL}th-level restricted \
                         feat list is recognized on this bounded seam; no restricted-list feat \
                         identity is grounded and no mechanical value is fabricated (+0)"
                    )
                };
                explanations.push(ComputationExplanation {
                    id: "class_chassis.ranger.combat_style_bonus_feat_2_choice".to_owned(),
                    value: 0,
                    detail: detail_2,
                });
            }

            // SD13-E5 level-10 widening: the ranger's THIRD combat-style bonus
            // feat, gated on both the level-10 milestone and the same style
            // already recognized above. Mirrors the first and second bonus
            // feats' grounding idiom exactly (a restricted-list choice
            // recognition, +0, no mechanical effect computed), validated
            // against each style's own 10th-level list only (Archery: Pinpoint
            // Targeting, Shot on the Run; Two-Weapon Combat: Greater
            // Two-Weapon Fighting, Two-Weapon Rend — verified independently
            // against d20pfsrd and legacy.aonprd.com) rather than the
            // cumulative 2nd+6th+10th-level list.
            if let Some(style) = style_name
                && level >= RANGER_COMBAT_STYLE_BONUS_FEAT_3_LEVEL
                && let Some(feat_selection_3) =
                    choice_selection(input, RANGER_COMBAT_STYLE_BONUS_FEAT_3_CHOICE_ID)
            {
                let recognized_feat_name_3 = if style == "Archery" {
                    if feat_selection_3 == PINPOINT_TARGETING_FEAT_SELECTION {
                        Some("Pinpoint Targeting")
                    } else if feat_selection_3 == SHOT_ON_THE_RUN_FEAT_SELECTION {
                        Some("Shot on the Run")
                    } else {
                        None
                    }
                } else if feat_selection_3 == GREATER_TWO_WEAPON_FIGHTING_FEAT_SELECTION {
                    Some("Greater Two-Weapon Fighting")
                } else if feat_selection_3 == TWO_WEAPON_REND_FEAT_SELECTION {
                    Some("Two-Weapon Rend")
                } else {
                    None
                };

                let detail_3 = if let Some(feat_name) = recognized_feat_name_3 {
                    format!(
                        "Ranger {style} THIRD combat style bonus feat at ranger level {level} \
                         ({RANGER_COMBAT_STYLE_BONUS_FEAT_3_CHOICE_ID} -> {feat_selection_3}) \
                         names {feat_name}, drawn from the PF1 Core Rulebook {style} combat \
                         style's own {RANGER_COMBAT_STYLE_BONUS_FEAT_3_LEVEL}th-level restricted \
                         feat list (verified independently against d20pfsrd and \
                         legacy.aonprd.com: the ranger's combat style grants bonus feats at 2nd, \
                         6th, 10th, 14th, and 18th level). This is a recognition record of the \
                         choice slot only, so it carries no fabricated mechanical value (+0): \
                         {feat_name}'s own mechanics are not grounded here, and no such \
                         execution engine exists in this codebase"
                    )
                } else {
                    format!(
                        "Ranger THIRD combat style bonus feat at ranger level {level} is \
                         present ({RANGER_COMBAT_STYLE_BONUS_FEAT_3_CHOICE_ID} -> \
                         {feat_selection_3}), but only the PF1 Core Rulebook {style} combat \
                         style's own {RANGER_COMBAT_STYLE_BONUS_FEAT_3_LEVEL}th-level restricted \
                         feat list is recognized on this bounded seam; no restricted-list feat \
                         identity is grounded and no mechanical value is fabricated (+0)"
                    )
                };
                explanations.push(ComputationExplanation {
                    id: "class_chassis.ranger.combat_style_bonus_feat_3_choice".to_owned(),
                    value: 0,
                    detail: detail_3,
                });
            }

            // SD18 cycle-2026-07-15T2100 (level-14 widening): the ranger's FOURTH
            // combat-style bonus feat. Unlike feats 1-3, this slot is NOT validated
            // against a restricted feat-name list: the PF1 Core Rulebook's own
            // Combat Style feat tables (Archery, Two-Weapon Combat) do not name any
            // options beyond the 10th-level tier (verified independently against
            // three sources dedicated to the combat-style feat lists specifically —
            // d20pfsrd's Ranger Combat Styles page, the Archives of Nethys
            // aonprd.com RangerCombatStyles page, and a Paizo rules-forum thread —
            // all three agree the printed list stops after 10th level; later
            // sourcebooks add 14th/18th-level options, out of SD-18's
            // Core-Rulebook-only scope). So this mirrors the Favored
            // Terrain/Quarry OPEN-ENDED recognition idiom instead: the raw chosen
            // feat string is recorded as a +0 identity record with no
            // restricted-list validation and no feat mechanics computed.
            if let Some(style) = style_name
                && level >= RANGER_COMBAT_STYLE_BONUS_FEAT_4_LEVEL
                && let Some(feat_selection_4) =
                    choice_selection(input, RANGER_COMBAT_STYLE_BONUS_FEAT_4_CHOICE_ID)
            {
                explanations.push(ComputationExplanation {
                    id: "class_chassis.ranger.combat_style_bonus_feat_4_choice".to_owned(),
                    value: 0,
                    detail: format!(
                        "Ranger {style} FOURTH combat style bonus feat at ranger level {level} \
                         ({RANGER_COMBAT_STYLE_BONUS_FEAT_4_CHOICE_ID} -> {feat_selection_4}): \
                         the ranger's combat style grants bonus feats at 2nd, 6th, 10th, 14th, \
                         and 18th level (verified independently against d20pfsrd and the \
                         Archives of Nethys aonprd.com mirror), but unlike the 2nd/6th/10th-level \
                         grants, the PF1 Core Rulebook does not tabulate any named {style} feat \
                         options at the 14th-level tier (verified independently against three \
                         sources dedicated to the combat-style feat lists themselves; later \
                         sourcebooks such as the Advanced Player's Guide are the ones that add \
                         named 14th-level options, outside SD-18's Core-Rulebook-only scope). \
                         This is therefore an OPEN-ENDED recognition record of the choice slot \
                         only (mirroring the Favored Terrain/Quarry idiom, not the \
                         closed-restricted-list idiom used for feats 1-3): no restricted-list \
                         validation is attempted at this tier, and no mechanical value is \
                         fabricated (+0)"
                    ),
                });
            }

            // SD18 cycle-2026-07-16T0244 (level-18 widening): the ranger's FIFTH
            // combat-style bonus feat. Mirrors the fourth bonus feat's own
            // reasoning exactly: the PF1 Core Rulebook's own Combat Style feat
            // tables (Archery, Two-Weapon Combat) do not name any options beyond
            // the 10th-level tier (verified independently against three sources
            // dedicated to the combat-style feat lists specifically — d20pfsrd's
            // Ranger Combat Styles page, the Archives of Nethys aonprd.com
            // RangerCombatStyles page, and a Paizo rules-forum thread — all three
            // agree the printed list stops after 10th level; later sourcebooks
            // add 18th-level options, out of SD-18's Core-Rulebook-only scope).
            // So this mirrors the fourth bonus feat's own OPEN-ENDED recognition
            // idiom: the raw chosen feat string is recorded as a +0 identity
            // record with no restricted-list validation and no feat mechanics
            // computed.
            if let Some(style) = style_name
                && level >= RANGER_COMBAT_STYLE_BONUS_FEAT_5_LEVEL
                && let Some(feat_selection_5) =
                    choice_selection(input, RANGER_COMBAT_STYLE_BONUS_FEAT_5_CHOICE_ID)
            {
                explanations.push(ComputationExplanation {
                    id: "class_chassis.ranger.combat_style_bonus_feat_5_choice".to_owned(),
                    value: 0,
                    detail: format!(
                        "Ranger {style} FIFTH combat style bonus feat at ranger level {level} \
                         ({RANGER_COMBAT_STYLE_BONUS_FEAT_5_CHOICE_ID} -> {feat_selection_5}): \
                         the ranger's combat style grants bonus feats at 2nd, 6th, 10th, 14th, \
                         and 18th level (verified independently against d20pfsrd and the \
                         Archives of Nethys aonprd.com mirror), but like the 14th-level grant, \
                         the PF1 Core Rulebook does not tabulate any named {style} feat options \
                         at the 18th-level tier (verified independently against three sources \
                         dedicated to the combat-style feat lists themselves; later sourcebooks \
                         such as the Advanced Player's Guide are the ones that add named \
                         18th-level options, outside SD-18's Core-Rulebook-only scope). This is \
                         therefore an OPEN-ENDED recognition record of the choice slot only \
                         (mirroring the Favored Terrain/Quarry idiom, not the \
                         closed-restricted-list idiom used for feats 1-3): no restricted-list \
                         validation is attempted at this tier, and no mechanical value is \
                         fabricated (+0)"
                    ),
                });
            }
        }
    }

    // The third named F6 pillar, Track, is grounded for real: a bounded, flat
    // numeric Survival bonus with no execution engine behind it.
    let track_bonus = (level_value / 2).max(1);
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.track".to_owned(),
        value: track_bonus,
        detail: format!(
            "Ranger Track class feature: grants a bonus on Survival checks made to follow tracks \
             equal to max(ranger level / 2, 1) (PF1 Core Rulebook Track: +1/2 ranger level, minimum \
             +1). At Ranger level {level} this bonus is \
             max({level} / 2, 1) = {track_bonus}. This grounds only the flat numeric \
             Track bonus on Survival checks to follow tracks; it is not a tracking-check execution \
             engine and computes no full Survival check, no DC resolution, and no tracking narrative"
        ),
    });

    // The Favored Enemy FLAT surface is grounded for real (SD13-E5). PF1 Core
    // Rulebook: the ranger selects one favored-enemy type and gains a +2 bonus on
    // Bluff, Knowledge, Perception, Sense Motive, and Survival checks against it,
    // plus a +2 bonus on weapon attack and damage rolls against it (PF1 includes
    // attack rolls, unlike D&D 3.5). PF1 Core Rulebook only increases this bonus at
    // 4th ranger level and beyond, so it stays the flat +2 at both level 1 and level
    // 2 via the same formula, not a new record. Only the flat magnitudes are
    // grounded: no target-type matching and no conditional-application engine
    // decides whether any specific check or attack is actually made against the
    // favored enemy.
    if let Some(favored_enemy) = choice_selection(input, "choice:ranger_favored_enemy") {
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_choice".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Favored Enemy selection (choice:ranger_favored_enemy -> {favored_enemy}): \
                 the level-{level} favored-enemy type chosen for this character is \
                 {favored_enemy}. This is a bounded recognition record of the chosen enemy type \
                 only; the flat bonus magnitudes are grounded separately, and no target-type \
                 matching or conditional-application engine is implemented, so it carries no \
                 fabricated mechanical value (+0)"
            ),
        });
    }

    // SD13-E5 ranger level 5: recognize the bonus-increase TARGET choice, only
    // meaningful once the ranger has reached the Favored Enemy rule's 5th-level
    // interval. PF1 Core Rulebook: "the bonus against any one favored enemy
    // (including the one just selected, if so desired) increases by 2" -- a
    // genuine, free player choice of which ONE favored enemy is boosted, not an
    // automatic bump to the first one. Absent an explicit target selection in
    // chosen input, nothing is fabricated: both favored enemies stay at the flat
    // base magnitude.
    let favored_enemy_bonus_increase_target = if level >= RANGER_FAVORED_ENEMY_SECOND_INTERVAL_LEVEL
    {
        choice_selection(input, RANGER_FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID)
    } else {
        None
    };

    if let Some(target) = favored_enemy_bonus_increase_target {
        let target_name = if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION {
            Some("the first favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION {
            Some("the second favored enemy")
        } else {
            None
        };
        let detail = if let Some(name) = target_name {
            format!(
                "Ranger Favored Enemy bonus-increase target selection at ranger level {level} \
                 ({RANGER_FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID} -> {target}): names {name} as \
                 the one favored enemy whose bonus increases by +2 at this 5th-level interval, \
                 per the PF1 Core Rulebook rule that the bonus against any ONE favored enemy -- \
                 including a newly selected one, if so desired -- increases by 2 at each such \
                 interval (5th, 10th, 15th, and 20th ranger level). This is a recognition record \
                 of the choice slot only (+0); the increased magnitude itself is grounded \
                 separately on whichever favored enemy was actually named"
            )
        } else {
            format!(
                "Ranger Favored Enemy bonus-increase target selection at ranger level {level} is \
                 present ({RANGER_FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID} -> {target}), but only \
                 the PF1 Core Rulebook restricted pair (the first favored enemy, the second \
                 favored enemy) is recognized on this bounded seam; no target identity is \
                 grounded and no mechanical value is fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_bonus_increase_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    let first_favored_enemy_targeted =
        favored_enemy_bonus_increase_target == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION);
    let second_favored_enemy_targeted = favored_enemy_bonus_increase_target
        == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION);

    // SD13-E5 ranger level 10: recognize the SECOND interval's own
    // bonus-increase TARGET choice, only meaningful once the ranger has
    // reached the Favored Enemy rule's 10th-level interval. Each interval
    // grants its own +2 increase to any ONE favored enemy — a genuine, free
    // player choice, so an increase targeting the same enemy at both
    // intervals STACKS (2 base + 2 + 2). Absent an explicit target
    // selection in chosen input, nothing is fabricated.
    let favored_enemy_second_bonus_increase_target =
        if level >= RANGER_FAVORED_ENEMY_THIRD_INTERVAL_LEVEL {
            choice_selection(input, RANGER_FAVORED_ENEMY_SECOND_BONUS_INCREASE_CHOICE_ID)
        } else {
            None
        };

    if let Some(target) = favored_enemy_second_bonus_increase_target {
        let target_name = if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION {
            Some("the first favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION {
            Some("the second favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_THIRD_SELECTION {
            Some("the third favored enemy")
        } else {
            None
        };
        let detail = if let Some(name) = target_name {
            format!(
                "Ranger Favored Enemy 10th-level-interval bonus-increase target selection at \
                 ranger level {level} \
                 ({RANGER_FAVORED_ENEMY_SECOND_BONUS_INCREASE_CHOICE_ID} -> {target}): names \
                 {name} as the one favored enemy whose bonus increases by +2 at this \
                 10th-level interval, per the PF1 Core Rulebook rule that the bonus against \
                 any ONE favored enemy -- including a newly selected one, if so desired -- \
                 increases by +2 at each such interval (5th, 10th, 15th, and 20th ranger \
                 level); an increase targeting the same enemy at both grounded intervals \
                 stacks. This is a recognition record of the choice slot only (+0); the \
                 increased magnitude itself is grounded separately on whichever favored enemy \
                 was actually named"
            )
        } else {
            format!(
                "Ranger Favored Enemy 10th-level-interval bonus-increase target selection at \
                 ranger level {level} is present \
                 ({RANGER_FAVORED_ENEMY_SECOND_BONUS_INCREASE_CHOICE_ID} -> {target}), but \
                 only the PF1 Core Rulebook restricted set (the first, second, or third \
                 favored enemy) is recognized on this bounded seam; no target identity is \
                 grounded and no mechanical value is fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_bonus_increase_2_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    let first_favored_enemy_targeted_at_second_interval =
        favored_enemy_second_bonus_increase_target
            == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION);
    let second_favored_enemy_targeted_at_second_interval =
        favored_enemy_second_bonus_increase_target
            == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION);
    let third_favored_enemy_targeted_at_second_interval =
        favored_enemy_second_bonus_increase_target
            == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_THIRD_SELECTION);

    // SD18 ranger level 15: recognize the THIRD interval's own
    // bonus-increase TARGET choice, only meaningful once the ranger has
    // reached the Favored Enemy rule's 15th-level interval. Each interval
    // grants its own +2 increase to any ONE favored enemy — a genuine, free
    // player choice, so an increase targeting the same enemy at all three
    // grounded intervals STACKS (2 base + 2 + 2 + 2). Absent an explicit
    // target selection in chosen input, nothing is fabricated.
    let favored_enemy_third_bonus_increase_target =
        if level >= RANGER_FAVORED_ENEMY_FOURTH_INTERVAL_LEVEL {
            choice_selection(input, RANGER_FAVORED_ENEMY_THIRD_BONUS_INCREASE_CHOICE_ID)
        } else {
            None
        };

    if let Some(target) = favored_enemy_third_bonus_increase_target {
        let target_name = if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION {
            Some("the first favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION {
            Some("the second favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_THIRD_SELECTION {
            Some("the third favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_FOURTH_SELECTION {
            Some("the fourth favored enemy")
        } else {
            None
        };
        let detail = if let Some(name) = target_name {
            format!(
                "Ranger Favored Enemy 15th-level-interval bonus-increase target selection at \
                 ranger level {level} \
                 ({RANGER_FAVORED_ENEMY_THIRD_BONUS_INCREASE_CHOICE_ID} -> {target}): names \
                 {name} as the one favored enemy whose bonus increases by +2 at this \
                 15th-level interval, per the PF1 Core Rulebook rule that the bonus against \
                 any ONE favored enemy -- including a newly selected one, if so desired -- \
                 increases by +2 at each such interval (5th, 10th, 15th, and 20th ranger \
                 level); an increase targeting the same enemy at multiple grounded intervals \
                 stacks. This is a recognition record of the choice slot only (+0); the \
                 increased magnitude itself is grounded separately on whichever favored enemy \
                 was actually named"
            )
        } else {
            format!(
                "Ranger Favored Enemy 15th-level-interval bonus-increase target selection at \
                 ranger level {level} is present \
                 ({RANGER_FAVORED_ENEMY_THIRD_BONUS_INCREASE_CHOICE_ID} -> {target}), but \
                 only the PF1 Core Rulebook restricted set (the first, second, third, or \
                 fourth favored enemy) is recognized on this bounded seam; no target identity \
                 is grounded and no mechanical value is fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_bonus_increase_3_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    let first_favored_enemy_targeted_at_third_interval = favored_enemy_third_bonus_increase_target
        == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION);
    let second_favored_enemy_targeted_at_third_interval =
        favored_enemy_third_bonus_increase_target
            == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION);
    let third_favored_enemy_targeted_at_third_interval = favored_enemy_third_bonus_increase_target
        == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_THIRD_SELECTION);
    let fourth_favored_enemy_targeted_at_third_interval =
        favored_enemy_third_bonus_increase_target
            == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_FOURTH_SELECTION);

    // SD18 ranger level 20: recognize the FOURTH (and final) interval's own
    // bonus-increase TARGET choice, only meaningful once the ranger has
    // reached the Favored Enemy rule's FINAL 20th-level interval. Each
    // interval grants its own +2 increase to any ONE favored enemy — a
    // genuine, free player choice, so an increase targeting the same enemy
    // at all four grounded intervals STACKS (2 base + 2 + 2 + 2 + 2). Absent
    // an explicit target selection in chosen input, nothing is fabricated.
    let favored_enemy_fourth_bonus_increase_target =
        if level >= RANGER_FAVORED_ENEMY_FIFTH_INTERVAL_LEVEL {
            choice_selection(input, RANGER_FAVORED_ENEMY_FOURTH_BONUS_INCREASE_CHOICE_ID)
        } else {
            None
        };

    if let Some(target) = favored_enemy_fourth_bonus_increase_target {
        let target_name = if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION {
            Some("the first favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION {
            Some("the second favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_THIRD_SELECTION {
            Some("the third favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_FOURTH_SELECTION {
            Some("the fourth favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIFTH_SELECTION {
            Some("the fifth favored enemy")
        } else {
            None
        };
        let detail = if let Some(name) = target_name {
            format!(
                "Ranger Favored Enemy 20th-level-interval bonus-increase target selection at \
                 ranger level {level} \
                 ({RANGER_FAVORED_ENEMY_FOURTH_BONUS_INCREASE_CHOICE_ID} -> {target}): names \
                 {name} as the one favored enemy whose bonus increases by +2 at this FINAL \
                 20th-level interval, per the PF1 Core Rulebook rule that the bonus against \
                 any ONE favored enemy -- including a newly selected one, if so desired -- \
                 increases by +2 at each such interval (5th, 10th, 15th, and 20th ranger \
                 level, the last one within PF1's 1-20 level cap); an increase targeting the \
                 same enemy at multiple grounded intervals stacks. This is a recognition \
                 record of the choice slot only (+0); the increased magnitude itself is \
                 grounded separately on whichever favored enemy was actually named"
            )
        } else {
            format!(
                "Ranger Favored Enemy 20th-level-interval bonus-increase target selection at \
                 ranger level {level} is present \
                 ({RANGER_FAVORED_ENEMY_FOURTH_BONUS_INCREASE_CHOICE_ID} -> {target}), but \
                 only the PF1 Core Rulebook restricted set (the first, second, third, fourth, \
                 or fifth favored enemy) is recognized on this bounded seam; no target identity \
                 is grounded and no mechanical value is fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_bonus_increase_4_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    let first_favored_enemy_targeted_at_fourth_interval =
        favored_enemy_fourth_bonus_increase_target
            == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION);
    let second_favored_enemy_targeted_at_fourth_interval =
        favored_enemy_fourth_bonus_increase_target
            == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION);
    let third_favored_enemy_targeted_at_fourth_interval =
        favored_enemy_fourth_bonus_increase_target
            == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_THIRD_SELECTION);
    let fourth_favored_enemy_targeted_at_fourth_interval =
        favored_enemy_fourth_bonus_increase_target
            == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_FOURTH_SELECTION);

    let favored_enemy_bonus: i16 = 2
        + if first_favored_enemy_targeted { 2 } else { 0 }
        + if first_favored_enemy_targeted_at_second_interval { 2 } else { 0 }
        + if first_favored_enemy_targeted_at_third_interval { 2 } else { 0 }
        + if first_favored_enemy_targeted_at_fourth_interval { 2 } else { 0 };
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.favored_enemy_skill_bonus".to_owned(),
        value: favored_enemy_bonus,
        detail: format!(
            "Ranger Favored Enemy skill bonus (PF1 Core Rulebook, level \
             {level}): +{favored_enemy_bonus} on Bluff, Knowledge, Perception, \
             Sense Motive, and Survival checks against the chosen favored enemy. This grounds only \
             the flat +{favored_enemy_bonus} magnitude; no target-type matching and no \
             conditional-application engine is implemented, so whether any specific skill check is \
             actually made against the favored enemy is never resolved and no skill total is \
             modified by this record"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.favored_enemy_attack_damage_bonus".to_owned(),
        value: favored_enemy_bonus,
        detail: format!(
            "Ranger Favored Enemy weapon attack/damage bonus (PF1 Core Rulebook, level \
             {level}): +{favored_enemy_bonus} on weapon attack rolls AND weapon \
             damage rolls against the chosen favored enemy — PF1 includes attack rolls, unlike the \
             damage-only D&D 3.5 favored enemy. This grounds only the flat \
             +{favored_enemy_bonus} magnitude; no target-type matching and no \
             conditional-application engine is implemented, so whether any specific attack is \
             actually made against the favored enemy is never resolved and no combat baseline is \
             modified by this record"
        ),
    });

    // `AT-34-E3-002` (bucket C, cycle 9): an exact-slug identity record for
    // this feature -- mirrors `"class_feature.ranger.favored_terrain"`'s own
    // already-shipped idiom (pushed a few lines below this function's own
    // Favored Terrain block) for this record's sibling. The two records
    // above (`favored_enemy_skill_bonus` / `favored_enemy_attack_damage_bonus`)
    // both carry a magnitude-descriptor suffix `v06_work_inventory.rs`'s own
    // `CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES` list does not include ("skill_
    // bonus"/"attack_damage_bonus" do not end in a listed word), and the
    // choice-recognition record above ends in "_choice" (also unlisted), so
    // NONE of the three is reachable from the corpus's own bare `"Ranger ~
    // Favored Enemy"` header record's exact `feature_slug`
    // ("favored_enemy") through that file's generic `class_feature_exact_
    // suffix_grounded` check -- a real, disclosed gap, not a broadened
    // matcher. This record carries the SAME `favored_enemy_bonus` value
    // already verified above; it computes no new magnitude.
    explanations.push(ComputationExplanation {
        id: "class_feature.ranger.favored_enemy".to_owned(),
        value: favored_enemy_bonus,
        detail: format!(
            "Ranger Favored Enemy at ranger level {level} (PF1 Core Rulebook, 1st-level ranger \
             class feature): the same flat +{favored_enemy_bonus} magnitude already grounded \
             separately above for the skill-check bonus (class_chassis.ranger.\
             favored_enemy_skill_bonus) and the weapon attack/damage bonus (class_chassis.\
             ranger.favored_enemy_attack_damage_bonus). This record exists to give the exact \
             corpus key `\"Ranger ~ Favored Enemy\"` its own exact-slug explanation id, \
             mirroring Favored Terrain's own `class_feature.ranger.favored_terrain` idiom; no \
             new magnitude is computed here"
        ),
    });

    // SD13-E5 ranger level 5: recognize the SECOND favored-enemy selection, the
    // rule's other 5th-level interval grant. Mirrors the first favored enemy's
    // own choice-recognition idiom exactly (open-ended, raw string
    // interpolation, no restricted-list validation) and its own flat magnitude
    // formula (base +2, or +4 if this interval's bonus-increase target names the
    // second favored enemy).
    if level >= RANGER_FAVORED_ENEMY_SECOND_INTERVAL_LEVEL
        && let Some(second_favored_enemy) =
            choice_selection(input, RANGER_FAVORED_ENEMY_SECOND_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_2_choice".to_owned(),
            value: 0,
            detail: format!(
                "Ranger 2nd Favored Enemy selection \
                 ({RANGER_FAVORED_ENEMY_SECOND_CHOICE_ID} -> {second_favored_enemy}): at \
                 ranger level {level}, PF1 Core Rulebook Favored Enemy grants \"an additional \
                 favored enemy\" at the 5th-level interval. The level-{level} SECOND \
                 favored-enemy type chosen for this character is {second_favored_enemy}. This \
                 is a bounded recognition record of the chosen enemy type only; the flat bonus \
                 magnitude is grounded separately, and no target-type matching or \
                 conditional-application engine is implemented, so it carries no fabricated \
                 mechanical value (+0)"
            ),
        });

        let second_favored_enemy_bonus: i16 = 2
            + if second_favored_enemy_targeted { 2 } else { 0 }
            + if second_favored_enemy_targeted_at_second_interval { 2 } else { 0 }
            + if second_favored_enemy_targeted_at_third_interval { 2 } else { 0 }
            + if second_favored_enemy_targeted_at_fourth_interval { 2 } else { 0 };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_2_skill_bonus".to_owned(),
            value: second_favored_enemy_bonus,
            detail: format!(
                "Ranger 2nd Favored Enemy skill bonus (PF1 Core Rulebook, level {level}): \
                 +{second_favored_enemy_bonus} on Bluff, Knowledge, Perception, Sense Motive, \
                 and Survival checks against the second favored enemy. This grounds only the \
                 flat +{second_favored_enemy_bonus} magnitude; no target-type matching and no \
                 conditional-application engine is implemented, so whether any specific skill \
                 check is actually made against this favored enemy is never resolved and no \
                 skill total is modified by this record"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_2_attack_damage_bonus".to_owned(),
            value: second_favored_enemy_bonus,
            detail: format!(
                "Ranger 2nd Favored Enemy weapon attack/damage bonus (PF1 Core Rulebook, \
                 level {level}): +{second_favored_enemy_bonus} on weapon attack rolls AND \
                 weapon damage rolls against the second favored enemy. This grounds only the \
                 flat +{second_favored_enemy_bonus} magnitude; no target-type matching and no \
                 conditional-application engine is implemented, so whether any specific attack \
                 is actually made against this favored enemy is never resolved and no combat \
                 baseline is modified by this record"
            ),
        });
    }

    // SD13-E5 ranger level 10: recognize the THIRD favored-enemy selection,
    // the rule's 10th-level interval grant. Mirrors the second favored
    // enemy's own choice-recognition idiom exactly (open-ended, raw string
    // interpolation, no restricted-list validation) and its own flat
    // magnitude formula (base +2, or +4 if the 10th-level interval's
    // bonus-increase target names the third favored enemy).
    if level >= RANGER_FAVORED_ENEMY_THIRD_INTERVAL_LEVEL
        && let Some(third_favored_enemy) =
            choice_selection(input, RANGER_FAVORED_ENEMY_THIRD_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_3_choice".to_owned(),
            value: 0,
            detail: format!(
                "Ranger 3rd Favored Enemy selection \
                 ({RANGER_FAVORED_ENEMY_THIRD_CHOICE_ID} -> {third_favored_enemy}): at ranger \
                 level {level}, PF1 Core Rulebook Favored Enemy grants \"an additional \
                 favored enemy\" at the 10th-level interval. The level-{level} THIRD \
                 favored-enemy type chosen for this character is {third_favored_enemy}. This \
                 is a bounded recognition record of the chosen enemy type only; the flat \
                 bonus magnitude is grounded separately, and no target-type matching or \
                 conditional-application engine is implemented, so it carries no fabricated \
                 mechanical value (+0)"
            ),
        });

        let third_favored_enemy_bonus: i16 = 2
            + if third_favored_enemy_targeted_at_second_interval { 2 } else { 0 }
            + if third_favored_enemy_targeted_at_third_interval { 2 } else { 0 }
            + if third_favored_enemy_targeted_at_fourth_interval { 2 } else { 0 };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_3_skill_bonus".to_owned(),
            value: third_favored_enemy_bonus,
            detail: format!(
                "Ranger 3rd Favored Enemy skill bonus (PF1 Core Rulebook, level {level}): \
                 +{third_favored_enemy_bonus} on Bluff, Knowledge, Perception, Sense Motive, \
                 and Survival checks against the third favored enemy. This grounds only the \
                 flat +{third_favored_enemy_bonus} magnitude; no target-type matching and no \
                 conditional-application engine is implemented, so whether any specific skill \
                 check is actually made against this favored enemy is never resolved and no \
                 skill total is modified by this record"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_3_attack_damage_bonus".to_owned(),
            value: third_favored_enemy_bonus,
            detail: format!(
                "Ranger 3rd Favored Enemy weapon attack/damage bonus (PF1 Core Rulebook, \
                 level {level}): +{third_favored_enemy_bonus} on weapon attack rolls AND \
                 weapon damage rolls against the third favored enemy. This grounds only the \
                 flat +{third_favored_enemy_bonus} magnitude; no target-type matching and no \
                 conditional-application engine is implemented, so whether any specific attack \
                 is actually made against this favored enemy is never resolved and no combat \
                 baseline is modified by this record"
            ),
        });
    }

    // SD18 ranger level 15: recognize the FOURTH favored-enemy selection,
    // the rule's 15th-level interval grant. Mirrors the third favored
    // enemy's own choice-recognition idiom exactly (open-ended, raw string
    // interpolation, no restricted-list validation) and its own flat
    // magnitude formula (base +2, or +4 if the 15th-level interval's
    // bonus-increase target names the fourth favored enemy -- the ONLY
    // interval that can target it, since it did not exist before this
    // level).
    if level >= RANGER_FAVORED_ENEMY_FOURTH_INTERVAL_LEVEL
        && let Some(fourth_favored_enemy) =
            choice_selection(input, RANGER_FAVORED_ENEMY_FOURTH_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_4_choice".to_owned(),
            value: 0,
            detail: format!(
                "Ranger 4th Favored Enemy selection \
                 ({RANGER_FAVORED_ENEMY_FOURTH_CHOICE_ID} -> {fourth_favored_enemy}): at \
                 ranger level {level}, PF1 Core Rulebook Favored Enemy grants \"an additional \
                 favored enemy\" at the 15th-level interval. The level-{level} FOURTH \
                 favored-enemy type chosen for this character is {fourth_favored_enemy}. This \
                 is a bounded recognition record of the chosen enemy type only; the flat \
                 bonus magnitude is grounded separately, and no target-type matching or \
                 conditional-application engine is implemented, so it carries no fabricated \
                 mechanical value (+0)"
            ),
        });

        let fourth_favored_enemy_bonus: i16 = 2
            + if fourth_favored_enemy_targeted_at_third_interval { 2 } else { 0 }
            + if fourth_favored_enemy_targeted_at_fourth_interval { 2 } else { 0 };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_4_skill_bonus".to_owned(),
            value: fourth_favored_enemy_bonus,
            detail: format!(
                "Ranger 4th Favored Enemy skill bonus (PF1 Core Rulebook, level {level}): \
                 +{fourth_favored_enemy_bonus} on Bluff, Knowledge, Perception, Sense Motive, \
                 and Survival checks against the fourth favored enemy. This grounds only the \
                 flat +{fourth_favored_enemy_bonus} magnitude; no target-type matching and no \
                 conditional-application engine is implemented, so whether any specific skill \
                 check is actually made against this favored enemy is never resolved and no \
                 skill total is modified by this record"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_4_attack_damage_bonus".to_owned(),
            value: fourth_favored_enemy_bonus,
            detail: format!(
                "Ranger 4th Favored Enemy weapon attack/damage bonus (PF1 Core Rulebook, \
                 level {level}): +{fourth_favored_enemy_bonus} on weapon attack rolls AND \
                 weapon damage rolls against the fourth favored enemy. This grounds only the \
                 flat +{fourth_favored_enemy_bonus} magnitude; no target-type matching and no \
                 conditional-application engine is implemented, so whether any specific attack \
                 is actually made against this favored enemy is never resolved and no combat \
                 baseline is modified by this record"
            ),
        });
    }

    // SD18 ranger level 20: recognize the FIFTH (and final) favored-enemy
    // selection, the rule's 20th-level interval grant. Mirrors the fourth
    // favored enemy's own choice-recognition idiom exactly (open-ended, raw
    // string interpolation, no restricted-list validation) and its own flat
    // magnitude formula (base +2, or +4 if the 20th-level interval's
    // bonus-increase target names the fifth favored enemy -- the ONLY
    // interval that can target it, since it did not exist before this
    // level).
    if level >= RANGER_FAVORED_ENEMY_FIFTH_INTERVAL_LEVEL
        && let Some(fifth_favored_enemy) =
            choice_selection(input, RANGER_FAVORED_ENEMY_FIFTH_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_5_choice".to_owned(),
            value: 0,
            detail: format!(
                "Ranger 5th Favored Enemy selection \
                 ({RANGER_FAVORED_ENEMY_FIFTH_CHOICE_ID} -> {fifth_favored_enemy}): at \
                 ranger level {level}, PF1 Core Rulebook Favored Enemy grants \"an additional \
                 favored enemy\" at the FINAL 20th-level interval. The level-{level} FIFTH \
                 favored-enemy type chosen for this character is {fifth_favored_enemy}. This \
                 is a bounded recognition record of the chosen enemy type only; the flat \
                 bonus magnitude is grounded separately, and no target-type matching or \
                 conditional-application engine is implemented, so it carries no fabricated \
                 mechanical value (+0)"
            ),
        });

        let fifth_favored_enemy_targeted_at_fourth_interval =
            favored_enemy_fourth_bonus_increase_target
                == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIFTH_SELECTION);
        let fifth_favored_enemy_bonus: i16 =
            if fifth_favored_enemy_targeted_at_fourth_interval { 4 } else { 2 };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_5_skill_bonus".to_owned(),
            value: fifth_favored_enemy_bonus,
            detail: format!(
                "Ranger 5th Favored Enemy skill bonus (PF1 Core Rulebook, level {level}): \
                 +{fifth_favored_enemy_bonus} on Bluff, Knowledge, Perception, Sense Motive, \
                 and Survival checks against the fifth favored enemy. This grounds only the \
                 flat +{fifth_favored_enemy_bonus} magnitude; no target-type matching and no \
                 conditional-application engine is implemented, so whether any specific skill \
                 check is actually made against this favored enemy is never resolved and no \
                 skill total is modified by this record"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_5_attack_damage_bonus".to_owned(),
            value: fifth_favored_enemy_bonus,
            detail: format!(
                "Ranger 5th Favored Enemy weapon attack/damage bonus (PF1 Core Rulebook, \
                 level {level}): +{fifth_favored_enemy_bonus} on weapon attack rolls AND \
                 weapon damage rolls against the fifth favored enemy. This grounds only the \
                 flat +{fifth_favored_enemy_bonus} magnitude; no target-type matching and no \
                 conditional-application engine is implemented, so whether any specific attack \
                 is actually made against this favored enemy is never resolved and no combat \
                 baseline is modified by this record"
            ),
        });
    }

    // Master Hunter: below the level-20 gate, this stays a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it (SD18 level-20
    // widening, the class capstone), it transitions to a bounded GRANT-only
    // identity record (mirroring the Paladin Holy Champion idiom exactly).
    // No action-economy engine, no attack-resolution engine, and no
    // saving-throw-resolution engine exists anywhere in this codebase to
    // apply this to.
    if level < RANGER_MASTER_HUNTER_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.master_hunter".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Master Hunter at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Master Hunter is the 20th-level ranger capstone."
            ),
        });
    } else if let Some(dc) = resolve_class_feature_bonus_var(
        "Ranger ~ Master Hunter",
        "RangerLVL",
        "MasterHunterDC",
        level,
        ability_modifiers,
    ) {
        // SD-32 Epic 1 (compute-library wiring, F3): the corpus's own
        // `BONUS:VAR|MasterHunterDC|10+(MasterHunterLVL/2)+WIS`
        // (`cr_abilities_class.lst:1427`), resolved through
        // `resolve_class_feature_bonus_var` -- no longer a fabricated 0.
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.master_hunter".to_owned(),
            value: dc,
            detail: format!(
                "Ranger Master Hunter granted at ranger level {level} (PF1 Core Rulebook, \
                 20th-level ranger capstone): \"A ranger of 20th level becomes a master \
                 hunter. He can always move at full speed while using Survival to follow \
                 tracks without penalty. He can, as a standard action, make a single attack \
                 against a favored enemy at his full attack bonus. If the attack hits, the \
                 target takes damage normally and must make a Fortitude save or die. The DC \
                 of this save is {dc} (10 + 1/2 the ranger's level + the ranger's Wisdom \
                 modifier). A ranger can choose instead to deal an amount of nonlethal damage \
                 equal to the creature's current hit points... A ranger can use this ability \
                 once per day against each favored enemy type he possesses, but not against \
                 the same creature more than once in a 24-hour period.\" The save DC is a \
                 genuinely computed magnitude (corpus formula, resolved against this \
                 character's real Wisdom modifier); no action-economy engine and no \
                 attack-resolution engine exists anywhere in this codebase to apply this to, \
                 so this still grounds no actual attack or damage resolution -- only the save \
                 DC itself is computed"
            ),
        });
    } else {
        // The interpreter's own formula chain did not resolve -- refuse
        // rather than guess, falling back to the bounded grant-only
        // identity record the pre-wiring behaviour always used.
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.master_hunter".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Master Hunter granted at ranger level {level} (PF1 Core Rulebook, \
                 20th-level ranger capstone). This is a bounded grant-only identity record \
                 (value 0, non-fabricated): the corpus formula for the save DC did not \
                 resolve, so no save-DC computation is claimed."
            ),
        });
    }

    // Grounded (SD13-E5): Endurance, a 3rd-level Ranger class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Endurance, favored terrain" as the Ranger
    // 3rd-level special feature entry). Endurance is a bonus feat granted
    // automatically, with no player choice involved (PF1 Core Rulebook: "A
    // ranger gains Endurance as a bonus feat at 3rd level"). Below the level-3
    // gate this is a correct level-gate absence (value 0); at or above it, it is
    // a bounded grant-only identity record (value 0, non-fabricated) — mirroring
    // the Wizard Scribe Scroll / Barbarian Uncanny Dodge idiom: no feat-effect
    // execution engine exists anywhere in this codebase to apply Endurance's own
    // mechanical benefits.
    if level < RANGER_ENDURANCE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.endurance".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Endurance at ranger level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant feat is named but not computed. \
                 Endurance is a 3rd-level ranger class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.endurance".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Endurance granted at ranger level {level} (PF1 Core Rulebook, 3rd-level \
                 ranger class feature): the ranger gains Endurance as a bonus feat automatically, \
                 with no player choice involved. This is a bounded grant-only identity record \
                 (value 0, non-fabricated): Endurance's own mechanical effects are not computed, \
                 since no feat-effect execution engine exists anywhere in this codebase"
            ),
        });
    }

    // Grounded (SD13-E5): Favored Terrain, the class table's other 3rd-level
    // "Special" column entry alongside Endurance, verified independently against
    // two primary PF1 sources (d20pfsrd and legacy.aonprd.com both list
    // "Endurance, favored terrain" as the Ranger 3rd-level special feature entry,
    // and both state the exact bonus text: "+2 bonus on Initiative checks and
    // Knowledge (geography), Perception, Stealth, and Survival skill checks" made
    // when the ranger is in the chosen terrain, selected from Table: Ranger
    // Favored Terrains' fixed eleven-entry list). Below the level-3 gate this is
    // a correct level-gate absence (value 0); at or above it: the chosen terrain
    // (when present in chosen input) is recognized as a bounded `+0` identity
    // record naming whichever raw terrain string was actually selected —
    // mirroring the Favored Enemy choice-recognition idiom exactly, with no
    // restricted-list validation — and the rule's own flat `+2` magnitude is
    // grounded as a standalone, non-applied record: no terrain-detection engine
    // decides whether the character is actually in the chosen terrain, so no
    // Initiative total or skill-check total is modified by this record. The
    // level-8th/13th/18th additional-terrain and bonus-increase progression
    // stays out of scope for this bounded slice.
    if level < RANGER_FAVORED_TERRAIN_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.favored_terrain".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Favored Terrain at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant terrain choice and \
                 magnitude are named but not computed. Favored Terrain is a 3rd-level ranger \
                 class feature."
            ),
        });
    } else {
        if let Some(favored_terrain) = choice_selection(input, RANGER_FAVORED_TERRAIN_CHOICE_ID) {
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.favored_terrain_choice".to_owned(),
                value: 0,
                detail: format!(
                    "Ranger Favored Terrain selection \
                     ({RANGER_FAVORED_TERRAIN_CHOICE_ID} -> {favored_terrain}): the \
                     level-{level} favored terrain type chosen for this character is \
                     {favored_terrain}. This is a bounded recognition record of the chosen \
                     terrain type only; the flat bonus magnitude is grounded separately, and no \
                     terrain-detection or conditional-application engine is implemented, so it \
                     carries no fabricated mechanical value (+0)"
                ),
            });
        }

        // SD13-E5 ranger level 8: recognize the bonus-increase TARGET choice,
        // only meaningful once the ranger has reached the Favored Terrain
        // rule's 8th-level interval. Mirrors the Favored Enemy 5th-level
        // bonus-increase-target idiom exactly: a genuine, free player choice
        // of which ONE favored terrain is boosted by +2, not an automatic
        // bump to the first one. Absent an explicit target selection in
        // chosen input, nothing is fabricated: both terrains stay at the
        // flat base magnitude.
        let favored_terrain_bonus_increase_target =
            if level >= RANGER_FAVORED_TERRAIN_SECOND_INTERVAL_LEVEL {
                choice_selection(input, RANGER_FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID)
            } else {
                None
            };

        if let Some(target) = favored_terrain_bonus_increase_target {
            let target_name = if target == RANGER_FAVORED_TERRAIN_BONUS_INCREASE_FIRST_SELECTION {
                Some("the first favored terrain")
            } else if target == RANGER_FAVORED_TERRAIN_BONUS_INCREASE_SECOND_SELECTION {
                Some("the second favored terrain")
            } else {
                None
            };
            let detail = if let Some(name) = target_name {
                format!(
                    "Ranger Favored Terrain bonus-increase target selection at ranger level \
                     {level} ({RANGER_FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID} -> {target}): \
                     names {name} as the one favored terrain whose skill and initiative bonus \
                     increases by +2 at this 8th-level interval, per the PF1 Core Rulebook rule \
                     that the bonus in any ONE favored terrain -- including a newly selected \
                     one, if so desired -- increases by +2 at each such interval (8th, 13th, \
                     and 18th ranger level). This is a recognition record of the choice slot \
                     only (+0); the increased magnitude itself is grounded separately on \
                     whichever favored terrain was actually named"
                )
            } else {
                format!(
                    "Ranger Favored Terrain bonus-increase target selection at ranger level \
                     {level} is present ({RANGER_FAVORED_TERRAIN_BONUS_INCREASE_CHOICE_ID} -> \
                     {target}), but only the PF1 Core Rulebook restricted pair (the first \
                     favored terrain, the second favored terrain) is recognized on this bounded \
                     seam; no target identity is grounded and no mechanical value is fabricated \
                     (+0)"
                )
            };
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.favored_terrain_bonus_increase_choice".to_owned(),
                value: 0,
                detail,
            });
        }

        let first_favored_terrain_targeted = favored_terrain_bonus_increase_target
            == Some(RANGER_FAVORED_TERRAIN_BONUS_INCREASE_FIRST_SELECTION);
        let second_favored_terrain_targeted = favored_terrain_bonus_increase_target
            == Some(RANGER_FAVORED_TERRAIN_BONUS_INCREASE_SECOND_SELECTION);

        // SD18 ranger level 13: recognize the 13th-level interval's own
        // bonus-increase TARGET choice, only meaningful once the ranger has
        // reached the Favored Terrain rule's 13th-level interval. Each
        // interval grants its own +2 increase to any ONE favored terrain --
        // a genuine, free player choice, so an increase targeting the same
        // terrain at both grounded intervals STACKS (2 base + 2 + 2). Absent
        // an explicit target selection in chosen input, nothing is
        // fabricated. Mirrors the already-grounded Favored Enemy 10th-level
        // interval's own bonus-increase-target idiom exactly.
        let favored_terrain_second_bonus_increase_target =
            if level >= RANGER_FAVORED_TERRAIN_THIRD_INTERVAL_LEVEL {
                choice_selection(input, RANGER_FAVORED_TERRAIN_SECOND_BONUS_INCREASE_CHOICE_ID)
            } else {
                None
            };

        if let Some(target) = favored_terrain_second_bonus_increase_target {
            let target_name = if target == RANGER_FAVORED_TERRAIN_BONUS_INCREASE_FIRST_SELECTION {
                Some("the first favored terrain")
            } else if target == RANGER_FAVORED_TERRAIN_BONUS_INCREASE_SECOND_SELECTION {
                Some("the second favored terrain")
            } else if target == RANGER_FAVORED_TERRAIN_BONUS_INCREASE_THIRD_SELECTION {
                Some("the third favored terrain")
            } else {
                None
            };
            let detail = if let Some(name) = target_name {
                format!(
                    "Ranger Favored Terrain 13th-level-interval bonus-increase target selection \
                     at ranger level {level} \
                     ({RANGER_FAVORED_TERRAIN_SECOND_BONUS_INCREASE_CHOICE_ID} -> {target}): \
                     names {name} as the one favored terrain whose skill and initiative bonus \
                     increases by +2 at this 13th-level interval, per the PF1 Core Rulebook rule \
                     that the bonus in any ONE favored terrain -- including a newly selected \
                     one, if so desired -- increases by +2 at each such interval (8th, 13th, and \
                     18th ranger level); an increase targeting the same terrain at both grounded \
                     intervals stacks. This is a recognition record of the choice slot only \
                     (+0); the increased magnitude itself is grounded separately on whichever \
                     favored terrain was actually named"
                )
            } else {
                format!(
                    "Ranger Favored Terrain 13th-level-interval bonus-increase target selection \
                     at ranger level {level} is present \
                     ({RANGER_FAVORED_TERRAIN_SECOND_BONUS_INCREASE_CHOICE_ID} -> {target}), but \
                     only the PF1 Core Rulebook restricted set (the first, second, or third \
                     favored terrain) is recognized on this bounded seam; no target identity is \
                     grounded and no mechanical value is fabricated (+0)"
                )
            };
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.favored_terrain_bonus_increase_2_choice".to_owned(),
                value: 0,
                detail,
            });
        }

        let first_favored_terrain_targeted_at_second_interval =
            favored_terrain_second_bonus_increase_target
                == Some(RANGER_FAVORED_TERRAIN_BONUS_INCREASE_FIRST_SELECTION);
        let second_favored_terrain_targeted_at_second_interval =
            favored_terrain_second_bonus_increase_target
                == Some(RANGER_FAVORED_TERRAIN_BONUS_INCREASE_SECOND_SELECTION);
        let third_favored_terrain_targeted_at_second_interval =
            favored_terrain_second_bonus_increase_target
                == Some(RANGER_FAVORED_TERRAIN_BONUS_INCREASE_THIRD_SELECTION);

        // SD18 ranger level 18: recognize the 18th-level interval's own
        // bonus-increase TARGET choice, only meaningful once the ranger has
        // reached the Favored Terrain rule's 18th-level interval. Each
        // interval grants its own +2 increase to any ONE favored terrain --
        // a genuine, free player choice, so an increase targeting the same
        // terrain at all three grounded intervals STACKS (2 base + 2 + 2 +
        // 2). Absent an explicit target selection in chosen input, nothing
        // is fabricated. Mirrors the already-grounded Favored Enemy
        // 15th-level interval's own bonus-increase-target idiom exactly
        // (widened to the four-terrain set).
        let favored_terrain_third_bonus_increase_target =
            if level >= RANGER_FAVORED_TERRAIN_FOURTH_INTERVAL_LEVEL {
                choice_selection(input, RANGER_FAVORED_TERRAIN_THIRD_BONUS_INCREASE_CHOICE_ID)
            } else {
                None
            };

        if let Some(target) = favored_terrain_third_bonus_increase_target {
            let target_name = if target == RANGER_FAVORED_TERRAIN_BONUS_INCREASE_FIRST_SELECTION {
                Some("the first favored terrain")
            } else if target == RANGER_FAVORED_TERRAIN_BONUS_INCREASE_SECOND_SELECTION {
                Some("the second favored terrain")
            } else if target == RANGER_FAVORED_TERRAIN_BONUS_INCREASE_THIRD_SELECTION {
                Some("the third favored terrain")
            } else if target == RANGER_FAVORED_TERRAIN_BONUS_INCREASE_FOURTH_SELECTION {
                Some("the fourth favored terrain")
            } else {
                None
            };
            let detail = if let Some(name) = target_name {
                format!(
                    "Ranger Favored Terrain 18th-level-interval bonus-increase target selection \
                     at ranger level {level} \
                     ({RANGER_FAVORED_TERRAIN_THIRD_BONUS_INCREASE_CHOICE_ID} -> {target}): \
                     names {name} as the one favored terrain whose skill and initiative bonus \
                     increases by +2 at this 18th-level interval, per the PF1 Core Rulebook rule \
                     that the bonus in any ONE favored terrain -- including a newly selected \
                     one, if so desired -- increases by +2 at each such interval (8th, 13th, and \
                     18th ranger level); an increase targeting the same terrain at all three \
                     grounded intervals stacks. This is a recognition record of the choice slot \
                     only (+0); the increased magnitude itself is grounded separately on \
                     whichever favored terrain was actually named"
                )
            } else {
                format!(
                    "Ranger Favored Terrain 18th-level-interval bonus-increase target selection \
                     at ranger level {level} is present \
                     ({RANGER_FAVORED_TERRAIN_THIRD_BONUS_INCREASE_CHOICE_ID} -> {target}), but \
                     only the PF1 Core Rulebook restricted set (the first, second, third, or \
                     fourth favored terrain) is recognized on this bounded seam; no target \
                     identity is grounded and no mechanical value is fabricated (+0)"
                )
            };
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.favored_terrain_bonus_increase_3_choice".to_owned(),
                value: 0,
                detail,
            });
        }

        let first_favored_terrain_targeted_at_third_interval =
            favored_terrain_third_bonus_increase_target
                == Some(RANGER_FAVORED_TERRAIN_BONUS_INCREASE_FIRST_SELECTION);
        let second_favored_terrain_targeted_at_third_interval =
            favored_terrain_third_bonus_increase_target
                == Some(RANGER_FAVORED_TERRAIN_BONUS_INCREASE_SECOND_SELECTION);
        let third_favored_terrain_targeted_at_third_interval =
            favored_terrain_third_bonus_increase_target
                == Some(RANGER_FAVORED_TERRAIN_BONUS_INCREASE_THIRD_SELECTION);
        let fourth_favored_terrain_targeted_at_third_interval =
            favored_terrain_third_bonus_increase_target
                == Some(RANGER_FAVORED_TERRAIN_BONUS_INCREASE_FOURTH_SELECTION);

        let favored_terrain_bonus: i16 = 2
            + if first_favored_terrain_targeted { 2 } else { 0 }
            + if first_favored_terrain_targeted_at_second_interval { 2 } else { 0 }
            + if first_favored_terrain_targeted_at_third_interval { 2 } else { 0 };
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.favored_terrain".to_owned(),
            value: favored_terrain_bonus,
            detail: format!(
                "Ranger Favored Terrain bonus granted at ranger level {level} (PF1 Core \
                 Rulebook, 3rd-level ranger class feature): a flat \
                 +{favored_terrain_bonus} bonus on Initiative checks and Knowledge \
                 (geography), Perception, Stealth, and Survival checks made when the ranger is \
                 in the chosen favored terrain (drawn from Table: Ranger Favored Terrains --  \
                 Cold, Desert, Forest, Jungle, Mountain, Plains, Planes, Swamp, Underground, \
                 Urban, Water). This is a bounded flat-magnitude record only, non-fabricated: \
                 no terrain-detection engine decides whether the character is actually in the \
                 chosen terrain anywhere in this codebase, so no Initiative total or \
                 skill-check total is modified by this record"
            ),
        });

        // SD13-E5 ranger level 8: recognize the SECOND favored-terrain
        // selection, the rule's 8th-level interval grant. Mirrors the second
        // Favored Enemy's own choice-recognition idiom exactly (open-ended,
        // raw string interpolation, no restricted-list validation) and its
        // own flat magnitude formula (base +2, plus +2 per grounded interval
        // whose bonus-increase target names the second favored terrain).
        if level >= RANGER_FAVORED_TERRAIN_SECOND_INTERVAL_LEVEL
            && let Some(second_favored_terrain) =
                choice_selection(input, RANGER_FAVORED_TERRAIN_SECOND_CHOICE_ID)
        {
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.favored_terrain_2_choice".to_owned(),
                value: 0,
                detail: format!(
                    "Ranger 2nd Favored Terrain selection \
                     ({RANGER_FAVORED_TERRAIN_SECOND_CHOICE_ID} -> {second_favored_terrain}): \
                     at ranger level {level}, PF1 Core Rulebook Favored Terrain grants \"an \
                     additional favored terrain\" at the 8th-level interval. The \
                     level-{level} SECOND favored-terrain type chosen for this character is \
                     {second_favored_terrain}. This is a bounded recognition record of the \
                     chosen terrain type only; the flat bonus magnitude is grounded \
                     separately, and no terrain-detection or conditional-application engine \
                     is implemented, so it carries no fabricated mechanical value (+0)"
                ),
            });

            let second_favored_terrain_bonus: i16 = 2
                + if second_favored_terrain_targeted { 2 } else { 0 }
                + if second_favored_terrain_targeted_at_second_interval { 2 } else { 0 }
                + if second_favored_terrain_targeted_at_third_interval { 2 } else { 0 };
            explanations.push(ComputationExplanation {
                id: "class_feature.ranger.favored_terrain_2".to_owned(),
                value: second_favored_terrain_bonus,
                detail: format!(
                    "Ranger 2nd Favored Terrain bonus (PF1 Core Rulebook, 8th-level \
                     interval): a flat +{second_favored_terrain_bonus} bonus on Initiative \
                     checks and Knowledge (geography), Perception, Stealth, and Survival \
                     checks made when the ranger is in this second chosen favored terrain. \
                     This is a bounded flat-magnitude record only, non-fabricated: no \
                     terrain-detection engine decides whether the character is actually in \
                     the chosen terrain anywhere in this codebase, so no Initiative total or \
                     skill-check total is modified by this record"
                ),
            });
        }

        // SD18 ranger level 13: recognize the THIRD favored-terrain
        // selection, the rule's 13th-level interval grant. Mirrors the
        // third favored enemy's own choice-recognition idiom exactly
        // (open-ended, raw string interpolation, no restricted-list
        // validation) and its own flat magnitude formula (base +2, or +4 if
        // the 13th-level interval's bonus-increase target names it).
        if level >= RANGER_FAVORED_TERRAIN_THIRD_INTERVAL_LEVEL
            && let Some(third_favored_terrain) =
                choice_selection(input, RANGER_FAVORED_TERRAIN_THIRD_CHOICE_ID)
        {
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.favored_terrain_3_choice".to_owned(),
                value: 0,
                detail: format!(
                    "Ranger 3rd Favored Terrain selection \
                     ({RANGER_FAVORED_TERRAIN_THIRD_CHOICE_ID} -> {third_favored_terrain}): at \
                     ranger level {level}, PF1 Core Rulebook Favored Terrain grants \"an \
                     additional favored terrain\" at the 13th-level interval. The \
                     level-{level} THIRD favored-terrain type chosen for this character is \
                     {third_favored_terrain}. This is a bounded recognition record of the \
                     chosen terrain type only; the flat bonus magnitude is grounded \
                     separately, and no terrain-detection or conditional-application engine \
                     is implemented, so it carries no fabricated mechanical value (+0)"
                ),
            });

            let third_favored_terrain_bonus: i16 = 2
                + if third_favored_terrain_targeted_at_second_interval { 2 } else { 0 }
                + if third_favored_terrain_targeted_at_third_interval { 2 } else { 0 };
            explanations.push(ComputationExplanation {
                id: "class_feature.ranger.favored_terrain_3".to_owned(),
                value: third_favored_terrain_bonus,
                detail: format!(
                    "Ranger 3rd Favored Terrain bonus (PF1 Core Rulebook, 13th-level \
                     interval): a flat +{third_favored_terrain_bonus} bonus on Initiative \
                     checks and Knowledge (geography), Perception, Stealth, and Survival \
                     checks made when the ranger is in this third chosen favored terrain. \
                     This is a bounded flat-magnitude record only, non-fabricated: no \
                     terrain-detection engine decides whether the character is actually in \
                     the chosen terrain anywhere in this codebase, so no Initiative total or \
                     skill-check total is modified by this record"
                ),
            });
        }

        // SD18 ranger level 18: recognize the FOURTH favored-terrain
        // selection, the rule's 18th-level interval grant. Mirrors the
        // fourth favored enemy's own choice-recognition idiom exactly
        // (open-ended, raw string interpolation, no restricted-list
        // validation) and its own flat magnitude formula (base +2, or +4 if
        // the 18th-level interval's own bonus-increase target names it --
        // this is the only interval that can boost the fourth favored
        // terrain, since it is the interval that grants it).
        if level >= RANGER_FAVORED_TERRAIN_FOURTH_INTERVAL_LEVEL
            && let Some(fourth_favored_terrain) =
                choice_selection(input, RANGER_FAVORED_TERRAIN_FOURTH_CHOICE_ID)
        {
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.favored_terrain_4_choice".to_owned(),
                value: 0,
                detail: format!(
                    "Ranger 4th Favored Terrain selection \
                     ({RANGER_FAVORED_TERRAIN_FOURTH_CHOICE_ID} -> {fourth_favored_terrain}): at \
                     ranger level {level}, PF1 Core Rulebook Favored Terrain grants \"an \
                     additional favored terrain\" at the 18th-level interval. The \
                     level-{level} FOURTH favored-terrain type chosen for this character is \
                     {fourth_favored_terrain}. This is a bounded recognition record of the \
                     chosen terrain type only; the flat bonus magnitude is grounded \
                     separately, and no terrain-detection or conditional-application engine \
                     is implemented, so it carries no fabricated mechanical value (+0)"
                ),
            });

            let fourth_favored_terrain_bonus: i16 =
                2 + if fourth_favored_terrain_targeted_at_third_interval { 2 } else { 0 };
            explanations.push(ComputationExplanation {
                id: "class_feature.ranger.favored_terrain_4".to_owned(),
                value: fourth_favored_terrain_bonus,
                detail: format!(
                    "Ranger 4th Favored Terrain bonus (PF1 Core Rulebook, 18th-level \
                     interval): a flat +{fourth_favored_terrain_bonus} bonus on Initiative \
                     checks and Knowledge (geography), Perception, Stealth, and Survival \
                     checks made when the ranger is in this fourth chosen favored terrain. \
                     This is a bounded flat-magnitude record only, non-fabricated: no \
                     terrain-detection engine decides whether the character is actually in \
                     the chosen terrain anywhere in this codebase, so no Initiative total or \
                     skill-check total is modified by this record"
                ),
            });
        }
    }

    // Grounded (SD13-E5): Hunter's Bond, the class table's 4th-level "Special"
    // column entry, verified independently against two primary PF1 sources
    // (d20pfsrd and legacy.aonprd.com both list "Hunter's bond" as the Ranger
    // 4th-level special feature entry, and both state the exact rule text: "At
    // 4th level, a ranger forms a bond with his hunting companions. This bond
    // can take one of two forms. Once the form is chosen, it cannot be
    // changed."). Below the level-4 gate this is a correct level-gate absence
    // (value 0); at or above it: the chosen form (when present in chosen input)
    // is recognized as a bounded `+0` identity record naming whichever of the
    // two restricted forms was selected -- mirroring the combat-style choice
    // idiom (a restricted two-option recognition, unlike the open-ended Favored
    // Enemy/Favored Terrain choice-slots) -- and an unconditional grant-only
    // identity record is emitted, mirroring the Endurance/Favored Terrain grant
    // idiom. Only when the "bond" form is chosen is a further flat magnitude
    // grounded: half the already-grounded Favored Enemy bonus, granted to allies
    // within 30 feet who can see or hear the ranger against a single target of
    // the appropriate type. This grounds only the flat magnitude: no
    // move-action/action-economy engine, no ally-range-and-perception check, and
    // no favored-enemy target-type matching is implemented, so no ally's attack
    // or damage total is ever modified by this record. The "companion" form's
    // own animal-companion stat block/advancement subsystem is deliberately left
    // named-but-unproven: it does not exist anywhere in this codebase.
    if level < RANGER_HUNTERS_BOND_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.hunters_bond".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Hunter's Bond at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant form choice and the \
                 bond form's ally-bonus magnitude are named but not computed. Hunter's Bond is \
                 a 4th-level ranger class feature."
            ),
        });
    } else {
        let bond_selection = choice_selection(input, RANGER_HUNTERS_BOND_CHOICE_ID);
        let bond_form_name = bond_selection.and_then(|selection| {
            if selection == RANGER_HUNTERS_BOND_BOND_SELECTION {
                Some("a bond to his hunting companions")
            } else if selection == RANGER_HUNTERS_BOND_COMPANION_SELECTION {
                Some("an animal companion")
            } else {
                None
            }
        });

        if let Some(selection) = bond_selection {
            let detail = if let Some(form) = bond_form_name {
                format!(
                    "Ranger Hunter's Bond form selection at ranger level {level} \
                     ({RANGER_HUNTERS_BOND_CHOICE_ID} -> {selection}): names {form}, one of the \
                     two PF1 Core Rulebook Hunter's Bond forms granted at {RANGER_HUNTERS_BOND_LEVEL}th \
                     level. This is a recognition record of the choice slot only (+0): {form}'s own \
                     mechanics are not grounded here beyond the standalone flat magnitude recorded \
                     separately for the bond form, and no animal-companion stat block/advancement \
                     engine exists in this codebase"
                )
            } else {
                format!(
                    "Ranger Hunter's Bond form selection at ranger level {level} is present \
                     ({RANGER_HUNTERS_BOND_CHOICE_ID} -> {selection}), but only the PF1 Core \
                     Rulebook restricted pair (a bond to his hunting companions, an animal \
                     companion) is recognized on this bounded seam; no form identity is grounded \
                     and no mechanical value is fabricated (+0)"
                )
            };
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.hunters_bond_choice".to_owned(),
                value: 0,
                detail,
            });
        }

        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.hunters_bond".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Hunter's Bond granted at ranger level {level} (PF1 Core Rulebook, \
                 4th-level ranger class feature): the ranger forms a bond with his hunting \
                 companions, taking one of two forms once chosen permanently. This is a bounded \
                 grant-only identity record (value 0, non-fabricated): the chosen form's own \
                 mechanical effects are not computed here beyond the standalone flat magnitude \
                 recorded separately for the bond form, since no move-action/action-economy \
                 engine and no animal-companion stat block/advancement engine exist anywhere in \
                 this codebase"
            ),
        });

        if bond_form_name == Some("a bond to his hunting companions") {
            let hunters_bond_ally_bonus = favored_enemy_bonus / 2;
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.hunters_bond_ally_bonus".to_owned(),
                value: hunters_bond_ally_bonus,
                detail: format!(
                    "Ranger Hunter's Bond ally-bonus magnitude (PF1 Core Rulebook, level \
                     {level}, \"bond to his hunting companions\" form): half the ranger's own \
                     favored-enemy bonus ({favored_enemy_bonus} / 2 = {hunters_bond_ally_bonus}), \
                     grantable via a move action to allies within 30 feet who can see or hear the \
                     ranger, against a single target of the appropriate type. This grounds only \
                     the flat +{hunters_bond_ally_bonus} magnitude; no move-action/action-economy \
                     engine, no ally-range-and-perception check, and no favored-enemy \
                     target-type matching is implemented, so no ally's attack or damage total is \
                     ever modified by this record"
                ),
            });
        }
    }

    // Grounded (SD13-E5): Woodland Stride, the class table's 7th-level "Special"
    // column entry, verified independently against two primary PF1 sources
    // (d20pfsrd and legacy.aonprd.com both list "Woodland stride" as the Ranger
    // 7th-level special feature entry, with no other new class feature named at
    // 7th level). Unlike Track or Favored Terrain, Woodland Stride carries no
    // numeric magnitude of its own -- it is a pure boolean, no-choice grant, so
    // it mirrors the Endurance grant-only identity idiom exactly rather than a
    // flat-magnitude record. Below the level-7 gate this is a correct
    // level-gate absence (value 0); at or above it, it is a bounded grant-only
    // identity record (value 0, non-fabricated): no terrain-detection or
    // movement-resolution engine exists anywhere in this codebase to determine
    // whether the ranger is actually moving through undergrowth, so only the
    // grant itself is recorded.
    if level < RANGER_WOODLAND_STRIDE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.woodland_stride".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Woodland Stride at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant undergrowth-movement \
                 identity is named but not computed. Woodland Stride is a 7th-level ranger \
                 class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.woodland_stride".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Woodland Stride granted at ranger level {level} (PF1 Core Rulebook, \
                 7th-level ranger class feature): the ranger may move through any sort of \
                 undergrowth (such as natural thorns, briars, overgrown areas, and similar \
                 terrain) at his normal speed and without taking damage or suffering any other \
                 impairment; magically manipulated undergrowth still affects him normally. This \
                 is a bounded grant-only identity record (value 0, non-fabricated): no \
                 terrain-detection or movement-resolution engine exists anywhere in this \
                 codebase to determine whether the ranger is actually moving through \
                 undergrowth, so this only records the grant itself"
            ),
        });
    }

    // Grounded (SD13-E5): Swift Tracker, one of the class table's two 8th-level
    // "Special" column entries, verified independently against two primary PF1
    // sources (d20pfsrd and legacy.aonprd.com both list "Swift tracker" and "2nd
    // favored terrain" as the Ranger 8th-level special feature entries). Swift
    // Tracker only modifies a tracking-while-moving penalty resolution ("a
    // ranger can move at his normal speed while using Survival to follow tracks
    // without taking the normal -5 penalty. He takes only a -10 penalty
    // (instead of the normal -20) when moving at up to twice normal speed while
    // tracking") that does not exist anywhere in this codebase -- this codebase
    // grounds only the flat Track skill-bonus magnitude, never a
    // check-execution/movement-penalty engine -- so, exactly like Woodland
    // Stride, it is a genuinely flat/identity-shaped, no-choice, no-magnitude
    // grant. Below the level-8 gate this is a correct level-gate absence (value
    // 0); at or above it, it is a bounded grant-only identity record (value 0,
    // non-fabricated). The level-8 row's OTHER named entry, "2nd favored
    // terrain" (mirroring the Favored Enemy 5th-level idiom: a second
    // terrain-type selection plus a bonus-increase-target choice), is
    // deliberately left named-but-unproven this slice -- a real, newly
    // discovered multi-record burden, not an invented one.
    if level < RANGER_SWIFT_TRACKER_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.swift_tracker".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Swift Tracker at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant track-penalty-reduction \
                 identity is named but not computed. Swift Tracker is an 8th-level ranger \
                 class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.swift_tracker".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Swift Tracker granted at ranger level {level} (PF1 Core Rulebook, \
                 8th-level ranger class feature): the ranger can move at his normal speed \
                 while using Survival to follow tracks without taking the normal -5 penalty, \
                 and takes only a -10 penalty (instead of the normal -20) when moving at up \
                 to twice normal speed while tracking. This is a bounded grant-only identity \
                 record (value 0, non-fabricated): no tracking-while-moving \
                 check-execution/movement-penalty engine exists anywhere in this codebase to \
                 apply the reduced penalty, so this only records the grant itself"
            ),
        });
    }

    // Grounded (SD13-E5 level-9 slice): Evasion, the 9th-level Ranger class
    // feature verified independently against two primary PF1 sources (d20pfsrd
    // and legacy.aonprd.com both list "Evasion" as the Ranger 9th-level
    // "Special" entry — the same rule text as Rogue's and Monk's own Evasion).
    // Grounded as a bounded +0 identity/recognition record at or above the
    // gate, mirroring exactly how Rogue's and Monk's Evasion records were
    // grounded — no saving-throw-resolution or damage-resolution engine exists
    // in this codebase, so no damage math is fabricated from the record. Below
    // the level-9 gate no record is pushed at all (the level-9 slice's own
    // level-8 control pins that absence).
    if level >= RANGER_EVASION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.evasion".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Evasion granted at ranger level {level} (PF1 Core Rulebook, 9th-level \
                 ranger class feature): if the ranger makes a successful Reflex saving throw \
                 against an attack that normally deals half damage on a successful save, he \
                 instead takes no damage; Evasion can be used only when wearing light armor, \
                 medium armor, or no armor. This is a bounded identity/recognition record only \
                 (value 0, non-fabricated): no saving-throw-resolution engine and no \
                 damage-resolution engine exists anywhere in this codebase to apply it, so \
                 this grounds no actual damage reduction on any save outcome"
            ),
        });
    }

    // Grounded (SD18 cycle-2026-07-15T6100): Improved Evasion, the 16th-level
    // Ranger class feature verified independently against two primary PF1
    // sources (d20pfsrd and the Archives of Nethys aonprd.com mirror both
    // list "Improved evasion" as the Ranger 16th-level "Special" entry,
    // byte-for-byte agreement). An upgrade of the 9th-level Evasion identity:
    // the ranger still takes no damage on a successful Reflex save, and
    // henceforth takes only HALF damage on a failed save. Grounded as a
    // bounded +0 identity/recognition record only at/above the gate,
    // mirroring exactly how Monk's own Improved Evasion and Ranger's own
    // base Evasion were grounded — no saving-throw-resolution or
    // damage-resolution engine exists in this codebase, so no damage math is
    // fabricated from the record. Below the level-16 gate no record is
    // pushed at all (the level-15 slice's own level-16 negative control
    // pinned that absence).
    if level >= RANGER_IMPROVED_EVASION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.improved_evasion".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Improved Evasion granted at ranger level {level} (PF1 Core Rulebook, \
                 16th-level ranger class feature): the ranger's evasion improves — he still \
                 takes no damage on a successful Reflex saving throw against attacks, and \
                 henceforth takes only half damage on a failed save. This is a bounded \
                 identity/recognition record only (value 0, non-fabricated): no \
                 saving-throw-resolution engine and no damage-resolution engine exists anywhere \
                 in this codebase to apply it, so this grounds no actual damage reduction on \
                 any save outcome"
            ),
        });
    }

    // Grounded (SD18 cycle-2026-07-15T7000): Hide in Plain Sight, the
    // 17th-level Ranger "Special" column entry, verified independently
    // against three primary PF1 sources (d20pfsrd, the Archives of Nethys
    // aonprd.com mirror, and legacy.aonprd.com all list "Hide in plain
    // sight" as the sole Ranger 17th-level special feature entry, with
    // identical rule text): "While in any of his favored terrains, a
    // ranger of 17th level or higher can use the Stealth skill even while
    // being observed." Hide in Plain Sight carries no numeric magnitude of
    // its own and only modifies a hide-while-observed check resolution
    // that does not exist anywhere in this codebase -- exactly like
    // Camouflage, it is a genuinely flat/identity-shaped, no-choice,
    // no-magnitude grant. Below the level-17 gate this is a correct
    // level-gate absence (value 0); at or above it, it is a bounded
    // grant-only identity record (value 0, non-fabricated): no
    // terrain-classification engine and no Stealth-check-execution engine
    // exists anywhere in this codebase to determine whether the ranger is
    // actually within a favored terrain while observed, so only the grant
    // itself is recorded.
    if level < RANGER_HIDE_IN_PLAIN_SIGHT_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.hide_in_plain_sight".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Hide in Plain Sight at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant Stealth-while-observed \
                 identity is named but not computed. Hide in Plain Sight is a 17th-level ranger \
                 class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.hide_in_plain_sight".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Hide in Plain Sight granted at ranger level {level} (PF1 Core \
                 Rulebook, 17th-level ranger class feature): while in any of his favored \
                 terrains, the ranger can use the Stealth skill even while being observed. \
                 This is a bounded grant-only identity record (value 0, non-fabricated): no \
                 terrain-classification engine and no Stealth-check-execution engine exists \
                 anywhere in this codebase to determine whether the ranger is actually within a \
                 favored terrain while observed, so this only records the grant itself"
            ),
        });
    }

    // Grounded (SD18 cycle-2026-07-14T2300): Quarry, the 11th-level Ranger
    // "Special" column entry, verified independently against two primary PF1
    // sources (d20pfsrd and the Archives of Nethys aonprd.com mirror both
    // list "Quarry" as the sole Ranger 11th-level special feature entry,
    // with identical rule text): "At 11th level, a ranger can select one
    // target within line of sight as his quarry... While tracking his
    // quarry, a ranger can take 10 on his Survival skill checks while
    // moving at normal speed without penalty. In addition, the ranger
    // receives a +2 insight bonus on attack rolls made against his quarry,
    // and he confirms all critical threats against the quarry
    // automatically. A ranger can select a new quarry as a standard
    // action... Once a ranger has selected a quarry, he cannot select a
    // different quarry until 24 hours have passed or the current quarry is
    // dead, in which case he must wait 1 hour before selecting a new one."
    //
    // Quarry mirrors precedent exactly, as a three-part bundle: the
    // take-10-while-tracking and auto-confirm-critical-threats behaviors are
    // grant-only identity records (value 0, mirroring the Woodland
    // Stride/Swift Tracker idiom), since neither a Survival-check-execution
    // engine nor a critical-confirmation-roll engine exists anywhere in this
    // codebase; the chosen quarry target (when present in chosen input) is
    // an open-ended +0 recognition record (mirroring the Favored
    // Enemy/Favored Terrain choice-recognition idiom exactly, no
    // restricted-list validation, no matching against the ranger's own
    // favored-enemy types); and the rule's own flat +2 insight attack-roll
    // bonus is grounded as a standalone, non-applied magnitude (mirroring
    // the Favored Enemy attack/damage-bonus idiom exactly). Below the
    // level-11 gate this is a correct level-gate absence (value 0); no
    // active-quarry state (the 24-hour reselection cooldown, the 1-hour
    // post-kill cooldown, or "only one quarry at a time") is tracked
    // anywhere in this codebase.
    if level < RANGER_QUARRY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.quarry".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Quarry at ranger level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant target-selection identity, its flat \
                 attack-bonus magnitude, and its check/roll-resolution overrides are named but \
                 not computed. Quarry is an 11th-level ranger class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.quarry".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Quarry granted at ranger level {level} (PF1 Core Rulebook, 11th-level \
                 ranger class feature): the ranger can designate one target within line of \
                 sight as his quarry with a standard action, take 10 on Survival checks while \
                 tracking it at normal speed, and automatically confirm all critical threats \
                 made against it. This is a bounded grant-only identity record (value 0, \
                 non-fabricated): no Survival-check-execution engine and no \
                 critical-confirmation-roll engine exists anywhere in this codebase, so only \
                 the grant itself is recorded; the 24-hour reselection cooldown, the 1-hour \
                 post-kill cooldown, and the \"only one quarry at a time\" constraint are named \
                 but not tracked as state"
            ),
        });

        if let Some(quarry_target) = choice_selection(input, RANGER_QUARRY_CHOICE_ID) {
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.quarry_choice".to_owned(),
                value: 0,
                detail: format!(
                    "Ranger Quarry target selection ({RANGER_QUARRY_CHOICE_ID} -> \
                     {quarry_target}): the level-{level} quarry target named for this character \
                     is {quarry_target}. This is a bounded recognition record of the chosen \
                     target identity only; PF1 Core Rulebook restricts the quarry to a target \
                     corresponding to one of the ranger's own favored-enemy types, but no \
                     favored-enemy-type matching is implemented, so no restriction is enforced \
                     and no fabricated mechanical value is carried (+0)"
                ),
            });
        }

        // Grounded (SD18 cycle-2026-07-16T3200): the Quarry insight attack-roll
        // bonus genuinely rises from +2 to +4 at the 19th-level Improved Quarry
        // gate (RANGER_IMPROVED_QUARRY_LEVEL), verified independently against
        // two primary PF1 sources (both byte-for-byte identical: "His insight
        // bonus to attack his quarry increases to +4"). Reuses the
        // already-existing explanation id, mirroring the Bard Inspire
        // Competence tiered-magnitude idiom (same id, larger value at the
        // higher tier) rather than minting a new id for the same magnitude
        // family.
        let quarry_attack_bonus = if level >= RANGER_IMPROVED_QUARRY_LEVEL {
            4
        } else {
            2
        };
        let quarry_tier_note = if level >= RANGER_IMPROVED_QUARRY_LEVEL {
            "a flat +2 base insight bonus, raised to +4 by Improved Quarry at 19th ranger level"
        } else {
            "a flat +2 insight bonus (rises to +4 at 19th ranger level via Improved Quarry, out \
             of scope below that gate)"
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.quarry_attack_bonus".to_owned(),
            value: quarry_attack_bonus,
            detail: format!(
                "Ranger Quarry attack-roll bonus (PF1 Core Rulebook, level {level}): {quarry_tier_note} \
                 on attack rolls made against the ranger's quarry. This grounds only the flat \
                 {quarry_attack_bonus} magnitude; no target-selection engine and no \
                 conditional-application engine is implemented, so whether any specific attack \
                 is actually made against the quarry is never resolved and no combat baseline \
                 is modified by this record"
            ),
        });

        // Grounded (SD18 cycle-2026-07-16T3200): Improved Quarry, the
        // 19th-level Ranger "Special" column entry, verified independently
        // against two primary PF1 sources (d20pfsrd and the Archives of
        // Nethys aonprd.com mirror both list "Improved quarry" as the sole
        // Ranger 19th-level special feature entry, with identical rule
        // text): "At 19th level, the ranger's ability to hunt his quarry
        // improves. He can now select a quarry as a free action, and can
        // now take 20 while using Survival to track his quarry, while
        // moving at normal speed without penalty... If his quarry is
        // killed or dismissed, he can select a new one after 10 minutes
        // have passed." Grounded as a bounded +0 identity/recognition
        // record only at/above the gate, mirroring exactly how Ranger's own
        // Improved Evasion upgrade of Evasion was grounded (no separate
        // absence record below the gate) — no action-economy engine and no
        // Survival-check-execution engine exists in this codebase, so
        // neither the free-action reselection nor the take-20 tracking
        // behavior is ever applied to any actual roll or action
        // resolution. The insight attack-roll bonus increase (+2 to +4) is
        // grounded separately above, on the already-existing
        // `quarry_attack_bonus` explanation id.
        if level >= RANGER_IMPROVED_QUARRY_LEVEL {
            explanations.push(ComputationExplanation {
                id: "class_feature.ranger.improved_quarry".to_owned(),
                value: 0,
                detail: format!(
                    "Ranger Improved Quarry granted at ranger level {level} (PF1 Core \
                     Rulebook, 19th-level ranger class feature): the ranger's ability to hunt \
                     his quarry improves — he can now select a quarry as a free action \
                     (up from a standard action) and can now take 20 while using Survival to \
                     track his quarry while moving at normal speed without penalty (up from \
                     take 10); if his quarry is killed or dismissed, he can select a new one \
                     after 10 minutes have passed (down from the base Quarry cooldown of 24 \
                     hours, or 1 hour after the quarry is confirmed dead). This is a bounded \
                     identity/recognition record only (value 0, non-fabricated): no \
                     action-economy engine and no Survival-check-execution engine exists \
                     anywhere in this codebase to apply it, so this grounds no actual \
                     free-action resolution, take-20 resolution, or cooldown-state tracking"
                ),
            });
        }
    }

    // Grounded (SD18 cycle-2026-07-15T0900): Camouflage, the 12th-level
    // Ranger "Special" column entry, verified independently against two
    // primary PF1 sources (d20pfsrd and the Archives of Nethys aonprd.com
    // mirror both list "Camouflage" as the sole Ranger 12th-level special
    // feature entry, with identical rule text): "A ranger of 12th level or
    // higher can use the Stealth skill to hide, even while being observed,
    // as long as she is within any sort of natural terrain that grants at
    // least partial concealment or partial cover." Camouflage carries no
    // numeric magnitude of its own and only modifies a
    // hide-while-observed check resolution that does not exist anywhere in
    // this codebase -- exactly like Woodland Stride and Swift Tracker, it
    // is a genuinely flat/identity-shaped, no-choice, no-magnitude grant.
    // Below the level-12 gate this is a correct level-gate absence (value
    // 0); at or above it, it is a bounded grant-only identity record (value
    // 0, non-fabricated): no terrain-classification engine and no
    // Stealth-check-execution engine exists anywhere in this codebase to
    // determine whether the ranger is actually within qualifying terrain
    // while observed, so only the grant itself is recorded.
    if level < RANGER_CAMOUFLAGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.camouflage".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Camouflage at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant Stealth-while-observed \
                 identity is named but not computed. Camouflage is a 12th-level ranger class \
                 feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.camouflage".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Camouflage granted at ranger level {level} (PF1 Core Rulebook, \
                 12th-level ranger class feature): the ranger can use the Stealth skill to \
                 hide, even while being observed, as long as she is within any sort of \
                 natural terrain that grants at least partial concealment or partial cover. \
                 This is a bounded grant-only identity record (value 0, non-fabricated): no \
                 terrain-classification engine and no Stealth-check-execution engine exists \
                 anywhere in this codebase to determine whether the ranger is actually within \
                 qualifying terrain while observed, so this only records the grant itself"
            ),
        });
    }

    // SD13-E5: the Ranger partial-caster identity pair, mirroring the
    // Paladin's effective_caster_level + spell_level_access records
    // record-for-record (PF1 CRB Ranger Spells: "At 4th level and higher,
    // his caster level is equal to his ranger level – 3"; access ladder per
    // the Cleric/Wizard "first non-'—' spells-per-day column" threshold
    // doctrine, verified against the raw table rows of both primary
    // sources). Both records ground gate arithmetic and the access ladder
    // ONLY — no per-day counts, no prepared posture (Wisdom-based, from the
    // ranger list), no bonus slots, and no spell save DCs. The real
    // claim-blocking diagnostic for this burden is pushed unconditionally
    // near the top of this function (covering both single-class and
    // multiclass Ranger) — see that diagnostic's own doc comment for why.
    let ranger_effective_caster_level = (level_value - 3).max(0);
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.partial_caster.effective_caster_level".to_owned(),
        value: ranger_effective_caster_level,
        detail: format!(
            "Ranger effective caster level at ranger level {level}: max(ranger level - 3, 0) = \
             max({level_value} - 3, 0) = {ranger_effective_caster_level} (PF1 Core Rulebook: \
             \"At 4th level and higher, his caster level is equal to his ranger level – 3\"; \
             ranger spells begin at ranger level 4). This grounds only the caster-level gate \
             arithmetic; it computes no spells known or prepared, no spells per day, no bonus \
             spell slots, and no spell save DCs"
        ),
    });

    let ranger_spell_level_access: i16 = ranger_spell_level_access(level);
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.partial_caster.spell_level_access".to_owned(),
        value: ranger_spell_level_access,
        detail: format!(
            "Ranger spell-level access at ranger level {level}: the highest ranger spell level \
             with a non-\"—\" spells-per-day column in the PF1 Core Rulebook Ranger class \
             table is {ranger_spell_level_access} (verified against the raw table rows of \
             all three primary sources: 1st-level spells begin at ranger level \
             {RANGER_FIRST_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 2nd-level at \
             {RANGER_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 3rd-level at \
             {RANGER_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 4th-level at \
             {RANGER_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}). A gate-level \"0\" \
             spells-per-day entry is access via Wisdom bonus spells only, per the PF1 rule \
             text — Wisdom, not the Paladin's Charisma. This grounds the access ladder only: \
             no spells-per-day counts, no spells known or prepared posture, no bonus slots \
             from a high Wisdom, and no spell save DCs are computed"
        ),
    });

    // SD13-E5: the BASE spells-per-day counts, one record per ACCESSIBLE
    // spell level, as a literal table lookup mirroring the Paladin per-day
    // slice and the Cleric domain-slot-count precedent — the PF1
    // spells-per-day table is a lookup table, not arithmetic, so no formula
    // is invented for it. Verified against the raw table rows of both
    // primary sources (identical on d20pfsrd and legacy.aonprd.com, and
    // numerically identical to the Paladin's rows): level 4 "0/—/—/—",
    // level 5 "1/—/—/—", level 6 "1/—/—/—", level 7 "1/0/—/—", level 8
    // "1/1/—/—", level 9 "2/1/—/—", level 10 "2/1/0/—", level 11 "2/1/1/—"
    // (verified independently for the SD18 level-11 widening cycle: the
    // 3rd-level column genuinely rises from 0 to 1, the 1st/2nd-level
    // columns stay 2/1 unchanged, and the 4th-level column stays "—" —
    // 4th-level ranger spells begin at level 13, outside this row's ceiling,
    // checked rather than assumed away), level 12 "2/2/1/—" (verified
    // independently for the SD18 level-12 widening cycle: the 2nd-level
    // column genuinely rises from 1 to 2, the 1st/3rd-level columns stay
    // 2/1 unchanged, and the 4th-level column stays "—"), and level 13
    // "3/2/1/0" (verified independently for the SD18 level-13 widening
    // cycle against all three primary sources: the 1st-level column
    // genuinely rises from 2 to 3, the 2nd/3rd-level columns stay 2/1
    // unchanged, and the 4th-level column newly opens at 0 — a genuine
    // table entry, not an absence), and level 14 "3/2/1/1" (verified
    // independently for the SD18 level-14 widening cycle against all three
    // primary sources: the 1st/2nd/3rd-level columns stay 3/2/1 unchanged,
    // and the 4th-level column genuinely rises from 0 to 1), and level 15
    // "3/2/2/1" (verified independently for the SD18 level-15 widening
    // cycle against two primary sources, d20pfsrd and the Archives of
    // Nethys aonprd.com mirror, byte-for-byte agreement: the 1st/2nd/4th-
    // level columns stay 3/2/1 unchanged, and the 3rd-level column
    // genuinely rises from 1 to 2), and level 16 "3/3/2/1" (verified
    // independently for the SD18 level-16 widening cycle against two
    // primary sources, d20pfsrd and the Archives of Nethys aonprd.com
    // mirror, byte-for-byte agreement: the 2nd/3rd/4th-level columns stay
    // 3/2/1 unchanged, and the 1st-level column genuinely rises from 2 to
    // 3), and level 17 "4/3/2/1" (verified independently for the SD18
    // level-17 widening cycle against three primary sources, d20pfsrd, the
    // Archives of Nethys aonprd.com mirror, and legacy.aonprd.com, all
    // byte-for-byte identical: the 2nd/3rd/4th-level columns stay 3/2/1
    // unchanged, and the 1st-level column genuinely rises from 3 to 4), and
    // level 18 "4/3/2/2" (verified independently for the SD18 level-18
    // widening cycle against two primary sources, d20pfsrd and the
    // Archives of Nethys aonprd.com mirror, byte-for-byte agreement: the
    // 1st/2nd/3rd-level columns stay 4/3/2 unchanged, and the 4th-level
    // column genuinely rises from 1 to 2 -- numerically identical to the
    // already-landed Paladin level-18 row), and level 19 "4/3/3/2"
    // (verified independently for the SD18 level-19 widening cycle
    // against two primary sources, d20pfsrd and the Archives of Nethys
    // aonprd.com mirror, byte-for-byte agreement: the 1st/2nd/4th-level
    // columns stay 4/3/2 unchanged, and the 3rd-level column genuinely
    // rises from 2 to 3), and level 20 "4/4/3/3" (verified independently
    // for the SD18 level-20 widening cycle against two primary sources,
    // d20pfsrd and the Archives of Nethys aonprd.com mirror, byte-for-byte
    // agreement: the 1st/3rd-level columns stay 4/3 unchanged, and the
    // 2nd/4th-level columns both genuinely rise at once, 3 to 4 and 2 to
    // 3). A
    // "0" is a
    // genuine table entry (Wisdom-bonus-spells-only access), NOT an
    // absence — inaccessible spell levels ("—" columns) get no record at
    // all. Only the base counts are grounded: bonus spells per day from a
    // high Wisdom are never computed.
    let ranger_base_spells_per_day: [Option<i16>; 4] = ranger_base_spells_per_day_table(level);
    for (index, base_count) in ranger_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = index + 1;
        let zero_nuance = if *base_count == 0 {
            " A base count of 0 is a genuine table entry, not an absence: per the PF1 rule \
             text, the ranger gains only the bonus spells he would be entitled to based on \
             his Wisdom score for that spell level."
        } else {
            ""
        };
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.ranger.partial_caster.base_spells_per_day.spell_level_{spell_level}"
            ),
            value: *base_count,
            detail: format!(
                "Ranger base spells per day at ranger level {level}, spell level \
                 {spell_level}: {base_count}, read directly from the PF1 Core Rulebook \
                 Ranger class table's spells-per-day row (verified against the raw table \
                 rows of both primary sources; a literal table lookup, not a derived \
                 formula).{zero_nuance} This grounds the base count only: bonus spells per \
                 day from a high Wisdom are never computed, no prepared posture or \
                 spell-source lineage is grounded, and no spell save DCs are computed"
            ),
        });
    }

    // SD13-E5: the base spell-save-DC arithmetic, one record per ACCESSIBLE
    // spell level, mirroring the Sorcerer/Bard/Paladin DC slices and
    // completing the DC family. Verified against both primary sources,
    // which state the rule identically: "The Difficulty Class for a saving
    // throw against a ranger's spell is 10 + the spell level + the ranger's
    // Wisdom modifier." — the family's only WISDOM caster. This grounds
    // only the base formula over values already on the seam: no
    // saving-throw resolution, no target, no spell selection, and no feat
    // DC modifiers are computed.
    for spell_level in 1..=ranger_spell_level_access {
        let spell_save_dc = 10 + spell_level + ability_modifiers.wisdom;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.ranger.partial_caster.spell_save_dc.spell_level_{spell_level}"
            ),
            value: spell_save_dc,
            detail: format!(
                "Ranger spell save DC at ranger level {level}, spell level {spell_level}: \
                 10 + {spell_level} + Wisdom modifier {} = {spell_save_dc} (PF1 Core \
                 Rulebook, verified identically on both primary sources: \"The Difficulty \
                 Class for a saving throw against a ranger's spell is 10 + the spell level \
                 + the ranger's Wisdom modifier\" — Wisdom, not the Paladin's Charisma). \
                 This grounds the base DC formula only: no saving-throw resolution, no \
                 target, no spell selection, and no feat DC modifiers are computed",
                ability_modifiers.wisdom
            ),
        });
    }

    // SD13-E5: the bonus spells per day from a high Wisdom, one record per
    // ACCESSIBLE spell level, from PF1's shared Table: Ability Modifiers
    // and Bonus Spells, completing the bonus family across all four
    // partial/spontaneous casters — verified against both primary sources'
    // ability-scores pages: for modifier m and spell level N, 0 when
    // m < N, otherwise (m - N)/4 + 1, gated by the grounded access ladder.
    // The ranger-specific rule text ("he gains only the bonus spells he
    // would be entitled to based on his Wisdom score for that spell
    // level") was verified on both class pages — Wisdom, the family's only
    // non-Charisma caster. The bonus is never added to the base per-day
    // counts here — no total is computed.
    for spell_level in 1..=ranger_spell_level_access {
        let bonus_spells = if ability_modifiers.wisdom < spell_level {
            0
        } else {
            (ability_modifiers.wisdom - spell_level) / 4 + 1
        };
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.ranger.partial_caster.bonus_spells_per_day.spell_level_{spell_level}"
            ),
            value: bonus_spells,
            detail: format!(
                "Ranger bonus spells per day at ranger level {level}, spell level \
                 {spell_level}: {bonus_spells} from Wisdom modifier {} (PF1 Core Rulebook \
                 Table: Ability Modifiers and Bonus Spells, verified identically on both \
                 primary sources; for modifier m and spell level N the table value is 0 \
                 when m < N, otherwise (m - N)/4 + 1, and bonus spells apply only to spell \
                 levels the character is of a high enough class level to cast — the \
                 grounded access ladder). A computed 0 means the modifier grants no bonus \
                 at this spell level; it is never added to the base per-day count here — no \
                 total is computed, no spell selection, and no spell save DCs",
                ability_modifiers.wisdom
            ),
        });
    }

    // SD13-E5: the TOTAL spells per day — the pure sum of the two records
    // grounded above (base table count + Wisdom bonus count) per ACCESSIBLE
    // spell level, completing the integrated totals across all four
    // partial/spontaneous casters. No new rules content: each input record
    // carries its own two-source verification. The gate-level "0"-base
    // entries land as arithmetic (level-4 total 1 from the bonus alone;
    // level-7 2nd and level-10 3rd are honest ZERO totals at Wisdom +1 —
    // accessible but currently uncastable). Counts only — no
    // prepared-posture selection, no casting execution, no slot consumption
    // or tracking, no save resolution.
    for (index, base_count) in ranger_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = (index + 1) as i16;
        let bonus_spells = if ability_modifiers.wisdom < spell_level {
            0
        } else {
            (ability_modifiers.wisdom - spell_level) / 4 + 1
        };
        let total_spells = base_count + bonus_spells;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.ranger.partial_caster.total_spells_per_day.spell_level_{spell_level}"
            ),
            value: total_spells,
            detail: format!(
                "Ranger total spells per day at ranger level {level}, spell level \
                 {spell_level}: base table count {base_count} + Wisdom bonus \
                 {bonus_spells} = {total_spells} — the pure sum of the two separately \
                 grounded records (each carrying its own two-source verification), giving \
                 the actual castable slot count per day; a total of 0 is honest arithmetic \
                 (accessible spell level, no castable slots at this Wisdom). This grounds \
                 the count only: no prepared-posture selection, no casting execution, no \
                 slot consumption or tracking, and no spell save resolution"
            ),
        });
    }

    // SD-34 AT-34-E3-001 (`decisions.md §16`): the combat-style feat pool's
    // slot COUNT grounds regardless of whether this seam has recognized
    // which style (Archery or Two-Weapon Combat) the character chose.
    ground_ranger_combat_style_feat_pool(level, explanations, diagnostics);
}

/// The highest ranger spell level with a non-"—" spells-per-day column at
/// the given ranger level (0 means no spell access yet). Pure function,
/// race-independent -- extracted so both the (Human-only) flat explanation
/// block above and the real prepared-spell validation below share one
/// source of truth instead of two copies of the same level-gate ladder.
pub(super) fn ranger_spell_level_access(level: u8) -> i16 {
    if level >= RANGER_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        4
    } else if level >= RANGER_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        3
    } else if level >= RANGER_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        2
    } else if level >= RANGER_FIRST_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        1
    } else {
        0
    }
}

/// The PF1 Core Rulebook Ranger class table's BASE spells-per-day row, one
/// entry per spell level 1-4 (`None` for an inaccessible "—" column). A
/// literal table lookup, not a derived formula -- see
/// `explain_ranger_level1_chassis_and_class_feature_separation`'s own doc
/// comment for the two-source verification history of every row. Pure
/// function, race-independent, extracted for the same reason as
/// `ranger_spell_level_access`.
pub(super) fn ranger_base_spells_per_day_table(level: u8) -> [Option<i16>; 4] {
    match level {
        4 => [Some(0), None, None, None],
        5 | 6 => [Some(1), None, None, None],
        7 => [Some(1), Some(0), None, None],
        8 => [Some(1), Some(1), None, None],
        9 => [Some(2), Some(1), None, None],
        10 => [Some(2), Some(1), Some(0), None],
        11 => [Some(2), Some(1), Some(1), None],
        12 => [Some(2), Some(2), Some(1), None],
        13 => [Some(3), Some(2), Some(1), Some(0)],
        14 => [Some(3), Some(2), Some(1), Some(1)],
        15 => [Some(3), Some(2), Some(2), Some(1)],
        16 => [Some(3), Some(3), Some(2), Some(1)],
        17 => [Some(4), Some(3), Some(2), Some(1)],
        18 => [Some(4), Some(3), Some(2), Some(2)],
        19 => [Some(4), Some(3), Some(3), Some(2)],
        20 => [Some(4), Some(4), Some(3), Some(3)],
        _ => [None, None, None, None],
    }
}

/// The real per-day slot budget per spell level 1-4 (base table count +
/// Wisdom bonus, `None` for an inaccessible column), reusing
/// `ability_bonus_spells` -- the same shared "Table: Ability Modifiers and
/// Bonus Spells" formula already grounded for Wizard/Paladin/Sorcerer/Bard.
pub(super) fn ranger_total_spells_per_day(level: u8, wisdom_modifier: i16) -> [Option<i16>; 4] {
    let base = ranger_base_spells_per_day_table(level);
    let mut total = [None; 4];
    for (index, base_count) in base.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = (index + 1) as i16;
        total[index] = Some(base_count + ability_bonus_spells(wisdom_modifier, spell_level));
    }
    total
}

/// Return the list of unmet conditions for Ranger's real prepared-spell
/// posture. An empty list means the posture is fully valid: every
/// `AcquisitionMode::Prepared` selection with `source_class_id ==
/// "class:ranger"` names a real spell on `ranger_spell_list::RANGER_SPELL_LIST`,
/// at a spell level within the ranger's own access ceiling for their ranger
/// level, and no spell level's prepared count exceeds that level's total
/// slot budget (base table count + Wisdom bonus). Zero prepared spells is
/// always valid -- see the call site's own doc comment for why this
/// deliberately does not mirror Wizard's "at least one recorded and one
/// prepared" requirement.
pub(super) fn unmet_ranger_prepared_spell_conditions(
    input: &CharacterInput,
    ranger_level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Vec<String> {
    let mut unmet = Vec::new();

    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == RANGER_CLASS_ID && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    let access_ceiling = ranger_spell_level_access(ranger_level);
    let total_per_day = ranger_total_spells_per_day(ranger_level, ability_modifiers.wisdom);

    let mut consumed_per_level: [i16; 4] = [0; 4];
    for spell_id in &prepared {
        let Some(spell_level) = ranger_spell_list::ranger_spell_level(spell_id) else {
            unmet.push(format!(
                "prepared spell '{spell_id}' is not on the real PF1 ranger spell list"
            ));
            continue;
        };
        if i16::from(spell_level) > access_ceiling {
            unmet.push(format!(
                "prepared spell '{spell_id}' targets spell level {spell_level}, not yet \
                 accessible at ranger level {ranger_level} (access ceiling {access_ceiling})"
            ));
            continue;
        }
        consumed_per_level[usize::from(spell_level) - 1] += 1;
    }

    for (index, consumed) in consumed_per_level.iter().enumerate() {
        if *consumed == 0 {
            continue;
        }
        let spell_level = index + 1;
        let total_slots = total_per_day[index].unwrap_or(0);
        if *consumed > total_slots {
            unmet.push(format!(
                "spell level {spell_level} over-prepared: {consumed} spells prepared but only \
                 {total_slots} slots available (base {} + Wisdom bonus)",
                ranger_base_spells_per_day_table(ranger_level)[index].unwrap_or(0)
            ));
        }
    }

    unmet
}

/// Ground the real prepared-spell posture once
/// `unmet_ranger_prepared_spell_conditions` reports an empty unmet list:
/// the daily preparation selection (count + list, mirroring
/// `class_spell.wizard.daily_preparation`'s shape) and the total
/// spells-per-day budget per accessible spell level.
pub(super) fn ground_ranger_prepared_spells(
    input: &CharacterInput,
    ranger_level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == RANGER_CLASS_ID && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.ranger.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Ranger level {ranger_level} daily preparation selection ({} spells, \
             AcquisitionMode::Prepared): {}. Each prepared spell is verified against the real \
             PF1 ranger spell list (`ranger_spell_list::RANGER_SPELL_LIST`, all ingested \
             books), the \
             ranger's own spell-level access ceiling, and the per-level slot budget (base table \
             count + Wisdom bonus). Real PF1 Ranger rules have no personal 'recorded spellbook' \
             step (unlike Wizard) -- a ranger prepares directly from the full ranger spell list \
             each day. This grounds the prepared-spell selection for real; it computes no spell \
             save DC resolution against a target and no casting execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let total_per_day = ranger_total_spells_per_day(ranger_level, ability_modifiers.wisdom);
    for (index, total) in total_per_day.iter().enumerate() {
        let Some(total) = total else {
            continue;
        };
        let spell_level = index + 1;
        explanations.push(ComputationExplanation {
            id: format!("class_spell.ranger.total_spells_per_day.spell_level_{spell_level}"),
            value: *total,
            detail: format!(
                "Ranger level {ranger_level} total spells per day at spell level {spell_level}: \
                 {total} (base table count + Wisdom bonus, the same records already grounded as \
                 `class_chassis.ranger.partial_caster.total_spells_per_day.spell_level_{spell_level}` \
                 for a Human ranger, computed here independent of race). This is the real slot \
                 budget the daily preparation selection above is validated against"
            ),
        });
    }
}

/// v0.6 alpha swarm, risks item 8 (2026-07-24, real spell posture landed
/// 2026-07-25): direct verification of Ranger's real prepared-spell
/// posture, for both a single-class Ranger at any supported level and a
/// Ranger-containing multiclass mix. The first version of this module
/// (written after an adversarial scoping review found widening
/// `table_class_id` to recognize Ranger was a real, not theoretical, false-
/// `Computed` risk: `multiclass_class_level_supported`/
/// `is_supported_multiclass_mix` accept a Ranger+Fighter/Wizard/Rogue mix,
/// and `compute_multiclass_base_chassis` deliberately discards each
/// isolated per-class sub-computation's own diagnostics) asserted Ranger
/// stayed `Blocked` unconditionally. That is no longer true: Ranger's spell
/// posture is now genuinely computed (`unmet_ranger_prepared_spell_conditions`
/// / `ground_ranger_prepared_spells`), so a Ranger (alone or multiclassed)
/// with no invalid `AcquisitionMode::Prepared` selection now reaches real
/// `Computed` -- this module proves both that positive case and that a
/// genuine posture violation (an off-list spell, a too-high spell level,
/// or an over-prepared slot) still blocks, in both the single-class and
/// multiclass shapes, so the original adversarial-review finding's spirit
/// (nothing silently bypasses this diagnostic in a multiclass mix) still
/// holds.
#[cfg(test)]
mod ranger_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, HeadlessReceiptStatus, FIGHTER_CLASS_ID,
        RANGER_CLASS_ID, ROGUE_CLASS_ID, WIZARD_CLASS_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, AcquisitionMode, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// A single-class Ranger at a representative mid-range level (5, inside
    /// the partial-caster range where the original false-positive risk was
    /// found), with no spells prepared, now genuinely reaches `Computed` --
    /// the first real proof Ranger can reach `Computed` at all, pinned
    /// directly here (not relying solely on the QA-owned `tests/**` files
    /// this widening also affects) so a future change to those files can't
    /// silently drop this coverage.
    #[test]
    fn single_class_ranger_level5_with_no_prepared_spells_reaches_computed() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: RANGER_CLASS_ID.to_owned(), level: 5 }];

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a single-class Ranger with a valid (empty) spell posture must reach Computed: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.ranger.partial_caster.unsupported"),
            "the spell-posture diagnostic must not fire when the posture is valid: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The permanent-test gap the lead found while verifying b7642d97: the
    /// existing positive proof above only exercises the *empty* prepared-spell
    /// case, which can't by itself distinguish "correctly validates real
    /// spells" from "just happens to allow the trivial empty case" -- a
    /// vacuous check would pass both this test's absence and the empty-case
    /// test identically. This proves a real, *non-empty*, genuinely valid
    /// prepared spell also reaches `Computed`: "Alarm" is a real 1st-level
    /// ranger spell (`rules_tables::crb::ranger_spell_list`), accessible at
    /// ranger level 4 (spells begin at ranger level 4), and fits the real
    /// total budget at that level (base 0 + Wisdom-12 bonus 1 = 1 slot,
    /// exactly matching this fixture's own
    /// `single_class_ranger_over_prepared_slot_budget_stays_blocked` sibling
    /// test's own cited budget arithmetic for the identical level/Wisdom
    /// combination, just staying within it here instead of exceeding it).
    #[test]
    fn single_class_ranger_with_a_real_valid_prepared_spell_reaches_computed() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: RANGER_CLASS_ID.to_owned(), level: 4 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Alarm".to_owned(),
            source_class_id: RANGER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a single-class Ranger preparing a real, in-budget, accessible spell must reach \
             Computed, not just an empty prepared-spell list: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.ranger.partial_caster.unsupported"),
            "the spell-posture diagnostic must not fire when a real prepared spell is \
             genuinely valid: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Ranger with a genuinely invalid prepared-spell
    /// selection (a spell level not yet accessible at ranger level 5, whose
    /// highest accessible level is 1st) must still stay `Blocked`, carrying
    /// the real spell-posture diagnostic.
    #[test]
    fn single_class_ranger_with_an_inaccessible_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: RANGER_CLASS_ID.to_owned(), level: 5 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Animal Growth".to_owned(),
            source_class_id: RANGER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "a 4th-level ranger spell is not accessible at ranger level 5 (access ceiling 1st): \
             {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.ranger.partial_caster.unsupported" && d.claim_blocking),
            "expected the real spell-posture diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Ranger at level 4 (base spells-per-day for 1st-level
    /// spells is a genuine 0 -- the fixture's Wisdom 12 grants exactly one
    /// bonus slot, so the real total budget is 1) preparing the same real
    /// 1st-level ranger spell twice consumes 2 against a budget of 1 and
    /// must stay `Blocked`.
    #[test]
    fn single_class_ranger_over_prepared_slot_budget_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: RANGER_CLASS_ID.to_owned(), level: 4 }];
        for _ in 0..2 {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: "Alarm".to_owned(),
                source_class_id: RANGER_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Prepared,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "ranger level 4 with Wisdom 12 has a real total budget of 1 slot for 1st-level \
             spells (base 0 + Wisdom bonus 1), so preparing the same spell twice over-prepares: \
             {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.ranger.partial_caster.unsupported" && d.claim_blocking),
            "expected the real spell-posture diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Ranger preparing a spell that is not on the real PF1
    /// ranger spell list at all must also stay `Blocked`.
    #[test]
    fn single_class_ranger_with_an_off_list_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: RANGER_CLASS_ID.to_owned(), level: 4 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: RANGER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Magic Missile is not on the real ranger spell list: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A valid Ranger+Fighter multiclass mix (no invalid preparation) now
    /// genuinely reaches `Computed` too -- the multiclass counterpart of the
    /// single-class positive proof above.
    #[test]
    fn ranger_fighter_multiclass_with_no_prepared_spells_reaches_computed() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: RANGER_CLASS_ID.to_owned(), level: 4 },
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
        ];

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Ranger+Fighter multiclass with a valid spell posture must reach Computed: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The real regression the original review found still holds: a
    /// Ranger+Fighter multiclass mix with a genuine posture violation must
    /// still stay `Blocked`, not silently reach `Computed` via the isolated
    /// per-class sub-computation path that discards diagnostics.
    #[test]
    fn ranger_fighter_multiclass_with_an_invalid_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: RANGER_CLASS_ID.to_owned(), level: 4 },
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
        ];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: RANGER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "a Ranger+Fighter multiclass must not reach Computed while Ranger's spell posture \
             is genuinely violated: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.ranger.partial_caster.unsupported" && d.claim_blocking),
            "expected the real spell-posture diagnostic to fire in the multiclass mix too: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Same positive proof for Ranger+Rogue -- the risk is generic to any
    /// multiclass partner `table_class_id` already recognized, not specific
    /// to Fighter. (Ranger+Wizard is deliberately not used for the positive
    /// direction here: a level-1 Wizard has its own, unrelated posture
    /// requirement -- the canonical Evocation specialization -- that a bare
    /// fixture doesn't satisfy, which would conflate a Wizard-side gate
    /// with what this test is actually proving about Ranger. Ranger+Wizard
    /// is exercised below for the negative direction instead, where either
    /// class's own blocker independently proves the mix cannot be falsely
    /// Computed.)
    #[test]
    fn ranger_rogue_multiclass_both_directions() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let base_input = result.character_input.expect("valid fixture");

        let mut valid_input = base_input.clone();
        valid_input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: RANGER_CLASS_ID.to_owned(), level: 4 },
            CharacterClassLevel { class_id: ROGUE_CLASS_ID.to_owned(), level: 1 },
        ];
        let valid_receipt = build_pilot_headless_receipt(&valid_input);
        assert_eq!(
            valid_receipt.status,
            HeadlessReceiptStatus::Computed,
            "Ranger+Rogue multiclass with a valid spell posture must reach Computed: {:?}",
            valid_receipt.computation.diagnostics
        );

        let mut invalid_input = valid_input;
        invalid_input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: RANGER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
        let invalid_receipt = build_pilot_headless_receipt(&invalid_input);
        assert_eq!(
            invalid_receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Ranger+Rogue multiclass must still block a genuine posture violation: {:?}",
            invalid_receipt.computation.diagnostics
        );
    }

    /// Ranger+Wizard multiclass still cannot be falsely Computed when
    /// Ranger's own posture is genuinely violated -- proving the original
    /// adversarial-review finding's fix isn't specific to Fighter/Rogue
    /// partners. (Not tested for the positive direction: see this test
    /// module's doc comment on why a bare level-1 Wizard has its own
    /// unrelated blocker.)
    #[test]
    fn ranger_wizard_multiclass_with_an_invalid_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: RANGER_CLASS_ID.to_owned(), level: 4 },
            CharacterClassLevel { class_id: WIZARD_CLASS_ID.to_owned(), level: 1 },
        ];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: RANGER_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Ranger+Wizard multiclass must block a genuine Ranger posture violation: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.ranger.partial_caster.unsupported" && d.claim_blocking),
            "expected the real Ranger spell-posture diagnostic to fire in this mix too: {:?}",
            receipt.computation.diagnostics
        );
    }
}

/// v0.6 alpha swarm, risks item 8, third slice (2026-07-25): Paladin's real
/// prepared-spell posture, mirroring `ranger_dispatch_widening_safety_tests`
/// exactly (same shape: a genuine structural risk existed here too --
/// `explain_paladin_level1_chassis_and_spell_burden_separation`'s own
/// diagnostic was pushed AFTER the single-class-only/Human gate, so widening
/// `table_class_id` to recognize Paladin would have let a Paladin+X
/// multiclass or a non-Human Paladin silently reach `Computed` the exact
/// way the Ranger adversarial review found -- fixed as part of this same
/// slice, not left for a follow-up, since the fix was already proven and
/// cheap to apply directly this time).
#[cfg(test)]
mod paladin_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, CharacterClassLevel, HeadlessReceiptStatus,
        FIGHTER_CLASS_ID, PALADIN_CLASS_ID, ROGUE_CLASS_ID,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, SpellSelection};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// A single-class Paladin at a representative mid-range level (5, inside
    /// the partial-caster range), with no spells prepared, genuinely reaches
    /// `Computed` -- proving the same real posture that landed for Ranger
    /// now also applies to Paladin.
    #[test]
    fn single_class_paladin_level5_with_no_prepared_spells_reaches_computed() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: PALADIN_CLASS_ID.to_owned(), level: 5 }];

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a single-class Paladin with a valid (empty) spell posture must reach Computed: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.paladin.partial_caster.unsupported"),
            "the spell-posture diagnostic must not fire when the posture is valid: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A non-Human single-class Paladin with no prepared spells must also
    /// reach `Computed` -- the fix moved the check outside the Human-only
    /// gate deliberately, so this proves the gate no longer silently
    /// exempts non-Human Paladins from validation (or from Computed).
    #[test]
    fn non_human_single_class_paladin_with_no_prepared_spells_reaches_computed() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.race_id = "race:elf".to_owned();
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: PALADIN_CLASS_ID.to_owned(), level: 5 }];

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a non-Human Paladin with a valid spell posture must also reach Computed: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Paladin preparing a spell beyond their spell-level
    /// access ceiling must stay `Blocked`.
    #[test]
    fn single_class_paladin_with_an_inaccessible_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: PALADIN_CLASS_ID.to_owned(), level: 5 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Break Enchantment".to_owned(),
            source_class_id: PALADIN_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "a 4th-level paladin spell is not accessible at paladin level 5 (access ceiling \
             1st): {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.paladin.partial_caster.unsupported"
                    && d.claim_blocking),
            "expected the real spell-posture diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Paladin preparing a spell not on the real PF1 Core
    /// Rulebook paladin spell list must also stay `Blocked`.
    #[test]
    fn single_class_paladin_with_an_off_list_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: PALADIN_CLASS_ID.to_owned(), level: 4 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: PALADIN_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Magic Missile is not on the real paladin spell list: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A valid Paladin+Fighter multiclass mix (no invalid preparation) now
    /// genuinely reaches `Computed` too.
    #[test]
    fn paladin_fighter_multiclass_with_no_prepared_spells_reaches_computed() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: PALADIN_CLASS_ID.to_owned(), level: 4 },
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
        ];

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Paladin+Fighter multiclass with a valid spell posture must reach Computed: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// The real regression this mirrors: a Paladin+Fighter multiclass mix
    /// with a genuine posture violation must still stay `Blocked`, not
    /// silently reach `Computed` via the isolated per-class sub-computation
    /// path that discards diagnostics.
    #[test]
    fn paladin_fighter_multiclass_with_an_invalid_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: PALADIN_CLASS_ID.to_owned(), level: 4 },
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
        ];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: PALADIN_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "a Paladin+Fighter multiclass must not reach Computed while Paladin's spell posture \
             is genuinely violated: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.paladin.partial_caster.unsupported"
                    && d.claim_blocking),
            "expected the real spell-posture diagnostic to fire in the multiclass mix too: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Same positive+negative proof for Paladin+Rogue -- the risk is generic
    /// to any multiclass partner `table_class_id` already recognizes, not
    /// specific to Fighter.
    #[test]
    fn paladin_rogue_multiclass_both_directions() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let base_input = result.character_input.expect("valid fixture");

        let mut valid_input = base_input.clone();
        valid_input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: PALADIN_CLASS_ID.to_owned(), level: 4 },
            CharacterClassLevel { class_id: ROGUE_CLASS_ID.to_owned(), level: 1 },
        ];
        let valid_receipt = build_pilot_headless_receipt(&valid_input);
        assert_eq!(
            valid_receipt.status,
            HeadlessReceiptStatus::Computed,
            "Paladin+Rogue multiclass with a valid spell posture must reach Computed: {:?}",
            valid_receipt.computation.diagnostics
        );

        let mut invalid_input = valid_input;
        invalid_input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: PALADIN_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });
        let invalid_receipt = build_pilot_headless_receipt(&invalid_input);
        assert_eq!(
            invalid_receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Paladin+Rogue multiclass must still block a genuine posture violation: {:?}",
            invalid_receipt.computation.diagnostics
        );
    }

    /// A single-class Paladin at level 4 (base spells-per-day for 1st-level
    /// spells is a genuine 0 -- the fixture's Charisma 8 grants no bonus
    /// spells at all) preparing any 1st-level spell over-prepares its slot
    /// budget (0 total) and must stay `Blocked`.
    #[test]
    fn single_class_paladin_over_prepared_slot_budget_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: PALADIN_CLASS_ID.to_owned(), level: 4 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Bless".to_owned(),
            source_class_id: PALADIN_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "paladin level 4 has a genuine 0 base slots for 1st-level spells, and the \
             fixture's Charisma 8 (mod -1) grants no bonus, so preparing any 1st-level spell \
             over-prepares: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.paladin.partial_caster.unsupported"
                    && d.claim_blocking),
            "expected the real spell-posture diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }
}

/// `decisions.md §22`'s "FURTHER UPDATE, 2026-09-04": Paladin's Detect Evil
/// and Cleric's Aura, both genuinely new compute (no explanation id existed
/// anywhere in the engine before this cycle), both pure class-level
/// pass-throughs following the exact structural precedent already built for
/// the Antipaladin's own mirror features (`aura_of_evil_strength_level` /
/// `detect_good_caster_level`, `rules_tables::apg::antipaladin_features`).
/// These tests prove both formulas directly AND prove each explanation id
/// is actually reachable through the real pipeline end to end -- the same
/// "unit test the formula, then also prove reachability" discipline wave
/// 41's own three corrected units were closed under, since a formula this
/// codebase has repeatedly gotten right in isolation has ALSO repeatedly
/// turned out unreachable in practice (`decisions.md §22`'s own "cheap
/// fix... mischaracterized" finding).
#[cfg(test)]
mod paladin_detect_evil_and_cleric_aura_tests {
    use super::{
        build_pilot_headless_receipt, cleric_aura_strength_level, paladin_detect_evil_caster_level,
        CharacterClassLevel, CharacterInput,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn character(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture must load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn explanation_value(input: &CharacterInput, id: &str) -> Option<i16> {
        build_pilot_headless_receipt(input)
            .computation
            .explanations
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.value)
    }

    #[test]
    fn paladin_detect_evil_caster_level_is_the_raw_class_level_from_level_one() {
        assert_eq!(paladin_detect_evil_caster_level(1), Some(1));
        assert_eq!(paladin_detect_evil_caster_level(20), Some(20));
        assert_eq!(paladin_detect_evil_caster_level(0), None);
    }

    #[test]
    fn cleric_aura_strength_level_is_the_raw_class_level_from_level_one() {
        assert_eq!(cleric_aura_strength_level(1), Some(1));
        assert_eq!(cleric_aura_strength_level(11), Some(11));
        assert_eq!(cleric_aura_strength_level(0), None);
    }

    /// Reachability proof: a level-1 Paladin's Detect Evil caster level
    /// must actually appear in a real receipt's explanations, under the
    /// exact id `core_rulebook:class_feature:paladin_detect_evil`'s own
    /// classifier slug expects (`class_feature.paladin.detect_evil.
    /// caster_level`).
    #[test]
    fn paladin_detect_evil_reaches_the_real_pipeline_from_level_one() {
        let level1 = character("class:paladin", 1);
        assert_eq!(
            explanation_value(&level1, "class_feature.paladin.detect_evil.caster_level"),
            Some(1),
            "a level-1 Paladin must have Detect Evil grounded at caster level 1"
        );

        let level5 = character("class:paladin", 5);
        assert_eq!(
            explanation_value(&level5, "class_feature.paladin.detect_evil.caster_level"),
            Some(5),
            "Detect Evil's caster level must track paladin level directly"
        );
    }

    /// Reachability proof: a level-1 Cleric's Aura strength level must
    /// actually appear in a real receipt's explanations, under the exact
    /// id `core_rulebook:class_feature:cleric_aura`'s own classifier slug
    /// expects (`class_feature.cleric.aura.strength_level`).
    #[test]
    fn cleric_aura_reaches_the_real_pipeline_from_level_one() {
        let level1 = character("class:cleric", 1);
        assert_eq!(
            explanation_value(&level1, "class_feature.cleric.aura.strength_level"),
            Some(1),
            "a level-1 Cleric must have Aura grounded at strength level 1"
        );

        let level11 = character("class:cleric", 11);
        assert_eq!(
            explanation_value(&level11, "class_feature.cleric.aura.strength_level"),
            Some(11),
            "Aura's strength level must track cleric level directly (overwhelming tier at 11+)"
        );
    }

    /// Neither record may leak onto the other class, nor onto an unrelated
    /// class entirely.
    #[test]
    fn neither_record_leaks_onto_an_unrelated_class() {
        let fighter = character("class:fighter", 5);
        assert_eq!(
            explanation_value(&fighter, "class_feature.paladin.detect_evil.caster_level"),
            None,
            "a Fighter must not gain Paladin's Detect Evil"
        );
        assert_eq!(
            explanation_value(&fighter, "class_feature.cleric.aura.strength_level"),
            None,
            "a Fighter must not gain Cleric's Aura"
        );

        let paladin = character("class:paladin", 5);
        assert_eq!(
            explanation_value(&paladin, "class_feature.cleric.aura.strength_level"),
            None,
            "a Paladin must not gain Cleric's own Aura record"
        );

        let cleric = character("class:cleric", 5);
        assert_eq!(
            explanation_value(&cleric, "class_feature.paladin.detect_evil.caster_level"),
            None,
            "a Cleric must not gain Paladin's own Detect Evil record"
        );
    }
}

