#[allow(unused_imports)]
pub(crate) use super::*;

// Grounded SD13-E4/E5 Human Cleric level-1/level-2/level-3/level-4 prepared divine
// spell-bearing baseline identity. Cleric is the canonical PF1 prepared divine full
// caster; unlike the arcane Sorcerer/Wizard/Bard baselines already recognized, its
// bounded burden is split across a domain powers class-feature family (the granted
// powers of the chosen domains and the domain spell-list contents — Channel Energy
// has been grounded for real: ceil(cleric level / 2) d6, minimum 1d6, usable
// 3 + Charisma modifier times per day; and the SD13-E5 domain slice grounds the
// domain choice seam and the flat domain spell slot count) and a prepared divine
// spell posture family (spells prepared from the full Cleric list, spontaneous
// cure/inflict conversion, spell slots per day, bonus spells from a high Wisdom,
// spell save DCs). A later SD13-E5 slice widens the level-1-only gate to a
// level-range gate (level 1-2), extending base attack/base save/Channel
// Energy/domain-spell-slot/domain-power formulas to level 2 without re-derivation. A
// further SD13-E5 slice widens the gate again to level 1-3: Channel Energy's die
// count and the domain spell slot count both change for real at level 3 (verified
// independently against the PF1 Core Rulebook Cleric class table and spells-per-day
// table), since level 3 is exactly when a cleric first casts 2nd-level spells. A
// further SD13-E5 slice widens the gate again to level 1-4: the Good domain's Touch
// of Good sacred bonus genuinely changes for real at level 4 (half cleric level,
// minimum 1, so `max(4/2, 1) = 2`, up from 1), verified independently against the PF1
// Core Rulebook Good Domain granted-power rule text; Channel Energy's die count and
// the domain spell slot count both stay unchanged at level 4 (verified independently
// against the class table's blank level-4 "Special" column and the spells-per-day
// table's still-blank 3rd-level spell column at level 4).
pub(super) const CLERIC_CLASS_ID: &str = "class:cleric";

/// SD13-E5 Cleric level-range gate, mirroring the Fighter `supported_fighter_level` /
/// Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` idiom. Verified against
/// the PF1 Core Rulebook Cleric class table (d20pfsrd and legacy.aonprd.com) before
/// widening: a level-2 cleric still only casts 1st-level cleric spells (2nd-level
/// cleric spells begin at caster level 3), gains no new class feature at 2nd level
/// (the Cleric class table's level-2 "Special" column is blank), and Channel Energy
/// stays 1d6 through level 2 (it next increases at level 3), so every level-1 formula
/// this seam already grounds extends to level 2 without re-derivation. A further
/// SD13-E5 slice widens this to 1..=3: a level-3 cleric's Channel Energy die count
/// becomes 2d6 (`ceil(3 / 2) = 2`, the class table's level-3 "Special" column reads
/// "Channel energy 2d6") and a level-3 cleric casts 2nd-level cleric spells for the
/// first time (verified against the raw Cleric spells-per-day table rows), so the
/// domain spell slot count also changes for real at level 3. A further SD13-E5 slice
/// widens this to 1..=4, verified independently against both primary sources: the
/// class table's level-4 "Special" column is blank (no new class feature is gained),
/// Channel Energy's die count stays 2d6 (`ceil(4 / 2) = 2`, unchanged from level 3, it
/// next increases only at level 5), and the domain spell slot count stays 2 (a
/// level-4 cleric's 3rd-level spell column is still "—" on the raw spells-per-day
/// table — 3rd-level cleric spells begin only at level 5) — but the Good domain's
/// Touch of Good sacred bonus (half cleric level, minimum 1) genuinely increases to 2
/// via the same pre-existing formula (`max(4/2, 1) = 2`). A further SD13-E5 slice
/// widens this to 1..=5, verified independently against both primary sources: the
/// class table's level-5 "Special" column reads "Channel energy 3d6" — Channel
/// Energy's die count genuinely increases to 3d6 (`ceil(5 / 2) = 3`) — and a
/// level-5 cleric casts 3rd-level cleric spells for the first time (the raw
/// spells-per-day table's level-5 row is the first to show a non-"—" 3rd-level
/// column, "1+1"), so the domain spell slot count also changes for real, to 3, at
/// level 5. The Good domain's Touch of Good sacred bonus stays 2 at level 5
/// (`max(5/2, 1) = 2`, integer division; it next increases only at level 6). A
/// further SD13-E5 slice widens this to 1..=6, verified independently against both
/// primary sources: the class table's level-6 "Special" column is genuinely blank
/// (no new class feature is gained at 6th level), Channel Energy's die count stays
/// 3d6 (`ceil(6 / 2) = 3`, unchanged from level 5 — both primary sources confirm
/// the die count rises only every odd cleric level, 1st/3rd/5th/7th/..., so level 6
/// is not one of those levels), and the domain spell slot count stays 3 (the raw
/// spells-per-day table's level-6 row still shows "—" in the 4th-level spell
/// column, so 4th-level cleric spells do not begin at level 6) — but the Good
/// domain's Touch of Good sacred bonus genuinely increases to 3 via the same
/// pre-existing formula (`max(6/2, 1) = 3`). A further SD13-E5 slice widens this to
/// 1..=7, verified independently against both primary sources: the class table's
/// level-7 "Special" column reads "Channel energy 4d6" — Channel Energy's die count
/// genuinely increases to 4d6 (`ceil(7 / 2) = 4`), confirming level 7 IS one of the
/// odd cleric levels where the die count rises — and the domain spell slot count
/// also genuinely increases, to 4 (a level-7 cleric casts 4th-level cleric spells
/// for the first time, the raw spells-per-day table's level-7 row being the first to
/// show a non-"—" 4th-level spell column), mirroring exactly the level-3 and
/// level-5 domain-spell-slot widenings. The Good domain's Touch of Good sacred
/// bonus stays 3 at level 7 (`max(7/2, 1) = 3`, integer division; it next increases
/// only at level 8). No other new class feature is gained at 7th level (verified
/// independently against both primary sources' level-7 Special column), so no new
/// pillar record is added at level 7 either — only the Channel Energy and domain
/// spell slot count pillars are widened to genuinely new values. A further SD13-E5
/// slice widens this to 1..=8, verified independently against both primary sources:
/// the class table's level-8 "Special" column is genuinely blank (no new class
/// feature is gained at 8th level — the iterative-attack notation "+6/+1" on the
/// level-8 base-attack column is not modeled anywhere in this codebase, only the
/// flat base value of 6), Channel Energy's die count stays 4d6
/// (`ceil(8 / 2) = 4`, unchanged from level 7 — both primary sources confirm the
/// die count rises only every odd cleric level, 1st/3rd/5th/7th/9th/..., so level 8
/// is not one of those levels), and the domain spell slot count stays 4 (the raw
/// spells-per-day table's level-8 row still shows "—" in the 5th-level spell
/// column, verified independently against both primary sources — 5th-level cleric
/// spells do not begin until level 9) — but the Good domain's Touch of Good sacred
/// bonus GENUINELY increases to 4 via the same pre-existing formula
/// (`max(8/2, 1) = 4`), confirming the level-7 comment's own forecast that it next
/// increases at level 8.
// A further SD13-E5 slice widens the gate to level 9 (verified independently
// against d20pfsrd and legacy.aonprd.com): level 9 base attack stays +6
// (9 * 3 / 4) and good Fortitude/Will both stay +6 (9 / 2 + 2),
// integer-division coincidences, while poor Reflex genuinely rises to +3
// (9 / 3); the level-9 "Special" column reads "Channel energy 5d6" — a
// tier-rise on the already-grounded die-count pillar ((level + 1) / 2 = 5,
// the odd-level cadence), not a new class feature; 5th-level cleric spells
// first appear at 9th, so the domain spell slot count genuinely rises to 5
// via the same one-slot-per-castable-spell-level rule
// (CLERIC_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL); Touch of Good's bonus
// stays 4 (9 / 2, a coincidence) and both domain-power uses-per-day pools
// stay level-independent; no new pillar is grounded. A further SD13-E5 slice
// widens the gate to level 10 — the tranche ceiling (verified independently
// against d20pfsrd and legacy.aonprd.com): level 10 base attack genuinely
// rises to +7 (10 * 3 / 4) and both good saves genuinely rise to +7
// (10 / 2 + 2), while poor Reflex stays +3 (10 / 3, a coincidence); the
// level-10 "Special" column is genuinely blank (the die-count rises land at
// odd levels) so Channel Energy stays 5d6 ((10 + 1) / 2, next rise 11th);
// the domain spell slot count stays 5 (6th-level cleric spells first appear
// at 11th, the level-10 spells-per-day row's 6th-level column still "—");
// Touch of Good's bonus genuinely rises to 5 (10 / 2) via the same
// half-cleric-level formula; both domain-power uses-per-day pools stay
// level-independent; no new pillar is grounded. An SD18 slice
// (`cycle-2026-07-15T9600`, mirroring the Ranger/Bard/Rogue/Fighter/Wizard
// level-17 widenings — the loop's SIXTH §3.2 level-17 landing, and the
// second full 9-level-caster class after Wizard to reach it) widens the
// gate again to 1..=17 (`MAX_SUPPORTED_CLERIC_LEVEL = 17`): the class
// table's level-17 "Special" column reads "Channel energy 9d6" (verified
// independently against two primary sources — d20pfsrd and the Archives of
// Nethys aonprd.com mirror, byte-for-byte agreement, so a third source was
// not required) — Channel Energy's die count genuinely rises to 9d6
// (`(17 + 1) / 2 = 9`, up from 8d6 at level 16) via the same pre-existing
// formula, not re-derived — and the domain spell slot count also genuinely
// rises, to 9 (a level-17 cleric casts 9th-level cleric spells for the
// first time, verified independently against both primary sources' raw
// spells-per-day table rows, mirroring the Wizard level-17 cycle's own
// 9th-level spell column opening), while base attack bonus stays +12
// (`17 * 3 / 4 = 12`), base Fortitude/Reflex/Will saves all stay
// numerically unchanged from level 16, and Touch of Good's bonus stays 8
// (`17 / 2 = 8`) — integer-division coincidences, checked not assumed — so
// two pillars whose underlying formulas genuinely change (Channel Energy
// dice, domain spell slot count) are widened; no new pillar record is added
// at level 17 either, since "Channel energy 9d6" names only a tier-rise on
// the already-grounded Channel Energy dice pillar, not a new class feature.
// A further SD18 slice (`cycle-2026-07-15T14300`, mirroring
// `a3762ca`'s Wizard level-18 widening — the loop's SECOND §3.2 level-18
// landing, and the first full 9-level-caster class after Wizard to reach it)
// widens the gate again to 1..=18 (`MAX_SUPPORTED_CLERIC_LEVEL = 18`): the
// class table's level-18 "Special" column is genuinely BLANK (verified
// independently against two primary sources — d20pfsrd and the Archives of
// Nethys aonprd.com mirror, byte-for-byte agreement on the full
// levels-16-through-19 block, so a third source was not required) — a pure
// ceiling raise, exactly mirroring the Wizard level-18 cycle's own pure
// ceiling raise: base attack bonus genuinely rises to +13 (`18 * 3 / 4 =
// 13`), both good saves genuinely rise to +11 (`18 / 2 + 2 = 11`), poor
// Reflex genuinely rises to +6 (`18 / 3 = 6`), and Touch of Good's bonus
// genuinely rises to 9 (`18 / 2 = 9`), all via the same pre-existing
// formulas, not re-derived, while Channel Energy's die count stays 9d6
// (`(18 + 1) / 2 = 9`, the odd-level cadence's next rise landing at 19th)
// and the domain spell slot count stays 9 (a level-18 cleric still casts
// only up to 9th-level cleric spells, the highest cleric spell level in
// PF1 — the top `domain_spell_slot_count` arm, gated on
// `level >= CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL`, already covers
// level 18 with zero code change, mirroring exactly how the Wizard
// level-18 cycle's specialist-bonus-slot top arm already covered level 18)
// — so no new pillar record is added at level 18 either, since no class
// feature is named in the level-18 Special column. A further SD18 slice
// (`cycle-2026-07-16T1100`, the loop's SECOND §3.2 level-19 landing, after
// Barbarian) widens the gate again to 1..=19 (`MAX_SUPPORTED_CLERIC_LEVEL =
// 19`): the class table's level-19 "Special" column reads "Channel energy
// 10d6" (verified independently against two primary sources — a raw curl
// fetch of d20pfsrd and the Archives of Nethys aonprd.com mirror, both
// covering the full levels-16-through-20 block, byte-for-byte agreement, so
// a third source was not required) — Channel Energy's die count genuinely
// rises to 10d6 (`(19 + 1) / 2 = 10`, up from 9d6 at level 18) via the same
// pre-existing formula, not re-derived, naming only a tier-rise on the
// already-grounded Channel Energy dice pillar, not a new class feature,
// exactly mirroring the level-17 cycle's own "Channel energy 9d6" finding —
// so no new pillar record is added at level 19 either. Base attack bonus
// genuinely rises to +14 (`19 * 3 / 4 = 14`), while both good saves stay put
// at +11 (`19 / 2 + 2 = 11`) and poor Reflex stays put at +6 (`19 / 3 = 6`),
// integer-division coincidences with level 18, checked not assumed. The
// domain spell slot count stays 9 (still only 9th-level cleric spells, the
// PF1 ceiling) and Touch of Good's bonus stays 9 (`19 / 2 = 9`, also an
// integer-division coincidence with level 18) — all via the same
// pre-existing formulas, not re-derived. An SD18 slice
// (`cycle-2026-07-16T0844`, the loop's FIRST §3.2 level-20 landing,
// opening the level-20 sweep — the final remaining level within PF1's
// 1-20 character-level cap) widens the gate again to 1..=20
// (`MAX_SUPPORTED_CLERIC_LEVEL = 20`): the class table's level-20
// "Special" column is genuinely BLANK (verified independently against
// two primary sources — d20pfsrd and the Archives of Nethys aonprd.com
// mirror, byte-for-byte agreement on the full levels-16-through-20
// block, so a third source was not required) — Cleric has no named
// capstone class feature at 20th level at all, unlike Barbarian's Mighty
// Rage, Fighter's Weapon Mastery, Rogue's Master Strike, Paladin's Holy
// Champion, or Ranger's Master Hunter — a pure ceiling raise, exactly
// mirroring the level-16 and level-18 cycles' own pure ceiling raises:
// base attack bonus genuinely rises to +15 (`20 * 3 / 4 = 15`), both
// good saves genuinely rise to +12 (`20 / 2 + 2 = 12`), and Touch of
// Good's bonus genuinely rises to 10 (`20 / 2 = 10`), all via the same
// pre-existing formulas, not re-derived, while poor Reflex stays +6
// (`20 / 3 = 6`), Channel Energy's die count stays 10d6
// (`(20 + 1) / 2 = 10`, the odd-level cadence's last rise having landed
// at level 19; PF1 character levels do not go past 20, so no further
// rise is possible), and the domain spell slot count stays 9 (a
// level-20 cleric still casts only up to 9th-level cleric spells, the
// highest cleric spell level in PF1 — the pre-existing top
// domain-spell-slot-count arm already covers level 20 with zero code
// change) — so no new pillar is grounded at level 20 either, only the
// base-attack, base-save, and Touch of Good pillars are widened. This is
// the final level within PF1's 1-20 character-level cap for this class
// row.
pub(super) const MAX_SUPPORTED_CLERIC_LEVEL: u8 = 20;

// SD13-E5 canonical Human Cleric domain-choice seam. These name the exact accepted
// deterministic domain selections on the level-1/level-2/level-3 seam (a cleric
// chooses two domains from among those belonging to her deity). This slice surfaces
// the named selections as an explicit choice seam only and grounds no domain power
// and no domain spell-list contents, mirroring the Fighter bonus-feat choice-slot
// seam pattern.
pub(super) const CLERIC_DOMAIN_CHOICE_ID: &str = "choice:cleric_domain";

// PF1 Core Rulebook Domains: a cleric gains one domain spell slot per level of
// cleric spells she can cast, 1st and up. At levels 1-2 this bounded seam supports
// she casts only 1st-level cleric spells (2nd-level cleric spells begin at caster
// level 3, verified against the PF1 Core Rulebook Cleric spells-per-day table via
// d20pfsrd and legacy.aonprd.com), so exactly one 1st-level domain slot is granted —
// confirmed unchanged at level 2, not a new record. At level 3 a cleric casts
// 2nd-level cleric spells for the first time (the raw spells-per-day table's level-3
// row is the first to show a non-"—" 2nd-level column), so the domain spell slot
// count genuinely becomes 2 at level 3: one 1st-level domain slot plus one
// 2nd-level domain slot, mirroring exactly the Wizard specialist-bonus-slot
// level-3 widening (`WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_3`). At level 5 a
// cleric casts 3rd-level cleric spells for the first time (the raw
// spells-per-day table's level-5 row is the first to show a non-"—" 3rd-level
// column), so the count genuinely becomes 3: one 1st-level, one 2nd-level, and
// one 3rd-level domain slot. Confirmed unchanged at level 6 (the raw
// spells-per-day table's level-6 row still shows "—" in the 4th-level spell
// column, verified independently against both primary sources), so the count
// stays 3 through level 6 — it next changes only when 4th-level cleric spells
// become available at a later level. At level 7 a cleric casts 4th-level
// cleric spells for the first time (the raw spells-per-day table's level-7 row
// is the first to show a non-"—" 4th-level column, verified independently
// against both primary sources), so the count genuinely becomes 4: one
// 1st-level, one 2nd-level, one 3rd-level, and one 4th-level domain slot.
pub(super) const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_1_AND_2: i16 = 1;

pub(super) const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_3_AND_4: i16 = 2;

pub(super) const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_5_AND_6: i16 = 3;

pub(super) const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_7: i16 = 4;

pub(super) const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_9: i16 = 5;

pub(super) const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_11: i16 = 6;

pub(super) const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_13: i16 = 7;

pub(super) const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_15: i16 = 8;

pub(super) const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_17: i16 = 9;

/// The cleric level at which 2nd-level cleric spells (and so the second domain
/// spell slot) first become available, verified against the raw PF1 Core Rulebook
/// Cleric spells-per-day table rows (d20pfsrd and legacy.aonprd.com): level 2 shows
/// "4/2+1/—", level 3 shows "4/2+1/1+1" — the first non-"—" 2nd-level column.
pub(super) const CLERIC_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 3;

/// The cleric level at which 3rd-level cleric spells (and so the third domain
/// spell slot) first become available, verified against the raw PF1 Core Rulebook
/// Cleric spells-per-day table rows (d20pfsrd and legacy.aonprd.com): level 4 shows
/// "5/3+1/2+1/—", level 5 shows "5/3+1/2+1/1+1" — the first non-"—" 3rd-level
/// column. Confirmed the count stays at 3 domain slots through level 6 (the
/// level-6 row's 4th-level spell column is still "—"), since 4th-level cleric
/// spells are not yet available.
pub(super) const CLERIC_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 5;

/// The cleric level at which 4th-level cleric spells (and so the fourth domain
/// spell slot) first become available, verified against the raw PF1 Core
/// Rulebook Cleric spells-per-day table rows (d20pfsrd and legacy.aonprd.com):
/// level 6 shows a still-"—" 4th-level column, level 7 is the first to show a
/// non-"—" 4th-level column ("1+1"). Confirmed the count stays at 4 domain
/// slots through level 8 (the level-8 row's 5th-level spell column is still
/// "—", verified independently against both primary sources), since 5th-level
/// cleric spells are not yet available — they first appear at level 9.
pub(super) const CLERIC_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 7;

/// The cleric level at which 5th-level cleric spells (and so the fifth domain
/// spell slot) first become available, verified against the raw PF1 Core
/// Rulebook Cleric spells-per-day table rows (d20pfsrd and legacy.aonprd.com):
/// level 8 shows a still-"—" 5th-level column, level 9 is the first to show a
/// non-"—" 5th-level column ("1+1", the level-9 row reading
/// "4/4+1/4+1/3+1/2+1/1+1").
pub(super) const CLERIC_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 9;

/// The cleric level at which 6th-level cleric spells (and so the sixth domain
/// spell slot) first become available, verified against the raw PF1 Core
/// Rulebook Cleric spells-per-day table rows (d20pfsrd and legacy.aonprd.com):
/// level 10 shows a still-"—" 6th-level column, level 11 is the first to show
/// a non-"—" 6th-level column ("1+1"). This is also the cleric level at which
/// Channel Energy's die count rises again (the class table's level-11
/// "Special" column reads "Channel energy 6d6"), via the same pre-existing
/// `(level + 1) / 2` formula, not re-derived.
pub(super) const CLERIC_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 11;

/// The cleric level at which 7th-level cleric spells (and so the seventh domain
/// spell slot) first become available, verified against the raw PF1 Core
/// Rulebook Cleric spells-per-day table rows across three independent sources
/// (d20pfsrd, Archives of Nethys aonprd.com, and legacy.aonprd.com, all three
/// byte-for-byte identical): level 12 shows a still-"—" 7th-level column,
/// level 13 is the first to show a non-"—" 7th-level column ("1+1"). This is
/// also the cleric level at which Channel Energy's die count rises again (the
/// class table's level-13 "Special" column reads "Channel energy 7d6"), via
/// the same pre-existing `(level + 1) / 2` formula, not re-derived.
pub(super) const CLERIC_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 13;

/// The cleric level at which 8th-level cleric spells (and so the eighth domain
/// spell slot) first become available, verified against the raw PF1 Core
/// Rulebook Cleric spells-per-day table rows: legacy.aonprd.com's full
/// multi-row table extraction (levels 14-17 side by side) shows the 8th-level
/// column as still "—" at level 14 and first non-"—" ("1+1") at level 15,
/// internally consistent with the established every-other-odd-level cadence
/// already grounded for the 2nd through 7th spell-level thresholds (3, 5, 7,
/// 9, 11, 13). Two single-row summarized fetches (one from d20pfsrd, one from
/// Archives of Nethys aonprd.com) disagreed with each other (claiming level
/// 17 and level 16 respectively) in a way that broke that established
/// cadence and were rejected as tool artifacts rather than treated as
/// genuine source conflicts. This is also the cleric level at which Channel
/// Energy's die count rises again (the class table's level-15 "Special"
/// column reads "Channel energy 8d6"), via the same pre-existing
/// `(level + 1) / 2` formula, not re-derived.
pub(super) const CLERIC_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 15;

/// The cleric level at which 9th-level cleric spells (and so the ninth domain
/// spell slot) first become available, verified independently against two
/// primary sources (d20pfsrd and the Archives of Nethys aonprd.com mirror,
/// byte-for-byte agreement): level 16 shows a still-"—" 9th-level column,
/// level 17 is the first to show a non-"—" 9th-level column ("1+1"),
/// mirroring the Wizard level-17 cycle's own 9th-level spell column opening
/// (`WIZARD_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 17`). This is also the
/// cleric level at which Channel Energy's die count rises again (the class
/// table's level-17 "Special" column reads "Channel energy 9d6"), via the
/// same pre-existing `(level + 1) / 2` formula, not re-derived.
pub(super) const CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 17;

/// The bounded Cleric milestone level this decomposition surface grounds, if any.
/// Returns the single Cleric level when the chosen input is exactly a single-class
/// Cleric at one of the supported milestone levels (1 through 10). Returns `None` for
/// no Cleric, a non-Cleric class, a multiclass mix, or any level-11+ Cleric this slice
/// deliberately does not recognize — each of which stays claim-blocked exactly as
/// before. Mirrors the Fighter `supported_fighter_level` / Paladin
/// `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` level-range gate idiom.
pub(super) fn supported_cleric_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == CLERIC_CLASS_ID
                && (1..=MAX_SUPPORTED_CLERIC_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E4/E5 runtime evidence for the deterministic Human Cleric
/// level-1/level-2/level-3/level-4 prepared divine spell-bearing baseline, while
/// keeping it explicitly claim-blocked on its remaining still-missing burdens.
///
/// This deliberately does not compute a supported spell surface. It grounds Channel
/// Energy's flat die-count and uses-per-day math, the domain choice seam, the flat
/// domain spell slot count, the Good domain's Touch of Good (flat sacred-bonus
/// magnitude and flat uses-per-day count), the Healing domain's Rebuke Death (flat
/// uses-per-day count only), and the foundational base-attack-bonus / base-save
/// progression pillar that every other class row in this matrix already has and Cleric
/// never had; it grounds no Rebuke Death heal amount, no domain spell-list contents, no
/// channel energy save DC or damage/healing resolution, no spellbook posture, no spells
/// prepared, no spontaneous cure/inflict conversion, no general spell slots per day, no
/// spell save DCs, and no bonus spell slots from a high Wisdom. A later SD13-E5 slice
/// widens the level-1-only gate (`supported_cleric_level`, 1..=2) and extends every one
/// of the formulas below to level 2 via the same formula, without re-derivation,
/// verified independently against the PF1 Core Rulebook Cleric class table (d20pfsrd
/// and legacy.aonprd.com): Cleric gains no new class feature at 2nd level (the class
/// table's level-2 "Special" column is blank), so no new pillar is added, only the
/// existing ones widened. A further SD13-E5 slice widens the gate again to 1..=3
/// (`MAX_SUPPORTED_CLERIC_LEVEL = 3`): Channel Energy's die count and the domain
/// spell slot count both change for real at level 3, since level 3 is exactly when
/// a cleric first casts 2nd-level spells (verified independently against both
/// primary sources' raw class table and spells-per-day table rows); the level-3
/// "Special" column names only the Channel Energy die-count increase, so no new
/// pillar record is added. A further SD13-E5 slice widens the gate again to 1..=4
/// (`MAX_SUPPORTED_CLERIC_LEVEL = 4`): the Good domain's Touch of Good sacred bonus
/// changes for real at level 4 (half cleric level, minimum 1: `max(4/2, 1) = 2`, up
/// from 1), verified independently against the PF1 Core Rulebook Good Domain
/// granted-power rule text; Channel Energy's die count and the domain spell slot
/// count both stay unchanged at level 4 (verified independently against both primary
/// sources: the class table's level-4 "Special" column is blank, and the
/// spells-per-day table's 3rd-level spell column is still "—" at level 4), so no new
/// pillar record is added at level 4 either. A further SD13-E5 slice widens the gate
/// again to 1..=5 (`MAX_SUPPORTED_CLERIC_LEVEL = 5`): Channel Energy's die count
/// genuinely increases to 3d6 (`ceil(5/2) = 3`, the class table's level-5 "Special"
/// column reads "Channel energy 3d6") and the domain spell slot count genuinely
/// increases to 3 (a level-5 cleric casts 3rd-level cleric spells for the first
/// time, verified independently against both primary sources' raw spells-per-day
/// table rows), while the Good domain's Touch of Good sacred bonus stays 2
/// (`max(5/2, 1) = 2`, unchanged from level 4 — it next increases only at level 6),
/// so only the two pillars whose underlying formulas genuinely change are widened;
/// no new pillar record is added at level 5 either. A further SD13-E5 slice widens
/// the gate again to 1..=6 (`MAX_SUPPORTED_CLERIC_LEVEL = 6`): the class table's
/// level-6 "Special" column is genuinely blank (no new class feature is gained),
/// Channel Energy's die count stays 3d6 (`ceil(6/2) = 3`, unchanged from level 5 —
/// both primary sources confirm the die count rises only every odd cleric level),
/// and the domain spell slot count stays 3 (the spells-per-day table's level-6 row
/// still shows "—" in the 4th-level spell column), while the Good domain's Touch of
/// Good sacred bonus genuinely increases to 3 (`max(6/2, 1) = 3`, up from 2) — so
/// only the one pillar whose underlying formula genuinely changes is widened; no
/// new pillar record is added at level 6 either. A further SD13-E5 slice widens
/// the gate again to 1..=7 (`MAX_SUPPORTED_CLERIC_LEVEL = 7`): the class table's
/// level-7 "Special" column reads "Channel energy 4d6" — Channel Energy's die
/// count genuinely increases to 4d6 (`ceil(7/2) = 4`, confirming level 7 IS one of
/// the odd cleric levels where the die count rises) — and the domain spell slot
/// count also genuinely increases, to 4 (a level-7 cleric casts 4th-level cleric
/// spells for the first time, verified independently against both primary
/// sources' raw spells-per-day table rows), while the Good domain's Touch of Good
/// sacred bonus stays 3 (`max(7/2, 1) = 3`, unchanged from level 6 — it next
/// increases only at level 8) — so two pillars whose underlying formulas
/// genuinely change (Channel Energy dice, domain spell slot count) are widened;
/// no new pillar record is added at level 7 either, since no other class feature
/// is named in the level-7 Special column. An SD18 slice
/// (`cycle-2026-07-13T2007`, mirroring `cycle-2026-07-13T1255`'s Barbarian
/// level-11 widening and `cycle-2026-07-13T1830`'s Bard level-11 widening)
/// widens the gate again to 1..=11 (`MAX_SUPPORTED_CLERIC_LEVEL = 11`,
/// generalized from the SD13-E5 1..=10 ceiling): the class table's level-11
/// "Special" column reads "Channel energy 6d6" (verified independently
/// against d20pfsrd and legacy.aonprd.com) — Channel Energy's die count
/// genuinely rises to 6d6 (`(11 + 1) / 2 = 6`, up from 5d6 at level 10) via
/// the same pre-existing formula, not re-derived — and the domain spell slot
/// count also genuinely rises, to 6 (a level-11 cleric casts 6th-level cleric
/// spells for the first time, verified independently against both primary
/// sources' raw spells-per-day table rows), while base attack bonus rises to
/// +8 (`11 * 3 / 4 = 8`) and base Fortitude/Reflex/Will saves and Touch of
/// Good's sacred bonus all stay numerically unchanged from level 10
/// (integer-division coincidences, checked not assumed) — so two pillars
/// whose underlying formulas genuinely change (Channel Energy dice, domain
/// spell slot count) plus the base-attack-bonus arithmetic extension are
/// widened; no new pillar record is added at level 11 either, since no other
/// class feature is named in the level-11 Special column. A further SD18
/// slice (`cycle-2026-07-15T0200`, mirroring `cycle-2026-07-14T1814`'s
/// Barbarian level-12 widening and `cycle-2026-07-14T2359`'s Bard level-12
/// widening) widens the gate again to 1..=12 (`MAX_SUPPORTED_CLERIC_LEVEL =
/// 12`): the class table's level-12 "Special" column is genuinely BLANK
/// (verified independently against d20pfsrd and Archives of Nethys
/// aonprd.com) — base attack bonus rises to +9 (`12 * 3 / 4 = 9`), base
/// Fortitude/Will (good saves) rise to +8 (`12 / 2 + 2 = 8`), base Reflex
/// (poor save) rises to +4 (`12 / 3 = 4`), and the Good domain's Touch of
/// Good sacred bonus rises to 6 (`12 / 2 = 6`) via the same pre-existing
/// formulas, not re-derived, while Channel Energy's die count stays 6d6
/// (`(12 + 1) / 2 = 6`, the odd-level cadence's next rise landing at 13th)
/// and the domain spell slot count stays 6 (a level-12 cleric still casts
/// only up to 6th-level cleric spells; 7th-level cleric spells first appear
/// at level 13) — so four pillars whose underlying formulas genuinely
/// change (base attack, all three base saves via two distinct formulas,
/// Touch of Good's bonus) are widened; no new pillar record is added at
/// level 12 either, since no other class feature is named in the level-12
/// Special column. An SD18 slice (`cycle-2026-07-15T1500`, mirroring
/// `cycle-2026-07-15T1100`'s Rogue, `cycle-2026-07-15T1200`'s Barbarian,
/// `cycle-2026-07-15T1300`'s Fighter, and `cycle-2026-07-15T1400`'s Ranger
/// level-13 widenings) widens the gate again to 1..=13
/// (`MAX_SUPPORTED_CLERIC_LEVEL = 13`): the class table's level-13 "Special"
/// column reads "Channel energy 7d6" (verified independently against three
/// primary sources — d20pfsrd, Archives of Nethys aonprd.com, and
/// legacy.aonprd.com, all three byte-for-byte identical) — Channel Energy's
/// die count genuinely rises to 7d6 (`(13 + 1) / 2 = 7`, up from 6d6 at
/// level 12) via the same pre-existing formula, not re-derived — and the
/// domain spell slot count also genuinely rises, to 7 (a level-13 cleric
/// casts 7th-level cleric spells for the first time, verified independently
/// against all three primary sources' raw spells-per-day table rows), while
/// base attack bonus stays +9 (`13 * 3 / 4 = 9`), base Fortitude/Reflex/Will
/// saves all stay numerically unchanged from level 12, and Touch of Good's
/// bonus stays 6 (`13 / 2 = 6`) (integer-division coincidences, checked not
/// assumed) — so two pillars whose underlying formulas genuinely change
/// (Channel Energy dice, domain spell slot count) are widened; no new
/// pillar record is added at level 13 either, since no other class feature
/// is named in the level-13 Special column. An SD18 slice
/// (`cycle-2026-07-15T2300`, mirroring `cycle-2026-07-15T1900`'s Barbarian,
/// `cycle-2026-07-15T2000`'s Fighter and Rogue, and `cycle-2026-07-15T2100`'s
/// Ranger level-14 widenings) widens the gate again to 1..=14
/// (`MAX_SUPPORTED_CLERIC_LEVEL = 14`): the class table's level-14 "Special"
/// column is genuinely BLANK (verified independently against d20pfsrd,
/// Archives of Nethys aonprd.com, and legacy.aonprd.com, all three
/// byte-for-byte identical) — base attack bonus genuinely rises to +10
/// (`14 * 3 / 4 = 10`), both good saves genuinely rise to +9
/// (`14 / 2 + 2 = 9`), and the Good domain's Touch of Good sacred bonus
/// genuinely rises to 7 (`14 / 2 = 7`), all via the same pre-existing
/// formulas, not re-derived, while poor Reflex stays +4 (`14 / 3 = 4`),
/// Channel Energy's die count stays 7d6 (`(14 + 1) / 2 = 7`, the odd-level
/// cadence's next rise landing at 15th), and the domain spell slot count
/// stays 7 (a level-14 cleric still casts only up to 7th-level cleric
/// spells; 8th-level cleric spells first appear at level 15) — all
/// integer-division coincidences, checked not assumed — so no new pillar
/// record is added at level 14 either, since no class feature is named in
/// the level-14 Special column. An SD18 slice (`cycle-2026-07-15T3100`,
/// mirroring `cycle-2026-07-15T2800`'s Barbarian, `cycle-2026-07-15T2900`'s
/// Rogue, and `cycle-2026-07-15T3000`'s Fighter level-15 widenings, and the
/// first §3.2 level-15 landing on a full 9-level-caster class) widens the
/// gate again to 1..=15 (`MAX_SUPPORTED_CLERIC_LEVEL = 15`): the class
/// table's level-15 "Special" column reads "Channel energy 8d6" (verified
/// independently against two primary sources — d20pfsrd and the Archives of
/// Nethys aonprd.com mirror, byte-for-byte agreement) — Channel Energy's die
/// count genuinely rises to 8d6 (`(15 + 1) / 2 = 8`, up from 7d6 at level
/// 14) via the same pre-existing formula, not re-derived — and the domain
/// spell slot count also genuinely rises, to 8 (a level-15 cleric casts
/// 8th-level cleric spells for the first time, verified against
/// legacy.aonprd.com's raw spells-per-day table rows after two single-row
/// summarized fetches disagreed with each other and were rejected as tool
/// artifacts, see `CLERIC_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL`), while
/// base attack bonus genuinely rises to +11 (`15 * 3 / 4 = 11`), base Reflex
/// (poor save) genuinely rises to +5 (`15 / 3 = 5`), and both good saves
/// stay +9 (`15 / 2 + 2 = 9`) and Touch of Good's bonus stays 7
/// (`15 / 2 = 7`), integer-division coincidences with level 14, checked not
/// assumed — so two pillars whose underlying formulas genuinely change
/// (Channel Energy dice, domain spell slot count) are widened, plus the
/// base-attack/base-save pillar; no new pillar record is added at level 15
/// either, since the level-15 Special column names only the Channel Energy
/// tier-rise, not a new class feature. An SD18 slice (`cycle-2026-07-15T5300`,
/// mirroring `cycle-2026-07-15T4600`'s Barbarian, the Fighter, `fd3fbe9`'s
/// Wizard, and `c265972`'s Rogue level-16 widenings — the loop's FIFTH §3.2
/// level-16 landing, and the second full 9-level-caster class after Wizard
/// to reach it) widens the gate again to 1..=16 (`MAX_SUPPORTED_CLERIC_LEVEL
/// = 16`): the class table's level-16 "Special" column is genuinely BLANK
/// (verified independently against two primary sources — d20pfsrd and the
/// Archives of Nethys aonprd.com mirror, byte-for-byte agreement) — a pure
/// ceiling raise, exactly mirroring the Wizard level-16 cycle's own pure
/// ceiling raise: base attack bonus genuinely rises to +12 (`16 * 3 / 4 =
/// 12`), both good saves genuinely rise to +10 (`16 / 2 + 2 = 10`), and
/// Touch of Good's bonus genuinely rises to 8 (`16 / 2 = 8`), all via the
/// same pre-existing formulas, not re-derived, while poor Reflex stays +5
/// (`16 / 3 = 5`), Channel Energy's die count stays 8d6 (`(16 + 1) / 2 = 8`,
/// the odd-level cadence's next rise landing at 17th), and the domain spell
/// slot count stays 8 (a level-16 cleric still casts only up to 8th-level
/// cleric spells; 9th-level cleric spells are out of this bounded ceiling's
/// scope) — all integer-division coincidences, checked not assumed — so no
/// new pillar record is added at level 16 either, since no class feature is
/// named in the level-16 Special column. It only:
/// - leaves one recognition explanation so the `class:cleric:N` identity is acknowledged
///   as a prepared divine spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value),
/// - leaves one grounded base-attack-bonus explanation (PF1 Core Rulebook Cleric class
///   table: 3/4 BAB, the same formula shape as Rogue/Monk/Druid) and three grounded
///   base-save explanations (good Fortitude, good Will, poor Reflex), each a standalone
///   record not wired into `PilotBaseChassisComputation.base_attack_bonus`,
///   `compute_total_saves`, or `compute_combat_baseline`,
/// - grounds Channel Energy's die count and daily use count for real (PF1 Core
///   Rulebook Channel Energy: `ceil(cleric level / 2)` d6, minimum 1d6; usable
///   `3 + Charisma modifier` times per day; confirmed the die count stays 1d6 at
///   level 2 and becomes 2d6 at level 3, both via the same formula, the level-3
///   value not re-derived),
/// - surfaces the canonical two-domain choice seam (`choice:cleric_domain ->
///   domain:good` and `choice:cleric_domain -> domain:healing`) as an explicit
///   recognition record carrying no mechanical value, mirroring the Fighter
///   bonus-feat choice-slot seam,
/// - grounds the flat domain spell slot count for real (PF1 Core Rulebook Domains:
///   one domain spell slot per level of cleric spells she can cast, 1st and up —
///   exactly one 1st-level domain slot at levels 1-2, since a level-2 cleric still
///   only casts 1st-level cleric spells; at level 3 a cleric casts 2nd-level cleric
///   spells for the first time, so the count becomes 2 — one 1st-level plus one
///   2nd-level domain slot; the slots' contents are not grounded at any level),
/// - grounds the Good domain's Touch of Good in full when Good is a chosen domain
///   (PF1 Core Rulebook Good Domain: a flat sacred bonus equal to half cleric level,
///   minimum 1, and a flat `3 + Wisdom modifier` uses-per-day count — both formulas
///   are non-dice, so both ground for real at every supported level),
/// - grounds only the Healing domain's Rebuke Death uses-per-day count when Healing
///   is a chosen domain (PF1 Core Rulebook Healing Domain: `3 + Wisdom modifier`
///   times per day), leaving its heal amount (`1d4` plus a per-level bonus, gated on
///   the target's hit-point state) explicitly named but unproven because it is not a
///   flat number, and
/// - emits two distinct claim-blocking diagnostics naming the still-unproven pieces of
///   the domain powers burden and the prepared divine spell posture burden explicitly,
///   rather than hiding behind a generic "unsupported caster" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Cleric prepared divine spell-bearing
/// identity, its grounded base-attack/base-save/Channel-Energy/domain-choice/
/// domain-slot-count/domain-power pillars, and its two named remaining burdens legible
/// on the runtime path.
pub(super) fn explain_cleric_level1_spell_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // v0.6 alpha swarm, risks item 8, sixth slice (2026-07-25): both Cleric
    // burdens are validated/checked regardless of whether Cleric appears
    // alone or in a multiclass mix, and regardless of race -- checked
    // BEFORE the single-class-only/Human gate below, mirroring the Ranger/
    // Paladin/Sorcerer fix exactly. The domain-powers burden is permanently
    // unconditional (no domain-power execution or domain spell-list content
    // is grounded anywhere in this codebase, so this never becomes valid).
    // The prepared-divine spell posture burden is a real, conditional
    // validation, mirroring `unmet_ranger_prepared_spell_conditions` exactly
    // (a PREPARED caster, like Ranger/Paladin, not spontaneous like
    // Sorcerer): validates every `AcquisitionMode::Prepared` selection with
    // `source_class_id == "class:cleric"` against the real
    // `cleric_spell_list::CLERIC_SPELL_LIST` (the general list only --
    // domain spells stay part of the separate domain-powers burden), the
    // cleric's own spell-level access ceiling (1st+; orisons have no access
    // gate, always available from level 1), and the per-level slot budget
    // (base + Wisdom bonus, excluding the separate domain spell slot).
    if let Some(cleric_level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == CLERIC_CLASS_ID)
        .map(|class_level| class_level.level)
    {
        // `decisions.md §22`'s "FURTHER UPDATE, 2026-09-04": a pure
        // class-level pass-through, so pushed unconditionally here
        // alongside this function's other unconditional-on-race burdens --
        // see `cleric_aura_strength_level`'s own doc comment.
        if let Some(strength_level) = cleric_aura_strength_level(cleric_level) {
            explanations.push(ComputationExplanation {
                id: "class_feature.cleric.aura.strength_level".to_owned(),
                value: strength_level,
                detail: format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   `BONUS:VAR|AlignmentAuraLVL|ClericLVL`
                    "Cleric level {cleric_level} Aura: aura strength level {strength_level} (a pure \
                     class-level pass-through selecting one of four DESC-prose tiers depending on \
                     deity alignment: faint at 1, moderate at 2-4, strong at 5-10, overwhelming at \
                     11+; PF1 Core Rulebook `cr_abilities_class.lst`'s)"
                ),
            });
        }

        // v0.6 alpha swarm, risks item 8 (Cleric Good domain closure,
        // adversarially reviewed 2026-07-25): the domain-powers burden is
        // no longer flatly unconditional for every Cleric -- Good domain's
        // Touch of Good (self-application only) can now genuinely close.
        // The catch-all below (no domain chosen, an unrecognized domain
        // chosen, or Good not chosen at all) preserves the EXACT original
        // unconditional diagnostic unchanged, so no input this seam didn't
        // specifically improve can silently reach Computed -- the same
        // false-Computed risk the Ranger dispatch review caught, closed
        // here by construction rather than by review alone.
        let domain_selections_top: Vec<&str> = input
            .chosen
            .selected_choices
            .iter()
            .filter(|c| c.choice_set_id == CLERIC_DOMAIN_CHOICE_ID)
            .map(|c| c.selection_id.as_str())
            .collect();
        let good_domain_chosen = domain_selections_top.contains(&GOOD_DOMAIN_SELECTION);
        let healing_domain_chosen = domain_selections_top.contains(&HEALING_DOMAIN_SELECTION);

        // SD-32 T12 Epic 8 row 18 cycle 5: the generic "select ONE domain,
        // inherit every one of its real corpus powers" pass, covering the
        // other 67 real Cleric domain groups (310 real records total,
        // `census_class_feature_pool_group_names.py`) beyond the six this
        // file already hand-models via `DOMAIN_POWER_CATALOG` -- purely
        // additive alongside the hand-modelled branches below.
        push_generic_pool_group_selection_magnitude(
            input,
            cleric_level,
            ability_modifiers,
            CLERIC_DOMAIN_CHOICE_ID,
            "Cleric",
            "Domain",
            "domain:",
            "class_feature.cleric.domain.generic",
            1,
            explanations,
        );
        // SD-31 wave 26 (OPERATOR-RULINGS-2026-08-21.md section 20): widened
        // from the Good-only gate above to every OTHER `domain_power::
        // DOMAIN_POWER_CATALOG` entry -- Inquisitor's own
        // `ground_or_block_inquisitor_domain_power` already reads this
        // catalog generically; Cleric's own branch had been left
        // Good/Healing-only since wave 25, the exact gap this lane's brief
        // names ("continue, don't restart"). Good keeps its own
        // specially-integrated branch below (its bonus is wired into real
        // combat/skill/save totals via `active_touch_of_good_bonus`, unlike
        // any other catalog entry), so it is deliberately excluded from
        // this generic list to avoid double-grounding it.
        let other_catalog_domains: Vec<&'static DomainPowerSpec> = domain_selections_top
            .iter()
            .filter(|d| **d != GOOD_DOMAIN_SELECTION)
            .filter_map(|d| resolve_domain_power(d))
            .collect();
        let unrecognized_other_domain_chosen = domain_selections_top.iter().any(|d| {
            *d != GOOD_DOMAIN_SELECTION
                && *d != HEALING_DOMAIN_SELECTION
                && resolve_domain_power(d).is_none()
        });

        if (good_domain_chosen || !other_catalog_domains.is_empty())
            && !unrecognized_other_domain_chosen
        {
            if good_domain_chosen {
                let touch_of_good_bonus = cleric_touch_of_good_bonus(cleric_level);
                let touch_of_good_activation = input
                    .chosen
                    .class_ability_activations
                    .iter()
                    .find(|activation| activation.ability_id == TOUCH_OF_GOOD_ABILITY_ID);
                match touch_of_good_activation.map(|activation| activation.active_state) {
                    Some(ActiveState::EquippedActive) => {
                        explanations.push(ComputationExplanation {
                            id: "class_feature.domain.good_touch_of_good_self_application"
                                .to_owned(),
                            value: touch_of_good_bonus,
                            detail: format!(
                                "Cleric level {cleric_level} is actively using Touch of Good on \
                                 HERSELF, SELF-APPLICATION ONLY (PF1 Core Rulebook Good Domain: \
                                 touch a creature, granting it a +{touch_of_good_bonus} sacred \
                                 bonus on attack rolls, skill checks, ability checks, and saving \
                                 throws for 1 round). The +{touch_of_good_bonus} bonus is \
                                 applied to her own baseline melee attack bonus, selected-skill \
                                 modifiers, and total saves (see compute_combat_baseline, \
                                 compute_selected_skill_modifiers, compute_total_saves). \
                                 Granting this bonus to ANOTHER creature -- Touch of Good's real \
                                 primary use in play -- is NOT modeled: no target-creature \
                                 entity exists anywhere in this codebase, only the acting \
                                 character's own rolls are ever computed. The ability-check \
                                 facet has no separate integrated total in this codebase and \
                                 stays a flat, unintegrated magnitude"
                            ),
                        });
                    }
                    _ => {
                        explanations.push(ComputationExplanation {
                            id: "class_feature.domain.good_touch_of_good_not_active".to_owned(),
                            value: 0,
                            detail: format!(
                                "Cleric level {cleric_level} is not currently using Touch of \
                                 Good (no active class_ability_activations entry for \
                                 \"{TOUCH_OF_GOOD_ABILITY_ID}\"): a genuinely valid PF1 \
                                 posture -- not every Good-domain Cleric is using this \
                                 limited-use power at every moment -- so no sacred bonus is \
                                 claimed"
                            ),
                        });
                    }
                }
            }
            // SD-31 wave 26: every OTHER recognized catalog domain (War,
            // Strength, Destruction, Glory -- Good is grounded above, on its
            // own specially-integrated path) grounded generically, mirroring
            // `ground_or_block_inquisitor_domain_power`'s own non-Good
            // branch: real magnitude, real self-application activation
            // state, real uses-per-day, honestly disclosed as NOT
            // integrated into any other computed total (unlike Good's
            // bonus, which IS wired into melee attack/skill/save totals).
            for spec in &other_catalog_domains {
                // SD-34 wave 37 lane A: `grounds_self_application` gates the
                // magnitude/activation-state block below -- `false` for a
                // catalog entry whose corpus formula is NOT a flat combat/
                // skill/save bonus (Undead Subdomain's Death's Kiss: its
                // formula is the power's own effect DURATION in rounds, and
                // the shared "a +{magnitude} {label} {duration}" sentence
                // below would misrepresent a round count as a game bonus).
                // Every pre-existing entry (Good's own special branch above,
                // War/Strength/Destruction/Glory here) is a real flat bonus
                // and keeps `grounds_self_application: true`, so this guard
                // changes nothing for any of them.
                if spec.grounds_self_application {
                    let magnitude = domain_power_magnitude(spec, cleric_level, ability_modifiers);
                    let activation = input
                        .chosen
                        .class_ability_activations
                        .iter()
                        .find(|activation| activation.ability_id == spec.ability_id);
                    match activation.map(|activation| activation.active_state) {
                        Some(ActiveState::EquippedActive) => {
                            explanations.push(ComputationExplanation {
                                id: domain_power_explanation_id(spec, "self_application"),
                                value: magnitude,
                                detail: format!(
                                    "Cleric level {cleric_level}, whose Domain class feature \
                                     selected {domain}, is actively using {power} on HERSELF, \
                                     SELF-APPLICATION ONLY: a +{magnitude} {label} {duration}. This \
                                     grounds only the flat magnitude; it is not integrated into any \
                                     other computed total (melee attack, melee damage, skill, or \
                                     save totals) anywhere in this codebase, unlike Good's own Touch \
                                     of Good. Granting this bonus to ANOTHER creature is NOT \
                                     modeled: no target-creature entity exists anywhere in this \
                                     codebase.",
                                    domain = spec.domain_display_name,
                                    power = spec.granted_power_name,
                                    label = spec.magnitude_label,
                                    duration = spec.effect_duration_phrase,
                                ),
                            });
                        }
                        _ => {
                            explanations.push(ComputationExplanation {
                                id: domain_power_explanation_id(spec, "not_active"),
                                value: 0,
                                detail: format!(
                                    "Cleric level {cleric_level} is not currently using {power} (no \
                                     active class_ability_activations entry for \"{ability_id}\"): a \
                                     genuinely valid PF1 posture -- not every {domain}-domain Cleric \
                                     is using this limited-use power at every moment -- so no \
                                     {label} is claimed",
                                    power = spec.granted_power_name,
                                    ability_id = spec.ability_id,
                                    domain = spec.domain_display_name,
                                    label = spec.magnitude_label,
                                ),
                            });
                        }
                    }
                }
                let wisdom_modifier = ability_modifier(input.chosen.ability_scores.wisdom);
                // SD-34 wave 38 lane A: `domain_power_uses_per_day_for` reads
                // `spec.uses_per_day_formula` when a spec carries one
                // (Animate Servant: `DomainArtificeLVL/4-1`, class-level-
                // dependent, NOT the shared `3+WIS`), else falls back to the
                // exact same `3+WIS` formula `domain_power_uses_per_day`
                // computes -- every pre-existing entry's own value is
                // unchanged by this call-site swap.
                let uses_per_day = domain_power_uses_per_day_for(
                    spec,
                    cleric_level,
                    &AbilityModifiers { wisdom: wisdom_modifier, ..AbilityModifiers::default() },
                );
                let uses_per_day_detail = if let Some(formula) = spec.uses_per_day_formula {
                    format!(
                        "Cleric {domain} domain granted power {power} uses per day (PF1 \
                         Advanced Player's Guide {domain}): this power's OWN corpus formula, \
                         {formula} (not the shared 3 + Wisdom modifier chain every other \
                         catalogued domain power uses), evaluated at Cleric level \
                         {cleric_level} and floored at 0. This is {uses_per_day}. This grounds \
                         only the flat daily use count; it performs no per-use consumption \
                         tracking",
                        domain = spec.domain_display_name,
                        power = spec.granted_power_name,
                    )
                } else {
                    format!(
                        "Cleric {domain} domain granted power {power} uses per day (PF1 Core \
                         Rulebook Domains): 3 + Wisdom modifier, floored at 0. At Wisdom \
                         modifier {wisdom_modifier} this is max(3 + {wisdom_modifier}, 0) = \
                         {uses_per_day}. This grounds only the flat daily use count; it \
                         performs no per-use consumption tracking",
                        domain = spec.domain_display_name,
                        power = spec.granted_power_name,
                    )
                };
                explanations.push(ComputationExplanation {
                    id: domain_power_explanation_id(spec, "uses_per_day"),
                    value: uses_per_day,
                    detail: uses_per_day_detail,
                });
            }
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.cleric.domain_spell_list_contents.unmodeled".to_owned(),
                message: "Cleric domain spell-list contents remain unmodeled: which specific \
                     domain spell fills the already-grounded domain spell slot (per domain, per \
                     spell level) is not implemented for any domain in this codebase. The slot \
                     COUNT is grounded for real; its CONTENT is named but not computed, the \
                     same 'grant-only identity record, no execution engine' idiom used \
                     throughout this session (e.g. Bard's six other bardic performances). This \
                     does not block an otherwise-valid domain posture"
                    .to_owned(),
                claim_blocking: false,
            });
            if healing_domain_chosen {
                diagnostics.push(ComputationDiagnostic {
                    id: "class_feature.cleric.healing_domain.rebuke_death.unsupported".to_owned(),
                    message: "Cleric remains blocked on the Healing domain's granted power, \
                         Rebuke Death: the heal amount (1d4 points plus 1 per two cleric levels) \
                         is a real dice roll, not a flat number, and its target (a living \
                         creature below 0 hit points) is a different creature's hit-point state \
                         this codebase has no concept of anywhere -- unlike Touch of Good, there \
                         is no honest self-scoped version of healing another creature, so no \
                         Rebuke Death support is claimed"
                        .to_owned(),
                    claim_blocking: true,
                });
            }
        } else {
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.cleric.domain_powers.unsupported".to_owned(),
                message: "Cleric remains blocked on its domain powers burden: domain selection \
                     and the granted powers of any domain other than Good, War, Strength, \
                     Destruction, Glory, Undead Subdomain, or Construct Subdomain (whose own \
                     granted powers are grounded separately when actually chosen) are not \
                     implemented anywhere in this codebase (e.g. Healing's Rebuke Death, whose \
                     heal amount is not a flat number), so no Cleric domain-power support is \
                     claimed for this selection"
                    .to_owned(),
                claim_blocking: true,
            });
        }

        let unmet = unmet_cleric_prepared_spell_conditions(input, cleric_level, ability_modifiers);
        if unmet.is_empty() {
            ground_cleric_prepared_spells(input, cleric_level, ability_modifiers, explanations);
        } else {
            diagnostics.push(ComputationDiagnostic {
                id: "class_spell.cleric.prepared_divine.unsupported".to_owned(),
                message: format!(
                    "Cleric remains blocked on its prepared divine spell posture burden: Cleric \
                     is a full 9th-level divine caster (spells begin at cleric level 1); unmet \
                     prepared-spell posture: {}",
                    unmet.join("; ")
                ),
                claim_blocking: true,
            });
        }
    }

    let Some(level) = supported_cleric_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Cleric level-1/
    // level-2 prepared divine spell-bearing identity. This is a recognition record
    // only; it fabricates no domain power math and no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.cleric".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Cleric level {level} prepared divine \
             spell-bearing baseline: the {CLERIC_CLASS_ID}:{level} class identity is \
             acknowledged as a prepared divine spell-bearing class on the rules-core seam rather than \
             an undocumented packet placeholder. This is a bounded recognition record only; it grounds \
             no domain selection, no domain spells, no domain powers, no channel energy execution, no \
             spellbook posture, no spells prepared per day, no spontaneous cure/inflict conversion, no \
             spell slots per day, no spell save DCs, and no bonus spell slots from a high Wisdom, so it \
             carries no fabricated mechanical value (+0)"
        ),
    });

    // Grounded: the foundational base-attack-bonus / base-save progression pillar.
    // Unlike every other class row in this matrix (Fighter, Barbarian, Monk, Rogue,
    // Paladin, Druid all already ground this pillar), Cleric had never had it
    // grounded at all until this SD13-E5 slice. Both formulas were verified against
    // the PF1 Core Rulebook Cleric class table (d20pfsrd and the legacy Paizo PRD
    // mirror) before writing this code, cross-checking the level 2-5 base-attack-bonus
    // values (+0/+1/+2/+3/+3) to disambiguate the exact fraction (level 1 alone floors
    // both a 1/2 and a 3/4 progression to the same +0, so it cannot disambiguate on its
    // own). A later SD13-E5 slice widens this level-1-only gate to level 2; the
    // formula is extended, not re-derived (level 2 base attack +1, all base saves +3,
    // confirmed against the raw class table).
    let level_value = i16::from(level);

    // Grounded (1/2): 3/4-BAB base-attack progression, the same formula shape as
    // Rogue/Monk/Druid (classlevel * 3 / 4). No PCGen .lst file exists for the Cleric
    // class in this repo, so the formula cites the PF1 Core Rulebook Cleric class
    // table directly.
    let base_attack_bonus = level_value * 3 / 4;
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Cleric level {level} base attack bonus from the PF1 Core Rulebook \
             Cleric class table's 3/4-BAB progression, the same formula shape as \
             Rogue/Monk/Druid: classlevel * 3 / 4 = {base_attack_bonus}. This is a standalone \
             explanation record; it is not wired into the integrated base_attack_bonus field or \
             into compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — good Fortitude, poor Reflex, good
    // Will, verified against the PF1 Core Rulebook Cleric class table (Fortitude
    // +2, Reflex +0, Will +2 at level 1).
    let good_save = level_value / 2 + 2;
    let poor_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.base_save.fortitude".to_owned(),
        value: good_save,
        detail: format!(
            "Cleric level {level} base Fortitude save (good save) from the PF1 \
             Core Rulebook Cleric class table: classlevel/2+2 = {good_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.base_save.reflex".to_owned(),
        value: poor_save,
        detail: format!(
            "Cleric level {level} base Reflex save (poor save) from the PF1 \
             Core Rulebook Cleric class table: classlevel/3 = {poor_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.base_save.will".to_owned(),
        value: good_save,
        detail: format!(
            "Cleric level {level} base Will save (good save) from the PF1 Core \
             Rulebook Cleric class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded for real: Channel Energy's flat die count. PF1 Core Rulebook Channel
    // Energy: the cleric channels a number of d6s equal to ceil(cleric level / 2),
    // minimum 1d6. At level 1 this is ceil(1 / 2) = 1d6; confirmed unchanged at level
    // 2 (ceil(2 / 2) = 1d6 too, via the same formula, not a new record). A further
    // SD13-E5 slice confirms this genuinely increases to 2d6 at level 3
    // (ceil(3 / 2) = 2), verified against the PF1 Core Rulebook Cleric class table's
    // level-3 "Special" column ("Channel energy 2d6") — via the same pre-existing
    // formula, not re-derived.
    let channel_energy_dice = (level_value + 1) / 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.channel_energy_dice".to_owned(),
        value: channel_energy_dice,
        detail: format!(
            "Cleric Channel Energy die count: ceil(cleric level / 2) d6 (PF1 Core Rulebook Channel \
             Energy), minimum 1d6. At Cleric level {level} this is \
             ceil({level} / 2) = {channel_energy_dice}d6. This grounds only the flat \
             d6 die count; it computes no channel energy save DC and no positive/negative energy \
             burst damage or healing resolution"
        ),
    });

    // Grounded for real: Channel Energy's flat daily use count. PF1 Core Rulebook
    // Channel Energy: usable 3 + Charisma modifier times per day, floored at 0 (a
    // cleric cannot channel energy a negative number of times per day).
    let channel_energy_uses_per_day = (3 + ability_modifiers.charisma).max(0);
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.channel_energy_uses_per_day".to_owned(),
        value: channel_energy_uses_per_day,
        detail: format!(
            "Cleric Channel Energy uses per day: 3 + Charisma modifier (PF1 Core Rulebook Channel \
             Energy), floored at 0. At Charisma modifier {} this is max(3 + {}, 0) = \
             {channel_energy_uses_per_day}. This grounds only the flat daily use count; it computes \
             no channel energy save DC and no positive/negative energy burst damage or healing \
             resolution",
            ability_modifiers.charisma, ability_modifiers.charisma
        ),
    });

    // Grounded for real: the canonical two-domain choice seam. A PF1 cleric chooses
    // two domains from among those belonging to her deity; the deterministic fixture
    // carries the canonical Good + Healing pair. Mirroring the Fighter bonus-feat
    // choice-slot seam, this surfaces the named selections as an explicit choice seam
    // only when both canonical selections are present — an absent slot is not
    // fabricated — and contributes no computed mechanical value.
    let domain_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == CLERIC_DOMAIN_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();
    if domain_selections.len() == 2
        && domain_selections.contains(&GOOD_DOMAIN_SELECTION)
        && domain_selections.contains(&HEALING_DOMAIN_SELECTION)
    {
        explanations.push(ComputationExplanation {
            id: "class_chassis.cleric.domain_choice".to_owned(),
            value: 0,
            detail: format!(
                "Cleric level {level} chooses two domains from among those \
                 belonging to her deity (PF1 Core Rulebook Domains); the named canonical \
                 selections ({CLERIC_DOMAIN_CHOICE_ID} -> {GOOD_DOMAIN_SELECTION}, \
                 {CLERIC_DOMAIN_CHOICE_ID} -> {HEALING_DOMAIN_SELECTION}) are surfaced as an \
                 explicit choice seam only, mirroring the Fighter bonus-feat choice-slot seam. \
                 This slice grounds the domain choice slot, not the chosen domains' granted \
                 powers or domain spell lists, so it contributes no computed mechanical value \
                 (+0)"
            ),
        });
    }

    // Grounded for real: the flat domain spell slot count. PF1 Core Rulebook Domains:
    // a cleric gains one domain spell slot per level of cleric spells she can cast,
    // 1st and up. This count is class-chassis math independent of which domains were
    // chosen; only the slot's contents (which domain spell may fill it) depend on the
    // chosen domains, and those are deliberately not grounded. Confirmed unchanged at
    // level 2 (a level-2 cleric still only casts 1st-level cleric spells — 2nd-level
    // cleric spells begin at caster level 3, verified against the PF1 Core Rulebook
    // Cleric spells-per-day table via d20pfsrd and legacy.aonprd.com), so this is the
    // same value at level 2, not a new record. A further SD13-E5 slice widens this for
    // real at level 3: a level-3 cleric casts 2nd-level cleric spells for the first
    // time (verified independently against both primary sources' raw spells-per-day
    // table rows), so the count genuinely becomes 2 — one 1st-level domain slot plus
    // one 2nd-level domain slot — mirroring exactly the Wizard specialist-bonus-slot
    // level-3 widening. Confirmed unchanged at level 4 (the level-4 3rd-level spell
    // column is still "—"). A further SD13-E5 slice widens this for real at level 5:
    // a level-5 cleric casts 3rd-level cleric spells for the first time (verified
    // independently against both primary sources' raw spells-per-day table rows), so
    // the count genuinely becomes 3 — one 1st-level, one 2nd-level, and one
    // 3rd-level domain slot. Confirmed unchanged at level 6 (the level-6 4th-level
    // spell column is still "—", verified independently against both primary
    // sources), so the count stays 3 through level 6 as well. A further SD13-E5
    // slice widens this for real at level 7: a level-7 cleric casts 4th-level
    // cleric spells for the first time (verified independently against both
    // primary sources' raw spells-per-day table rows), so the count genuinely
    // becomes 4 — one 1st-level, one 2nd-level, one 3rd-level, and one
    // 4th-level domain slot.
    let domain_spell_slot_count = if level >= CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_17
    } else if level >= CLERIC_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_15
    } else if level >= CLERIC_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_13
    } else if level >= CLERIC_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_11
    } else if level >= CLERIC_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_9
    } else if level >= CLERIC_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_7
    } else if level >= CLERIC_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_5_AND_6
    } else if level >= CLERIC_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_3_AND_4
    } else {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_1_AND_2
    };
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.domain_spell_slot".to_owned(),
        value: domain_spell_slot_count,
        detail: format!(
            "Cleric domain spell slot count: one domain spell slot per level of cleric spells \
             she can cast, 1st and up (PF1 Core Rulebook Domains). At levels 1-2 a cleric \
             casts only 1st-level cleric spells, so the flat count is exactly \
             {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_1_AND_2} 1st-level domain spell slot; \
             at levels {CLERIC_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}-4 a cleric also casts \
             2nd-level cleric spells (2nd-level cleric spells begin at caster level 3, \
             verified against both primary sources' raw spells-per-day table rows), so the \
             flat count becomes {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_3_AND_4} (one \
             1st-level domain slot plus one 2nd-level domain slot); at levels \
             {CLERIC_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}-6 a cleric also casts 3rd-level \
             cleric spells for the first time (verified against both primary sources' raw \
             spells-per-day table rows, including the level-6 row's still-\"—\" 4th-level \
             column), so the flat count becomes \
             {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_5_AND_6} (one 1st-level, one \
             2nd-level, and one 3rd-level domain slot); at level \
             {CLERIC_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a cleric also casts 4th-level \
             cleric spells for the first time (verified against both primary sources' raw \
             spells-per-day table rows), so the flat count becomes \
             {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_7} (one 1st-level, one 2nd-level, one \
             3rd-level, and one 4th-level domain slot); at levels \
             {CLERIC_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}-10 a cleric also casts 5th-level \
             cleric spells for the first time, so the flat count becomes \
             {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_9}; at level \
             {CLERIC_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a cleric also casts 6th-level \
             cleric spells for the first time (verified against both primary sources' raw \
             spells-per-day table rows), so the flat count becomes \
             {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_11} (one each of 1st through 6th-level \
             domain slots); at level {CLERIC_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a \
             cleric also casts 7th-level cleric spells for the first time (verified \
             independently against three primary sources' raw spells-per-day table rows: \
             d20pfsrd, Archives of Nethys aonprd.com, and legacy.aonprd.com, all three \
             byte-for-byte identical), so the flat count becomes \
             {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_13} (one each of 1st through 7th-level \
             domain slots); at level {CLERIC_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a \
             cleric also casts 8th-level cleric spells for the first time (verified against \
             legacy.aonprd.com's raw spells-per-day table rows), so the flat count becomes \
             {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_15} (one each of 1st through 8th-level \
             domain slots); at level {CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a cleric \
             also casts 9th-level cleric spells for the first time (verified independently \
             against two primary sources' raw spells-per-day table rows: d20pfsrd and the \
             Archives of Nethys aonprd.com mirror, byte-for-byte agreement), so the flat count \
             becomes {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_17} (one each of 1st through \
             9th-level domain slots). At Cleric level {level} this is \
             {domain_spell_slot_count} domain spell slot(s). \
             This grounds only the flat slot count; it grounds no slot contents (which domain \
             spell may fill it), no domain spell lists, and no prepared-spell posture"
        ),
    });

    // Grounded for real: the Good domain's granted power, Touch of Good. PF1 Core
    // Rulebook Good Domain: as a standard action, touch a creature to grant it a
    // sacred bonus on attack rolls, skill checks, ability checks, and saving throws
    // equal to half the cleric's level (minimum 1) for 1 round; usable 3 + Wisdom
    // modifier times per day. Verified against the PF1 Core Rulebook Good Domain rule
    // text (d20pfsrd, cross-checked by a second independent search) rather than
    // trusted from the pre-existing blocker-message claim or from memory. Both the
    // bonus magnitude and the uses-per-day count are flat, non-dice formulas, so both
    // ground for real, gated on the Good domain actually being one of the two chosen
    // domains (an absent selection is not fabricated, mirroring the domain-choice
    // seam above).
    if domain_selections.contains(&GOOD_DOMAIN_SELECTION) {
        // At level 1 this floors to the minimum (0 / 2 = 0, floored up to 1); at
        // levels 2-3 it is naturally 1 without needing the floor (2 / 2 = 1,
        // 3 / 2 = 1, integer division) — all three land on the same value,
        // confirmed via the same formula, not a new record. A further SD13-E5
        // slice confirms this genuinely increases to 2 at level 4 (4 / 2 = 2),
        // verified independently against the PF1 Core Rulebook Good Domain
        // granted-power rule text, via the same pre-existing formula, not
        // re-derived. A further SD13-E5 slice confirms this genuinely increases again
        // to 3 at level 6 (6 / 2 = 3), verified independently against the PF1 Core
        // Rulebook Good Domain granted-power rule text, via the same pre-existing
        // formula, not re-derived. A further SD13-E5 slice confirms this genuinely
        // increases again to 4 at level 8 (8 / 2 = 4), verified independently
        // against the PF1 Core Rulebook Good Domain granted-power rule text, via
        // the same pre-existing formula, not re-derived.
        let touch_of_good_bonus = (level_value / 2).max(1);
        explanations.push(ComputationExplanation {
            id: "class_feature.domain.good_touch_of_good_bonus".to_owned(),
            value: touch_of_good_bonus,
            detail: format!(
                "Cleric Good domain granted power Touch of Good sacred bonus (PF1 Core Rulebook \
                 Good Domain): half cleric level, minimum 1, applied for 1 round to attack \
                 rolls, skill checks, ability checks, and saving throws after a touch. At \
                 Cleric level {level} this is \
                 max({level} / 2, 1) = {touch_of_good_bonus}. This grounds only \
                 the flat sacred-bonus magnitude; it computes no touch-attack resolution and no \
                 application of the bonus to any actual attack roll, skill check, ability \
                 check, or saving throw"
            ),
        });

        // SD-31 wave 25 (OPERATOR-RULINGS-2026-08-21.md section 20): interprets
        // the corpus's own shared `3+WIS` formula rather than a hand-written
        // `(3 + wisdom).max(0)` closed form.
        let touch_of_good_uses_per_day = domain_power_uses_per_day(ability_modifiers);
        explanations.push(ComputationExplanation {
            id: "class_feature.domain.good_touch_of_good_uses_per_day".to_owned(),
            value: touch_of_good_uses_per_day,
            detail: format!(
                "Cleric Good domain granted power Touch of Good uses per day (PF1 Core Rulebook \
                 Good Domain): 3 + Wisdom modifier, floored at 0. At Wisdom modifier {} this is \
                 max(3 + {}, 0) = {touch_of_good_uses_per_day}. This grounds only the flat \
                 daily use count; it performs no per-use consumption tracking",
                ability_modifiers.wisdom, ability_modifiers.wisdom
            ),
        });
    }

    // Grounded for real (uses/day only): the Healing domain's granted power, Rebuke
    // Death. PF1 Core Rulebook Healing Domain: as a standard action, touch a living
    // creature below 0 hit points to heal it 1d4 points of damage plus 1 for every
    // two cleric levels; usable 3 + Wisdom modifier times per day. Verified against
    // the PF1 Core Rulebook Healing Domain rule text (d20pfsrd, cross-checked by a
    // second independent search): the uses-per-day rate is genuinely the same "3 +
    // Wisdom modifier" formula as Touch of Good, so the pre-existing blocker-message
    // claim was correct on independent verification — but the heal amount itself is
    // NOT a flat number (a 1d4 dice roll, plus a hit-point-state gating check on the
    // target), so it deliberately stays named-but-unproven rather than fabricated.
    if domain_selections.contains(&HEALING_DOMAIN_SELECTION) {
        // SD-31 wave 25 (OPERATOR-RULINGS-2026-08-21.md section 20): interprets
        // the corpus's own shared `3+WIS` formula rather than a hand-written
        // `(3 + wisdom).max(0)` closed form.
        let rebuke_death_uses_per_day = domain_power_uses_per_day(ability_modifiers);
        explanations.push(ComputationExplanation {
            id: "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day".to_owned(),
            value: rebuke_death_uses_per_day,
            detail: format!(
                "Cleric Healing domain granted power Rebuke Death uses per day (PF1 Core \
                 Rulebook Healing Domain): 3 + Wisdom modifier, floored at 0. At Wisdom \
                 modifier {} this is max(3 + {}, 0) = {rebuke_death_uses_per_day}. This \
                 grounds only the flat daily use count; the heal amount itself (1d4 points of \
                 damage plus 1 for every two cleric levels, usable only on a living creature \
                 below 0 hit points) is not a flat number and is not grounded here — it \
                 requires a dice-roll execution engine and a hit-point-state gating check that \
                 do not exist in this codebase",
                ability_modifiers.wisdom, ability_modifiers.wisdom
            ),
        });
    }

    // v0.6 alpha swarm, risks item 8, sixth slice (2026-07-25): the real
    // Cleric spell math ladder, built from scratch (unlike Ranger/Paladin/
    // Sorcerer, none of this pre-existed -- confirmed by direct grep before
    // starting). Cleric is a full 9th-level caster (spells begin at cleric
    // level 1, including 0th-level orisons), so this covers spell levels
    // 0-9, not the 4-column partial-caster shape. The base spells-per-day
    // table was verified against two independent primary sources
    // (d20pfsrd.com and the Archives of Nethys aonprd.com mirror,
    // byte-for-byte identical for all 20 rows): level 1 "3/1/—/—/—/—/—/—/—/—",
    // level 2 "4/2/—/—/—/—/—/—/—/—", level 3 "4/2/1/—/—/—/—/—/—/—", level 4
    // "4/3/2/—/—/—/—/—/—/—", level 5 "4/3/2/1/—/—/—/—/—/—", level 6
    // "4/3/3/2/—/—/—/—/—/—", level 7 "4/4/3/2/1/—/—/—/—/—", level 8
    // "4/4/3/3/2/—/—/—/—/—", level 9 "4/4/4/3/2/1/—/—/—/—", level 10
    // "4/4/4/3/3/2/—/—/—/—", level 11 "4/4/4/4/3/2/1/—/—/—", level 12
    // "4/4/4/4/3/3/2/—/—/—", level 13 "4/4/4/4/4/3/2/1/—/—", level 14
    // "4/4/4/4/4/3/3/2/—/—", level 15 "4/4/4/4/4/4/3/2/1/—", level 16
    // "4/4/4/4/4/4/3/3/2/—", level 17 "4/4/4/4/4/4/4/3/2/1", level 18
    // "4/4/4/4/4/4/4/3/3/2", level 19 "4/4/4/4/4/4/4/4/3/3", level 20
    // "4/4/4/4/4/4/4/4/4/4" (these are the BASE counts only, excluding the
    // separate "+1" domain spell slot per accessible spell level, already
    // grounded above as `class_chassis.cleric.domain_spell_slot` -- the raw
    // table's own "+1" notation names that domain slot, verified via both
    // primary sources' own rule text: "A cleric gains one domain spell slot
    // for each level of cleric spell she can cast, from 1st on up"). The
    // 0th-level (orison) column never gets a domain-slot addition or a
    // Wisdom bonus (PF1 rule: bonus spells apply only to spell levels 1+),
    // and the level thresholds for each spell-level column's first
    // appearance exactly match the already-grounded
    // CLERIC_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL through
    // CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL constants (cross-checked
    // against this same table, not independently re-derived).
    let cleric_base_spells_per_day = cleric_base_spells_per_day_table(level);
    for (spell_level, base_count) in cleric_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.cleric.base_spells_per_day.spell_level_{spell_level}"),
            value: *base_count,
            detail: format!(
                "Cleric base spells per day at cleric level {level}, spell level \
                 {spell_level}: {base_count}, read directly from the PF1 Core Rulebook \
                 Cleric class table's spells-per-day row (verified against the raw table rows \
                 of both primary sources; a literal table lookup, not a derived formula; \
                 excludes the separate domain spell slot, already grounded above). This grounds \
                 the base count only: bonus spells per day from a high Wisdom are never \
                 computed here, no prepared posture or spell-source lineage is grounded, and no \
                 spell save DCs are computed"
            ),
        });
    }

    // Grounded: the base spell-save-DC arithmetic, one record per ACCESSIBLE
    // spell level 1+ (0th-level orisons have no save-DC record, mirroring
    // the Sorcerer/Wizard/Ranger/Paladin precedent of DC records starting
    // at 1st level). Verified against both primary sources, which state the
    // rule identically: "The Difficulty Class for a saving throw against a
    // cleric's spell is 10 + the spell level + the cleric's Wisdom modifier."
    let cleric_spell_level_access = cleric_spell_level_access(level);
    for spell_level in 1..=cleric_spell_level_access {
        let spell_save_dc = 10 + spell_level + ability_modifiers.wisdom;
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.cleric.spell_save_dc.spell_level_{spell_level}"),
            value: spell_save_dc,
            detail: format!(
                "Cleric spell save DC at cleric level {level}, spell level {spell_level}: \
                 10 + {spell_level} + Wisdom modifier {} = {spell_save_dc} (PF1 Core Rulebook, \
                 verified identically on both primary sources: \"The Difficulty Class for a \
                 saving throw against a cleric's spell is 10 + the spell level + the cleric's \
                 Wisdom modifier\"). This grounds the base DC formula only: no saving-throw \
                 resolution, no target, no spell selection, and no domain DC modifiers are \
                 computed",
                ability_modifiers.wisdom
            ),
        });
    }

    // Grounded: the bonus spells per day from a high Wisdom, one record per
    // ACCESSIBLE spell level 1+, from PF1's shared Table: Ability Modifiers
    // and Bonus Spells (the same shared formula already grounded for
    // Ranger/Paladin/Sorcerer/Wizard): for modifier m and spell level N, 0
    // when m < N, otherwise (m - N)/4 + 1. Orisons (0th level) never gain a
    // bonus slot from Wisdom, per the same rule text cited for every other
    // caster in this family.
    for spell_level in 1..=cleric_spell_level_access {
        let bonus_spells = ability_bonus_spells(ability_modifiers.wisdom, spell_level);
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.cleric.bonus_spells_per_day.spell_level_{spell_level}"),
            value: bonus_spells,
            detail: format!(
                "Cleric bonus spells per day at cleric level {level}, spell level {spell_level}: \
                 {bonus_spells} from Wisdom modifier {} (PF1 Core Rulebook Table: Ability \
                 Modifiers and Bonus Spells). A computed 0 means the modifier grants no bonus at \
                 this spell level; it is never added to the base per-day count here -- no total \
                 is computed, no spell selection, and no spell save DCs",
                ability_modifiers.wisdom
            ),
        });
    }

    // Grounded: the TOTAL spells per day -- the pure sum of the base table
    // count and the Wisdom bonus count (0 for orisons) per accessible spell
    // level, completing the integrated totals across the whole partial/
    // full-caster family now grounded in this codebase. This deliberately
    // does NOT include the separate domain spell slot (its own contents
    // remain the unproven domain-powers burden).
    for (spell_level, base_count) in cleric_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let bonus_spells = if spell_level == 0 {
            0
        } else {
            ability_bonus_spells(ability_modifiers.wisdom, spell_level as i16)
        };
        let total_spells = base_count + bonus_spells;
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.cleric.total_spells_per_day.spell_level_{spell_level}"),
            value: total_spells,
            detail: format!(
                "Cleric total spells per day at cleric level {level}, spell level {spell_level}: \
                 base table count {base_count} + Wisdom bonus {bonus_spells} = {total_spells} -- \
                 the pure sum of the two separately grounded records, giving the actual \
                 castable slot count per day (excluding the separate domain spell slot). This \
                 grounds the count only: no prepared-posture selection, no casting execution, \
                 no slot consumption or tracking, and no spell save resolution"
            ),
        });
    }

    // (v0.6 alpha swarm, risks item 8) Both remaining burdens are pushed at
    // the top of this function (see that push site's own doc comment for
    // the full conditional shape): the domain-powers burden is no longer
    // flatly unconditional -- Good domain's Touch of Good (self-scoped)
    // can genuinely close it -- and the prepared divine spell posture is a
    // real, conditional validation, same as before.
}

/// Good domain's Touch of Good sacred bonus: half cleric level, minimum 1
/// (PF1 Core Rulebook Good Domain). SD-31 wave 25 (OPERATOR-RULINGS-2026-08-21.md
/// section 20): no longer a hand-written closed form -- interprets the
/// corpus's own `max(DomainGoodLVL/2,1)` formula
/// (`domain_power::DOMAIN_POWER_CATALOG`'s Good entry) via
/// `domain_power::domain_power_magnitude`, with `AbilityModifiers::default()`
/// since this formula never references an ability score. Kept as its own
/// named function (rather than inlining the catalog lookup at every call
/// site) so the pre-existing informational explanation record and the real
/// self-application closure still share one source of truth, mirroring
/// `bard_inspire_courage_bonus`'s own shape; `domain_power`'s own
/// `domain_power_magnitude_for_good_matches_cleric_touch_of_good_bonus` test
/// pins this function's output against the SAME formula string evaluated a
/// second, independent way (a literal-int lookup table for levels 1-20) so
/// the two do not silently drift now that one wraps the other.
pub(super) fn cleric_touch_of_good_bonus(level: u8) -> i16 {
    let spec = resolve_domain_power(GOOD_DOMAIN_SELECTION)
        .expect("Good is always present in DOMAIN_POWER_CATALOG");
    domain_power_magnitude(spec, level, &AbilityModifiers::default())
}

/// Cleric's Aura (`core_rulebook:class_feature:cleric_aura`,
/// `cr_abilities_class.lst:563`): `BONUS:VAR|AlignmentAuraLVL|ClericLVL` --
/// a pure class-level pass-through selecting which of the four
/// `PREDEITYALIGN`-gated virtual sub-abilities (Aura of Chaos/Evil/Good/Law,
/// `cr_abilities_class.lst:2874-2877`) displays which DESC-prose tier
/// (faint at 1, moderate at 2-4, strong at 5-10, overwhelming at 11+) --
/// the identical shape and tier breakpoints as the Antipaladin's own mirror
/// feature, `rules_tables::apg::antipaladin_features::
/// aura_of_evil_strength_level` (`decisions.md §22`'s "FURTHER UPDATE,
/// 2026-09-04": this class had the identical structural precedent already
/// built for its own mirror class, just never symmetrically added here).
/// The alignment-gating itself (which of the four flavors of aura a given
/// cleric projects) depends on the cleric's deity, which this engine does
/// not model deity selection for at all; this grounds only the magnitude
/// that is identical regardless of which of the four is chosen. `None`
/// below level 1 -- the class feature's own
/// `PREVARGTEQ:Cleric_CFP_Level,1` grant gate.
pub(super) fn cleric_aura_strength_level(level: u8) -> Option<i16> {
    if level < 1 {
        return None;
    }
    Some(i16::from(level))
}

/// The highest ACCESSIBLE cleric spell level (1st+) at the given cleric
/// level -- orisons (0th level) have no access gate at all, always
/// available from level 1. Pure function, race-independent, mirrors
/// `ranger_spell_level_access`/`paladin_spell_level_access`/
/// `sorcerer_spell_level_access` -- extracted so both the (Human-only)
/// flat explanation block above and the real prepared-spell validation
/// below share one source of truth.
pub(super) fn cleric_spell_level_access(level: u8) -> i16 {
    if level >= CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        9
    } else if level >= CLERIC_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        8
    } else if level >= CLERIC_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        7
    } else if level >= CLERIC_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        6
    } else if level >= CLERIC_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        5
    } else if level >= CLERIC_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        4
    } else if level >= CLERIC_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        3
    } else if level >= CLERIC_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        2
    } else {
        1
    }
}

/// The PF1 Core Rulebook Cleric class table's BASE spells-per-day row, one
/// entry per spell level 0-9 (`None` for an inaccessible "—" column; index
/// 0 is orisons). A literal table lookup, not a derived formula -- verified
/// against two independent primary sources (d20pfsrd.com and the Archives
/// of Nethys aonprd.com mirror, byte-for-byte identical) -- see
/// `explain_cleric_level1_spell_baseline`'s own doc comment for the full
/// row-by-row citation. Excludes the separate "+1" domain spell slot,
/// already grounded independently as `class_chassis.cleric.domain_spell_slot`.
/// Pure function, race-independent, extracted for the same reason as
/// `cleric_spell_level_access`.
pub(super) fn cleric_base_spells_per_day_table(level: u8) -> [Option<i16>; 10] {
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
        13 => {
            [Some(4), Some(4), Some(4), Some(4), Some(4), Some(3), Some(2), Some(1), None, None]
        }
        14 => {
            [Some(4), Some(4), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None, None]
        }
        15 => {
            [Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(3), Some(2), Some(1), None]
        }
        16 => {
            [Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None]
        }
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

/// The real per-day slot budget per spell level 0-9 (base table count +
/// Wisdom bonus for 1st+, orisons never get a bonus, `None` for an
/// inaccessible column), excluding the separate domain spell slot.
pub(super) fn cleric_total_spells_per_day(level: u8, wisdom_modifier: i16) -> [Option<i16>; 10] {
    let base = cleric_base_spells_per_day_table(level);
    let mut total = [None; 10];
    for (spell_level, base_count) in base.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let bonus = if spell_level == 0 {
            0
        } else {
            ability_bonus_spells(wisdom_modifier, spell_level as i16)
        };
        total[spell_level] = Some(base_count + bonus);
    }
    total
}

/// Return the list of unmet conditions for Cleric's real prepared-spell
/// posture. Mirrors `unmet_ranger_prepared_spell_conditions` exactly,
/// substituting Wisdom (same ability) and
/// `cleric_spell_list::CLERIC_SPELL_LIST` (the general list only -- domain
/// spells stay part of the separate domain-powers burden) for the ranger
/// list, and covering spell levels 0-9 (a full caster) instead of 1-4 (a
/// partial caster). An empty list means the posture is fully valid: every
/// `AcquisitionMode::Prepared` selection with `source_class_id ==
/// "class:cleric"` names a real general-list spell, at a spell level within
/// the cleric's own access ceiling (0 always accessible), and no spell
/// level's prepared count exceeds that level's total slot budget (base +
/// Wisdom bonus, excluding the domain slot). Zero prepared spells is always
/// valid, same reasoning as every other class in this family.
pub(super) fn unmet_cleric_prepared_spell_conditions(
    input: &CharacterInput,
    cleric_level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Vec<String> {
    let mut unmet = Vec::new();

    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == CLERIC_CLASS_ID && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    let access_ceiling = cleric_spell_level_access(cleric_level);
    let total_per_day = cleric_total_spells_per_day(cleric_level, ability_modifiers.wisdom);

    let mut consumed_per_level: [i16; 10] = [0; 10];
    for spell_id in &prepared {
        let Some(spell_level) = cleric_spell_list::cleric_spell_level(spell_id) else {
            unmet.push(format!(
                "prepared spell '{spell_id}' is not on the real PF1 general cleric spell list"
            ));
            continue;
        };
        if spell_level > 0 && i16::from(spell_level) > access_ceiling {
            unmet.push(format!(
                "prepared spell '{spell_id}' targets spell level {spell_level}, not yet \
                 accessible at cleric level {cleric_level} (access ceiling {access_ceiling})"
            ));
            continue;
        }
        consumed_per_level[usize::from(spell_level)] += 1;
    }

    for (spell_level, consumed) in consumed_per_level.iter().enumerate() {
        if *consumed == 0 {
            continue;
        }
        let total_slots = total_per_day[spell_level].unwrap_or(0);
        if *consumed > total_slots {
            unmet.push(format!(
                "spell level {spell_level} over-prepared: {consumed} spells prepared but only \
                 {total_slots} slots available (base + Wisdom bonus, excluding the domain slot)"
            ));
        }
    }

    unmet
}

/// Ground the real prepared-spell posture once
/// `unmet_cleric_prepared_spell_conditions` reports an empty unmet list.
/// Mirrors `ground_ranger_prepared_spells` exactly, substituting Wisdom and
/// covering spell levels 0-9.
pub(super) fn ground_cleric_prepared_spells(
    input: &CharacterInput,
    cleric_level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == CLERIC_CLASS_ID && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.cleric.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Cleric level {cleric_level} daily preparation selection ({} spells, \
             AcquisitionMode::Prepared): {}. Each prepared spell is verified against the real \
             PF1 general cleric spell list (`cleric_spell_list::CLERIC_SPELL_LIST`, all \
             ingested books), \
             the cleric's own spell-level access ceiling, and the per-level slot budget (base \
             table count + Wisdom bonus, excluding the separate domain spell slot). This grounds \
             the prepared-spell selection for real; it computes no spell save DC resolution \
             against a target and no casting execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let total_per_day = cleric_total_spells_per_day(cleric_level, ability_modifiers.wisdom);
    for (spell_level, total) in total_per_day.iter().enumerate() {
        let Some(total) = total else {
            continue;
        };
        explanations.push(ComputationExplanation {
            id: format!("class_spell.cleric.total_spells_per_day.spell_level_{spell_level}"),
            value: *total,
            detail: format!(
                "Cleric level {cleric_level} total spells per day at spell level {spell_level}: \
                 {total} (base table count + Wisdom bonus, excluding the separate domain spell \
                 slot -- the same records already grounded as \
                 `class_chassis.cleric.total_spells_per_day.spell_level_{spell_level}` for a \
                 Human cleric, computed here independent of race). This is the real slot budget \
                 the daily preparation selection above is validated against"
            ),
        });
    }
}

/// v0.6 alpha swarm, risks item 8, sixth slice (2026-07-25): Cleric's real
/// prepared-divine known-spell posture, mirroring
/// `ranger_dispatch_widening_safety_tests`/`paladin_dispatch_widening_safety_tests`
/// exactly (PREPARED, like Ranger/Paladin, not spontaneous like Sorcerer;
/// same gate-ordering structural risk existed here too and was fixed
/// proactively as part of this same slice). Unlike Ranger/Paladin, Cleric's
/// domain-powers burden stays permanently unconditional (mirrors Sorcerer's
/// bloodline-power shape), so a single-class Cleric never reaches
/// `Computed` even with a fully valid prepared-spell posture -- only the
/// spell-specific diagnostic is conditional.
#[cfg(test)]
mod cleric_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, ActiveState, ANIMATE_SERVANT_ABILITY_ID,
        BATTLE_RAGE_ABILITY_ID, CharacterClassLevel, CharacterInput, CLERIC_CLASS_ID,
        CLERIC_DOMAIN_CHOICE_ID, CONSTRUCT_SUBDOMAIN_SELECTION, DEATH_S_KISS_ABILITY_ID,
        DESTRUCTION_DOMAIN_SELECTION, DESTRUCTIVE_SMITE_ABILITY_ID, FIGHTER_CLASS_ID,
        GLORY_DOMAIN_SELECTION, GOOD_DOMAIN_SELECTION, HEALING_DOMAIN_SELECTION,
        HeadlessReceiptStatus, STRENGTH_DOMAIN_SELECTION, TOUCH_OF_GLORY_ABILITY_ID,
        TOUCH_OF_GOOD_ABILITY_ID, UNDEAD_SUBDOMAIN_SELECTION, WAR_DOMAIN_SELECTION,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, ClassAbilityActivation, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// A single-class Cleric with no domain chosen at all (this test's
    /// fixture never selects a domain) stays `Blocked` on the catch-all
    /// domain-powers diagnostic -- unlike the Good-domain-recognized case
    /// (see `single_class_cleric_with_good_domain_touch_of_good_active_reaches_computed`
    /// below), an unrecognized/absent domain choice has no vacuous or
    /// self-scopable pieces to resolve, so it stays unconditionally
    /// blocking. The spell-specific diagnostic must NOT fire when the
    /// posture is genuinely valid, regardless.
    #[test]
    fn single_class_cleric_with_no_prepared_spells_stays_blocked_only_on_domain_powers() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: CLERIC_CLASS_ID.to_owned(), level: 5 }];

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Cleric's domain-powers burden is permanently unconditional: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.cleric.domain_powers.unsupported"
                    && d.claim_blocking),
            "expected the permanent domain-powers diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.cleric.prepared_divine.unsupported"),
            "the spell-posture diagnostic must not fire when the prepared-spell posture is \
             valid: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Cleric preparing a real, in-budget, accessible orison
    /// (0th-level spell, always accessible from level 1) does not trip the
    /// spell-posture diagnostic -- proving orisons have no access gate.
    #[test]
    fn single_class_cleric_with_a_valid_orison_does_not_trip_the_spell_posture() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: CLERIC_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Bleed".to_owned(),
            source_class_id: CLERIC_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.cleric.prepared_divine.unsupported"),
            "an orison is always accessible from level 1 with no access gate: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Cleric preparing a spell beyond their spell-level
    /// access ceiling must carry the real spell-posture diagnostic.
    #[test]
    fn single_class_cleric_with_an_inaccessible_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: CLERIC_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Aid".to_owned(),
            source_class_id: CLERIC_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.cleric.prepared_divine.unsupported"
                    && d.claim_blocking),
            "a 2nd-level cleric spell is not accessible at cleric level 1: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Cleric preparing a spell not on the real PF1 Core
    /// Rulebook general cleric spell list at all must also carry the
    /// diagnostic.
    #[test]
    fn single_class_cleric_with_an_off_list_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: CLERIC_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: CLERIC_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Magic Missile is not on the real general cleric spell list: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Cleric at level 1 (Wisdom 12, mod +1: base 1 + bonus
    /// 1 = total budget 2 for 1st-level spells) preparing 3 distinct
    /// 1st-level spells over-prepares the real slot budget.
    #[test]
    fn single_class_cleric_over_prepared_slot_budget_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: CLERIC_CLASS_ID.to_owned(), level: 1 }];
        for spell_id in ["Bane", "Bless", "Cause Fear"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: CLERIC_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Prepared,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "cleric level 1 with Wisdom 12 has a real total budget of 2 slots for 1st-level \
             spells (base 1 + Wisdom bonus 1), so preparing 3 distinct spells over-prepares: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.cleric.prepared_divine.unsupported"
                    && d.claim_blocking),
            "expected the real spell-posture diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Multiclass safety, verified directly. A Cleric-containing multiclass
    /// mix with a genuine posture violation must still stay Blocked: the
    /// posture check runs for Cleric alone or mixed, and since SD-36 F3b the
    /// mix also carries Cleric's isolated-run blocking lines
    /// (`multiclass_fold`).
    #[test]
    fn cleric_fighter_multiclass_with_an_invalid_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: CLERIC_CLASS_ID.to_owned(), level: 1 },
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
        ];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: CLERIC_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "a Cleric+Fighter multiclass must not reach Computed while Cleric's posture is \
             genuinely violated: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.cleric.prepared_divine.unsupported"
                    && d.claim_blocking),
            "expected the real spell-posture diagnostic to fire in the multiclass mix too: {:?}",
            receipt.computation.diagnostics
        );
    }

    fn human_cleric_input_with_domains(level: u8, domains: &[&str]) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: CLERIC_CLASS_ID.to_owned(), level }];
        for domain in domains {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: CLERIC_DOMAIN_CHOICE_ID.to_owned(),
                selection_id: (*domain).to_owned(),
            });
        }
        input
    }

    /// v0.6 alpha swarm, risks item 8 (Cleric Good domain closure): a
    /// single-class Human Cleric with ONLY the Good domain chosen, no
    /// prepared-spell posture violation, and Touch of Good genuinely
    /// active (self-application) reaches `Computed` -- the domain-powers
    /// burden is no longer permanently unconditional once Good's Touch of
    /// Good is the ONLY domain power in play (no Healing domain's Rebuke
    /// Death to keep it blocked).
    #[test]
    fn single_class_cleric_with_good_domain_touch_of_good_active_reaches_computed() {
        let mut input = human_cleric_input_with_domains(1, &[GOOD_DOMAIN_SELECTION]);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: TOUCH_OF_GOOD_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a Good-domain-only Cleric with Touch of Good active and a valid spell posture \
             should reach Computed: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.cleric.domain_spell_list_contents.unmodeled"
                    && !d.claim_blocking),
            "expected the honest, non-blocking domain-spell-list-contents diagnostic: {:?}",
            receipt.computation.diagnostics
        );

        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        // Cleric level 1 base attack bonus (3/4 BAB: 1*3/4 = 0) + Strength
        // modifier (+4, fixture's chosen Human +2 Strength applied to
        // base 16) + Weapon Focus (+1) + Touch of Good (+1, half level 1
        // = 0, floored to minimum 1) - 4 nonproficiency = 2.
        //
        // Corrected 6 -> 2 (risks item #89, tasks #80+#86, 2026-07-29).
        // The old 6 was a real wrong number, not a changed expectation:
        // this baseline swings a Longsword, and Cleric's corpus grant is
        // the Simple tier plus `AUTO:WEAPONPROF|DEITYWEAPONS`. The
        // Longsword is Martial, and no deity is modelled anywhere in this
        // engine, so a Cleric here is genuinely non-proficient and owes
        // PF1's -4 (`WEAPONNONPROFPENALTY` in
        // `system/gameModes/Pathfinder/miscinfo.lst:193`).
        assert_eq!(
            melee_attack_bonus.value, 2,
            "Touch of Good's sacred bonus must be applied to the attack roll: {melee_attack_bonus:?}"
        );
    }

    /// A Good-domain Cleric who is NOT currently using Touch of Good (no
    /// activation entry) is a genuinely valid posture too, and also
    /// reaches `Computed` (with no bonus applied).
    #[test]
    fn single_class_cleric_with_good_domain_not_using_touch_of_good_reaches_computed() {
        let input = human_cleric_input_with_domains(1, &[GOOD_DOMAIN_SELECTION]);

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);
        let melee_attack_bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee attack bonus must be grounded");
        // Cleric level 1 base attack bonus (0) + Strength modifier (+4) +
        // Weapon Focus (+1) + no Touch of Good bonus (not active)
        // - 4 nonproficiency = 1. Corrected 5 -> 1 for the same reason as
        // the Touch-of-Good case above (risks item #89).
        assert_eq!(melee_attack_bonus.value, 1);
    }

    /// A Cleric with BOTH Good and Healing domains (the pre-existing
    /// fixture shape) still stays `Blocked` -- Healing's Rebuke Death heal
    /// amount is a real dice roll targeting a different creature's
    /// hit-point state, which has no honest self-scoped closure the way
    /// Touch of Good does.
    #[test]
    fn single_class_cleric_with_good_and_healing_domains_stays_blocked_on_rebuke_death() {
        let mut input =
            human_cleric_input_with_domains(1, &[GOOD_DOMAIN_SELECTION, HEALING_DOMAIN_SELECTION]);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: TOUCH_OF_GOOD_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Rebuke Death's heal amount stays a genuine, unresolved blocker: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.cleric.healing_domain.rebuke_death.unsupported"
                    && d.claim_blocking),
            "expected the real, still-blocking Rebuke Death diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.cleric.domain_powers.unsupported"),
            "the old generic catch-all diagnostic must not ALSO fire once Good is recognized: \
             {:?}",
            receipt.computation.diagnostics
        );
    }

    /// SD-31 wave 26: a single-class Cleric with War domain (not previously
    /// wired for Cleric, only for Inquisitor since wave 25) and Battle Rage
    /// genuinely active reaches `Computed`, mirroring Good's own shape but
    /// through the new generic catalog loop rather than the special-cased
    /// Good branch.
    #[test]
    fn single_class_cleric_with_war_domain_battle_rage_active_reaches_computed() {
        let mut input = human_cleric_input_with_domains(4, &[WAR_DOMAIN_SELECTION]);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: BATTLE_RAGE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a War-domain-only Cleric with Battle Rage active and a valid spell posture should \
             reach Computed: {:?}",
            receipt.computation.diagnostics
        );
        let self_application = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.war_battle_rage_self_application")
            .expect("Battle Rage self-application explanation must be grounded");
        // War domain magnitude: max(level/2, 1); level 4 -> max(2,1) = 2.
        assert_eq!(self_application.value, 2, "{self_application:?}");
        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.domain.war_battle_rage_uses_per_day"),
            "Battle Rage uses-per-day must also be grounded: {:?}",
            receipt.computation.explanations
        );
    }

    /// SD-31 wave 26: a Cleric with Strength domain who is NOT currently
    /// using Strength Surge (no activation entry) also reaches `Computed`,
    /// with the magnitude at 0 -- the "not active" arm of the new generic
    /// loop.
    #[test]
    fn single_class_cleric_with_strength_domain_not_using_strength_surge_reaches_computed() {
        let input = human_cleric_input_with_domains(1, &[STRENGTH_DOMAIN_SELECTION]);

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);
        let not_active = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.strength_strength_surge_not_active")
            .expect("Strength Surge not-active explanation must be grounded");
        assert_eq!(not_active.value, 0);
    }

    /// SD-31 wave 26: the two newly-widened domains (Destruction's
    /// Destructive Smite, Glory's Touch of Glory) also reach `Computed` for
    /// Cleric through the SAME generic loop, proving the widening is not
    /// special-cased to War/Strength alone.
    #[test]
    fn single_class_cleric_with_destruction_domain_destructive_smite_active_reaches_computed() {
        let mut input = human_cleric_input_with_domains(6, &[DESTRUCTION_DOMAIN_SELECTION]);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: DESTRUCTIVE_SMITE_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Computed, "{:?}", receipt.computation.diagnostics);
        let self_application = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.destruction_destructive_smite_self_application")
            .expect("Destructive Smite self-application explanation must be grounded");
        // Destruction domain magnitude: max(level/2, 1); level 6 -> max(3,1) = 3.
        assert_eq!(self_application.value, 3, "{self_application:?}");
    }

    #[test]
    fn single_class_cleric_with_glory_domain_touch_of_glory_active_reaches_computed() {
        let mut input = human_cleric_input_with_domains(7, &[GLORY_DOMAIN_SELECTION]);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: TOUCH_OF_GLORY_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Computed, "{:?}", receipt.computation.diagnostics);
        let self_application = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.glory_touch_of_glory_self_application")
            .expect("Touch of Glory self-application explanation must be grounded");
        // Glory domain magnitude: the bare cleric level, unhalved -- level 7 -> 7.
        assert_eq!(self_application.value, 7, "{self_application:?}");
    }

    /// SD-34 wave 37 lane A: a Cleric with Undead Subdomain reaches
    /// `Computed` through the SAME generic loop as War/Strength/Destruction/
    /// Glory -- the first APG SUBDOMAIN, not a base CRB domain, proven
    /// through this seam. Only Death's Kiss's real, honestly-computed
    /// uses-per-day is ever grounded: NEITHER a "self_application" NOR a
    /// "not_active" explanation is emitted for it, regardless of whether an
    /// activation entry is present -- `grounds_self_application: false`
    /// (its own corpus formula is an effect duration in rounds, not a flat
    /// bonus, so no "+{magnitude}" sentence is ever produced for it).
    #[test]
    fn single_class_cleric_with_undead_subdomain_reaches_computed_via_uses_per_day_only() {
        let mut input = human_cleric_input_with_domains(4, &[UNDEAD_SUBDOMAIN_SELECTION]);
        // Even WITH an activation entry present, no bonus explanation may be
        // emitted -- proves the guard is unconditional, not merely "happens
        // not to fire because nothing activated it".
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: DEATH_S_KISS_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "{:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.domain.undead_subdomain_death_s_kiss_self_application"
                    || e.id == "class_feature.domain.undead_subdomain_death_s_kiss_not_active"),
            "Death's Kiss must never surface a self_application/not_active magnitude \
             explanation (its corpus formula is a duration, not a bonus): {:?}",
            receipt.computation.explanations
        );
        let uses_per_day = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.undead_subdomain_death_s_kiss_uses_per_day")
            .expect("Death's Kiss uses-per-day explanation must be grounded");
        // `human_cleric_input_with_domains`'s fixture Wisdom modifier is +1 -> 3 + 1 = 4.
        assert_eq!(uses_per_day.value, 4, "{uses_per_day:?}");
    }

    /// SD-34 wave 38 lane A: a Cleric with Construct Subdomain reaches
    /// `Computed` through the SAME generic loop as Undead Subdomain -- the
    /// second APG SUBDOMAIN proven through this seam. Neither a
    /// "self_application" nor a "not_active" explanation is ever emitted for
    /// Animate Servant (its real effect, casting animate objects, has no
    /// bonus at all to ground), and its uses-per-day count comes from its
    /// OWN `uses_per_day_formula` override (`DomainArtificeLVL/4-1`), NOT
    /// the shared `3+WIS` chain -- proven by using a Cleric level/Wisdom
    /// combination where the two formulas disagree.
    #[test]
    fn single_class_cleric_with_construct_subdomain_reaches_computed_via_its_own_uses_per_day_formula()
     {
        let mut input = human_cleric_input_with_domains(12, &[CONSTRUCT_SUBDOMAIN_SELECTION]);
        // Even WITH an activation entry present, no bonus explanation may be
        // emitted -- proves the guard is unconditional, mirroring Death's
        // Kiss's own proof above.
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: ANIMATE_SERVANT_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "{:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.domain.construct_subdomain_animate_servant_self_application"
                    || e.id == "class_feature.domain.construct_subdomain_animate_servant_not_active"),
            "Animate Servant must never surface a self_application/not_active bonus \
             explanation (it has no bonus at all to ground): {:?}",
            receipt.computation.explanations
        );
        let uses_per_day = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.domain.construct_subdomain_animate_servant_uses_per_day")
            .expect("Animate Servant uses-per-day explanation must be grounded");
        // DomainArtificeLVL/4-1 at Cleric level 12: 12/4-1 = 2. The fixture's
        // own Wisdom modifier is +1, so the shared 3+WIS formula would give
        // 4 -- a DIFFERENT value, proving the override, not the shared
        // formula, genuinely computed this number.
        assert_eq!(
            uses_per_day.value, 2,
            "DomainArtificeLVL/4-1 at level 12 = 2, NOT the shared 3+WIS value of 4: \
             {uses_per_day:?}"
        );
    }

    /// A Cleric with an unrecognized domain (not Good or Healing) still
    /// falls through to the unchanged catch-all diagnostic -- the false-
    /// Computed risk the adversarial review specifically checked for.
    #[test]
    fn single_class_cleric_with_an_unrecognized_domain_stays_blocked_on_the_catch_all() {
        let input = human_cleric_input_with_domains(1, &["domain:fire"]);

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.cleric.domain_powers.unsupported"
                    && d.claim_blocking),
            "an unrecognized domain must still trip the original catch-all diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A Cleric with Good domain PLUS an unrecognized second domain also
    /// falls through to the catch-all -- Good's recognition alone must
    /// not silently ignore an otherwise-unmodeled domain slot.
    #[test]
    fn single_class_cleric_with_good_plus_an_unrecognized_domain_stays_blocked_on_the_catch_all() {
        let input = human_cleric_input_with_domains(1, &[GOOD_DOMAIN_SELECTION, "domain:travel"]);

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.cleric.domain_powers.unsupported"
                    && d.claim_blocking),
            "Good + an unrecognized domain must still trip the catch-all, not silently reach \
             Computed on Good alone: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A non-Cleric character carrying a spoofed `"touch_of_good"`
    /// activation entry must have it silently ignored -- the class-
    /// ownership gate is by construction
    /// (`active_touch_of_good_bonus` only ever reads
    /// `class_ability_activations` after confirming both `class_levels`
    /// contains Cleric and the Good domain is chosen), not a bolt-on
    /// rejection.
    #[test]
    fn non_cleric_characters_spoofed_touch_of_good_activation_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.class_ability_activations.push(ClassAbilityActivation {
            ability_id: TOUCH_OF_GOOD_ABILITY_ID.to_owned(),
            active_state: ActiveState::EquippedActive,
            rounds_consumed_today: None,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Cleric Touch of Good entry: \
             {:?}",
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
            "a non-Cleric character's spoofed Touch of Good entry must never apply a bonus: \
             {melee_attack_bonus:?}"
        );
    }
}

