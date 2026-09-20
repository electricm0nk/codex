#[allow(unused_imports)]
pub(crate) use super::*;

/// v0.6 alpha swarm, risks item 8 (Shaman full-build closure, 12th
/// ACG/APG class-specific closure): APG Shaman, verified directly
/// against `acg_classes.lst`'s own `SPELLSTAT:WIS MEMORIZE:YES` tokens
/// (a prepared, 9th-level divine caster, no `SPELLLIST:` reuse token --
/// a fresh own-list caster, deferred entirely this slice). This
/// closure's own MVP was corrected mid-scoping: the original comparative
/// pass proposed Life Spirit's own Healer's Touch revelation, but direct
/// verification of the real `ABILITY:...AUTOMATIC` grant line found it
/// gated behind `PREVARGTEQ:ShamanSpiritGreater,1` (itself only set via
/// `PRECLASS:1,Shaman=8`) -- genuinely a level 8+ feature, not
/// immediately available the way Oracle's Healing Hands is. The lead
/// independently confirmed this and greenlit the swap: Life Spirit's
/// OTHER immediately-granted ability, Channel (a real Channel Positive
/// Energy variant, `SERVESAS:ABILITY=Special Ability|Channel Positive
/// Energy`), carries no such gate and grounds via the exact "flat uses-
/// per-day + dice + DC" shape Cleric's own Channel Energy and
/// Warpriest's Blessings already use. See
/// `docs/release/v0.6/shaman-summoner-witch-comparative-scoping.md` for
/// the full corpus verification and scope record.
pub(super) const SHAMAN_CLASS_ID: &str = "class:shaman";

/// The choice set for which Spirit a Shaman selects at 1st level
/// (mirrors `ORACLE_MYSTERY_CHOICE_ID`'s/`WITCH_HEX_CHOICE_ID`'s own
/// single-selection shape).
pub(super) const SHAMAN_SPIRIT_CHOICE_ID: &str = "choice:shaman_spirit";

// Grounded SD13-E4 Human Druid level-1 prepared divine spell-bearing baseline
// identity. Druid is a prepared divine caster whose bounded burden splits across
// a nature bond / wild empathy class-feature family (nature bond choice between
// an animal companion and a domain, nature sense, wild empathy) and a prepared
// divine spell posture family (spells prepared from the full Druid list,
// spontaneous summon nature's ally conversion, spell slots per day, bonus spells
// from a high Wisdom, spell save DCs). Wild Empathy (SD13-E4), Nature Sense, and
// the deterministic nature-bond choice recognition (SD13-E5) are grounded; the
// chosen bond's execution and the whole spell posture stay claim-blocked.
pub(super) const DRUID_CLASS_ID: &str = "class:druid";

/// v0.6 alpha swarm, risks item 8, seventh slice (2026-07-25): the Druid
/// spell-level access thresholds. Druid's real spells-per-day table is
/// byte-for-byte identical to Cleric's own base table (verified
/// independently against two primary sources, d20pfsrd.com and the
/// Archives of Nethys aonprd.com mirror, both agreeing with each other and
/// with Cleric's own already-grounded table), so these thresholds match
/// Cleric's CLERIC_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL through
/// CLERIC_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL constants exactly, not
/// independently re-derived: 2nd-level spells begin at level 3, 3rd at 5,
/// 4th at 7, 5th at 9, 6th at 11, 7th at 13, 8th at 15, 9th at 17.
pub(super) const DRUID_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 3;

pub(super) const DRUID_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 5;

pub(super) const DRUID_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 7;

pub(super) const DRUID_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 9;

pub(super) const DRUID_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 11;

pub(super) const DRUID_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 13;

pub(super) const DRUID_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 15;

pub(super) const DRUID_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 17;

/// SD13-E5 Druid level-range gate, mirroring the Fighter `supported_fighter_level` /
/// Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` / Bard `supported_bard_level` idiom. Verified against the
/// PF1 Core Rulebook Druid class table (d20pfsrd and legacy.aonprd.com) before
/// widening: level 2 base attack bonus is +1, base saves are +3/+0/+3
/// (Fortitude/Reflex/Will), so every level-1 base-attack/base-save/Wild-Empathy
/// formula this seam already grounds extends to level 2 without re-derivation; Nature
/// Sense and the nature-bond choice recognition are level-independent and unaffected;
/// the class table's level-2 "Special" column reads "Woodland stride" (a new,
/// flat/identity-shaped class feature grounded separately below). A further SD13-E5
/// slice widens the gate to level 3 (verified independently against d20pfsrd and
/// legacy.aonprd.com): level 3 base attack bonus is +2, base saves are +3/+1/+3
/// (Fortitude/Reflex/Will), extended via the same formulas; Woodland Stride stays
/// granted, not re-derived; the class table's level-3 "Special" column reads
/// "Trackless step" (a new, flat/identity-shaped class feature grounded separately
/// below); Druid has no currently-grounded spell-slot-count pillar (unlike Wizard's
/// specialist bonus slot or Cleric's domain slot), so there is no analogous level-3
/// doubling to widen. A further SD13-E5 slice widens the gate to level 4 (verified
/// independently against d20pfsrd and legacy.aonprd.com): level 4 base attack bonus
/// is +3, base saves are +4/+1/+4 (Fortitude/Reflex/Will), extended via the same
/// formulas; Woodland Stride and Trackless Step both stay granted, not re-derived.
/// The class table's level-4 "Special" column reads "Resist nature's lure, wild
/// shape (1/day)" — TWO distinct entries, both checked independently rather than
/// assumed. Resist Nature's Lure is flat/identity-shaped (a standalone +4
/// saving-throw bonus against the spell-like and supernatural abilities of fey,
/// and against spells/effects that target plants) and is grounded separately below,
/// mirroring the Woodland Stride/Trackless Step idiom. Wild Shape is NOT flat — it
/// is a full shapeshifting subsystem (new form, new stat block, duration tracking)
/// with no execution engine anywhere in this codebase — so it is deliberately left
/// named-but-unproven, exactly like the animal-companion execution burden. A still
/// further SD13-E5 slice widens the gate to level 5 (verified independently against
/// d20pfsrd and legacy.aonprd.com): level 5 base attack bonus is +3, base saves are
/// +4/+1/+4 (Fortitude/Reflex/Will) — all three numerically unchanged from level 4
/// (integer-division coincidences of `level * 3 / 4`, `level / 2 + 2`, and
/// `level / 3`, not a sign any formula stopped scaling), extended via the same
/// formulas, not re-derived; Wild Empathy grounds correctly to 6 (5 + Charisma
/// modifier 1) via the same level-generic formula; Nature Sense stays the flat +2
/// bonus; Woodland Stride, Trackless Step, and Resist Nature's Lure all stay
/// granted, not re-derived. The class table's level-5 "Special" column is genuinely
/// blank (verified independently against both primary sources rather than assumed),
/// so this slice grounds no new pillar — only the existing pillars are widened. A
/// still further SD13-E5 slice widens the gate to level 6 (verified independently
/// against d20pfsrd and legacy.aonprd.com): level 6 base attack bonus is +4, base
/// saves are +5/+2/+5 (Fortitude/Reflex/Will), all three genuinely new values, up
/// from +3/+1/+4 at level 5, extended via the same formulas, not re-derived; Wild
/// Empathy grounds correctly to 7 (6 + Charisma modifier 1) via the same
/// level-generic formula; Nature Sense stays the flat +2 bonus; Woodland Stride,
/// Trackless Step, and Resist Nature's Lure all stay granted, not re-derived. The
/// class table's level-6 "Special" column reads "Wild shape (2/day)" — checked per
/// the operator brief's explicit instruction to verify whether Druid gains an
/// actual new class feature at 6th level, and confirmed NOT a genuinely separable
/// flat/identity-shaped element: the rule text bundles the "2/day" frequency
/// increase together with a form-list expansion (a druid can now wild shape into a
/// Large or Tiny animal or a Small elemental) and a functioning-level upgrade (the
/// animal form now functions as beast shape II, the elemental form as elemental
/// body I) — none of which exist in this codebase's engine-free record set, and
/// none of which are separable from the "2/day" numeral without misrepresenting the
/// bundled feature as fully flat. Wild Shape (including its level-6 frequency
/// increase and form-list expansion) is therefore deliberately left entirely
/// named-but-unproven, exactly as at level 4/5 — no explanation or diagnostic
/// record is fabricated for it this slice either. A still further SD13-E5 slice
/// widens the gate to level 8 (verified independently against d20pfsrd and
/// legacy.aonprd.com): level 8 base attack bonus is +6 (genuinely risen from +5;
/// the class table's own "+6/+1" iterative-attack notation is not modeled
/// anywhere in this codebase, only the flat base value), base saves are +6/+2/+6
/// (Fortitude/Reflex/Will — both good saves genuinely rise from +5, while poor
/// Reflex stays +2, an integer-division coincidence), extended via the same
/// formulas, not re-derived; Wild Empathy genuinely rises to 9 (8 + Charisma
/// modifier 1) via the same level-generic formula; Nature Sense stays the flat
/// +2; Woodland Stride, Trackless Step, and Resist Nature's Lure all stay
/// granted, not re-derived. The class table's level-8 "Special" column reads
/// "Wild shape (3/day)" — checked rather than assumed away, and confirmed to be
/// the same non-separable bundled shape as at level 6: the frequency increase
/// arrives together with a form-list expansion (Huge/Diminutive animal, Medium
/// elemental, Small/Medium plant) and functioning-level upgrades (beast shape
/// III / elemental body II / plant shape I), so Wild Shape (including its
/// level-8 frequency increase) stays entirely named-but-unproven, exactly as at
/// level 4/6 — no explanation or diagnostic record is fabricated for it this
/// slice either.
// A still further SD13-E5 slice widens the gate to level 9 (verified
// independently against d20pfsrd and legacy.aonprd.com): level 9 base attack
// stays +6 (9 * 3 / 4) and both good saves stay +6 (9 / 2 + 2),
// integer-division coincidences, while poor Reflex genuinely rises to +3
// (9 / 3); Wild Empathy genuinely rises to 10 (9 + Charisma modifier 1) via
// the same level-generic formula; Nature Sense, Woodland Stride, Trackless
// Step, and Resist Nature's Lure all stay granted, not re-derived; Wild
// Shape's uses stay 3/day (the next rise lands at 10th, checked rather than
// assumed) and it stays entirely named-but-unproven; the level-9 "Special"
// column reads "Venom immunity" — a genuinely flat, no-choice, no-magnitude
// grant (immunity to all poisons), grounded as a +0 identity/recognition
// record only (DRUID_VENOM_IMMUNITY_LEVEL), mirroring Monk's Purity of Body
// precedent exactly; no poison/condition engine exists here, so no immunity
// effect is fabricated. A further SD13-E5 slice widens the gate to level 10 —
// the tranche ceiling (verified independently against d20pfsrd and
// legacy.aonprd.com): level 10 base attack genuinely rises to +7
// (10 * 3 / 4) and both good saves genuinely rise to +7 (10 / 2 + 2), while
// poor Reflex stays +3 (10 / 3, a coincidence); Wild Empathy genuinely rises
// to 11 (10 + Charisma modifier 1); Nature Sense, the granted features, and
// Venom Immunity all carry over unchanged; the level-10 "Special" column
// reads "Wild shape (4/day)" — checked rather than assumed away, and
// confirmed the same non-separable frequency-plus-form-expansion bundle as
// at levels 6/8 (a Large elemental or Large plant, functioning as elemental
// body III / plant shape II), so Wild Shape stays entirely
// named-but-unproven, exactly as at level 4/6/8 — no explanation or
// diagnostic record is fabricated for it this slice either.
// A still further SD18 slice widens the gate to level 11 (verified
// independently against d20pfsrd and legacy.aonprd.com): level 11 base
// attack bonus genuinely rises to +8 (11 * 3 / 4; the table's own "+8/+3"
// iterative notation is not modeled anywhere in this codebase, only the
// flat base value); both good saves stay +7 (11 / 2 + 2, an
// integer-division coincidence with level 10) and poor Reflex stays +3
// (11 / 3, also a coincidence); Wild Empathy genuinely rises to 12
// (11 + Charisma modifier); Nature Sense, Woodland Stride, Trackless Step,
// Resist Nature's Lure, Venom Immunity, and the nature-bond choice
// recognition all carry over unchanged. UNLIKE every prior widened level
// (2, 3, 4, 6, 8, 9, 10), the PF1 Core Rulebook Druid class table's
// level-11 "Special" column is genuinely blank (checked independently
// against both primary sources rather than assumed away — the next Wild
// Shape frequency rise, "Wild shape (5/day)", does not land until 12th
// level), so this slice grounds no new pillar at level 11 either — only
// the existing arithmetic pillars are widened. A further SD18 slice
// (cycle-2026-07-15T0500, mirroring cycle-2026-07-14T1814's Barbarian
// level-12 widening, cycle-2026-07-14T2359's Bard level-12 widening, and
// cycle-2026-07-15T0200's Cleric level-12 widening) widens the gate again
// to level 12 (verified independently against d20pfsrd and Archives of
// Nethys aonprd.com's legacy mirror): level 12 base attack bonus genuinely
// rises to +9 (12 * 3 / 4) and all three base saves genuinely rise (both
// good saves to +8, 12 / 2 + 2; poor Reflex to +4, 12 / 3); Wild Empathy
// genuinely rises to 13 (12 + Charisma modifier). UNLIKE level 11, the
// class table's level-12 "Special" column is NOT blank — it reads "Wild
// shape (5/day)" — but per the level-4/6/8/10 precedent that frequency
// rise is bundled with a non-separable form-list expansion (Huge elemental
// or Huge plant creature) and functioning-level upgrade (elemental body IV
// / plant shape III), so Wild Shape stays entirely named-but-unproven and
// this slice grounds no new pillar at level 12 either — only the existing
// arithmetic pillars are widened. A still further SD18 slice
// (cycle-2026-07-15T1600, mirroring cycle-2026-07-15T1500's Cleric
// level-13 widening, the loop's fifth §3.2 level-13 landing after Rogue,
// Barbarian, Fighter, and Ranger) widens the gate again to level 13. All
// three primary sources (d20pfsrd, Archives of Nethys aonprd.com, and
// legacy.aonprd.com) were read directly and agree: level 13 base attack
// bonus STAYS +9 (13 * 3 / 4 = 9) and all three base saves STAY +8/+4/+8
// (Fortitude/Reflex/Will: 13/2+2=8, 13/3=4) — four integer-division
// coincidences with level 12, checked rather than assumed; Wild Empathy
// genuinely rises to 14 (13 + Charisma modifier). UNLIKE every prior
// widened level's Wild-Shape-shaped "Special" column entry (levels
// 4/6/8/10/12), the level-13 "Special" column reads "A thousand faces" — a
// DIFFERENT class feature, not a Wild Shape frequency increase. Checked
// directly rather than assumed: in PF1 (unlike the D&D 3.5 version of this
// ability, which referenced the stronger `alter self` spell), A Thousand
// Faces grants the druid the ability to change her own apparent appearance
// at will, as if using `disguise self`, but only while in her normal
// (unshifted) form. This is a genuinely flat/identity-shaped, no-choice,
// no-magnitude, no-duration-tracking, at-will grant — mirroring exactly
// how Venom Immunity was grounded at level 9 — so it is grounded here as a
// bounded +0 identity/recognition record: no illusion-effect execution
// engine and no Disguise-check-resolution engine exists anywhere in this
// codebase, so no actual appearance-change or Disguise-check outcome is
// fabricated. The spells-per-day table's 7th-level spell column also newly
// opens at level 13 (matching the Cleric precedent exactly, since Druid
// shares the identical "high" 9-level-caster progression shape), but Druid
// has no currently-grounded spell-slot-count pillar (unlike Cleric's
// domain slot), so there is no analogous pillar to widen. A further SD18
// slice (cycle-2026-07-15T2400, mirroring cycle-2026-07-15T2300's Cleric
// level-14 widening, the loop's sixth §3.2 level-14 landing after
// Barbarian, Fighter, Rogue, Ranger, and Bard) widens the gate again to
// level 14 (verified independently against both d20pfsrd and Archives of
// Nethys aonprd.com, which agree byte-for-byte): base attack bonus
// GENUINELY RISES to +10 (14 * 3 / 4) and both good saves GENUINELY RISE
// to +9 (14 / 2 + 2), while poor Reflex STAYS +4 (14 / 3, an
// integer-division coincidence with level 13); Wild Empathy genuinely
// rises to 15 (14 + Charisma modifier). The level-14 "Special" column
// reads "Wild shape (6/day)" — per the level-4/6/8/10/12 precedent this
// frequency increase is bundled with a non-separable functioning-level
// upgrade with no execution engine anywhere in this codebase, so Wild
// Shape stays entirely named-but-unproven and this slice grounds no new
// pillar at level 14 either — only the existing arithmetic pillars are
// widened. A still further SD18 slice (the loop's FIFTH §3.2 level-15
// landing after Barbarian, Rogue, Fighter, and Cleric) widens the gate
// again to level 15 (verified independently against all three primary
// sources: d20pfsrd, Archives of Nethys aonprd.com, and legacy.aonprd.com,
// which agree byte-for-byte): base attack bonus GENUINELY RISES to +11
// (15 * 3 / 4) and poor Reflex GENUINELY RISES to +5 (15 / 3), while both
// good saves STAY +9 (15 / 2 + 2, an integer-division coincidence with
// level 14); Wild Empathy genuinely rises to 16 (15 + Charisma modifier).
// UNLIKE every prior widened level's Wild-Shape-shaped "Special" column
// entry (levels 4/6/8/10/12/14), and unlike level 13's "A thousand faces",
// the level-15 "Special" column reads "Timeless body" ONLY — checked
// directly rather than assumed to also carry a Wild Shape frequency
// increase (the next one, "Wild shape (7/day)", does not land until 16th
// level). Timeless Body is a genuinely flat/identity-shaped, no-choice,
// no-magnitude, no-duration-tracking grant (a druid no longer takes
// ability score penalties for old age and cannot be magically aged),
// mirroring exactly how Venom Immunity (level 9) and A Thousand Faces
// (level 13) were grounded: a bounded +0 identity/recognition record, with
// no aging-penalty-resolution engine fabricated.
//
// A final v0.6 slice (2026-07-29) widens the gate the rest of the way to 20,
// closing the last five blocked Druid levels so the class computes at every
// level 1-20. Unlike every widening above it this one opens a five-level band
// at once, so each row was transcribed individually from the PCGen PF1 Core
// Rulebook data set rather than assumed to continue the pattern, reading whole
// records including the `.MOD` blocks a token-filtered grep hides:
//
//   - `cr_classes.lst` line 93 (`CLASS:Druid`) carries `MAXLEVEL:20` and the
//     three progression formulas, which are byte-for-byte identical to
//     `CLASS:Cleric` (line 55) -- already grounded to level 20 here. Base
//     attack bonus (`classlevel*3/4`) rises 12/12/13/14/15 across 16-20; both
//     good saves (`classlevel/2+2`) rise 10/10/11/11/12; poor Reflex
//     (`classlevel/3`) rises 5/5/6/6/6. Reflex STAYING at 5 from level 15 to
//     16-17, and at 6 from 18 through 20, are integer-division coincidences
//     that were checked rather than assumed to keep climbing.
//   - `cr_classes.lst` lines 131-135 are the real Druid `CAST:` rows for
//     levels 16-20 and are byte-for-byte identical to Cleric's own lines
//     85-89 -- which is what already justifies
//     `druid_base_spells_per_day_table` delegating to
//     `cleric_base_spells_per_day_table`, whose 16-20 rows therefore need no
//     change. Level 16 has NINE columns; the tenth (9th-level druid spells)
//     first appears at level 17, matching the already-present
//     `DRUID_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL = 17` -- a threshold
//     this widening makes reachable for the first time.
//   - `cr_abilities_class.lst` lines 223-234 are the COMPLETE
//     `CATEGORY=Class|Druid.MOD` class-feature block, and its highest gate is
//     `PREVARGTEQ:Druid_CFP_Level,15` (Timeless Body, line 234). There is NO
//     new named Druid class feature at 16, 17, 18, 19, or 20 -- read off the
//     whole block rather than inferred from the level-15 stopping point.
//     Timeless Body is genuinely Druid's last named CRB class feature, so no
//     new pillar is grounded here and nothing text-only is left unshown to
//     the player by this slice.
//   - The only 16/18/20 "Special" column entries are Wild Shape frequency
//     increments (7/day, 8/day, at will), from `cr_abilities_class.lst` line
//     853's `BONUS:VAR|WildShapeTimes|(DruidLVL>=4)+...+(DruidLVL>=16)+
//     (DruidLVL>=18)+(DruidLVL>=20)` and line 796's
//     `DESC:You can change shape at will ...|PREVARGTEQ:WildShapeProgression,9`.
//     Levels 17 and 19 are genuinely blank. Worth recording for whoever
//     eventually grounds Wild Shape: these three increments are PURELY
//     frequency, because the form-tier grants
//     (`CATEGORY=Special Ability|Wild Shape.MOD`, lines 857-861) stop at
//     `PREVARGTEQ:DruidWildShape,12` -- the bundling argument that kept
//     levels 4/6/8/10/12 unproven does not apply above 12. It stays
//     named-but-unproven regardless, on the unchanged and still-true ground
//     that this codebase has no shapeshifting execution engine at all; a
//     uses/day counter with nothing to spend it on would be a fabricated
//     number, which is the one outcome worse than a reported gap.
pub(super) const MAX_SUPPORTED_DRUID_LEVEL: u8 = 20;

/// PF1 Core Rulebook level gate at which Druid gains Venom Immunity (9th
/// level, verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Venom immunity" as the Druid 9th-level
/// "Special" column entry).
pub(super) const DRUID_VENOM_IMMUNITY_LEVEL: u8 = 9;

/// PF1 Core Rulebook level gate at which Druid gains A Thousand Faces (13th
/// level, verified independently against three primary sources: d20pfsrd,
/// Archives of Nethys aonprd.com, and legacy.aonprd.com all list "A
/// thousand faces" as the Druid 13th-level "Special" column entry). In PF1
/// (unlike the D&D 3.5 version of this ability, which referenced the
/// stronger `alter self` spell), this grants the druid the ability to
/// change her own apparent appearance at will, as if using `disguise
/// self`, but only while in her normal (unshifted) form.
pub(super) const DRUID_A_THOUSAND_FACES_LEVEL: u8 = 13;

/// PF1 Core Rulebook level gate at which Druid gains Timeless Body (15th
/// level, verified independently against three primary sources: d20pfsrd,
/// Archives of Nethys aonprd.com, and legacy.aonprd.com all list "Timeless
/// body" — and ONLY that entry, with no accompanying Wild Shape frequency
/// increase — as the Druid 15th-level "Special" column entry). A druid no
/// longer takes ability score penalties for old age and cannot be
/// magically aged (existing penalties remain in place; bonuses still
/// accrue).
pub(super) const DRUID_TIMELESS_BODY_LEVEL: u8 = 15;

/// PF1 Core Rulebook level gate at which Druid gains Resist Nature's Lure (4th
/// level, verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Resist nature's lure" as part of the Druid
/// 4th-level special feature entry, alongside "Wild shape (1/day)").
pub(super) const DRUID_RESIST_NATURES_LURE_LEVEL: u8 = 4;

/// PF1 Core Rulebook Resist Nature's Lure flat magnitude: "a druid gains a +4
/// bonus on saving throws against the spell-like and supernatural abilities of
/// fey. This bonus also applies to spells and effects that utilize or target
/// plants, such as blight, entangle, spike growth, and warp wood." Flat and
/// level-independent once granted (it does not scale further with druid level).
pub(super) const DRUID_RESIST_NATURES_LURE_BONUS: i16 = 4;

/// PF1 Core Rulebook level gate at which Druid gains Woodland Stride (2nd level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Woodland stride" as the Druid 2nd-level special
/// feature entry).
pub(super) const DRUID_WOODLAND_STRIDE_LEVEL: u8 = 2;

/// PF1 Core Rulebook level gate at which Druid gains Trackless Step (3rd level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Trackless step" as the Druid 3rd-level special
/// feature entry).
pub(super) const DRUID_TRACKLESS_STEP_LEVEL: u8 = 3;

// PF1 Core Rulebook Nature Sense: a druid gains a +2 bonus on Knowledge (nature)
// and Survival checks. Flat and level-independent.
pub(super) const DRUID_NATURE_SENSE_BONUS: i16 = 2;

// The deterministic SD13 fixture's nature-bond selection seam: the choice set and
// the one selection this bounded slice recognizes (an animal companion; a domain
// bond is not part of the deterministic fixture and stays unrecognized).
pub(super) const DRUID_NATURE_BOND_CHOICE_ID: &str = "choice:druid_nature_bond";

pub(super) const DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID: &str = "bond:animal_companion";

/// v0.6 alpha swarm, risks item 8 (Shaman full-build closure): whether
/// `input` is a single-class Shaman at a level within
/// `acg::class_chassis_resolve`'s declared ceiling for Shaman -- mirrors
/// the other nine ACG/APG exact-match gates exactly.
pub(super) fn is_supported_shaman_single_class(input: &CharacterInput) -> bool {
    let [class_level] = input.chosen.class_levels.as_slice() else {
        return false;
    };
    if AcgClassId::from_class_id_str(&class_level.class_id) != Some(AcgClassId::Shaman) {
        return false;
    }
    acg::class_chassis_resolve(AcgClassId::Shaman, class_level.level, RuleSetId::Acg).is_some()
}

/// The ACG Shaman class table's BASE spells-per-day row, one entry per
/// spell level 0-9 (`None` for an inaccessible "—" column; index 0 is
/// orisons). Transcribed directly from `acg_classes.lst`'s own 20 `CAST:`
/// rows -- unlike Cleric's table (which needed an external citation
/// because the corpus was not consulted at the time), this one IS
/// corpus-derived, `CAST:3,1` at level 1 through
/// `CAST:4,4,4,4,4,4,4,4,4,4` at 20.
///
/// **Level 11 is a deliberate, flagged corpus divergence.** Shaman's row
/// is `4,4,4,3,3,2,1`; every other 9-level prepared caster in the entire
/// PCGen tree -- Cleric, Druid, Wizard, and Witch -- carries
/// `4,4,4,4,3,2,1` there, and Shaman's own rows at levels 10 and 12 are
/// byte-identical to all four. Shaman is the ONLY class anywhere in the
/// tree with this row, i.e. it gains its fourth 3rd-level slot one level
/// later than every peer. That is either a real ACG rules quirk or a
/// PCGen data typo; this table follows the corpus per the standing
/// corpus-first rule, and `shaman_level_11_follows_the_corpus_not_the_peer_row`
/// pins it so the choice is visible rather than silently baked in. It is
/// a one-cell change if the ruling goes the other way.
pub(super) fn shaman_base_spells_per_day_table(level: u8) -> [Option<i16>; 10] {
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
        11 => [Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), Some(1), None, None, None],
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

/// Shaman's highest accessible spell level at a given class level,
/// **derived from `shaman_base_spells_per_day_table` itself** rather than
/// duplicated as its own ladder of threshold constants (Cleric's shape).
/// One source of truth means the ceiling and the slot budget cannot drift
/// apart -- a real hazard, since those are two hand-maintained tables
/// describing the same corpus rows.
pub(super) fn shaman_spell_level_access(level: u8) -> i16 {
    shaman_base_spells_per_day_table(level)
        .iter()
        .rposition(Option::is_some)
        .map_or(0, |index| index as i16)
}

/// The real per-day slot budget per spell level 0-9 (base table count +
/// Wisdom bonus spells). Orisons take no bonus, matching Cleric/Druid.
pub(super) fn shaman_total_spells_per_day(level: u8, wisdom_modifier: i16) -> [Option<i16>; 10] {
    let base = shaman_base_spells_per_day_table(level);
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

/// Parses a Shaman spell's `spell_id` into its real Shaman-specific spell
/// level by looking it up in `shaman_spell_list::SHAMAN_SPELL_LIST`.
/// Shaman is the one class with no `SPELLLIST:` reuse token, so this
/// consults its own freshly-ingested list rather than delegating.
pub(super) fn parse_shaman_spell_id(spell_id: &str) -> Option<u8> {
    shaman_spell_list::shaman_spell_level(spell_id)
}

/// Return the list of unmet conditions for Shaman's real prepared-spell
/// posture. Mirrors `unmet_cleric_prepared_spell_conditions` exactly --
/// same Wisdom casting stat, same 0-9 full-caster range, same
/// `MEMORIZE:YES` prepared shape (verified against `acg_classes.lst`'s own
/// `SPELLSTAT:WIS MEMORIZE:YES`, with no `SPELLBOOK:YES`, so there is no
/// separate recorded-spellbook step: a shaman prepares directly from the
/// full shaman spell list each day, exactly as a cleric does).
///
/// An empty list means the posture is fully valid. Zero prepared spells is
/// always valid, same reasoning as every other class in this family.
pub(super) fn unmet_shaman_prepared_spell_conditions(
    input: &CharacterInput,
    shaman_level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Vec<String> {
    let mut unmet = Vec::new();

    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == SHAMAN_CLASS_ID && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    let access_ceiling = shaman_spell_level_access(shaman_level);
    let total_per_day = shaman_total_spells_per_day(shaman_level, ability_modifiers.wisdom);

    let mut consumed_per_level: [i16; 10] = [0; 10];
    for spell_id in &prepared {
        let Some(spell_level) = parse_shaman_spell_id(spell_id) else {
            unmet.push(format!(
                "prepared spell '{spell_id}' is not on the real PF1 shaman spell list"
            ));
            continue;
        };
        if spell_level > 0 && i16::from(spell_level) > access_ceiling {
            unmet.push(format!(
                "prepared spell '{spell_id}' targets spell level {spell_level}, not yet \
                 accessible at shaman level {shaman_level} (access ceiling {access_ceiling})"
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
                 {total_slots} slots available (base + Wisdom bonus)"
            ));
        }
    }

    unmet
}

/// Ground Shaman's real prepared-spell posture once
/// `unmet_shaman_prepared_spell_conditions` reports an empty unmet list.
/// Mirrors `ground_cleric_prepared_spells` exactly.
pub(super) fn ground_shaman_prepared_spells(
    input: &CharacterInput,
    shaman_level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == SHAMAN_CLASS_ID && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.acg.shaman.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Shaman level {shaman_level} daily preparation selection ({} spells, \
             AcquisitionMode::Prepared): {}. Each prepared spell is verified against the real \
             PF1 shaman spell list (`shaman_spell_list::SHAMAN_SPELL_LIST`, the class's own \
             freshly-ingested 304-record list -- Shaman carries no `SPELLLIST:` reuse token, so \
             unlike Investigator or Oracle it shares no other class's list), the shaman's own \
             spell-level access ceiling, and the per-level slot budget (base table count + \
             Wisdom bonus). This grounds the prepared-spell selection for real; it computes no \
             spell save DC resolution against a target and no casting execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let total_per_day = shaman_total_spells_per_day(shaman_level, ability_modifiers.wisdom);
    for (spell_level, total) in total_per_day.iter().enumerate() {
        let Some(total) = total else {
            continue;
        };
        explanations.push(ComputationExplanation {
            id: format!(
                "class_spell.acg.shaman.total_spells_per_day.spell_level_{spell_level}"
            ),
            value: *total,
            detail: format!(
                "Shaman level {shaman_level} total spells per day at spell level \
                 {spell_level}: {total} (base table count from `acg_classes.lst`'s own `CAST:` \
                 row + Wisdom bonus; orisons take no bonus). This is the real slot budget the \
                 daily preparation selection above is validated against"
            ),
        });
    }
}

/// The shared `3+CHA` per-day resource eight of the ten Spirits' base
/// abilities use (`BONUS:VAR|Shaman<Ability>Times|3+CHA` on Touch of the
/// Grave, Touch of Flame, Stardust, Monstrous Insight, Storm Burst, Touch
/// of Acid, Wave Strike and Shocking Touch -- and, at higher tiers,
/// Enemies' Bane and Paragon of Battle).
///
/// **Battle Spirit deliberately does NOT use this**, despite carrying the
/// identical `3+CHA` formula: its variable is
/// `ShamanBattleSpiritRounds`, and its own DESC reads "The shaman can use
/// this ability %2 **rounds/day**" -- a duration pool, not a use count.
/// Same arithmetic, different resource; flattening the two would mislabel
/// it in the receipt a player reads.
///
/// Floored at 0, matching `shaman_channel_uses_per_day`'s own precedent.
pub(super) fn shaman_spirit_uses_per_day(charisma_modifier: i16) -> i16 {
    (3 + charisma_modifier).max(0)
}

/// The shared `ShamanSpiritLVL/2` bonus-damage magnitude five Spirits'
/// base abilities use: Bones' Touch of the Grave, Flame's Touch of Flame,
/// Stone's Touch of Acid, Waves' Wave Strike, and Wind's Shocking Touch.
/// One formula, five corpus records -- the same shared-mechanism shape as
/// Witch's 53 hexes sharing a single save-DC variable.
///
/// `ShamanSpiritLVL` resolves to `ShamanLVL` for a single-class Shaman.
/// It carries a second setter, `BONUS:VAR|ShamanSpiritLVL|SorcererLVL`,
/// but that belongs to `KEY:Spirit Summoner ~ Spirit` -- a Summoner
/// archetype this repo does not ingest -- so it is provably vacuous here,
/// the same archetype-gated shape already confirmed on Cavalier, Brawler
/// and Slayer.
pub(super) fn shaman_spirit_touch_bonus_damage(level: u8) -> i16 {
    i16::from(level) / 2
}

/// Battle Spirit's morale bonus on allies' attack and weapon damage
/// rolls: `1+min(2,ShamanSpiritLVL/8)`, i.e. +1 at levels 1-7, +2 at
/// 8-15, +3 at 16+. The `min(2,...)` is a real bound that DOES bind
/// within 1-20, unlike the inert bounds documented on Brawler's Flurry.
pub(super) fn shaman_battle_spirit_bonus(level: u8) -> i16 {
    1 + (i16::from(level) / 8).min(2)
}

/// Heavens' Stardust attack-roll/Perception penalty:
/// `-1*(1+min(5,ShamanSpiritLVL/4))`, i.e. -1 at levels 1-3 down to -6 at
/// 20. Returned as the real negative magnitude the corpus states, not an
/// absolute value.
pub(super) fn shaman_stardust_penalty(level: u8) -> i16 {
    -(1 + (i16::from(level) / 4).min(5))
}

/// Heavens' Stardust duration in rounds: `max(1,ShamanSpiritLVL/2)`. The
/// `max(1,...)` genuinely binds at levels 1 only.
pub(super) fn shaman_stardust_duration_rounds(level: u8) -> i16 {
    (i16::from(level) / 2).max(1)
}

/// Lore's Monstrous Insight bonus on the identifying Knowledge check:
/// flat `ShamanSpiritLVL`.
pub(super) fn shaman_monstrous_insight_bonus(level: u8) -> i16 {
    i16::from(level)
}

/// Nature's Storm Burst duration in rounds: `1+ShamanSpiritLVL/4`.
pub(super) fn shaman_storm_burst_duration_rounds(level: u8) -> i16 {
    1 + i16::from(level) / 4
}

/// PF1 Advanced Class Guide Life Spirit's Channel ability's uses per
/// day: `1+Charisma modifier`, verified directly against
/// `acg_abilities_class.lst`'s own `BONUS:VAR|ShamanChannelTimes|1+CHA`.
/// Floored at 0, mirroring Cleric's own `channel_energy_uses_per_day`
/// precedent (the corpus tag itself carries no explicit floor function).
pub(super) fn shaman_channel_uses_per_day(charisma_modifier: i16) -> i16 {
    (1 + charisma_modifier).max(0)
}

/// PF1 Advanced Class Guide Life Spirit's Channel ability's die count:
/// `(ShamanLVL+1)/2` d6, verified directly against
/// `acg_abilities_class.lst`'s own
/// `BONUS:VAR|ShamanChannelDice|(ShamanChannelLVL+1)/2` (where
/// `ShamanChannelLVL` resolves to `ShamanSpiritLVL` resolves to
/// `ShamanLVL`) and `BONUS:VAR|ShamanChannelDieSize|6`. Structurally
/// identical to Cleric's own Channel Energy die-count formula
/// (`ceil(level/2)`).
pub(super) fn shaman_channel_dice(level: u8) -> i16 {
    (i16::from(level) + 1) / 2
}

/// PF1 Advanced Class Guide Life Spirit's Channel ability's save DC:
/// `10+(ShamanLVL/2)+Charisma modifier`, verified directly against
/// `acg_abilities_class.lst`'s own
/// `BONUS:VAR|ShamanChannelDC|10+(ShamanChannelLVL/2)+CHA`. Unlike
/// Cleric's own Channel Energy (which this codebase deliberately does
/// not ground a DC for), Shaman's own corpus record genuinely carries
/// one, so it is grounded here, mirroring Warpriest's own Blessing DC
/// precedent.
pub(super) fn shaman_channel_dc(level: u8, charisma_modifier: i16) -> i16 {
    10 + i16::from(level) / 2 + charisma_modifier
}

/// Grounds or claim-blocks Shaman's Spirit choice (v0.6 alpha swarm,
/// risks item 8, Shaman full-build closure, 12th ACG/APG class-specific
/// closure). A recognized `choice:shaman_spirit` selection naming
/// `spirit:life` grounds Life Spirit's own immediately-available Channel
/// ability (flat uses-per-day, dice, and DC facts, no activation gate --
/// the same "flat, no gate" shape as Cleric's Channel Energy/Warpriest's
/// Blessings) and replaces the claim-blocking spirit-powers diagnostic
/// with a non-blocking note naming the other 9 primary spirits as still
/// deferred, mirroring Oracle's own Mystery/Curse and Witch's own Hex
/// three-branch dispatch shape. An unrecognized or missing choice keeps
/// a claim-blocking `spirit_powers.unsupported` diagnostic.
pub(super) fn ground_or_block_shaman_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    ground_familiar_master_benefit(input, level, explanations);

    let unmet_spells = unmet_shaman_prepared_spell_conditions(input, level, ability_modifiers);
    if unmet_spells.is_empty() {
        ground_shaman_prepared_spells(input, level, ability_modifiers, explanations);
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_spell.acg.shaman.prepared_spells.unsupported".to_owned(),
            message: format!(
                "{SHAMAN_CLASS_ID} prepared-spell posture is not satisfied: {}",
                unmet_spells.join("; ")
            ),
            claim_blocking: true,
        });
    }

    let spirit_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == SHAMAN_SPIRIT_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();

    // SD-32 T12 Epic 8 row 18 cycle 5: the generic "select ONE spirit,
    // inherit every one of its real corpus powers" pass, covering the
    // other 4 real Shaman spirit groups (73 real records total,
    // `census_class_feature_pool_group_names.py`) beyond the 10 this file
    // already hand-models by name below -- purely additive.
    push_generic_pool_group_selection_magnitude(
        input,
        level,
        ability_modifiers,
        SHAMAN_SPIRIT_CHOICE_ID,
        "Shaman",
        "Spirit",
        "spirit:",
        "class_feature.acg.shaman.spirit.generic",
        1,
        explanations,
    );

    let spirit_recognized;

    if spirit_selections.contains(&LIFE_SPIRIT_SELECTION) {
        let uses_per_day = shaman_channel_uses_per_day(ability_modifiers.charisma);
        let dice = shaman_channel_dice(level);
        let dc = shaman_channel_dc(level, ability_modifiers.charisma);

        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.life_spirit.channel_uses_per_day".to_owned(),
            value: uses_per_day,
            detail: format!(
                "Shaman level {level} with Life Spirit's Channel: usable max(1 + Charisma \
                 modifier ({}), 0) = {uses_per_day} times per day",
                ability_modifiers.charisma
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.life_spirit.channel_dice".to_owned(),
            value: dice,
            detail: format!(
                "Shaman level {level} with Life Spirit's Channel: (level+1)/2 = {dice}d6 \
                 positive energy damage/healing. Grounds only the flat die count; it computes \
                 no channel-energy burst damage or healing resolution"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.life_spirit.channel_dc".to_owned(),
            value: dc,
            detail: format!(
                "Shaman level {level} with Life Spirit's Channel: save DC 10 + (level/2) + \
                 Charisma modifier ({}) = {dc}",
                ability_modifiers.charisma
            ),
        });
        spirit_recognized = true;
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.acg.shaman.spirit_powers_beyond_life.unmodeled".to_owned(),
            message: "Shaman Spirit power content beyond Life's own Channel remains unmodeled: \
                 Life Spirit's own higher-tier abilities (Healer's Touch, gated to level 8+; \
                 Quick Healing; Manifestation) are not implemented. STALENESS CORRECTION \
                 (canonical-narrowing pass): this message also claimed \"the other 9 primary \
                 spirits (Battle, Bones, Flame, Heavens, Lore, Nature, Stone, Waves, Wind) and \
                 their own granted abilities are not implemented\". That is false and was \
                 already false when written -- `ground_shaman_spirit_base_ability` grounds the \
                 immediately-available base ability of all nine, which the sibling \
                 `spirit_powers_beyond_base.unmodeled` record states directly. This does not \
                 block an otherwise-valid Life-Spirit posture"
                .to_owned(),
            claim_blocking: false,
        });
    } else if ground_shaman_spirit_base_ability(
        &spirit_selections,
        level,
        ability_modifiers,
        explanations,
    ) {
        spirit_recognized = true;
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.acg.shaman.spirit_powers_beyond_base.unmodeled".to_owned(),
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   PRECLASS:1,Shaman=8
            //   PRECLASS:1,Shaman=16
            //   `BONUS:SAVE|ALL|...`, Life's Healer's Touch `BONUS:SKILL|Heal|4`, Lore's Perfect
            //   Knowledge `BONUS:SKILL|TYPE.Knowledge,Linguistics,Spellcraft|10|TYPE=Insight`
            //   Worth naming explicitly, because it is where the mechanical loss actually is: NONE
            //   of the ten base abilities carries a `BONUS:` landing on a total this engine
            //   computes -- every one is a `BONUS:VAR` feeding its own DESC text -- whereas the
            //   gated tiers genuinely do (Heavens' Manifestation).
            message: "Only the selected Spirit's own immediately-available base ability is grounded. \
                 Each Spirit's three higher-tier abilities stay deferred: the `ShamanSpiritGreater` \
                 tier, the `ShamanSpiritTrue` tier, and Manifestation (the capstone). This does not \
                 block an otherwise-valid Spirit posture"
                .to_owned(),
            claim_blocking: false,
        });
    } else {
        spirit_recognized = false;
        diagnostics.push(ComputationDiagnostic {
            id: "class_feature.acg.shaman.spirit_powers.unsupported".to_owned(),
            message: "Shaman remains blocked on its Spirit powers burden: no recognized Spirit \
                 choice is present. All ten primary Spirits (Battle, Bones, Flame, Heavens, \
                 Life, Lore, Nature, Stone, Waves, Wind) are recognized through their own \
                 immediately-available base ability, so an unrecognized selection means no \
                 Shaman Spirit-power support is claimed. The corpus also carries two \
                 later-book Spirits this codebase does not recognize -- Mammoth \
                 (`adventurers_guide/ag_abilities_class.lst`, Powerful Smash) and Wood \
                 (`ultimate_wilderness/uw_abilities_class.lst`, Tree Limb), neither of which \
                 carries any magnitude at all"
                .to_owned(),
            claim_blocking: true,
        });
    }

    push_shaman_other_features_deferred_diagnostic(diagnostics, spirit_recognized);
}

/// Ground the immediately-available (ungated) base ability of whichever
/// of the nine non-Life Spirits was selected, returning `true` if one was
/// recognized.
///
/// Every Spirit grants exactly one ability with no `PREVARGTEQ` gate;
/// those are the only ones grounded here. Each Spirit's other three
/// abilities are gated at `ShamanSpiritGreater` (`PRECLASS:1,Shaman=8`),
/// `ShamanSpiritTrue` (`PRECLASS:1,Shaman=16`) and `Shaman
/// Manifestation`, and all stay deferred -- the same tiering already
/// applied to Life's own Healer's Touch/Quick Healing/Manifestation.
pub(super) fn ground_shaman_spirit_base_ability(
    spirit_selections: &[&str],
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) -> bool {
    let charisma = ability_modifiers.charisma;
    let uses = shaman_spirit_uses_per_day(charisma);

    // The five Spirits whose base ability is a touch attack sharing one
    // `ShamanSpiritLVL/2` bonus-damage formula and one `3+CHA` use pool.
    let touch_family: [(&str, &str, &str); 5] = [
        (BONES_SPIRIT_SELECTION, "bones", "Touch of the Grave (negative energy)"),
        (FLAME_SPIRIT_SELECTION, "flame", "Touch of Flame (fire)"),
        (STONE_SPIRIT_SELECTION, "stone", "Touch of Acid (acid)"),
        (WAVES_SPIRIT_SELECTION, "waves", "Wave Strike (cold)"),
        (WIND_SPIRIT_SELECTION, "wind", "Shocking Touch (electricity)"),
    ];
    for (selection, key, label) in touch_family {
        if !spirit_selections.contains(&selection) {
            continue;
        }
        let bonus_damage = shaman_spirit_touch_bonus_damage(level);
        explanations.push(ComputationExplanation {
            id: format!("class_feature.acg.shaman.{key}_spirit.touch_bonus_damage"),
            value: bonus_damage,
            detail: format!(
                "Shaman level {level} with {} Spirit's {label}: the touch deals an extra \
                 ShamanSpiritLVL/2 = {bonus_damage} points of damage. This grounds the \
                 magnitude as a scoped standalone fact; no attack resolution against a target \
                 is computed",
                key_titlecase(key)
            ),
        });
        explanations.push(ComputationExplanation {
            id: format!("class_feature.acg.shaman.{key}_spirit.touch_uses_per_day"),
            value: uses,
            detail: format!(
                "Shaman level {level} with {} Spirit's {label}: usable max(3 + Charisma \
                 modifier ({charisma}), 0) = {uses} times per day, the same 3+CHA pool eight of \
                 the ten Spirits' base abilities share",
                key_titlecase(key)
            ),
        });
        return true;
    }

    if spirit_selections.contains(&BATTLE_SPIRIT_SELECTION) {
        let bonus = shaman_battle_spirit_bonus(level);
        let rounds = shaman_spirit_uses_per_day(charisma);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.battle_spirit.morale_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Shaman level {level} with Battle Spirit: allies within 30 feet (including the \
                 shaman) gain a +{bonus} morale bonus on attack rolls and weapon damage rolls \
                 (1+min(2,ShamanSpiritLVL/8)). The 30-foot radius and ally targeting are not \
                 modeled; this grounds the bonus magnitude only"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.battle_spirit.rounds_per_day".to_owned(),
            value: rounds,
            detail: format!(
                "Shaman level {level} with Battle Spirit: usable max(3 + Charisma modifier \
                 ({charisma}), 0) = {rounds} ROUNDS per day (not uses -- the corpus variable is \
                 `ShamanBattleSpiritRounds` and the ability's own text reads \
                 'rounds/day'). The rounds need not be consecutive; no round clock is enforced \
                 by this engine"
            ),
        });
        return true;
    }

    if spirit_selections.contains(&HEAVENS_SPIRIT_SELECTION) {
        let penalty = shaman_stardust_penalty(level);
        let duration = shaman_stardust_duration_rounds(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.heavens_spirit.stardust_penalty".to_owned(),
            value: penalty,
            detail: format!(
                "Shaman level {level} with Heavens Spirit's Stardust: the target takes a \
                 {penalty} penalty on attack rolls and Perception checks \
                 (-1*(1+min(5,ShamanSpiritLVL/4))), and cannot benefit from concealment. This \
                 grounds the penalty magnitude as a scoped standalone fact; no opposed \
                 resolution against a target is computed"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.heavens_spirit.stardust_duration_rounds".to_owned(),
            value: duration,
            detail: format!(
                "Shaman level {level} with Heavens Spirit's Stardust: the effect lasts \
                 max(1, ShamanSpiritLVL/2) = {duration} rounds. No round clock is enforced"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.heavens_spirit.stardust_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Shaman level {level} with Heavens Spirit's Stardust: usable max(3 + Charisma \
                 modifier ({charisma}), 0) = {uses} times per day"
            ),
        });
        return true;
    }

    if spirit_selections.contains(&LORE_SPIRIT_SELECTION) {
        let bonus = shaman_monstrous_insight_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.lore_spirit.monstrous_insight_bonus".to_owned(),
            value: bonus,
            detail: format!(
                "Shaman level {level} with Lore Spirit's Monstrous Insight: +{bonus} bonus \
                 (flat ShamanSpiritLVL) on the Knowledge check to identify a creature and its \
                 abilities. This grounds the bonus magnitude; the identification check itself \
                 is not resolved"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.lore_spirit.monstrous_insight_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Shaman level {level} with Lore Spirit's Monstrous Insight: usable max(3 + \
                 Charisma modifier ({charisma}), 0) = {uses} times per day"
            ),
        });
        return true;
    }

    if spirit_selections.contains(&NATURE_SPIRIT_SELECTION) {
        let duration = shaman_storm_burst_duration_rounds(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.nature_spirit.storm_burst_duration_rounds".to_owned(),
            value: duration,
            detail: format!(
                "Shaman level {level} with Nature Spirit's Storm Burst: the struck creature is \
                 buffeted for 1+ShamanSpiritLVL/4 = {duration} rounds. This grounds the \
                 duration magnitude; no attack resolution or round clock is computed"
            ),
        });
        explanations.push(ComputationExplanation {
            id: "class_feature.acg.shaman.nature_spirit.storm_burst_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                "Shaman level {level} with Nature Spirit's Storm Burst: usable max(3 + Charisma \
                 modifier ({charisma}), 0) = {uses} times per day"
            ),
        });
        return true;
    }

    false
}

/// Pushes the narrower diagnostic replacing
/// `class_feature.acg.shaman.unsupported` for Shaman specifically (v0.6
/// alpha swarm, risks item 8, Shaman full-build closure): names ONLY the
/// genuinely still-missing pieces.
///
/// **Canonical-narrowing pass**: this used to be pushed unconditionally
/// claim-blocking, so no Shaman could reach `Computed` however she
/// chose. It now carries the SAME id and the SAME named gaps in both
/// branches -- only `claim_blocking` differs -- the shape Arcanist's own
/// `exploits_deferred` established. Once a real Spirit is recognized,
/// what remains is breadth (Spirit Magic, the gated tiers, Wandering
/// Spirit, the hex chooser-lists), not a wrong number; with no Spirit at
/// all, the class's defining chooser is unanswered and that IS
/// claim-blocking.
pub(super) fn push_shaman_other_features_deferred_diagnostic(
    diagnostics: &mut Vec<ComputationDiagnostic>,
    spirit_recognized: bool,
) {
    let posture = if spirit_recognized {
        "Genuinely still deferred, and NOT claim-blocking now that the class's defining \
         chooser is answered by a corpus-verified Spirit"
    } else {
        "Claim-blocking, because no recognized Spirit choice is present at all. Also still \
         deferred"
    };
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.acg.shaman.other_features_deferred.unsupported".to_owned(),
        message: format!(
            // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
            //   `PRECLASS:1,Shaman=8`
            //   `PRECLASS:1,Shaman=16`
            "{SHAMAN_CLASS_ID} is grounded for its base-attack-bonus/base-save chassis pillar, its \
             own prepared spellcasting including Orisons, Spirit Animal's familiar master benefit, \
             and the immediately-available base ability of ALL TEN primary Spirits (Battle, Bones, \
             Flame, Heavens, Life, Lore, Nature, Stone, Waves, Wind). {posture}, ungrounded anywhere \
             in this codebase: Spirit Magic (the spirit-granted bonus spells layered on top of the \
             class list), Manifestation (a capstone ability), each Spirit's three higher abilities \
             gated at `ShamanSpiritGreater`, `ShamanSpiritTrue` and Manifestation, Wandering Spirit, \
             the two later-book Spirits (Mammoth, Wood), and the hex chooser-lists (13 `Shaman Hex`, \
             59 `Shaman Spirit Hex`, 59 `Shaman Wandering Hex` records). No class-feature or spell \
             execution is fabricated here. This message previously claimed \"the other 9 primary \
             spirits\" were ungrounded while the sibling `spirit_powers.unsupported` record \
             simultaneously stated all ten are recognized: the sibling was right and this clause was \
             stale. Battle, Heavens, Lore and Nature ship as ordinary `id: \"...\"` literals; Bones, \
             Flame, Stone, Waves and Wind ship as table-constructed \
             `format!(\"class_feature.acg.shaman.{{key}}_spirit.touch_bonus_damage\")` ids that an \
             `id: \"`-prefixed search does not match, which is how those five stayed invisible to an \
             id-grep audit (task #76). Wandering Spirit and Wandering Hex were named in NEITHER \
             clause. The Spirit Hex / Wandering Hex counts read 49/49 until the canonical-narrowing \
             pass re-derived them across the WHOLE corpus rather than `acg_abilities_class.lst` \
             alone: the Adventurer's Guide and Ultimate Wilderness each add 5 more of each, for \
             59/59"
        ),
        claim_blocking: !spirit_recognized,
    });
}

/// The bounded Druid milestone level this decomposition surface grounds, if any.
/// Returns the single Druid level when the chosen input is exactly a single-class
/// Druid at one of the supported milestone levels (1 through 10). Returns `None` for no
/// Druid, a non-Druid class, a multiclass mix, or any level-11+ Druid this slice
/// deliberately does not recognize — each of which stays claim-blocked exactly as
/// before. Mirrors the Fighter `supported_fighter_level` / Paladin
/// `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` / Bard `supported_bard_level` level-range gate idiom.
pub(super) fn supported_druid_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == DRUID_CLASS_ID
                && (1..=MAX_SUPPORTED_DRUID_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E4/SD13-E5 runtime evidence for the deterministic Human Druid
/// level-1/level-2/level-3 prepared divine spell-bearing baseline, while keeping it
/// explicitly claim-blocked on its remaining burdens. The SD13-E4 Wild Empathy slice
/// grounds Wild Empathy for real; the SD13-E5 Nature Sense / nature-bond-choice slice
/// grounds Nature Sense for real and recognizes the deterministic nature-bond
/// selection; a later SD13-E5 slice grounds the foundational base-attack-bonus /
/// base-save progression pillar that every other class row in this matrix already
/// has and Druid never had; a further SD13-E5 slice widens the level-1-only gate
/// (`supported_druid_level`, 1..=2) and extends every one of the formulas below to
/// level 2 via the same formula, without re-derivation, verified independently
/// against the PF1 Core Rulebook Druid class table (d20pfsrd and legacy.aonprd.com):
/// level 2 base attack bonus is +1, base saves are +3/+0/+3 (Fortitude/Reflex/Will).
/// That same slice also grounds Woodland Stride, the class table's level-2 "Special"
/// column entry, as a bounded identity record (flat/identity-shaped, no numeric
/// formula). A still further SD13-E5 slice widens the gate to level 3
/// (`supported_druid_level`, 1..=3), extending every formula above to level 3 via the
/// same formula (level 3 base attack bonus is +2, base saves are +3/+1/+3
/// Fortitude/Reflex/Will), keeps Woodland Stride granted (not re-derived), and grounds
/// Trackless Step, the class table's level-3 "Special" column entry, as a bounded
/// identity record (flat/identity-shaped, no numeric formula) mirroring the Woodland
/// Stride idiom exactly; Druid has no currently-grounded spell-slot-count pillar, so
/// there is no analogous level-3 doubling to widen. A further SD13-E5 slice widens the
/// gate to level 4 (`supported_druid_level`, 1..=4), extending every formula above to
/// level 4 via the same formula (level 4 base attack bonus is +3, base saves are
/// +4/+1/+4 Fortitude/Reflex/Will), keeping Woodland Stride and Trackless Step both
/// granted (not re-derived), and grounds Resist Nature's Lure, one of two distinct
/// entries in the class table's level-4 "Special" column, as a bounded flat-magnitude
/// identity record (+4 saving-throw bonus against fey spell-like/supernatural abilities
/// and plant-targeting spells/effects, never applied to any actual save total),
/// mirroring the Woodland Stride/Trackless Step idiom. The other level-4 "Special"
/// entry, Wild Shape (1/day), was checked and confirmed NOT flat (a full shapeshifting
/// subsystem with no execution engine anywhere in this codebase), so it is deliberately
/// left named-but-unproven, exactly like the animal-companion execution burden. A still
/// further SD13-E5 slice widens the gate to level 5 (`supported_druid_level`, 1..=5),
/// extending every formula above to level 5 via the same formula (level 5 base attack
/// bonus is +3, base saves are +4/+1/+4 Fortitude/Reflex/Will, all three numerically
/// unchanged from level 4 as an integer-division coincidence, not a stopped-scaling
/// formula), keeping Woodland Stride, Trackless Step, and Resist Nature's Lure all
/// granted (not re-derived); the PF1 Core Rulebook Druid class table's level-5
/// "Special" column is genuinely blank (verified independently against d20pfsrd and
/// legacy.aonprd.com rather than assumed), so no new pillar is grounded at level 5.
/// A still further SD13-E5 slice widens the gate to level 6 (`supported_druid_level`,
/// 1..=6), extending every formula above to level 6 via the same formula (level 6
/// base attack bonus is +4, base saves are +5/+2/+5 Fortitude/Reflex/Will, all three
/// genuinely new values), keeping Woodland Stride, Trackless Step, and Resist
/// Nature's Lure all granted (not re-derived). The class table's level-6 "Special"
/// column ("Wild shape (2/day)") was checked and confirmed NOT a genuinely separable
/// flat/identity-shaped element — the frequency increase is bundled with a
/// form-list expansion and a functioning-level upgrade, neither of which exist in
/// this codebase — so it is deliberately left named-but-unproven, exactly as at
/// level 4/5; no new pillar is grounded at level 6 either. A still further SD13-E5
/// slice widens the gate to level 7 (`supported_druid_level`, 1..=7), extending
/// every formula above to level 7 via the same formula (level 7 base attack bonus
/// is +5, a genuinely new value up from +4 at level 6; base saves are +5/+2/+5
/// Fortitude/Reflex/Will, all three numerically unchanged from level 6 — an
/// integer-division coincidence, re-verified against the raw class table rather
/// than assumed), keeping Woodland Stride, Trackless Step, and Resist Nature's
/// Lure all granted (not re-derived). The class table's level-7 "Special" column
/// is genuinely blank (verified independently against d20pfsrd and
/// legacy.aonprd.com rather than assumed): Wild Shape's next usage-count increase
/// ("Wild shape (3/day)") does not land until 8th level, so this slice makes no
/// Wild Shape claim at level 7 either way, and no new pillar is grounded at level
/// 7. The chosen bond's execution and the prepared divine spell posture burden
/// remain claim-blocked.
///
/// This deliberately does not compute a supported spell surface. It grounds no nature
/// bond power execution (no companion stat block, no companion advancement, no link /
/// share spells, no domain math), no spellbook posture, no spells prepared, no
/// spontaneous summon nature's ally conversion, no spell slots per day, no spell save
/// DCs, and no bonus spell slots from a high Wisdom. It only:
/// - leaves one recognition explanation so the `class:druid:N` identity is acknowledged
///   as a prepared divine spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value),
/// - leaves one grounded base-attack-bonus explanation (PF1 Core Rulebook Druid class
///   table: 3/4 BAB, the same formula shape as Rogue/Monk) and three grounded
///   base-save explanations (good Fortitude, good Will, poor Reflex), each a
///   standalone record not wired into `PilotBaseChassisComputation.base_attack_bonus`,
///   `compute_total_saves`, or `compute_combat_baseline`,
/// - leaves one grounded Wild Empathy explanation (the flat druid-level +
///   Charisma-modifier modifier, not a d20 roll and not a Diplomacy-check execution
///   engine),
/// - leaves one grounded Nature Sense explanation (the flat, level-independent PF1
///   +2 bonus on Knowledge (nature) and Survival checks, kept as a standalone record
///   not wired into any skill-check total),
/// - when the deterministic `choice:druid_nature_bond -> bond:animal_companion`
///   selection is present, leaves one +0 recognition record acknowledging that
///   selection without executing it (no record is fabricated when the selection is
///   absent),
/// - leaves one Woodland Stride explanation — a correct level-gate absence below
///   level 2, and a bounded identity/recognition record (value 0) at or above it,
///   mirroring exactly how Rogue's/Monk's own Evasion was grounded, with no
///   terrain-detection engine and no movement-execution engine implemented,
/// - leaves one Trackless Step explanation — a correct level-gate absence below
///   level 3, and a bounded identity/recognition record (value 0) at or above it,
///   mirroring exactly how Woodland Stride was grounded, with no tracking-resolution
///   engine and no terrain-detection engine implemented, and
/// - emits two distinct claim-blocking diagnostics naming the animal-companion
///   execution burden and the prepared divine spell posture burden explicitly,
///   rather than hiding behind a generic "unsupported caster" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Druid prepared divine spell-bearing
/// identity, its grounded base-attack/base-save/Wild Empathy/Nature Sense/Woodland
/// Stride values, its recognized nature-bond choice, and its remaining named burdens
/// legible on the runtime path.
pub(super) fn explain_druid_level1_spell_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // v0.6 alpha swarm, risks item 8, seventh slice (2026-07-25): both
    // Druid burdens are validated/checked regardless of whether Druid
    // appears alone or in a multiclass mix, and regardless of race --
    // checked BEFORE the single-class-only/Human gate below, mirroring the
    // Ranger/Paladin/Sorcerer/Cleric fix exactly. The animal-companion/
    // nature-bond burden is no longer flatly permanently unconditional (a
    // later slice made an animal companion's own Wolf stat block genuinely
    // closable at Druid level 1 -- see the conditional block below for the
    // full shape); the exact original unconditional diagnostic is
    // preserved as the catch-all for every case that closure doesn't
    // specifically cover (a domain-type bond, an unrecognized/absent
    // nature bond, or any Druid level other than 1). The prepared-divine
    // spell posture burden is a real, conditional validation, mirroring
    // `unmet_cleric_prepared_spell_conditions` exactly (a PREPARED caster,
    // like Cleric, not spontaneous): validates every
    // `AcquisitionMode::Prepared` selection with `source_class_id ==
    // "class:druid"` against the real
    // `druid_spell_list::DRUID_SPELL_LIST` (the general list only --
    // domain spells, when Nature Bond chooses a domain, stay part of the
    // separate nature-bond burden), the druid's own spell-level access
    // ceiling (1st+; orisons have no access gate), and the per-level slot
    // budget (base + Wisdom bonus, excluding any domain spell slot).
    if let Some(druid_level) = input
        .chosen
        .class_levels
        .iter()
        .find(|class_level| class_level.class_id == DRUID_CLASS_ID)
        .map(|class_level| class_level.level)
    {
        let animal_companion_chosen_top = choice_selection(input, DRUID_NATURE_BOND_CHOICE_ID)
            == Some(DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID);

        // v0.6 alpha swarm, risks item 8 (Druid animal companion closure,
        // adversarially reviewed 2026-07-25): the animal-companion burden
        // is no longer flatly unconditional for every Druid -- an animal
        // companion's own stat block (Wolf, the canonical species) can
        // genuinely close it. The catch-all below (no nature bond chosen,
        // or a domain-type bond chosen -- never recognized by this seam)
        // preserves the EXACT original unconditional diagnostic unchanged,
        // mirroring the Cleric review's catch-all-preservation requirement
        // exactly: no input this seam didn't specifically improve can
        // silently reach Computed.
        //
        // The former `&& druid_level == 1` half of this gate is gone: the
        // companion's Hit Dice progression is now read from the real
        // corpus table at every master level 1-20
        // (`ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL`), together with the
        // natural-armor and Strength advances that scale off the same
        // master level, so there is no longer a level at which this seam
        // would have to guess.
        //
        // What replaced it is the companion table's own domain bound. This
        // whole block deliberately runs BEFORE the `supported_druid_level`
        // single-class/level gate below, so that a multiclass Druid cannot
        // silently escape the burden -- which also means it is the first
        // thing an out-of-domain druid level reaches. `MAXLEVEL:20`
        // (`core_rulebook/cr_classes.lst`, CLASS:Druid) caps a real Druid at
        // 20, and `animal_companion_table_index`'s own doc comment states
        // that a caller passing a level outside 1-20 "is still a bug and
        // still trips in test/debug builds". Both are true, so the caller is
        // fixed here rather than the guard weakened: an out-of-domain level
        // falls through to the catch-all claim-blocking diagnostic below,
        // matching how every other class's level-21 implementation-gate
        // check already behaves, instead of panicking on a debug_assert.
        if animal_companion_chosen_top && druid_level <= MAX_ANIMAL_COMPANION_MASTER_LEVEL {
            // v0.6 alpha swarm, risks item 8 (fourth APG/ACG closure):
            // extracted into shared helpers so Hunter's own animal
            // companion (mechanically identical -- see
            // `ground_wolf_companion_stat_block`'s own doc comment) can
            // reuse this exact, already-3-source-verified math. Byte-for-
            // byte identical output to the original inline implementation
            // for Druid (owner_class_label = "Druid").
            ground_selected_companion_or_default(
                input,
                "class_chassis.druid.animal_companion",
                "Druid",
                druid_level,
                ground_wolf_companion_stat_block,
                explanations,
            );
            ground_wolf_companion_link_and_share_spells_vacuous(
                "class_feature.druid.animal_companion",
                "druid",
                explanations,
            );
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.druid.animal_companion.advancement_absent".to_owned(),
                message: "Druid animal companion advancement is grounded for every column that \
                     has a consumer in this engine -- Hit Dice across all twenty master levels \
                     (2 HD at 1st through 16 HD at 20th, the corpus's own \
                     cr_companionmods.lst progression), and with them base attack bonus, all \
                     three base saves and hit points, plus the natural-armor and Strength \
                     advances (2*floor(level/3) and floor(level/3), \
                     cr_abilities_companion.lst:59-60) that the armor-class and attack/damage \
                     records consume. Deliberately NOT grounded, because nothing in this \
                     codebase consumes them: the Dexterity half of the same stat advance (no \
                     Dexterity-to-companion-AC contribution is computed at all), bonus tricks \
                     (1+floor(level/3) -- no trick engine), the companion's skill ranks and \
                     feats (no companion skill or feat engine), the player-chosen Companion \
                     Stat Increase at master levels 4/9/14/20 (a chooser input with no \
                     canonical default), the optional species size advance offered from master \
                     level 7 for a Wolf, and the named abilities Evasion (3rd), Devotion (6th), \
                     Multiattack (9th), Spell Resistance (15th) and Improved Evasion (16th), \
                     none of which has an engine to act on"
                    .to_owned(),
                claim_blocking: false,
            });
        } else {
            diagnostics.push(ComputationDiagnostic {
                id: "class_feature.druid.animal_companion.unsupported".to_owned(),
                message: if animal_companion_chosen_top {
                    "Druid remains blocked on its animal companion execution burden: the chosen \
                     nature bond (an animal companion) is recognized as input only -- the \
                     companion's stat block, its advancement, and its link and share spells \
                     abilities are not implemented anywhere in this codebase, so no Druid animal \
                     companion support is claimed"
                        .to_owned()
                } else {
                    "Druid remains blocked on its animal companion execution burden: no nature bond \
                     selection is recognized as chosen input, and even when an animal companion bond \
                     or a domain is chosen, neither the companion's stat block/advancement/link and \
                     share spells abilities nor any domain's granted powers or spell-list contents \
                     are implemented anywhere in this codebase, so no Druid nature-bond support is \
                     claimed"
                        .to_owned()
                },
                claim_blocking: true,
            });
        }

        let unmet = unmet_druid_prepared_spell_conditions(input, druid_level, ability_modifiers);
        if unmet.is_empty() {
            ground_druid_prepared_spells(input, druid_level, ability_modifiers, explanations);
        } else {
            diagnostics.push(ComputationDiagnostic {
                id: "class_spell.druid.prepared_divine.unsupported".to_owned(),
                message: format!(
                    "Druid remains blocked on its prepared divine spell posture burden: Druid \
                     is a full 9th-level divine caster (spells begin at druid level 1); unmet \
                     prepared-spell posture: {}",
                    unmet.join("; ")
                ),
                claim_blocking: true,
            });
        }
    }

    let Some(level) = supported_druid_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Druid level-1/
    // level-2 prepared divine spell-bearing identity. This is a recognition record
    // only; it fabricates no nature-bond power math and no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.druid".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Druid level {level} prepared divine \
             spell-bearing baseline: the {DRUID_CLASS_ID}:{level} class identity is \
             acknowledged as a prepared divine spell-bearing class on the rules-core seam rather than \
             an undocumented packet placeholder. This is a bounded recognition record only; the Wild \
             Empathy and Nature Sense values and the nature-bond choice recognition are grounded \
             separately below, but this record still grounds no nature bond power execution, no \
             spellbook posture, no spells prepared per day, no spontaneous summon nature's ally \
             conversion, no spell slots per day, no spell save DCs, and no bonus spell slots from a \
             high Wisdom, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Grounded: the foundational base-attack-bonus / base-save progression pillar.
    // Unlike every other class row in this matrix (Fighter, Barbarian, Monk, Rogue,
    // Paladin all already ground this pillar), Druid had never had it grounded at
    // all until this SD13-E5 slice. Both formulas were verified against the PF1
    // Core Rulebook Druid class table (d20pfsrd and the legacy Paizo PRD mirror)
    // before writing this code, cross-checking the level 4/5 base-attack-bonus
    // values to disambiguate the exact fraction (level 1 alone floors both a 1/2
    // and a 3/4 progression to the same +0, so it cannot disambiguate on its own). A
    // later SD13-E5 slice widens this level-1-only gate to level 2; the formula is
    // extended, not re-derived (level 2 base attack +1, all base saves +3/+0/+3,
    // confirmed against the raw class table).
    let level_value = i16::from(level);

    // Grounded (1/2): 3/4-BAB base-attack progression, the same formula shape as
    // Rogue/Monk (classlevel * 3 / 4). No PCGen .lst file exists for the Druid
    // class in this repo, so the formula cites the PF1 Core Rulebook Druid class
    // table directly.
    let base_attack_bonus = level_value * 3 / 4;
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Druid level {level} base attack bonus from the PF1 Core Rulebook \
             Druid class table's 3/4-BAB progression, the same formula shape as Rogue/Monk: \
             classlevel * 3 / 4 = {base_attack_bonus}. This is a standalone explanation record; \
             it is not wired into the integrated base_attack_bonus field or into \
             compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — good Fortitude, poor Reflex, good
    // Will, verified against the PF1 Core Rulebook Druid class table (Fortitude
    // +2, Reflex +0, Will +2 at level 1; +3/+0/+3 at level 2).
    let good_save = level_value / 2 + 2;
    let poor_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.base_save.fortitude".to_owned(),
        value: good_save,
        detail: format!(
            "Druid level {level} base Fortitude save (good save) from the PF1 \
             Core Rulebook Druid class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.base_save.reflex".to_owned(),
        value: poor_save,
        detail: format!(
            "Druid level {level} base Reflex save (poor save) from the PF1 Core \
             Rulebook Druid class table: classlevel/3 = {poor_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.base_save.will".to_owned(),
        value: good_save,
        detail: format!(
            "Druid level {level} base Will save (good save) from the PF1 Core \
             Rulebook Druid class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded: Wild Empathy (PF1 Core Rulebook). A druid uses Wild Empathy to
    // improve the attitude of an animal, resolved like a Diplomacy check: the
    // druid rolls 1d20 and adds her druid level and her Charisma modifier. Only
    // the flat level + Cha-modifier bonus is grounded here; no d20 roll and no
    // Diplomacy-check/attitude-outcome execution engine is computed. This formula
    // was already level-generic (it takes `level` as a value, not a hardcoded
    // baseline), so it extends correctly to level 2 without re-derivation.
    let wild_empathy_modifier = level_value + ability_modifiers.charisma;
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.wild_empathy".to_owned(),
        value: wild_empathy_modifier,
        detail: format!(
            "Druid Wild Empathy modifier (PF1 Core Rulebook): a druid uses Wild Empathy to improve \
             an animal's attitude as if making a Diplomacy check, rolling 1d20 and adding her druid \
             level and her Charisma modifier. At Druid level {level} with a Charisma \
             modifier of {}, the modifier is {level} + {} = {wild_empathy_modifier}. \
             This grounds only the flat druid-level + Charisma-modifier bonus; it computes no d20 \
             roll, no Diplomacy-check resolution, and no attitude-improvement outcome",
            ability_modifiers.charisma, ability_modifiers.charisma
        ),
    });

    // Grounded: Nature Sense (PF1 Core Rulebook). A druid gains a +2 bonus on
    // Knowledge (nature) and Survival checks. Flat and level-independent; grounded
    // as a standalone record only — it is not wired into any skill-check total and
    // resolves no Knowledge (nature) or Survival check. Confirmed unchanged at
    // level 2 via the same formula, not a new record.
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.nature_sense".to_owned(),
        value: DRUID_NATURE_SENSE_BONUS,
        detail: format!(
            "Druid Nature Sense bonus (PF1 Core Rulebook): a druid gains a \
             +{DRUID_NATURE_SENSE_BONUS} bonus on Knowledge (nature) and Survival checks. The \
             bonus is flat and level-independent. This is a standalone grounded record only: it \
             is not wired into any computed skill-check total and it resolves no Knowledge \
             (nature) or Survival check"
        ),
    });

    // Recognized: the deterministic nature-bond selection. The fixture carries
    // `choice:druid_nature_bond -> bond:animal_companion`; when that selection is
    // present it is acknowledged as chosen input, carrying no fabricated bond
    // execution. When the selection is absent (the desktop composer threads no
    // nature-bond slot) no record is fabricated. This recognition is not
    // level-gated; it still fires at level 2 for the same fixture selection.
    let animal_companion_chosen = choice_selection(input, DRUID_NATURE_BOND_CHOICE_ID)
        == Some(DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID);
    if animal_companion_chosen {
        explanations.push(ComputationExplanation {
            id: "class_chassis.druid.nature_bond_choice".to_owned(),
            value: 0,
            detail: format!(
                "Recognized Druid nature bond selection ({DRUID_NATURE_BOND_CHOICE_ID} -> \
                 {DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID}): the deterministic fixture \
                 chooses an animal companion as its PF1 nature bond. This is a bounded \
                 recognition record of the chosen input only; the chosen bond's execution stays \
                 ungrounded — no animal companion stat block, no companion advancement, and no \
                 link or share-spells behavior is computed — so it carries no fabricated \
                 mechanical value (+0)"
            ),
        });
    }

    // Grounded (SD13-E5): Woodland Stride, a 2nd-level Druid class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Woodland stride" as the Druid 2nd-level special
    // feature entry). Below the level-2 gate this is a correct PF1 Core Rulebook
    // level-gate absence (value 0); at or above it, it is a bounded
    // identity/recognition record only (value 0, non-fabricated) naming the rule
    // text — mirroring exactly how Rogue's/Monk's own Evasion was grounded, without
    // folding into any actual terrain-detection engine or movement-execution
    // engine, neither of which exists in this codebase.
    if level < DRUID_WOODLAND_STRIDE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.woodland_stride".to_owned(),
            value: 0,
            detail: format!(
                "Druid Woodland Stride at druid level {level}: correctly absent at level {level} \
                 by PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Woodland Stride is a 2nd-level druid class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.woodland_stride".to_owned(),
            value: 0,
            detail: format!(
                "Druid Woodland Stride granted at druid level {level} (PF1 Core Rulebook, \
                 2nd-level druid class feature): starting at 2nd level, a druid may move through \
                 any sort of undergrowth (natural thorns, briars, overgrown areas, and similar \
                 terrain) at her normal speed and without taking damage or suffering any other \
                 impairment; magically manipulated terrain still affects her. This is a bounded \
                 identity/recognition record only (value 0, non-fabricated): no terrain-detection \
                 engine and no movement-execution engine exists anywhere in this codebase to apply \
                 it, so this grounds no actual movement or terrain-impediment resolution"
            ),
        });
    }

    // Grounded (SD13-E5): Trackless Step, a 3rd-level Druid class feature verified
    // independently against two primary PF1 sources (d20pfsrd and legacy.aonprd.com
    // both list "Trackless step" as the Druid 3rd-level special feature entry). Below
    // the level-3 gate this is a correct PF1 Core Rulebook level-gate absence (value
    // 0); at or above it, it is a bounded identity/recognition record only (value 0,
    // non-fabricated) naming the rule text — mirroring exactly how Woodland Stride and
    // Rogue's/Monk's own Evasion were grounded, without folding into any actual
    // tracking-resolution engine or terrain-detection engine, neither of which exists
    // in this codebase.
    if level < DRUID_TRACKLESS_STEP_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.trackless_step".to_owned(),
            value: 0,
            detail: format!(
                "Druid Trackless Step at druid level {level}: correctly absent at level {level} \
                 by PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Trackless Step is a 3rd-level druid class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.trackless_step".to_owned(),
            value: 0,
            detail: format!(
                "Druid Trackless Step granted at druid level {level} (PF1 Core Rulebook, \
                 3rd-level druid class feature): starting at 3rd level, a druid leaves no trail \
                 in natural surroundings and cannot be tracked; she may choose to leave a trail \
                 if so desired. This is a bounded identity/recognition record only (value 0, \
                 non-fabricated): no tracking-resolution engine and no terrain-detection engine \
                 exists anywhere in this codebase to apply it, so this grounds no actual \
                 tracking-check or trail-detection resolution"
            ),
        });
    }

    // Grounded (SD13-E5): Resist Nature's Lure, one of two distinct entries in the
    // class table's 4th-level "Special" column, verified independently against two
    // primary PF1 sources (d20pfsrd and legacy.aonprd.com both list "Resist nature's
    // lure" alongside "Wild shape (1/day)" as the Druid 4th-level special feature
    // entry). Below the level-4 gate this is a correct PF1 Core Rulebook level-gate
    // absence (value 0); at or above it, it is a bounded flat-magnitude identity
    // record only (the rule's own flat +4 magnitude, non-fabricated as an applied
    // total) — mirroring exactly how Bravery/Divine Grace/Trap Sense were grounded:
    // this record is never wired into any actual saving-throw total, since no
    // saving-throw resolution engine exists in this codebase. The class table's
    // other level-4 entry, Wild Shape (1/day), was checked and confirmed NOT flat —
    // it is a full shapeshifting subsystem (new form, new stat block, duration
    // tracking) with no execution engine anywhere in this codebase — so it is
    // deliberately left named-but-unproven here, exactly like the animal-companion
    // execution burden below, and no record or diagnostic for it is fabricated.
    if level < DRUID_RESIST_NATURES_LURE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.resist_natures_lure".to_owned(),
            value: 0,
            detail: format!(
                "Druid Resist Nature's Lure at druid level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant flat magnitude is named but \
                 not computed. Resist Nature's Lure is a 4th-level druid class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.resist_natures_lure".to_owned(),
            value: DRUID_RESIST_NATURES_LURE_BONUS,
            detail: format!(
                "Druid Resist Nature's Lure granted at druid level {level} (PF1 Core Rulebook, \
                 4th-level druid class feature): a druid gains a +{DRUID_RESIST_NATURES_LURE_BONUS} \
                 bonus on saving throws against the spell-like and supernatural abilities of fey; \
                 this bonus also applies to spells and effects that utilize or target plants, such \
                 as blight, entangle, spike growth, and warp wood. This is a bounded flat-magnitude \
                 identity record only: no saving-throw resolution engine exists anywhere in this \
                 codebase to apply it, so this grounds no actual saving-throw total"
            ),
        });
    }

    // Grounded (SD13-E5 level-9 slice): Venom Immunity, the 9th-level Druid class
    // feature verified independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Venom immunity" as the Druid 9th-level "Special"
    // entry, the rule text reading "At 9th level, a druid gains immunity to all
    // poisons"). A genuinely flat/identity-shaped, no-choice, no-magnitude grant —
    // exactly like Monk's Purity of Body (immunity to disease) — grounded as a
    // bounded +0 identity/recognition record at or above the gate: no
    // poison/condition-resolution engine exists anywhere in this codebase to apply
    // it, so no immunity effect is fabricated. Below the level-9 gate no record is
    // pushed at all (the level-9 slice's own level-8 control pins that absence).
    if level >= DRUID_VENOM_IMMUNITY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.venom_immunity".to_owned(),
            value: 0,
            detail: format!(
                "Druid Venom Immunity granted at druid level {level} (PF1 Core Rulebook, \
                 9th-level druid class feature): the druid gains immunity to all poisons. \
                 This is a bounded identity/recognition record only (value 0, non-fabricated): \
                 no poison-application or condition-resolution engine exists anywhere in this \
                 codebase to apply it, so this grounds no actual immunity effect on any \
                 poison outcome"
            ),
        });
    }

    // Grounded (SD18 level-13 slice): A Thousand Faces, the 13th-level Druid class
    // feature verified independently against three primary PF1 sources (d20pfsrd,
    // Archives of Nethys aonprd.com, and legacy.aonprd.com all list "A thousand
    // faces" as the Druid 13th-level "Special" column entry). UNLIKE the class
    // table's Wild-Shape-shaped entries at levels 4/6/8/10/12, this is a genuinely
    // flat/identity-shaped, no-choice, no-magnitude, no-duration-tracking, at-will
    // grant — in PF1 (unlike the D&D 3.5 version of this ability, which referenced
    // the stronger `alter self` spell), the druid gains the ability to change her
    // own apparent appearance at will, as if using `disguise self`, but only while
    // in her normal (unshifted) form — mirroring exactly how Venom Immunity was
    // grounded at level 9: a bounded +0 identity/recognition record at or above the
    // gate: no illusion-effect execution engine and no Disguise-check-resolution
    // engine exists anywhere in this codebase to apply it, so no actual
    // appearance-change or Disguise-check outcome is fabricated. Below the
    // level-13 gate no record is pushed at all (the level-13 slice's own level-12
    // control pins that absence).
    if level >= DRUID_A_THOUSAND_FACES_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.a_thousand_faces".to_owned(),
            value: 0,
            detail: format!(
                "Druid A Thousand Faces granted at druid level {level} (PF1 Core Rulebook, \
                 13th-level druid class feature): the druid gains the ability to change her \
                 own apparent appearance at will, as if using disguise self, but only while in \
                 her normal (unshifted) form. This is a bounded identity/recognition record \
                 only (value 0, non-fabricated): no illusion-effect execution engine and no \
                 Disguise-check-resolution engine exists anywhere in this codebase to apply it, \
                 so this grounds no actual appearance-change or Disguise-check outcome"
            ),
        });
    }

    // Grounded (SD18 level-15 slice): Timeless Body, the 15th-level Druid class
    // feature verified independently against three primary PF1 sources (d20pfsrd,
    // Archives of Nethys aonprd.com, and legacy.aonprd.com all list "Timeless body"
    // — and ONLY that entry, with no accompanying Wild Shape frequency increase —
    // as the Druid 15th-level "Special" column entry). A genuinely flat/
    // identity-shaped, no-choice, no-magnitude, no-duration-tracking grant — a
    // druid no longer takes ability score penalties for old age and cannot be
    // magically aged (existing penalties remain in place; bonuses still accrue) —
    // mirroring exactly how Venom Immunity was grounded at level 9 and A Thousand
    // Faces at level 13: a bounded +0 identity/recognition record at or above the
    // gate: no aging-penalty-resolution engine exists anywhere in this codebase to
    // apply it, so this grounds no actual ability-score-penalty or magical-aging
    // outcome. Below the level-15 gate no record is pushed at all (the level-15
    // slice's own level-14 control pins that absence).
    if level >= DRUID_TIMELESS_BODY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.timeless_body".to_owned(),
            value: 0,
            detail: format!(
                "Druid Timeless Body granted at druid level {level} (PF1 Core Rulebook, \
                 15th-level druid class feature): the druid no longer takes ability score \
                 penalties for old age and cannot be magically aged (existing penalties remain \
                 in place; bonuses still accrue). This is a bounded identity/recognition record \
                 only (value 0, non-fabricated): no aging-penalty-resolution engine exists \
                 anywhere in this codebase to apply it, so this grounds no actual \
                 ability-score-penalty or magical-aging outcome"
            ),
        });
    }

    // v0.6 alpha swarm, risks item 8, seventh slice (2026-07-25): the real
    // Druid spell math ladder, built from scratch (matching Cleric's own
    // build -- neither pre-existed). Druid's base spells-per-day table is
    // byte-for-byte identical to Cleric's (verified independently against
    // two primary sources), so these are the same values, just without the
    // guaranteed "+1" domain slot (Druid's Nature Bond choice is EITHER an
    // animal companion OR a domain -- when a domain is chosen the same
    // domain slot would apply, but that choice and its slot contents stay
    // part of the separate animal-companion/nature-bond burden).
    let druid_base_spells_per_day = druid_base_spells_per_day_table(level);
    for (spell_level, base_count) in druid_base_spells_per_day.iter().enumerate() {
        let Some(base_count) = base_count else {
            continue;
        };
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.druid.base_spells_per_day.spell_level_{spell_level}"),
            value: *base_count,
            detail: format!(
                "Druid base spells per day at druid level {level}, spell level {spell_level}: \
                 {base_count}, read directly from the PF1 Core Rulebook Druid class table's \
                 spells-per-day row (verified against the raw table rows of both primary \
                 sources; a literal table lookup, not a derived formula; byte-for-byte identical \
                 to Cleric's own base table). This grounds the base count only: bonus spells \
                 per day from a high Wisdom are never computed here, no prepared posture or \
                 spell-source lineage is grounded, and no spell save DCs are computed"
            ),
        });
    }

    let druid_spell_level_access = druid_spell_level_access(level);
    for spell_level in 1..=druid_spell_level_access {
        let spell_save_dc = 10 + spell_level + ability_modifiers.wisdom;
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.druid.spell_save_dc.spell_level_{spell_level}"),
            value: spell_save_dc,
            detail: format!(
                "Druid spell save DC at druid level {level}, spell level {spell_level}: 10 + \
                 {spell_level} + Wisdom modifier {} = {spell_save_dc} (PF1 Core Rulebook: \"The \
                 Difficulty Class for a saving throw against a druid's spell is 10 + the spell \
                 level + the druid's Wisdom modifier\"). This grounds the base DC formula only: \
                 no saving-throw resolution, no target, no spell selection, and no domain DC \
                 modifiers are computed",
                ability_modifiers.wisdom
            ),
        });
    }

    for spell_level in 1..=druid_spell_level_access {
        let bonus_spells = ability_bonus_spells(ability_modifiers.wisdom, spell_level);
        explanations.push(ComputationExplanation {
            id: format!("class_chassis.druid.bonus_spells_per_day.spell_level_{spell_level}"),
            value: bonus_spells,
            detail: format!(
                "Druid bonus spells per day at druid level {level}, spell level {spell_level}: \
                 {bonus_spells} from Wisdom modifier {} (PF1 Core Rulebook Table: Ability \
                 Modifiers and Bonus Spells). A computed 0 means the modifier grants no bonus at \
                 this spell level; it is never added to the base per-day count here -- no total \
                 is computed, no spell selection, and no spell save DCs",
                ability_modifiers.wisdom
            ),
        });
    }

    for (spell_level, base_count) in druid_base_spells_per_day.iter().enumerate() {
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
            id: format!("class_chassis.druid.total_spells_per_day.spell_level_{spell_level}"),
            value: total_spells,
            detail: format!(
                "Druid total spells per day at druid level {level}, spell level {spell_level}: \
                 base table count {base_count} + Wisdom bonus {bonus_spells} = {total_spells} -- \
                 the pure sum of the two separately grounded records, giving the actual \
                 castable slot count per day (excluding any domain spell slot). This grounds \
                 the count only: no prepared-posture selection, no casting execution, no slot \
                 consumption or tracking, and no spell save resolution"
            ),
        });
    }

    // (v0.6 alpha swarm, risks item 8) Both remaining burdens are pushed
    // at the top of this function (see that push site's own doc comment
    // for the full conditional shape): the animal-companion/nature-bond
    // burden is no longer flatly unconditional -- an animal companion's
    // own Wolf stat block can genuinely close it at Druid level 1 -- and
    // the prepared divine spell posture is a real, conditional validation,
    // same as before.
}

/// The highest ACCESSIBLE druid spell level (1st+) at the given druid
/// level -- orisons (0th level) have no access gate at all, always
/// available from level 1. Pure function, race-independent, mirrors
/// `cleric_spell_level_access` exactly (the same table shape) -- extracted
/// so both the (Human-only) flat explanation block above and the real
/// prepared-spell validation below share one source of truth.
pub(super) fn druid_spell_level_access(level: u8) -> i16 {
    if level >= DRUID_NINTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        9
    } else if level >= DRUID_EIGHTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        8
    } else if level >= DRUID_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        7
    } else if level >= DRUID_SIXTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        6
    } else if level >= DRUID_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        5
    } else if level >= DRUID_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        4
    } else if level >= DRUID_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        3
    } else if level >= DRUID_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        2
    } else {
        1
    }
}

/// The PF1 Core Rulebook Druid class table's BASE spells-per-day row, one
/// entry per spell level 0-9 (`None` for an inaccessible "—" column; index
/// 0 is orisons). A literal table lookup, not a derived formula -- verified
/// against two independent primary sources (d20pfsrd.com and the Archives
/// of Nethys aonprd.com mirror, byte-for-byte identical, and matching
/// Cleric's own already-grounded base table exactly). Excludes any domain
/// spell slot (only applies when Nature Bond chooses a domain, itself part
/// of the separate animal-companion/nature-bond burden). Pure function,
/// race-independent, extracted for the same reason as
/// `druid_spell_level_access`.
pub(super) fn druid_base_spells_per_day_table(level: u8) -> [Option<i16>; 10] {
    // Byte-for-byte identical to cleric_base_spells_per_day_table -- same
    // real PF1 Core Rulebook table, shared source of truth.
    cleric_base_spells_per_day_table(level)
}

/// The real per-day slot budget per spell level 0-9 (base table count +
/// Wisdom bonus for 1st+, orisons never get a bonus, `None` for an
/// inaccessible column), excluding any domain spell slot.
pub(super) fn druid_total_spells_per_day(level: u8, wisdom_modifier: i16) -> [Option<i16>; 10] {
    let base = druid_base_spells_per_day_table(level);
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

/// Return the list of unmet conditions for Druid's real prepared-spell
/// posture. Mirrors `unmet_cleric_prepared_spell_conditions` exactly,
/// substituting `druid_spell_list::DRUID_SPELL_LIST` (the general list
/// only -- domain spells, when Nature Bond chooses a domain, stay part of
/// the separate nature-bond burden) for the cleric list. An empty list
/// means the posture is fully valid: every `AcquisitionMode::Prepared`
/// selection with `source_class_id == "class:druid"` names a real
/// general-list spell, at a spell level within the druid's own access
/// ceiling (0 always accessible), and no spell level's prepared count
/// exceeds that level's total slot budget (base + Wisdom bonus, excluding
/// any domain slot). Zero prepared spells is always valid, same reasoning
/// as every other class in this family.
pub(super) fn unmet_druid_prepared_spell_conditions(
    input: &CharacterInput,
    druid_level: u8,
    ability_modifiers: &AbilityModifiers,
) -> Vec<String> {
    let mut unmet = Vec::new();

    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == DRUID_CLASS_ID && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    let access_ceiling = druid_spell_level_access(druid_level);
    let total_per_day = druid_total_spells_per_day(druid_level, ability_modifiers.wisdom);

    let mut consumed_per_level: [i16; 10] = [0; 10];
    for spell_id in &prepared {
        let Some(spell_level) = druid_spell_list::druid_spell_level(spell_id) else {
            unmet.push(format!(
                "prepared spell '{spell_id}' is not on the real PF1 general druid spell list"
            ));
            continue;
        };
        if spell_level > 0 && i16::from(spell_level) > access_ceiling {
            unmet.push(format!(
                "prepared spell '{spell_id}' targets spell level {spell_level}, not yet \
                 accessible at druid level {druid_level} (access ceiling {access_ceiling})"
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
                 {total_slots} slots available (base + Wisdom bonus, excluding any domain slot)"
            ));
        }
    }

    unmet
}

/// Ground the real prepared-spell posture once
/// `unmet_druid_prepared_spell_conditions` reports an empty unmet list.
/// Mirrors `ground_cleric_prepared_spells` exactly.
pub(super) fn ground_druid_prepared_spells(
    input: &CharacterInput,
    druid_level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let prepared: Vec<&str> = input
        .chosen
        .spells_selected
        .iter()
        .filter(|s| {
            s.source_class_id == DRUID_CLASS_ID && s.acquisition_mode == AcquisitionMode::Prepared
        })
        .map(|s| s.spell_id.as_str())
        .collect();

    explanations.push(ComputationExplanation {
        id: "class_spell.druid.daily_preparation".to_owned(),
        value: prepared.len() as i16,
        detail: format!(
            "Druid level {druid_level} daily preparation selection ({} spells, \
             AcquisitionMode::Prepared): {}. Each prepared spell is verified against the real \
             PF1 general druid spell list (`druid_spell_list::DRUID_SPELL_LIST`, all \
             ingested books), \
             the druid's own spell-level access ceiling, and the per-level slot budget (base \
             table count + Wisdom bonus, excluding any domain spell slot). This grounds the \
             prepared-spell selection for real; it computes no spell save DC resolution against \
             a target and no casting execution",
            prepared.len(),
            prepared.join(", ")
        ),
    });

    let total_per_day = druid_total_spells_per_day(druid_level, ability_modifiers.wisdom);
    for (spell_level, total) in total_per_day.iter().enumerate() {
        let Some(total) = total else {
            continue;
        };
        explanations.push(ComputationExplanation {
            id: format!("class_spell.druid.total_spells_per_day.spell_level_{spell_level}"),
            value: *total,
            detail: format!(
                "Druid level {druid_level} total spells per day at spell level {spell_level}: \
                 {total} (base table count + Wisdom bonus, excluding any domain spell slot -- \
                 the same records already grounded as \
                 `class_chassis.druid.total_spells_per_day.spell_level_{spell_level}` for a \
                 Human druid, computed here independent of race). This is the real slot budget \
                 the daily preparation selection above is validated against"
            ),
        });
    }
}

/// v0.6 alpha swarm, risks item 8, seventh slice (2026-07-25): Druid's real
/// prepared-divine spell posture, mirroring
/// `cleric_dispatch_widening_safety_tests` exactly (PREPARED, a full
/// 9th-level caster, same gate-ordering structural risk existed here too
/// and was fixed proactively as part of this same slice). Unlike
/// Ranger/Paladin/Cleric, Druid's animal-companion/nature-bond burden
/// stays permanently unconditional (mirrors Sorcerer's bloodline-power and
/// Cleric's domain-powers shape), so a single-class Druid never reaches
/// `Computed` even with a fully valid prepared-spell posture -- only the
/// spell-specific diagnostic is conditional.
#[cfg(test)]
mod druid_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, AcquisitionMode, CharacterClassLevel, CharacterInput,
        DRUID_CLASS_ID, DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID,
        DRUID_NATURE_BOND_CHOICE_ID, FIGHTER_CLASS_ID, HeadlessReceiptStatus,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    /// A single-class Druid with no invalid prepared-spell selection still
    /// stays `Blocked` (the animal-companion/nature-bond burden is
    /// permanently unconditional), but the spell-specific diagnostic must
    /// NOT fire when the posture is genuinely valid.
    #[test]
    fn single_class_druid_with_no_prepared_spells_stays_blocked_only_on_nature_bond() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: DRUID_CLASS_ID.to_owned(), level: 5 }];

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Druid's animal-companion/nature-bond burden is permanently unconditional: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.druid.animal_companion.unsupported"
                    && d.claim_blocking),
            "expected the permanent nature-bond diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.druid.prepared_divine.unsupported"),
            "the spell-posture diagnostic must not fire when the prepared-spell posture is \
             valid: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Druid preparing a real, in-budget, accessible orison
    /// (0th-level spell, always accessible from level 1) does not trip the
    /// spell-posture diagnostic -- proving orisons have no access gate.
    #[test]
    fn single_class_druid_with_a_valid_orison_does_not_trip_the_spell_posture() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: DRUID_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Detect Magic".to_owned(),
            source_class_id: DRUID_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.druid.prepared_divine.unsupported"),
            "an orison is always accessible from level 1 with no access gate: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Druid preparing a spell beyond their spell-level
    /// access ceiling must carry the real spell-posture diagnostic.
    #[test]
    fn single_class_druid_with_an_inaccessible_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: DRUID_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Barkskin".to_owned(),
            source_class_id: DRUID_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.druid.prepared_divine.unsupported"
                    && d.claim_blocking),
            "a 2nd-level druid spell is not accessible at druid level 1: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Druid preparing a spell not on the real PF1 Core
    /// Rulebook general druid spell list at all must also carry the
    /// diagnostic.
    #[test]
    fn single_class_druid_with_an_off_list_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: DRUID_CLASS_ID.to_owned(), level: 1 }];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: DRUID_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Magic Missile is not on the real general druid spell list: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Druid at level 1 (Wisdom 12, mod +1: base 1 + bonus
    /// 1 = total budget 2 for 1st-level spells) preparing 3 distinct
    /// 1st-level spells over-prepares the real slot budget.
    #[test]
    fn single_class_druid_over_prepared_slot_budget_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: DRUID_CLASS_ID.to_owned(), level: 1 }];
        for spell_id in ["Calm Animals", "Charm Animal", "Cure Light Wounds"] {
            input.chosen.spells_selected.push(SpellSelection {
                spell_id: spell_id.to_owned(),
                source_class_id: DRUID_CLASS_ID.to_owned(),
                acquisition_mode: AcquisitionMode::Prepared,
            });
        }

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "druid level 1 with Wisdom 12 has a real total budget of 2 slots for 1st-level \
             spells (base 1 + Wisdom bonus 1), so preparing 3 distinct spells over-prepares: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.druid.prepared_divine.unsupported"
                    && d.claim_blocking),
            "expected the real spell-posture diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Multiclass safety, verified directly. A Druid-containing multiclass
    /// mix with a genuine posture violation must still stay Blocked, since
    /// `DRUID_CLASS_ID` is deliberately not registered with
    /// `multiclass_class_level_supported` beyond `table_class_id` itself
    /// (the same construction Ranger/Paladin/Sorcerer/Cleric already
    /// proved safe).
    #[test]
    fn druid_fighter_multiclass_with_an_invalid_prepared_spell_stays_blocked() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels = vec![
            CharacterClassLevel { class_id: DRUID_CLASS_ID.to_owned(), level: 1 },
            CharacterClassLevel { class_id: FIGHTER_CLASS_ID.to_owned(), level: 1 },
        ];
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: DRUID_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "a Druid+Fighter multiclass must not reach Computed while Druid's posture is \
             genuinely violated: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.druid.prepared_divine.unsupported"
                    && d.claim_blocking),
            "expected the real spell-posture diagnostic to fire in the multiclass mix too: {:?}",
            receipt.computation.diagnostics
        );
    }

    fn human_druid_input_with_nature_bond(level: u8, bond_selection: Option<&str>) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: DRUID_CLASS_ID.to_owned(), level }];
        if let Some(selection) = bond_selection {
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: DRUID_NATURE_BOND_CHOICE_ID.to_owned(),
                selection_id: selection.to_owned(),
            });
        }
        input
    }

    /// v0.6 alpha swarm, risks item 8 (Druid animal companion closure): a
    /// single-class Human Druid at level 1 with the animal companion
    /// nature bond chosen, and a genuinely valid (empty) prepared-spell
    /// posture, reaches `Computed` -- the animal-companion burden is no
    /// longer permanently unconditional once the companion's own stat
    /// block (Wolf, the canonical species) is grounded, and Link/Share
    /// Spells are recognized as vacuous under this bounded seam.
    #[test]
    fn single_class_druid_level1_with_animal_companion_reaches_computed() {
        let input = human_druid_input_with_nature_bond(
            1,
            Some(DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID),
        );

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "a level-1 Druid with an animal companion and a valid spell posture should reach \
             Computed: {:?}",
            receipt.computation.diagnostics
        );
        let stat_block_ids = [
            "class_chassis.druid.animal_companion.wolf_stat_block",
            "class_chassis.druid.animal_companion.base_attack_bonus",
            "class_chassis.druid.animal_companion.base_save.fortitude",
            "class_chassis.druid.animal_companion.base_save.reflex",
            "class_chassis.druid.animal_companion.base_save.will",
            "class_chassis.druid.animal_companion.armor_class",
            "class_chassis.druid.animal_companion.bite_attack",
            "class_chassis.druid.animal_companion.hit_points",
        ];
        for id in stat_block_ids {
            assert!(
                receipt.computation.explanations.iter().any(|e| e.id == id),
                "expected the real companion stat-block record {id}: {:?}",
                receipt.computation.explanations
            );
        }
        // Wolf: HD 2, BAB = 2*3/4 = 1, Str 13 (+1 mod) -> attack bonus 2.
        let companion_attack = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.druid.animal_companion.base_attack_bonus")
            .expect("companion base attack bonus must be grounded");
        assert_eq!(companion_attack.value, 2);
        // Fort/Ref = 2/2+2 = 3, Will = 2/3 = 0.
        let companion_fort = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.druid.animal_companion.base_save.fortitude")
            .expect("companion Fortitude save must be grounded");
        assert_eq!(companion_fort.value, 3);
        // AC = 10 + 2 natural armor = 12.
        let companion_ac = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_chassis.druid.animal_companion.armor_class")
            .expect("companion armor class must be grounded");
        assert_eq!(companion_ac.value, 12);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.druid.animal_companion.advancement_absent"
                    && !d.claim_blocking),
            "expected the honest, non-blocking advancement-absent diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A Druid above 1st level with an animal companion chosen now reaches
    /// Computed too. This test previously asserted the opposite -- that
    /// level 5 fell through to the blocking catch-all -- because the
    /// companion's Hit Dice were hardcoded to a 1st-level master's 2 HD.
    /// The progression is now read from the real corpus table at every
    /// master level, so the level gate is gone and the values below are
    /// the genuine 5th-level ones, not the 1st-level ones.
    ///
    /// Wolf at master level 5: 5 HD, so base attack 5*3/4 = 3 plus a
    /// Strength modifier of +2 (base Str 13 + floor(5/3) = 14) = +5;
    /// Fortitude/Reflex 5/2+2 = +4; Will 5/3 = +1; armor class 10 + (2
    /// base natural armor + 2*floor(5/3)) = 14.
    #[test]
    fn single_class_druid_level5_with_animal_companion_reaches_computed() {
        let input = human_druid_input_with_nature_bond(
            5,
            Some(DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID),
        );

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);
        let value_of = |id: &str| {
            receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected a `{id}` record"))
                .value
        };
        assert_eq!(value_of("class_chassis.druid.animal_companion.base_attack_bonus"), 5);
        assert_eq!(value_of("class_chassis.druid.animal_companion.base_save.fortitude"), 4);
        assert_eq!(value_of("class_chassis.druid.animal_companion.base_save.will"), 1);
        assert_eq!(value_of("class_chassis.druid.animal_companion.armor_class"), 14);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.druid.animal_companion.advancement_absent"
                    && !d.claim_blocking),
            "the columns with no consumer stay named in a non-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A Druid with no nature bond selection at all still falls through
    /// to the unchanged catch-all diagnostic.
    #[test]
    fn single_class_druid_level1_with_no_nature_bond_still_blocks_on_the_catch_all() {
        let input = human_druid_input_with_nature_bond(1, None);

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.druid.animal_companion.unsupported"
                    && d.claim_blocking),
            "no nature bond chosen must still trip the catch-all: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A Druid choosing a domain-type nature bond (never recognized by
    /// this seam at all) also falls through to the unchanged catch-all --
    /// confirms the animal-companion closure doesn't silently ignore the
    /// domain-bond alternative.
    #[test]
    fn single_class_druid_level1_with_a_domain_bond_still_blocks_on_the_catch_all() {
        let input = human_druid_input_with_nature_bond(1, Some("bond:domain"));

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.druid.animal_companion.unsupported"
                    && d.claim_blocking),
            "an unrecognized domain-type nature bond must still trip the catch-all: {:?}",
            receipt.computation.diagnostics
        );
    }
}

#[cfg(test)]
mod shaman_spirit_tests {
    use super::{
        shaman_battle_spirit_bonus, shaman_monstrous_insight_bonus, shaman_spirit_touch_bonus_damage,
        shaman_spirit_uses_per_day, shaman_stardust_duration_rounds, shaman_stardust_penalty,
        shaman_storm_burst_duration_rounds,
    };

    /// The `min(2,...)` genuinely binds inside 1-20, unlike the inert
    /// bounds documented on Brawler's Flurry -- so all three tiers are
    /// reachable and worth pinning.
    #[test]
    fn battle_spirit_bonus_steps_at_8_and_16_and_then_caps() {
        assert_eq!(shaman_battle_spirit_bonus(1), 1);
        assert_eq!(shaman_battle_spirit_bonus(7), 1);
        assert_eq!(shaman_battle_spirit_bonus(8), 2);
        assert_eq!(shaman_battle_spirit_bonus(15), 2);
        assert_eq!(shaman_battle_spirit_bonus(16), 3);
        assert_eq!(shaman_battle_spirit_bonus(20), 3, "min(2,...) caps at +3");
    }

    /// Stardust's penalty is a real negative magnitude, and its
    /// `min(5,...)` cap is reached at level 20 exactly.
    #[test]
    fn stardust_penalty_is_negative_and_caps_at_minus_six() {
        assert_eq!(shaman_stardust_penalty(1), -1);
        assert_eq!(shaman_stardust_penalty(3), -1);
        assert_eq!(shaman_stardust_penalty(4), -2);
        assert_eq!(shaman_stardust_penalty(20), -6);
    }

    /// The `max(1,...)` binds only at level 1.
    #[test]
    fn stardust_duration_floor_binds_only_at_level_one() {
        assert_eq!(shaman_stardust_duration_rounds(1), 1, "max(1,0) floor binds");
        assert_eq!(shaman_stardust_duration_rounds(2), 1);
        assert_eq!(shaman_stardust_duration_rounds(4), 2);
        assert_eq!(shaman_stardust_duration_rounds(20), 10);
    }

    #[test]
    fn the_five_touch_spirits_share_one_bonus_damage_formula() {
        assert_eq!(shaman_spirit_touch_bonus_damage(1), 0, "level/2 is 0 at level 1");
        assert_eq!(shaman_spirit_touch_bonus_damage(2), 1);
        assert_eq!(shaman_spirit_touch_bonus_damage(20), 10);
    }

    #[test]
    fn monstrous_insight_and_storm_burst_match_their_corpus_formulas() {
        assert_eq!(shaman_monstrous_insight_bonus(1), 1);
        assert_eq!(shaman_monstrous_insight_bonus(20), 20);
        assert_eq!(shaman_storm_burst_duration_rounds(1), 1);
        assert_eq!(shaman_storm_burst_duration_rounds(4), 2);
        assert_eq!(shaman_storm_burst_duration_rounds(20), 6);
    }

    /// Shared 3+CHA pool, floored at 0 so a punishing Charisma cannot
    /// produce a negative resource.
    #[test]
    fn the_shared_uses_per_day_pool_is_floored_at_zero() {
        assert_eq!(shaman_spirit_uses_per_day(0), 3);
        assert_eq!(shaman_spirit_uses_per_day(4), 7);
        assert_eq!(shaman_spirit_uses_per_day(-3), 0);
        assert_eq!(shaman_spirit_uses_per_day(-9), 0, "floored, never negative");
    }
}

/// Shaman's own prepared-spellcasting surface (task #12): the
/// corpus-transcribed slot table, the ceiling derived from it, and the
/// Wisdom bonus-spell layering.
#[cfg(test)]
mod shaman_spellcasting_tests {
    use super::{
        shaman_base_spells_per_day_table, shaman_spell_level_access, shaman_total_spells_per_day,
    };

    /// Every row transcribed from `acg_classes.lst`'s own `CAST:` rows.
    #[test]
    fn the_base_table_matches_the_corpus_cast_rows() {
        assert_eq!(
            shaman_base_spells_per_day_table(1),
            [Some(3), Some(1), None, None, None, None, None, None, None, None]
        );
        assert_eq!(
            shaman_base_spells_per_day_table(20),
            [
                Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4),
                Some(4)
            ]
        );
        // Level 0 and out-of-range levels grant nothing at all.
        assert_eq!(shaman_base_spells_per_day_table(0), [None; 10]);
        assert_eq!(shaman_base_spells_per_day_table(21), [None; 10]);
    }

    /// **The flagged one-cell corpus divergence.** Shaman's level-11 row
    /// is `4,4,4,3,3,2,1`. Cleric, Druid, Wizard and Witch -- every other
    /// 9-level prepared caster in the PCGen tree -- all carry
    /// `4,4,4,4,3,2,1` there, and Shaman's own rows at 10 and 12 are
    /// byte-identical to theirs. Shaman is the ONLY class anywhere in the
    /// tree with this row, so it gains its fourth 3rd-level slot one
    /// level later than every peer.
    ///
    /// This test exists to make that choice visible rather than silently
    /// baked in: it follows the corpus per the standing corpus-first
    /// rule, and pins BOTH the value we use and the peer value we
    /// deliberately did not use, so flipping it is a one-line change with
    /// an obvious failure message.
    #[test]
    fn shaman_level_11_follows_the_corpus_not_the_peer_row() {
        let shaman_row = shaman_base_spells_per_day_table(11);
        let corpus_row =
            [Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), Some(1), None, None, None];
        let peer_row =
            [Some(4), Some(4), Some(4), Some(4), Some(3), Some(2), Some(1), None, None, None];
        assert_eq!(shaman_row, corpus_row, "must follow acg_classes.lst's own CAST row");
        assert_ne!(
            shaman_row, peer_row,
            "if this now matches the Cleric/Druid/Wizard/Witch row, the corpus-vs-peer \
             divergence was resolved in favour of the peers -- update this test deliberately"
        );
        // The rows on either side are unchanged, which is what makes the
        // single-cell difference the whole of the divergence.
        assert_eq!(
            shaman_base_spells_per_day_table(10),
            [Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None, None, None, None]
        );
        assert_eq!(
            shaman_base_spells_per_day_table(12),
            [Some(4), Some(4), Some(4), Some(4), Some(3), Some(3), Some(2), None, None, None]
        );
    }

    /// The ceiling is derived from the table, so this also proves the two
    /// cannot drift apart.
    #[test]
    fn spell_level_access_is_derived_from_the_table() {
        assert_eq!(shaman_spell_level_access(1), 1);
        assert_eq!(shaman_spell_level_access(3), 2);
        assert_eq!(shaman_spell_level_access(5), 3);
        assert_eq!(shaman_spell_level_access(7), 4);
        assert_eq!(shaman_spell_level_access(9), 5);
        assert_eq!(shaman_spell_level_access(11), 6);
        assert_eq!(shaman_spell_level_access(13), 7);
        assert_eq!(shaman_spell_level_access(15), 8);
        assert_eq!(shaman_spell_level_access(17), 9);
        assert_eq!(shaman_spell_level_access(20), 9);
        // A non-caster level exposes no spell levels at all.
        assert_eq!(shaman_spell_level_access(0), 0);
    }

    /// Wisdom bonus spells layer onto levels 1+, never onto orisons.
    #[test]
    fn wisdom_bonus_spells_apply_to_every_level_except_orisons() {
        let none = shaman_total_spells_per_day(1, 0);
        assert_eq!(none[0], Some(3));
        assert_eq!(none[1], Some(1));

        let wis_18 = shaman_total_spells_per_day(1, 4);
        assert_eq!(wis_18[0], Some(3), "orisons never take a bonus spell");
        assert_eq!(wis_18[1], Some(2), "1st level gains one bonus spell at +4 Wisdom");

        // A negative modifier must not inflate the budget.
        let wis_8 = shaman_total_spells_per_day(1, -1);
        assert_eq!(wis_8[0], Some(3));
        assert_eq!(wis_8[1], Some(1));
    }

    /// Inaccessible levels stay `None` rather than becoming `Some(0)` --
    /// otherwise a high-Wisdom low-level shaman would appear to have
    /// slots at spell levels the class table says are "—".
    #[test]
    fn inaccessible_spell_levels_stay_none_even_with_high_wisdom() {
        let totals = shaman_total_spells_per_day(1, 5);
        for (spell_level, total) in totals.iter().enumerate().skip(2) {
            assert_eq!(*total, None, "spell level {spell_level} must be inaccessible at level 1");
        }
    }
}

#[cfg(test)]
mod shaman_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, HeadlessReceiptStatus,
        FIGHTER_CLASS_ID, LIFE_SPIRIT_SELECTION, SHAMAN_CLASS_ID, SHAMAN_SPIRIT_CHOICE_ID,
    };
    use crate::rules_core::character_input::{
        load_character_input_fixture, AcquisitionMode, SelectedChoice, SpellSelection,
    };

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_shaman_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SHAMAN_CLASS_ID.to_owned(), level }];
        input
    }

    /// The spellcasting surface must be genuinely reachable through the
    /// real dispatch, not just a pure function with tests. A real Shaman
    /// preparing a real spell from the class's own list grounds it, and
    /// the per-level slot budget lands alongside it.
    #[test]
    fn a_shaman_preparing_a_real_spell_grounds_it_through_the_live_dispatch() {
        let mut input = human_shaman_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Cure Light Wounds".to_owned(),
            source_class_id: SHAMAN_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.shaman.prepared_spells.unsupported"),
            "a real on-list spell must not trip the prepared-spell blocker: {:?}",
            receipt.computation.diagnostics
        );
        let preparation = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.shaman.daily_preparation")
            .expect("daily preparation must be grounded");
        assert_eq!(preparation.value, 1);
        let orisons = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_spell.acg.shaman.total_spells_per_day.spell_level_0")
            .expect("orison budget must be grounded");
        assert_eq!(orisons.value, 3, "level 1 CAST:3,1 -- three orisons");
    }

    /// The converse: a spell that is not on Shaman's own list claim-blocks
    /// rather than being silently accepted. Proves the list is actually
    /// consulted, not merely present.
    #[test]
    fn a_shaman_preparing_an_off_list_spell_is_claim_blocked() {
        let mut input = human_shaman_input(1);
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: SHAMAN_CLASS_ID.to_owned(),
            acquisition_mode: AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.acg.shaman.prepared_spells.unsupported"
                    && d.claim_blocking),
            "an off-list spell must claim-block: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// All ten Spirits must be reachable through the real dispatch, each
    /// grounding at least one explanation and clearing the claim-blocking
    /// spirit_powers diagnostic. Enumerated rather than sampled, so a
    /// Spirit wired into the constant list but missed in the dispatch
    /// fails here.
    #[test]
    fn every_one_of_the_ten_spirits_is_recognized_through_the_live_dispatch() {
        let spirits = [
            "spirit:life",
            "spirit:battle",
            "spirit:bones",
            "spirit:flame",
            "spirit:heavens",
            "spirit:lore",
            "spirit:nature",
            "spirit:stone",
            "spirit:waves",
            "spirit:wind",
        ];
        for spirit in spirits {
            let mut input = human_shaman_input(8);
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: SHAMAN_SPIRIT_CHOICE_ID.to_owned(),
                selection_id: spirit.to_owned(),
            });
            let receipt = build_pilot_headless_receipt(&input);

            assert!(
                !receipt
                    .computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id == "class_feature.acg.shaman.spirit_powers.unsupported"),
                "{spirit} must be recognized: {:?}",
                receipt.computation.diagnostics
            );
            let grounded = receipt
                .computation
                .explanations
                .iter()
                .filter(|e| e.id.starts_with("class_feature.acg.shaman."))
                .count();
            assert!(grounded > 0, "{spirit} grounded no class-feature explanation");
        }
    }

    /// Battle Spirit's 3+CHA pool is ROUNDS per day, not uses -- same
    /// arithmetic as the other eight, different resource. The explanation
    /// id must say so, because that is what a player reads.
    #[test]
    fn battle_spirit_reports_rounds_per_day_not_uses_per_day() {
        let mut input = human_shaman_input(8);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SHAMAN_SPIRIT_CHOICE_ID.to_owned(),
            selection_id: "spirit:battle".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == "class_feature.acg.shaman.battle_spirit.rounds_per_day"),
            "Battle Spirit must report a rounds_per_day fact"
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.contains("battle_spirit.uses_per_day")),
            "Battle Spirit must NOT be labelled uses_per_day -- its corpus variable is \
             ShamanBattleSpiritRounds"
        );
        let bonus = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.shaman.battle_spirit.morale_bonus")
            .expect("morale bonus must be grounded");
        assert_eq!(bonus.value, 2, "level 8 is the first +2 tier");
    }

    /// An unrecognized spirit id must still claim-block rather than
    /// silently pass now that nine more selections are accepted.
    #[test]
    fn an_unrecognized_spirit_selection_still_claim_blocks() {
        let mut input = human_shaman_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SHAMAN_SPIRIT_CHOICE_ID.to_owned(),
            selection_id: "spirit:not_a_real_spirit".to_owned(),
        });
        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.shaman.spirit_powers.unsupported"
                    && d.claim_blocking),
            "an unknown spirit must claim-block: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A bare single-class Human Shaman (no Spirit choice) stays
    /// `Blocked` on both the spirit_powers diagnostic and
    /// other_features_deferred, never the retired generic diagnostic.
    #[test]
    fn single_class_shaman_bare_stays_blocked_on_spirit_powers_and_other_features() {
        let input = human_shaman_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "Shaman must stay Blocked without a recognized Spirit choice: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.shaman.unsupported"),
            "the retired generic diagnostic must never appear for Shaman: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.shaman.spirit_powers.unsupported"
                    && d.claim_blocking),
            "expected the spirit_powers claim-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.shaman.other_features_deferred.unsupported"
                    && d.claim_blocking),
            "expected the other_features_deferred diagnostic: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// A single-class Human Shaman with Life Spirit recognized grounds
    /// Channel's real uses-per-day/dice/DC facts and clears the
    /// spirit_powers diagnostic in favor of the non-blocking "other
    /// spirits" note -- stays `Blocked` on other_features_deferred
    /// regardless.
    ///
    /// Fixture Charisma 8 (-1 modifier). Level 1: uses/day max(1-1,0)=0,
    /// dice (1+1)/2=1, DC 10+0-1=9.
    #[test]
    fn single_class_shaman_with_life_spirit_grounds_channels_real_facts() {
        let mut input = human_shaman_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SHAMAN_SPIRIT_CHOICE_ID.to_owned(),
            selection_id: LIFE_SPIRIT_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.shaman.spirit_powers.unsupported"),
            "spirit_powers must not fire once Life Spirit is recognized: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.shaman.spirit_powers_beyond_life.unmodeled"
                    && !d.claim_blocking),
            "expected the non-blocking other-spirits note: {:?}",
            receipt.computation.diagnostics
        );

        let uses_per_day = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.shaman.life_spirit.channel_uses_per_day")
            .expect("Channel uses-per-day must be grounded once recognized");
        assert_eq!(
            uses_per_day.value, 0,
            "fixture Charisma 8 (-1 modifier): max(1-1,0)=0: {:?}",
            uses_per_day
        );

        let dice = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.shaman.life_spirit.channel_dice")
            .expect("Channel dice must be grounded once recognized");
        assert_eq!(dice.value, 1, "Shaman level 1 Channel dice: (1+1)/2=1: {:?}", dice);

        let dc = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.shaman.life_spirit.channel_dc")
            .expect("Channel DC must be grounded once recognized");
        assert_eq!(dc.value, 9, "Shaman level 1 Channel DC: 10+0-1=9: {:?}", dc);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Life is a recognized Spirit, so the canonical-narrowing posture is satisfied and \
             other_features_deferred reports non-blocking: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// **Canonical narrowing, Shaman's own (task: chooser-shaped power
    /// lists).** Life is the canonical Spirit this codebase seeds. Unlike
    /// Witch -- where only 3 of 53 hexes are grounded -- all TEN primary
    /// Spirits are already recognized through their own
    /// immediately-available base ability, so the narrowing here is about
    /// which one the default posture seeds, not about which ones work.
    /// Life earns it by grounding the richest real magnitude set (Channel
    /// uses-per-day, dice AND save DC, the Cleric-Channel-Energy shape).
    #[test]
    fn shaman_with_the_canonical_life_spirit_stays_computed_at_every_level() {
        for level in 1..=20u8 {
            let mut input = human_shaman_input(level);
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: SHAMAN_SPIRIT_CHOICE_ID.to_owned(),
                selection_id: LIFE_SPIRIT_SELECTION.to_owned(),
            });

            let receipt = build_pilot_headless_receipt(&input);

            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Computed,
                "Shaman level {level} with the canonical Life Spirit must be Computed: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// Every one of the ten Spirits must reach `Computed`, not merely
    /// clear `spirit_powers`. This is the stronger form of
    /// `every_one_of_the_ten_spirits_is_recognized_through_the_live_dispatch`:
    /// it pins that retiring `other_features_deferred` is keyed off real
    /// Spirit recognition and not off the Life branch alone.
    #[test]
    fn all_ten_spirits_reach_computed_not_just_the_canonical_one() {
        let spirits = [
            "spirit:life",
            "spirit:battle",
            "spirit:bones",
            "spirit:flame",
            "spirit:heavens",
            "spirit:lore",
            "spirit:nature",
            "spirit:stone",
            "spirit:waves",
            "spirit:wind",
        ];
        for spirit in spirits {
            let mut input = human_shaman_input(8);
            input.chosen.selected_choices.push(SelectedChoice {
                choice_set_id: SHAMAN_SPIRIT_CHOICE_ID.to_owned(),
                selection_id: spirit.to_owned(),
            });
            let receipt = build_pilot_headless_receipt(&input);

            assert_eq!(
                receipt.status,
                HeadlessReceiptStatus::Computed,
                "{spirit} must reach Computed: {:?}",
                receipt.computation.diagnostics
            );
        }
    }

    /// An unrecognized Spirit must leave the Shaman `Blocked` outright,
    /// not merely carry the `spirit_powers` diagnostic alongside an
    /// otherwise-green receipt.
    #[test]
    fn an_unrecognized_spirit_leaves_the_shaman_blocked_outright() {
        let mut input = human_shaman_input(1);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SHAMAN_SPIRIT_CHOICE_ID.to_owned(),
            selection_id: "spirit:not_a_real_spirit".to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "an unknown Spirit must leave the Shaman Blocked: {:?}",
            receipt.computation.diagnostics
        );
    }

    /// Channel's dice/DC progression at a higher level, verified against
    /// the PCGen corpus formula directly (not merely trusting the
    /// level-1 case above): level 5 dice (5+1)/2=3, DC 10+2-1=11.
    #[test]
    fn shaman_channel_dice_and_dc_match_the_corpus_formula_at_a_higher_level() {
        let mut input = human_shaman_input(5);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SHAMAN_SPIRIT_CHOICE_ID.to_owned(),
            selection_id: LIFE_SPIRIT_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        let dice = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.shaman.life_spirit.channel_dice")
            .expect("Channel dice must be grounded");
        assert_eq!(dice.value, 3, "level 5 Channel dice: (5+1)/2=3: {:?}", dice);

        let dc = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.shaman.life_spirit.channel_dc")
            .expect("Channel DC must be grounded");
        assert_eq!(dc.value, 11, "level 5 Channel DC: 10+2-1=11: {:?}", dc);
    }

    /// A non-Shaman character carrying a spoofed Life Spirit choice must
    /// have it silently ignored -- the class-ownership gate is by
    /// construction, not a bolt-on rejection. Also proves Fighter's own
    /// golden path is unaffected.
    #[test]
    fn non_shaman_characters_spoofed_life_spirit_choice_is_ignored() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: SHAMAN_SPIRIT_CHOICE_ID.to_owned(),
            selection_id: LIFE_SPIRIT_SELECTION.to_owned(),
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected by a stray Shaman choice: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.acg.shaman.")),
            "a non-Shaman character must never ground any Shaman explanation: {:?}",
            receipt.computation.explanations
        );
    }
}

