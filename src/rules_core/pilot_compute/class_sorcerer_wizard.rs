#[allow(unused_imports)]
pub(crate) use super::*;

// SD13-E4-F7 spell-bearing baseline identity. Sorcerer is a spontaneous full arcane
// caster; this slice recognizes only its bounded single-class level-1 identity as direct
// runtime evidence and grounds no bloodline power and no spell math (spell slots, spells
// known, spell DCs, bonus spells, or prepared posture) for it. A further SD13-E5 slice
// widens the level-1-only gate to a level-range gate (`supported_sorcerer_level`,
// 1..=MAX_SUPPORTED_SORCERER_LEVEL), mirroring the Fighter/Paladin/Rogue/Barbarian/
// Monk/Cleric/Bard/Druid idiom: the PF1 Core Rulebook Sorcerer class table's level-2
// "Special" column is blank (verified against d20pfsrd and legacy.aonprd.com), so no new
// class feature is gained at 2nd level, unlike Rogue/Monk/Druid's Evasion/Woodland
// Stride — this widening extends the existing pillars only, adding no new one. A further
// SD13-E5 slice widens the gate again to level 3: the PF1 Core Rulebook Sorcerer class
// table's level-3 "Special" column reads "Bloodline power, bloodline spell" (verified
// against d20pfsrd and legacy.aonprd.com), NOT blank like level 2 — but both entries are
// bloodline-specific (they name a different power/spell per bloodline, e.g. the Arcane
// bloodline's own 3rd-level power is Metamagic Adept and its 3rd-level bloodline spell is
// Identify) and neither is flat/identity-shaped the way Rogue's Trap Sense or Monk's Still
// Mind are, so no new pillar record is added for level 3 either — both entries stay named
// by the pre-existing `arcane_bond_and_bloodline_progression.unsupported` diagnostic's
// "bonus spells/feats at 3rd+ level" and "bloodline power" language, unchanged. This
// widening extends only the already-grounded base-attack/base-save/bloodline-choice
// pillars to level 3. A further SD13-E5 slice widens the gate again to level 4: the PF1
// Core Rulebook Sorcerer class table's level-4 "Special" column is blank (verified
// independently against d20pfsrd and legacy.aonprd.com), UNLIKE the level-3 row's
// "Bloodline power, bloodline spell" entry, so no new class feature is gained at 4th
// level and no new pillar record is added — this widening extends only the
// already-grounded base-attack/base-save/bloodline-choice/bloodline-class-skill-choice
// pillars to level 4 via the same formulas, without re-derivation. A further SD13-E5
// slice widens the gate again to level 5: the PF1 Core Rulebook Sorcerer class table's
// level-5 "Special" column reads "Bloodline spell" (verified independently against
// d20pfsrd and legacy.aonprd.com), UNLIKE the blank level-4 column, so this was checked
// rather than assumed away — this is the sorcerer's second bloodline spell grant (the
// first came at level 3, alongside the level-3 bloodline power), and the Arcane
// bloodline's own 5th-level bloodline spell is invisibility, but the entry is
// bloodline-specific (it names a different spell per bloodline) and not
// flat/identity-shaped the way Rogue's Trap Sense or Monk's Still Mind are, so no new
// pillar record is added for level 5 either, mirroring exactly how the level-3
// "Bloodline power, bloodline spell" entry was left unproven — this widening extends
// only the already-grounded base-attack/base-save/bloodline-choice/
// bloodline-class-skill-choice pillars to level 5 via the same formulas, without
// re-derivation. A further SD13-E5 slice widens the gate again to level 8: the PF1
// Core Rulebook Sorcerer class table's level-8 "Special" column is blank (verified
// independently against d20pfsrd and legacy.aonprd.com, checked rather than assumed
// away) — like levels 2, 4, and 6, and UNLIKE the level-7 "Bloodline feat, bloodline
// spell" row — so no new class feature is gained at 8th level. The first 4th-level
// spell slots arrive at 8th, but spells per day belong to the spontaneous spell
// burden that stays named-but-unproven, so this widening extends only the
// already-grounded base-attack/base-save/bloodline-choice/
// bloodline-class-skill-choice pillars to level 8 via the same formulas, without
// re-derivation. A further SD13-E5 slice widens the gate again to level 9: the
// PF1 Core Rulebook Sorcerer class table's level-9 "Special" column reads
// "Bloodline power, bloodline spell" (verified independently against d20pfsrd
// and legacy.aonprd.com, checked rather than assumed away) — UNLIKE the blank
// level-8 column — but both entries are bloodline-specific (the Arcane
// bloodline's own 9th-level power is New Arcana and its 9th-level bloodline
// spell is overland flight) and neither is flat/identity-shaped, so no new
// pillar record is added for level 9 either, mirroring exactly how the
// level-3/5/7 bloodline entries were left unproven — this widening extends
// only the already-grounded base-attack/base-save/bloodline-choice/
// bloodline-class-skill-choice pillars to level 9 via the same formulas,
// without re-derivation (poor Fortitude/Reflex both genuinely rise to +3 at
// level 9 while base attack and good Will stay put, integer-division
// coincidences). A further SD13-E5 slice widens the gate again to level 10 —
// the tranche ceiling: the PF1 Core Rulebook Sorcerer class table's level-10
// "Special" column is blank (verified independently against d20pfsrd and
// legacy.aonprd.com, checked rather than assumed away) — like levels 2, 4,
// 6, and 8, and UNLIKE the level-9 "Bloodline power, bloodline spell" row —
// so no new class feature is gained at 10th level; the first 5th-level spell
// slots arrive at 10th but belong to the spontaneous spell burden that stays
// named-but-unproven; this widening extends only the already-grounded
// pillars to level 10 via the same formulas (base attack genuinely rises to
// +5 and good Will genuinely rises to +7, while poor Fortitude/Reflex stay
// +3, integer-division coincidences).
//
// A further SD18 slice widens the gate again to level 11 (verified
// independently against d20pfsrd and the Archives of Nethys aonprd.com
// mirror): level 11 base attack bonus and all three base saves stay
// numerically IDENTICAL to level 10 (`11/2=5`, `11/3=3`, `11/3=3`,
// `11/2+2=7`, integer-division coincidences), the level-11 "Special" column
// reads only "Bloodline spell" (bloodline-specific, left unproven,
// mirroring levels 3/5/7/9), and the already-grounded spells-per-day and
// spells-known table lookups both genuinely widen at level 11 (per-day
// `6/6/6/5/3` -> `6/6/6/6/4`; known `9/5/4/3/2/1` -> `9/5/5/4/3/2`), with
// the 6th-level column staying inaccessible at level 11 (arrives at level
// 12) on both tables.
//
// SD18 cycle-2026-07-15T4400 widens the gate again to level 15 (the
// loop's ninth §3.2 level-15 landing, after Barbarian, Rogue, Fighter,
// Cleric, Druid, Ranger, Wizard, and Paladin), verified independently
// against d20pfsrd and the Archives of Nethys aonprd.com mirror, both
// byte-for-byte identical: base attack bonus (`15/2=7`) and good Will
// (`15/2+2=9`) both stay numerically IDENTICAL to level 14, integer-division
// coincidences, while both poor saves genuinely rise to +5 (`15/3=5`, up
// from level 14's +4). The level-15 "Special" column reads "Bloodline
// power, bloodline spell" — bloodline-specific, left named-but-unproven by
// the pre-existing Arcane Bond / bloodline progression blocker, exactly
// mirroring levels 3/5/7/9/11/13 — so no new pillar is grounded from it.
// The already-grounded spells-per-day and spells-known table lookups both
// genuinely widen at level 15 within their existing array shapes (per-day
// `6/6/6/6/6/5/3` -> `6/6/6/6/6/6/4`; known `9/5/5/4/4/3/2/1` ->
// `9/5/5/4/4/4/3/2`), with no genuinely new spell-level column opening (the
// 8th-level column stays inaccessible through level 15) on either table.
//
// SD18 cycle-2026-07-15T5800 widens the gate again to level 16 (the loop's
// SEVENTH §3.2 level-16 landing, after Barbarian, Fighter, Wizard, Rogue,
// Cleric, and Paladin), verified independently against d20pfsrd and the
// Archives of Nethys aonprd.com mirror, both byte-for-byte identical: base
// attack bonus genuinely rises to +8 (`16/2=8`, up from level 15's +7) and
// good Will genuinely rises to +10 (`16/2+2=10`, up from level 15's +9),
// while both poor saves stay numerically IDENTICAL to level 15 at +5
// (`16/3=5`, an integer-division coincidence). The level-16 "Special"
// column is genuinely BLANK on both primary sources — UNLIKE level 15's
// "Bloodline power, bloodline spell" entry, this is a pure ceiling raise
// with no bloodline-specific text left named-but-unproven. The
// already-grounded spells-per-day and spells-known table lookups both
// genuinely widen at level 16, each opening a genuinely NEW 8th-level
// column for the first time (per-day `6/6/6/6/6/6/4` -> `6/6/6/6/6/6/5/3`;
// known `9/5/5/4/4/4/3/2` -> `9/5/5/4/4/4/3/2/1`), so both arrays widen
// from their prior 7/8-element shapes to 8/9-element shapes via a new
// `SORCERER_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 16` threshold
// constant, mirroring the Wizard's own 8th-level-column-opening cycle. The
// spell-level access ladder genuinely rises to 8 (up from 7 at level 15).
//
// SD18 cycle-2026-07-15T14100 widens the gate again to level 17 (the
// loop's NINTH §3.2 level-17 landing, after Ranger, Bard, Rogue, Fighter,
// Wizard, Cleric, Paladin, and Barbarian, and closing the level-17 sweep
// at 9 of 10 non-Monk classes), verified independently against d20pfsrd
// and the Archives of Nethys aonprd.com mirror, fetching the full
// levels-15-through-19 class-table block (including the separate Spells
// Known table) in one pass to rule out level-misattribution — both
// sources agreed byte-for-byte on all five rows, so a third source was
// not required: base attack bonus STAYS at +8 (`17/2=8`) and both poor
// saves STAY at +5 (`17/3=5`) and good Will STAYS at +10 (`17/2+2=10`),
// all integer-division coincidences with level 16. The level-17
// "Special" column reads "Bloodline spell" — bloodline-specific, left
// named-but-unproven by the pre-existing Arcane Bond / bloodline
// progression blocker, exactly mirroring levels 3/5/7/9/11/13/15 — so no
// new pillar is grounded from it. The already-grounded spells-per-day and
// spells-known table lookups both genuinely widen at level 17 within
// their already-widened 8/9-element shapes (per-day `6/6/6/6/6/6/5/3` ->
// `6/6/6/6/6/6/6/4`; known `9/5/5/4/4/4/3/2/1` -> `9/5/5/4/4/4/3/3/2`),
// with no genuinely new spell-level column opening (the 8th-level column
// already opened at level 16). The spell-level access ladder STAYS at 8
// (unchanged from level 16; no new threshold constant is needed).
//
// SD18 cycle-2026-07-16T0400 widens the gate again to level 18 (the loop's
// EIGHTH §3.2 level-18 landing, after Wizard, Cleric, Paladin, Fighter,
// Barbarian, Rogue, and Ranger). This cycle's primary task was resolving a
// multi-cycle-carried-forward flag: prior cycles repeatedly noted that a raw
// Sorcerer spells-per-day fetch looked internally inconsistent at level 18
// (an apparent "premature" 9th-level spell column), suspected as a tool
// artifact against a commonly-repeated folk-rule that sorcerers gain
// 9th-level spells only at 20th level. This cycle re-fetched Sorcerer's full
// levels 14-20 block fresh from THREE independent primary sources: a raw,
// non-AI-summarized parse of d20pfsrd.com's own HTML table (bypassing any
// tabular-summarization ambiguity entirely), the Archives of Nethys
// aonprd.com mirror, and the legacy.aonprd.com CRB mirror. All three agree
// byte-for-byte: the Sorcerer's spells-per-day table opens a genuinely NEW
// spell-level column every two class levels starting at 4th (2nd at 4, 3rd
// at 6, 4th at 8, 5th at 10, 6th at 12, 7th at 14, 8th at 16, 9th at 18 —
// exactly matching this row's own already-grounded and already-verified
// SORCERER_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL through
// SORCERER_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL thresholds), with
// 1st-level spells available from level 1 (no zero step). The 9th-level
// column therefore genuinely, correctly opens at class level 18 — ONE LEVEL
// EARLIER than Wizard/Cleric's own already-grounded
// WIZARD_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL /
// CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL threshold of 17, which is
// exactly consistent with every other already-proven Sorcerer threshold in
// this row being one level later than Wizard's own equivalent threshold
// (e.g. Wizard's 8th-level threshold is 15, Sorcerer's is 16). The
// previously-flagged "premature 9th-level column" was therefore the CORRECT
// reading all along; the folk-rule assuming a 20th-level-only 9th-spell-level
// threshold for Sorcerer never held for this class and was never
// independently re-verified against a primary source in any prior cycle,
// only carried forward by analogy. With the row now definitively resolved,
// this slice widens the level-range gate again (supported_sorcerer_level,
// 1..=18) and extends every one of the base-attack/base-save formulas above
// to level 18 via the same formula, without re-derivation: level 18 base
// attack bonus genuinely rises to +9 (18/2, up from level 17's +8) and both
// poor saves genuinely rise to +6 (18/3, up from level 17's +5) and good
// Will genuinely rises to +11 (18/2+2, up from level 17's +10); the
// bloodline choice and bloodline class-skill choice recognitions are not
// level-gated, so both still fire at level 18 for the same fixture
// selections; the PF1 Core Rulebook Sorcerer class table's level-18
// "Special" column is genuinely BLANK on all three sources — UNLIKE level
// 17's "Bloodline spell" entry — so no new pillar is grounded from the
// Special column; the already-grounded base spells-per-day table genuinely
// widens (6/6/6/6/6/6/6/4 -> 6/6/6/6/6/6/6/5/3, the 8th-level column rising
// by one AND a genuinely NEW 9th-level column opening at 3) and the
// already-grounded base spells-known table genuinely widens
// (9/5/5/4/4/4/3/3/2 -> 9/5/5/4/4/4/3/3/2/1, the 0th-8th columns staying
// numerically unchanged while a genuinely NEW 9th-level column opens at 1);
// the spell-level access ladder genuinely rises to 9 (up from 8 at level 17)
// via a new SORCERER_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 18 threshold
// constant, mirroring the Wizard's and Cleric's own
// 9th-level-column-opening cycles; the spell-save-DC and
// Charisma-bonus-spell formulas widen automatically over the newly-risen
// access ladder, with no new code needed. The row stays Partial, not
// Supported: the Arcane Bond / bloodline progression burden and the
// spontaneous which-spells-known / casting-execution burden remain named and
// unproven, unchanged from level 17. No spell math is fabricated and no
// Sorcerer level 19+ is proven.
// SD18 cycle-2026-07-16T4900 widens the gate again to level 19 (the loop's
// EIGHTH §3.2 level-19 landing, after Barbarian, Cleric, Fighter, Bard,
// Paladin, Ranger, and Rogue), verified independently against TWO primary
// sources (a raw non-AI-summarized parse of d20pfsrd.com's own HTML table
// and the Archives of Nethys aonprd.com mirror, both byte-for-byte
// identical, fetching the full levels-15-through-20 class-table block in
// one pass to rule out level-misattribution; no disagreement was found, so
// a third source was not required): level 19 base attack bonus STAYS at +9
// (19/2, an integer-division coincidence with level 18's +9, confirmed
// genuine by the raw table's own `+9/+4` cell matching level 18's `+9/+4`
// cell exactly) and both poor saves STAY at +6 (19/3) and good Will STAYS
// at +11 (19/2+2), all integer-division coincidences with level 18, not a
// sign any formula stopped scaling; the bloodline choice and bloodline
// class-skill choice recognitions are not level-gated, so both still fire
// at level 19 for the same fixture selections; the PF1 Core Rulebook
// Sorcerer class table's level-19 "Special" column reads "Bloodline feat,
// bloodline spell" — bloodline-specific, left named-but-unproven by the
// pre-existing Arcane Bond / bloodline progression blocker, exactly
// mirroring levels 3/5/7/9/11/13/15/17 — so no new pillar is grounded from
// it; the already-grounded base spells-per-day table genuinely widens
// (6/6/6/6/6/6/6/5/3 -> 6/6/6/6/6/6/6/6/4, the 8th-level column rising by
// one AND the 9th-level column rising by one, with no genuinely new
// spell-level column opening) and the already-grounded base spells-known
// table genuinely widens (9/5/5/4/4/4/3/3/2/1 -> 9/5/5/4/4/4/3/3/3/2, the
// 0th-7th columns staying numerically unchanged while the 8th-level column
// rises by one AND the 9th-level column rises by one); the spell-level
// access ladder STAYS at 9 (unchanged from level 18; the ladder was already
// fully populated through 9th-level spells, so no new threshold constant is
// needed); the spell-save-DC and Charisma-bonus-spell formulas widen
// automatically over the unchanged access ladder, with no new code needed.
// The row stays Partial, not Supported: the Arcane Bond / bloodline
// progression burden and the spontaneous which-spells-known /
// casting-execution burden remain named and unproven, unchanged from level
// 18. No spell math is fabricated and no Sorcerer level 20 is proven.
pub(super) const SORCERER_CLASS_ID: &str = "class:sorcerer";

pub(super) const MAX_SUPPORTED_SORCERER_LEVEL: u8 = 20;

/// The sorcerer level at which 2nd-level sorcerer spells first become
/// available, verified against the raw PF1 Core Rulebook Sorcerer
/// spells-per-day table rows (d20pfsrd and legacy.aonprd.com, identical):
/// level 3 shows "5/—/…", level 4 shows "6/3/—/…" — the first non-"—"
/// 2nd-level column. Like the Bard and unlike the Paladin, the Sorcerer
/// table has NO "0" spells-per-day entries at levels 1-10; 1st-level
/// sorcerer spells are available from level 1 ("3/—/…"), so the ladder has
/// no zero step and no 1st-level threshold const is needed.
pub(super) const SORCERER_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 4;

/// The sorcerer level at which 3rd-level sorcerer spells first become
/// available, verified against the raw table rows (both sources): level 5
/// shows "6/4/—/…", level 6 shows "6/5/3/—/…" — the first non-"—" 3rd-level
/// column. This is the sorcerer's two-level cadence (4/6/8/10), not the
/// bard's three-level one (4/7/10).
pub(super) const SORCERER_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 6;

/// The sorcerer level at which 4th-level sorcerer spells first become
/// available, verified against the raw table rows (both sources): level 7
/// shows "6/6/4/—/…", level 8 shows "6/6/5/3/—/…" — the first non-"—"
/// 4th-level column.
pub(super) const SORCERER_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 8;

/// The sorcerer level at which 5th-level sorcerer spells first become
/// available, verified against the raw table rows (both sources): level 9
/// shows "6/6/6/4/—/…", level 10 shows "6/6/6/5/3/—/…" — the first non-"—"
/// 5th-level column.
pub(super) const SORCERER_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 10;

/// The sorcerer level at which 6th-level sorcerer spells first become
/// available, verified against the raw table rows of all three primary-
/// source fetches this SD18 slice performed (d20pfsrd, aonprd.com, and
/// legacy.aonprd.com, all identical): level 11 shows "6/6/6/6/4/—", level 12
/// shows "6/6/6/6/5/3" — the first non-"—" 6th-level column. This is the
/// sorcerer's two-level cadence continuing exactly (4/6/8/10/12), one spell
/// level deeper than the tranche's prior ceiling.
pub(super) const SORCERER_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 12;

/// The sorcerer level at which 7th-level sorcerer spells first become
/// available, verified against the raw table rows of two independent
/// primary-source fetches this SD18 slice performed (d20pfsrd and
/// legacy.aonprd.com, identical): level 13 shows "6/6/6/6/6/4/—", level 14
/// shows "6/6/6/6/6/5/3" — the first non-"—" 7th-level column (a third
/// fetch, aonprd.com, was internally inconsistent with the already-landed
/// level-13 truth on this same table and was rejected as a tool artifact,
/// not treated as a genuine conflict). This is the sorcerer's two-level
/// cadence continuing exactly (4/6/8/10/12/14), one spell level deeper than
/// the tranche's prior ceiling.
pub(super) const SORCERER_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 14;

/// The sorcerer level at which 8th-level sorcerer spells first become
/// available, verified against the raw table rows of both primary-source
/// fetches this SD18 slice performed (d20pfsrd and the Archives of Nethys
/// aonprd.com mirror, byte-for-byte identical): level 15 shows
/// "6/6/6/6/6/6/4/—", level 16 shows "6/6/6/6/6/6/5/3" — the first non-"—"
/// 8th-level column. This is the sorcerer's two-level cadence continuing
/// exactly (4/6/8/10/12/14/16), one spell level deeper than the tranche's
/// prior ceiling, mirroring the Wizard's own
/// `WIZARD_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` threshold idiom.
pub(super) const SORCERER_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 16;

/// The sorcerer level at which 9th-level sorcerer spells first become
/// available, definitively resolved this SD18 slice (cycle-2026-07-16T0400)
/// against THREE independent primary-source fetches (a raw, non-AI-
/// summarized parse of d20pfsrd.com's own HTML table; the Archives of
/// Nethys aonprd.com mirror; and the legacy.aonprd.com CRB mirror, all
/// byte-for-byte identical): level 17 shows "6/6/6/6/6/6/6/4/—", level 18
/// shows "6/6/6/6/6/6/6/5/3" — the first non-"—" 9th-level column. This is
/// the sorcerer's two-level cadence continuing exactly
/// (4/6/8/10/12/14/16/18), one level EARLIER than the Wizard's own
/// `WIZARD_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 17` threshold — exactly
/// consistent with every other Sorcerer threshold in this ladder being one
/// level behind Wizard's equivalent. This resolves a flag repeatedly carried
/// forward across multiple prior SD18 cycles, which suspected (but never
/// independently re-verified) that a 9th-level column appearing at 18 was a
/// tool artifact contradicting a folk-rule that Sorcerer 9th-level spells
/// arrive only at 20th level; that folk-rule does not hold for this class.
pub(super) const SORCERER_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 18;

// SD13-E5 canonical Sorcerer bloodline choice seam. The deterministic fixture names the
// Arcane bloodline as its chosen selection; the compute seam recognizes exactly that
// chosen input. Recognition only: the Arcane bloodline's level-1 power is Arcane Bond
// (a familiar or a bonded object — an execution engine, not a flat number), so no
// power value is ever fabricated from this choice.
pub(super) const SORCERER_BLOODLINE_CHOICE_ID: &str = "choice:sorcerer_bloodline";

/// PF1 Core Rulebook Draconic bloodline's 3rd-level "Dragon Resistances" power
/// (corpus: cr_abilities_class.lst, KEY:Draconic Bloodline ~ Dragon Resistances,
/// PREVARGTEQ:Sorcerer_Draconic_BloodlineProgressionLVL,3).
pub(super) const SORCERER_DRACONIC_DRAGON_RESISTANCES_LEVEL: u8 = 3;

pub(super) const SORCERER_DRACONIC_DRAGON_RESISTANCES_NATURAL_ARMOR_EXPLANATION_ID: &str =
    "class_feature.sorcerer.draconic_bloodline.dragon_resistances.natural_armor_bonus";
pub(super) const SORCERER_DRACONIC_DRAGON_RESISTANCES_RESISTANCE_EXPLANATION_ID: &str =
    "class_feature.sorcerer.draconic_bloodline.dragon_resistances.resistance_bonus";
pub(super) const SORCERER_DRACONIC_DRAGON_RESISTANCES_ENERGY_TYPE_UNRESOLVED_DIAGNOSTIC_ID: &str =
    "class_feature.sorcerer.draconic_bloodline.dragon_resistances.energy_type_unresolved";

/// v0.6 alpha swarm, risks item 8 (Sorcerer Arcane bloodline closure): the
/// Arcane bloodline's 1st-level power, Arcane Bond ("you gain an arcane
/// bond, as a wizard... Once per day, your bonded item allows you to cast
/// any one of your spells known"), grants a choice between a familiar and
/// a bonded item, verified independently against two primary sources.
/// Recognition only: the "cast a spell known" half of this benefit
/// requires a spell-casting-resolution engine that exists nowhere in this
/// codebase, for any class (confirmed by direct inspection) -- so no
/// power value beyond the chosen bond type and its flat 1/day budget is
/// ever fabricated from this choice.
pub(super) const SORCERER_ARCANE_BOND_CHOICE_ID: &str = "choice:sorcerer_arcane_bond";

/// SD-34 AT-34-E3-001 (`decisions.md §16`, "only the count grounds" is
/// ratified precedent). The full corpus-wide union of every named
/// `"Sorcerer Bloodline Feat ~ <X>"` option this engine's `class_feature`
/// corpus carries, across every PF1 Core Rulebook Sorcerer bloodline
/// (Aberrant, Abyssal, Arcane, Celestial, Destined, Draconic, Elemental,
/// Fey, Infernal, Undead) — verified against
/// `data/corpus/core_rulebook/class_feature/sorcerer_bloodline_feat/*.json`
/// (87 entries with this exact evidence shape; four further corpus keys —
/// Deadly Aim, Spell Focus, Toughness, Weapon Focus — fail the owner match
/// at a different corpus-key group entirely and are out of this list's
/// scope, unaffected by it).
///
/// This engine recognizes only the Arcane bloodline as chosen input
/// (`ARCANE_BLOODLINE_SELECTION_ID`); `ground_sorcerer_bloodline_feat_pool`
/// below deliberately does not require that recognition, because the SLOT
/// COUNT this pool grants is bloodline-invariant (every CRB bloodline
/// shares the identical `BONUS:VAR|BloodlineFeatCount|
/// (BloodlineFeatProgression-1)/6` formula; only the ELIGIBLE SET differs
/// per bloodline, and this seam cannot narrow to one bloodline it does not
/// recognize). Naming the full corpus-wide union here is a superset of any
/// single bloodline's real list, never a fabricated one — every named
/// option really is eligible for SOME CRB bloodline.
pub(super) const SORCERER_BLOODLINE_FEAT_POOL_ELIGIBLE_FEATS: &[&str] = &[
    "Acrobatic Steps",
    "Agile Maneuvers",
    "Alertness",
    "Arcane Armor Mastery",
    "Arcane Armor Training",
    "Arcane Strike",
    "Armor Proficiency (Light)",
    "Augment Summoning",
    "Blind-Fight",
    "Cleave",
    "Combat Casting",
    "Combat Expertise",
    "Combat Reflexes",
    "Craft Rod",
    "Craft Staff",
    "Craft Wondrous Item",
    "Deceitful",
    "Defensive Combat Training",
    "Deft Hands",
    "Diehard",
    "Dodge",
    "Empower Spell",
    "Endurance",
    "Enlarge Spell",
    "Extend Spell",
    "Far Shot",
    "Fleet",
    "Forge Ring",
    "Great Fortitude",
    "Greater Spell Focus (Enchantment)",
    "Heighten Spell",
    "Improved Bull Rush",
    "Improved Counterspell",
    "Improved Disarm",
    "Improved Feint",
    "Improved Grapple",
    "Improved Great Fortitude",
    "Improved Initiative",
    "Improved Iron Will",
    "Improved Overrun",
    "Improved Sunder",
    "Improved Unarmed Strike",
    "Intimidating Prowess",
    "Iron Will",
    "Leadership",
    "Lightning Reflexes",
    "Magical Aptitude",
    "Martial Weapon Proficiency",
    "Maximize Spell",
    "Mobility",
    "Mounted Combat",
    "Nimble Moves",
    "Persuasive",
    "Point-Blank Shot",
    "Power Attack",
    "Precise Shot",
    "Quick Draw",
    "Quicken Spell",
    "Ride-By Attack",
    "Scribe Scroll",
    "Silent Spell",
    "Skill Focus (Acrobatics)",
    "Skill Focus (Bluff)",
    "Skill Focus (Craft)",
    "Skill Focus (Disguise)",
    "Skill Focus (Fly)",
    "Skill Focus (Intimidate)",
    "Skill Focus (Knowledge (Arcana))",
    "Skill Focus (Knowledge (Dungeoneering))",
    "Skill Focus (Knowledge (Engineering))",
    "Skill Focus (Knowledge (History))",
    "Skill Focus (Knowledge (Nature))",
    "Skill Focus (Knowledge (Planes))",
    "Skill Focus (Knowledge (Religion))",
    "Skill Focus (Perception)",
    "Skill Focus (Perform)",
    "Skill Focus (Sense Motive)",
    "Skill Focus (Stealth)",
    "Skill Focus (Swim)",
    "Spell Focus (Enchantment)",
    "Spell Focus (Necromancy)",
    "Spell Penetration",
    "Stealthy",
    "Still Spell",
    "Weapon Finesse",
    "Widen Spell",
    "Wind Stance",
];

/// Ten of `SORCERER_BLOODLINE_FEAT_POOL_ELIGIBLE_FEATS`' own names contain,
/// as a literal substring of their `class_feature_engine_join_slug`, a
/// SHORT, unrelated corpus record's own exact slug for the SAME owner
/// (`sorcerer`) — verified empirically by regenerating `docs/work-inventory.
/// json`, diffing before/after this pool's diagnostics first shipped, AND a
/// second corpus-wide cross-check (every `sorcerer`-/`ranger`-owned
/// `class_feature` record in the whole corpus, not only this book's bucket
/// B) before trusting the exclusion list complete: all seven
/// `"Skill Focus (Knowledge (<X>))"` entries → slug contains `"knowledge"`
/// (`Sorcerer Domain ~ Knowledge`'s own slug — the FIRST regeneration only
/// excluded the Arcana entry and left `Dungeoneering`/`Engineering`/
/// `History`/`Nature`/`Planes`/`Religion` still colliding, caught by the
/// second regeneration's own before/after diff); `"Improved Sunder"` →
/// contains `"sun"` (`Sorcerer Domain ~ Sun`); `"Skill Focus (Fly)"` →
/// contains `"fly"` (`Sorcerer Bonus Spell L3 ~ Fly`); `"Magical
/// Aptitude"` → contains `"magic"` (`Sorcerer Domain ~ Magic`).
/// `v06_work_inventory.rs::diagnostic_id_names_feature` matches by
/// SUBSTRING within the owner's namespace, not by exact key, so a
/// diagnostic naming any of these ten would misattribute a false "count
/// grounds, choice not modelled" reason to an unrelated domain-power or
/// bonus-spell record — a real correctness defect, not a cosmetic one.
/// (Two further theoretical collisions the cross-check found —
/// `"Sorcerer Bloodline Feat ~ Spell Focus"` against `"Spell Focus
/// (Enchantment)"`/`"(Necromancy)"`/`"Greater Spell Focus (Enchantment)"`,
/// and `"Sorcerer Bloodline ~ Arcane"` against `"Arcane Armor
/// Mastery"`/`"Training"`/`"Arcane Strike"` — are confirmed harmless: the
/// first names an object that genuinely IS a real bloodline-feat-pool
/// member, so the templated message stays true of it; the second is
/// already `grounded` via a different, earlier-resolving mechanism and
/// never reaches this diagnostic check at all.)
///
/// Excluded from the per-option diagnostic loop ONLY: each name stays
/// listed in `SORCERER_BLOODLINE_FEAT_POOL_ELIGIBLE_FEATS` above (a real,
/// honestly-named eligible feat), so the count explanation's eligible-set
/// size is unaffected. Only these ten feats' OWN corpus records
/// (`Sorcerer Bloodline Feat ~ <name>`) stay correctly unclaimed
/// (`engine-does-not-hold`) rather than closing via a false attribution.
pub(super) const SORCERER_BLOODLINE_FEAT_POOL_DIAGNOSTIC_EXCLUSIONS: &[&str] = &[
    "Skill Focus (Knowledge (Arcana))",
    "Skill Focus (Knowledge (Dungeoneering))",
    "Skill Focus (Knowledge (Engineering))",
    "Skill Focus (Knowledge (History))",
    "Skill Focus (Knowledge (Nature))",
    "Skill Focus (Knowledge (Planes))",
    "Skill Focus (Knowledge (Religion))",
    "Improved Sunder",
    "Skill Focus (Fly)",
    "Magical Aptitude",
];

/// Grounds the Sorcerer bloodline feat pool's slot COUNT
/// (`arcane_bloodline_bonus_feat_count`'s own bloodline-invariant formula)
/// and names its full eligible set
/// (`SORCERER_BLOODLINE_FEAT_POOL_ELIGIBLE_FEATS`), without seeding any
/// default choice — the ratified Fighter/Cavalier/Brawler/Arcane-bloodline
/// treatment (`decisions.md §16`): "only the count grounds; which option
/// fills a slot is not modelled". Runs unconditionally for any Sorcerer,
/// independent of `ground_sorcerer_arcane_bloodline_progression`'s own
/// Arcane-only canonical narrowing — this is the one Sorcerer bloodline
/// record deliberately widened past that narrowing, because its magnitude
/// genuinely does not vary by bloodline.
///
/// One non-claim-blocking diagnostic is emitted per eligible option,
/// carrying the SAME templated message regardless of which option it
/// names — deliberately, not an oversight: two of this list's own names
/// are substrings of a third (`"Iron Will"` inside `"Improved Iron
/// Will"`, `"Great Fortitude"` inside `"Improved Great Fortitude"`), so
/// the downstream classifier's own substring-based diagnostic lookup
/// (`v06_work_inventory.rs::diagnostic_id_names_feature`) can legitimately
/// resolve a given corpus record to a different-but-textually-related
/// diagnostic than the one this loop built for its own name. Templating
/// every message identically makes that indeterminacy harmless: whichever
/// diagnostic a lookup lands on, the content it reports is equally true of
/// the record it was asked about.
pub(super) fn ground_sorcerer_bloodline_feat_pool(
    sorcerer_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let slot_count = arcane_bloodline_bonus_feat_count(sorcerer_level);
    let eligible_count = SORCERER_BLOODLINE_FEAT_POOL_ELIGIBLE_FEATS.len();
    let detail = if slot_count == 0 {
        format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR|BloodlineFeatCount|(BloodlineFeatProgression-1)/6|TYPE=Base
            "Sorcerer bloodline feat pool slot count at sorcerer level {sorcerer_level}: none yet, \
             correctly absent by PF1 Core Rulebook level gate (the first slot is granted at sorcerer \
             level {ARCANE_BLOODLINE_FIRST_BONUS_FEAT_LEVEL}). This formula is IDENTICAL across \
             every PF1 Core Rulebook Sorcerer bloodline, not specific to Arcane"
        )
    } else {
        format!(
            "Sorcerer bloodline feat pool slot count at sorcerer level {sorcerer_level}: \
             {slot_count} slot(s) granted ((sorcerer level - 1)/6 -- one at 7th, 13th, and \
             19th). This formula is IDENTICAL across every PF1 Core Rulebook Sorcerer \
             bloodline, not specific to Arcane -- it grounds regardless of which bloodline \
             (or none this seam recognizes) the character chose. Only the COUNT grounds; \
             which of the {eligible_count} corpus-wide eligible feats fills each slot is a \
             player choice this bounded seam does not model, the ratified Fighter/Cavalier/\
             Brawler/Arcane-bloodline treatment"
        )
    };
    explanations.push(ComputationExplanation {
        id: "class_feature.sorcerer.bloodline_feat_pool.slot_count".to_owned(),
        value: slot_count,
        detail,
    });
    if slot_count == 0 {
        return;
    }
    for feat in SORCERER_BLOODLINE_FEAT_POOL_ELIGIBLE_FEATS {
        if SORCERER_BLOODLINE_FEAT_POOL_DIAGNOSTIC_EXCLUSIONS.contains(feat) {
            continue;
        }
        diagnostics.push(ComputationDiagnostic {
            id: format!(
                "class_feature.sorcerer.bloodline_feat_pool.option.{}.not_modelled",
                slugify_id_segment(feat)
            ),
            message: format!(
                "Sorcerer bloodline feat pool at sorcerer level {sorcerer_level}: the slot \
                 count above is grounded, but WHICH of the {eligible_count} corpus-wide \
                 eligible feats (drawn from every PF1 Core Rulebook Sorcerer bloodline's own \
                 feat list) fills any given slot is a player choice not resolved on this \
                 bounded seam; no default feat is fabricated for any slot"
            ),
            claim_blocking: false,
        });
    }
}

/// School Power's spell save DC bonus, from the corpus record's only numeric
/// token: `BONUS:DC|SCHOOL.%LIST|2|TYPE=SchoolPower`.
pub(super) const SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_BONUS: i16 = 2;

pub(super) const SORCERER_ARCANE_BLOODLINE_BONUS_SPELLS_EXPLANATION_ID: &str =
    "class_feature.sorcerer.arcane_bloodline.bonus_spells_known";
pub(super) const SORCERER_ARCANE_BLOODLINE_BONUS_FEAT_COUNT_EXPLANATION_ID: &str =
    "class_feature.sorcerer.arcane_bloodline.bonus_feat_count";
pub(super) const SORCERER_ARCANE_BLOODLINE_METAMAGIC_ADEPT_EXPLANATION_ID: &str =
    "class_feature.sorcerer.arcane_bloodline.metamagic_adept_uses_per_day";
pub(super) const SORCERER_ARCANE_BLOODLINE_NEW_ARCANA_EXPLANATION_ID: &str =
    "class_feature.sorcerer.arcane_bloodline.new_arcana_spell_count";
pub(super) const SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_EXPLANATION_ID: &str =
    "class_feature.sorcerer.arcane_bloodline.school_power_spell_dc_bonus";
pub(super) const SORCERER_ARCANE_BLOODLINE_ARCANE_APOTHEOSIS_EXPLANATION_ID: &str =
    "class_feature.sorcerer.arcane_bloodline.arcane_apotheosis";
pub(super) const SORCERER_ARCANE_BLOODLINE_SUBCHOICES_DIAGNOSTIC_ID: &str =
    "class_feature.sorcerer.arcane_bloodline.progression_subchoices_unresolved";

// SD13-E5 Arcane bloodline class-skill choice seam. The PF1 Core Rulebook Arcane
// bloodline entry reads "Class Skill: Knowledge (any one)" (verified against both
// d20pfsrd and the legacy Paizo PRD mirror) — a player's choice of any one Knowledge
// skill, not a fixed grant of Knowledge (arcana) specifically. This choice-slot
// selection is recognized only when the Arcane bloodline itself was recognized above,
// since this class-skill grant belongs to that bloodline.
pub(super) const SORCERER_BLOODLINE_CLASS_SKILL_CHOICE_ID: &str = "choice:sorcerer_bloodline_class_skill";

/// v0.6 alpha swarm, risks item 8 (Arcanist full-build closure, first
/// non-CRB class attempting real `Computed` status): APG/ACG Arcanist,
/// verified directly against `acg_abilities_class.lst`'s own `Arcanist ~
/// Spells Prepared`/`Arcane Reservoir` records. Unlike every prior ACG/
/// APG closure this session (a single named feature, spellcasting
/// deferred), this closure builds Arcanist's REAL prepared-spellbook
/// spellcasting from scratch, mirroring `unmet_wizard_spellbook_conditions`/
/// `ground_wizard_prepared_spellbook`'s own shape -- confirmed via two
/// independent sources (the raw corpus `BONUS:VAR` formulas, hand-
/// evaluated at levels 1-5, and legacy.aonprd.com's own printed "Table:
/// Arcanist Spells Prepared") that Arcanist's own per-day counts and
/// access ladder are genuinely DIFFERENT from Wizard's (4/2 at level 1
/// vs Wizard's 3/1; 2nd-level spells at Arcanist level 4 vs Wizard level
/// 3) even though the spell-list CONTENT and casting SHAPE (prepared,
/// spellbook-gated, `SPELLLIST:1|Wizard`) are genuinely shared -- so
/// `arcanist_base_spells_per_day` is a real, independently-verified
/// parallel table, not a byte-identical reuse the way Skald's own tables
/// turned out to be for Bard. Arcanist also has NO arcane-school/
/// specialization mechanic at all (confirmed: no "School" record
/// anywhere in its own `KEY:Arcanist ~ ...` list), so its own spellbook
/// validation needs no opposed-school gate or per-school slot-cost
/// multiplier the way Wizard's own `wizard_opposed_school_slot_cost`
/// does -- genuinely simpler than Wizard's own validation in that one
/// respect. See `docs/release/v0.6/arcanist-acg-full-build-scoping.md`
/// for the full corpus verification and scope record.
pub(super) const ARCANIST_CLASS_ID: &str = "class:arcanist";

/// Mirrors `WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL` exactly, including its
/// v0.6 widening: this table originally verified Arcanist's own spells-
/// prepared table for levels 1-3 only and deferred levels 4-20 to a later
/// cycle. That cycle has now run -- the full 1-20 table is derived from
/// the corpus's own `BONUS:VAR` formulas (see
/// `arcanist_base_spells_per_day`), so no legal class level is refused for
/// lack of a verified row any more.
pub(super) const ARCANIST_SPELLBOOK_SUPPORTED_MAX_LEVEL: u8 = 20;

/// v0.6 alpha swarm, risks item 8 (Arcanist Metamagic Knowledge Exploit
/// closure, follow-on to the Arcanist full-build closure): of Arcanist's
/// 46 real `KEY:Arcanist Exploit ~ ...` records, Metamagic Knowledge is
/// the one genuinely different from the other 45 -- verified directly
/// against its own raw corpus record: no `BONUS:VAR` reservoir-cost tag
/// at all (it uses `BONUS:ABILITYPOOL` instead), unlike every other
/// Exploit checked (Quick Study/Potent Magic/Fast Healing/See Magic all
/// explicitly "expend one point from your arcane reservoir"). It is a
/// one-time bonus-feat GRANT, not an activation-gated, Reservoir-
/// consuming ability -- the same shape as Fighter's own bonus feat
/// choice, not the Rage-shaped/Judgment-shaped activation pattern.
/// Reuses `feat_prereqs::metamagic::evaluate_metamagic_feat_prerequisites`/
/// `resolve_metamagic_feat_effect` directly (a real, already-built,
/// already-tested module from an earlier, unrelated SD-20 Epic 3 cycle)
/// to validate the chosen feat against the real CRB Metamagic feat
/// catalog -- genuine reuse, not new validation logic. `Empower Spell`
/// is the canonical MVP: verified directly against `cr_feats.lst` to
/// carry zero `PREREQ:` token of any kind (no CRB Metamagic feat does,
/// per PF1 rules -- this isn't an arbitrary pick among ambiguous
/// options). Confirmed before building: `push_arcanist_exploits_deferred_diagnostic`
/// is the ONLY claim-blocking diagnostic left once a valid spellbook
/// posture exists, so narrowing it to recognize Metamagic Knowledge
/// genuinely closes the last claim-blocking gap -- Arcanist reaches
/// real `HeadlessReceiptStatus::Computed` for the first time among all
/// ACG/APG classes this session, in a headless test fixture. This is
/// explicitly NOT the same as product-reachability: no picker exists in
/// the real character-creation UI for `choice:arcanist_metamagic_knowledge`
/// either, the same Path A gap Sorcerer/Cleric/Druid/Wizard's own
/// `compose_character_input` seeding closed -- a genuine, separate
/// follow-on, not assumed free here. See
/// `docs/release/v0.6/arcanist-metamagic-knowledge-exploit-scoping.md`
/// for the full record.
pub(super) const ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID: &str = "choice:arcanist_metamagic_knowledge";

/// Translates an `ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID` `selection_id`
/// -- a `metamagic:<snake_case_slug>`-namespaced value satisfying
/// `local_store.rs`'s save-layer colon-segment requirement -- into the
/// real CRB Metamagic feat name `feat_prereqs::metamagic` expects (e.g.
/// `metamagic:empower_spell` -> `Empower Spell`). Every real CRB
/// Metamagic feat is a simple Title Case "Word Spell" phrase, so this is
/// a generic, reversible slug transform, not a hardcoded per-feat lookup
/// table -- it recognizes ANY real Metamagic feat named this way,
/// mirroring this closure's own "don't restrict recognition to Empower
/// Spell alone" design intent (see `EMPOWER_SPELL_METAMAGIC_SELECTION`'s
/// own doc comment for the real save-time bug this fixes). Returns `None`
/// when `selection_id` doesn't carry the `metamagic:` namespace prefix at
/// all (a spoofed/foreign choice, not a translation failure).
pub(super) fn arcanist_metamagic_knowledge_feat_name(selection_id: &str) -> Option<String> {
    let slug = selection_id.strip_prefix("metamagic:")?;
    if slug.is_empty() {
        return None;
    }
    Some(
        slug.split('_')
            .filter(|word| !word.is_empty())
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
    )
}

// Grounded SD13-E4-R3 Human Wizard level-1 prepared arcane spell-bearing baseline
// identities. The Wizard class is the canonical PF1 prepared arcane full caster;
// its class identity differs from Sorcerer in two ways that this bounded slice
// surfaces explicitly: the prepared posture (spellbook + spells prepared per day +
// spell slots per day) and the school specialization (one school chosen, two
// opposed schools locked, specialty school bonus at later levels).
pub(crate) const WIZARD_CLASS_ID: &str = "class:wizard";

// SD13-E5 Wizard level-2/level-3/level-4/level-5/level-6 progression widening:
// mirrors the Fighter `supported_fighter_level` / Paladin `supported_paladin_level` /
// Rogue `supported_rogue_level` / Barbarian `supported_barbarian_level` / Monk
// `supported_monk_level` / Cleric `supported_cleric_level` / Bard
// `supported_bard_level` / Druid `supported_druid_level` / Sorcerer
// `supported_sorcerer_level` idiom (an `Option<u8>` level-range gate) rather than a
// boolean level-1-only check. Verified against the PF1 Core Rulebook Wizard class
// table (d20pfsrd and a second independent Archives of Nethys mirror): the level-2,
// level-3, level-4, AND level-6 "Special" columns are all blank, so no new class
// feature is gained at 2nd, 3rd, 4th, or 6th level (like Cleric/Sorcerer's level-2
// gate, unlike Rogue/Monk/Druid's Evasion/Woodland Stride or Rogue/Monk/Barbarian's
// own 3rd-level features); the level-5 "Special" column reads "Bonus feat" — a
// genuinely NEW class feature, verified rather than assumed, but checked and
// confirmed NOT flat (a choice among an open-ended set of metamagic feats, item
// creation feats, or Spell Mastery — a general feat-selection/feat-prerequisite
// engine, mirroring the Monk High Jump precedent exactly), so it is deliberately left
// named-but-unproven and grounds no record; the specialist bonus slot flat count DOES
// change at level 3 (see `explain_wizard_level1_prepared_spell_baseline`), since a
// level-3 wizard casts 2nd-level spells for the first time, STAYS at that same value
// through level 4 (3rd-level wizard spells do not become available until level 5,
// verified independently against both primary sources' raw spells-per-day table
// rows), DOES change again for real at level 5 (a level-5 wizard casts 3rd-level
// spells for the first time, so the specialist bonus slot count becomes 3: one bonus
// slot of each of 1st/2nd/3rd spell level), then STAYS at 3 through level 6
// (4th-level wizard spells do not become available until level 7, verified
// independently against both primary sources' raw spells-per-day table rows); the
// Intense Spells bonus-damage magnitude DOES change at level 4 (half wizard level,
// minimum 1, reaches 2 for the first time via the pre-existing formula), STAYS at 2
// through level 5 (`max(5/2, 1) = 2`, an integer-division coincidence, not a formula
// that stopped scaling), then DOES change again for real at level 6
// (`max(6/2, 1) = 3`, up from 2 at level 5, via the same pre-existing formula, not
// re-derived).
//
// A further SD13-E5 slice widens the gate again to level 7
// (`MAX_SUPPORTED_WIZARD_LEVEL = 7`): base attack bonus and all three base saves
// are numerically UNCHANGED from level 6 (`7/2 = 3`, `7/3 = 2`, `7/2+2 = 5`), an
// integer-division coincidence re-verified against the raw PF1 CRB Wizard class
// table rather than assumed; the specialist bonus slot flat count GENUINELY RISES
// to 4 (the raw spells-per-day table's level-7 row is "4/4/3/2/1", the first
// non-"—" 4th-level column — a level-7 specialist now casts 4th-level spells for
// the first time, so the bonus slot count becomes one of each spell level 1st
// through 4th); Intense Spells' bonus-damage magnitude STAYS at 3
// (`max(7/2, 1) = 3`, unchanged from level 6, another integer-division
// coincidence); the level-7 "Special" column is genuinely blank (verified
// independently against both primary sources), so no new class feature is
// gained at 7th level.
//
// A further SD13-E5 slice widens the gate again to level 8
// (`MAX_SUPPORTED_WIZARD_LEVEL = 8`): base attack bonus GENUINELY RISES to +4
// (`8/2 = 4`, up from +3) and good Will GENUINELY RISES to +6 (`8/2+2 = 6`, up
// from +5), while poor Fortitude/Reflex both STAY at +2 (`8/3 = 2`,
// integer-division coincidences); the specialist bonus slot flat count STAYS
// at 4 (the raw spells-per-day table's level-8 row is "4/4/3/3/2" with the
// 5th-level column still "—" — 5th-level spells first appear at level 9, so
// the next slot-count rise lands there, not at level 8, a threshold stasis
// verified against both primary sources' raw table rows rather than assumed);
// Intense Spells' bonus-damage magnitude GENUINELY RISES to 4
// (`max(8/2, 1) = 4`, up from 3 at levels 6-7, via the same pre-existing
// formula, not re-derived); the level-8 "Special" column is genuinely blank
// (verified independently against both primary sources — the Wizard's bonus
// feats land at levels 5, 10, 15, and 20), so no new class feature is gained
// at 8th level.
//
// A further SD13-E5 slice widens the gate again to level 9
// (`MAX_SUPPORTED_WIZARD_LEVEL = 9`): base attack stays +4 (`9/2 = 4`) and
// good Will stays +6 (`9/2+2 = 6`), integer-division coincidences, while
// poor Fortitude/Reflex both GENUINELY RISE to +3 (`9/3 = 3`); the
// specialist bonus slot flat count GENUINELY RISES to 5 (the raw
// spells-per-day table's level-9 row is "4/4/4/3/2/1", the first non-"—"
// 5th-level column — a level-9 specialist now casts 5th-level spells for
// the first time, so the bonus slot count becomes one of each spell level
// 1st through 5th, via WIZARD_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL);
// Intense Spells' bonus-damage magnitude STAYS at 4 (`max(9/2, 1) = 4`,
// another integer-division coincidence — the next rise lands at level 10);
// the level-9 "Special" column is genuinely blank (verified independently
// against both primary sources), so no new class feature is gained at 9th
// level.
//
// A further SD13-E5 slice widens the gate again to level 10 — the tranche
// ceiling (`MAX_SUPPORTED_WIZARD_LEVEL = 10`): base attack GENUINELY RISES
// to +5 (`10/2 = 5`) and good Will GENUINELY RISES to +7 (`10/2+2 = 7`),
// while poor Fortitude/Reflex stay +3 (`10/3 = 3`, integer-division
// coincidences); the specialist bonus slot flat count STAYS at 5 (the raw
// spells-per-day table's level-10 row is "4/4/4/3/3/2" with the 6th-level
// column still "—" — 6th-level spells first appear at 11th, a threshold
// stasis checked rather than assumed); Intense Spells' bonus-damage
// magnitude GENUINELY RISES to 5 (`max(10/2, 1) = 5`, up from 4 at levels
// 8-9); the level-10 "Special" column reads "Bonus feat" (verified
// independently against both primary sources) — the same genuinely
// open-ended metamagic/item-creation/Spell-Mastery choice already left
// named-but-unproven at 5th level, not a new type of class feature, so no
// new pillar record is grounded at level 10.
//
// A further SD18 slice (the first §3.2 landing for Wizard beyond the SD13
// tranche ceiling) widens the gate again to level 11
// (`MAX_SUPPORTED_WIZARD_LEVEL = 11`): base attack and all three base saves
// stay numerically IDENTICAL to level 10 (`11/2 = 5`, `11/3 = 3`,
// `11/2+2 = 7`, integer-division coincidences, re-verified rather than
// assumed); the specialist bonus slot flat count GENUINELY RISES to 6, since
// the raw spells-per-day table's level-11 row is "4/4/4/4/3/2/1" — the first
// non-"—" 6th-level column, up from the level-10 row "4/4/4/3/3/2" whose
// 6th-level column is still "—" (verified independently against both
// primary sources, d20pfsrd and the Archives of Nethys aonprd.com mirror,
// checked rather than assumed) — so a level-11 specialist wizard casts
// 6th-level spells for the first time; Intense Spells' bonus-damage
// magnitude stays 5 (`max(11/2, 1) = 5`, another integer-division
// coincidence); Force Missile's pool is level-independent and unchanged; the
// level-11 "Special" column is genuinely blank (verified independently
// against both sources — the Wizard's bonus feats land only at levels
// 5/10/15/20), so no new pillar record is grounded at level 11 beyond
// widening the specialist-bonus-slot pillar to its new value.
//
// A further SD18 slice widens the gate again to level 12
// (`MAX_SUPPORTED_WIZARD_LEVEL = 12`): UNLIKE level 11 (where base attack
// bonus and all three base saves stayed numerically unchanged from level
// 10, integer-division coincidences), base attack bonus GENUINELY RISES to
// +6 (`12/2 = 6`) and all three base saves GENUINELY RISE too (poor
// Fortitude/Reflex `12/3 = 4`, good Will `12/2+2 = 8`) — verified rather
// than assumed against both primary sources (d20pfsrd and the Archives of
// Nethys aonprd.com mirror, which agree byte-for-byte), mirroring the
// Sorcerer level-11-then-level-12 pattern exactly. The raw spells-per-day
// table's level-12 row is "4/4/4/4/3/3/2" — the 6th-level column rises from
// 1 to 2, but there is still no 7th-level column at all (7th-level wizard
// spells do not become accessible until level 13), so the specialist
// bonus-slot flat count (one bonus slot of each spell level she can cast)
// STAYS at 6, a threshold stasis checked rather than assumed; Intense
// Spells' bonus-damage magnitude GENUINELY RISES to 6 (`max(12/2, 1) = 6`,
// up from 5 at level 11) via the pre-existing formula, not re-derived;
// Force Missile's pool is level-independent and unchanged; the level-12
// "Special" column is genuinely blank on both primary sources (the
// Wizard's bonus feats land only at levels 5/10/15/20), so no new pillar
// record is grounded at level 12 beyond the arithmetic pillars above. This
// cycle independently re-verified (rather than assumed from the
// immediately-preceding Sorcerer level-12 cycle's outcome) that Wizard's
// own live `class_spell.wizard.prepared_spellbook.unsupported`
// claim-blocker is pushed unconditionally alongside the level's other
// explanations — it does not gate `supported_wizard_level` or this
// constant, exactly mirroring every sibling class's own remaining-burden
// diagnostics, so it marks incomplete coverage without blocking this
// arithmetic widening.
//
// A further SD18 slice widens the gate again to level 13 — the LAST §3.2
// level-13 landing among the 11 core classes (Monk excluded, confirmed dead
// end: Diamond Soul needs spell resistance, not grounded in this codebase):
// base attack bonus and all three base saves STAY numerically unchanged from
// level 12 (`13/2 = 6`, `13/3 = 4`, `13/2+2 = 8`), integer-division
// coincidences verified against both primary sources (d20pfsrd and the
// Archives of Nethys aonprd.com mirror, which agree byte-for-byte) rather
// than assumed. The raw spells-per-day table's level-13 row is
// "4/4/4/4/4/3/2/1" — the first non-"—" 7th-level column, up from the
// level-12 row "4/4/4/4/3/3/2" whose 7th-level column does not exist at all
// — so a level-13 specialist wizard casts 7th-level spells for the first
// time, and the specialist bonus-slot flat count (one bonus slot of each
// spell level she can cast) GENUINELY RISES to 7, from 6 at level 12, via a
// new tier constant (`WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_13`) gated on a
// new threshold constant (`WIZARD_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL`),
// mirroring the existing level-3/5/7/9/11 idiom exactly; Intense Spells'
// bonus-damage magnitude STAYS at 6 (`max(13/2, 1) = 6`, another
// integer-division coincidence); Force Missile's pool is level-independent
// and unchanged; the level-13 "Special" column is genuinely BLANK on both
// primary sources (the Wizard's bonus feats land only at levels 5/10/15/20),
// so no new pillar record is grounded at level 13 beyond widening the
// specialist-bonus-slot pillar to its new value.
//
// A further SD18 slice widens the gate again to level 14 — the TENTH §3.2
// level-14 landing, and the LAST of the 11 core classes to reach level 14
// (Monk excluded, confirmed dead end at level 13): base attack bonus
// GENUINELY RISES to +7 (`14/2 = 7`, up from +6) and good Will GENUINELY
// RISES to +9 (`14/2+2 = 9`, up from +8), while poor Fortitude/Reflex both
// STAY at +4 (`14/3 = 4`, an integer-division coincidence with level 13) —
// verified against three primary sources (d20pfsrd, the Archives of Nethys
// aonprd.com mirror, and legacy.aonprd.com, all byte-for-byte identical).
// The raw spells-per-day table's level-14 row is "4/4/4/4/4/3/3/2" — up
// from the level-13 row "4/4/4/4/4/3/2/1" (the 6th-level column rises from
// 2 to 3 and the 7th-level column rises from 1 to 2), but the 8th-level
// column stays "—" (8th-level wizard spells do not become accessible until
// level 15, verified rather than assumed), so the specialist bonus-slot
// flat count STAYS at 7, unchanged from level 13 — no new tier constant is
// needed since the existing `>= WIZARD_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL`
// branch already covers level 14. Intense Spells' bonus-damage magnitude
// GENUINELY RISES to 7 (`max(14/2, 1) = 7`, up from 6) via the pre-existing
// formula, not re-derived; Force Missile's pool is level-independent and
// unchanged; the level-14 "Special" column is genuinely BLANK on all three
// primary sources (the Wizard's bonus feats land only at levels
// 5/10/15/20), so no new pillar record is grounded at level 14 — only the
// already-grounded arithmetic pillars widen, and this is a pure ceiling
// raise: every consuming formula already reads `level` generically, so no
// new tier constant or threshold constant is needed at all.
//
// A further SD18 slice (the loop's SEVENTH §3.2 level-15 landing, after
// Barbarian, Rogue, Fighter, Cleric, Druid, and Ranger) widens the gate
// again to level 15: base attack bonus STAYS at +7 (`15/2 = 7`) and good
// Will STAYS at +9 (`15/2+2 = 9`), both integer-division coincidences with
// level 14, while poor Fortitude/Reflex both GENUINELY RISE to +5
// (`15/3 = 5`, up from +4) — verified against two primary sources
// (d20pfsrd and the Archives of Nethys aonprd.com mirror, which agree
// byte-for-byte, so no third source was required). The raw spells-per-day
// table's level-15 row is "4/4/4/4/4/4/3/2/1" — up from the level-14 row
// "4/4/4/4/4/3/3/2" (the 5th-level column rises from 3 to 4) AND a
// genuinely NEW 8th-level column appears for the first time (value 1) —
// so a level-15 specialist wizard casts 8th-level spells for the first
// time, and the specialist bonus-slot flat count GENUINELY RISES to 8, up
// from 7 at level 14, via a new `WIZARD_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL
// = 15` threshold constant gated exactly like the existing
// level-3/5/7/9/11/13 idiom. Intense Spells' bonus-damage magnitude STAYS
// at 7 (`max(15/2, 1) = 7`, another integer-division coincidence with
// level 14) via the pre-existing formula, not re-derived; Force Missile's
// pool is level-independent and unchanged; the level-15 "Special" column
// reads "Bonus feat" on both primary sources — the SAME genuinely
// open-ended metamagic/item-creation/Spell-Mastery choice already left
// named-but-unproven at levels 5 and 10, not a new type of class feature,
// so no new pillar record is grounded at level 15 beyond widening the
// specialist-bonus-slot pillar to its new value.
//
// A further SD18 slice widens the gate again to level 16 — the loop's
// THIRD §3.2 level-16 landing, after Barbarian and Fighter, and the first
// spell-bearing class to reach level 16 in the level-16 sweep: base
// attack bonus GENUINELY RISES to +8 (`16/2 = 8`, up from +7) and good
// Will GENUINELY RISES to +10 (`16/2+2 = 10`, up from +9), while poor
// Fortitude/Reflex both STAY at +5 (`16/3 = 5`, an integer-division
// coincidence with level 15) — verified independently against two primary
// sources (d20pfsrd and the Archives of Nethys aonprd.com mirror, which
// agree byte-for-byte, so no third source was required). The raw
// spells-per-day table's level-16 row is "4/4/4/4/4/4/3/3/2" — up from the
// level-15 row "4/4/4/4/4/4/3/2/1" (the 7th-level column rises from 2 to 3
// and the 8th-level column rises from 1 to 2), but the 9th-level column
// stays "—" (9th-level wizard spells do not become accessible until level
// 17, verified rather than assumed), so the specialist bonus-slot flat
// count STAYS at 8, unchanged from level 15 — the pre-existing
// `>= WIZARD_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL` branch already
// covers level 16, so no new tier or threshold constant is needed at all.
// Intense Spells' bonus-damage magnitude GENUINELY RISES to 8
// (`max(16/2, 1) = 8`, up from 7) via the pre-existing formula, not
// re-derived; Force Missile's pool is level-independent and unchanged; the
// level-16 "Special" column is genuinely BLANK on both primary sources
// (the Wizard's bonus feats land only at levels 5/10/15/20), so no new
// pillar record is grounded at level 16 — only the already-grounded
// arithmetic pillars widen, and this is a pure ceiling raise: every
// consuming formula already reads `level` generically, so no new tier
// constant or threshold constant is needed at all.
//
// A further SD18 slice (the loop's FIFTH §3.2 level-17 landing, after
// Ranger, Bard, Rogue, and Fighter, and the first spell-bearing class to
// reach level 17 in the level-17 sweep) widens the gate again to level 17:
// base attack bonus STAYS at +8 (`17/2 = 8`) and good Will STAYS at +10
// (`17/2+2 = 10`), both integer-division coincidences with level 16, while
// poor Fortitude/Reflex both STAY at +5 (`17/3 = 5`, also an
// integer-division coincidence with level 16) — verified independently
// against two primary sources (d20pfsrd and the Archives of Nethys
// aonprd.com mirror, which agree byte-for-byte, so no third source was
// required), fetching the full levels-15-through-18 block in one pass to
// rule out level-misattribution. The raw spells-per-day table's level-17
// row is "4/4/4/4/4/4/4/3/2/1" — up from the level-16 row
// "4/4/4/4/4/4/3/3/2/—" (the 6th-level column rises from 3 to 4) AND a
// genuinely NEW 9th-level column appears for the first time (value 1), so
// a level-17 specialist wizard casts 9th-level spells for the first time,
// and the specialist bonus-slot flat count GENUINELY RISES to 9, up from 8
// at levels 15-16, via a new
// `WIZARD_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 17` threshold constant
// gated exactly like the existing level-3/5/7/9/11/13/15 idiom. Intense
// Spells' bonus-damage magnitude STAYS at 8 (`max(17/2, 1) = 8`, an
// integer-division coincidence with level 16) via the pre-existing
// formula, not re-derived; Force Missile's pool is level-independent and
// unchanged; the level-17 "Special" column is genuinely BLANK on both
// primary sources (the Wizard's bonus feats land only at levels
// 5/10/15/20), so no new named-feature pillar record is grounded at level
// 17 beyond the 9th-level-column-opening widening of the pre-existing
// specialist-bonus-slot pillar.
//
// A further SD18 slice (the loop's FIRST §3.2 level-18 landing, opening the
// level-18 sweep) widens the gate again to level 18: base attack bonus
// GENUINELY RISES to +9 (`18/2 = 9`, up from +8 at level 17) and good Will
// GENUINELY RISES to +11 (`18/2+2 = 11`, up from +10 at level 17), while
// poor Fortitude/Reflex both GENUINELY RISE to +6 (`18/3 = 6`, up from +5
// at level 17) — verified independently against two primary sources
// (d20pfsrd and the Archives of Nethys aonprd.com mirror, which agree
// byte-for-byte, so no third source was required), fetching the full
// levels-16-through-19 block in one pass to rule out level-misattribution.
// The raw spells-per-day table's level-18 row is "4/4/4/4/4/4/4/3/3/2" —
// up from the level-17 row "4/4/4/4/4/4/4/3/2/1" (the 8th-level column
// rises from 2 to 3 and the 9th-level column rises from 1 to 2), but NO
// genuinely new spell-level column opens (9th is already the highest
// wizard spell level in PF1), so the specialist bonus-slot flat count
// STAYS at 9 (checked rather than assumed) — no new threshold constant is
// needed. Intense Spells' bonus-damage magnitude GENUINELY RISES to 9
// (`max(18/2, 1) = 9`, up from 8 at level 17) via the pre-existing
// formula, not re-derived; Force Missile's pool is level-independent and
// unchanged; the level-18 "Special" column is genuinely BLANK on both
// primary sources (the Wizard's bonus feats land only at levels
// 5/10/15/20), so no new named-feature pillar record is grounded at level
// 18 — a pure arithmetic-pillar widening on the already-grounded base
// attack / base save / Intense Spells formulas, with the specialist
// bonus-slot pillar staying flat.
//
// A further SD18 slice (the loop's NINTH §3.2 level-19 landing, after
// Barbarian, Cleric, Fighter, Bard, Paladin, Ranger, Rogue, and Sorcerer,
// and the LAST of the 9 eligible classes, fully closing the level-19 sweep)
// widens the gate again to level 19: base attack bonus STAYS at +9
// (18/2 = 9 vs 19/2 = 9, an integer-division coincidence with level 18) and
// good Will STAYS at +11 (19/2+2 = 11), while poor Fortitude/Reflex both
// STAY at +6 (19/3 = 6) — verified independently against two primary
// sources (d20pfsrd and the Archives of Nethys aonprd.com mirror, which
// agree byte-for-byte, so no third source was required), fetching the full
// levels-16-through-20 block in one pass to rule out level-misattribution.
// The raw spells-per-day table's level-19 row is "4/4/4/4/4/4/4/4/3/3" — up
// from the level-18 row "4/4/4/4/4/4/4/3/3/2" (the 7th-level column rises
// from 3 to 4 and the 9th-level column rises from 2 to 3), but NO genuinely
// new spell-level column opens (9th is already the highest wizard spell
// level in PF1, first opened at level 17), so the specialist bonus-slot
// flat count STAYS at 9 (checked rather than assumed) — no new threshold
// constant is needed. Intense Spells' bonus-damage magnitude STAYS at 9
// (max(19/2, 1) = 9) via the pre-existing formula, not re-derived; Force
// Missile's pool is level-independent and unchanged; the level-19
// "Special" column is genuinely BLANK on both primary sources (the
// Wizard's bonus feats land only at levels 5/10/15/20, confirmed directly
// rather than assumed — 19 is NOT a bonus-feat level), so no new
// named-feature pillar record is grounded at level 19 — a pure
// arithmetic-pillar widening on the already-grounded base attack / base
// save / Intense Spells formulas, with the specialist bonus-slot pillar
// staying flat.
//
// A further SD18 slice (cycle-2026-07-16T1000, the loop's SECOND §3.2
// level-20 landing, after Cleric) widens the gate again to level 20 — the
// final remaining level within PF1's 1-20 character-level cap for this
// class row: base attack bonus GENUINELY RISES to +10 (`20/2 = 10`, up
// from +9) and good Will GENUINELY RISES to +12 (`20/2+2 = 12`, up from
// +11), while poor Fortitude/Reflex both STAY at +6 (`20/3 = 6`, an
// integer-division coincidence with level 19) — verified independently
// against two primary sources (d20pfsrd and the Archives of Nethys
// aonprd.com mirror, byte-for-byte agreement on the full
// levels-15-through-20 block, so a third source was not required). The raw
// spells-per-day table's level-20 row is "4/4/4/4/4/4/4/4/4/4" — up from
// the level-19 row "4/4/4/4/4/4/4/4/3/3" (the 8th- and 9th-level columns
// both rise to 4), but NO genuinely new spell-level column opens (9th is
// already the highest wizard spell level in PF1, first opened at level
// 17), so the specialist bonus-slot flat count STAYS at 9 (checked rather
// than assumed) — no new threshold constant is needed. Intense Spells'
// bonus-damage magnitude GENUINELY RISES to 10 (`max(20/2, 1) = 10`, up
// from 9) via the pre-existing formula, not re-derived; Force Missile's
// pool is level-independent and unchanged; the level-20 "Special" column
// reads "Bonus feat" on both primary sources — the SAME genuinely
// open-ended metamagic/item-creation/Spell-Mastery choice already left
// named-but-unproven at levels 5, 10, and 15 (the class table's own
// "Bonus Feats" ability text, "At 5th, 10th, 15th, and 20th level, a
// wizard gains a bonus feat," is identical wording on both sources and
// names no new mechanic at 20th level), so it stays deliberately
// named-but-unproven and no new pillar record is grounded at level 20
// either — only the base-attack, base-save, and Intense Spells pillars are
// widened. A separate, non-Core-Rulebook "Well-Prepared" alternate
// capstone appears on both sources but is explicitly sourced to
// Pathfinder Player Companion: Chronicle of Legends (an optional splatbook
// replacement ability) — out of SD18's Core Rulebook scope, not modeled
// here. This is the final level within PF1's 1-20 character-level cap for
// this class row.
pub(crate) const MAX_SUPPORTED_WIZARD_LEVEL: u8 = 20;

// SD13-E5 Wizard specialization slice: the canonical deterministic fixture
// selections for the school specialization choice. The bounded seam recognizes
// exactly this canonical triple (Evocation chosen; Necromancy and Transmutation
// opposed) versus "absent or anything else" — it is not a general school engine.
pub(super) const WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID: &str = "choice:wizard_school_specialization";

pub(super) const WIZARD_OPPOSED_SCHOOLS_CHOICE_ID: &str = "choice:wizard_opposed_schools";

pub(super) const NECROMANCY_SCHOOL_SELECTION: &str = "school:necromancy";

/// PF1 Core Rulebook arcane school class feature: a specialist wizard gains one
/// additional spell slot of each spell level she can cast, 1st and up, usable only
/// for spells of the chosen school. At the bounded baseline level 1 that is exactly
/// one 1st-level slot; there is no cantrip-level bonus slot. Confirmed unchanged at
/// level 2 (SD13-E5): a level-2 wizard still only casts 1st-level wizard spells
/// (2nd-level wizard spells require caster level 3), so the count stays exactly 1
/// through the whole level 1-2 range this seam supports.
pub(super) const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVELS_1_AND_2: i16 = 1;

/// SD13-E5 level-3 widening: a level-3 wizard casts 2nd-level spells for the first
/// time (verified independently against both primary sources' raw Wizard
/// spells-per-day table rows: level 2 shows "4/2/—/—", level 3 shows "4/2/1/—" — the
/// first non-"—" 2nd-level column), so a specialist wizard now gains one bonus slot
/// of EACH spell level she can cast: one 1st-level bonus slot plus one 2nd-level
/// bonus slot, for a flat count of 2. Confirmed unchanged at level 4 (SD13-E5
/// widening): the level-4 row is still "4/3/2/—/—" — 3rd-level wizard spells do not
/// become available until wizard level 5 (level 5 row: "4/3/2/1/—", the first
/// non-"—" 3rd-level column), verified independently against both primary sources
/// rather than assumed from the level-3 doubling precedent — so the flat count stays
/// exactly 2 through the level 3-4 range this constant covers.
pub(super) const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_3: i16 = 2;

/// SD13-E5 level-5 widening: a level-5 wizard casts 3rd-level spells for the first
/// time (verified independently against both primary sources' raw Wizard
/// spells-per-day table rows: level 4 shows "4/3/2/—/—", level 5 shows "4/3/2/1/—" —
/// the first non-"—" 3rd-level column), so a specialist wizard now gains one bonus
/// slot of EACH spell level she can cast, 1st through 3rd: one 1st-level bonus slot,
/// one 2nd-level bonus slot, and one 3rd-level bonus slot, for a flat count of 3, up
/// from 2 at levels 3-4.
pub(super) const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_5: i16 = 3;

/// SD13-E5 level-7 widening: a level-7 wizard casts 4th-level spells for the first
/// time (verified independently against both primary sources' raw Wizard
/// spells-per-day table rows: level 6 shows "4/3/3/2/—", level 7 shows "4/4/3/2/1" —
/// the first non-"—" 4th-level column), so a specialist wizard now gains one bonus
/// slot of EACH spell level she can cast, 1st through 4th: one 1st-level bonus slot,
/// one 2nd-level bonus slot, one 3rd-level bonus slot, and one 4th-level bonus slot,
/// for a flat count of 4, up from 3 at levels 5-6, mirroring exactly the Cleric
/// domain-spell-slot level-7 widening idiom (`CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_7`).
pub(super) const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_7: i16 = 4;

pub(super) const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_9: i16 = 5;

/// SD18 level-11 widening: a level-11 wizard casts 6th-level spells for the first
/// time (verified independently against both primary sources' raw Wizard
/// spells-per-day table rows: level 10 shows "4/4/4/3/3/2", level 11 shows
/// "4/4/4/4/3/2/1" — the first non-"—" 6th-level column), so a specialist wizard now
/// gains one bonus slot of EACH spell level she can cast, 1st through 6th, for a flat
/// count of 6, up from 5 at levels 9-10.
pub(super) const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_11: i16 = 6;

/// SD18 level-13 widening: a level-13 wizard casts 7th-level spells for the
/// first time (verified independently against both primary sources' raw
/// Wizard spells-per-day table rows: level 12 shows "4/4/4/4/3/3/2", level 13
/// shows "4/4/4/4/4/3/2/1" — the first non-"—" 7th-level column), so a
/// specialist wizard now gains one bonus slot of EACH spell level she can
/// cast, 1st through 7th, for a flat count of 7, up from 6 at levels 11-12.
pub(super) const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_13: i16 = 7;

/// SD18 level-15 widening: a level-15 wizard casts 8th-level spells for the
/// first time (verified independently against both primary sources' raw
/// Wizard spells-per-day table rows: level 14 shows "4/4/4/4/4/3/3/2", level
/// 15 shows "4/4/4/4/4/4/3/2/1" — the first non-"—" 8th-level column), so a
/// specialist wizard now gains one bonus slot of EACH spell level she can
/// cast, 1st through 8th, for a flat count of 8, up from 7 at levels 13-14.
pub(super) const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_15: i16 = 8;

/// SD18 level-17 widening: a level-17 wizard casts 9th-level spells for the
/// first time (verified independently against both primary sources' raw
/// Wizard spells-per-day table rows: level 16 shows "4/4/4/4/4/4/3/3/2",
/// level 17 shows "4/4/4/4/4/4/4/3/2/1" — the first non-"—" 9th-level
/// column), so a specialist wizard now gains one bonus slot of EACH spell
/// level she can cast, 1st through 9th, for a flat count of 9, up from 8 at
/// levels 15-16.
pub(super) const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_17: i16 = 9;

/// PF1 Core Rulebook Wizard spells-per-day table: the wizard class level at which
/// 2nd-level wizard spells first become available (verified independently against
/// both primary sources: level 1-2 wizards cast only 1st-level spells; level 3 is
/// the first row with a non-"—" 2nd-level column).
pub(super) const WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 3;

/// PF1 Core Rulebook Wizard spells-per-day table: the wizard class level at which
/// 3rd-level wizard spells first become available (verified independently against
/// both primary sources: levels 3-4 wizards cast only up to 2nd-level spells; level
/// 5 is the first row with a non-"—" 3rd-level column).
pub(super) const WIZARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 5;

/// PF1 Core Rulebook Wizard spells-per-day table: the wizard class level at which
/// 4th-level wizard spells first become available (verified independently against
/// both primary sources: levels 5-6 wizards cast only up to 3rd-level spells; level
/// 7 is the first row with a non-"—" 4th-level column).
pub(super) const WIZARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 7;

/// The wizard level at which 5th-level wizard spells (and so the fifth
/// Evocation-only specialist bonus slot) first become available, verified
/// against the raw PF1 Core Rulebook Wizard spells-per-day table rows
/// (d20pfsrd and legacy.aonprd.com): level 8 shows a still-"—" 5th-level
/// column, level 9 is the first to show a non-"—" 5th-level column ("1", the
/// level-9 row reading "4/4/4/3/2/1").
pub(super) const WIZARD_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 9;

/// The wizard level at which 6th-level wizard spells (and so the sixth
/// Evocation-only specialist bonus slot) first become available, verified
/// against the raw PF1 Core Rulebook Wizard spells-per-day table rows
/// (d20pfsrd and the Archives of Nethys aonprd.com mirror): level 10 shows
/// a still-"—" 6th-level column, level 11 is the first to show a non-"—"
/// 6th-level column ("1", the level-11 row reading "4/4/4/4/3/2/1").
pub(super) const WIZARD_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 11;

/// The wizard level at which 7th-level wizard spells (and so the seventh
/// Evocation-only specialist bonus slot) first become available, verified
/// against the raw PF1 Core Rulebook Wizard spells-per-day table rows
/// (d20pfsrd and the Archives of Nethys aonprd.com mirror, which agree
/// byte-for-byte): level 12 shows a still-"—" 7th-level column, level 13 is
/// the first to show a non-"—" 7th-level column ("1", the level-13 row
/// reading "4/4/4/4/4/3/2/1").
pub(super) const WIZARD_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 13;

/// The wizard level at which 8th-level wizard spells (and so the eighth
/// Evocation-only specialist bonus slot) first become available, verified
/// against the raw PF1 Core Rulebook Wizard spells-per-day table rows
/// (d20pfsrd and the Archives of Nethys aonprd.com mirror, which agree
/// byte-for-byte): level 14 shows a still-"—" 8th-level column (the level-14
/// row reading "4/4/4/4/4/3/3/2"), level 15 is the first to show a
/// non-"—" 8th-level column ("1", the level-15 row reading
/// "4/4/4/4/4/4/3/2/1").
pub(super) const WIZARD_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 15;

/// The wizard level at which 9th-level wizard spells (and so the ninth
/// Evocation-only specialist bonus slot) first become available, verified
/// against the raw PF1 Core Rulebook Wizard spells-per-day table rows
/// (d20pfsrd and the Archives of Nethys aonprd.com mirror, which agree
/// byte-for-byte): level 16 shows a still-"—" 9th-level column (the
/// level-16 row reading "4/4/4/4/4/4/3/3/2"), level 17 is the first to show
/// a non-"—" 9th-level column ("1", the level-17 row reading
/// "4/4/4/4/4/4/4/3/2/1").
pub(super) const WIZARD_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 17;

/// v0.6 alpha swarm, risks item 8 (Arcanist full-build closure, fifth
/// APG/ACG class-specific closure): whether `input` is a single-class
/// Arcanist at a level within `acg::class_chassis_resolve`'s declared
/// ceiling for Arcanist -- mirrors the other four ACG exact-match gates
/// exactly, including the same exact-match discipline (`==
/// Some(AcgClassId::Arcanist)`, not a broad `.is_some()`).
pub(super) fn is_supported_arcanist_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if AcgClassId::from_class_id_str(&class_level.class_id) != Some(AcgClassId::Arcanist) {
        return false;
    }
    acg::class_chassis_resolve(AcgClassId::Arcanist, class_level.level, RuleSetId::Acg).is_some()
}

/// Compute the Wizard base-attack-bonus / base-save chassis pillar (SD-21 E6.26).
///
/// Composes `rules_tables::crb::class_tables::class_tables()`'s Wizard row
/// (`BabProgression::Half`; good Will only, poor Fortitude/Reflex) rather than
/// re-deriving the progression — that row was independently spot-checked against
/// this file's own already-primary-source-verified Wizard formulas (see
/// `explain_wizard_level1_prepared_spell_baseline`'s standalone
/// `class_chassis.wizard.base_attack_bonus` / `base_save.*` explanation records, and
/// `level_up/wizard.rs`'s SD-20 cycle, which performed the identical spot-check
/// before composing with it: "The two sources agree at every level 1-20"). Mirrors
/// `compute_fighter_chassis`'s generic, un-prefixed `class_chassis.base_attack_bonus`
/// / `class_chassis.base_save.*` explanation ids — the ones actually wired into the
/// integrated `base_attack_bonus` / `base_saves` fields — which is why they are
/// distinct from the pre-existing, still-standalone `class_chassis.wizard.*` ids
/// `explain_wizard_level1_prepared_spell_baseline` already grounds.
pub(super) fn compute_wizard_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> (i16, BaseSaves) {
    let Some(level) = supported_wizard_level(input) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis is only supported for a single-class {WIZARD_CLASS_ID} at \
                 levels 1-{MAX_SUPPORTED_WIZARD_LEVEL}; chosen class levels {:?} do not provide \
                 it, so no chassis values were computed",
                input.chosen.class_levels
            ),
            claim_blocking: true,
        });
        return (0, BaseSaves::default());
    };

    let Some(row) = class_tables()
        .into_iter()
        .find(|row| row.class_id == ClassId::Wizard && row.level == level)
    else {
        // Cannot happen while MAX_SUPPORTED_WIZARD_LEVEL stays 20 and class_tables()'s
        // own Wizard CLASS_META row's max_supported_level stays 20 (both independently
        // confirmed identical by level_up/wizard.rs's SD-20 spot-check), but stays a
        // named, claim-blocking fallback rather than a panic if that ever drifts.
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis has no {WIZARD_CLASS_ID} class_tables() row at level \
                 {level}, so no chassis values were computed"
            ),
            claim_blocking: true,
        });
        return (0, BaseSaves::default());
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
            "Wizard level {level} base attack bonus from \
             rules_tables::crb::class_tables::class_tables()'s Wizard row (1/2 BAB, PF1 Core \
             Rulebook Wizard class table): {base_attack_bonus}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: base_saves.fortitude,
        detail: format!(
            "Wizard level {level} base Fortitude save (poor) from \
             rules_tables::crb::class_tables::class_tables()'s Wizard row: {}",
            base_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: base_saves.reflex,
        detail: format!(
            "Wizard level {level} base Reflex save (poor) from \
             rules_tables::crb::class_tables::class_tables()'s Wizard row: {}",
            base_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: base_saves.will,
        detail: format!(
            "Wizard level {level} base Will save (good) from \
             rules_tables::crb::class_tables::class_tables()'s Wizard row: {}",
            base_saves.will
        ),
    });

    (base_attack_bonus, base_saves)
}

/// Wizard's own level within `input.chosen.class_levels`, whether single-class
/// or part of a supported Fighter+Wizard multiclass mix (SD-24 Epic 5,
/// criterion 5.1). Mirrors `fighter_level_in_mix` exactly (SD-21 E7.30's own
/// precedent), reconciling Wizard's own per-class explanations (the level-1
/// prepared arcane spell-bearing recognition, Scribe Scroll, the school
/// specialization choice recognition, the specialist bonus slot ladder, the
/// two Evocation school powers, and the SD-21 E6b.2 prepared-spellbook
/// posture) to keep firing using Wizard's own sub-level once a supported
/// second class (Fighter) joins the mix, instead of silently dropping every
/// one of those real facts the moment the build stops being single-class --
/// the same gap `fighter_level_in_mix` already closed for Fighter's own
/// features. Returns `None` for a multiclass mix that is not itself a
/// supported combination (mirroring `is_supported_multiclass_mix`), so
/// Wizard's own explanations never surface ahead of that mix's own base
/// chassis becoming genuinely supported -- and returns `None` when Wizard is
/// absent from the mix entirely, so introducing another class never makes
/// Wizard's features appear for a build that isn't actually part-Wizard.
pub(super) fn wizard_level_in_mix(input: &CharacterInput) -> Option<u8> {
    if let Some(level) = supported_wizard_level(input) {
        return Some(level);
    }
    if !is_supported_multiclass_mix(input) {
        return None;
    }
    input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == WIZARD_CLASS_ID)
        .map(|class_level| class_level.level)
}

/// The bounded Sorcerer milestone level this decomposition surface grounds, if any.
/// Returns the single Sorcerer level when the chosen input is exactly a single-class
/// Sorcerer at one of the supported milestone levels (1 through 10). Returns `None` for
/// no Sorcerer, a non-Sorcerer class, a multiclass mix, or any level-11+ Sorcerer this
/// slice deliberately does not recognize — each of which stays claim-blocked exactly
/// as before. Mirrors the Fighter `supported_fighter_level` / Paladin
/// `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` / Bard `supported_bard_level` / Druid
/// `supported_druid_level` level-range gate idiom.
pub(super) fn supported_sorcerer_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == SORCERER_CLASS_ID
                && (1..=MAX_SUPPORTED_SORCERER_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E4-F7 runtime evidence for the deterministic Human Sorcerer
/// level-1 spell-bearing baseline, while keeping it explicitly claim-blocked on its
/// still-missing burdens.
///
/// The SD13-E4 Sorcerer decomposition slice splits the original combined bloodline
/// blocker into two named diagnostics and grounds one of them for real: Eschew
/// Materials, the universal, bloodline-independent bonus feat every 1st-level Sorcerer
/// receives (PF1 Core Rulebook: it lets a Sorcerer cast a spell with a material
/// component costing 1 gp or less without needing that material component). This is a
/// boolean feat grant, not a numeric formula, so it carries no fabricated mechanical
/// value; it grounds no bloodline power, no bloodline arcana, and no spell math
/// whatsoever — no spell slots, spells known, spell DCs, bonus spells, prepared
/// posture, or school choice.
///
/// The SD13-E5 Sorcerer bloodline-choice slice grounds the next honest pillar: the
/// canonical deterministic bloodline choice-slot selection
/// (`choice:sorcerer_bloodline -> bloodline:arcane`) is recognized as chosen input,
/// mirroring the Fighter bonus-feat choice-slot / Wizard Scribe Scroll precedent. This
/// is recognition only: the Arcane bloodline's level-1 power is Arcane Bond (a familiar
/// or a bonded object — an execution engine, not a flat number), so no power value is
/// fabricated. The former combined `bloodline_power` blocker narrows to an
/// `arcane_bond_and_bloodline_progression` blocker naming what stays unimplemented. It
/// only:
/// - leaves one recognition explanation so the `class:sorcerer:1` identity is acknowledged
///   as a spontaneous arcane spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value),
/// - leaves one grounded explanation recognizing the Eschew Materials bonus-feat grant
///   (also carrying no fabricated mechanical value, since it is a boolean grant),
/// - conditionally leaves one grounded explanation recognizing the canonical bloodline
///   choice-slot selection when a `choice:sorcerer_bloodline` selection is present
///   (carrying no fabricated mechanical value, since Arcane Bond is an execution engine
///   rather than a number), and
/// - emits two distinct claim-blocking diagnostics naming the Arcane Bond / bloodline
///   progression burden (Arcane Bond execution, the bloodline arcana, the bloodline
///   class skill grant, and the 3rd+-level bonus spells/feats) and the spontaneous
///   known-spell / slot posture burden explicitly, rather than hiding behind a generic
///   "unsupported caster" label. The Arcane Bond blocker names the Arcane bloodline's
///   specific mechanics only when the Arcane bloodline was the recognized selection;
///   otherwise it stays bloodline-agnostic so it never claims a specific bloodline's
///   facts for a character whose chosen bloodline this seam did not recognize.
///
/// The SD13-E5 Sorcerer base-attack/base-save slice grounds the foundational martial
/// pillar that every other class row in this matrix (Fighter, Barbarian, Monk, Rogue,
/// Paladin, Druid, Cleric, Bard) already has and Sorcerer never had: base attack bonus
/// (1/2 BAB, `classlevel / 2`) and base save progression (good Will only, poor
/// Fortitude, poor Reflex). Unlike every other class this loop has grounded so far
/// (Rogue/Monk/Druid/Cleric/Bard are all 3/4 BAB), the Sorcerer's own PF1 Core Rulebook
/// class table was verified independently (d20pfsrd and the legacy Paizo PRD mirror,
/// reading the raw level 1-6 rows: BAB +0/+1/+1/+2/+2/+3, Fort +0/+0/+1/+1/+1/+2, Ref
/// +0/+0/+1/+1/+1/+2, Will +2/+3/+3/+4/+4/+5) and found to be 1/2 BAB, not 3/4 — the
/// level 4/5 BAB values (+2 at both) disambiguate the 1/2-vs-3/4 fraction since level 1
/// alone floors every fraction to the same +0. Both pillars are grounded as flat,
/// standalone `ComputationExplanation` records, mirroring the exact "standalone, not
/// wired into the integrated `PilotBaseChassisComputation`" idiom already used for every
/// other class's own base-attack/base-save grounding: neither is wired into
/// `base_attack_bonus`, `compute_total_saves`, or `compute_combat_baseline`.
///
/// A further SD13-E5 slice widens the level-1..=2 gate to level 3
/// (`MAX_SUPPORTED_SORCERER_LEVEL = 3`) and extends every one of the formulas above to
/// level 3 via the same formula, without re-derivation, verified independently against
/// the PF1 Core Rulebook Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 3
/// base attack bonus is +1, base saves are +1/+1/+3 (Fortitude/Reflex/Will); the bloodline
/// choice and bloodline class-skill choice recognitions are not level-gated, so both
/// still fire at level 3 for the same fixture selections. UNLIKE Sorcerer's own blank
/// level-2 "Special" column, the level-3 "Special" column reads "Bloodline power,
/// bloodline spell" (verified independently against both primary sources) — this was
/// checked, not assumed away, but both named entries are bloodline-specific (they vary
/// per bloodline, e.g. the Arcane bloodline's own 3rd-level power is Metamagic Adept and
/// its 3rd-level bloodline spell is Identify) and neither is flat/identity-shaped the way
/// Rogue's Trap Sense or Monk's Still Mind are, so this slice grounds neither: both stay
/// named by the pre-existing `arcane_bond_and_bloodline_progression.unsupported`
/// diagnostic, unchanged.
///
/// A further SD13-E5 slice widens the level-1..=3 gate to level 4
/// (`MAX_SUPPORTED_SORCERER_LEVEL = 4`) and extends every one of the formulas above to
/// level 4 via the same formula, without re-derivation, verified independently against
/// the PF1 Core Rulebook Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 4
/// base attack bonus is +2, base saves are +1/+1/+4 (Fortitude/Reflex/Will); the
/// bloodline choice and bloodline class-skill choice recognitions are not level-gated,
/// so both still fire at level 4 for the same fixture selections. UNLIKE the level-3
/// "Special" column's "Bloodline power, bloodline spell" entry, the level-4 "Special"
/// column is blank (verified independently against both primary sources, checked rather
/// than assumed), so this slice grounds no new pillar for level 4 — only the existing
/// pillars are widened.
///
/// A further SD13-E5 slice widens the level-1..=4 gate to level 5
/// (`MAX_SUPPORTED_SORCERER_LEVEL = 5`) and extends every one of the formulas above to
/// level 5 via the same formula, without re-derivation, verified independently against
/// the PF1 Core Rulebook Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 5
/// base attack bonus is +2, base saves are +1/+1/+4 (Fortitude/Reflex/Will) — every one
/// of these four values is numerically unchanged from level 4, an integer-division
/// coincidence, not a sign any formula stopped scaling; the bloodline choice and
/// bloodline class-skill choice recognitions are not level-gated, so both still fire at
/// level 5 for the same fixture selections. UNLIKE the blank level-4 "Special" column,
/// the level-5 column reads "Bloodline spell" (verified independently against both
/// primary sources, checked rather than assumed away) — the sorcerer's second bloodline
/// spell grant (the Arcane bloodline's own 5th-level bloodline spell is invisibility),
/// but the entry is bloodline-specific and not flat/identity-shaped, so this slice
/// grounds no new pillar for level 5 either, mirroring exactly how the level-3
/// "Bloodline power, bloodline spell" entry was left unproven.
///
/// A further SD13-E5 slice widens the level-1..=5 gate to level 6
/// (`MAX_SUPPORTED_SORCERER_LEVEL = 6`) and extends every one of the formulas above to
/// level 6 via the same formula, without re-derivation, verified independently against
/// the PF1 Core Rulebook Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 6
/// base attack bonus is +3, base saves are +2/+2/+5 (Fortitude/Reflex/Will) — every one
/// of these four values is a genuinely NEW value, up from +2/+1/+1/+4 at level 5; the
/// bloodline choice and bloodline class-skill choice recognitions are not level-gated,
/// so both still fire at level 6 for the same fixture selections. UNLIKE the level-5
/// "Bloodline spell" entry, the level-6 "Special" column is genuinely blank (verified
/// independently against both primary sources, checked rather than assumed away), so
/// this slice grounds no new pillar for level 6 either — only the existing pillars are
/// widened.
///
/// A further SD13-E5 slice widens the level-1..=6 gate to level 7
/// (`MAX_SUPPORTED_SORCERER_LEVEL = 7`) and extends every one of the formulas above to
/// level 7 via the same formula, without re-derivation, verified independently against
/// the PF1 Core Rulebook Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 7
/// base attack bonus is +3, base saves are +2/+2/+5 (Fortitude/Reflex/Will) — every one
/// of these four values is numerically unchanged from level 6, an integer-division
/// coincidence, not a sign any formula stopped scaling; the bloodline choice and
/// bloodline class-skill choice recognitions are not level-gated, so both still fire at
/// level 7 for the same fixture selections. UNLIKE the blank level-6 "Special" column,
/// the level-7 column reads "Bloodline feat, bloodline spell" (verified independently
/// against both primary sources, checked rather than assumed away) — a bloodline feat
/// (chosen from a list specific to each bloodline, first granted at 7th level and every
/// six levels thereafter) and the sorcerer's third bloodline spell grant (the Arcane
/// bloodline's own 7th-level bloodline spell is dispel magic), but both entries are
/// bloodline-specific and not flat/identity-shaped, so this slice grounds no new pillar
/// for level 7 either, mirroring exactly how the level-3 and level-5 bloodline
/// power/spell entries were left unproven — only the existing pillars are widened.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Sorcerer spell-bearing identity, the
/// grounded Eschew Materials grant, the grounded bloodline choice recognition, the
/// grounded base-attack/base-save progression through level 7, and the two remaining
/// named burdens legible on the runtime path.
pub(super) fn explain_sorcerer_level1_spell_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // v0.6 alpha swarm, risks item 8, fifth slice (2026-07-25): both Sorcerer
    // burdens are validated/checked regardless of whether Sorcerer appears
    // alone or in a multiclass mix, and regardless of race -- checked
    // BEFORE the single-class-only/Human gate below, mirroring the Ranger/
    // Paladin fix exactly (see `ranger_dispatch_widening_safety_tests`' doc
    // comment for the full history of why this ordering matters once
    // `table_class_id` recognizes a class). The bloodline-power burden is
    // permanently unconditional (mirrors APG/ACG's own
    // `class_feature.<book>.<class>.unsupported` shape -- no bloodline
    // power execution is grounded anywhere in this codebase, so this never
    // becomes valid). The spell posture burden is a real, conditional
    // validation, mirroring `unmet_ranger_prepared_spell_conditions`
    // exactly, substituted for a spontaneous caster: validates every
    // `AcquisitionMode::Known` selection with `source_class_id ==
    // "class:sorcerer"` against the real
    // `sorcerer_spell_list::SORCERER_SPELL_LIST`, the sorcerer's own
    // spell-level access ceiling (1st+; cantrips have no access gate), and
    // the Sorcerer Spells Known table's own per-level cap on distinct known
    // spells (not a per-day consumable resource, unlike Ranger/Paladin's
    // prepared posture -- known spells are permanent).
    if let Some(sorcerer_level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == SORCERER_CLASS_ID)
        .map(|class_level| class_level.level)
    {
        // Read once here (race/level-independent chosen-input reads) so the
        // diagnostic message below can name whether the class-skill grant is
        // already grounded separately, same detail the pre-widening message
        // carried -- reused again by the explanation records further down,
        // not recomputed.
        let bloodline_selection = choice_selection(input, SORCERER_BLOODLINE_CHOICE_ID);
        let recognized_arcane_bloodline =
            bloodline_selection == Some(ARCANE_BLOODLINE_SELECTION_ID);

        // SD-32 T12 Epic 8 row 18 cycle 5: the generic "select ONE
        // bloodline, inherit every one of its real corpus powers" pass,
        // covering the other 51 real Sorcerer bloodline groups (391 real
        // records total, `census_class_feature_pool_group_names.py`) this
        // file has never hand-modelled by name -- purely additive
        // alongside the Arcane/Draconic hand-modelled branches below.
        push_generic_pool_group_selection_magnitude(
            input,
            sorcerer_level,
            ability_modifiers,
            SORCERER_BLOODLINE_CHOICE_ID,
            "Sorcerer",
            "Bloodline",
            "bloodline:",
            "class_feature.sorcerer.bloodline.generic",
            1,
            explanations,
        );

        // SD-34 AT-34-E3-001 (`decisions.md §16`): the bloodline feat pool's
        // slot COUNT is bloodline-invariant, so this grounds unconditionally
        // here rather than inside `ground_sorcerer_arcane_bloodline_progression`'s
        // Arcane-only canonical narrowing below.
        ground_sorcerer_bloodline_feat_pool(sorcerer_level, explanations, diagnostics);

        let bloodline_class_skill_selection =
            choice_selection(input, SORCERER_BLOODLINE_CLASS_SKILL_CHOICE_ID);
        let recognized_bloodline_class_skill_name = bloodline_class_skill_selection
            .filter(|_| recognized_arcane_bloodline)
            .and_then(knowledge_skill_display_name);

        // v0.6 alpha swarm, risks item 8 (Sorcerer Arcane bloodline closure):
        // Arcane Bond's own identity (familiar vs. bonded object) is a real,
        // representable choice -- recognized the same way the bloodline and
        // class-skill choices already are.
        let arcane_bond_selection =
            choice_selection(input, SORCERER_ARCANE_BOND_CHOICE_ID).filter(|_| recognized_arcane_bloodline);
        let recognized_arcane_bond_name = arcane_bond_selection.and_then(|selection| {
            if selection == ARCANE_BOND_FAMILIAR_SELECTION_ID {
                Some("a familiar")
            } else if selection == ARCANE_BOND_BONDED_OBJECT_SELECTION_ID {
                Some("a bonded object")
            } else {
                None
            }
        });
        // 2026-07-29 (Sorcerer levels 3-20 closure): this used to carry a
        // `bonus_spells_and_feats_correctly_absent = sorcerer_level <
        // ARCANE_BLOODLINE_BONUS_LEVEL` term in the condition below, which
        // dropped every 3rd-level-and-above Sorcerer into the claim-blocking
        // `else` branch because the bloodline's bonus spells and bonus feats
        // were genuinely unimplemented. They are implemented now — see
        // `ground_sorcerer_arcane_bloodline_progression`, which transcribes the
        // whole 3rd-and-above progression from the corpus — so the term is gone
        // and the level gate now lives inside that function, where it selects
        // between a level-gate-absence record and a real magnitude per feature
        // rather than blocking the whole class. The blocker below is NOT
        // weakened: it still fires, unconditionally, for any Sorcerer whose
        // bloodline or Arcane Bond this seam does not recognize.
        if recognized_arcane_bloodline
            && let Some(bond_name) = recognized_arcane_bond_name
        {
            explanations.push(ComputationExplanation {
                id: "class_feature.sorcerer.arcane_bloodline.arcane_bond_choice".to_owned(),
                value: 0,
                detail: format!(
                    "Recognized Sorcerer level {sorcerer_level} Arcane Bond choice \
                     ({SORCERER_ARCANE_BOND_CHOICE_ID} -> {}): the Arcane bloodline's level-1 \
                     power grants {bond_name}. This is a recognition record of the chosen bond \
                     type only; it fabricates no familiar or bonded-item stat block, and no \
                     spell is ever actually cast through it -- this codebase has no \
                     spell-casting-resolution engine anywhere, for any class, so \"cast any one \
                     of your spells known\" can never be triggered here regardless of build \
                     (+0)",
                    arcane_bond_selection.expect("checked by recognized_arcane_bond_name above")
                ),
            });
            explanations.push(ComputationExplanation {
                id: "class_feature.sorcerer.arcane_bloodline.arcane_bond_uses_per_day".to_owned(),
                value: ARCANE_BOND_USES_PER_DAY,
                detail: format!(
                    "Sorcerer Arcane Bond daily use budget (PF1 Core Rulebook: \"Once per day, \
                     your bonded item allows you to cast any one of your spells known\"): a \
                     flat {ARCANE_BOND_USES_PER_DAY} use per day, not level-scaled. This grounds \
                     only the flat budget value; no consumption is ever tracked, since the \
                     underlying benefit (casting a spell known) can never be exercised in this \
                     codebase regardless (no spell-casting-resolution engine exists for any \
                     class), so there is nothing for a consumption count to meaningfully gate"
                ),
            });
            explanations.push(ComputationExplanation {
                id: "class_feature.sorcerer.arcane_bloodline.bloodline_arcana_absent".to_owned(),
                value: 0,
                detail: "Sorcerer Arcane bloodline arcana (PF1 Core Rulebook: \"Whenever you \
                     apply a metamagic feat to a spell that increases the slot used by at least \
                     one level, increase the spell's DC by +1\") is vacuous under this bounded \
                     seam: SpellSelection carries no metamagic field or slot-level-override \
                     concept anywhere in this codebase, so a metamagic feat can never be \
                     represented as applied to a known spell here. The precondition is not \
                     merely unclaimed -- it is unrepresentable, so it never arises for any \
                     input this bounded slice can compute. This record documents that \
                     correction only; it carries no mechanical value (+0)"
                    .to_owned(),
            });
            // The bonus spells, bonus feats, and 3rd/9th/15th/20th-level
            // bloodline powers this seam used to name-but-not-compute. The
            // record that used to sit here said a later widening "must re-block
            // on this specific record rather than silently treat it as still
            // absent"; that instruction is honoured by grounding the values
            // rather than by deleting the claim — see
            // `ground_sorcerer_arcane_bloodline_progression`, which keeps the
            // below-gate absence records AND supplies the at-grant magnitudes,
            // every one transcribed from the corpus.
            ground_sorcerer_arcane_bloodline_progression(
                sorcerer_level,
                explanations,
                diagnostics,
            );
        } else {
            // Reachable only when the bloodline itself is unrecognized, or when
            // it is Arcane but no Arcane Bond choice was recorded — the
            // `recognized_arcane_bloodline && recognized_arcane_bond_name
            // .is_some()` case above now grounds instead of blocking, so the
            // former third sub-branch here (which claimed the 3rd+-level bonus
            // spells/feats were unimplemented) is unreachable and has been
            // removed rather than left asserting a gap that has since closed.
            let arcane_bond_clause = if recognized_arcane_bloodline {
                "the Arcane bloodline's level-1 power Arcane Bond (a familiar or a bonded \
                 object) is not recognized as chosen input on this bounded seam, and the \
                 bloodline arcana (+1 spell save DC on spells modified by a metamagic feat \
                 that raises the spell's level) is a conditional effect not resolved until \
                 Arcane Bond itself is recognized"
                    .to_owned()
            } else {
                // Deliberately says "Arcane" nowhere: this branch describes a
                // character whose chosen bloodline this seam did not recognize,
                // and naming the one canonical bloodline here would assert a
                // bloodline-specific fact at a character who never chose it.
                // `sorcerer_level1_with_non_arcane_bloodline_choice_stays_bloodline_agnostic`
                // and its no-bloodline sibling pin exactly that.
                "no bloodline power, bloodline arcana, bloodline class skill grant, or bonus \
                 spells/feats at 3rd+ level are implemented for this character's bloodline; only \
                 the single canonical bloodline this seam recognizes has its progression \
                 grounded anywhere in this codebase"
                    .to_owned()
            };
            let class_skill_clause = if recognized_bloodline_class_skill_name.is_some() {
                ". The bloodline class skill grant (a player's choice of any one Knowledge \
                 skill) is grounded separately above as a recognition record and is no longer \
                 part of this blocker"
                    .to_owned()
            } else if recognized_arcane_bloodline {
                ", plus the bloodline class skill grant (a player's choice of any one \
                 Knowledge skill)"
                    .to_owned()
            } else {
                String::new()
            };
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
                    .to_owned(),
                message: format!(
                    "Sorcerer remains blocked on its bloodline power and progression burden: \
                     {arcane_bond_clause}{class_skill_clause}, so no Sorcerer bloodline-power \
                     support is claimed"
                ),
                claim_blocking: true,
            });
        }

        let unmet = unmet_sorcerer_known_spell_conditions(input, sorcerer_level);
        if unmet.is_empty() {
            ground_sorcerer_known_spells(input, sorcerer_level, explanations);
        } else {
            diagnostics.push(ComputationDiagnostic {
                id: "class_spell.sorcerer.spontaneous.unsupported".to_owned(),
                message: format!(
                    "Sorcerer remains blocked on its spontaneous known-spell posture burden: \
                     Sorcerer is a full spontaneous arcane caster (spells known from 1st level); \
                     unmet known-spell posture: {}",
                    unmet.join("; ")
                ),
                claim_blocking: true,
            });
        }
    }

    let Some(level) = supported_sorcerer_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Sorcerer level-1/
    // level-2 spell-bearing identity. This is a recognition record only; it fabricates
    // no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.sorcerer".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Sorcerer level {level} spell-bearing \
             baseline: the {SORCERER_CLASS_ID}:{level} class identity is acknowledged \
             as a spontaneous arcane spell-bearing class on the rules-core seam rather than an \
             undocumented packet placeholder. This is a bounded recognition record only; it grounds no \
             bloodline power and no spell math (spell slots, spells known, spell DCs, bonus spells, or \
             prepared posture), so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Grounded: the foundational base-attack-bonus / base-save progression pillar.
    // Unlike every other class row in this matrix (Fighter, Barbarian, Monk, Rogue,
    // Paladin, Druid, Cleric, Bard all already ground this pillar), Sorcerer had never
    // had it grounded at all until this SD13-E5 slice. Both formulas were verified
    // against the PF1 Core Rulebook Sorcerer class table (d20pfsrd and the legacy Paizo
    // PRD mirror) before writing this code, reading the raw level 1-6 table rows
    // directly (BAB +0/+1/+1/+2/+2/+3, Fort +0/+0/+1/+1/+1/+2, Ref +0/+0/+1/+1/+1/+2,
    // Will +2/+3/+3/+4/+4/+5) rather than trusting memory or assuming Sorcerer's shape
    // merely because it resembles another spontaneous/spell-bearing class: the level
    // 4/5 BAB values (+2 at both) disambiguate the 1/2-vs-3/4 fraction (level 1 alone
    // floors every fraction to +0) and confirm Sorcerer is 1/2 BAB, UNLIKE the 3/4 BAB
    // shared by Rogue/Monk/Druid/Cleric/Bard, and the raw Fort/Ref/Will columns
    // independently confirm good Will only, poor Fortitude, poor Reflex. A further
    // SD13-E5 slice widens the level-1-only gate to level 2 and extends both formulas
    // via the same formula, without re-derivation, verified independently against the
    // PF1 Core Rulebook Sorcerer class table: level 2 base attack bonus is +1, base
    // saves are +0/+0/+3 (Fortitude/Reflex/Will); the class table's level-2 "Special"
    // column is blank, so no new class feature is gained at 2nd level.
    let level_value = i16::from(level);

    // Grounded (1/2): 1/2-BAB base-attack progression (classlevel / 2) — the Sorcerer's
    // own class table, NOT the 3/4-BAB shape shared by Rogue/Monk/Druid/Cleric/Bard. No
    // PCGen .lst file exists for the Sorcerer class in this repo, so the formula cites
    // the PF1 Core Rulebook Sorcerer class table directly.
    let base_attack_bonus = level_value / 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Sorcerer level {level} base attack bonus from the PF1 Core \
             Rulebook Sorcerer class table's 1/2-BAB progression — UNLIKE the 3/4-BAB shape \
             shared by Rogue/Monk/Druid/Cleric/Bard: classlevel / 2 = {base_attack_bonus}. This \
             is a standalone explanation record; it is not wired into the integrated \
             base_attack_bonus field or into compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — poor Fortitude, poor Reflex, good Will,
    // verified against the PF1 Core Rulebook Sorcerer class table (Fortitude +0, Reflex
    // +0, Will +2 at level 1).
    let good_save = level_value / 2 + 2;
    let poor_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.base_save.fortitude".to_owned(),
        value: poor_save,
        detail: format!(
            "Sorcerer level {level} base Fortitude save (poor save) from the \
             PF1 Core Rulebook Sorcerer class table: classlevel/3 = {poor_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.base_save.reflex".to_owned(),
        value: poor_save,
        detail: format!(
            "Sorcerer level {level} base Reflex save (poor save) from the PF1 \
             Core Rulebook Sorcerer class table: classlevel/3 = {poor_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.base_save.will".to_owned(),
        value: good_save,
        detail: format!(
            "Sorcerer level {level} base Will save (good save) from the PF1 \
             Core Rulebook Sorcerer class table: classlevel/2+2 = {good_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded for real: Eschew Materials is a universal, bloodline-independent bonus
    // feat granted to every 1st-level Sorcerer. It is a boolean feat grant, not a
    // numeric formula, so it carries no fabricated mechanical value; it grounds no
    // bloodline power and no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.eschew_materials".to_owned(),
        value: 0,
        detail: format!(
            "Sorcerer level {level} Eschew Materials bonus feat: the PF1 Core \
             Rulebook grants every Sorcerer the Eschew Materials feat at 1st level regardless of \
             chosen bloodline, letting them cast a spell with a material component costing 1 gp or \
             less without needing that material component. This is a boolean feat grant, not a \
             numeric bonus, so it carries no fabricated mechanical value (+0); it grounds no \
             bloodline power, no bloodline arcana, and no spell math (spell slots, spells known, \
             spell DCs, or bonus spells)"
        ),
    });

    // Grounded for real: the canonical bloodline choice-slot selection is recognized as
    // chosen input, mirroring the Fighter bonus-feat choice-slot / Wizard Scribe Scroll
    // precedent. Recognition only: the Arcane bloodline's level-1 power is Arcane Bond,
    // an execution engine rather than a flat number, so no power value is fabricated.
    let bloodline_selection = choice_selection(input, SORCERER_BLOODLINE_CHOICE_ID);
    let recognized_arcane_bloodline = bloodline_selection == Some(ARCANE_BLOODLINE_SELECTION_ID);
    if let Some(selection) = bloodline_selection {
        let detail = if recognized_arcane_bloodline {
            format!(
                "Sorcerer level {level} bloodline choice recognized: the \
                 canonical deterministic selection ({SORCERER_BLOODLINE_CHOICE_ID} -> \
                 {selection}) names the Arcane bloodline as chosen input on the compute seam. \
                 This is a recognition record of the choice slot only, so it carries no \
                 fabricated mechanical value (+0): the Arcane bloodline's level-1 power is \
                 Arcane Bond (a familiar or a bonded object), an execution engine rather than a \
                 flat number, and neither it nor the bloodline arcana, bloodline class skill \
                 grant, or higher-level bonus spells/feats is grounded here"
            )
        } else {
            format!(
                "Sorcerer level {level} bloodline choice slot is present \
                 ({SORCERER_BLOODLINE_CHOICE_ID} -> {selection}), but only the canonical \
                 deterministic Arcane bloodline selection is recognized on this bounded seam; \
                 no bloodline power is grounded and no mechanical value is fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.sorcerer.bloodline_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    // Grounded for real: the Arcane bloodline's "Class Skill: Knowledge (any one)" grant
    // — a player's choice of any one Knowledge skill, verified against both d20pfsrd and
    // the legacy Paizo PRD mirror, NOT a fixed grant of Knowledge (arcana) specifically —
    // is recognized as chosen input. This is specific to the Arcane bloodline, so it is
    // only recognized when the Arcane bloodline selection itself was recognized above; it
    // is never fabricated for an unrecognized or absent bloodline choice.
    let bloodline_class_skill_selection =
        choice_selection(input, SORCERER_BLOODLINE_CLASS_SKILL_CHOICE_ID);
    let recognized_bloodline_class_skill_name = bloodline_class_skill_selection
        .filter(|_| recognized_arcane_bloodline)
        .and_then(knowledge_skill_display_name);
    if recognized_arcane_bloodline
        && let Some(selection) = bloodline_class_skill_selection
    {
        let detail = if let Some(skill_name) = &recognized_bloodline_class_skill_name {
            format!(
                "Sorcerer level {level} Arcane bloodline class-skill choice \
                 recognized: the canonical deterministic selection \
                 ({SORCERER_BLOODLINE_CLASS_SKILL_CHOICE_ID} -> {selection}) names {skill_name} \
                 as the player's chosen class skill. The PF1 Core Rulebook Arcane bloodline \
                 grants \"Class Skill: Knowledge (any one)\" — a player's choice of any one \
                 Knowledge skill, not a fixed grant of Knowledge (arcana) specifically. This is \
                 a recognition record of the choice slot only, so it carries no fabricated \
                 mechanical value (+0): granting a class skill confers no flat modifier by \
                 itself in this codebase (no skill-rank allocation or untrained-skill-use engine \
                 exists here), so no skill-check total is computed or fabricated"
            )
        } else {
            format!(
                "Sorcerer level {level} Arcane bloodline class-skill choice \
                 slot is present ({SORCERER_BLOODLINE_CLASS_SKILL_CHOICE_ID} -> {selection}), \
                 but only a \"knowledge:<skill>\"-shaped selection is recognized as the PF1 Core \
                 Rulebook's \"Class Skill: Knowledge (any one)\" grant on this bounded seam; no \
                 class-skill identity is grounded and no mechanical value is fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.sorcerer.bloodline_class_skill_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    // (v0.6 alpha swarm, risks item 8, fifth slice, 2026-07-25) The
    // bloodline-power blocker itself is now pushed unconditionally at the
    // top of this function (see that push site's own doc comment for why:
    // it must fire regardless of race/multiclass once table_class_id
    // recognizes Sorcerer). `recognized_arcane_bloodline`/
    // `recognized_bloodline_class_skill_name` still drive the two
    // choice-recognition EXPLANATIONS above (unaffected, still real,
    // non-fabricated +0 records); only the redundant second diagnostic
    // push that used to live here (with bloodline-specific message
    // wording) was removed, since a single diagnostic ID pushed twice
    // would be confusing, not more informative.

    // SD13-E5: the spontaneous spell-level ACCESS ladder, mirroring the
    // Paladin/Bard spell_level_access records and the Cleric/Wizard
    // <CLASS>_<N>TH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL threshold doctrine
    // exactly ("first non-'—' spells-per-day column", verified against the
    // raw table rows of both primary sources). This grounds the highest
    // ACCESSIBLE sorcerer spell level only; the per-day counts themselves
    // are never computed. Cantrips (0th level, "spells known" only) are
    // outside the spells-per-day ladder and are not counted.
    let sorcerer_spell_level_access: i16 =
        if level >= SORCERER_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            9
        } else if level >= SORCERER_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            8
        } else if level >= SORCERER_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            7
        } else if level >= SORCERER_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            6
        } else if level >= SORCERER_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            5
        } else if level >= SORCERER_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            4
        } else if level >= SORCERER_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            3
        } else if level >= SORCERER_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
            2
        } else {
            1
        };
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.spontaneous.spell_level_access".to_owned(),
        value: sorcerer_spell_level_access,
        detail: format!(
            "Sorcerer spell-level access at sorcerer level {level}: the highest sorcerer spell \
             level (1st+) with a non-\"—\" spells-per-day column in the PF1 Core Rulebook \
             Sorcerer class table is {sorcerer_spell_level_access} (verified against the raw \
             table rows of the primary-source fetches: 1st-level spells from level 1 — \
             the ladder has no zero step — 2nd-level at \
             {SORCERER_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 3rd-level at \
             {SORCERER_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 4th-level at \
             {SORCERER_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 5th-level at \
             {SORCERER_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 6th-level at \
             {SORCERER_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 7th-level at \
             {SORCERER_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 8th-level at \
             {SORCERER_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}, 9th-level at \
             {SORCERER_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL} — the sorcerer's two-level \
             cadence continuing exactly). Cantrips are \"spells known\" only and sit outside \
             the spells-per-day ladder, so they are not counted. This grounds the access \
             ladder only: no spells-per-day counts, no spells-known posture, no bonus slots \
             from a high Charisma, and no spell save DCs are computed"
        ),
    });

    // SD13-E5: the BASE spells-per-day counts, one record per ACCESSIBLE
    // spell level, as a literal table lookup mirroring the
    // Paladin/Ranger/Bard per-day slices and the Cleric domain-slot-count
    // precedent — the PF1 spells-per-day table is a lookup table, not
    // arithmetic, so no formula is invented for it. Verified against the
    // raw table rows of both primary sources (identical on d20pfsrd and
    // legacy.aonprd.com): level 1 "3/—/—/—/—/—", level 2 "4/—/—/—/—/—",
    // level 3 "5/—/—/—/—/—", level 4 "6/3/—/—/—/—", level 5 "6/4/—/—/—/—",
    // level 6 "6/5/3/—/—/—", level 7 "6/6/4/—/—/—", level 8
    // "6/6/5/3/—/—", level 9 "6/6/6/4/—/—", level 10 "6/6/6/5/3/—",
    // level 11 "6/6/6/6/4/—", level 12 "6/6/6/6/5/3", level 13 "6/6/6/6/6/4",
    // level 14 "6/6/6/6/6/5/3", level 15 "6/6/6/6/6/6/4", level 16
    // "6/6/6/6/6/6/5/3", level 17 "6/6/6/6/6/6/6/4"
    // (level 13 was an SD18 widening verified independently against all
    // three primary-source fetches that slice performed — d20pfsrd,
    // aonprd.com, and legacy.aonprd.com, all identical: the 5th-level column
    // rose from 5 to 6 and the 6th-level column rose from 3 to 4, with no
    // genuinely new spell-level column at level 13. Level 14 widened the
    // 6th-level column from 4 to 5 AND opened a genuinely new 7th-level
    // column at 3 — the sorcerer's two-level cadence (4/6/8/10/12/14)
    // continuing exactly. Level 15 widened the 6th-level column from 5 to 6
    // AND the 7th-level column from 3 to 4, with no genuinely new
    // spell-level column opening. Level 16 widened the 7th-level column
    // from 4 to 5 AND a genuinely new 8th-level column opened at 3 —
    // verified independently against d20pfsrd and the Archives of Nethys
    // aonprd.com mirror, both byte-for-byte identical — the sorcerer's
    // two-level cadence (4/6/8/10/12/14/16) continuing exactly, mirroring
    // the Wizard's own 8th-level-column-opening cycle. Level 17 is THIS
    // SD18 slice's widening (cycle-2026-07-15T14100), verified independently
    // against d20pfsrd and the Archives of Nethys aonprd.com mirror, both
    // byte-for-byte identical (fetching the full levels-15-through-19 block
    // in one pass to rule out level-misattribution): the 7th-level column
    // rises from 5 to 6 AND the 8th-level column rises from 3 to 4, with no
    // genuinely new spell-level column opening. Level 18 is THIS SD18
    // slice's widening (cycle-2026-07-16T0400), verified independently
    // against THREE primary sources (a raw non-AI-summarized parse of
    // d20pfsrd.com's own HTML table, the Archives of Nethys aonprd.com
    // mirror, and legacy.aonprd.com, all byte-for-byte identical): the
    // 8th-level column rises from 4 to 5 AND a genuinely NEW 9th-level
    // column opens at 3 for the first time — this definitively resolves a
    // flag carried forward across multiple prior cycles that suspected this
    // 9th-level column was a tool artifact; it is not (see the
    // SORCERER_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL doc comment for the
    // full resolution). Level 19 is THIS SD18 slice's widening
    // (cycle-2026-07-16T4900), verified independently against two primary
    // sources (raw d20pfsrd parse and the Archives of Nethys aonprd.com
    // mirror, byte-for-byte identical): the 8th-level column rises from 5 to
    // 6 AND the 9th-level column rises from 3 to 4, with no genuinely new
    // spell-level column opening (the 9th-level column already opened at
    // level 18). Level 20 is THIS SD18 slice's widening
    // (cycle-2026-07-16T1503), the FINAL level within PF1's 1-20
    // character-level cap, verified independently against two primary
    // sources (raw d20pfsrd parse and the Archives of Nethys aonprd.com
    // mirror, byte-for-byte identical): the 9th-level column rises from 4 to
    // 6, with no genuinely new spell-level column opening.
    // Like the Bard and
    // unlike the
    // Paladin/Ranger, there are NO "0" entries at levels 1-18; every
    // accessible column carries a positive base count. Inaccessible spell
    // levels ("—" columns) get no record at all. Only the base counts are
    // grounded: bonus spells per day from a high Charisma are never
    // computed, and spells KNOWN (a separate table) stays untouched.
    let sorcerer_base_spells_per_day: [Option<i16>; 9] = match level {
        1 => [Some(3), None, None, None, None, None, None, None, None],
        2 => [Some(4), None, None, None, None, None, None, None, None],
        3 => [Some(5), None, None, None, None, None, None, None, None],
        4 => [Some(6), Some(3), None, None, None, None, None, None, None],
        5 => [Some(6), Some(4), None, None, None, None, None, None, None],
        6 => [Some(6), Some(5), Some(3), None, None, None, None, None, None],
        7 => [Some(6), Some(6), Some(4), None, None, None, None, None, None],
        8 => [Some(6), Some(6), Some(5), Some(3), None, None, None, None, None],
        9 => [Some(6), Some(6), Some(6), Some(4), None, None, None, None, None],
        10 => [Some(6), Some(6), Some(6), Some(5), Some(3), None, None, None, None],
        11 => [Some(6), Some(6), Some(6), Some(6), Some(4), None, None, None, None],
        12 => [Some(6), Some(6), Some(6), Some(6), Some(5), Some(3), None, None, None],
        13 => [Some(6), Some(6), Some(6), Some(6), Some(6), Some(4), None, None, None],
        14 => [Some(6), Some(6), Some(6), Some(6), Some(6), Some(5), Some(3), None, None],
        15 => [Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(4), None, None],
        16 => [Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(5), Some(3), None],
        17 => [Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(4), None],
        18 => [Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(5), Some(3)],
        19 => [Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(4)],
        20 => [Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(6), Some(6)],
        _ => [None, None, None, None, None, None, None, None, None],
    };
    for (index, base_count) in sorcerer_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = index + 1;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_{spell_level}"
            ),
            value: *base_count,
            detail: format!(
                "Sorcerer base spells per day at sorcerer level {level}, spell level \
                 {spell_level}: {base_count}, read directly from the PF1 Core Rulebook \
                 Sorcerer class table's spells-per-day row (verified against the raw table \
                 rows of both primary sources; a literal table lookup, not a derived \
                 formula; the Sorcerer table has no \"0\" entries at levels 1-10). This \
                 grounds the base count only: bonus spells per day from a high Charisma are \
                 never computed, spells KNOWN (a separate table) is not grounded, and no \
                 spell save DCs are computed"
            ),
        });
    }

    // SD13-E5: the base spell-save-DC arithmetic, one record per ACCESSIBLE
    // spell level, mirroring the Bard Fascinate DC's 10-plus-modifier idiom.
    // Verified against both primary sources, which state the rule
    // identically: "The Difficulty Class for a saving throw against a
    // sorcerer's spell is 10 + the spell level + the sorcerer's Charisma
    // modifier." This grounds only the base formula over values already on
    // the seam (the chosen-ability Charisma modifier and the access
    // ladder): no saving-throw resolution, no target, no spell selection,
    // and no bloodline-arcana or feat DC modifiers are computed.
    let sorcerer_charisma_modifier = ability_modifier_for(ability_modifiers, "charisma");
    for spell_level in 1..=sorcerer_spell_level_access {
        let spell_save_dc = 10 + spell_level + sorcerer_charisma_modifier;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.sorcerer.spontaneous.spell_save_dc.spell_level_{spell_level}"
            ),
            value: spell_save_dc,
            detail: format!(
                "Sorcerer spell save DC at sorcerer level {level}, spell level {spell_level}: \
                 10 + {spell_level} + Charisma modifier {sorcerer_charisma_modifier} = \
                 {spell_save_dc} (PF1 Core Rulebook, verified identically on both primary \
                 sources: \"The Difficulty Class for a saving throw against a sorcerer's \
                 spell is 10 + the spell level + the sorcerer's Charisma modifier\"). This \
                 grounds the base DC formula only: no saving-throw resolution, no target, no \
                 spell selection, and no bloodline-arcana or feat DC modifiers are computed"
            ),
        });
    }

    // SD13-E5: the BASE spells-KNOWN counts, one record per spell level with
    // a non-"—" column in the Sorcerer Spells Known table, as a literal
    // table lookup mirroring the Bard known slice. Verified against the raw
    // table rows of both primary sources (identical on d20pfsrd and
    // legacy.aonprd.com): level 1 "4/2/—/—/—/—/—", level 2 "5/2/—/—/—/—/—",
    // level 3 "5/3/—/—/—/—/—", level 4 "6/3/1/—/—/—/—", level 5
    // "6/4/2/—/—/—/—", level 6 "7/4/2/1/—/—/—", level 7 "7/5/3/2/—/—/—",
    // level 8 "8/5/3/2/1/—/—", level 9 "8/5/4/3/2/—/—", level 10
    // "9/5/4/3/2/1/—", level 11 "9/5/5/4/3/2/—", level 12 "9/5/5/4/3/2/1",
    // level 13 "9/5/5/4/4/3/2", level 14 "9/5/5/4/4/3/2/1",
    // level 15 "9/5/5/4/4/4/3/2", level 16 "9/5/5/4/4/4/3/2/1", level 17
    // "9/5/5/4/4/4/3/3/2"
    // (0th through 8th spell level; the level-13 row was an SD18 widening
    // verified independently against all three primary-source fetches that
    // slice performed — d20pfsrd, aonprd.com, and legacy.aonprd.com, all
    // identical: the 0th through 3rd columns stayed numerically unchanged
    // from level 12's row, and the 4th, 5th, and 6th-level columns each
    // rose by one. Level 14 widened the 0th through 6th columns numerically
    // unchanged from level 13's row, AND opened a genuinely new 7th-level
    // column at 1. Level 15 widened the 0th through 4th columns numerically
    // unchanged from level 14's row, while the 5th, 6th, and 7th-level
    // columns each rose by one — no genuinely new spell-level column
    // opened. Level 16 widened the 0th through 7th columns numerically
    // unchanged from level 15's row, while a genuinely new 8th-level
    // column opened at 1 — verified independently against d20pfsrd and the
    // Archives of Nethys aonprd.com mirror, both byte-for-byte identical —
    // mirroring the same-cycle opening of the 8th-level column on the
    // spells-per-day table above. Level 17 is the loop's cycle-2026-07-15T14100
    // widening, verified independently against d20pfsrd and the Archives of
    // Nethys aonprd.com mirror, both byte-for-byte identical (fetching the
    // full levels-15-through-19 block in one pass to rule out
    // level-misattribution): the 0th through 6th columns stay numerically
    // unchanged from level 16's row, while the 7th-level column rises from
    // 2 to 3 AND the 8th-level column rises from 1 to 2 — no genuinely new
    // spell-level column opens. Level 18 is THIS SD18 slice's widening
    // (cycle-2026-07-16T0400), verified independently against THREE primary
    // sources (a raw non-AI-summarized parse of d20pfsrd.com's own HTML
    // table, the Archives of Nethys aonprd.com mirror, and
    // legacy.aonprd.com, all byte-for-byte identical): the 0th through 8th
    // columns stay numerically unchanged from level 17's row, while a
    // genuinely NEW 9th-level column opens at 1 for the first time — this
    // definitively resolves the multi-cycle-carried-forward "premature
    // 9th-level column" flag (see the
    // SORCERER_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL doc comment for the
    // full resolution). Level 19 is THIS SD18 slice's widening
    // (cycle-2026-07-16T4900), verified independently against two primary
    // sources (raw d20pfsrd parse and the Archives of Nethys aonprd.com
    // mirror, byte-for-byte identical): the 0th through 7th columns stay
    // numerically unchanged from level 18's row, while the 8th-level column
    // rises from 2 to 3 AND the 9th-level column rises from 1 to 2, with no
    // genuinely new spell-level column opening. Level 20 is THIS SD18
    // slice's widening (cycle-2026-07-16T1503), the FINAL level within
    // PF1's 1-20 character-level cap, verified independently against two
    // primary sources (raw d20pfsrd parse and the Archives of Nethys
    // aonprd.com mirror, byte-for-byte identical): the 0th through 8th
    // columns stay numerically unchanged from level 19's row, while the
    // 9th-level column rises from 2 to 3, with no genuinely new spell-level
    // column opening.
    // The known table
    // INCLUDES the 0th level (cantrips are "spells
    // known" only), and its new-spell-level cadence matches the grounded
    // per-day access ladder exactly (2nd at 4, 3rd at 6, 4th at 8, 5th at
    // 10, 6th at 12, 7th at 14, 8th at 16, 9th at 18 — checked rather than
    // assumed).
    // Only the known COUNTS
    // are grounded: the selection of WHICH spells are known is never
    // computed, and the 3rd/5th/7th/9th/11th/13th/15th/17th-level bloodline
    // bonus spells are part of the still-unproven bloodline progression
    // burden, not this table.
    let sorcerer_spells_known: [Option<i16>; 10] = match level {
        1 => [Some(4), Some(2), None, None, None, None, None, None, None, None],
        2 => [Some(5), Some(2), None, None, None, None, None, None, None, None],
        3 => [Some(5), Some(3), None, None, None, None, None, None, None, None],
        4 => [Some(6), Some(3), Some(1), None, None, None, None, None, None, None],
        5 => [Some(6), Some(4), Some(2), None, None, None, None, None, None, None],
        6 => [Some(7), Some(4), Some(2), Some(1), None, None, None, None, None, None],
        7 => [Some(7), Some(5), Some(3), Some(2), None, None, None, None, None, None],
        8 => [Some(8), Some(5), Some(3), Some(2), Some(1), None, None, None, None, None],
        9 => [Some(8), Some(5), Some(4), Some(3), Some(2), None, None, None, None, None],
        10 => [Some(9), Some(5), Some(4), Some(3), Some(2), Some(1), None, None, None, None],
        11 => [Some(9), Some(5), Some(5), Some(4), Some(3), Some(2), None, None, None, None],
        12 => [Some(9), Some(5), Some(5), Some(4), Some(3), Some(2), Some(1), None, None, None],
        13 => [Some(9), Some(5), Some(5), Some(4), Some(4), Some(3), Some(2), None, None, None],
        14 => [Some(9), Some(5), Some(5), Some(4), Some(4), Some(3), Some(2), Some(1), None, None],
        15 => [Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(2), None, None],
        16 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(2), Some(1), None,
        ],
        17 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None,
        ],
        18 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2),
            Some(1),
        ],
        19 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(3),
            Some(2),
        ],
        20 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(3),
            Some(3),
        ],
        _ => [None, None, None, None, None, None, None, None, None, None],
    };
    for (spell_level, known_count) in sorcerer_spells_known.iter().enumerate() {
        let Some(known_count) = known_count else {
            continue;
        };
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.sorcerer.spontaneous.spells_known.spell_level_{spell_level}"
            ),
            value: *known_count,
            detail: format!(
                "Sorcerer base spells known at sorcerer level {level}, spell level \
                 {spell_level}: {known_count}, read directly from the PF1 Core Rulebook \
                 Sorcerer Spells Known table (verified against the raw table rows of both \
                 primary sources; a literal table lookup, not a derived formula; unlike the \
                 spells-per-day table, this table includes the 0th level — cantrips are \
                 spells known only). This grounds the base known count only: the selection \
                 of WHICH spells are known is never computed — no spell-list content, no \
                 spell identities, no swap/retraining rules, and no bloodline bonus-spell \
                 additions (those are part of the still-unproven bloodline progression \
                 burden)"
            ),
        });
    }

    // SD13-E5: the bonus spells per day from a high Charisma, one record
    // per ACCESSIBLE spell level (1st+; the bonus table has no 0th-level
    // column — cantrips never gain bonus spells), from PF1's shared Table:
    // Ability Modifiers and Bonus Spells, verified against both primary
    // sources (legacy.aonprd.com Getting Started / d20pfsrd Ability
    // Scores): modifier +1 grants 1 bonus 1st-level spell, +2 grants 1/1,
    // +3 grants 1/1/1, +4 grants 1/1/1/1, +5 grants 2/1/1/1/1 — for
    // modifier m and spell level N, 0 when m < N, otherwise (m - N)/4 + 1.
    // Both sources state the gating rule identically: "a spellcaster must
    // be of a high enough class level to be able to cast spells of a given
    // spell level" — exactly the grounded access ladder. A computed 0
    // (modifier below the spell level) is an honest arithmetic result,
    // distinct from the base table's literal "0" entries. The bonus is
    // never added to the base per-day counts here — no total is computed;
    // that integration is named as the next uplift.
    for spell_level in 1..=sorcerer_spell_level_access {
        let bonus_spells = if sorcerer_charisma_modifier < spell_level {
            0
        } else {
            (sorcerer_charisma_modifier - spell_level) / 4 + 1
        };
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.sorcerer.spontaneous.bonus_spells_per_day.spell_level_{spell_level}"
            ),
            value: bonus_spells,
            detail: format!(
                "Sorcerer bonus spells per day at sorcerer level {level}, spell level \
                 {spell_level}: {bonus_spells} from Charisma modifier \
                 {sorcerer_charisma_modifier} (PF1 Core Rulebook Table: Ability Modifiers \
                 and Bonus Spells, verified identically on both primary sources; for \
                 modifier m and spell level N the table value is 0 when m < N, otherwise \
                 (m - N)/4 + 1, and bonus spells apply only to spell levels the character \
                 is of a high enough class level to cast — the grounded access ladder). A \
                 computed 0 means the modifier grants no bonus at this spell level; it is \
                 never added to the base per-day count here — no total is computed, no \
                 spell selection, and no spell save DCs"
            ),
        });
    }

    // SD13-E5: the TOTAL spells per day — the pure sum of the two records
    // grounded above (base table count + Charisma bonus count) per
    // ACCESSIBLE spell level. No new rules content: the base and bonus
    // records each carry their own two-source verification; this record
    // integrates them into the sorcerer's actual castable slot count per
    // day. Counts only — no spell selection, no spontaneous-casting
    // execution, no slot consumption or tracking, no save resolution.
    for (index, base_count) in sorcerer_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        let spell_level = (index + 1) as i16;
        let bonus_spells = if sorcerer_charisma_modifier < spell_level {
            0
        } else {
            (sorcerer_charisma_modifier - spell_level) / 4 + 1
        };
        let total_spells = base_count + bonus_spells;
        explanations.push(ComputationExplanation {
            id: format!(
                "class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_{spell_level}"
            ),
            value: total_spells,
            detail: format!(
                "Sorcerer total spells per day at sorcerer level {level}, spell level \
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

    // (v0.6 alpha swarm, risks item 8, fifth slice, 2026-07-25) The spell
    // posture blocker is now pushed conditionally at the top of this
    // function (real validation, see that push site's own doc comment) --
    // the redundant unconditional push that used to live here was removed.
}

/// Grounds the Arcane bloodline's entire 3rd-level-and-above progression: the
/// bonus spells, the bonus-feat pool, and the 3rd/9th/15th/20th-level bloodline
/// powers (2026-07-29, Sorcerer levels 3-20 closure).
///
/// **What this replaced.** This progression used to be the sole reason a
/// Sorcerer could not compute above 2nd level: the previous seam emitted a
/// `bonus_spells_and_feats_absent` level-gate-absence record below 3rd, and at
/// 3rd and above fell through to the claim-blocking
/// `arcane_bond_and_bloodline_progression.unsupported` diagnostic. That record's
/// own text instructed whoever widened the seam that they "must re-block on this
/// specific record rather than silently treat it as still absent" — a real
/// constraint, and it is honoured literally: the gap is not treated as absent
/// and the diagnostic is not weakened. It is closed, by transcribing every value
/// from the corpus record that had never been read.
///
/// **Canonical narrowing.** Only the Arcane bloodline is grounded — the same
/// canonical-narrowing treatment already ratified for Oracle's Mystery, Cleric's
/// Good domain, and Wizard's Abjuration school, and already the recognized
/// bloodline for every other Sorcerer pillar in this file. The other CRB
/// bloodlines' progressions stay genuinely ungrounded, and a Sorcerer whose
/// bloodline this seam does not recognize as Arcane never reaches this function
/// at all, so no Arcane-specific grant is ever fabricated for them.
///
/// **Every record's shape.** Each of the five magnitudes carries its own real
/// number at or above its grant level and a `0` with explicit
/// "correctly-absent-by-level-gate" wording below it — the Draconic Dragon
/// Resistances / Barbarian Trap Sense idiom, so a reader can always tell an
/// absent grant from a zero one. The single exception is Metamagic Adept at 20th
/// level, where the `0` means *superseded*, not absent, and says so; see below.
///
/// **The 20th-level supersession.** The corpus Arcane Apotheosis record carries
/// `BONUS:VAR|Sorcerer_Arcane_BloodlinePower3|-1`, which switches off the
/// bloodline's own `ABILITY:...|Arcane Bloodline ~ Power LVL 03|PREVARGTEQ:
/// Sorcerer_Arcane_BloodlinePower3,1` grant, and the Metamagic Adept record's
/// uses-per-day `ASPECT:`s are all suppressed by
/// `!PREABILITY:1,CATEGORY=Special Ability,Arcane Bloodline ~ Arcane Apotheosis`.
/// Both agree: at 20th level Arcane Apotheosis replaces Metamagic Adept's
/// per-day budget with an unlimited one. Reporting `5/day` at 20th — what the
/// formula alone yields — would therefore be a wrong number, so this seam
/// reports `0` and explains the supersession in the record itself.
///
/// **What is deliberately NOT claimed**, and is named by a non-claim-blocking
/// diagnostic rather than left implicit:
/// - which of the eight eligible feats fills each bloodline-feat slot (only the
///   count grounds — the ratified Fighter/Cavalier/Brawler treatment),
/// - which sorcerer/wizard spells New Arcana adds (a free chooser over the whole
///   list),
/// - which school of magic School Power names (its `+2` is school-agnostic, so
///   the magnitude is grounded and the label is not — exactly how the sibling
///   Draconic Dragon Resistances grounds its numbers without picking an energy
///   type).
pub(super) fn ground_sorcerer_arcane_bloodline_progression(
    sorcerer_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // --- Bonus spells (3rd, and every odd level through 19th). ---
    let bonus_spells_known = arcane_bloodline_bonus_spells_known(sorcerer_level);
    let bonus_spell_detail = if bonus_spells_known == 0 {
        format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   , PREVARGTEQ:BloodlineCasterLVL,{ARCANE_BLOODLINE_BONUS_LEVEL}
            "Sorcerer Arcane bloodline bonus spells at sorcerer level {sorcerer_level}: none yet, \
             correctly absent by PF1 Core Rulebook level gate (the first bonus spell, Identify, is \
             granted at sorcerer level {ARCANE_BLOODLINE_BONUS_LEVEL}; corpus KEY:Arcane Bloodline ~ \
             Bonus Spells). This is a level-gate absence, not an ungrounded gap: the whole \
             nine-spell progression is transcribed and computed above this gate"
        )
    } else {
        format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   Every entry is transcribed from the corpus record KEY:Arcane Bloodline ~ Bonus
            //   Spells (cr_abilities_class.lst), whose SPELLKNOWN:CLASS|Sorcerer=<spell
            //   level>|<spell> tokens are each gated PREVARGTEQ:BloodlineCasterLVL,<grant level>,
            //   and each spell's level is independently cross-checked against
            //   sorcerer_spell_list::SORCERER_SPELL_LIST.
            "Sorcerer Arcane bloodline bonus spells at sorcerer level {sorcerer_level}: \
             {bonus_spells_known} granted so far — {}. This grounds the COUNT and the named \
             identities of the bonus spells the bloodline adds to spells known; it computes no spell \
             save DC against a target and no casting execution, since no spell-casting-resolution \
             engine exists anywhere in this codebase for any class",
            arcane_bloodline_granted_bonus_spells(sorcerer_level).join(", ")
        )
    };
    explanations.push(ComputationExplanation {
        id: SORCERER_ARCANE_BLOODLINE_BONUS_SPELLS_EXPLANATION_ID.to_owned(),
        value: bonus_spells_known,
        detail: bonus_spell_detail,
    });

    // --- Bonus feats (7th, 13th, 19th). ---
    let bonus_feat_count = arcane_bloodline_bonus_feat_count(sorcerer_level);
    let bonus_feat_detail = if bonus_feat_count == 0 {
        format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:ABILITYPOOL|Sorcerer Bloodline Feat|BloodlineFeatCount
            //   BONUS:VAR|BloodlineFeatCount|(BloodlineFeatProgression-1)/6
            "Sorcerer Arcane bloodline bonus feats at sorcerer level {sorcerer_level}: none yet, \
             correctly absent by PF1 Core Rulebook level gate (the first is granted at 7th \
             level). The corpus sizes the bloodline feat pool as one per six bloodline levels \
             after the first, which is 0 below 7th"
        )
    } else {
        format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   Corpus: BONUS:ABILITYPOOL|Sorcerer Bloodline Feat|BloodlineFeatCount,
            //   BONUS:VAR|BloodlineFeatCount|(BloodlineFeatProgression-1)/6|TYPE=Base,
            //   BONUS:VAR|BloodlineFeatProgression|BloodlineProgressionLVL|TYPE=Base,
            //   BONUS:VAR|BloodlineProgressionLVL|SorcererLVL|TYPE=Base.
            "Sorcerer Arcane bloodline bonus feats at sorcerer level {sorcerer_level}: \
             {bonus_feat_count} slot(s) granted ((sorcerer level - 1)/6 — one at 7th, 13th, and \
             19th). The three corpus `-1` deductions against this same pool are each gated on a \
             Sorcerer_CF_BloodlineFeat<n> archetype flag and this repo ingests no sorcerer \
             archetype, so all three are provably vacuous here. Only the COUNT grounds; which feat \
             fills a slot is not modelled — the ratified Fighter/Cavalier/Brawler bonus-feat \
             treatment. The eight eligible feats for this bloodline (corpus Arcane Bloodline ~ Feat \
             Tracker) are: {}",
            ARCANE_BLOODLINE_ELIGIBLE_BONUS_FEATS.join(", ")
        )
    };
    explanations.push(ComputationExplanation {
        id: SORCERER_ARCANE_BLOODLINE_BONUS_FEAT_COUNT_EXPLANATION_ID.to_owned(),
        value: bonus_feat_count,
        detail: bonus_feat_detail,
    });

    // --- 3rd-level power: Metamagic Adept (superseded at 20th). ---
    let (metamagic_uses, metamagic_detail) = if sorcerer_level
        < ARCANE_BLOODLINE_METAMAGIC_ADEPT_LEVEL
    {
        (
            0,
            format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   ,
                //   PREVARGTEQ:Sorcerer_Arcane_BloodlineProgressionLVL,{ARCANE_BLOODLINE_METAMAGIC_ADEPT_LEVEL}
                "Sorcerer Arcane bloodline Metamagic Adept at sorcerer level {sorcerer_level}: \
                 correctly absent by PF1 Core Rulebook level gate (a \
                 {ARCANE_BLOODLINE_METAMAGIC_ADEPT_LEVEL}rd-level bloodline power; corpus KEY:Arcane \
                 Bloodline ~ Metamagic Adept)"
            ),
        )
    } else if sorcerer_level >= ARCANE_BLOODLINE_ARCANE_APOTHEOSIS_LEVEL {
        (
            0,
            format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   The corpus Arcane Apotheosis record carries
                //   BONUS:VAR|Sorcerer_Arcane_BloodlinePower3|-1, which switches off the
                //   bloodline's own 3rd-level-power grant at 20th level, and Metamagic Adept's
                //   uses-per-day ASPECTs are each suppressed by !PREABILITY:1,CATEGORY=Special
                //   Ability,Arcane Bloodline ~ Arcane Apotheosis.
                "Sorcerer Arcane bloodline Metamagic Adept at sorcerer level {sorcerer_level}: \
                 SUPERSEDED, not absent and not zero-limited. Arcane Apotheosis replaces the per-day \
                 budget with an unlimited one, so reporting the raw formula's {} uses/day here would \
                 be a wrong number",
                arcane_bloodline_metamagic_adept_uses_per_day(sorcerer_level)
            ),
        )
    } else {
        let uses = arcane_bloodline_metamagic_adept_uses_per_day(sorcerer_level);
        (
            uses,
            format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   Corpus:
                //   BONUS:VAR|Sorcerer_ArcaneMetamagicAdept_Times|floor((Sorcerer_Arcane_BloodlinePower3LVL+1)/4).
                "Sorcerer Arcane bloodline Metamagic Adept at sorcerer level {sorcerer_level}: \
                 usable {uses} time(s) per day (floor((sorcerer level + 1)/4) — 1/day at 3rd, rising \
                 at 7th, 11th, 15th, and 19th). Only the daily BUDGET grounds; the benefit itself \
                 (applying a known metamagic feat without increasing casting time) can never be \
                 exercised here, since SpellSelection carries no metamagic field anywhere in this \
                 codebase and no casting-resolution engine exists, so there is nothing for a \
                 consumption count to gate"
            ),
        )
    };
    explanations.push(ComputationExplanation {
        id: SORCERER_ARCANE_BLOODLINE_METAMAGIC_ADEPT_EXPLANATION_ID.to_owned(),
        value: metamagic_uses,
        detail: metamagic_detail,
    });

    // --- 9th-level power: New Arcana. ---
    let (new_arcana, new_arcana_detail) = if sorcerer_level < ARCANE_BLOODLINE_NEW_ARCANA_LEVEL {
        (
            0,
            format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   ,
                //   PREVARGTEQ:Sorcerer_Arcane_BloodlineProgressionLVL,{ARCANE_BLOODLINE_NEW_ARCANA_LEVEL}
                "Sorcerer Arcane bloodline New Arcana at sorcerer level {sorcerer_level}: correctly \
                 absent by PF1 Core Rulebook level gate (a \
                 {ARCANE_BLOODLINE_NEW_ARCANA_LEVEL}th-level bloodline power; corpus KEY:Arcane \
                 Bloodline ~ New Arcana)"
            ),
        )
    } else {
        let count = arcane_bloodline_new_arcana_spell_count(sorcerer_level);
        (
            count,
            format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   Corpus:
                //   BONUS:VAR|Sorcerer_NewArcana_Number|floor((Sorcerer_Arcane_BloodlinePower9LVL-5)/4),
                //   feeding BONUS:ABILITYPOOL|New Arcana|Sorcerer_NewArcana_Number.
                "Sorcerer Arcane bloodline New Arcana at sorcerer level {sorcerer_level}: {count} \
                 additional spell(s) may be added to spells known (floor((sorcerer level - 5)/4) — \
                 one at 9th, a second at 13th, a third at 17th). Only the COUNT grounds; WHICH \
                 sorcerer/wizard spells are added is a free chooser over the whole list and is not \
                 modelled, mirroring how Brawler's Martial Flexibility grounds its pool without \
                 seeding a canonical feat"
            ),
        )
    };
    explanations.push(ComputationExplanation {
        id: SORCERER_ARCANE_BLOODLINE_NEW_ARCANA_EXPLANATION_ID.to_owned(),
        value: new_arcana,
        detail: new_arcana_detail,
    });

    // --- 15th-level power: School Power. ---
    let (school_power_bonus, school_power_detail) =
        if sorcerer_level < ARCANE_BLOODLINE_SCHOOL_POWER_LEVEL {
            (
                0,
                format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   ,
                    //   PREVARGTEQ:Sorcerer_Arcane_BloodlineProgressionLVL,{ARCANE_BLOODLINE_SCHOOL_POWER_LEVEL}
                    "Sorcerer Arcane bloodline School Power at sorcerer level {sorcerer_level}: \
                     correctly absent by PF1 Core Rulebook level gate (a \
                     {ARCANE_BLOODLINE_SCHOOL_POWER_LEVEL}th-level bloodline power; corpus \
                     KEY:Arcane Bloodline ~ School Power Choice)"
                ),
            )
        } else {
            (
                SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_BONUS,
                format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   Corpus: KEY:Arcane Bloodline ~ School Power Choice,
                    //   BONUS:DC|SCHOOL.%LIST|2|TYPE=SchoolPower.
                    "Sorcerer Arcane bloodline School Power at sorcerer level {sorcerer_level}: \
                     +{SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_BONUS} to the spell save DC of \
                     spells from one chosen school of magic, stacking with Spell Focus. This grounds \
                     the magnitude as a school-agnostic fact — the number is identical whichever \
                     school the CHOOSE:SCHOOLS|ALL sub-choice names, so no canonical school is \
                     fabricated here, exactly as the sibling Draconic Dragon Resistances grounds its \
                     two numbers without picking an energy type. It is not folded into any spell-DC \
                     total: no spell-save-DC-resolution engine against a target exists anywhere in \
                     this codebase for any class"
                ),
            )
        };
    explanations.push(ComputationExplanation {
        id: SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_EXPLANATION_ID.to_owned(),
        value: school_power_bonus,
        detail: school_power_detail,
    });

    // --- 20th-level capstone: Arcane Apotheosis (genuinely zero-magnitude). ---
    // Emitted only at its own grant level. Unlike the five records above, a
    // "0 = correctly absent" record below 20th would be indistinguishable from
    // this record's true magnitude, which really is zero: the corpus Arcane
    // Apotheosis record's only BONUS token is the
    // `BONUS:VAR|Sorcerer_Arcane_BloodlinePower3|-1` supersession flag already
    // accounted for in the Metamagic Adept record above. Its whole benefit is a
    // resolution, so this is a bounded grant-only identity record quoting the
    // real corpus DESC — the same idiom Rogue's Master Strike already uses in
    // this file. It is deliberately NOT routed through
    // `description_completion::feat_description_completion`: that module resolves
    // FEATS against the CRB feat catalog and certifies the Feats tab renders
    // them, and Arcane Apotheosis is a class feature carried on no such surface,
    // so claiming its text reaches the player would be exactly the unearned
    // `success: true` that module exists to prevent.
    if sorcerer_level >= ARCANE_BLOODLINE_ARCANE_APOTHEOSIS_LEVEL {
        explanations.push(ComputationExplanation {
            id: SORCERER_ARCANE_BLOODLINE_ARCANE_APOTHEOSIS_EXPLANATION_ID.to_owned(),
            value: 0,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   For every three levels of spell slots that you expend, you consume one less
                //   charge when using a magic item that expends charges.\" This is a bounded
                //   grant-only identity record (value 0, non-fabricated): the record's only numeric
                //   corpus token is BONUS:VAR|Sorcerer_Arcane_BloodlinePower3|-1, the Metamagic
                //   Adept supersession already grounded in that record above.
                "Sorcerer Arcane bloodline capstone Arcane Apotheosis, granted at sorcerer level \
                 {ARCANE_BLOODLINE_ARCANE_APOTHEOSIS_LEVEL} (corpus KEY:Arcane Bloodline ~ Arcane \
                 Apotheosis): \"Your body surges with arcane power. You can add any metamagic feats \
                 that you know to your spells without increasing their casting time, although you \
                 must still expend higher-level spell slots. Whenever you use magic items that \
                 require charges, you can instead expend spell slots to power the item. The \
                 remaining benefits have no magnitude to compute — no metamagic-application engine \
                 and no magic-item-charge engine exists anywhere in this codebase — so this grounds \
                 the grant and its real rulebook text, and no charge arithmetic"
            ),
        });
    }

    // The sub-choices this progression deliberately leaves to the player,
    // named explicitly rather than left implicit. Non-claim-blocking, mirroring
    // the Draconic Dragon Resistances energy-type diagnostic exactly: none of
    // these changes any magnitude grounded above, so none of them is a gap in
    // what this seam claims — but a reader deserves to be told which labels are
    // still unfilled. Emitted only once the first such sub-choice actually
    // arises (the 7th-level bloodline feat).
    if sorcerer_level >= ARCANE_BLOODLINE_FIRST_BONUS_FEAT_LEVEL {
        diagnostics.push(ComputationDiagnostic {
            id: SORCERER_ARCANE_BLOODLINE_SUBCHOICES_DIAGNOSTIC_ID.to_owned(),
            message: format!(
                "Sorcerer Arcane bloodline progression at sorcerer level {sorcerer_level}: every \
                 magnitude is grounded above from the corpus, but three player sub-choices stay \
                 unresolved on this bounded seam and no canonical value is fabricated for any of \
                 them — which of the eight eligible bloodline feats fills each granted slot, \
                 which sorcerer/wizard spells New Arcana adds, and which school of magic School \
                 Power names. None of the three changes a grounded number: the feat and New \
                 Arcana records ground counts only, and School Power's +\
                 {SORCERER_ARCANE_BLOODLINE_SCHOOL_POWER_DC_BONUS} is identical for every school"
            ),
            claim_blocking: false,
        });
    }
}

/// PF1 Core Rulebook Draconic bloodline's 3rd-level "Dragon Resistances" power
/// natural armor bonus magnitude, verified against the corpus source
/// (cr_abilities_class.lst, KEY:Draconic Bloodline ~ Dragon Resistances):
///
///   BONUS:VAR|Sorcerer_DraconicDragonResistances_NaturalArmorBonus|
///       min(floor((LVL-3)/6)+1,3)
///   BONUS:VAR|Sorcerer_DraconicDragonResistances_NaturalArmorBonus|1|
///       PREVARGTEQ:...,15
///
/// TWO separate corpus `BONUS:VAR` lines both feed the SAME
/// `Sorcerer_DraconicDragonResistances_NaturalArmorBonus` variable, and PCGen
/// accumulates same-named `BONUS:VAR` contributions additively -- a real, repeated
/// corpus pattern, not a single-line override -- so the total returned here is the
/// SUM of both lines: 1 at level 3, 2 at level 9, and 4 (the first line's value of 3,
/// capped, PLUS the second line's additional +1) at level 15 and higher. Grounding
/// only the first line would (incorrectly) cap the level-15+ total at 3.
///
/// Callers must only invoke this at `level >= SORCERER_DRACONIC_DRAGON_RESISTANCES_LEVEL`
/// (3): `level - 3` is computed as a signed value and would floor incorrectly toward 0
/// for a negative numerator below that gate.
pub(super) fn sorcerer_draconic_dragon_resistances_natural_armor_bonus(level: u8) -> i16 {
    let level = i16::from(level);
    let base = ((level - 3) / 6 + 1).min(3);
    let level_15_bonus = i16::from(level >= 15);
    base + level_15_bonus
}

/// PF1 Core Rulebook Draconic bloodline's 3rd-level "Dragon Resistances" power
/// energy resistance bonus magnitude, verified against the corpus source
/// (cr_abilities_class.lst, KEY:Draconic Bloodline ~ Dragon Resistances):
///
///   BONUS:VAR|Sorcerer_DraconicDragonResistances_ResistanceBonus|
///       min(floor((LVL-3)/6)+1,2)*5
///
/// 5 at level 3, 10 at level 9 and higher. Same `level >= 3` precondition as
/// `sorcerer_draconic_dragon_resistances_natural_armor_bonus`.
pub(super) fn sorcerer_draconic_dragon_resistances_energy_resistance_bonus(level: u8) -> i16 {
    let level = i16::from(level);
    ((level - 3) / 6 + 1).min(2) * 5
}

/// Whether `input` is a Sorcerer who has reached the Draconic Bloodline's 3rd-level
/// "Dragon Resistances" power, and if so, its natural armor bonus magnitude, ready to
/// layer into `compute_combat_baseline` (task #61, 2026-07-28). Mirrors
/// `active_brawler_ac_bonus`/`active_oracle_natures_whispers_ac_bonus`'s exact
/// class-ownership-gated-by-construction shape: no `class_ability_activations` check,
/// since Dragon Resistances is a permanent (Ex) quality once granted, not an on/off
/// activation -- gated instead by class ownership, the level-3 gate, AND (unlike
/// Brawler, but like Oracle's own revelation gate) a recognized choice, since this
/// power is specific to the Draconic bloodline and must never be fabricated for the
/// Arcane bloodline or any other/absent bloodline selection. Uses
/// `supported_sorcerer_level` (the same single-class, level-1..=20 gate every other
/// Sorcerer pillar in this file already uses) rather than a bare `class_levels` scan,
/// so this stays consistent with the sibling facts in
/// `explain_sorcerer_level1_spell_baseline`. Deliberately does NOT gate on
/// `HUMAN_RACE_ID`: unlike the older SD13-era `explain_sorcerer_level1_spell_baseline`
/// pillars, this is a v0.6 alpha swarm addition and Dragon Resistances has no
/// race-specific text in the PF1 corpus, so it is race-agnostic like every other
/// `active_<class>_<ability>_bonus` function this session added (Brawler AC Bonus,
/// Oracle Nature's Whispers, Alchemist Mutagen).
pub(super) fn active_sorcerer_draconic_dragon_resistances_natural_armor_bonus(
    input: &CharacterInput,
) -> Option<i16> {
    let level = supported_sorcerer_level(input)?;
    if choice_selection(input, SORCERER_BLOODLINE_CHOICE_ID) != Some(DRACONIC_BLOODLINE_SELECTION_ID)
    {
        return None;
    }
    if level < SORCERER_DRACONIC_DRAGON_RESISTANCES_LEVEL {
        return None;
    }
    Some(sorcerer_draconic_dragon_resistances_natural_armor_bonus(level))
}

/// Applies Draconic Bloodline Dragon Resistances' natural armor bonus to
/// `base_armor_class` when `input` is a Draconic-bloodline Sorcerer at or above the
/// power's level-3 gate (task #61, 2026-07-28). Mirrors
/// `apply_brawler_ac_bonus_to_combat_baseline`'s exact shape: a small helper (rather
/// than an inline `if active_<x>(...).is_some() { CONST } else { 0 }` ternary, since
/// the magnitude here is level-dependent, not a single fixed value), called directly
/// from `compute_combat_baseline`.
pub(super) fn apply_sorcerer_draconic_dragon_resistances_ac_bonus_to_combat_baseline(
    input: &CharacterInput,
) -> i16 {
    active_sorcerer_draconic_dragon_resistances_natural_armor_bonus(input).unwrap_or(0)
}

/// Grounds the Draconic Bloodline's 3rd-level "Dragon Resistances" power's two flat
/// numeric magnitudes (task #61, 2026-07-28): a natural armor bonus to Armor Class and
/// an energy resistance value, verified against the PF1 Core Rulebook corpus source
/// (cr_abilities_class.lst, KEY:Draconic Bloodline ~ Dragon Resistances, gated
/// PREVARGTEQ:Sorcerer_Draconic_BloodlineProgressionLVL,3). See
/// `sorcerer_draconic_dragon_resistances_natural_armor_bonus`'s own doc comment for why
/// the natural armor total is the SUM of two separate corpus `BONUS:VAR` lines.
///
/// Both magnitudes are grounded as type-agnostic facts: the numbers are identical
/// regardless of which energy type (acid/cold/electricity/fire) the Draconic
/// bloodline's own dragon-type selection ultimately names, so this function
/// deliberately never picks a canonical energy type -- that label is a separate,
/// deferred corpus sub-choice (the same dragon-type selection that also gates the
/// Bloodline Arcana / Claws / Breath Weapon / Power of Wyrms energy-type text), named
/// explicitly by a non-claim-blocking diagnostic rather than silently implied.
///
/// Mirrors the Barbarian Trap Sense / Damage Reduction level-gate-absence idiom: below
/// the level-3 gate (a Draconic-bloodline Sorcerer at level 1 or 2 -- the bloodline
/// itself is chosen at level 1, well before Dragon Resistances is granted), both
/// magnitudes are grounded as a correct level-gate absence (value 0); at or above
/// level 3, both are grounded as bounded flat-magnitude records. The natural armor
/// bonus record additionally documents its own integration into
/// `compute_combat_baseline`'s `defense.baseline_armor_class` total via
/// `apply_sorcerer_draconic_dragon_resistances_ac_bonus_to_combat_baseline` (real
/// integration, not standalone -- mirrors Brawler's own AC Bonus, which carries both a
/// standalone flat-magnitude explanation record AND a live wire into the shared Armor
/// Class total); the energy resistance bonus stays standalone only, since no
/// energy-damage-resistance-resolution engine exists anywhere in this codebase to
/// receive it (mirrors Alchemist's/Investigator's own standalone Poison Resistance
/// bonus, never wired into any total).
///
/// Only recognized for the canonical deterministic Draconic bloodline selection
/// (`choice:sorcerer_bloodline -> bloodline:draconic`), mirroring exactly how the
/// Arcane bloodline's own class-skill grant is only recognized when the Arcane
/// bloodline itself was the recognized selection: a character whose bloodline choice
/// this seam does not recognize as Draconic (a different bloodline, or none at all)
/// never gains a fabricated Draconic-specific grant. Deliberately race-agnostic (no
/// `HUMAN_RACE_ID` gate) -- see `active_sorcerer_draconic_dragon_resistances_natural_armor_bonus`'s
/// own doc comment for why.
pub(super) fn ground_sorcerer_draconic_bloodline_dragon_resistances(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(level) = supported_sorcerer_level(input) else {
        return;
    };
    if choice_selection(input, SORCERER_BLOODLINE_CHOICE_ID) != Some(DRACONIC_BLOODLINE_SELECTION_ID)
    {
        return;
    }

    if level < SORCERER_DRACONIC_DRAGON_RESISTANCES_LEVEL {
        explanations.push(ComputationExplanation {
            id: SORCERER_DRACONIC_DRAGON_RESISTANCES_NATURAL_ARMOR_EXPLANATION_ID.to_owned(),
            value: 0,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   PREVARGTEQ:Sorcerer_Draconic_BloodlineProgressionLVL,3
                "Sorcerer Draconic Bloodline Dragon Resistances natural armor bonus at sorcerer \
                 level {level}: correctly absent by PF1 Core Rulebook level gate (Dragon Resistances \
                 is a 3rd-level Draconic bloodline power, corpus); the at-grant magnitude is named \
                 but not computed."
            ),
        });
        explanations.push(ComputationExplanation {
            id: SORCERER_DRACONIC_DRAGON_RESISTANCES_RESISTANCE_EXPLANATION_ID.to_owned(),
            value: 0,
            detail: format!(
                "Sorcerer Draconic Bloodline Dragon Resistances energy resistance bonus at \
                 sorcerer level {level}: correctly absent by PF1 Core Rulebook level gate \
                 (3rd-level Draconic bloodline power); the at-grant magnitude is named but not \
                 computed."
            ),
        });
        diagnostics.push(ComputationDiagnostic {
            id: SORCERER_DRACONIC_DRAGON_RESISTANCES_ENERGY_TYPE_UNRESOLVED_DIAGNOSTIC_ID
                .to_owned(),
            message: format!(
                "Sorcerer Draconic Bloodline Dragon Resistances at sorcerer level {level}: both \
                 magnitudes are correctly grounded as absent below the 3rd-level gate above; \
                 WHICH energy type (acid, cold, electricity, or fire) the eventual grant would \
                 apply against is a separate corpus sub-choice not resolved on this bounded \
                 seam, so no specific energy type is claimed even once the gate is met."
            ),
            claim_blocking: false,
        });
        return;
    }

    let natural_armor_bonus = sorcerer_draconic_dragon_resistances_natural_armor_bonus(level);
    explanations.push(ComputationExplanation {
        id: SORCERER_DRACONIC_DRAGON_RESISTANCES_NATURAL_ARMOR_EXPLANATION_ID.to_owned(),
        value: natural_armor_bonus,
        detail: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   BONUS:VAR lines that accumulate additively into the same variable, not an override
            "Sorcerer Draconic Bloodline Dragon Resistances natural armor bonus at sorcerer level \
             {level} (PF1 Core Rulebook, 3rd-level Draconic bloodline power, corpus KEY:Draconic \
             Bloodline ~ Dragon Resistances): min(floor(({level}-3)/6)+1,3) PLUS an additional +1 at \
             15th level and higher (two separate corpus) = {natural_armor_bonus}. This grounds the \
             magnitude as a type-agnostic fact (the number is identical regardless of which dragon \
             type this power ultimately names); see \
             apply_sorcerer_draconic_dragon_resistances_ac_bonus_to_combat_baseline for its real \
             integration into the shared defense.baseline_armor_class total (only when the shared \
             GE-06 combat posture -- Longsword/Chain Shirt/Dodge/Weapon Focus/no shield --is also \
             met)."
        ),
    });

    let resistance_bonus = sorcerer_draconic_dragon_resistances_energy_resistance_bonus(level);
    explanations.push(ComputationExplanation {
        id: SORCERER_DRACONIC_DRAGON_RESISTANCES_RESISTANCE_EXPLANATION_ID.to_owned(),
        value: resistance_bonus,
        detail: format!(
            "Sorcerer Draconic Bloodline Dragon Resistances energy resistance bonus at sorcerer \
             level {level} (PF1 Core Rulebook, 3rd-level Draconic bloodline power, corpus \
             KEY:Draconic Bloodline ~ Dragon Resistances): min(floor(({level}-3)/6)+1,2)*5 = \
             {resistance_bonus}. This grounds the magnitude only, as a type-agnostic fact (the \
             number is identical regardless of which dragon type this power ultimately names): \
             it is not wired into any energy-damage-reduction-resolution engine, since none \
             exists anywhere in this codebase."
        ),
    });

    diagnostics.push(ComputationDiagnostic {
        id: SORCERER_DRACONIC_DRAGON_RESISTANCES_ENERGY_TYPE_UNRESOLVED_DIAGNOSTIC_ID.to_owned(),
        message: format!(
            "Sorcerer Draconic Bloodline Dragon Resistances at sorcerer level {level}: both \
             magnitudes (natural armor bonus and energy resistance bonus) are grounded above as \
             type-agnostic facts, but WHICH energy type (acid, cold, electricity, or fire) the \
             resistance and natural-armor grant apply against is a separate corpus sub-choice \
             (the Draconic Bloodline's own dragon-type selection, which also gates the Bloodline \
             Arcana / Claws / Breath Weapon / Power of Wyrms energy-type text) and is not \
             resolved on this bounded seam; no specific energy type is claimed."
        ),
        claim_blocking: false,
    });
}

/// The highest ACCESSIBLE sorcerer spell level (1st+) at the given sorcerer
/// level -- cantrips (0th level) have no access gate at all, always
/// available from level 1. Pure function, race-independent, mirrors
/// `ranger_spell_level_access`/`paladin_spell_level_access` -- extracted so
/// both the (Human-only) flat explanation block above and the real
/// known-spell validation below share one source of truth.
pub(super) fn sorcerer_spell_level_access(level: u8) -> i16 {
    if level >= SORCERER_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        9
    } else if level >= SORCERER_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        8
    } else if level >= SORCERER_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        7
    } else if level >= SORCERER_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        6
    } else if level >= SORCERER_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        5
    } else if level >= SORCERER_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        4
    } else if level >= SORCERER_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        3
    } else if level >= SORCERER_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        2
    } else {
        1
    }
}

/// The PF1 Core Rulebook Sorcerer Spells Known table's row, one entry per
/// spell level 0-9 (`None` for an inaccessible "—" column; index 0 is
/// cantrips). A literal table lookup, not a derived formula -- see
/// `explain_sorcerer_level1_spell_baseline`'s own doc comment for the
/// two-source verification history. Pure function, race-independent,
/// extracted for the same reason as `sorcerer_spell_level_access`. This is
/// the cap on distinct spells KNOWN (permanent), not a per-day consumable
/// resource like Ranger/Paladin's prepared-spell slot budget.
pub(super) fn sorcerer_spells_known_table(level: u8) -> [Option<i16>; 10] {
    match level {
        1 => [Some(4), Some(2), None, None, None, None, None, None, None, None],
        2 => [Some(5), Some(2), None, None, None, None, None, None, None, None],
        3 => [Some(5), Some(3), None, None, None, None, None, None, None, None],
        4 => [Some(6), Some(3), Some(1), None, None, None, None, None, None, None],
        5 => [Some(6), Some(4), Some(2), None, None, None, None, None, None, None],
        6 => [Some(7), Some(4), Some(2), Some(1), None, None, None, None, None, None],
        7 => [Some(7), Some(5), Some(3), Some(2), None, None, None, None, None, None],
        8 => [Some(8), Some(5), Some(3), Some(2), Some(1), None, None, None, None, None],
        9 => [Some(8), Some(5), Some(4), Some(3), Some(2), None, None, None, None, None],
        10 => [Some(9), Some(5), Some(4), Some(3), Some(2), Some(1), None, None, None, None],
        11 => [Some(9), Some(5), Some(5), Some(4), Some(3), Some(2), None, None, None, None],
        12 => [Some(9), Some(5), Some(5), Some(4), Some(3), Some(2), Some(1), None, None, None],
        13 => [Some(9), Some(5), Some(5), Some(4), Some(4), Some(3), Some(2), None, None, None],
        14 => {
            [Some(9), Some(5), Some(5), Some(4), Some(4), Some(3), Some(2), Some(1), None, None]
        }
        15 => {
            [Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(2), None, None]
        }
        16 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(2), Some(1), None,
        ],
        17 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None,
        ],
        18 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2),
            Some(1),
        ],
        19 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(3),
            Some(2),
        ],
        20 => [
            Some(9), Some(5), Some(5), Some(4), Some(4), Some(4), Some(3), Some(3), Some(3),
            Some(3),
        ],
        _ => [None, None, None, None, None, None, None, None, None, None],
    }
}

/// Return the list of unmet conditions for Sorcerer's real known-spell
/// posture. An empty list means the posture is fully valid: every
/// `AcquisitionMode::Known` selection with `source_class_id ==
/// "class:sorcerer"` names a real spell on
/// `sorcerer_spell_list::SORCERER_SPELL_LIST`, at a spell level within the
/// sorcerer's own access ceiling for their sorcerer level (1st+ only --
/// cantrips have no access gate), and no spell level's known count exceeds
/// that level's real cap from the Sorcerer Spells Known table. Zero known
/// spells is always valid, same reasoning as Ranger/Paladin's prepared
/// posture: real PF1 rules don't require every fixture to have picked
/// spells yet.
pub(super) fn unmet_sorcerer_known_spell_conditions(input: &CharacterInput, sorcerer_level: u8) -> Vec<String> {
    let mut unmet = Vec::new();

    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == SORCERER_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    let access_ceiling = sorcerer_spell_level_access(sorcerer_level);
    let known_table = sorcerer_spells_known_table(sorcerer_level);

    let mut known_per_level: [i16; 10] = [0; 10];
    for spell_id in &known {
        let Some(spell_level) = sorcerer_spell_list::sorcerer_spell_level(spell_id) else {
            unmet.push(format!(
                "known spell '{spell_id}' is not on the real PF1 sorcerer spell list"
            ));
            continue;
        };
        if spell_level > 0 && i16::from(spell_level) > access_ceiling {
            unmet.push(format!(
                "known spell '{spell_id}' targets spell level {spell_level}, not yet accessible \
                 at sorcerer level {sorcerer_level} (access ceiling {access_ceiling})"
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
                 slots available on the Sorcerer Spells Known table"
            ));
        }
    }

    unmet
}

/// Ground the real known-spell posture once
/// `unmet_sorcerer_known_spell_conditions` reports an empty unmet list: the
/// known-spell selection (count + list, mirroring
/// `class_spell.ranger.daily_preparation`'s shape, substituting "known" for
/// "prepared" since spontaneous casters have no daily preparation step at
/// all).
pub(super) fn ground_sorcerer_known_spells(
    input: &CharacterInput,
    sorcerer_level: u8,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let known: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == SORCERER_CLASS_ID && s.acquisition_mode == AcquisitionMode::Known
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.sorcerer.known_spells".to_owned(),
        value: known.len() as i16,
        detail: format!(
            "Sorcerer level {sorcerer_level} known-spell selection ({} spells, \
             AcquisitionMode::Known): {}. Each known spell is verified against the real PF1 \
             sorcerer spell list (`sorcerer_spell_list::SORCERER_SPELL_LIST`, all ingested \
             books), the \
             sorcerer's own spell-level access ceiling, and the Sorcerer Spells Known table's \
             own per-level cap. Real PF1 Sorcerer rules have no daily preparation step at all \
             (unlike Ranger/Paladin) -- a sorcerer's known spells are permanent once learned, \
             cast spontaneously using the already-grounded per-day slot totals. This grounds \
             the known-spell selection for real; it computes no spell save DC resolution \
             against a target and no casting execution",
            known.len(),
            known.join(", ")
        ),
    });
}

/// The bounded Wizard milestone level this decomposition surface grounds, if any.
/// Returns the single Wizard level when the chosen input is exactly a single-class
/// Wizard at one of the supported milestone levels (1 through 11). Returns `None` for
/// no Wizard, a non-Wizard class, a multiclass mix, or any level-12+ Wizard this slice
/// deliberately does not recognize — each of which stays claim-blocked exactly as
/// before. Mirrors the Fighter `supported_fighter_level` / Paladin
/// `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` / Bard `supported_bard_level` / Druid
/// `supported_druid_level` / Sorcerer `supported_sorcerer_level` level-range gate
/// idiom.
pub(super) fn supported_wizard_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == WIZARD_CLASS_ID
                && (1..=MAX_SUPPORTED_WIZARD_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Return `true` when the input carries exactly the canonical deterministic school
/// specialization selections: Evocation chosen as the specialty school, with
/// Necromancy and Transmutation as the two opposed schools. Anything else — the
/// choice slots absent (e.g. a universalist-shaped request) or any non-canonical
/// selection — returns `false`, so no specialization grounding is fabricated for a
/// choice that was never made or that this bounded slice does not know.
pub(super) fn wizard_has_canonical_specialization_selections(input: &CharacterInput) -> bool {
    if choice_selection(input, WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID)
        != Some(EVOCATION_SCHOOL_SELECTION)
    {
        return false;
    }
    let opposed: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == WIZARD_OPPOSED_SCHOOLS_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();
    opposed.len() == 2
        && opposed.contains(&NECROMANCY_SCHOOL_SELECTION)
        && opposed.contains(&TRANSMUTATION_SCHOOL_SELECTION)
}

/// Task #66: return `true` when the input carries the canonical deterministic
/// Abjuration school specialization selections (Abjuration chosen as the
/// specialty school, with Necromancy and Transmutation as the two opposed
/// schools). Mirrors `wizard_has_canonical_specialization_selections`
/// exactly, one school swapped for another; anything else — the choice slots
/// absent or any non-canonical selection — returns `false`, so no Abjuration
/// school-power grounding is fabricated for a choice that was never made.
pub(super) fn wizard_has_canonical_abjuration_selection(input: &CharacterInput) -> bool {
    if choice_selection(input, WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID)
        != Some(ABJURATION_SCHOOL_SELECTION)
    {
        return false;
    }
    let opposed: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == WIZARD_OPPOSED_SCHOOLS_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();
    opposed.len() == 2
        && opposed.contains(&NECROMANCY_SCHOOL_SELECTION)
        && opposed.contains(&TRANSMUTATION_SCHOOL_SELECTION)
}

/// `AT-34-E3-001` (mechanism 2 continuation, cycle 6): a third canonical
/// deterministic school selection, alongside Evocation and Abjuration above.
/// Transmutation specialized, with Necromancy and Evocation as the two
/// opposed schools -- both already-existing selection constants, reused
/// rather than duplicated (PF1's only opposition restriction is "not your
/// own specialty school and not Divination", so any two non-Transmutation,
/// non-Divination schools are legal; Necromancy/Evocation is picked simply
/// because both constants already exist).
pub(super) fn wizard_has_canonical_transmutation_selection(input: &CharacterInput) -> bool {
    if choice_selection(input, WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID)
        != Some(TRANSMUTATION_SCHOOL_SELECTION)
    {
        return false;
    }
    let opposed: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == WIZARD_OPPOSED_SCHOOLS_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();
    opposed.len() == 2
        && opposed.contains(&NECROMANCY_SCHOOL_SELECTION)
        && opposed.contains(&EVOCATION_SCHOOL_SELECTION)
}

/// `AT-34-E3-001` (mechanism 2 continuation, cycle 7): a fourth canonical
/// deterministic school selection, alongside Evocation, Abjuration, and
/// Transmutation above. Conjuration specialized, with Necromancy and
/// Abjuration as the two opposed schools -- both already-existing
/// selection constants, reused rather than duplicated (PF1's only
/// opposition restriction is "not your own specialty school and not
/// Divination", so any two non-Conjuration, non-Divination schools are
/// legal; Necromancy/Abjuration is picked simply because both constants
/// already exist).
pub(super) fn wizard_has_canonical_conjuration_selection(input: &CharacterInput) -> bool {
    if choice_selection(input, WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID)
        != Some(CONJURATION_SCHOOL_SELECTION)
    {
        return false;
    }
    let opposed: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == WIZARD_OPPOSED_SCHOOLS_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();
    opposed.len() == 2
        && opposed.contains(&NECROMANCY_SCHOOL_SELECTION)
        && opposed.contains(&ABJURATION_SCHOOL_SELECTION)
}

/// SD-34 wave 44 (`decisions.md §22`, Piece 2 item 1): a fifth canonical
/// deterministic school selection, alongside Evocation, Abjuration,
/// Transmutation, and Conjuration above. Necromancy specialized, with
/// Abjuration and Conjuration as the two opposed schools -- both
/// already-existing selection constants, reused rather than duplicated (PF1's
/// only opposition restriction is "not your own specialty school and not
/// Divination", so any two non-Necromancy, non-Divination schools are legal;
/// Abjuration/Conjuration is picked simply because both constants already
/// exist and neither is Necromancy itself).
pub(super) fn wizard_has_canonical_necromancy_selection(input: &CharacterInput) -> bool {
    if choice_selection(input, WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID)
        != Some(NECROMANCY_SCHOOL_SELECTION)
    {
        return false;
    }
    let opposed: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == WIZARD_OPPOSED_SCHOOLS_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();
    opposed.len() == 2
        && opposed.contains(&ABJURATION_SCHOOL_SELECTION)
        && opposed.contains(&CONJURATION_SCHOOL_SELECTION)
}

/// `AT-34-E3-001` (mechanism 2 continuation, cycle 8): the universalist
/// (no-specialization) selection. Unlike every specialist gate above, PF1's
/// own rule for a wizard who does not specialize is "need not select an
/// opposition school" (`cr_abilities_class.lst`'s `Universal School` record
/// carries only the shared, unconditional `BONUS:VAR|OppositionalSchool|-2`
/// token, no per-school opposition choice at all) -- so the canonical
/// universalist fixture is required to carry ZERO
/// `WIZARD_OPPOSED_SCHOOLS_CHOICE_ID` selections, not exactly two. Anything
/// else -- the specialization slot absent, any non-universal selection, or
/// any opposed-school selection present at all -- returns `false`, so no
/// Universal School power grounding is fabricated for a choice shape that
/// contradicts the rule this gate exists to recognize.
pub(super) fn wizard_has_canonical_universal_selection(input: &CharacterInput) -> bool {
    if choice_selection(input, WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID)
        != Some(UNIVERSAL_SCHOOL_SELECTION)
    {
        return false;
    }
    !input
        .chosen
        .selected_choices
        .iter()
        .any(|c| c.choice_set_id == WIZARD_OPPOSED_SCHOOLS_CHOICE_ID)
}

